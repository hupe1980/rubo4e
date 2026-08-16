//! Cross-implementation compatibility tests.
//!
//! These tests verify that JSON payloads conforming to the BO4E standard can be
//! deserialized by rubo4e regardless of which BO4E implementation produced them.
//!
//! The payloads in `tests/compat/python/` represent the JSON format emitted by
//! the reference Python implementation (`bo4e/BO4E-python`).
//!
//! The payloads in `tests/compat/go/` represent the JSON format emitted by the
//! Go implementation (`Hochfrequenz/go-bo4e`).
//!
//! See `tests/compat/README.md` for how to regenerate these vectors.

#[cfg(all(feature = "json", feature = "versioned"))]
mod compat_tests {
    use rubo4e::v202607::{Marktlokation, Messlokation, Rechnung, Vertrag};

    // ── Python vectors ────────────────────────────────────────────────────────

    #[test]
    fn python_marktlokation_deserializes() {
        let json = include_str!("compat/python/marktlokation.json");
        let malo: Marktlokation =
            serde_json::from_str(json).expect("python/marktlokation.json must deserialize");
        assert_eq!(
            malo.marktlokations_id.as_ref().map(|id| id.as_ref()),
            Some("51238696781")
        );
        assert!(malo.sparte.is_some(), "sparte must be present");
        assert!(
            malo.energierichtung.is_some(),
            "energierichtung must be present"
        );
    }

    #[test]
    fn python_messlokation_deserializes() {
        let json = include_str!("compat/python/messlokation.json");
        let melo: Messlokation =
            serde_json::from_str(json).expect("python/messlokation.json must deserialize");
        assert_eq!(
            melo.messlokations_id.as_ref().map(|id| id.as_ref()),
            Some("DE0000000000000000000000000000001")
        );
        assert!(melo.sparte.is_some(), "sparte must be present");
    }

    #[test]
    fn python_vertrag_deserializes() {
        let json = include_str!("compat/python/vertrag.json");
        let v: Vertrag = serde_json::from_str(json).expect("python/vertrag.json must deserialize");
        assert_eq!(v.vertragsnummer.as_deref(), Some("V-2025-001"));
        assert!(v.sparte.is_some(), "sparte must be present");
        assert!(v.vertragsart.is_some(), "vertragsart must be present");
    }

    #[test]
    fn python_rechnung_deserializes() {
        let json = include_str!("compat/python/rechnung.json");
        let r: Rechnung =
            serde_json::from_str(json).expect("python/rechnung.json must deserialize");
        assert_eq!(r.rechnungsnummer.as_deref(), Some("R-2025-0042"));
        assert!(r.gesamtbrutto.is_some(), "gesamtbrutto must be present");
        assert!(r.gesamtnetto.is_some(), "gesamtnetto must be present");
        assert!(r.gesamtsteuer.is_some(), "gesamtsteuer must be present");
    }

    // ── Go vectors ────────────────────────────────────────────────────────────

    #[test]
    fn go_marktlokation_deserializes() {
        let json = include_str!("compat/go/marktlokation.json");
        let malo: Marktlokation =
            serde_json::from_str(json).expect("go/marktlokation.json must deserialize");
        assert_eq!(
            malo.marktlokations_id.as_ref().map(|id| id.as_ref()),
            Some("51238696781")
        );
        assert!(malo.sparte.is_some(), "sparte must be present");
        assert!(
            malo.energierichtung.is_some(),
            "energierichtung must be present"
        );
    }

    #[test]
    fn go_messlokation_deserializes() {
        let json = include_str!("compat/go/messlokation.json");
        let melo: Messlokation =
            serde_json::from_str(json).expect("go/messlokation.json must deserialize");
        assert_eq!(
            melo.messlokations_id.as_ref().map(|id| id.as_ref()),
            Some("DE0000000000000000000000000000002")
        );
        assert!(melo.sparte.is_some(), "sparte must be present");
    }

