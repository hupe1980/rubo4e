//! Integration tests for the `strum` feature (E15-S52, E15-S53).
//!
//! Verifies:
//! - `Display` and `FromStr` are derived on all generated enums
//! - Unknown strings return `Err(strum::ParseError::VariantNotFound)`
//! - `IntoStaticStr` and `AsRefStr` are available
//! - `EnumIter` iterates all variants (including `Unknown`)
//!
//! Run with:
//! ```text
//! cargo test --test strum_integration --features strum,versioned
//! ```

#[cfg(all(feature = "strum", feature = "versioned"))]
mod strum_tests {
    use std::str::FromStr;
    use strum::IntoEnumIterator as _;

    #[test]
    fn display_produces_variant_name() {
        use rubo4e::v202501::Sparte;
        // H-08: strum::Display must produce the canonical BO4E JSON string,
        // identical to what serde serializes — not the Rust PascalCase variant name.
        assert_eq!(Sparte::Strom.to_string(), "STROM");
        assert_eq!(Sparte::Gas.to_string(), "GAS");
        // L-01: Unknown uses SCREAMING_SNAKE_CASE like all other BO4E enum values.
        assert_eq!(Sparte::Unknown.to_string(), "UNKNOWN");
    }

    #[test]
    fn from_str_roundtrips_known_variants() {
        use rubo4e::v202501::Sparte;
        // H-08: FromStr must accept the canonical BO4E JSON strings.
        assert_eq!(Sparte::from_str("STROM").unwrap(), Sparte::Strom);
        assert_eq!(Sparte::from_str("GAS").unwrap(), Sparte::Gas);
        // L-01: Unknown uses SCREAMING_SNAKE_CASE.
        assert_eq!(Sparte::from_str("UNKNOWN").unwrap(), Sparte::Unknown);
    }

    #[test]
    fn unknown_string_returns_variant_not_found() {
        use rubo4e::v202501::Sparte;
        let result = Sparte::from_str("Invalid");
        assert_eq!(result, Err(strum::ParseError::VariantNotFound));
    }

    #[test]
    fn into_static_str_returns_variant_name() {
        use rubo4e::v202501::Sparte;
        // H-08: IntoStaticStr derive adds `From<Sparte> for &'static str`.
        // Use the canonical conversion — no trait import required since it goes
        // through the derived `From` impl, not a method on the `IntoStaticStr` trait.
        let s: &'static str = <&'static str>::from(Sparte::Strom);
        assert_eq!(s, "STROM");
    }

    #[test]
    fn as_ref_str_returns_variant_name() {
        use rubo4e::v202501::Sparte;
        // H-08: AsRef<str> returns the canonical BO4E JSON string.
        assert_eq!(Sparte::Gas.as_ref(), "GAS");
    }

    #[test]
    fn enum_iter_includes_all_variants() {
        use rubo4e::v202501::Sparte;
        let variants: Vec<Sparte> = Sparte::iter().collect();
        // Must include all known variants and the catch-all Unknown
        assert!(variants.contains(&Sparte::Strom));
        assert!(variants.contains(&Sparte::Gas));
        assert!(variants.contains(&Sparte::Unknown));
        assert!(
            variants.len() >= 7,
            "expected at least 7 Sparte variants, got {}",
            variants.len()
        );
    }

    #[test]
    fn enum_iter_works_on_bo_typ() {
        use rubo4e::v202501::BoTyp;
        let variants: Vec<BoTyp> = BoTyp::iter().collect();
        assert!(!variants.is_empty(), "BoTyp should have variants");
        assert!(variants.contains(&BoTyp::Vertrag));
        assert!(variants.contains(&BoTyp::Marktlokation));
    }

    #[test]
    fn all_sparte_variants_round_trip_display_from_str() {
        use rubo4e::v202501::Sparte;
        for variant in Sparte::iter() {
            let displayed = variant.to_string();
            let parsed = Sparte::from_str(&displayed)
                .unwrap_or_else(|_| panic!("could not parse back '{displayed}' for {variant:?}"));
            assert_eq!(
                variant, parsed,
                "Display/FromStr round-trip failed for {variant:?}"
            );
        }
    }

    #[test]
    fn iter_known_excludes_unknown() {
        use rubo4e::v202501::Sparte;
        let known: Vec<Sparte> = Sparte::iter_known().collect();
        // iter_known() must not include the Unknown catch-all
        assert!(
            !known.contains(&Sparte::Unknown),
            "Unknown must not appear in iter_known()"
        );
        // iter_known() must include all schema-defined variants
        assert!(
            known.contains(&Sparte::Strom),
            "Strom must appear in iter_known()"
        );
        assert!(
            known.contains(&Sparte::Gas),
            "Gas must appear in iter_known()"
        );
        // iter_known() must yield fewer variants than iter() (which includes Unknown)
        let all: Vec<Sparte> = Sparte::iter().collect();
        assert_eq!(
            known.len() + 1,
            all.len(),
            "iter_known() should have exactly one fewer variant than iter()"
        );
    }

    #[test]
    fn iter_known_bo_typ_no_unknown() {
        use rubo4e::v202501::BoTyp;
        let known: Vec<BoTyp> = BoTyp::iter_known().collect();
        assert!(
            !known.is_empty(),
            "iter_known() must yield at least one BoTyp"
        );
        assert!(
            !known.contains(&BoTyp::Unknown),
            "Unknown must not appear in BoTyp::iter_known()"
        );
        assert!(
            known.contains(&BoTyp::Vertrag),
            "Vertrag must appear in BoTyp::iter_known()"
        );
    }
}
