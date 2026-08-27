//! Time-series handling end to end: a quarter-hourly `Lastgang` arrives as JSON,
//! is audited for completeness, and is turned into the energy an invoice bills.
//!
//! The unit tests in `src/timeseries.rs` build values in Rust. These start from
//! the wire, because the shape of a `Zeitreihenwert.zeitraum` — a date *and* a
//! time of day carrying its own UTC offset — is the part a hand-built value
//! quietly gets right and a real payload does not.

#![cfg(all(feature = "versioned", feature = "time", feature = "json"))]

use rubo4e::current::{Lastgang, Zeitreihe};
use rubo4e::json::Bo4eJsonExt;
use rubo4e::timeseries::{Bo4eTimeSeries, UnplacedReason};
use time::macros::datetime;
use time::Duration;

/// One quarter-hour of wire JSON, offset `+01:00` (German winter time).
fn slot(hh: u32, mm: u32, wert: &str) -> String {
    let end_total = hh * 60 + mm + 15;
    format!(
        r#"{{"_typ":"ZEITREIHENWERT","wert":"{wert}","zeitraum":{{"_typ":"ZEITRAUM",
           "startdatum":"2026-01-01","startuhrzeit":"{hh:02}:{mm:02}:00+01:00",
           "enddatum":"2026-01-01","enduhrzeit":"{:02}:{:02}:00+01:00"}}}}"#,
        end_total / 60,
        end_total % 60,
    )
}

fn lastgang_json(slots: &[String]) -> String {
    format!(
        r#"{{"_typ":"LASTGANG","_version":"202607.1.0",
           "sparte":"STROM","messgroesse":"KW","obisKennzahl":"1-0:1.8.0",
           "zeitIntervallLaenge":{{"_typ":"MENGE","wert":"15","einheit":"MINUTE"}},
           "werte":[{}]}}"#,
        slots.join(",")
    )
}

const T0: time::OffsetDateTime = datetime!(2026-01-01 00:00 +01:00);

#[test]
fn a_contiguous_hour_from_the_wire_audits_clean() {
    let json = lastgang_json(&[
        slot(0, 0, "400.0"),
        slot(0, 15, "480.0"),
        slot(0, 30, "512.5"),
        slot(0, 45, "390.0"),
    ]);
    let lg = Lastgang::from_json_german(&json).expect("valid Lastgang JSON");

    assert_eq!(lg.expected_interval(), Some(Duration::minutes(15)));
    assert_eq!(lg.span(), Some(T0..T0 + Duration::HOUR));

    let report = lg.audit();
    assert!(report.is_complete(), "{report:?}");
    assert_eq!(report.covered, Duration::HOUR);
    assert_eq!(report.coverage_ratio(), Some(1.0));
    assert_eq!(report.missing(), Duration::ZERO);
    assert!(!report.out_of_order);
}

#[test]
fn a_missing_slot_is_reported_at_its_place_on_the_clock() {
    let json = lastgang_json(&[slot(0, 0, "400.0"), slot(0, 30, "512.5")]);
    let lg = Lastgang::from_json_german(&json).expect("valid Lastgang JSON");

    let report = lg.audit();
    assert!(!report.is_complete());
    assert_eq!(
        report.gaps,
        [T0 + Duration::minutes(15)..T0 + Duration::minutes(30)]
    );
    assert_eq!(report.missing(), Duration::minutes(15));

    // Auditing against the whole day finds the rest of it too.
    let day = lg.audit_over(T0..T0 + Duration::days(1));
    assert_eq!(day.gaps.len(), 2);
    assert_eq!(day.covered, Duration::minutes(30));
}

/// The failure mode that quietly inflates a billed quantity: the same
/// quarter-hour delivered twice.
#[test]
fn a_repeated_slot_is_reported_and_counted_once() {
    let json = lastgang_json(&[
        slot(0, 0, "400.0"),
        slot(0, 0, "400.0"),
        slot(0, 15, "480.0"),
    ]);
    let lg = Lastgang::from_json_german(&json).expect("valid Lastgang JSON");

    let report = lg.audit();
    assert_eq!(report.overlaps, [T0..T0 + Duration::minutes(15)]);
    assert_eq!(report.covered, Duration::minutes(30));
    assert!(report.gaps.is_empty(), "no time is actually missing");
}

