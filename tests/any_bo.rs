//! Integration tests for `AnyBo` dynamic dispatch.
//!
//! Verifies:
//! - Each known BO type round-trips correctly through `AnyBo::deserialize`.
//! - `AnyBo::bo_type()` returns the correct `BoTyp` discriminant.
//! - Serializing `AnyBo` produces the same JSON as serializing the inner type directly.
//! - The `Unknown` variant is produced for unrecognised `_typ` values.
//! - Structurally unknown `_typ` is preserved in `Unknown::typ`.
//!
//! Run with:
//! ```text
//! cargo test --test any_bo --features json,versioned
//! ```

#[cfg(all(feature = "json", feature = "versioned"))]
mod any_bo_tests {
    use rubo4e::v202607::{AnyBo, BoTyp, Lastgang, Marktlokation, Messlokation, Rechnung, Vertrag};

    // ── Helpers ──────────────────────────────────────────────────────────────

    /// Deserialize `json` as `AnyBo`, assert the variant matches `expected_typ`,
    /// then re-serialize and assert structural equality with the original JSON.
    fn assert_any_bo_roundtrip(json: &str, expected_typ: BoTyp) {
        let any: AnyBo = serde_json::from_str(json)
            .unwrap_or_else(|e| panic!("AnyBo deserialization failed: {e}\ninput: {json}"));
        assert_eq!(
            any.bo_type(),
            expected_typ,
            "bo_type() mismatch for input: {json}"
        );
        // Re-serialize and compare as Value trees for structural equality.
        let original: serde_json::Value =
            serde_json::from_str(json).expect("parse original as Value");
        let reserialized: serde_json::Value =
            serde_json::from_str(&serde_json::to_string(&any).expect("serialize AnyBo"))
                .expect("re-parse serialized AnyBo");
        assert_eq!(
            original, reserialized,
            "AnyBo round-trip changed JSON structure for {expected_typ:?}"
        );
    }

    // ── Per-type dispatch tests ───────────────────────────────────────────────

    #[test]
    fn marktlokation_dispatch() {
        let json = r#"{"_typ":"MARKTLOKATION","_version":"202607.0.0"}"#;
        assert_any_bo_roundtrip(json, BoTyp::Marktlokation);
    }

    #[test]
    fn messlokation_dispatch() {
        let json = r#"{"_typ":"MESSLOKATION","_version":"202607.0.0"}"#;
        assert_any_bo_roundtrip(json, BoTyp::Messlokation);
    }

    #[test]
    fn vertrag_dispatch() {
        let json = r#"{"_typ":"VERTRAG","_version":"202607.0.0"}"#;
        assert_any_bo_roundtrip(json, BoTyp::Vertrag);
    }

    #[test]
    fn rechnung_dispatch() {
        let json = r#"{"_typ":"RECHNUNG","_version":"202607.0.0"}"#;
        assert_any_bo_roundtrip(json, BoTyp::Rechnung);
    }

    #[test]
    fn lastgang_dispatch() {
        // Lastgang requires zeitIntervallLaenge (non-optional Menge field).
        let json = r#"{"_typ":"LASTGANG","_version":"202607.0.0","zeitIntervallLaenge":{}}"#;
        assert_any_bo_roundtrip(json, BoTyp::Lastgang);
    }

    #[test]
    fn energiemenge_dispatch() {
        let json = r#"{"_typ":"ENERGIEMENGE","_version":"202607.0.0"}"#;
        assert_any_bo_roundtrip(json, BoTyp::Energiemenge);
    }

    #[test]
    fn geschaeftspartner_dispatch() {
        let json = r#"{"_typ":"GESCHAEFTSPARTNER","_version":"202607.0.0"}"#;
        assert_any_bo_roundtrip(json, BoTyp::Geschaeftspartner);
    }

    // ── Golden file round-trips through AnyBo ────────────────────────────────

    #[test]
    fn marktlokation_golden_minimal() {
        let json = include_str!("golden/marktlokation_minimal.json");
        assert_any_bo_roundtrip(json, BoTyp::Marktlokation);
    }

    #[test]
    fn marktlokation_golden_typical() {
        let json = include_str!("golden/marktlokation_typical.json");
        assert_any_bo_roundtrip(json, BoTyp::Marktlokation);
    }

    #[test]
    fn vertrag_golden_typical() {
        let json = include_str!("golden/vertrag_typical.json");
        assert_any_bo_roundtrip(json, BoTyp::Vertrag);
    }

    #[test]
    fn rechnung_golden_typical() {
        let json = include_str!("golden/rechnung_typical.json");
        assert_any_bo_roundtrip(json, BoTyp::Rechnung);
    }

    // ── Unknown variant ───────────────────────────────────────────────────────

