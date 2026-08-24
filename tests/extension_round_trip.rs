//! Extension data must survive a snake_case round-trip byte-for-byte.
//!
//! Keys are renamed as the parser yields them, before serde knows the struct, so
//! an unscoped transform renames every key at every depth — including inside a
//! vendor blob, where `{"a": 3}` would become `{"A": 3}` because `A` is a BO4E
//! field name somewhere.
//!
//! These pin the scoping rule — the transform descends only under keys the
//! schema defines — and the two ambiguities scoping cannot fix.

#![cfg(all(feature = "versioned", feature = "json"))]

use rubo4e::current::{Adresse, Marktlokation};
use rubo4e::json::{Bo4eExtensionData, Bo4eJsonExt};
use serde_json::json;

fn malo(body: &str) -> Marktlokation {
    Marktlokation::from_json_german(body).expect("valid BO4E payload")
}

/// Single-letter keys are the sharp case: `A`–`D` are `Sigmoidparameter`'s own
/// field names, and `{"a": …}` is about as common as JSON gets.
#[test]
fn single_letter_keys_inside_extension_data_are_not_renamed() {
    let m = malo(r#"{"_typ":"MARKTLOKATION","custom":{"a":1,"b":2,"c":3,"d":4}}"#);
    let snake = m.to_json_snake_case().expect("serialize");
    assert!(
        snake.contains(r#""a":1"#) && !snake.contains(r#""A":1"#),
        "extension keys were renamed on the way out: {snake}"
    );

    let back = Marktlokation::from_json_snake_case(&snake).expect("deserialize");
    assert_eq!(
        back.extension_data().get("custom"),
        Some(&json!({"a":1,"b":2,"c":3,"d":4})),
        "extension keys were renamed on the way back in"
    );
    assert_eq!(back, m, "the round-trip must be an identity");
}

/// A snake_case *field name* nested inside extension data is still the
/// producer's key, not a BO4E field, and must not be rewritten.
#[test]
fn a_field_name_nested_inside_extension_data_is_left_alone() {
    let m = malo(r#"{"_typ":"MARKTLOKATION","custom":{"marktlokations_id":"not-an-id"}}"#);
    let snake = m.to_json_snake_case().expect("serialize");
    let back = Marktlokation::from_json_snake_case(&snake).expect("deserialize");

    assert_eq!(
        back.extension_data().get("custom"),
        Some(&json!({"marktlokations_id": "not-an-id"})),
    );
    // …and it never reached the typed field, which would have rejected it.
    assert!(back.marktlokations_id.is_none());
}

/// Arrays under an extension key are covered too — the transform stays off for
/// the whole subtree, not just the first object.
#[test]
fn extension_arrays_and_deep_nesting_are_left_alone() {
    let m = malo(r#"{"_typ":"MARKTLOKATION","custom":[{"a":{"b":{"c":1}}},{"a":2}]}"#);
    let back = Marktlokation::from_json_snake_case(&m.to_json_snake_case().expect("serialize"))
        .expect("deserialize");
    assert_eq!(
        back.extension_data().get("custom"),
        Some(&json!([{"a":{"b":{"c":1}}},{"a":2}])),
    );
}

/// The other half of the rule: keys the schema *does* define must still be
/// renamed, at every depth. Scoping the transform must not have turned it off.
#[test]
fn schema_keys_are_still_renamed_at_every_depth() {
    let m = malo(
        r#"{"_typ":"MARKTLOKATION","marktlokationsId":"51238696781",
            "lokationsadresse":{"coErgaenzung":"c/o Muster","ort":"Bremen"}}"#,
    );
    let snake = m.to_json_snake_case().expect("serialize");
    assert!(
        snake.contains(r#""marktlokations_id""#),
        "top-level schema key was not renamed: {snake}"
    );
    assert!(
        snake.contains(r#""co_ergaenzung""#),
        "nested schema key was not renamed: {snake}"
    );

    let back = Marktlokation::from_json_snake_case(&snake).expect("deserialize");
    assert_eq!(
        back.lokationsadresse
            .as_ref()
            .and_then(|a| a.co_ergaenzung.as_deref()),
        Some("c/o Muster"),
        "nested schema key did not map back to its field"
    );
    assert_eq!(back, m);
}

/// Extension data mixed with schema fields on the same object: each key is
/// judged on its own, so one does not switch the other off.
#[test]
fn extension_and_schema_keys_coexist_on_one_object() {
    let m = malo(
        r#"{"_typ":"MARKTLOKATION","marktlokationsId":"51238696781",
            "vendorBlob":{"a":1},"lokationsadresse":{"coErgaenzung":"x"}}"#,
    );
    let back = Marktlokation::from_json_snake_case(&m.to_json_snake_case().expect("serialize"))
        .expect("deserialize");
    assert_eq!(back, m);
    assert_eq!(
        back.extension_data().get("vendorBlob"),
        Some(&json!({"a":1}))
    );
}

/// The German mode never renames anything, so it is unaffected either way —
/// and is the mode to use when extension data is in play.
#[test]
fn the_german_mode_round_trips_extension_data_unchanged() {
    let body = r#"{"_typ":"MARKTLOKATION","custom":{"a":1,"marktlokations_id":"x"}}"#;
    let m = malo(body);
    let back = Marktlokation::from_json_german(&m.to_json_german().expect("serialize"))
        .expect("deserialize");
    assert_eq!(back, m);
    assert_eq!(
        back.extension_data().get("custom"),
        Some(&json!({"a":1,"marktlokations_id":"x"})),
    );
}

// ─── Documented limitations ──────────────────────────────────────────────────

/// A *top-level* extension key that is a schema field's snake spelling is
/// genuinely ambiguous: once written in snake form there is nothing to tell it
/// apart from the field itself. Pinned so the behaviour is a known limitation
/// rather than a surprise — see the `json` module docs.
#[test]
fn a_top_level_extension_key_colliding_with_a_field_is_ambiguous() {
    let m = malo(r#"{"_typ":"MARKTLOKATION","marktlokations_id":"not-an-id"}"#);
    assert_eq!(
        m.extension_data().get("marktlokations_id"),
        Some(&json!("not-an-id")),
        "in German mode it is unambiguously extension data"
    );

    // In snake mode the same key is indistinguishable from the real field, and
    // the value fails the MaLo-ID check rather than landing in `_additional`.
    let err = Marktlokation::from_json_snake_case(&m.to_json_snake_case().expect("serialize"))
        .expect_err("the collision is reinterpreted as the typed field");
    assert!(
        err.to_string().contains("invalid length"),
        "unexpected error: {err}"
    );
}

/// A COM round-trips its own extension data on the same terms.
#[test]
fn a_com_round_trips_extension_data_too() {
    let a: Adresse =
        Adresse::from_json_german(r#"{"ort":"Bremen","vendor":{"a":1,"b":2}}"#).expect("valid");
    let back = Adresse::from_json_snake_case(&a.to_json_snake_case().expect("serialize"))
        .expect("deserialize");
    assert_eq!(back, a);
    assert_eq!(
        back.extension_data().get("vendor"),
        Some(&json!({"a":1,"b":2}))
    );
}