/// A payload that states a date but no time of day is a whole-day period, not a
/// quarter-hour, and it has no place on a 15-minute timeline.
#[test]
fn a_date_only_zeitraum_is_reported_rather_than_silently_dropped() {
    let json = lastgang_json(&[
        slot(0, 0, "400.0"),
        r#"{"wert":"480.0","zeitraum":{"startdatum":"2026-01-01","enddatum":"2026-01-01"}}"#
            .to_owned(),
        r#"{"wert":"512.5"}"#.to_owned(),
    ]);
    let lg = Lastgang::from_json_german(&json).expect("valid Lastgang JSON");

    let report = lg.audit();
    let reasons: Vec<_> = report
        .unplaced
        .iter()
        .map(|u| (u.index, u.reason.clone()))
        .collect();
    assert_eq!(
        reasons,
        [
            (1, UnplacedReason::NotAnInstantRange),
            (2, UnplacedReason::MissingZeitraum),
        ]
    );
    assert!(!report.is_complete());
}

/// A time of day with no UTC offset names a wall-clock reading, not a moment.
/// Germany changes offset twice a year, so guessing one is a two-hour error for
/// half the year.
#[test]
fn a_zeitraum_without_an_offset_is_not_an_instant() {
    let json = lastgang_json(&[r#"{"wert":"400.0","zeitraum":{
        "startdatum":"2026-01-01","startuhrzeit":"00:00:00",
        "enddatum":"2026-01-01","enduhrzeit":"00:15:00"}}"#
        .to_owned()]);
    let lg = Lastgang::from_json_german(&json).expect("valid Lastgang JSON");

    let report = lg.audit();
    assert_eq!(report.unplaced.len(), 1);
    assert!(matches!(
        report.unplaced[0].reason,
        UnplacedReason::Malformed(_)
    ));
    assert!(report.unplaced[0].reason.to_string().contains("offset"));
}

/// Slots written in different offsets are the same timeline: `01:30+02:00` and
/// `00:30+01:00` are one instant, so they overlap rather than abut.
#[test]
fn offsets_are_resolved_before_the_intervals_are_compared() {
    let json = lastgang_json(&[
        slot(0, 30, "400.0"),
        r#"{"wert":"480.0","zeitraum":{
            "startdatum":"2026-01-01","startuhrzeit":"01:30:00+02:00",
            "enddatum":"2026-01-01","enduhrzeit":"01:45:00+02:00"}}"#
            .to_owned(),
    ]);
    let lg = Lastgang::from_json_german(&json).expect("valid Lastgang JSON");

    let report = lg.audit();
    assert_eq!(report.covered, Duration::minutes(15), "one slot, twice");
    assert_eq!(report.overlaps.len(), 1);
}

/// The step every consumer of a `Lastgang` eventually takes: power over time is
/// energy. Four quarter-hours of kW become kWh.
#[cfg(feature = "decimal")]
#[test]
fn a_load_profile_integrates_to_the_energy_an_invoice_bills() {
    use rust_decimal::Decimal;

    let json = lastgang_json(&[
        slot(0, 0, "400.0"),
        slot(0, 15, "480.0"),
        slot(0, 30, "512.0"),
        slot(0, 45, "408.0"),
    ]);
    let lg = Lastgang::from_json_german(&json).expect("valid Lastgang JSON");

    // (400 + 480 + 512 + 408) / 4 = 450 kWh over the hour.
    assert_eq!(lg.integrate(), Some(Decimal::from(450)));

    // …and the plain sum is refused, because kW is not an extensive quantity.
    assert_eq!(lg.sum(), None);
}

/// A `Zeitreihe` in kWh is the other case: those values *are* additive, and it
/// declares no interval length, so an irregular series is not a defect.
#[cfg(feature = "decimal")]
#[test]
fn an_energy_zeitreihe_sums_and_tolerates_irregular_intervals() {
    use rust_decimal::Decimal;

    let json = format!(
        r#"{{"_typ":"ZEITREIHE","_version":"202607.1.0","einheit":"KWH",
           "messart":"NONE","werte":[{},{}]}}"#,
        slot(0, 0, "100.0"),
        r#"{"wert":"200.0","zeitraum":{
            "startdatum":"2026-01-01","startuhrzeit":"00:15:00+01:00",
            "enddatum":"2026-01-01","enduhrzeit":"01:00:00+01:00"}}"#,
    );
    let zr = Zeitreihe::from_json_german(&json).expect("valid Zeitreihe JSON");

    assert_eq!(zr.expected_interval(), None);
    let report = zr.audit();
    assert!(report.wrong_length.is_empty());
    assert!(report.is_complete(), "{report:?}");
    assert_eq!(zr.sum(), Some(Decimal::from(300)));
}

