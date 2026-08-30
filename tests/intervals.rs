//! One reading shape across `Lastgang`, `Zeitreihe` and `Energiemenge`.
//!
//! The point of `IntervalReading` is that a consumer writes *one* mapping.
//! These tests are that claim, checked: the three BO4E shapes produce the same
//! struct, the struct writes back into all three, and the round trip does not
//! lose what BO4E has a field for.

#![cfg(all(feature = "versioned", feature = "time", feature = "decimal"))]

use rubo4e::current::{
    Energiemenge, Lastgang, Menge, Mengeneinheit, Messwertstatus, Messwertstatuszusatz, Zeitraum,
    Zeitreihe, Zeitreihenwert,
};
use rubo4e::identifiers::ObisCode;
use rubo4e::timeseries::{Bo4eIntervals, Bo4eTimeSeries, IntervalReading};
use rust_decimal::Decimal;
use time::macros::datetime;
use time::{Duration, OffsetDateTime};

fn start() -> OffsetDateTime {
    datetime!(2026-01-01 00:00 +01:00)
}

fn quarter_hours(unit: Mengeneinheit, values: &[i64]) -> Vec<Zeitreihenwert> {
    let _ = unit;
    values
        .iter()
        .enumerate()
        .map(|(i, v)| {
            let from = start() + Duration::minutes(15 * i as i64);
            Zeitreihenwert {
                wert: Some(Decimal::from(*v)),
                zeitraum: Some(Zeitraum::from_instants(from, from + Duration::minutes(15))),
                status: Some(Messwertstatus::Abgelesen),
                ..Default::default()
            }
        })
        .collect()
}

fn quarter_hour_menge() -> Menge {
    Menge {
        wert: Some(Decimal::from(15)),
        einheit: Some(Mengeneinheit::Minute),
        ..Default::default()
    }
}

// ─── Reading ─────────────────────────────────────────────────────────────────

#[test]
fn a_lastgang_yields_one_reading_per_entry() {
    let obis = ObisCode::new("1-0:1.8.0").unwrap();
    let lastgang = Lastgang {
        messgroesse: Some(Mengeneinheit::Kw),
        obis_kennzahl: Some(obis.clone()),
        werte: Some(quarter_hours(Mengeneinheit::Kw, &[400, 480, 320, 400])),
        ..Lastgang::new(quarter_hour_menge())
    };

    let readings: Vec<_> = lastgang.intervals().collect();
    assert_eq!(readings.len(), 4);
    assert_eq!(readings[1].index, 1);
    assert_eq!(readings[1].wert, Some(Decimal::from(480)));
    assert_eq!(readings[1].einheit, Some(Mengeneinheit::Kw));
    assert_eq!(readings[1].status, Some(Messwertstatus::Abgelesen));
    assert_eq!(readings[1].obis_kennzahl, Some(&obis));
    assert_eq!(readings[1].duration(), Duration::minutes(15));
    assert_eq!(
        readings[1].range,
        datetime!(2026-01-01 00:15 +01:00)..datetime!(2026-01-01 00:30 +01:00)
    );

    // A power series answers in energy, and matches what `integrate` says.
    assert_eq!(
        lastgang.total_energy(),
        Some((Decimal::from(400), Mengeneinheit::Kwh))
    );
    assert_eq!(lastgang.integrate(), Some(Decimal::from(400)));
}

#[test]
fn a_zeitreihe_in_energy_units_needs_no_integration() {
    let zeitreihe = Zeitreihe {
        einheit: Some(Mengeneinheit::Kwh),
        werte: Some(quarter_hours(Mengeneinheit::Kwh, &[100, 120, 80, 100])),
        ..Default::default()
    };

    assert_eq!(zeitreihe.intervals().count(), 4);
    assert_eq!(
        zeitreihe.total_energy(),
        Some((Decimal::from(400), Mengeneinheit::Kwh))
    );
    // …and the same number the plain sum gives, because kWh is summable.
    assert_eq!(zeitreihe.sum(), Some(Decimal::from(400)));
}

