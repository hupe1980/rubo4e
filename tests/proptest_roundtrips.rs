//! Property-based round-trip tests using proptest (E17-S57).
//!
//! Verifies that all identifier types survive Display → FromStr and
//! serde serialization → deserialization round-trips for any valid value.
//!
//! Run with:
//! ```text
//! cargo test --test proptest_roundtrips --all-features
//! ```

// ─── Inline strategies (no dependency on library-private Arbitrary impls) ────

/// BDEW alternating-weight check digit (mirrors `src/identifiers/checksum.rs`).
fn bdew_check_digit(digits: &[u8; 10]) -> u8 {
    const WEIGHTS: [u8; 10] = [2, 1, 2, 1, 2, 1, 2, 1, 2, 1];
    let sum: u32 = digits
        .iter()
        .zip(WEIGHTS)
        .map(|(&d, w)| {
            let p = u32::from(d) * u32::from(w);
            if p >= 10 {
                p - 9
            } else {
                p
            }
        })
        .sum();
    ((10 - (sum % 10)) % 10) as u8
}

mod identifier_roundtrips {
    use proptest::prelude::*;

    fn valid_11digit() -> impl Strategy<Value = String> {
        prop::array::uniform10(0u8..=9u8).prop_map(|prefix| {
            let check = super::bdew_check_digit(&prefix);
            prefix
                .iter()
                .chain(std::iter::once(&check))
                .map(|&d| char::from_digit(u32::from(d), 10).unwrap())
                .collect::<String>()
        })
    }

    fn valid_melo_id() -> impl Strategy<Value = String> {
        prop::string::string_regex("[A-Z]{2}[A-Z0-9]{31}").expect("valid MeloId regex")
    }

    fn valid_nelo_id() -> impl Strategy<Value = String> {
        prop::string::string_regex("[A-Z0-9a-z]{11}").expect("valid NeloId regex")
    }

    fn valid_obis_code() -> impl Strategy<Value = String> {
        (
            0u8..=255u8,
            0u8..=255u8,
            1u8..=255u8,
            0u8..=255u8,
            0u8..=255u8,
            0u8..=255u8,
        )
            .prop_map(|(a, b, c, d, e, f)| format!("{a}-{b}:{c}.{d}.{e}*{f}"))
    }

    fn valid_eic_code() -> impl Strategy<Value = String> {
        use rubo4e::identifiers::EicCode;
        let type_chars = prop_oneof![
            Just('A'),
            Just('T'),
            Just('V'),
            Just('W'),
            Just('X'),
            Just('Y'),
            Just('Z'),
        ];
        let lio = prop::string::string_regex("[A-Z0-9]{2}").expect("LIO regex");
        let body = prop::string::string_regex("[A-Z0-9\\-]{12}").expect("body regex");
        (lio, type_chars, body).prop_filter_map("EIC check char not '-'", |(lio, tc, body)| {
            let prefix = format!("{lio}{tc}{body}");
            let check = EicCode::compute_check_char(&prefix)?;
            Some(format!("{prefix}{check}"))
        })
    }

    proptest! {
        #[test]
        fn malo_id_display_from_str_roundtrip(s in valid_11digit()) {
            let id = rubo4e::identifiers::MaloId::new(&s).expect("valid MaloId");
            let displayed = id.to_string();
            let parsed: rubo4e::identifiers::MaloId = displayed.parse()
                .expect("MaloId::from_str must succeed for valid ID");
            prop_assert_eq!(id.as_ref(), parsed.as_ref());
        }

        #[test]
        fn melo_id_display_from_str_roundtrip(s in valid_melo_id()) {
            let id = rubo4e::identifiers::MeloId::new(&s).expect("valid MeloId");
            let displayed = id.to_string();
            let parsed: rubo4e::identifiers::MeloId = displayed.parse()
                .expect("MeloId::from_str must succeed for valid ID");
            prop_assert_eq!(id.as_ref(), parsed.as_ref());
        }

        #[test]
        fn nelo_id_display_from_str_roundtrip(s in valid_nelo_id()) {
            let id = rubo4e::identifiers::NeloId::new(&s).expect("valid NeloId");
            let displayed = id.to_string();
            let parsed: rubo4e::identifiers::NeloId = displayed.parse()
                .expect("NeloId::from_str must succeed for valid ID");
            prop_assert_eq!(id.as_ref(), parsed.as_ref());
        }

        #[test]
        fn sr_id_display_from_str_roundtrip(s in valid_11digit()) {
            let id = rubo4e::identifiers::SrId::new(&s).expect("valid SrId");
            let displayed = id.to_string();
            let parsed: rubo4e::identifiers::SrId = displayed.parse()
                .expect("SrId::from_str must succeed for valid ID");
            prop_assert_eq!(id.as_ref(), parsed.as_ref());
        }

        #[test]
        fn tr_id_display_from_str_roundtrip(s in valid_11digit()) {
            let id = rubo4e::identifiers::TrId::new(&s).expect("valid TrId");
            let displayed = id.to_string();
            let parsed: rubo4e::identifiers::TrId = displayed.parse()
                .expect("TrId::from_str must succeed for valid ID");
            prop_assert_eq!(id.as_ref(), parsed.as_ref());
        }

        #[test]
        fn obis_code_display_from_str_roundtrip(s in valid_obis_code()) {
            let code = rubo4e::identifiers::ObisCode::new(&s).expect("valid ObisCode");
            let displayed = code.to_string();
            let parsed: rubo4e::identifiers::ObisCode = displayed.parse()
                .expect("ObisCode::from_str must succeed for valid code");
            prop_assert_eq!(code.as_ref(), parsed.as_ref());
        }

        #[test]
        fn eic_code_display_from_str_roundtrip(s in valid_eic_code()) {
            let code = rubo4e::identifiers::EicCode::new(&s).expect("valid EicCode");
            let displayed = code.to_string();
            let parsed: rubo4e::identifiers::EicCode = displayed.parse()
                .expect("EicCode::from_str must succeed for valid code");
            prop_assert_eq!(code.as_ref(), parsed.as_ref());
        }
    }
}