/// A slot that is not the declared 15 minutes is named by index, and it is a
/// *separate* finding from a gap: the timeline can still be contiguous.
#[cfg(feature = "decimal")]
#[test]
fn a_wrong_length_slot_is_named_without_being_called_a_gap() {
    let json = format!(
        r#"{{"_typ":"LASTGANG","messgroesse":"KW",
           "zeitIntervallLaenge":{{"wert":"15","einheit":"MINUTE"}},
           "werte":[{},{}]}}"#,
        slot(0, 0, "400.0"),
        r#"{"wert":"480.0","zeitraum":{
            "startdatum":"2026-01-01","startuhrzeit":"00:15:00+01:00",
            "enddatum":"2026-01-01","enduhrzeit":"00:45:00+01:00"}}"#,
    );
    let lg = Lastgang::from_json_german(&json).expect("valid Lastgang JSON");

    let report = lg.audit();
    assert_eq!(report.wrong_length, [1]);
    assert!(report.gaps.is_empty());
    assert!(report.overlaps.is_empty());
    assert!(!report.is_complete());
}

/// A `Zeitraum` this crate writes must be one it reads back identically, so a
/// producer and a consumer built on it agree.
#[test]
fn constructed_intervals_round_trip_through_the_wire() {
    use rubo4e::current::Zeitraum;

    for offset in [
        time::macros::offset!(+1),
        time::macros::offset!(+2),
        time::macros::offset!(UTC),
        time::macros::offset!(-5:30),
    ] {
        let start = datetime!(2026-06-30 23:45:00).assume_offset(offset);
        let end = start + Duration::minutes(15);
        let built = Zeitraum::from_instants(start, end);

        let json = built.to_json_german().expect("serializes");
        let back = Zeitraum::from_json_german(&json).expect("round-trips");
        assert_eq!(back, built);
        assert_eq!(back.as_instant_range(), Some(Ok(start..end)));
    }
}

/// `placed` skips what it cannot place, so a caller that wants the readings and
/// a caller that wants the defects use different entry points on purpose.
#[test]
fn placed_yields_only_the_resolvable_entries() {
    let json = lastgang_json(&[
        slot(0, 0, "400.0"),
        r#"{"wert":"480.0"}"#.to_owned(),
        slot(0, 15, "512.0"),
    ]);
    let lg = Lastgang::from_json_german(&json).expect("valid Lastgang JSON");

    let indices: Vec<usize> = lg.placed().map(|p| p.index).collect();
    assert_eq!(indices, [0, 2], "the entry with no zeitraum is skipped");
    assert_eq!(lg.audit().unplaced.len(), 1, "…and reported by audit");
}

// ─── Register readings (`Zaehlwerk`) ─────────────────────────────────────────
//
// The other time-series shape BO4E carries: a `Zaehlwerk` holds `Messwert`s,
// which are cumulative meter states at instants rather than quantities over
// intervals. The consumption is the difference between two of them, and the bare
// subtraction is wrong in two ways the schema itself tells you how to fix.

#[cfg(feature = "decimal")]
mod register {
    use rubo4e::current::{
        Menge, Mengeneinheit, Messwert, Messwertstatus, Messwertstatuszusatz, Zaehlwerk,
    };
    use rubo4e::json::Bo4eJsonExt;
    use rubo4e::timeseries::ConsumptionError;
    use rust_decimal::Decimal;
    use time::macros::datetime;
    use time::OffsetDateTime;

    fn reading(at: OffsetDateTime, state: i64) -> Messwert {
        Messwert {
            zeitpunkt: Some(at),
            wert: Some(Menge {
                wert: Some(Decimal::from(state)),
                einheit: Some(Mengeneinheit::Kwh),
                ..Default::default()
            }),
            messwertstatus: Some(Messwertstatus::Abgelesen),
            ..Default::default()
        }
    }

