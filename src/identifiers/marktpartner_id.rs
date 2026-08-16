use super::checksum::{compute_numeric_id_from_base, validate_numeric_id};
use crate::error::{IdentifierError, LengthExpectation};

/// Length of a Marktpartner-ID, including the check digit.
const LEN: usize = 13;

fn validate_format(s: &str) -> Result<(), IdentifierError> {
    if s.len() != LEN {
        return Err(IdentifierError::InvalidLength {
            expected: LengthExpectation::Exact(LEN),
            actual: s.len(),
        });
    }
    for (i, c) in s.char_indices() {
        if !c.is_ascii_digit() {
            return Err(IdentifierError::InvalidCharacter {
                position: i,
                character: c,
            });
        }
    }
    Ok(())
}

/// The authority that issued a [`MarktpartnerId`], derived from its first two digits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MpIdAuthority {
    /// Prefix `99` — Energie Codes und Services GmbH (BDEW), electricity.
    Bdew,
    /// Prefix `98` — DVGW Services und Consult GmbH, gas.
    Dvgw,
    /// Any other prefix — treated as a GS1 Global Location Number.
    ///
    /// Note that legacy BDEW-issued codes with other prefixes also land here; the
    /// prefix is a heuristic, not a guarantee. See
    /// [`MarktpartnerId::has_valid_bdew_check_digit`] for a stronger signal.
    Gs1Gln,
}

impl MpIdAuthority {
    /// The EDIFACT **NAD DE3055** agency code for this authority.
    ///
    /// Used in NAD segments, e.g. `NAD+MS+<id>::293`.
    #[must_use]
    pub fn nad_agency_code(self) -> &'static str {
        match self {
            Self::Bdew => "293",
            Self::Dvgw => "332",
            Self::Gs1Gln => "9",
        }
    }

    /// The EDIFACT **UNB DE0007** sender/receiver qualifier for this authority.
    ///
    /// Used in the interchange header, e.g. `UNB+UNOC:3+<id>:500+…`.
    #[must_use]
    pub fn unb_agency_code(self) -> &'static str {
        match self {
            Self::Bdew => "500",
            Self::Dvgw => "502",
            Self::Gs1Gln => "14",
        }
    }
}

impl std::fmt::Display for MpIdAuthority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Bdew => "BDEW",
            Self::Dvgw => "DVGW",
            Self::Gs1Gln => "GS1 GLN",
        })
    }
}

/// Marktpartner-ID (MP-ID) — identifies a market participant in one role and
/// one commodity.
///
/// Defined by BDEW "Identifikatoren in der Marktkommunikation" v1.2
/// (7 February 2025), §2. An MP-ID is one of three things, all 13 digits wide:
/// a BDEW-Codenummer (electricity), a DVGW-Codenummer (gas), or a GS1 Global
/// Location Number.
///
/// ## Format (§2.2)
///
/// | Position | Content |
/// |----------|---------|
/// | 1–2 | Vergabestelle / Sparte — `99` = BDEW/Strom, `98` = DVGW/Gas |
/// | 3 | `0`–`8` for BDEW, `0`–`9` for DVGW |
/// | 4–12 | Assigned by the issuing office |
/// | 13 | Check digit |
///
/// ## Why `new` does not verify the check digit
///
/// §2.3 specifies two *different* check-digit procedures depending on origin:
/// BDEW- and DVGW-Codenummern use the Lok- und Waggon-Kennzeichnungsverfahren
/// (§8.1), while a GS1-issued GLN uses the GS1/EAN-13 procedure. The two disagree,
/// and the first two digits do not reliably tell the cases apart — codes issued
/// before the `98`/`99` convention are still in circulation and validate under
/// §8.1 despite other prefixes.
///
/// Rejecting on a guess would drop valid production identifiers, so [`new`](Self::new)
/// validates only what is unambiguous: 13 decimal digits. Verify the check digit
/// explicitly when you know which family you are dealing with:
///
/// - [`has_valid_bdew_check_digit`](Self::has_valid_bdew_check_digit) — §8.1
/// - [`has_valid_gln_check_digit`](Self::has_valid_gln_check_digit) — GS1/EAN-13
/// - [`new_checked`](Self::new_checked) — accept only if **one of the two** matches
///
/// # Examples
/// ```
/// use rubo4e::identifiers::{MarktpartnerId, MpIdAuthority};
///
/// let bdew = MarktpartnerId::new("9900357000003").unwrap();
/// assert_eq!(bdew.authority(), MpIdAuthority::Bdew);
/// assert_eq!(bdew.authority().nad_agency_code(), "293");
/// assert!(bdew.has_valid_bdew_check_digit());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "validate", derive(garde::Validate))]
#[cfg_attr(feature = "validate", garde(allow_unvalidated))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(
    feature = "schemars",
    schemars(schema_with = "crate::schema_helpers::marktpartner_id_schema")
)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "utoipa", schema(
    value_type = String,
    pattern = r"^[0-9]{13}$",
    example = "9900357000003",
    description = "13-stellige Marktpartner-ID: BDEW-Codenummer Strom (Prefix 99), DVGW-Codenummer Gas (Prefix 98) oder GS1 GLN"
))]
pub struct MarktpartnerId(
    #[cfg_attr(feature = "validate", garde(custom(check_marktpartner_id)))] Box<str>,
);

