//! The recursive extension-field check, and the trap it exists to close.
//!
//! A decode round-trip cannot detect a misspelled or renamed field: serde ignores
//! keys a struct does not declare, and this crate goes further and *keeps* them
//! in `_additional` so a payload from a newer schema survives. Both are the right
//! defaults for inbound traffic — and together they mean that
//! `serde_json::from_value::<T>(literal)`, the natural way to check a document
//! assembled as JSON, proves nothing.
//!
//! These pin the trap in place (so nobody "fixes" it by making decodes strict and
//! breaking forward compatibility) and pin the check that actually answers.

#![cfg(all(feature = "json", feature = "versioned"))]

use rubo4e::current::{Adresse, Kosten, Marktlokation, Menge, Zeitraum};
use rubo4e::json::{Bo4eExtensionData, Bo4eExtensions, Bo4eJsonExt};

// ─── The trap ────────────────────────────────────────────────────────────────

/// The reported case, verbatim: a Redispatch Kostenblatt assembled as JSON and
/// "checked" by decoding it. The decode cannot fail, and the misspelling ships.
#[test]
fn a_decode_round_trip_does_not_detect_a_renamed_field() {
    let body = serde_json::json!({
        "_typ": "KOSTEN",
        "kostenbloecke": [{ "_typ": "KOSTENBLOCK", "kostenblockBEZEICHNUNG": "x" }]
    });

    let kosten: Kosten =
        serde_json::from_value(body).expect("the decode succeeds — that is the whole problem");

    // The field the key was meant to fill is empty…
    assert_eq!(
        kosten.kostenbloecke.as_ref().unwrap()[0].kostenblockbezeichnung,
        None
    );
    // …and the shallow accessor answers "clean" at the root, because the stray
    // key is one level down. A clean bill of health for a broken document.
    assert!(!kosten.has_extension_data());

    // The recursive check is the one that answers.
    assert_eq!(
        kosten.extension_paths(),
        ["kostenbloecke[0].kostenblockBEZEICHNUNG"]
    );
    let err = kosten.ensure_no_extension_data().unwrap_err();
    assert_eq!(err.paths.len(), 1);
    assert!(
        err.to_string()
            .contains("not defined by this BO4E schema version"),
        "{err}"
    );
}

/// The decode staying permissive is deliberate, not an oversight: it is how a
/// counterparty one schema release ahead reaches you at all. Pinned so a future
/// "make decoding strict" change has to argue with this test.
#[test]
fn the_permissive_decode_is_deliberate_and_round_trips_the_unknown_key() {
    let body = r#"{"_typ":"KOSTEN","vendorX":{"a":1},"kostenbloecke":[]}"#;
    let kosten = Kosten::from_json_german(body).expect("unknown fields do not fail a decode");

    assert_eq!(kosten.extension_paths(), ["vendorX"]);

    let out = kosten.to_json_german().unwrap();
    let back: serde_json::Value = serde_json::from_str(&out).unwrap();
    assert_eq!(back["vendorX"], serde_json::json!({"a": 1}));
}

// ─── What the walk covers ────────────────────────────────────────────────────

/// Every nesting shape the generator emits: a plain COM field, a `Vec` of COMs,
/// and the root itself — each reported at its own path.
///
/// The order is deterministic: a struct's own undefined keys first, then its
/// children's, depth-first in field order. Decoded from **text**, a struct's own
/// keys keep the order the wire carried them in.
#[test]
fn every_nesting_shape_is_reached_in_document_order() {
    let body = r#"{
        "_typ": "MARKTLOKATION",
        "marktlokationsId": "51238696781",
        "zzzAtRoot": 1,
        "aaaAtRoot": 2,
        "lokationsadresse": { "_typ": "ADRESSE", "ort": "Bremen", "inAComField": 3 },
        "zusatzAttribute": [
            { "name": "a", "wert": "1" },
            { "name": "b", "wert": "2", "inAVecElement": 4 }
        ]
    }"#;
    let malo = Marktlokation::from_json_german(body).unwrap();

    assert_eq!(
        malo.extension_paths(),
        [
            // Arrival order, not alphabetical: `_additional` is an `IndexMap`.
            "zzzAtRoot",
            "aaaAtRoot",
            "lokationsadresse.inAComField",
            "zusatzAttribute[1].inAVecElement",
        ]
    );
}

