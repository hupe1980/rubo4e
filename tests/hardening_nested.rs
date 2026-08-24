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
        JsonParseLimits::unlimited().with_max_extension_value_bytes(Some(16)),
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
        JsonParseLimits::unlimited().with_max_extension_field_count(Some(2)),
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
        JsonParseLimits::unlimited().with_max_extension_value_bytes(Some(30)),
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

// ─── str / bytes parity on a large payload ───────────────────────────────────

/// The `&str` and `&[u8]` entry points must agree on a payload far larger than
/// the few hundred bytes every other test here uses.
///
/// Checks the parts most likely to diverge: `#[serde(flatten)]` extension
/// capture, validating identifier newtypes, and nested COMs.
#[test]
fn large_payloads_decode_identically_on_str_and_bytes() {
    let padding = "p".repeat(4096);
    let json = format!(
        r#"{{"_typ":"MARKTLOKATION","_version":"202607.0.0",
             "marktlokationsId":"51238696781","sparte":"STROM",
             "lokationsadresse":{{"_typ":"ADRESSE","ort":"Bremen","postleitzahl":"28195"}},
             "vendorPadding":"{padding}"}}"#
    );
    assert!(json.len() > 4096, "payload must be large");

    let from_str = Marktlokation::from_json_german(&json).expect("str path parses");
    let from_bytes =
        Marktlokation::from_json_german_bytes(json.as_bytes()).expect("bytes path parses");

    assert_eq!(from_str, from_bytes, "the two entry points disagree");

    // The parts most likely to diverge.
    assert_eq!(
        from_str.marktlokations_id.as_ref().map(|id| id.as_ref()),
        Some("51238696781"),
        "identifier newtype must still validate on the large-payload path"
    );
    assert_eq!(
        from_str
            .lokationsadresse
            .as_ref()
            .and_then(|a| a.ort.as_deref()),
        Some("Bremen")
    );
    assert!(
        rubo4e::json::Bo4eExtensionData::extension_data(&from_str).contains_key("vendorPadding"),
        "flattened extension capture must survive the large-payload path"
    );

    // And the whole thing round-trips.
    let out = from_str.to_json_german().expect("serialize");
    assert_eq!(
        Marktlokation::from_json_german(&out).expect("re-parse"),
        from_str
    );
}

/// A pathologically deep payload is rejected on both entry points, even without
/// a hardened call — the default 128-level cap applies everywhere.
///
/// The wording is not asserted: `serde_json`'s own recursion limit and this
/// crate's `DepthLimitedDeserializer` both cap at 128, and whichever reports
/// first is an implementation detail. What matters is that the document never
/// reaches the deserializer.
#[test]
fn deeply_nested_large_payloads_are_rejected() {
    // 200 nested arrays, well past the 128-level default.
    let deep = format!("{}{}", "[".repeat(200), "]".repeat(200));
    let json = format!(
        r#"{{"_typ":"MARKTLOKATION","vendorPadding":"{}","deep":{deep}}}"#,
        "p".repeat(4096)
    );
    let err = Marktlokation::from_json_german(&json)
        .expect_err("200 levels must exceed the default cap of 128");
    let msg = err.to_string();
    assert!(
        msg.contains("nesting depth") || msg.contains("recursion limit"),
        "expected a depth rejection, got: {msg}"
    );
}

/// A hardened call with no extension limits must not inherit an outer call's
/// remaining allowance — the guard always shadows, even when it constrains
/// nothing.
#[test]
fn unlimited_hardened_call_does_not_inherit_an_outer_budget() {
    let junk = "x".repeat(100_000);
    let big = format!(r#"{{"_typ":"MARKTLOKATION","attack":"{junk}"}}"#);

    // Explicitly unlimited: the same payload that a tight budget rejects.
    Marktlokation::from_json_german_hardened(&big, JsonParseLimits::unlimited())
        .expect("an explicitly unlimited call is unbudgeted");

    // And the tight budget still bites when it is the one asked for.
    let err = Marktlokation::from_json_german_hardened(
        &big,
        JsonParseLimits::unlimited().with_max_extension_value_bytes(Some(16)),
    )
    .expect_err("a tight budget must still reject it");
    assert!(err.to_string().contains("extension value budget exceeded"));
}