    fn register(messwerte: Vec<Messwert>) -> Zaehlwerk {
        Zaehlwerk {
            einheit: Some(Mengeneinheit::Kwh),
            vorkommastelle: Some(6),
            messwerte: Some(messwerte),
            ..Default::default()
        }
    }

    const T0: OffsetDateTime = datetime!(2026-01-01 00:00 +01:00);
    const DAY: time::Duration = time::Duration::DAY;

    /// The plain case, and the formula the schema states on `wandlerfaktor`:
    /// *"Mit diesem Faktor wird eine Zählerstandsdifferenz multipliziert."*
    #[test]
    fn consumption_is_the_difference_times_the_wandlerfaktor() {
        let zw = Zaehlwerk {
            wandlerfaktor: Some(Decimal::from(40)),
            ..register(vec![])
        };
        assert_eq!(
            zw.consumption_between(Decimal::from(1_000), Decimal::from(1_050)),
            Ok(Decimal::from(2_000))
        );
    }

    /// An absent `wandlerfaktor` is 1 — a directly-measuring meter has no
    /// transformer and states none.
    #[test]
    fn an_absent_wandlerfaktor_is_one() {
        let zw = register(vec![]);
        assert!(zw.wandlerfaktor.is_none());
        assert_eq!(
            zw.consumption_between(Decimal::from(1_000), Decimal::from(1_050)),
            Ok(Decimal::from(50))
        );
    }

    /// The trap: a six-digit register going `999998 → 000012` has consumed 14,
    /// not −999 986. `vorkommastelle` is what BO4E gives you to know that.
    #[test]
    fn a_wrapped_register_is_resolved_rather_than_going_negative() {
        let zw = register(vec![]);
        assert_eq!(zw.register_capacity(), Some(Decimal::from(1_000_000)));
        assert_eq!(
            zw.consumption_between(Decimal::from(999_998), Decimal::from(12)),
            Ok(Decimal::from(14))
        );

        // …and the naive subtraction, for contrast.
        assert_eq!(
            Decimal::from(12) - Decimal::from(999_998),
            Decimal::from(-999_986)
        );
    }

    /// Without a stated width there is no way to tell a wrap-around from a
    /// fault, so it is refused rather than guessed either way.
    #[test]
    fn a_fall_with_no_stated_width_is_refused() {
        let zw = Zaehlwerk {
            vorkommastelle: None,
            ..register(vec![])
        };
        assert_eq!(zw.register_capacity(), None);
        assert!(matches!(
            zw.consumption_between(Decimal::from(999_998), Decimal::from(12)),
            Err(ConsumptionError::DecreasedWithoutRegisterWidth { .. })
        ));
    }

    /// A fall larger than one whole revolution is not a wrap-around at all —
    /// adding the capacity back still leaves it negative, so it is refused.
    #[test]
    fn a_fall_larger_than_the_register_is_not_a_wrap_around() {
        // Capacity 1000; a fall of 1500 cannot be one revolution.
        let zw = Zaehlwerk {
            vorkommastelle: Some(3),
            ..register(vec![])
        };
        assert!(matches!(
            zw.consumption_between(Decimal::from(2_000), Decimal::from(500)),
            Err(ConsumptionError::DecreasedWithoutRegisterWidth { .. })
        ));
    }

    /// The sequence walk sorts, differences each consecutive pair, and sums.
    #[test]
    fn the_total_sums_every_consecutive_pair_in_time_order() {
        // Deliberately out of order: `messwerte` is a bag, not a sequence.
        let zw = register(vec![
            reading(T0 + DAY * 2, 1_250),
            reading(T0, 1_000),
            reading(T0 + DAY, 1_100),
        ]);
        assert_eq!(
            zw.readings().iter().map(|r| r.value).collect::<Vec<_>>(),
            [
                Decimal::from(1_000),
                Decimal::from(1_100),
                Decimal::from(1_250)
            ]
        );
        assert_eq!(zw.total_consumption(), Ok(Decimal::from(250)));
    }

