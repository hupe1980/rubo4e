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

// ── Date serde round-trips (opt_date_serde / date_serde) ─────────────────────

/// Inline strategy: generates a valid `time::Date` in the range used by BO4E
/// (1900-01-01 to 2099-12-28; clamped to avoid leap-year edge cases without
/// needing month-length tables).
#[cfg(all(feature = "time", feature = "versioned"))]
mod date_roundtrips {
    use proptest::prelude::*;
    use time::Date;

    /// Any calendar date in 1900–2099 with day capped at 28 to avoid needing
    /// a leap-year table while still covering all 12 months.
    fn any_date() -> impl Strategy<Value = Date> {
        (1900i32..=2099i32, 1u8..=12u8, 1u8..=28u8).prop_map(|(y, m, d)| {
            Date::from_calendar_date(y, time::Month::try_from(m).unwrap(), d)
                .expect("day ≤28 is always valid for any month")
        })
    }

    proptest! {
        /// `date_serde` (required `time::Date` field): serialize → deserialize identity.
        #[test]
        fn required_date_serde_roundtrip(date in any_date()) {
            #[derive(serde::Serialize, serde::Deserialize)]
            struct Wrapper {
                #[serde(with = "rubo4e::time_serde::date_serde")]
                date: Date,
            }
            let w = Wrapper { date };
            let json = serde_json::to_string(&w).expect("serialize");
            let back: Wrapper = serde_json::from_str(&json).expect("deserialize");
            prop_assert_eq!(date, back.date, "date serde round-trip failed: json={}", json);
        }

        /// `opt_date_serde` (optional `time::Date` field): `Some(date)` round-trip.
        #[test]
        fn optional_date_serde_roundtrip_some(date in any_date()) {
            #[derive(serde::Serialize, serde::Deserialize)]
            struct Wrapper {
                #[serde(with = "rubo4e::time_serde::opt_date_serde")]
                date: Option<Date>,
            }
            let w = Wrapper { date: Some(date) };
            let json = serde_json::to_string(&w).expect("serialize");
            let back: Wrapper = serde_json::from_str(&json).expect("deserialize");
            prop_assert_eq!(
                Some(date),
                back.date,
                "opt_date_serde round-trip failed: json={}",
                json
            );
        }
    }

    /// `opt_date_serde`: `None` round-trip (unit test, no proptest needed).
    #[test]
    fn optional_date_serde_roundtrip_none() {
        #[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
        struct Wrapper {
            #[serde(with = "rubo4e::time_serde::opt_date_serde")]
            date: Option<Date>,
        }
        let w = Wrapper { date: None };
        let json = serde_json::to_string(&w).expect("serialize");
        let back: Wrapper = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(w, back);
    }

    /// `opt_date_serde`: explicit JSON `null` deserializes to `None`.
    #[test]
    fn optional_date_serde_null_is_none() {
        #[derive(serde::Deserialize, PartialEq, Debug)]
        struct Wrapper {
            #[serde(with = "rubo4e::time_serde::opt_date_serde")]
            date: Option<Date>,
        }
        let back: Wrapper = serde_json::from_str(r#"{"date":null}"#).expect("deserialize");
        assert_eq!(back.date, None);
    }

    /// `date_serde` and `opt_date_serde` use the ISO 8601 `YYYY-MM-DD` wire format.
    #[test]
    fn date_wire_format_is_iso8601() {
        #[derive(serde::Serialize)]
        struct Wrapper {
            #[serde(with = "rubo4e::time_serde::date_serde")]
            date: Date,
        }
        use time::macros::date;
        let w = Wrapper { date: date!(2025 - 06 - 15) };
        let json = serde_json::to_string(&w).expect("serialize");
        assert_eq!(json, r#"{"date":"2025-06-15"}"#);
    }
}
