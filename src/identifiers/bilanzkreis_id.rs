//! EIC codes pinned to a single ENTSO-E object type.
//!
//! The German electricity market reuses the ENTSO-E EIC namespace for two roles
//! that are distinguished *only* by the position-3 object-type character:
//!
//! | Type | Object type | Role |
//! |------|-------------|------|
//! | [`BilanzkreisId`] | `X` — Party | Bilanzkreis (balance group) |
//! | [`BilanzierungsgebietId`] | `Y` — Area | Bilanzierungsgebiet (balancing area) |
//!
//! Both are 16-character EIC codes with an ENTSO-E check character; giving each
//! its own Rust type means a Bilanzkreis cannot be passed where a
//! Bilanzierungsgebiet is expected.
//!
//! Codes are issued by Energie Codes und Services GmbH (ECS) on behalf of BDEW,
//! which is why real-world values start with the `11` Local Issuing Office
//! prefix — `11X…` for balance groups, `11Y…` for balancing areas.

use crate::identifiers::EicType;

eic_restricted_identifier! {
    /// Bilanzkreis-ID: a 16-character EIC code with object type `'X'` (Party).
    ///
    /// A Bilanzkreis (balance group) is held by a Bilanzkreisverantwortlicher, a
    /// market participant — so ENTSO-E classifies it as a **party**, not an area.
    /// BDEW/ECS issue these codes in the `11X…` range.
    ///
    /// Used in:
    /// - **MaBiS** (BK6-06-009) — electricity balance-group settlement
    /// - **GaBi Gas** (BK7-14-020) — gas balance-group settlement
    /// - **EDIFACT** — `NAD`/`LOC` segments with DE3227 qualifier `Z01`
    ///
    /// ## Not a Bilanzierungsgebiet
    ///
    /// A Bilanzierungs*gebiet* (balancing area) is a different object with a
    /// different type character — see [`BilanzierungsgebietId`].
    ///
    /// ## Format
    ///
    /// - Positions 1–2: Local Issuing Office (`11` for BDEW/ECS)
    /// - Position 3: **always `'X'`** (Party)
    /// - Positions 4–15: LIO-specific body, `-` padded on the right
    /// - Position 16: ENTSO-E check character
    BilanzkreisId,
    eic_type  = EicType::Party,
    schema    = "crate::schema_helpers::bilanzkreis_id_schema",
    schema_meta = crate::identifiers::schema::BILANZKREIS_ID,
    pattern   = r"^[A-Z0-9]{2}X[A-Z0-9-]{12}[A-Z0-9]$",
    expecting = "a 16-character EIC code with object type 'X' (Bilanzkreis-ID)",
    example   = "11XSUEDWESTSTRO8",
    check_fn  = check_bilanzkreis_id,
}

