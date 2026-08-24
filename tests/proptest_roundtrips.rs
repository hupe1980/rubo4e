//! Property-based round-trip tests using proptest (E17-S57).
//!
//! Verifies that all identifier types survive Display → FromStr and
//! serde serialization → deserialization round-trips for any valid value.
//!
//! Run with:
//! ```text
//! cargo test --test proptest_roundtrips --all-features
//! ```

// ─── Shared strategies ───────────────────────────────────────────────────────
//
// These build identifiers through the crate's **public** `from_base` constructors
// rather than mirroring the check-digit arithmetic: a test that reimplements the
// algorithm agrees with the implementation even when both disagree with BDEW.
// The reference vectors that pin the arithmetic live in
// `src/identifiers/checksum.rs`.

use proptest::prelude::*;

/// Uppercase alphanumeric body characters permitted by BDEW §8.2.
const ALNUM: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

/// A valid MaLo-ID: Vergabestelle digit `1`-`9`, nine free digits, check digit.
fn valid_malo_id() -> impl Strategy<Value = String> {
    prop::string::string_regex("[1-9][0-9]{9}")
        .expect("MaLo base regex")
        .prop_map(|base| {
            rubo4e::identifiers::MaloId::from_base(&base)
                .expect("generated base is valid")
                .to_string()
        })
}

/// A valid 13-digit Marktpartner-ID carrying the BDEW §8.1 check digit.
fn valid_marktpartner_id() -> impl Strategy<Value = String> {
    prop::string::string_regex("[0-9]{12}")
        .expect("MP-ID base regex")
        .prop_map(|base| {
            rubo4e::identifiers::MarktpartnerId::from_base(&base)
                .expect("generated base is valid")
                .to_string()
        })
}

/// Builds a strategy for one member of the §8.2 ASCII-Verfahren family.
macro_rules! valid_ascii_id {
    ($name:ident, $ty:ty, $body_len:expr) => {
        fn $name() -> impl Strategy<Value = String> {
            prop::collection::vec(prop::sample::select(ALNUM.as_bytes()), $body_len).prop_map(
                |body| {
                    let mut base = String::from(<$ty>::CODETYP);
                    base.extend(body.iter().map(|&b| b as char));
                    <$ty>::from_base(&base)
                        .expect("generated base is valid")
                        .to_string()
                },
            )
        }
    };
}

valid_ascii_id!(valid_nelo_id, rubo4e::identifiers::NeloId, 9);
valid_ascii_id!(valid_nebe_id, rubo4e::identifiers::NebeId, 9);
valid_ascii_id!(valid_cr_id, rubo4e::identifiers::CrId, 9);
valid_ascii_id!(valid_sg_id, rubo4e::identifiers::SgId, 9);
valid_ascii_id!(valid_sr_id, rubo4e::identifiers::SrId, 9);
valid_ascii_id!(valid_tr_id, rubo4e::identifiers::TrId, 9);
valid_ascii_id!(valid_paket_id, rubo4e::identifiers::PaketId, 8);

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

