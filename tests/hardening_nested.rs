//! Regression tests: the hardened parse limits must apply at *every* nesting
//! level, not only to the root object.
#![cfg(all(feature = "versioned", feature = "json"))]

use rubo4e::current::Marktlokation;
use rubo4e::json::{Bo4eJsonExt, JsonParseLimits};

/// Extension data hidden inside a nested COM must be charged to the same
/// per-call budget as extension data on the root.
#[test]
fn nested_extension_value_bytes_are_charged() {
    let junk = "x".repeat(50_000);
    let json = format!(
        r#"{{"_typ":"MARKTLOKATION","lokationsadresse":{{"ort":"Bremen","attack":"{junk}"}}}}"#
    );
    let err = Marktlokation::from_json_german_hardened(
        &json,
        JsonParseLimits {
            max_extension_value_bytes: Some(16),
            ..JsonParseLimits::unlimited()
        },
    )
    .expect_err("nested extension payload must exhaust the budget");
    assert!(
        err.to_string().contains("extension value budget exceeded"),
        "unexpected error: {err}"
    );
}

/// The per-struct field-count cap must also bind on nested structs.
#[test]
fn nested_extension_field_count_is_capped() {
    let json = r#"{"_typ":"MARKTLOKATION","lokationsadresse":{"ort":"Bremen","a":1,"b":2,"c":3}}"#;
    let err = Marktlokation::from_json_german_hardened(
        json,
        JsonParseLimits {
            max_extension_field_count: Some(2),
            ..JsonParseLimits::unlimited()
        },
    )
    .expect_err("nested struct exceeds the per-struct field cap");
    assert!(
        err.to_string().contains("extension field count"),
        "unexpected error: {err}"
    );
}

/// The byte budget is cumulative across the whole payload, so extension data
/// spread thinly over many structs still adds up.
#[test]
fn extension_budget_is_cumulative_across_structs() {
    let json = r#"{"_typ":"MARKTLOKATION","rootExtra":"aaaaaaaaaa",
                   "lokationsadresse":{"ort":"Bremen","nestedExtra":"bbbbbbbbbb"}}"#;
    // Each field costs key.len() + value.len(); either alone fits in 40 bytes,
    // but together they must not.
    let err = Marktlokation::from_json_german_hardened(
        json,
        JsonParseLimits {
            max_extension_value_bytes: Some(30),
            ..JsonParseLimits::unlimited()
        },
    )
    .expect_err("cumulative budget must be exhausted");
    assert!(
        err.to_string().contains("extension value budget exceeded"),
        "unexpected error: {err}"
    );
}

/// A payload comfortably inside the limits still parses, and the budget does
/// not leak into a later unhardened call.
#[test]
fn within_limits_parses_and_budget_does_not_leak() {
    let json = r#"{"_typ":"MARKTLOKATION","lokationsadresse":{"ort":"Bremen","x":"y"}}"#;
    let malo =
        Marktlokation::from_json_german_hardened(json, JsonParseLimits::untrusted_defaults())
            .expect("payload is well within the untrusted defaults");
    assert!(malo.lokationsadresse.is_some());

    // The guard must have restored the previous (absent) budget: a large
    // extension payload parsed *without* hardening must still succeed.
    let junk = "x".repeat(100_000);
    let big = format!(r#"{{"_typ":"MARKTLOKATION","attack":"{junk}"}}"#);
    Marktlokation::from_json_german(&big).expect("unhardened parse is unbudgeted");
}
