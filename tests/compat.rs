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
    use rubo4e::v202501::{Marktlokation, Messlokation, Rechnung, Vertrag};

    // ── Python vectors ────────────────────────────────────────────────────────

    #[test]
    fn python_marktlokation_deserializes() {
        let json = include_str!("compat/python/marktlokation.json");
        let malo: Marktlokation =
            serde_json::from_str(json).expect("python/marktlokation.json must deserialize");
        assert_eq!(
            malo.marktlokations_id.as_ref().map(|id| id.as_ref()),
            Some("51238696780")
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
            Some("51238696780")
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
