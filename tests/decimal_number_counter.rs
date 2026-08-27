//! Exact-delta assertions on `decimal_from_json_number_count()`.
//!
//! The counter is process-wide and monotonic, so any test binary where something
//! else also deserializes a decimal can only assert that it *moved*. This file
//! holds one test, so nothing else in its process touches the atomic and the
//! deltas are exact.
//!
//! What it pins is the counter's contract: it measures the **spelling** a
//! producer used, not whether digits were lost. An integer JSON number is exact
//! and still counted, because Go marshals a whole amount as `119` — so a counter
//! that only saw the fractional path would read zero against the very producer
//! it exists to identify.

#![cfg(feature = "serde")]

use rubo4e::decimal_serde::{decimal_from_json_number_count, deserialize_opt};

#[derive(serde::Deserialize)]
struct Holder {
    #[serde(default, deserialize_with = "deserialize_opt")]
    #[allow(dead_code)]
    wert: Option<Wert>,
}

#[cfg(feature = "decimal")]
type Wert = rust_decimal::Decimal;
#[cfg(not(feature = "decimal"))]
type Wert = String;

fn read(json: &str) {
    serde_json::from_str::<Holder>(json).expect("valid");
}

#[test]
fn the_counter_measures_the_spelling_not_the_damage() {
    let mut expected = decimal_from_json_number_count();

    // A JSON string is the exact spelling, and is never counted.
    for body in [
        r#"{"wert":"119.00"}"#,
        r#"{"wert":"0.1"}"#,
        r#"{"wert":"1"}"#,
    ] {
        read(body);
        assert_eq!(
            decimal_from_json_number_count(),
            expected,
            "a JSON string must not be counted: {body}"
        );
    }

    // `null` and an absent field are not reads at all.
    for body in [r#"{"wert":null}"#, "{}"] {
        read(body);
        assert_eq!(decimal_from_json_number_count(), expected, "{body}");
    }

    // Every number shape counts, integer and fractional alike.
    for body in [
        r#"{"wert":119}"#,              // Go's whole amount — exact, still a number
        r#"{"wert":-42}"#,              // i64 path
        r#"{"wert":9007199254740993}"#, // past 2^53: exact, and never f64
        r#"{"wert":119.00}"#,           // the lossy path: scale already gone
        r#"{"wert":0.1}"#,
    ] {
        expected += 1;
        read(body);
        assert_eq!(
            decimal_from_json_number_count(),
            expected,
            "a JSON number must be counted exactly once: {body}"
        );
    }
}