#[test]
fn an_energiemenge_is_a_one_entry_series() {
    let obis = ObisCode::new("1-0:1.8.0").unwrap();
    let menge = Energiemenge {
        menge: Some(Menge {
            wert: Some(Decimal::from(400)),
            einheit: Some(Mengeneinheit::Kwh),
            ..Default::default()
        }),
        zeitraum: Some(Zeitraum::from_instants(
            start(),
            start() + Duration::hours(1),
        )),
        obis_kennzahl: Some(obis.clone()),
        ..Default::default()
    };

    let readings: Vec<_> = menge.intervals().collect();
    assert_eq!(readings.len(), 1);
    assert_eq!(readings[0].index, 0);
    assert_eq!(readings[0].obis_kennzahl, Some(&obis));
    assert_eq!(readings[0].duration(), Duration::hours(1));
    assert_eq!(
        menge.total_energy(),
        Some((Decimal::from(400), Mengeneinheit::Kwh))
    );
}

#[test]
fn an_energiemenge_without_a_resolvable_period_yields_nothing() {
    let no_period = Energiemenge {
        menge: Some(Menge {
            wert: Some(Decimal::ONE),
            einheit: Some(Mengeneinheit::Kwh),
            ..Default::default()
        }),
        ..Default::default()
    };
    assert_eq!(no_period.intervals().count(), 0);

    // A period given as bare dates has no time of day, so it is not an interval.
    let dates_only = Energiemenge {
        menge: no_period.menge.clone(),
        zeitraum: Some(Zeitraum {
            startdatum: Some(time::macros::date!(2026 - 01 - 01)),
            enddatum: Some(time::macros::date!(2026 - 02 - 01)),
            ..Default::default()
        }),
        ..Default::default()
    };
    assert_eq!(dates_only.intervals().count(), 0);
}

/// The three shapes agree on the same physical quantity — which is the whole
/// reason for one reading type.
#[test]
fn the_three_shapes_agree_on_the_energy() {
    let lastgang = Lastgang {
        messgroesse: Some(Mengeneinheit::Kw),
        werte: Some(quarter_hours(Mengeneinheit::Kw, &[400, 480, 320, 400])),
        ..Lastgang::new(quarter_hour_menge())
    };
    let zeitreihe = Zeitreihe {
        einheit: Some(Mengeneinheit::Kwh),
        werte: Some(quarter_hours(Mengeneinheit::Kwh, &[100, 120, 80, 100])),
        ..Default::default()
    };
    let energiemenge = Energiemenge {
        menge: Some(Menge {
            wert: Some(Decimal::from(400)),
            einheit: Some(Mengeneinheit::Kwh),
            ..Default::default()
        }),
        zeitraum: Some(Zeitraum::from_instants(
            start(),
            start() + Duration::hours(1),
        )),
        ..Default::default()
    };

    let expected = Some((Decimal::from(400), Mengeneinheit::Kwh));
    assert_eq!(lastgang.total_energy(), expected);
    assert_eq!(zeitreihe.total_energy(), expected);
    assert_eq!(energiemenge.total_energy(), expected);
}

// ─── Status handling ─────────────────────────────────────────────────────────