/// Decoding from a `serde_json::Value` instead, a struct's own keys come out
/// **sorted** — `Value`'s object is a `BTreeMap` without `preserve_order`, so the
/// arrival order was gone before this crate saw the payload. The *structural*
/// order (own keys, then children, depth-first) holds either way, and that is
/// what the guarantee is.
#[test]
fn decoding_from_a_value_loses_arrival_order_but_not_structure() {
    let body = serde_json::json!({
        "_typ": "MARKTLOKATION",
        "zzzAtRoot": 1,
        "aaaAtRoot": 2,
        "lokationsadresse": { "_typ": "ADRESSE", "inAComField": 3 }
    });
    let malo: Marktlokation = serde_json::from_value(body).unwrap();

    assert_eq!(
        malo.extension_paths(),
        ["aaaAtRoot", "zzzAtRoot", "lokationsadresse.inAComField"]
    );
}

/// `ZusatzAttribut` is the one BO4E schema that declares **no** `_typ` — it has
/// exactly `name` and `wert`, and `ComTyp` has no variant for it. So a producer
/// that stamps `"_typ": "ZUSATZATTRIBUT"` on one, by analogy with every other
/// COM, is sending a field BO4E does not define, and this reports it.
///
/// Surprising, and correct: the reference implementation emits no such key
/// either, because pydantic emits what the model declares and the model declares
/// none.
#[test]
fn a_typ_on_a_zusatzattribut_is_an_undefined_field() {
    let body = serde_json::json!({
        "_typ": "ADRESSE",
        "zusatzAttribute": [{ "_typ": "ZUSATZATTRIBUT", "name": "a", "wert": "1" }]
    });
    let adresse: Adresse = serde_json::from_value(body).unwrap();
    assert_eq!(adresse.extension_paths(), ["zusatzAttribute[0]._typ"]);

    // Without it, the same value is clean.
    let body = serde_json::json!({
        "_typ": "ADRESSE",
        "zusatzAttribute": [{ "name": "a", "wert": "1" }]
    });
    let adresse: Adresse = serde_json::from_value(body).unwrap();
    assert!(adresse.ensure_no_extension_data().is_ok());
}

/// A BO nested inside a BO — `Angebot.angebotsgeber` is a boxed
/// `Geschaeftspartner`. The generated walk has to deref the `Box` to reach it,
/// which is a different code path from the COM case above.
#[test]
fn a_boxed_bo_field_is_descended_into() {
    use rubo4e::current::Angebot;

    let body = serde_json::json!({
        "_typ": "ANGEBOT",
        "angebotsgeber": {
            "_typ": "GESCHAEFTSPARTNER",
            "strayOnTheNestedBo": 1,
            "adresse": { "_typ": "ADRESSE", "strayTwoLevelsDown": 2 }
        }
    });
    let angebot: Angebot = serde_json::from_value(body).unwrap();

    assert_eq!(
        angebot.extension_paths(),
        [
            "angebotsgeber.strayOnTheNestedBo",
            "angebotsgeber.adresse.strayTwoLevelsDown",
        ]
    );
}

/// Everything below an extension key is opaque — the schema stops there and so
/// does the walk — so a vendor blob is one finding, not one per leaf.
#[test]
fn a_nested_vendor_blob_is_one_finding() {
    let body = serde_json::json!({
        "_typ": "ADRESSE",
        "vendorX": { "a": 1, "b": { "c": 2 }, "d": [3, 4] }
    });
    let adresse: Adresse = serde_json::from_value(body).unwrap();
    assert_eq!(adresse.extension_paths(), ["vendorX"]);
}

/// A clean document reports nothing, at any depth.
#[test]
fn a_conforming_document_is_silent() {
    let body = serde_json::json!({
        "_typ": "MARKTLOKATION",
        "marktlokationsId": "51238696781",
        "sparte": "STROM",
        "lokationsadresse": { "_typ": "ADRESSE", "ort": "Bremen", "postleitzahl": "28195" }
    });
    let malo: Marktlokation = serde_json::from_value(body).unwrap();
    assert!(malo.extension_paths().is_empty());
    assert!(malo.ensure_no_extension_data().is_ok());
}

/// A value built in Rust cannot carry an undefined field, which is the whole
/// argument for constructing typed rather than decode-to-check.
#[test]
fn a_typed_value_is_clean_by_construction() {
    let malo = Marktlokation {
        lokationsadresse: Some(Adresse {
            ort: Some("Bremen".into()),
            ..Default::default()
        }),
        ..Default::default()
    };
    assert!(malo.ensure_no_extension_data().is_ok());
}

