//! Drift guard: the interval conventions this crate implements must be the ones
//! the BO4E schema states.
//!
//! BO4E writes the inclusivity of a boundary into the field's own `description`,
//! in bold: *"Enddatum des betrachteten Zeitraums ist **inklusiv**"*. It is not
//! uniform — one release uses three conventions — and getting one wrong is an
//! off-by-one nobody notices until an invoice is short a day:
//!
//! | Kind | Convention |
//! |---|---|
//! | `date-time` pairs (`vertragsbeginn`/`vertragsende`, `von`/`bis`) | `[start, end)` |
//! | `Zeitraum`'s **date** pair | `[start, end]` |
//! | `Zeitraum`'s **time** pair | `[start, end)` |
//! | `Zeitraum`'s **instant** pair (all four fields) | `[start, end)` |
//! | decimal bounds (`staffelgrenzeVon`/`Bis`) | `[von, bis]` |
//!
//! These read each statement out of the committed schema and check it against
//! what the code does, so a release that changes a convention fails here rather
//! than in production.

#![cfg(all(feature = "versioned", feature = "time"))]

use std::path::PathBuf;

/// The committed schema snapshot for the `current` series.
fn schema_dir() -> PathBuf {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("generator/schemas");
    let mut matches: Vec<PathBuf> = std::fs::read_dir(&root)
        .unwrap_or_else(|e| panic!("reading {}: {e}", root.display()))
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| {
            p.is_dir()
                && p.file_name()
                    .and_then(|n| n.to_str())
                    .is_some_and(|n| n.starts_with("v202607"))
        })
        .collect();
    matches.sort();
    assert_eq!(
        matches.len(),
        1,
        "expected one v202607 snapshot: {matches:?}"
    );
    matches.pop().expect("checked len")
}

/// What the schema says about one property's boundary, as a `(inclusive?)` flag.
///
/// `None` when the description says nothing either way.
fn stated_inclusivity(rel_path: &str, property: &str) -> Option<bool> {
    let path = schema_dir().join(rel_path);
    let raw = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("reading {}: {e}", path.display()));
    let doc: serde_json::Value = serde_json::from_str(&raw).expect("valid schema JSON");
    let desc = doc
        .get("properties")?
        .get(property)?
        .get("description")?
        .as_str()?
        .to_lowercase();

    // BO4E marks it up as RST bold; match the word, not the markup.
    match (desc.contains("inklusiv"), desc.contains("exklusiv")) {
        (true, false) => Some(true),
        (false, true) => Some(false),
        // Both or neither: the guard cannot decide, and neither could a reader.
        _ => None,
    }
}

fn require_stated(rel_path: &str, property: &str) -> bool {
    stated_inclusivity(rel_path, property).unwrap_or_else(|| {
        panic!(
            "{rel_path}#{property} no longer states its inclusivity — the convention \
             this crate implements can no longer be checked against the schema. \
             Re-read the field and update `src/convenience.rs`."
        )
    })
}

// ─── Zeitraum: the date pair is closed ───────────────────────────────────────

/// The schema statement itself. If BO4E flips it, everything below is wrong, and
/// this says so first.
#[test]
fn the_schema_still_declares_both_zeitraum_dates_inclusive() {
    assert!(
        require_stated("com/Zeitraum.json", "startdatum"),
        "startdatum is documented inclusive"
    );
    assert!(
        require_stated("com/Zeitraum.json", "enddatum"),
        "enddatum is documented inclusive — if this now reads exclusive, \
         Zeitraum::contains and whole_days must change with it"
    );
}

/// …and the implementation agrees with the statement.
#[test]
fn zeitraum_contains_matches_the_stated_inclusivity() {
    use rubo4e::current::Zeitraum;
    use time::macros::date;

    let start = date!(2026 - 01 - 01);
    let end = date!(2026 - 01 - 31);
    let z = Zeitraum {
        startdatum: Some(start),
        enddatum: Some(end),
        ..Default::default()
    };

    assert_eq!(
        z.contains(start),
        require_stated("com/Zeitraum.json", "startdatum"),
        "contains() disagrees with the schema about startdatum"
    );
    assert_eq!(
        z.contains(end),
        require_stated("com/Zeitraum.json", "enddatum"),
        "contains() disagrees with the schema about enddatum"
    );
}