    /// A meter exchange restarts the register from an unrelated state, so the
    /// difference across that boundary is not a consumption. Refused, not
    /// guessed — only the caller knows which meter the second half belongs to.
    #[test]
    fn a_meter_exchange_is_refused_rather_than_differenced_across() {
        let mut werte = vec![reading(T0, 998_000), reading(T0 + DAY, 40)];
        werte[1].messwertstatuszusatz = Some(Messwertstatuszusatz::Z78Geraetewechsel);

        let zw = register(werte);
        assert_eq!(
            zw.total_consumption(),
            Err(ConsumptionError::MeterExchange { index: 1 })
        );
        // The wrap-around arithmetic would otherwise have "explained" it.
        assert_eq!(
            zw.consumption_between(Decimal::from(998_000), Decimal::from(40)),
            Ok(Decimal::from(2_040)),
        );
    }

    /// A reading marked unusable is not a zero, so it is dropped from the walk
    /// rather than differenced against.
    #[test]
    fn an_unusable_reading_is_left_out_of_the_series() {
        let mut werte = vec![
            reading(T0, 1_000),
            reading(T0 + DAY, 0),
            reading(T0 + DAY * 2, 1_250),
        ];
        werte[1].messwertstatus = Some(Messwertstatus::Fehlt);

        let zw = register(werte);
        assert_eq!(zw.readings().len(), 2);
        assert_eq!(zw.total_consumption(), Ok(Decimal::from(250)));
    }

    #[test]
    fn a_single_reading_is_not_a_consumption() {
        assert_eq!(
            register(vec![reading(T0, 1_000)]).total_consumption(),
            Err(ConsumptionError::TooFewReadings { count: 1 })
        );
        assert_eq!(
            register(vec![]).total_consumption(),
            Err(ConsumptionError::TooFewReadings { count: 0 })
        );
    }

    /// A reading in MWh on a kWh register is brought onto the register's scale
    /// before it is differenced — otherwise `1.1 MWh − 1000 kWh` reads as `0.1`.
    #[test]
    fn a_reading_in_another_unit_is_converted_to_the_registers() {
        let mut werte = vec![reading(T0, 1_000), reading(T0 + DAY, 0)];
        werte[1].wert = Some(Menge {
            wert: Some(Decimal::new(11, 1)), // 1.1 MWh == 1100 kWh
            einheit: Some(Mengeneinheit::Mwh),
            ..Default::default()
        });

        let zw = register(werte);
        assert_eq!(
            zw.readings().iter().map(|r| r.value).collect::<Vec<_>>(),
            [Decimal::from(1_000), Decimal::from(1_100)]
        );
        assert_eq!(zw.total_consumption(), Ok(Decimal::from(100)));
    }

    /// A reading in a unit that does not convert would silently vanish from the
    /// total, leaving a number that spans a gap it does not admit to.
    #[test]
    fn a_reading_in_an_incompatible_unit_is_an_error_not_an_omission() {
        let mut werte = vec![
            reading(T0, 1_000),
            reading(T0 + DAY, 0),
            reading(T0 + DAY * 2, 1_250),
        ];
        werte[1].wert = Some(Menge {
            wert: Some(Decimal::from(5)),
            einheit: Some(Mengeneinheit::Kw), // power, not energy
            ..Default::default()
        });

        let zw = register(werte);
        assert_eq!(
            zw.total_consumption(),
            Err(ConsumptionError::IncompatibleUnit { index: 1 })
        );
    }

    /// End to end from the wire, which is the shape a `Zaehler` actually arrives
    /// in — a register nested two levels down.
    #[cfg(feature = "json")]
    #[test]
    fn a_register_read_off_the_wire_computes_its_consumption() {
        let body = r#"{
            "_typ": "ZAEHLWERK",
            "zaehlwerkId": "1",
            "einheit": "KWH",
            "vorkommastelle": 6,
            "nachkommastelle": 3,
            "wandlerfaktor": "40",
            "obisKennzahl": "1-0:1.8.0",
            "messwerte": [
                {"zeitpunkt":"2026-01-01T00:00:00+01:00","messwertstatus":"ABGELESEN",
                 "wert":{"wert":"999998","einheit":"KWH"}},
                {"zeitpunkt":"2026-02-01T00:00:00+01:00","messwertstatus":"ABGELESEN",
                 "wert":{"wert":"12","einheit":"KWH"}}
            ]
        }"#;
        let zw = Zaehlwerk::from_json_german(body).expect("valid Zaehlwerk JSON");

        assert_eq!(zw.readings().len(), 2);
        // Wrapped by 14 register steps, times a Wandlerfaktor of 40.
        assert_eq!(zw.total_consumption(), Ok(Decimal::from(560)));
    }
}