#[cfg(feature = "validate")]
fn check_marktpartner_id(value: &str, _: &()) -> Result<(), garde::Error> {
    validate_format(value).map_err(garde::Error::from)
}

/// Computes the GS1/EAN-13 check digit for a 12-digit base.
///
/// Weights alternate `1, 3, 1, 3, …` starting at position 1; the check digit is
/// the difference to the next multiple of 10. This is *not* the BDEW §8.1
/// procedure, which weights even positions by 2.
fn gln_check_digit(base: &[u8]) -> u8 {
    let sum: u32 = base
        .iter()
        .enumerate()
        .map(|(i, &b)| u32::from(b - b'0') * if i % 2 == 0 { 1 } else { 3 })
        .sum();
    ((10 - (sum % 10)) % 10) as u8
}

impl MarktpartnerId {
    /// Creates a new `MarktpartnerId`, validating length and character set only.
    ///
    /// See the [type documentation](Self) for why the check digit is not verified
    /// here, and [`new_checked`](Self::new_checked) if you want it enforced.
    ///
    /// # Errors
    /// - [`IdentifierError::InvalidLength`] if `s` is not exactly 13 characters.
    /// - [`IdentifierError::InvalidCharacter`] if any character is not a decimal digit.
    #[must_use = "the validated identifier is returned; ignoring it discards the result"]
    pub fn new(s: &str) -> Result<Self, IdentifierError> {
        validate_format(s)?;
        Ok(Self(Box::from(s)))
    }

    /// Creates a `MarktpartnerId` that must additionally satisfy **either** the
    /// BDEW §8.1 check digit **or** the GS1/EAN-13 check digit.
    ///
    /// Use this at an ingest boundary to reject transposed or mistyped digits.
    /// Because it accepts either procedure it cannot distinguish the two families,
    /// but it still rejects roughly 80 % of random 13-digit strings.
    ///
    /// # Errors
    /// As [`new`](Self::new), plus [`IdentifierError::InvalidChecksum`] when
    /// neither procedure matches.
    ///
    /// # Examples
    /// ```
    /// use rubo4e::identifiers::MarktpartnerId;
    ///
    /// assert!(MarktpartnerId::new_checked("9900357000003").is_ok());  // §8.1
    /// assert!(MarktpartnerId::new_checked("4006381333931").is_ok());  // EAN-13
    /// assert!(MarktpartnerId::new_checked("9900357000000").is_err()); // neither
    /// ```
    pub fn new_checked(s: &str) -> Result<Self, IdentifierError> {
        let id = Self::new(s)?;
        if id.has_valid_bdew_check_digit() || id.has_valid_gln_check_digit() {
            Ok(id)
        } else {
            Err(IdentifierError::InvalidChecksum)
        }
    }