/// A `FEHLT` slot carrying `0` is an absence, not a zero.
#[test]
fn an_unusable_reading_is_skipped_rather_than_counted_as_zero() {
    let mut werte = quarter_hours(Mengeneinheit::Kwh, &[100, 0, 80, 100]);
    werte[1].status = Some(Messwertstatus::Fehlt);
    werte[1].statuszusatz = Some(Messwertstatuszusatz::Z75Kommunikationsstoerung);

    let zeitreihe = Zeitreihe {
        einheit: Some(Mengeneinheit::Kwh),
        werte: Some(werte),
        ..Default::default()
    };

    let readings: Vec<_> = zeitreihe.intervals().collect();
    assert_eq!(readings.len(), 4);
    assert!(!readings[1].is_usable());
    assert_eq!(
        readings[1].statuszusatz,
        Some(Messwertstatuszusatz::Z75Kommunikationsstoerung)
    );
    assert_eq!(readings[1].energy(), None);

    assert_eq!(zeitreihe.usable_intervals().count(), 3);
    assert_eq!(
        zeitreihe.total_energy(),
        Some((Decimal::from(280), Mengeneinheit::Kwh))
    );
    // `sum` refuses outright rather than understating — the two answer different
    // questions, and both are honest about which.
    assert_eq!(zeitreihe.sum(), None);
}

#[test]
fn a_reading_without_a_value_is_not_usable() {
    let reading = IntervalReading::new(
        start()..start() + Duration::hours(1),
        None,
        Some(Mengeneinheit::Kwh),
    );
    assert!(!reading.is_usable());
    assert_eq!(reading.energy(), None);
}

// ─── Units ───────────────────────────────────────────────────────────────────

#[test]
fn conversion_stays_inside_a_dimension() {
    let reading = IntervalReading::new(
        start()..start() + Duration::hours(1),
        Some(Decimal::from(2)),
        Some(Mengeneinheit::Mwh),
    );

    let kwh = reading.converted_to(Mengeneinheit::Kwh).unwrap();
    assert_eq!(kwh.wert, Some(Decimal::from(2000)));
    assert_eq!(kwh.einheit, Some(Mengeneinheit::Kwh));
    assert_eq!(kwh.range, reading.range);

    // Energy → power is not a conversion; it needs the interval length.
    assert!(reading.converted_to(Mengeneinheit::Kw).is_none());
}

#[test]
fn a_unit_that_is_neither_energy_nor_power_yields_no_energy() {
    let reading = IntervalReading::new(
        start()..start() + Duration::hours(1),
        Some(Decimal::from(5)),
        Some(Mengeneinheit::Kubikmeter),
    );
    assert_eq!(reading.energy(), None);
}

// ─── Writing back ────────────────────────────────────────────────────────────

#[test]
fn readings_build_a_zeitreihe_that_reads_back_the_same() {
    let readings: Vec<IntervalReading<'static>> = (0..4)
        .map(|i| {
            let from = start() + Duration::minutes(15 * i);
            IntervalReading::new(
                from..from + Duration::minutes(15),
                Some(Decimal::from(100 + i)),
                Some(Mengeneinheit::Kwh),
            )
            .with_status(Messwertstatus::Abgelesen)
        })
        .collect();

    let zeitreihe = Zeitreihe::from_intervals(readings.clone());
    assert_eq!(zeitreihe.einheit, Some(Mengeneinheit::Kwh));

    let back: Vec<_> = zeitreihe.intervals().collect();
    assert_eq!(back.len(), 4);
    for (original, round_tripped) in readings.iter().zip(&back) {
        assert_eq!(original.range, round_tripped.range);
        assert_eq!(original.wert, round_tripped.wert);
        assert_eq!(original.einheit, round_tripped.einheit);
        assert_eq!(original.status, round_tripped.status);
    }
    assert!(zeitreihe.audit().is_complete());
}

#[test]
fn readings_build_a_lastgang_that_audits_against_its_own_interval() {
    let obis = ObisCode::new("1-0:1.29.0").unwrap();
    let readings: Vec<IntervalReading<'_>> = (0..4)
        .map(|i| {
            let from = start() + Duration::minutes(15 * i);
            IntervalReading::new(
                from..from + Duration::minutes(15),
                Some(Decimal::from(400)),
                Some(Mengeneinheit::Kw),
            )
            .with_obis(&obis)
        })
        .collect();

    let lastgang = Lastgang::from_intervals(quarter_hour_menge(), readings);
    assert_eq!(lastgang.messgroesse, Some(Mengeneinheit::Kw));
    assert_eq!(lastgang.obis_kennzahl.as_ref(), Some(&obis));

    let report = lastgang.audit();
    assert!(report.is_complete());
    assert!(report.wrong_length.is_empty());
    assert_eq!(
        lastgang.total_energy(),
        Some((Decimal::from(400), Mengeneinheit::Kwh))
    );
}

