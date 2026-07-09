//! Integration tests for `AnyBo` dynamic dispatch (H-08).
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
        let json = r#"{"_typ":"MARKTLOKATION","_version":"v202607.0.0"}"#;
        assert_any_bo_roundtrip(json, BoTyp::Marktlokation);
    }

    #[test]
    fn messlokation_dispatch() {
        let json = r#"{"_typ":"MESSLOKATION","_version":"v202607.0.0"}"#;
        assert_any_bo_roundtrip(json, BoTyp::Messlokation);
    }

    #[test]
    fn vertrag_dispatch() {
        let json = r#"{"_typ":"VERTRAG","_version":"v202607.0.0"}"#;
        assert_any_bo_roundtrip(json, BoTyp::Vertrag);
    }

    #[test]
    fn rechnung_dispatch() {
        let json = r#"{"_typ":"RECHNUNG","_version":"v202607.0.0"}"#;
        assert_any_bo_roundtrip(json, BoTyp::Rechnung);
    }

    #[test]
    fn lastgang_dispatch() {
        // Lastgang requires zeitIntervallLaenge (non-optional Menge field).
        let json = r#"{"_typ":"LASTGANG","_version":"v202607.0.0","zeitIntervallLaenge":{}}"#;
        assert_any_bo_roundtrip(json, BoTyp::Lastgang);
    }

    #[test]
    fn energiemenge_dispatch() {
        let json = r#"{"_typ":"ENERGIEMENGE","_version":"v202607.0.0"}"#;
        assert_any_bo_roundtrip(json, BoTyp::Energiemenge);
    }

    #[test]
    fn geschaeftspartner_dispatch() {
        let json = r#"{"_typ":"GESCHAEFTSPARTNER","_version":"v202607.0.0"}"#;
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
        let json = r#"{"_typ":"ZUKUNFTSTYP","_version":"v202607.0.0","someField":"value"}"#;
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
        let json = r#"{"_version":"v202607.0.0","someField":42}"#;
        let any: AnyBo = serde_json::from_str(json).expect("AnyBo should accept missing _typ");
        assert_eq!(any.bo_type(), BoTyp::Unknown);
    }

    #[test]
    fn empty_typ_produces_unknown_variant() {
        let json = r#"{"_typ":"","_version":"v202607.0.0"}"#;
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