    #[test]
    fn go_vertrag_deserializes() {
        let json = include_str!("compat/go/vertrag.json");
        let v: Vertrag = serde_json::from_str(json).expect("go/vertrag.json must deserialize");
        assert_eq!(v.vertragsnummer.as_deref(), Some("VT-2025-9999"));
        assert!(v.sparte.is_some(), "sparte must be present");
        assert!(v.vertragsart.is_some(), "vertragsart must be present");
    }

    #[test]
    fn go_rechnung_deserializes() {
        let json = include_str!("compat/go/rechnung.json");
        let r: Rechnung = serde_json::from_str(json).expect("go/rechnung.json must deserialize");
        assert_eq!(r.rechnungsnummer.as_deref(), Some("INV-2025-007"));
        assert!(r.gesamtbrutto.is_some(), "gesamtbrutto must be present");
        assert!(r.gesamtnetto.is_some(), "gesamtnetto must be present");
        assert!(r.gesamtsteuer.is_some(), "gesamtsteuer must be present");
    }
}

/// Outbound direction: JSON that rubo4e *produces* must carry the same metadata
/// every other BO4E implementation emits.
///
/// The vectors above only prove rubo4e can read other implementations' output.
/// That leaves the opposite direction untested, which is how `_version` came to
/// be omitted from every freshly constructed value: the golden corpus round-trips
/// existing payloads, so it carried `_version` in from the input and never
/// exercised construction.
#[cfg(all(feature = "json", feature = "versioned"))]
mod outbound_tests {
    use rubo4e::current::{Betrag, Marktlokation, Rechnung, Vertrag};
    use rubo4e::json::Bo4eJsonExt;
    use rubo4e::Bo4eObject as _;

    /// Reference implementations stamp `_version` on every BO, and rubo4e knows
    /// the value statically — a caller should never have to supply it.
    #[test]
    fn constructed_bo_carries_typ_and_version() {
        let v = Vertrag::default();
        let json = v.to_json_german().expect("serialize");
        assert!(json.contains(r#""_typ":"VERTRAG""#), "missing _typ: {json}");
        assert!(
            json.contains(&format!(r#""_version":"{}""#, v.schema_version())),
            "missing _version: {json}"
        );
    }

    /// COMs carry `_version` but no `_typ` — matching the nested objects in the
    /// python/go vectors (`gesamtnetto`, `lokationsadresse`).
    #[test]
    fn constructed_com_carries_version_but_not_typ() {
        let json = Betrag::default().to_json_german().expect("serialize");
        assert!(json.contains(r#""_version""#), "missing _version: {json}");
        assert!(
            !json.contains(r#""_typ""#),
            "COM must not carry _typ: {json}"
        );
    }

    /// `_version` must reach nested COMs too, not just the root object.
    #[test]
    fn nested_com_carries_version() {
        let r = Rechnung {
            gesamtnetto: Some(Betrag::default()),
            ..Default::default()
        };
        let value: serde_json::Value =
            serde_json::from_str(&r.to_json_german().expect("serialize")).expect("valid json");
        assert!(
            value["gesamtnetto"]["_version"].is_string(),
            "nested COM lost _version: {value}"
        );
    }

    /// A payload that arrived stamped with a different series keeps that stamp:
    /// `_version` records the provenance of the data, and deserialization must
    /// not overwrite it with the version this crate was generated from.
    #[test]
    fn deserialized_version_is_not_overwritten() {
        let body =
            r#"{"_typ":"MARKTLOKATION","_version":"v202501.0.0","marktlokationsId":"51238696781"}"#;
        let malo: Marktlokation = serde_json::from_str(body).expect("deserialize");
        assert_eq!(malo.version.as_deref(), Some("v202501.0.0"));
        assert!(malo
            .to_json_german()
            .expect("serialize")
            .contains(r#""_version":"v202501.0.0""#));
    }

    /// An inbound payload without `_version` must not gain one: absence is
    /// information, and `Default` must not leak into the deserialization path.
    #[test]
    fn absent_version_stays_absent() {
        let malo: Marktlokation =
            serde_json::from_str(r#"{"_typ":"MARKTLOKATION"}"#).expect("deserialize");
        assert_eq!(malo.version, None);
        assert!(!malo
            .to_json_german()
            .expect("serialize")
            .contains("_version"));
    }
}