/// The day count follows from the same statement: with both bounds inclusive,
/// a period spanning `n` calendar days reports `n`.
#[test]
fn whole_days_follows_from_the_stated_inclusivity() {
    use rubo4e::current::Zeitraum;
    use time::macros::date;

    let start = date!(2026 - 01 - 01);
    let end = date!(2026 - 01 - 31);
    let raw = (end - start).whole_days();
    let expected = raw
        + i64::from(require_stated("com/Zeitraum.json", "startdatum"))
            * i64::from(require_stated("com/Zeitraum.json", "enddatum"));

    let z = Zeitraum {
        startdatum: Some(start),
        enddatum: Some(end),
        ..Default::default()
    };
    assert_eq!(z.whole_days(), Some(expected));
    assert_eq!(expected, 31, "January has 31 days");
}

// ─── Zeitraum: the time pair is *not* closed ─────────────────────────────────

/// The same COM uses the opposite convention for its time-of-day pair. Nothing
/// in the crate reads those fields as an interval today — they are `String`,
/// because they carry a UTC offset and `time` has no offset-bearing time type —
/// but the asymmetry is documented, so it is pinned.
#[test]
fn the_zeitraum_time_pair_is_half_open_unlike_its_date_pair() {
    assert!(
        require_stated("com/Zeitraum.json", "startuhrzeit"),
        "startuhrzeit is inclusive"
    );
    assert!(
        !require_stated("com/Zeitraum.json", "enduhrzeit"),
        "enduhrzeit is EXclusive — the opposite of enddatum on the same type"
    );
}

/// `Zeitraum`'s third mode joins a date and a time of day into an instant, and
/// it inherits the **time** pair's convention rather than the date pair's:
/// `startuhrzeit` inclusive, `enduhrzeit` exclusive, so the period is
/// `[start, end)` and consecutive quarter-hours abut without overlapping.
///
/// This is what `as_instant_range` returns and what the date accessors on the
/// same value do *not* say, so the two conventions have to stay pinned apart.
#[test]
fn the_zeitraum_instant_range_is_half_open() {
    use rubo4e::current::Zeitraum;
    use time::macros::datetime;

    // The schema statements the range's shape is derived from.
    assert!(require_stated("com/Zeitraum.json", "startuhrzeit"));
    assert!(!require_stated("com/Zeitraum.json", "enduhrzeit"));

    let start = datetime!(2026-01-01 00:00 +01:00);
    let end = start + time::Duration::minutes(15);
    let slot = Zeitraum::from_instants(start, end);

    let range = slot
        .as_instant_range()
        .expect("all four fields present")
        .expect("both ends parse");
    assert_eq!(range, start..end);
    assert!(slot.contains_instant(start), "start is inside");
    assert!(!slot.contains_instant(end), "end is outside");
    assert_eq!(
        slot.instant_duration(),
        Some(Ok(time::Duration::minutes(15)))
    );

    // Consecutive slots abut: the boundary belongs to exactly one of them.
    let next = Zeitraum::from_instants(end, end + time::Duration::minutes(15));
    assert!(next.contains_instant(end));
    assert!(!slot.contains_instant(end));

    // The date pair on the very same value still answers about whole days —
    // which is why routing on `is_instant_range` matters.
    assert!(slot.is_instant_range());
    assert_eq!(slot.whole_days(), Some(1));
    assert!(slot.contains(time::macros::date!(2026 - 01 - 01)));
}

