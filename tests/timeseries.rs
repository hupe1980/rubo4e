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