    /// Builds a `MarktpartnerId` from its 12-digit base by computing and appending
    /// the BDEW §8.1 check digit.
    ///
    /// # Errors
    /// - [`IdentifierError::InvalidLength`] if `base` is not exactly 12 characters.
    /// - [`IdentifierError::InvalidCharacter`] if any character is not a decimal digit.
    ///
    /// # Examples
    /// ```
    /// use rubo4e::identifiers::MarktpartnerId;
    ///
    /// let id = MarktpartnerId::from_base("990035700000").unwrap();
    /// assert_eq!(id.as_ref(), "9900357000003");
    /// ```
    pub fn from_base(base: &str) -> Result<Self, IdentifierError> {
        let full = compute_numeric_id_from_base(base, LEN, 0)?;
        Ok(Self(full.into_boxed_str()))
    }

    /// Computes the BDEW §8.1 check digit (`0`–`9`) for a 12-digit base.
    ///
    /// # Errors
    /// Same as [`from_base`](Self::from_base).
    pub fn check_digit(base: &str) -> Result<u8, IdentifierError> {
        let full = compute_numeric_id_from_base(base, LEN, 0)?;
        Ok(full.as_bytes()[LEN - 1] - b'0')
    }

    /// Returns the 12-digit base (everything except the check digit).
    #[must_use]
    pub fn base(&self) -> &str {
        &self.0[..LEN - 1]
    }

    /// Returns `true` if the 13th digit matches the BDEW/DVGW §8.1 check digit.
    ///
    /// This is the procedure that BDEW- and DVGW-Codenummern use.
    #[must_use]
    pub fn has_valid_bdew_check_digit(&self) -> bool {
        validate_numeric_id(&self.0, LEN, 0).is_ok()
    }

    /// Returns `true` if the 13th digit matches the GS1/EAN-13 check digit.
    ///
    /// This is the procedure that GS1-issued Global Location Numbers use.
    #[must_use]
    pub fn has_valid_gln_check_digit(&self) -> bool {
        let bytes = self.0.as_bytes();
        bytes[LEN - 1] - b'0' == gln_check_digit(&bytes[..LEN - 1])
    }

    /// Returns the issuing authority implied by the first two digits.
    #[must_use]
    pub fn authority(&self) -> MpIdAuthority {
        match &self.0[..2] {
            "99" => MpIdAuthority::Bdew,
            "98" => MpIdAuthority::Dvgw,
            _ => MpIdAuthority::Gs1Gln,
        }
    }

    /// Returns `true` if this ID carries the BDEW electricity prefix `99`.
    #[must_use]
    pub fn is_bdew(&self) -> bool {
        self.authority() == MpIdAuthority::Bdew
    }

    /// Returns `true` if this ID carries the DVGW gas prefix `98`.
    #[must_use]
    pub fn is_dvgw(&self) -> bool {
        self.authority() == MpIdAuthority::Dvgw
    }

    /// Returns `true` if this ID has neither the `99` nor the `98` prefix and is
    /// therefore presumed to be a GS1 GLN.
    #[must_use]
    pub fn is_gln(&self) -> bool {
        self.authority() == MpIdAuthority::Gs1Gln
    }

    /// The EDIFACT **NAD DE3055** agency code — shorthand for
    /// `self.authority().nad_agency_code()`.
    #[must_use]
    pub fn nad_agency_code(&self) -> &'static str {
        self.authority().nad_agency_code()
    }

    /// The EDIFACT **UNB DE0007** qualifier — shorthand for
    /// `self.authority().unb_agency_code()`.
    #[must_use]
    pub fn unb_agency_code(&self) -> &'static str {
        self.authority().unb_agency_code()
    }

    /// Converts this ID to its numeric `i64` representation.
    ///
    /// A validated 13-digit string always fits in `i64`
    /// (`9_999_999_999_999 < i64::MAX`), so this is infallible.
    ///
    /// # Examples
    /// ```
    /// use rubo4e::identifiers::MarktpartnerId;
    ///
    /// let id = MarktpartnerId::new("9900357000003").unwrap();
    /// assert_eq!(id.to_i64(), 9_900_357_000_003_i64);
    /// ```
    #[must_use]
    pub fn to_i64(&self) -> i64 {
        self.0
            .parse::<i64>()
            .expect("MarktpartnerId is validated as 13 ASCII digits; parse to i64 cannot fail")
    }
}