/// The two open ends of `contains_instant` are not the same thing: an *absent*
/// boundary is unbounded, a *malformed* one is unreadable. Admitting a record
/// whose bound you could not parse is the wrong direction for a filter.
#[test]
fn contains_instant_treats_an_absent_bound_as_open_and_a_broken_one_as_no() {
    use rubo4e::current::Zeitraum;
    use time::macros::{date, datetime};

    let t = datetime!(2026-01-01 12:00 +01:00);

    // Absent on both sides: unbounded, and contains everything.
    assert!(Zeitraum::default().contains_instant(t));

    // Absent on one side: open in that direction only.
    let from = Zeitraum {
        startdatum: Some(date!(2026 - 01 - 01)),
        startuhrzeit: Some("00:00:00+01:00".into()),
        ..Default::default()
    };
    assert!(from.contains_instant(t));
    assert!(!from.contains_instant(datetime!(2025-12-31 23:00 +01:00)));

    // Malformed: not open, not contained. `as_instant_range` still reports why.
    let broken = Zeitraum {
        startuhrzeit: Some("not a time".into()),
        ..from.clone()
    };
    assert!(!broken.contains_instant(t));
    assert!(broken.start_instant().is_some_and(|r| r.is_err()));

    // A time of day with no offset is malformed for this purpose too: it names a
    // wall-clock reading, not a moment.
    let no_offset = Zeitraum {
        startuhrzeit: Some("00:00:00".into()),
        ..from
    };
    assert!(!no_offset.contains_instant(t));
}

/// A `Zeitraum` whose instants collapse to a point encloses no time, because the
/// end is exclusive — so `.validate()` rejects it, the way it does for the
/// half-open `date-time` pairs elsewhere in the schema.
#[test]
fn validate_zeitraum_rejects_an_empty_instant_range() {
    #![cfg(feature = "validate")]
    use garde::Validate as _;
    use rubo4e::current::Zeitraum;
    use time::macros::datetime;

    let instant = datetime!(2026-01-01 00:00 +01:00);
    assert!(
        Zeitraum::from_instants(instant, instant)
            .validate()
            .is_err(),
        "start == end encloses no time when the end is exclusive"
    );
    assert!(
        Zeitraum::from_instants(instant, instant + time::Duration::nanoseconds(1))
            .validate()
            .is_ok(),
    );

    // The *date* pair keeps the opposite rule on the same type: both bounds are
    // inclusive, so a one-day period is start == end and stays valid.
    let one_day = Zeitraum {
        startdatum: Some(time::macros::date!(2026 - 03 - 15)),
        enddatum: Some(time::macros::date!(2026 - 03 - 15)),
        ..Default::default()
    };
    assert!(one_day.validate().is_ok());
}

// ─── Timestamp pairs are half-open ───────────────────────────────────────────

/// `date-time` pairs across the schema are `[begin, end)`, which is why
/// `validate_vertrag_dates` requires a strict `<`: a contract that begins and
/// ends at the same instant covers no time at all.
#[test]
fn timestamp_pairs_are_half_open() {
    for (file, begin, end) in [
        ("bo/Vertrag.json", "vertragsbeginn", "vertragsende"),
        ("bo/Buendelvertrag.json", "vertragsbeginn", "vertragsende"),
        (
            "com/Vertragsteil.json",
            "vertragsteilbeginn",
            "vertragsteilende",
        ),
        ("com/Kostenposition.json", "von", "bis"),
        ("com/Fremdkostenposition.json", "von", "bis"),
    ] {
        assert!(require_stated(file, begin), "{file}#{begin} is inclusive");
        assert!(!require_stated(file, end), "{file}#{end} is exclusive");
    }
}

/// The matching invariant: a zero-length half-open interval is empty, so the
/// validator rejects it.
#[test]
fn validate_vertrag_dates_rejects_a_zero_length_interval() {
    #![cfg(feature = "validate")]
    use garde::Validate as _;
    use rubo4e::current::Vertrag;
    use time::macros::datetime;

    let instant = datetime!(2026-01-01 00:00 UTC);
    let v = Vertrag {
        vertragsbeginn: Some(instant),
        vertragsende: Some(instant),
        ..Default::default()
    };
    assert!(
        v.validate().is_err(),
        "[t, t) is empty — a half-open interval must have begin < end"
    );
}

// ─── Price tiers: the schema contradicts itself, the example decides ─────────

/// `staffelgrenzeBis` reads *"**Exklusiver** oberer Wert, bis zu dem die Staffel
/// gilt (**inklusiv**)"* — both words, in one sentence, about one bound. The
/// word-matching helper above deliberately refuses to guess, so this is asserted
/// directly: it is the schema that is ambiguous, not the reader.
#[test]
fn the_price_tier_upper_bound_is_stated_both_ways() {
    assert!(
        require_stated("com/Preisstaffel.json", "staffelgrenzeVon"),
        "staffelgrenzeVon is stated inclusive, unambiguously"
    );
    assert_eq!(
        stated_inclusivity("com/Preisstaffel.json", "staffelgrenzeBis"),
        None,
        "staffelgrenzeBis says both 'Exklusiver' and '(inklusiv)'. If BO4E has \
         since settled on one, drop this test and pin the answer instead."
    );
}