fn valid_melo_id() -> impl Strategy<Value = String> {
    prop::string::string_regex("[A-Z]{2}[A-Z0-9]{31}").expect("valid MeloId regex")
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

/// Builds a strategy for an EIC code pinned to one object-type character.
///
/// Used for the EIC-restricted identifiers, whose whole point is that position 3
/// is fixed — generating the character randomly would not exercise them.
fn valid_eic_of_type(type_char: char) -> impl Strategy<Value = String> {
    use rubo4e::identifiers::EicCode;
    let lio = prop::string::string_regex("[A-Z0-9]{2}").expect("LIO regex");
    let body = prop::string::string_regex("[A-Z0-9\\-]{12}").expect("body regex");
    (lio, body).prop_filter_map("EIC check char not '-'", move |(lio, body)| {
        let prefix = format!("{lio}{type_char}{body}");
        let check = EicCode::compute_check_char(&prefix)?;
        Some(format!("{prefix}{check}"))
    })
}

fn valid_bilanzkreis_id() -> impl Strategy<Value = String> {
    valid_eic_of_type('X')
}

fn valid_bilanzierungsgebiet_id() -> impl Strategy<Value = String> {
    valid_eic_of_type('Y')
}

/// The identifier types under test, paired with a strategy that produces valid
/// values. Both the wire-format and the serde laws are asserted over this one
/// table, so adding an identifier here covers it everywhere.
macro_rules! for_each_identifier {
    ($mac:ident) => {
        $mac!(malo_id, rubo4e::identifiers::MaloId, valid_malo_id);
        $mac!(
            marktpartner_id,
            rubo4e::identifiers::MarktpartnerId,
            valid_marktpartner_id
        );
        $mac!(melo_id, rubo4e::identifiers::MeloId, valid_melo_id);
        $mac!(nelo_id, rubo4e::identifiers::NeloId, valid_nelo_id);
        $mac!(nebe_id, rubo4e::identifiers::NebeId, valid_nebe_id);
        $mac!(cr_id, rubo4e::identifiers::CrId, valid_cr_id);
        $mac!(sg_id, rubo4e::identifiers::SgId, valid_sg_id);
        $mac!(sr_id, rubo4e::identifiers::SrId, valid_sr_id);
        $mac!(tr_id, rubo4e::identifiers::TrId, valid_tr_id);
        $mac!(paket_id, rubo4e::identifiers::PaketId, valid_paket_id);
        $mac!(eic_code, rubo4e::identifiers::EicCode, valid_eic_code);
        $mac!(obis_code, rubo4e::identifiers::ObisCode, valid_obis_code);
        $mac!(
            bilanzkreis_id,
            rubo4e::identifiers::BilanzkreisId,
            valid_bilanzkreis_id
        );
        $mac!(
            bilanzierungsgebiet_id,
            rubo4e::identifiers::BilanzierungsgebietId,
            valid_bilanzierungsgebiet_id
        );
    };
}

/// `Display` → `FromStr` must be the identity for every identifier.
///
/// These traits are unconditional (no feature gate), so this module is too.
mod display_from_str_roundtrips {
    use super::*;

    macro_rules! law {
        ($name:ident, $ty:ty, $strategy:ident) => {
            proptest! {
                #[test]
                fn $name(s in $strategy()) {
                    let id = <$ty>::new(&s).expect("strategy must produce a valid identifier");
                    let parsed: $ty = id.to_string().parse()
                        .expect("FromStr must accept this type's own Display output");
                    prop_assert_eq!(&id, &parsed);
                    prop_assert_eq!(id.as_ref(), s.as_str());
                }
            }
        };
    }

    for_each_identifier!(law);
}

/// `Serialize` → `Deserialize` must be the identity, and identifiers must be
/// transparent on the wire (a bare JSON string, not a wrapper object).
#[cfg(feature = "serde")]
mod serde_roundtrips {
    use super::*;

    macro_rules! law {
        ($name:ident, $ty:ty, $strategy:ident) => {
            proptest! {
                #[test]
                fn $name(s in $strategy()) {
                    let id = <$ty>::new(&s).expect("strategy must produce a valid identifier");
                    let json = serde_json::to_string(&id).expect("serialize");
                    prop_assert_eq!(&json, &format!("\"{}\"", s));
                    let back: $ty = serde_json::from_str(&json).expect("deserialize");
                    prop_assert_eq!(&id, &back);
                }
            }
        };
    }

    for_each_identifier!(law);
}

/// A mutated check digit must be rejected. This is the property that actually
/// guards against typos, so it is asserted for every generated value rather than
/// for a handful of fixtures.
mod check_digit_rejects_mutation {
    use super::*;

    macro_rules! law {
        ($name:ident, $ty:ty, $strategy:ident) => {
            proptest! {
                #[test]
                fn $name(s in $strategy()) {
                    let bytes = s.as_bytes();
                    let last = bytes[bytes.len() - 1] - b'0';
                    for delta in 1..10u8 {
                        let mut mutated = bytes.to_vec();
                        let n = mutated.len();
                        mutated[n - 1] = b'0' + (last + delta) % 10;
                        let candidate = String::from_utf8(mutated).unwrap();
                        prop_assert!(
                            <$ty>::new(&candidate).is_err(),
                            "{} has a mutated check digit and must be rejected",
                            candidate
                        );
                    }
                }
            }
        };
    }

    law!(malo_id, rubo4e::identifiers::MaloId, valid_malo_id);
    law!(nelo_id, rubo4e::identifiers::NeloId, valid_nelo_id);
    law!(nebe_id, rubo4e::identifiers::NebeId, valid_nebe_id);
    law!(cr_id, rubo4e::identifiers::CrId, valid_cr_id);
    law!(sg_id, rubo4e::identifiers::SgId, valid_sg_id);
    law!(sr_id, rubo4e::identifiers::SrId, valid_sr_id);
    law!(tr_id, rubo4e::identifiers::TrId, valid_tr_id);
    law!(paket_id, rubo4e::identifiers::PaketId, valid_paket_id);
}

#[cfg(all(feature = "strum", feature = "versioned"))]
mod enum_roundtrips {
    use proptest::prelude::*;
    use std::str::FromStr;
    use strum::IntoEnumIterator as _;

    fn any_sparte() -> impl Strategy<Value = rubo4e::v202607::Sparte> {
        let variants: Vec<_> = rubo4e::v202607::Sparte::iter().collect();
        proptest::sample::select(variants)
    }

    fn any_bo_typ() -> impl Strategy<Value = rubo4e::v202607::BoTyp> {
        let variants: Vec<_> = rubo4e::v202607::BoTyp::iter().collect();
        proptest::sample::select(variants)
    }

    proptest! {
        #[test]
        fn sparte_display_from_str_roundtrip(variant in any_sparte()) {
            let displayed = variant.to_string();
            let parsed = rubo4e::v202607::Sparte::from_str(&displayed)
                .expect("Sparte::from_str should succeed for any Display output");
            prop_assert_eq!(variant, parsed);
        }

        #[test]
        fn bo_typ_display_from_str_roundtrip(variant in any_bo_typ()) {
            let displayed = variant.to_string();
            let parsed = rubo4e::v202607::BoTyp::from_str(&displayed)
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
        let w = Wrapper {
            date: date!(2025 - 06 - 15),
        };
        let json = serde_json::to_string(&w).expect("serialize");
        assert_eq!(json, r#"{"date":"2025-06-15"}"#);
    }
}

/// Laws specific to the EIC object-type character (position 3).
mod eic_object_type {
    use super::*;
    use rubo4e::identifiers::{BilanzierungsgebietId, BilanzkreisId, EicCode, EicType};

    proptest! {
        /// Position 3 of a valid EIC must always round-trip through `EicType`.
        ///
        /// `eic_type()` uses an `expect` justified by construction validating the
        /// character, so this is the property that keeps the `expect` honest.
        #[test]
        fn eic_type_matches_position_three(s in valid_eic_code()) {
            let eic = EicCode::new(&s).expect("strategy produces valid EIC");
            let ty = eic.eic_type();
            prop_assert_eq!(ty.as_char(), eic.type_char());
            prop_assert_eq!(EicType::from_char(eic.type_char()), Some(ty));
            prop_assert_eq!(s.as_bytes()[2] as char, ty.as_char());
        }
    }

    proptest! {
        /// A Bilanzkreis is a party (`X`) and a Bilanzierungsgebiet is an area
        /// (`Y`); neither may accept the other's codes.
        #[test]
        fn restricted_types_accept_only_their_own_object_type(
            bk in valid_bilanzkreis_id(),
            bg in valid_bilanzierungsgebiet_id(),
        ) {
            let bk_id = BilanzkreisId::new(&bk).expect("X-type code is a Bilanzkreis-ID");
            let bg_id = BilanzierungsgebietId::new(&bg).expect("Y-type code is a Bilanzierungsgebiet-ID");

            prop_assert_eq!(bk_id.to_eic_code().eic_type(), EicType::Party);
            prop_assert_eq!(bg_id.to_eic_code().eic_type(), EicType::Area);

            prop_assert!(BilanzierungsgebietId::new(&bk).is_err(), "{} is a party code", bk);
            prop_assert!(BilanzkreisId::new(&bg).is_err(), "{} is an area code", bg);
        }
    }
}

/// `ObisCode` stores a canonical form, so equality is semantic rather than textual.
mod obis_canonicalisation {
    use super::*;
    use rubo4e::identifiers::ObisCode;

    /// Re-spells an OBIS code with random leading zeros and the alternative `&`
    /// separator — a different string denoting the same value.
    fn respelled(code: &str, pad: usize) -> String {
        let zeros = "0".repeat(pad);
        let mut out = String::with_capacity(code.len() + pad * 6);
        let mut at_group_start = true;
        for ch in code.chars() {
            if ch.is_ascii_digit() {
                if at_group_start {
                    out.push_str(&zeros);
                    at_group_start = false;
                }
                out.push(ch);
            } else {
                at_group_start = true;
                out.push(if ch == '*' { '&' } else { ch });
            }
        }
        out
    }

    proptest! {
        /// Any spelling of a value parses to the same `ObisCode`: equal, equally
        /// hashed, and rendered identically.
        #[test]
        fn spelling_does_not_affect_identity(s in valid_obis_code(), pad in 1usize..4) {
            use std::collections::HashSet;

            let canonical = ObisCode::new(&s).expect("strategy produces valid OBIS");
            let variant = ObisCode::new(&respelled(&s, pad))
                .expect("a re-spelling of a valid OBIS code is still valid");

            prop_assert_eq!(&canonical, &variant);
            prop_assert_eq!(canonical.as_str(), variant.as_str());
            prop_assert_eq!(canonical.components(), variant.components());

            let set: HashSet<_> = [canonical, variant].into_iter().collect();
            prop_assert_eq!(set.len(), 1);
        }
    }

    proptest! {
        /// Canonicalisation is idempotent, so serialize → deserialize is stable.
        #[test]
        fn canonical_form_is_a_fixed_point(s in valid_obis_code()) {
            let once = ObisCode::new(&s).expect("valid OBIS");
            let twice = ObisCode::new(once.as_str()).expect("canonical form re-parses");
            prop_assert_eq!(once.as_str(), twice.as_str());
            prop_assert_eq!(&once, &twice);
        }
    }
}