// ─── Paths stay unambiguous ──────────────────────────────────────────────────

/// Extension keys come off the wire, so they can contain the very characters the
/// path syntax uses. `a.b` rendered as `parent.a.b` would name two fields that do
/// not exist, so a key that is not a plain identifier is bracket-quoted.
#[test]
fn a_key_that_is_not_an_identifier_is_quoted_rather_than_dotted() {
    let body = serde_json::json!({
        "_typ": "MARKTLOKATION",
        "lokationsadresse": { "_typ": "ADRESSE", "a.b": 1, "he\"y": 2, "plain_key": 3 }
    });
    let malo: Marktlokation = serde_json::from_value(body).unwrap();

    let mut paths = malo.extension_paths();
    paths.sort();
    assert_eq!(
        paths,
        [
            "lokationsadresse.plain_key",
            r#"lokationsadresse["a.b"]"#,
            r#"lokationsadresse["he\"y"]"#,
        ]
    );
}

/// JSON permits an empty key, and a bracket that quotes it stays unambiguous
/// where a dot would vanish into the parent's path.
#[test]
fn an_empty_key_is_quoted_rather_than_dropped() {
    use rubo4e::strict::extension_path;

    let body = serde_json::json!({ "_typ": "ADRESSE", "": 1 });
    let adresse: Adresse = serde_json::from_value(body).unwrap();
    assert_eq!(adresse.extension_paths(), [r#"[""]"#]);

    // …and the helper's own contract, at both nesting positions.
    assert_eq!(extension_path("", ""), r#"[""]"#);
    assert_eq!(extension_path("adresse", ""), r#"adresse[""]"#);
    // A key that is itself bracket syntax cannot be mistaken for one.
    assert_eq!(extension_path("x", r#"["y"]"#), r#"x["[\"y\"]"]"#);
}

// ─── The two checks are different questions ──────────────────────────────────

/// `Bo4eStrict` finds out-of-schema **values**; `Bo4eExtensions` finds
/// out-of-schema **fields**. Neither sees the other's finding, which is why a
/// strict ingest boundary that wants both has to ask both.
#[test]
fn the_value_check_and_the_field_check_do_not_overlap() {
    use rubo4e::Bo4eStrict;

    let bad_value = serde_json::json!({ "_typ": "MARKTLOKATION", "sparte": "PLASMA" });
    let malo: Marktlokation = serde_json::from_value(bad_value).unwrap();
    assert_eq!(malo.unknown_enum_paths(), ["sparte"]);
    assert!(
        malo.extension_paths().is_empty(),
        "`sparte` is a defined field; only its value is out of schema"
    );

    let bad_field = serde_json::json!({ "_typ": "MARKTLOKATION", "spartee": "STROM" });
    let malo: Marktlokation = serde_json::from_value(bad_field).unwrap();
    assert_eq!(malo.extension_paths(), ["spartee"]);
    assert!(
        malo.unknown_enum_paths().is_empty(),
        "no enum was decoded at all — the key never reached one"
    );
}

// ─── AnyBo ───────────────────────────────────────────────────────────────────

/// The dispatch delegates, so a gateway that decoded `AnyBo` can ask the same
/// question without matching out the concrete type first.
#[test]
fn any_bo_delegates_to_the_dispatched_type() {
    use rubo4e::current::AnyBo;

    let body = r#"{"_typ":"MARKTLOKATION","marktlokationsId":"51238696781","stray":1}"#;
    let bo: AnyBo = serde_json::from_str(body).unwrap();
    assert_eq!(bo.extension_paths(), ["stray"]);

    // A `_typ` no generated type matches is itself the finding: everything in
    // such a payload is undefined here, and listing each key would be noise.
    let unknown: AnyBo = serde_json::from_str(r#"{"_typ":"NOT_A_BO4E_TYPE","a":1,"b":2}"#).unwrap();
    assert_eq!(unknown.extension_paths(), ["_typ"]);
}

// ─── The snake_case reader ───────────────────────────────────────────────────

/// The key transform is a table lookup: a key it does not know passes through
/// unchanged, so the path reports what the producer actually wrote rather than a
/// camelCase guess at it.
#[test]
fn the_snake_case_reader_reports_the_key_as_written() {
    let body =
        r#"{"_typ":"MARKTLOKATION","marktlokations_id":"51238696781","kostenblock_BEZEICHNUNG":1}"#;
    let malo = Marktlokation::from_json_snake_case(body).unwrap();

    assert!(malo.marktlokations_id.is_some(), "the known key mapped");
    assert_eq!(malo.extension_paths(), ["kostenblock_BEZEICHNUNG"]);
}

// ─── The `serde_json::Value` reader ──────────────────────────────────────────

/// The `Value` readers carry the same budgets as the text ones, which is the
/// point: the caller who assembled a document with `json!` is exactly the caller
/// who had no way to reach them before.
#[test]
fn the_value_reader_enforces_the_same_budgets() {
    use rubo4e::json::JsonParseLimits;

    let body = serde_json::json!({
        "_typ": "MARKTLOKATION",
        "marktlokationsId": "51238696781",
        "stray": 1
    });

    // Permissive by default, like every other reader.
    assert!(Marktlokation::from_json_value(body.clone()).is_ok());

    // Closed extension budget turns the decode into the check.
    let closed = JsonParseLimits::unlimited().with_max_extension_field_count(Some(0));
    assert!(Marktlokation::from_json_value_hardened(body.clone(), closed).is_err());

    // …and a budget that has room lets it through.
    let roomy = JsonParseLimits::unlimited().with_max_extension_field_count(Some(4));
    assert!(Marktlokation::from_json_value_hardened(body, roomy).is_ok());
}

/// Depth is guarded on this path too — `Value` is itself a `Deserializer`, so the
/// same wrapper composes over it.
#[test]
fn the_value_reader_enforces_the_nesting_depth_cap() {
    use rubo4e::json::JsonParseLimits;

    let mut nested = serde_json::json!(1);
    for _ in 0..40 {
        nested = serde_json::Value::Array(vec![nested]);
    }
    let body = serde_json::json!({ "_typ": "MARKTLOKATION", "blob": nested });

    let shallow = JsonParseLimits::unlimited().with_max_nesting_depth(Some(4));
    assert!(Marktlokation::from_json_value_hardened(body.clone(), shallow).is_err());

    let roomy = JsonParseLimits::unlimited().with_max_nesting_depth(Some(64));
    assert!(Marktlokation::from_json_value_hardened(body, roomy).is_ok());
}

/// `max_payload_bytes` has nothing to cap here — the caller already paid for the
/// parse — so it is ignored rather than rejected, which is what lets one
/// `JsonParseLimits` be shared between the text and `Value` paths.
#[test]
fn the_value_reader_ignores_the_payload_size_cap() {
    use rubo4e::json::JsonParseLimits;

    let body = serde_json::json!({ "_typ": "MARKTLOKATION", "marktlokationsId": "51238696781" });
    let tiny = JsonParseLimits::untrusted_defaults().with_max_payload_bytes(Some(1));

    assert!(
        Marktlokation::from_json_value_hardened(body, tiny).is_ok(),
        "a cap on bytes cannot apply to a value that has none"
    );
}

// ─── Cost ────────────────────────────────────────────────────────────────────

/// A clean document allocates nothing: `Vec::new()` does not allocate until the
/// first push, and a struct with no extension data never pushes. That is what
/// makes the check affordable to run unconditionally on a 35 040-entry Lastgang.
#[test]
fn a_clean_document_costs_no_allocation() {
    let paths = Marktlokation::default().extension_paths();
    assert!(paths.is_empty());
    assert_eq!(paths.capacity(), 0, "an empty Vec must not have allocated");
}

/// The walk is one pass over the tree and allocates only the result, so it is
/// usable on the payloads that are actually large.
#[test]
fn a_large_nested_document_is_walked_once() {
    use rubo4e::current::{Lastgang, Mengeneinheit, Zeitreihenwert};

    let werte: Vec<Zeitreihenwert> = (0..2_000)
        .map(|i| {
            let mut v = Zeitreihenwert {
                zeitraum: Some(Zeitraum::default()),
                ..Default::default()
            };
            if i == 1_337 {
                v._additional
                    .try_insert("strayKey".into(), serde_json::json!(1))
                    .unwrap();
            }
            v
        })
        .collect();

    let lg = Lastgang {
        werte: Some(werte),
        messgroesse: Some(Mengeneinheit::Kw),
        ..Lastgang::new(Menge::default())
    };

    assert_eq!(lg.extension_paths(), ["werte[1337].strayKey"]);
}