/// What settles it is the worked example in the same description: tiers are
/// given as `0 – 1000, 1001 – 2000`, and *"Werte zwischen den Grenzen (z.B.
/// `1000,6`) rutschen in die obere Zone / Staffel"*.
///
/// So `1000` is inside the first tier — the upper bound is inclusive — and a
/// value in the gap between tiers belongs to the tier **above** it. Both follow
/// from the example, and both are what the crate implements.
#[test]
#[cfg(feature = "decimal")]
fn price_tier_selection_follows_the_schemas_worked_example() {
    use rubo4e::convenience::PreisstaffelSliceExt;
    use rubo4e::current::Preisstaffel;
    use rust_decimal::Decimal;

    // The example still says what this test rests on.
    let desc = {
        let path = schema_dir().join("com/Preisstaffel.json");
        let doc: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(path).expect("readable"))
                .expect("valid JSON");
        doc["properties"]["staffelgrenzeBis"]["description"]
            .as_str()
            .expect("described")
            .to_owned()
    };
    assert!(
        desc.contains("0 - 1000, 1001 - 2000") && desc.contains("obere Zone"),
        "the worked example this behaviour is derived from is gone: {desc}"
    );

    fn tier(von: i64, bis: i64, preis: i64) -> Preisstaffel {
        Preisstaffel {
            staffelgrenze_von: Some(Decimal::from(von)),
            staffelgrenze_bis: Some(Decimal::from(bis)),
            preis: Some(Decimal::from(preis)),
            ..Default::default()
        }
    }
    let staffeln = [tier(0, 1000, 30), tier(1001, 2000, 25)];
    let picked = |v: Decimal| staffeln.select_for(v).and_then(|s| s.preis);

    // The upper bound is inside its own tier.
    assert_eq!(picked(Decimal::from(1000)), Some(Decimal::from(30)));
    assert_eq!(picked(Decimal::ZERO), Some(Decimal::from(30)));
    // The schema's own gap value goes to the tier above.
    assert_eq!(picked(Decimal::new(10006, 1)), Some(Decimal::from(25)));
    assert_eq!(picked(Decimal::from(1001)), Some(Decimal::from(25)));
    assert_eq!(picked(Decimal::from(2000)), Some(Decimal::from(25)));
    // Outside the stated range entirely.
    assert!(staffeln.select_for(Decimal::from(2001)).is_none());
    assert!(staffeln.select_for(Decimal::from(-1)).is_none());
}

/// Tier order in the slice must not matter, and an open-topped final tier must
/// catch everything past the last stated bound.
#[test]
#[cfg(feature = "decimal")]
fn tier_selection_is_order_independent_and_handles_an_open_top() {
    use rubo4e::convenience::PreisstaffelSliceExt;
    use rubo4e::current::Preisstaffel;
    use rust_decimal::Decimal;

    fn tier(von: Option<i64>, bis: Option<i64>, preis: i64) -> Preisstaffel {
        Preisstaffel {
            staffelgrenze_von: von.map(Decimal::from),
            staffelgrenze_bis: bis.map(Decimal::from),
            preis: Some(Decimal::from(preis)),
            ..Default::default()
        }
    }

    let ascending = [
        tier(Some(0), Some(1000), 30),
        tier(Some(1001), None, 25), // open-topped
    ];
    let descending = [ascending[1].clone(), ascending[0].clone()];

    for staffeln in [&ascending, &descending] {
        let picked = |v: i64| staffeln.select_for(Decimal::from(v)).and_then(|s| s.preis);
        assert_eq!(picked(500), Some(Decimal::from(30)));
        assert_eq!(picked(1000), Some(Decimal::from(30)));
        assert_eq!(
            picked(999_999),
            Some(Decimal::from(25)),
            "an absent staffelgrenzeBis is unbounded above"
        );
    }

    // An empty tier list selects nothing.
    assert!(([] as [Preisstaffel; 0]).select_for(Decimal::ONE).is_none());
}