impl_identifier_traits!(
    MarktpartnerId,
    "a 13-digit Marktpartner-ID (BDEW-Codenummer, DVGW-Codenummer oder GS1 GLN)"
);

/// Serde adapter that encodes a [`MarktpartnerId`] as a JSON integer (`i64`)
/// instead of a string.
///
/// Some BDEW REST APIs (e.g. API-Webdienste Strom) represent Rollencodenummern as
/// 64-bit integers. Use this adapter on fields that must match such an endpoint:
///
/// ```
/// use rubo4e::identifiers::MarktpartnerId;
///
/// #[derive(serde::Serialize, serde::Deserialize)]
/// struct Request {
///     #[serde(with = "rubo4e::identifiers::marktpartner_id_as_i64")]
///     market_partner_id: MarktpartnerId,
/// }
///
/// // Emitted as a JSON integer, not a string:
/// let req = Request { market_partner_id: MarktpartnerId::new("9903155000006").unwrap() };
/// assert_eq!(serde_json::to_string(&req).unwrap(), r#"{"market_partner_id":9903155000006}"#);
///
/// // Both integer and string inputs decode:
/// let from_int: Request = serde_json::from_str(r#"{"market_partner_id":9903155000006}"#).unwrap();
/// let from_str: Request = serde_json::from_str(r#"{"market_partner_id":"9903155000006"}"#).unwrap();
/// assert_eq!(from_int.market_partner_id, from_str.market_partner_id);
/// ```
///
/// Serialization always emits an integer. Deserialization accepts an integer
/// **or** a string, so payloads from string-typed upstreams still decode. Integers
/// are zero-padded back to 13 digits before validation.
#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
pub mod serde_as_i64 {
    use super::MarktpartnerId;
    use crate::error::IdentifierError;

    /// Serializes a `MarktpartnerId` as a JSON integer.
    pub fn serialize<S: serde::Serializer>(id: &MarktpartnerId, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_i64(id.to_i64())
    }