/// A series assembled from mixed units is normalised to the first, rather than
/// silently summing MWh into kWh.
#[test]
fn mixed_units_are_converted_to_the_first() {
    let a = IntervalReading::new(
        start()..start() + Duration::hours(1),
        Some(Decimal::from(1000)),
        Some(Mengeneinheit::Kwh),
    );
    let b = IntervalReading::new(
        start() + Duration::hours(1)..start() + Duration::hours(2),
        Some(Decimal::from(2)),
        Some(Mengeneinheit::Mwh),
    );

    let zeitreihe = Zeitreihe::from_intervals([a, b]);
    assert_eq!(zeitreihe.einheit, Some(Mengeneinheit::Kwh));
    assert_eq!(
        zeitreihe.total_energy(),
        Some((Decimal::from(3000), Mengeneinheit::Kwh))
    );
}

#[test]
fn a_reading_writes_back_as_an_energiemenge_losslessly() {
    let obis = ObisCode::new("1-0:1.8.0").unwrap();
    let reading = IntervalReading::new(
        start()..start() + Duration::hours(1),
        Some(Decimal::from(400)),
        Some(Mengeneinheit::Kwh),
    )
    .with_obis(&obis);

    let menge = reading.to_energiemenge();
    let back = menge.intervals().next().unwrap();
    assert_eq!(back.range, reading.range);
    assert_eq!(back.wert, reading.wert);
    assert_eq!(back.einheit, reading.einheit);
    assert_eq!(back.obis_kennzahl, Some(&obis));
}

/// `Zeitreihenwert` has no unit field, so this direction cannot carry one — and
/// says so rather than inventing one.
#[test]
fn a_zeitreihenwert_carries_no_unit() {
    let reading = IntervalReading::new(
        start()..start() + Duration::hours(1),
        Some(Decimal::from(7)),
        Some(Mengeneinheit::Kwh),
    )
    .with_status(Messwertstatus::Ersatzwert);

    let wert = reading.to_zeitreihenwert();
    assert_eq!(wert.wert, Some(Decimal::from(7)));
    assert_eq!(wert.status, Some(Messwertstatus::Ersatzwert));
    assert_eq!(
        wert.zeitraum.as_ref().and_then(Zeitraum::instant_duration),
        Some(Ok(Duration::hours(1)))
    );
}

/// Two readings in different energy units cannot be totalled into one number, and
/// this refuses rather than picking one.
///
/// A `Zeitreihe` states one unit for every entry, so the mismatch cannot arise
/// there — it arises where readings are assembled by hand, which is exactly what
/// a downstream `Bo4eIntervals` impl does.
#[test]
fn readings_in_two_energy_units_have_no_total() {
    struct HandAssembled<'a>(Vec<IntervalReading<'a>>);
    impl Bo4eIntervals for HandAssembled<'_> {
        fn intervals(&self) -> impl Iterator<Item = IntervalReading<'_>> {
            self.0.iter().cloned()
        }
    }

    let mixed = HandAssembled(vec![
        IntervalReading::new(
            start()..start() + Duration::hours(1),
            Some(Decimal::ONE),
            Some(Mengeneinheit::Kwh),
        ),
        IntervalReading::new(
            start() + Duration::hours(1)..start() + Duration::hours(2),
            Some(Decimal::ONE),
            Some(Mengeneinheit::Kvarh),
        ),
    ]);
    assert_eq!(mixed.total_energy(), None);
}