eic_restricted_identifier! {
    /// Bilanzierungsgebiet-ID: a 16-character EIC code with object type `'Y'` (Area).
    ///
    /// A Bilanzierungsgebiet (balancing area) is the grid area a Marktlokation is
    /// assigned to for balancing purposes. Codes are issued by the TSO responsible
    /// for the control area, in the BDEW/ECS `11Y…` range — e.g. `11YR…` for
    /// Amprion, `11YW…` for TransnetBW.
    ///
    /// Used in:
    /// - **MaBiS** (BK6-06-009) — assignment of Marktlokationen to balancing areas
    /// - `StandorteigenschaftenStrom.bilanzierungsgebietEic`
    ///
    /// ## Not a Bilanzkreis
    ///
    /// A Bilanzkreis (balance group) is a market party with type `'X'` — see
    /// [`BilanzkreisId`].
    ///
    /// ## Format
    ///
    /// - Positions 1–2: Local Issuing Office (`11` for BDEW/ECS)
    /// - Position 3: **always `'Y'`** (Area or Domain)
    /// - Positions 4–15: LIO-specific body, `-` padded on the right
    /// - Position 16: ENTSO-E check character
    BilanzierungsgebietId,
    eic_type  = EicType::Area,
    schema    = "crate::schema_helpers::bilanzierungsgebiet_id_schema",
    schema_meta = crate::identifiers::schema::BILANZIERUNGSGEBIET_ID,
    pattern   = r"^[A-Z0-9]{2}Y[A-Z0-9-]{12}[A-Z0-9]$",
    expecting = "a 16-character EIC code with object type 'Y' (Bilanzierungsgebiet-ID)",
    example   = "11YN-0000-0001-Q",
    check_fn  = check_bilanzierungsgebiet_id,
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::IdentifierError;
    use crate::identifiers::EicCode;

    /// Real Bilanzkreis codes published by German market participants.
    ///
    /// Testing against real codes rather than synthetic ones is what pins the
    /// object type to `'X'` (Party): no registry issues a Bilanzkreis under any
    /// other type.
    const REAL_BILANZKREISE: [&str; 3] =
        ["11XSUEDWESTSTRO8", "11XENERGIE2----H", "11XENAGISME----J"];

    /// Real Bilanzierungsgebiet codes from the TSOs' published VNB EIC list.
    ///
    /// Note the embedded `-` characters: ENTSO-E uses `-` as padding but does not
    /// require it to be right-aligned, so the body must not assume otherwise.
    const REAL_BILANZIERUNGSGEBIETE: [&str; 4] = [
        "11YN-0000-0001-Q",
        "11YN-0000-0002-N",
        "11YN-0000-0007-8",
        "11YN10002949-01Z",
    ];

    #[test]
    fn accepts_real_published_bilanzkreis_codes() {
        for code in REAL_BILANZKREISE {
            let bk = BilanzkreisId::new(code).unwrap_or_else(|e| {
                panic!("{code} is a real Bilanzkreis-ID but was rejected: {e}")
            });
            assert_eq!(bk.as_ref(), code);
            assert_eq!(bk.to_eic_code().eic_type(), EicType::Party);
        }
    }

    #[test]
    fn accepts_real_published_bilanzierungsgebiet_codes() {
        for code in REAL_BILANZIERUNGSGEBIETE {
            let bg = BilanzierungsgebietId::new(code).unwrap_or_else(|e| {
                panic!("{code} is a real Bilanzierungsgebiet-EIC but was rejected: {e}")
            });
            assert_eq!(bg.as_ref(), code);
            assert_eq!(bg.to_eic_code().eic_type(), EicType::Area);
        }
    }

    #[test]
    fn bilanzkreis_pins_party_type() {
        assert_eq!(BilanzkreisId::EIC_TYPE, EicType::Party);
        assert_eq!(BilanzkreisId::EIC_TYPE.as_char(), 'X');
    }

    #[test]
    fn bilanzierungsgebiet_pins_area_type() {
        assert_eq!(BilanzierungsgebietId::EIC_TYPE, EicType::Area);
        assert_eq!(BilanzierungsgebietId::EIC_TYPE.as_char(), 'Y');
    }

    /// The two types must not accept each other's codes — that separation is the
    /// entire reason they are distinct Rust types.
    #[test]
    fn the_two_types_reject_each_other() {
        let bk = "11XSUEDWESTSTRO8";
        let bg = "11YN-0000-0001-Q";

        assert!(BilanzkreisId::new(bk).is_ok());
        assert!(BilanzierungsgebietId::new(bk).is_err());

        assert!(BilanzierungsgebietId::new(bg).is_ok());
        assert!(BilanzkreisId::new(bg).is_err());
    }

    /// A control-area EIC (`10Y…`) is an area code, so it is a valid
    /// Bilanzierungsgebiet shape but never a Bilanzkreis.
    #[test]
    fn rejects_wrong_object_type_with_a_useful_message() {
        let err = BilanzkreisId::new("10YDE-EON------1").unwrap_err();
        let msg = err.to_string();
        assert!(
            msg.contains('X') && msg.contains('Y'),
            "error should name both the required and the found type: {msg}"
        );
        assert!(matches!(err, IdentifierError::InvalidFormat { .. }));
    }

    #[test]
    fn rejects_invalid_length() {
        assert!(matches!(
            BilanzkreisId::new("11X-----------").unwrap_err(),
            IdentifierError::InvalidLength { .. }
        ));
        assert!(matches!(
            BilanzkreisId::from_prefix("11X-----------").unwrap_err(),
            IdentifierError::InvalidLength { .. }
        ));
    }

    #[test]
    fn rejects_bad_check_character() {
        // Flip the published check character.
        let mut bad = String::from("11XSUEDWESTSTRO");
        bad.push(if "11XSUEDWESTSTRO8".ends_with('7') {
            '6'
        } else {
            '7'
        });
        assert!(matches!(
            BilanzkreisId::new(&bad).unwrap_err(),
            IdentifierError::InvalidChecksum
        ));
    }

    #[test]
    fn from_prefix_computes_the_check_character() {
        let bk = BilanzkreisId::from_prefix("11XSUEDWESTSTRO").unwrap();
        assert_eq!(bk.as_ref(), "11XSUEDWESTSTRO8");
    }

    #[test]
    fn from_prefix_rejects_wrong_object_type() {
        assert!(matches!(
            BilanzkreisId::from_prefix("11YVEW---------").unwrap_err(),
            IdentifierError::InvalidFormat { .. }
        ));
    }

    // ── Conversions ───────────────────────────────────────────────────────

    #[test]
    fn widening_and_narrowing_round_trip() {
        let bk = BilanzkreisId::new("11XSUEDWESTSTRO8").unwrap();
        let eic: EicCode = bk.clone().into();
        assert_eq!(eic.eic_type(), EicType::Party);
        assert_eq!(BilanzkreisId::try_from(eic).unwrap(), bk);
    }

    #[test]
    fn shares_the_common_identifier_trait_surface() {
        use std::borrow::Borrow;

        let bk = BilanzkreisId::new("11XSUEDWESTSTRO8").unwrap();
        assert!(bk.starts_with("11X")); // Deref<Target = str>
        let borrowed: &str = bk.borrow(); // Borrow<str>
        assert_eq!(borrowed, "11XSUEDWESTSTRO8");
        assert_eq!(String::from(bk.clone()), "11XSUEDWESTSTRO8");
        assert_eq!(bk.to_string(), bk.as_ref());
    }

    #[cfg(feature = "serde")]
    #[test]
    fn roundtrip_serde() {
        for code in REAL_BILANZKREISE {
            let bk = BilanzkreisId::new(code).unwrap();
            let json = serde_json::to_string(&bk).unwrap();
            assert_eq!(json, format!("\"{code}\""));
            assert_eq!(serde_json::from_str::<BilanzkreisId>(&json).unwrap(), bk);
        }
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_rejects_wrong_object_type() {
        // A Y-type area code must not deserialize into a Bilanzkreis-ID.
        assert!(serde_json::from_str::<BilanzkreisId>("\"10YDE-EON------1\"").is_err());
    }
}
