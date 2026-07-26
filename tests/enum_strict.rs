//! Integration tests for feature-independent enum introspection and strict
//! parsing (`VARIANTS`, `COUNT`, `iter_known`, `as_wire`, `from_wire`,
//! `is_known` / `is_unknown`, and the `Bo4eEnum` trait).
//!
//! Crucially, these tests are gated on **only** `versioned` — NOT `strum` — to
//! prove the introspection/strict surface is available without the `strum`
//! feature (feedback FR-2 and BUG-3).
//!
//! Run with:
//! ```text
//! cargo test --test enum_strict --no-default-features --features versioned
//! ```

#![cfg(feature = "versioned")]

use rubo4e::current::{Geraetetyp, Sparte, Zaehlertyp};
use rubo4e::Bo4eEnum;

// ─── FR-2: variant introspection without `strum` ───────────────────────────

#[test]
fn variants_and_count_are_consistent() {
    // COUNT equals VARIANTS.len() and excludes the Unknown catch-all.
    assert_eq!(Zaehlertyp::COUNT, Zaehlertyp::VARIANTS.len());
    // v202607 defines exactly 13 Zaehlertyp variants (the count the mako guard pins).
    assert_eq!(Zaehlertyp::COUNT, 13);
    assert!(!Zaehlertyp::VARIANTS.contains(&Zaehlertyp::Unknown));
}

#[test]
fn iter_known_matches_variants_and_excludes_unknown() {
    let iterated: Vec<Zaehlertyp> = Zaehlertyp::iter_known().collect();
    assert_eq!(iterated.as_slice(), Zaehlertyp::VARIANTS);
    assert!(!iterated.contains(&Zaehlertyp::Unknown));
}

#[test]
fn trait_and_inherent_agree() {
    // The `Bo4eEnum` trait members forward to the inherent ones.
    assert_eq!(<Zaehlertyp as Bo4eEnum>::COUNT, Zaehlertyp::VARIANTS.len());
    assert_eq!(<Zaehlertyp as Bo4eEnum>::VARIANTS, Zaehlertyp::VARIANTS);
}

/// Generic coverage helper — exactly the pattern the mako project needs to prove
/// a SQL `CHECK` list covers every variant of an enum, generic over the type.
fn wire_values<T: Bo4eEnum>() -> Vec<&'static str> {
    T::VARIANTS.iter().map(|v| v.as_wire()).collect()
}

#[test]
fn generic_over_bo4e_enum() {
    let sparten = wire_values::<Sparte>();
    assert!(sparten.contains(&"STROM"));
    assert!(sparten.contains(&"GAS"));
    assert!(!sparten.contains(&"UNKNOWN"));
}

// ─── BUG-3: strict parsing rejects out-of-schema values ────────────────────

#[test]
fn from_wire_accepts_known_values() {
    assert_eq!(
        Zaehlertyp::from_wire("WASSERZAEHLER"),
        Ok(Zaehlertyp::Wasserzaehler)
    );
    assert_eq!(
        Zaehlertyp::from_wire("INTELLIGENTES_MESSSYSTEM"),
        Ok(Zaehlertyp::IntelligentesMesssystem)
    );
}

#[test]
fn from_wire_rejects_unknown_and_literal_unknown() {
    // A typo / legacy code is rejected rather than silently mapped to Unknown.
    let err = Zaehlertyp::from_wire("LFG").unwrap_err();
    assert_eq!(err.value, "LFG");
    // The synthetic "UNKNOWN" wire string is NOT a real schema value.
    assert!(Zaehlertyp::from_wire("UNKNOWN").is_err());
}

#[test]
fn round_trip_as_wire_from_wire() {
    for &variant in Zaehlertyp::VARIANTS {
        let wire = variant.as_wire();
        assert_eq!(Zaehlertyp::from_wire(wire), Ok(variant));
    }
    // Unknown renders to "UNKNOWN" but does not strictly parse back.
    assert_eq!(Zaehlertyp::Unknown.as_wire(), "UNKNOWN");
    assert!(Zaehlertyp::from_wire(Zaehlertyp::Unknown.as_wire()).is_err());
}

#[test]
fn is_known_is_unknown() {
    assert!(Zaehlertyp::Wasserzaehler.is_known());
    assert!(!Zaehlertyp::Wasserzaehler.is_unknown());
    assert!(Zaehlertyp::Unknown.is_unknown());
    assert!(!Zaehlertyp::Unknown.is_known());
}

// ─── BUG-3: detecting lenient-decode fall-through (serde path) ──────────────

#[cfg(feature = "json")]
#[test]
fn lenient_decode_falls_through_but_is_detectable() {
    // The serde path is intentionally lenient: an unknown value decodes to Unknown.
    let z: Zaehlertyp = serde_json::from_value(serde_json::json!("NOT_REAL")).unwrap();
    assert_eq!(z, Zaehlertyp::Unknown);
    // ...but is_unknown() lets the ingest boundary reject it in one call.
    assert!(z.is_unknown());
    // Whereas the strict parser rejects up front.
    assert!(Zaehlertyp::from_wire("NOT_REAL").is_err());
}

// ─── BUG-2: the two iMSys spellings are distinct on the wire ────────────────

#[test]
fn imsys_spelling_differs_across_bos() {
    // Zaehlertyp: three `s`.
    assert_eq!(
        Zaehlertyp::IntelligentesMesssystem.as_wire(),
        "INTELLIGENTES_MESSSYSTEM"
    );
    // Geraetetyp: two `s` — faithful to the (internally inconsistent) upstream schema.
    assert_eq!(
        Geraetetyp::IntelligentesMessystem.as_wire(),
        "INTELLIGENTES_MESSYSTEM"
    );
    // The two wire strings are genuinely different, and each BO rejects the other's.
    assert!(Zaehlertyp::from_wire("INTELLIGENTES_MESSYSTEM").is_err());
    assert!(Geraetetyp::from_wire("INTELLIGENTES_MESSSYSTEM").is_err());
}