    /// Deserializes a `MarktpartnerId` from either a JSON integer or a JSON string.
    pub fn deserialize<'de, D: serde::Deserializer<'de>>(d: D) -> Result<MarktpartnerId, D::Error> {
        struct Visitor;
        impl serde::de::Visitor<'_> for Visitor {
            type Value = MarktpartnerId;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a 13-digit Marktpartner-ID as an integer or string")
            }
            fn visit_i64<E: serde::de::Error>(self, v: i64) -> Result<MarktpartnerId, E> {
                let v = u64::try_from(v).map_err(|_| {
                    E::custom(IdentifierError::InvalidFormat {
                        description: "MarktpartnerId cannot be negative".into(),
                    })
                })?;
                self.visit_u64(v)
            }
            fn visit_u64<E: serde::de::Error>(self, v: u64) -> Result<MarktpartnerId, E> {
                MarktpartnerId::new(&format!("{v:013}")).map_err(E::custom)
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<MarktpartnerId, E> {
                MarktpartnerId::new(v).map_err(E::custom)
            }
        }
        d.deserialize_any(Visitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Real Marktpartner-IDs published by DB Energie GmbH for its Bahnstrom grid
    /// roles. They validate under BDEW §8.1 despite not carrying a `98`/`99`
    /// prefix, which is exactly why `new` does not enforce a prefix.
    const REAL_MP_IDS: &[&str] = &[
        "1900100300012",
        "1900100300020",
        "1900100301002",
        "1900100400010",
        "1900100500000",
        "1900100551582",
    ];

    #[test]
    fn real_world_ids_satisfy_bdew_check_digit() {
        for id in REAL_MP_IDS {
            let mp = MarktpartnerId::new(id).unwrap();
            assert!(
                mp.has_valid_bdew_check_digit(),
                "{id} is a published MP-ID and must satisfy BDEW §8.1"
            );
            assert!(MarktpartnerId::new_checked(id).is_ok());
        }
    }

    #[test]
    fn from_base_matches_bdew_check_digit() {
        for id in REAL_MP_IDS {
            let base = &id[..12];
            assert_eq!(MarktpartnerId::from_base(base).unwrap().as_ref(), *id);
            assert_eq!(
                MarktpartnerId::check_digit(base).unwrap(),
                id.as_bytes()[12] - b'0'
            );
        }
    }

    #[test]
    fn gln_check_digit_is_the_ean13_procedure() {
        // Well-known EAN-13 worked example: base 400638133393 → check digit 1.
        assert_eq!(gln_check_digit(b"400638133393"), 1);
        let gln = MarktpartnerId::new("4006381333931").unwrap();
        assert!(gln.has_valid_gln_check_digit());
        assert_eq!(gln.authority(), MpIdAuthority::Gs1Gln);
    }

    #[test]
    fn new_is_permissive_but_new_checked_is_not() {
        // 13 digits with a check digit matching neither procedure.
        let bogus = "9900357000000";
        assert!(MarktpartnerId::new(bogus).is_ok());
        assert!(matches!(
            MarktpartnerId::new_checked(bogus),
            Err(IdentifierError::InvalidChecksum)
        ));
    }

    #[test]
    fn authority_and_edifact_codes() {
        let cases = [
            ("9900357000003", MpIdAuthority::Bdew, "293", "500"),
            ("9812345000004", MpIdAuthority::Dvgw, "332", "502"),
            ("4006381333931", MpIdAuthority::Gs1Gln, "9", "14"),
        ];
        for (raw, authority, nad, unb) in cases {
            let id = MarktpartnerId::new(raw).unwrap();
            assert_eq!(id.authority(), authority);
            assert_eq!(id.nad_agency_code(), nad);
            assert_eq!(id.unb_agency_code(), unb);
            assert_eq!(id.is_bdew(), authority == MpIdAuthority::Bdew);
            assert_eq!(id.is_dvgw(), authority == MpIdAuthority::Dvgw);
            assert_eq!(id.is_gln(), authority == MpIdAuthority::Gs1Gln);
        }
    }

    #[test]
    fn format_errors() {
        assert!(matches!(
            MarktpartnerId::new("123456789012").unwrap_err(),
            IdentifierError::InvalidLength {
                expected: LengthExpectation::Exact(13),
                actual: 12
            }
        ));
        assert!(matches!(
            MarktpartnerId::new("12345678901234").unwrap_err(),
            IdentifierError::InvalidLength { actual: 14, .. }
        ));
        assert!(matches!(
            MarktpartnerId::new("123456789012A").unwrap_err(),
            IdentifierError::InvalidCharacter {
                position: 12,
                character: 'A'
            }
        ));
    }

    #[test]
    fn to_i64_conversion() {
        assert_eq!(
            MarktpartnerId::new("9900357000003").unwrap().to_i64(),
            9_900_357_000_003_i64
        );
        assert_eq!(MarktpartnerId::new("0000000000000").unwrap().to_i64(), 0);
        assert_eq!(
            MarktpartnerId::new("9999999999999").unwrap().to_i64(),
            9_999_999_999_999_i64
        );
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_as_i64_round_trip() {
        #[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
        struct Wrapper {
            #[serde(with = "super::serde_as_i64")]
            id: MarktpartnerId,
        }
        let w = Wrapper {
            id: MarktpartnerId::new("0900357000009").unwrap(),
        };
        let json = serde_json::to_string(&w).unwrap();
        assert_eq!(json, r#"{"id":900357000009}"#);
        // Round-trips back through the zero-padding path.
        assert_eq!(serde_json::from_str::<Wrapper>(&json).unwrap(), w);
        // Strings are accepted too.
        assert_eq!(
            serde_json::from_str::<Wrapper>(r#"{"id":"0900357000009"}"#).unwrap(),
            w
        );
        // Negative integers are rejected rather than wrapping.
        assert!(serde_json::from_str::<Wrapper>(r#"{"id":-1}"#).is_err());
    }
}