    #[test]
    fn unknown_typ_produces_unknown_variant() {
        let json = r#"{"_typ":"ZUKUNFTSTYP","_version":"202607.0.0","someField":"value"}"#;
        let any: AnyBo = serde_json::from_str(json).expect("AnyBo::Unknown should parse");
        assert_eq!(
            any.bo_type(),
            BoTyp::Unknown,
            "unrecognised _typ should yield BoTyp::Unknown"
        );
        if let AnyBo::Unknown { typ, .. } = any {
            assert_eq!(
                typ, "ZUKUNFTSTYP",
                "Unknown::typ should preserve the _typ value"
            );
        } else {
            panic!("expected AnyBo::Unknown but got a known variant");
        }
    }

    #[test]
    fn missing_typ_produces_unknown_variant() {
        // No _typ field at all → Unknown with empty string.
        let json = r#"{"_version":"202607.0.0","someField":42}"#;
        let any: AnyBo = serde_json::from_str(json).expect("AnyBo should accept missing _typ");
        assert_eq!(any.bo_type(), BoTyp::Unknown);
    }

    #[test]
    fn empty_typ_produces_unknown_variant() {
        let json = r#"{"_typ":"","_version":"202607.0.0"}"#;
        let any: AnyBo = serde_json::from_str(json).expect("AnyBo should accept empty _typ");
        assert_eq!(any.bo_type(), BoTyp::Unknown);
    }

    // ── From impls ────────────────────────────────────────────────────────────

    #[test]
    fn from_marktlokation_into_any_bo() {
        let malo = Marktlokation::default();
        let any: AnyBo = malo.into();
        assert_eq!(any.bo_type(), BoTyp::Marktlokation);
    }

    #[test]
    fn from_vertrag_into_any_bo() {
        let v = Vertrag::default();
        let any: AnyBo = v.into();
        assert_eq!(any.bo_type(), BoTyp::Vertrag);
    }

    #[test]
    fn from_rechnung_into_any_bo() {
        let r = Rechnung::default();
        let any: AnyBo = r.into();
        assert_eq!(any.bo_type(), BoTyp::Rechnung);
    }

    #[test]
    fn from_lastgang_into_any_bo() {
        use rubo4e::v202607::Menge;
        let l = Lastgang {
            zeit_intervall_laenge: Menge::default(),
            typ: Some(rubo4e::v202607::BoTyp::Lastgang),
            id: None,
            marktlokation: None,
            messgroesse: None,
            messlokation: None,
            obis_kennzahl: None,
            sparte: None,
            version: None,
            werte: None,
            zusatz_attribute: None,
            _additional: Default::default(),
        };
        let any: AnyBo = l.into();
        assert_eq!(any.bo_type(), BoTyp::Lastgang);
    }

    #[test]
    fn from_messlokation_into_any_bo() {
        let m = Messlokation::default();
        let any: AnyBo = m.into();
        assert_eq!(any.bo_type(), BoTyp::Messlokation);
    }
}

// ─── AnyBo must honour the caller's deserializer ─────────────────────────────
//
// `AnyBo::deserialize` buffers the payload before it can read `"_typ"` and pick a
// concrete type.  It has to buffer *through* the deserializer it was given, or it
// loses the two wrappers this crate installs: the key transform and the depth
// limiter.

/// `to_json_snake_case` → `from_json_snake_case` must round-trip through `AnyBo`.
///
/// Without the key transform every typed field lands in `_additional` instead,
/// and the call returns `Ok` with an empty object rather than failing.
#[cfg(all(feature = "versioned", feature = "json"))]
#[test]
fn any_bo_snake_case_round_trip_preserves_typed_fields() {
    use rubo4e::current::{AnyBo, Marktlokation, Sparte};
    use rubo4e::json::{Bo4eExtensionData, Bo4eJsonExt};

    let malo = Marktlokation {
        marktlokations_id: Some("51238696781".try_into().expect("valid MaLo-ID")),
        sparte: Some(Sparte::Strom),
        ..Default::default()
    };
    let any: AnyBo = malo.clone().into();

    let snake = any.to_json_snake_case().expect("serialize");
    assert!(
        snake.contains("\"marktlokations_id\""),
        "snake_case output should use Rust field names: {snake}"
    );

    let back = AnyBo::from_json_snake_case(&snake).expect("deserialize");
    let AnyBo::Marktlokation(round_tripped) = back else {
        panic!("_typ MARKTLOKATION must select the Marktlokation variant");
    };

    assert_eq!(round_tripped.marktlokations_id, malo.marktlokations_id);
    assert_eq!(round_tripped.sparte, malo.sparte);
    assert!(
        !round_tripped.has_extension_data(),
        "typed fields must not be diverted into extension data: {:?}",
        round_tripped.extension_data()
    );
}

