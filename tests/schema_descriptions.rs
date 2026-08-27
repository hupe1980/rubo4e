//! No Rust-facing prose may reach a published schema description.
//!
//! Both derives lift a type's rustdoc into the JSON Schema / OpenAPI
//! `description`, so anything written for whoever maintains the Rust binding —
//! a codelist provenance note, an upstream-gap caveat, a "requires the `time`
//! feature" hint — ends up in a wire contract read by people who do not have a
//! Rust compiler.
//!
//! What the derived `Ord` means, for instance, is documented on the `Bo4eEnum`
//! trait rather than on each enum, for exactly this reason.
//!
//! `tests/identifier_schemas.rs` covers the identifier newtypes; this covers the
//! generated ones.

#![cfg(all(feature = "schemars", feature = "versioned"))]

use rubo4e::current::*;

/// Markers that only ever appear in prose written for a Rust reader.
const RUSTDOC_MARKERS: &[(&str, &str)] = &[
    ("```", "a code fence"),
    ("assert_eq!", "a doctest assertion"),
    ("\n# ", "a rustdoc section heading"),
    ("](crate::", "an intra-doc link"),
    ("](super::", "an intra-doc link"),
    ("Requires the `", "a Cargo-feature note"),
    ("feature for the", "a Cargo-feature note"),
];

fn description_of(schema: &serde_json::Value) -> String {
    schema
        .get("description")
        .and_then(|d| d.as_str())
        .unwrap_or_default()
        .to_owned()
}

fn assert_clean(what: &str, description: &str) {
    for (marker, kind) in RUSTDOC_MARKERS {
        assert!(
            !description.contains(marker),
            "{what}: the schema description contains {kind} — Rust prose is \
             leaking into a wire contract.\nGot: {description}"
        );
    }
}

macro_rules! schema_of {
    ($ty:ty) => {
        serde_json::to_value(schemars::schema_for!($ty)).expect("schema is JSON")
    };
}

/// The three enums that carry a curated maintainer note. Each keeps the note in
/// its rustdoc and pins the schema's own sentence with
/// `#[schemars(description = …)]`.
#[test]
fn a_curated_enum_note_stays_out_of_the_schema() {
    for (name, schema, expected) in [
        (
            "BdewArtikelnummer",
            schema_of!(BdewArtikelnummer),
            "BDEW Artikelnummern",
        ),
        (
            "Gasqualitaet",
            schema_of!(Gasqualitaet),
            "Unterscheidung für hoch- und niedrig-kalorisches Gas.",
        ),
        (
            "Rechnungstyp",
            schema_of!(Rechnungstyp),
            "Abbildung verschiedener Rechnungstypen zur Kennzeichnung von Rechnungen",
        ),
    ] {
        let description = description_of(&schema);
        assert_clean(name, &description);
        assert_eq!(
            description, expected,
            "{name}: the schema must carry BO4E's own sentence, not the rustdoc"
        );
    }
}

/// An enum with no curated note needs no override, and must still be clean — so
/// this fails if a future note is added without the accompanying attribute.
#[test]
fn an_ordinary_enum_carries_its_bo4e_description_unchanged() {
    for (name, schema) in [
        ("Sparte", schema_of!(Sparte)),
        ("Zaehlertyp", schema_of!(Zaehlertyp)),
        ("Messwertstatus", schema_of!(Messwertstatus)),
        ("Mengeneinheit", schema_of!(Mengeneinheit)),
    ] {
        assert_clean(name, &description_of(&schema));
    }
}

/// Every property of a representative BO, at every depth. A struct's field docs
/// come from BO4E, so this is mostly a floor — but it is the floor that catches a
/// feature note or an intra-doc link added to a generated field.
#[test]
fn no_property_description_anywhere_carries_rust_prose() {
    let root = schema_of!(Marktlokation);
    let mut checked = 0usize;
    walk(&root, "Marktlokation", &mut checked);
    assert!(
        checked > 50,
        "only {checked} descriptions were reached — the walk is not finding them"
    );

    fn walk(node: &serde_json::Value, path: &str, checked: &mut usize) {
        if let Some(description) = node.get("description").and_then(|d| d.as_str()) {
            assert_clean(path, description);
            *checked += 1;
        }
        for key in ["properties", "$defs", "definitions"] {
            if let Some(map) = node.get(key).and_then(|v| v.as_object()) {
                for (name, child) in map {
                    walk(child, &format!("{path}.{name}"), checked);
                }
            }
        }
        for key in ["items", "additionalProperties"] {
            if let Some(child) = node.get(key) {
                walk(child, path, checked);
            }
        }
        for key in ["anyOf", "oneOf", "allOf"] {
            if let Some(list) = node.get(key).and_then(|v| v.as_array()) {
                for child in list {
                    walk(child, path, checked);
                }
            }
        }
    }
}

/// The half of the problem that only exists with `time` and `decimal` **off**.
///
/// A field whose type changes with a feature is generated twice, and both
/// declarations must carry BO4E's description — documenting the fallback with
/// "requires the `time` feature" instead is what the schema would then publish,
/// in place of the field's meaning.
///
/// This runs only in the configuration where the fallback is the compiled one;
/// `just test-schema-fallback` is the recipe that gets there.
#[cfg(all(not(feature = "time"), not(feature = "decimal")))]
#[test]
fn a_feature_fallback_field_keeps_the_schemas_description() {
    let schema = schema_of!(Zeitraum);
    let properties = schema
        .get("properties")
        .and_then(|p| p.as_object())
        .expect("Zeitraum has properties");

    for (name, expected) in [
        ("enddatum", "Enddatum des betrachteten Zeitraums"),
        ("startdatum", "Startdatum des betrachteten Zeitraums"),
    ] {
        let description = description_of(properties.get(name).expect("field present"));
        assert_clean(name, &description);
        assert!(
            description.contains(expected),
            "{name}: the fallback lost BO4E's description.\nGot: {description}"
        );
    }
}