#[cfg(feature = "serde")]
mod identifier_serde_roundtrips {
    use proptest::prelude::*;

    fn valid_11digit() -> impl Strategy<Value = String> {
        prop::array::uniform10(0u8..=9u8).prop_map(|prefix| {
            let check = super::bdew_check_digit(&prefix);
            prefix
                .iter()
                .chain(std::iter::once(&check))
                .map(|&d| char::from_digit(u32::from(d), 10).unwrap())
                .collect::<String>()
        })
    }

    fn valid_eic_code() -> impl Strategy<Value = String> {
        use rubo4e::identifiers::EicCode;
        let type_chars = prop_oneof![
            Just('A'),
            Just('T'),
            Just('V'),
            Just('W'),
            Just('X'),
            Just('Y'),
            Just('Z'),
        ];
        let lio = prop::string::string_regex("[A-Z0-9]{2}").expect("LIO regex");
        let body = prop::string::string_regex("[A-Z0-9\\-]{12}").expect("body regex");
        (lio, type_chars, body).prop_filter_map("EIC check char not '-'", |(lio, tc, body)| {
            let prefix = format!("{lio}{tc}{body}");
            let check = EicCode::compute_check_char(&prefix)?;
            Some(format!("{prefix}{check}"))
        })
    }

    proptest! {
        #[test]
        fn malo_id_serde_roundtrip(s in valid_11digit()) {
            let id = rubo4e::identifiers::MaloId::new(&s).expect("valid MaloId");
            let json = serde_json::to_string(&id).expect("serialize");
            let back: rubo4e::identifiers::MaloId = serde_json::from_str(&json).expect("deserialize");
            prop_assert_eq!(id.as_ref(), back.as_ref());
        }

        #[test]
        fn eic_code_serde_roundtrip(s in valid_eic_code()) {
            let code = rubo4e::identifiers::EicCode::new(&s).expect("valid EicCode");
            let json = serde_json::to_string(&code).expect("serialize");
            let back: rubo4e::identifiers::EicCode = serde_json::from_str(&json).expect("deserialize");
            prop_assert_eq!(code.as_ref(), back.as_ref());
        }
    }
}

#[cfg(all(feature = "strum", feature = "versioned"))]
mod enum_roundtrips {
    use proptest::prelude::*;
    use std::str::FromStr;
    use strum::IntoEnumIterator as _;

    fn any_sparte() -> impl Strategy<Value = rubo4e::v202501::Sparte> {
        let variants: Vec<_> = rubo4e::v202501::Sparte::iter().collect();
        proptest::sample::select(variants)
    }

    fn any_bo_typ() -> impl Strategy<Value = rubo4e::v202501::BoTyp> {
        let variants: Vec<_> = rubo4e::v202501::BoTyp::iter().collect();
        proptest::sample::select(variants)
    }

    proptest! {
        #[test]
        fn sparte_display_from_str_roundtrip(variant in any_sparte()) {
            let displayed = variant.to_string();
            let parsed = rubo4e::v202501::Sparte::from_str(&displayed)
                .expect("Sparte::from_str should succeed for any Display output");
            prop_assert_eq!(variant, parsed);
        }

        #[test]
        fn bo_typ_display_from_str_roundtrip(variant in any_bo_typ()) {
            let displayed = variant.to_string();
            let parsed = rubo4e::v202501::BoTyp::from_str(&displayed)
                .expect("BoTyp::from_str should succeed for any Display output");
            prop_assert_eq!(variant, parsed);
        }
    }
}