/// A hardened `max_nesting_depth` must bind `AnyBo` exactly as it binds a
/// concrete BO type.
///
/// A `RawValue` capture is only one level deep as far as the depth limiter can
/// see, so re-parsing left the configured limit entirely unenforced — the guard
/// silently did nothing on the polymorphic ingest path it matters most for.
#[cfg(all(feature = "versioned", feature = "json"))]
#[test]
fn any_bo_enforces_hardened_nesting_depth() {
    use rubo4e::current::{AnyBo, Marktlokation};
    use rubo4e::json::{Bo4eJsonExt, JsonParseLimits};

    fn payload(depth: usize) -> String {
        let (open, close) = ("[".repeat(depth), "]".repeat(depth));
        format!(r#"{{"_typ":"MARKTLOKATION","marktlokationsId":"51238696781","x":{open}1{close}}}"#)
    }

    let limits = JsonParseLimits::unlimited().with_max_nesting_depth(Some(8));

    // Within the limit: both accept.
    assert!(Marktlokation::from_json_german_hardened(&payload(4), limits).is_ok());
    assert!(AnyBo::from_json_german_hardened(&payload(4), limits).is_ok());

    // Beyond the limit: both must reject, and for the same reason.
    assert!(Marktlokation::from_json_german_hardened(&payload(40), limits).is_err());
    assert!(
        AnyBo::from_json_german_hardened(&payload(40), limits).is_err(),
        "AnyBo must not escape the configured nesting-depth limit"
    );
}

/// A concrete BO's `TYP` describes the **Rust type**, not the `_typ` the payload
/// carried — otherwise a `match` on it takes the branch the sender named.
/// `AnyBo` is the type that dispatches on a payload's claim.
#[test]
#[cfg(all(feature = "json", feature = "versioned"))]
fn a_concrete_bo_reports_its_own_type_not_the_payloads_claim() {
    use rubo4e::current::{BoTyp, Marktlokation, Vertrag};
    use rubo4e::Bo4eTyped;

    let malo: Marktlokation =
        serde_json::from_str(r#"{"_typ":"VERTRAG","marktlokationsId":"51238696781"}"#)
            .expect("the lenient decode succeeds — that is the point");

    assert_eq!(Marktlokation::TYP, BoTyp::Marktlokation);
    // The payload's own claim is preserved, so the mismatch stays detectable.
    assert_eq!(malo.typ, Some(BoTyp::Vertrag));
    assert_ne!(malo.typ, Some(Marktlokation::TYP));

    // A `_typ` the schema does not define decodes to the catch-all, and still
    // does not change what the value is.
    let v: Vertrag = serde_json::from_str(r#"{"_typ":"NOT_A_BO"}"#).expect("lenient");
    assert_eq!(Vertrag::TYP, BoTyp::Vertrag);
    assert_eq!(v.typ, Some(BoTyp::Unknown));

    // …and a payload with no `_typ` at all leaves the constant unchanged.
    let bare: Vertrag = serde_json::from_str("{}").expect("valid");
    assert_eq!(bare.typ, None);
}

/// `AnyBo` carries every `Bo4eObject` fact, since it is what a heterogeneous
/// collection uses in place of a trait object — plus the `Clone`, `PartialEq`,
/// and serde impls a trait object cannot have.
#[test]
#[cfg(all(feature = "json", feature = "versioned"))]
fn any_bo_carries_every_bo4e_object_fact() {
    use rubo4e::current::{AnyBo, BoTyp, Marktlokation, Vertrag};
    use rubo4e::Bo4eTyped;

    // The heterogeneous collection the `dyn` doctest used to build.
    let objects: Vec<AnyBo> = vec![Vertrag::default().into(), Marktlokation::default().into()];

    assert_eq!(
        objects.iter().map(AnyBo::bo_type).collect::<Vec<_>>(),
        [BoTyp::Vertrag, BoTyp::Marktlokation]
    );
    assert_eq!(
        objects.iter().map(AnyBo::typ_wire).collect::<Vec<_>>(),
        ["VERTRAG", "MARKTLOKATION"]
    );
    for bo in &objects {
        assert_eq!(bo.schema_version(), Some(Vertrag::SCHEMA_VERSION));
        assert_eq!(bo.schema_series(), Some(Vertrag::SCHEMA_SERIES));
    }

    // …and the things a trait object could not do.
    assert_eq!(objects.clone(), objects);
    let json = serde_json::to_string(&objects[0]).expect("serializes");
    assert!(json.contains(r#""_typ":"VERTRAG""#), "{json}");
}

/// For the `Unknown` catch-all there is no generated type, so there is no
/// release to report — but the wire discriminant the payload carried is still
/// there, which is the whole reason that variant keeps it.
#[test]
#[cfg(all(feature = "json", feature = "versioned"))]
fn any_bo_facts_are_honest_about_the_unknown_variant() {
    use rubo4e::current::{AnyBo, BoTyp};

    let bo: AnyBo = serde_json::from_str(r#"{"_typ":"MARKTROLLENWECHSEL","x":1}"#)
        .expect("an unknown _typ decodes to the catch-all");

    assert_eq!(bo.bo_type(), BoTyp::Unknown);
    assert_eq!(bo.typ_wire(), "MARKTROLLENWECHSEL");
    assert_eq!(bo.schema_version(), None);
    assert_eq!(bo.schema_series(), None);
}
