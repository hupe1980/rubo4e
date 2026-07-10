use super::checksum::{compute_ascii_id_from_base, validate_ascii_id};
use crate::error::IdentifierError;

/// Netzlokations-ID (NeLo-ID): 11-character identifier for a grid element.
///
/// Defined by **BNetzA-Festlegung BK6-22-128** and **BDEW "Identifikatoren in
/// der Marktkommunikation" v1.2** (February 2025), §4.
///
/// ## Format (§4.2)
///
/// | Position | Content | Character set |
/// |----------|---------|---------------|
/// | 1        | Codetyp — always `'E'` (Netzlokation) | `{E}` |
/// | 2–10     | Alphanumeric body (issued by Energie Codes und Services GmbH) | `[A-Z0-9]` |
/// | 11       | Check digit (ASCII-Verfahren, §8.2) | `[0-9]` |
///
/// ## Check digit — ASCII-Verfahren (§8.2)
///
/// 1. Each character is mapped: digit → numeric value (0–9);
///    uppercase letter → ASCII code (A = 65 … Z = 90).
/// 2. Values at odd positions (1, 3, 5, 7, 9) are summed.
/// 3. Values at even positions (2, 4, 6, 8, 10) are summed and multiplied by 2.
/// 4. check = (10 − ((odd + even × 2) % 10)) % 10.
///
/// ## Common confusion
///
/// `10YDE-EON------1` is an **[`EicCode`](super::EicCode)** for a
/// `Bilanzierungsgebiet` / `Regelzone` (control area) — **not** a NeLo-ID.
/// EIC codes appear on `Marktlokation.marktgebiet`; NeLo-IDs identify the
/// physical grid asset (transformer, line segment) in Redispatch 2.0.
///
/// ## Examples
/// ```
/// use rubo4e::identifiers::NeloId;
///
/// let id = NeloId::new("E0000000019").unwrap();
/// assert_eq!(id.to_string(), "E0000000019");
///
/// let id = NeloId::from_base("E000000001").unwrap();
/// assert_eq!(id.as_ref(), "E0000000019");
/// assert_eq!(NeloId::check_digit("E000000001").unwrap(), 9);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "validate", derive(garde::Validate))]
#[cfg_attr(feature = "validate", garde(allow_unvalidated))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(
    feature = "schemars",
    schemars(schema_with = "crate::schema_helpers::nelo_id_schema")
)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "utoipa", schema(value_type = String))]
pub struct NeloId(#[cfg_attr(feature = "validate", garde(custom(check_nelo_id)))] Box<str>);

#[cfg(feature = "validate")]
fn check_nelo_id(value: &str, _: &()) -> Result<(), garde::Error> {
    validate_ascii_id(value, b'E').map_err(garde::Error::from)
}

impl NeloId {
    /// Creates a new `NeloId` after full validation.
    ///
    /// # Errors
    /// - [`IdentifierError::InvalidLength`] if `s` is not exactly 11 characters.
    /// - [`IdentifierError::InvalidFormat`] if the first character is not `'E'`.
    /// - [`IdentifierError::InvalidCharacter`] if positions 2–10 contain characters
    ///   outside `[A-Z0-9]`, or position 11 is not a decimal digit.
    /// - [`IdentifierError::InvalidChecksum`] if the 11th digit does not match the
    ///   ASCII-Verfahren check digit computed from positions 1–10.
    #[must_use = "the validated identifier is returned; ignoring it discards the result"]
    pub fn new(s: &str) -> Result<Self, IdentifierError> {
        validate_ascii_id(s, b'E')?;
        Ok(Self(Box::from(s)))
    }

    /// Constructs a valid `NeloId` from a 10-character base string by computing
    /// and appending the ASCII-Verfahren check digit.
    ///
    /// `base[0]` must be `'E'`; `base[1..=9]` must be `[A-Z0-9]`.
    ///
    /// # Errors
    /// - [`IdentifierError::InvalidLength`] if `base` is not exactly 10 characters.
    /// - [`IdentifierError::InvalidFormat`] if `base[0]` is not `'E'`.
    /// - [`IdentifierError::InvalidCharacter`] if positions 2–10 are not `[A-Z0-9]`.
    ///
    /// # Examples
    /// ```
    /// use rubo4e::identifiers::NeloId;
    ///
    /// let id = NeloId::from_base("E000000001").unwrap();
    /// assert_eq!(id.as_ref(), "E0000000019");
    /// ```
    pub fn from_base(base: &str) -> Result<Self, IdentifierError> {
        let full = compute_ascii_id_from_base(base, b'E')?;
        Ok(Self(Box::from(full.as_str())))
    }

    /// Computes the ASCII-Verfahren check digit for a 10-character NeLo-ID base.
    ///
    /// Returns the check digit value (`0`–`9`) without constructing a `NeloId`.
    ///
    /// # Errors
    /// - [`IdentifierError::InvalidLength`] if `base` is not exactly 10 characters.
    /// - [`IdentifierError::InvalidFormat`] if `base[0]` is not `'E'`.
    /// - [`IdentifierError::InvalidCharacter`] if positions 2–10 are not `[A-Z0-9]`.
    ///
    /// # Examples
    /// ```
    /// use rubo4e::identifiers::NeloId;
    ///
    /// assert_eq!(NeloId::check_digit("E000000001").unwrap(), 9);
    /// assert_eq!(NeloId::check_digit("E000000000").unwrap(), 1);
    /// ```
    pub fn check_digit(base: &str) -> Result<u8, IdentifierError> {
        let full = compute_ascii_id_from_base(base, b'E')?;
        Ok(full.as_bytes().last().copied().expect("11 chars") - b'0')
    }
}

impl TryFrom<String> for NeloId {
    type Error = IdentifierError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::new(&s)
    }
}

impl TryFrom<&str> for NeloId {
    type Error = IdentifierError;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Self::new(s)
    }
}

impl AsRef<str> for NeloId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for NeloId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::str::FromStr for NeloId {
    type Err = IdentifierError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for NeloId {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&self.0)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for NeloId {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = NeloId;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(
                    "an 11-character Netzlokations-ID \
                     (Codetyp 'E' + 9 alphanumeric + ASCII-Verfahren check digit)",
                )
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<NeloId, E> {
                NeloId::new(v).map_err(|e| {
                    crate::identifiers::trace_identifier_deser_error("NeloId", v, &e);
                    serde::de::Error::custom(e)
                })
            }
        }
        d.deserialize_str(Visitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::LengthExpectation;

    // All expected values verified against BDEW ASCII-Verfahren §8.2:
    // odd(0,2,4,6,8) = Σ ascii_val; even(1,3,5,7,9)×2 = Σ ascii_val × 2;
    // check = (10 − ((odd + even) % 10)) % 10.

    // ── from_base / check_digit ──────────────────────────────────────────

    #[test]
    fn from_base_e000000001() {
        // E=69 at idx0; 1=1 at idx9.  odd=69, even=(0×4+1)×2=2 → total=71 → check=9
        let id = NeloId::from_base("E000000001").unwrap();
        assert_eq!(id.as_ref(), "E0000000019");
    }

    #[test]
    fn from_base_e000000000() {
        // odd=69, even=0 → total=69 → check=1
        let id = NeloId::from_base("E000000000").unwrap();
        assert_eq!(id.as_ref(), "E0000000001");
    }

    #[test]
    fn from_base_all_ones_body() {
        // E=69, body=1×9.  odd=69+1+1+1+1=73, even=(1+1+1+1+1)×2=10 → 83 → check=7
        let id = NeloId::from_base("E111111111").unwrap();
        assert_eq!(id.as_ref(), "E1111111117");
    }

    #[test]
    fn from_base_sequential_body() {
        // E=69, body=123456789.  odd=69+2+4+6+8=89, even=(1+3+5+7+9)×2=50 → 139 → check=1
        let id = NeloId::from_base("E123456789").unwrap();
        assert_eq!(id.as_ref(), "E1234567891");
    }

    #[test]
    fn check_digit_method() {
        assert_eq!(NeloId::check_digit("E000000001").unwrap(), 9);
        assert_eq!(NeloId::check_digit("E000000000").unwrap(), 1);
        assert_eq!(NeloId::check_digit("E111111111").unwrap(), 7);
    }

    // ── new / round-trip ─────────────────────────────────────────────────

    #[test]
    fn valid_ids_pass() {
        for s in ["E0000000019", "E0000000001", "E1111111117", "E1234567891"] {
            NeloId::new(s).unwrap_or_else(|e| panic!("{s} should be valid: {e}"));
        }
    }

    #[test]
    fn round_trip() {
        let s = "E0000000019";
        assert_eq!(s.parse::<NeloId>().unwrap().to_string(), s);
    }

    // ── validation error paths ───────────────────────────────────────────

    #[test]
    fn wrong_length_fails() {
        assert!(matches!(
            NeloId::new("E123456789").unwrap_err(), // 10 chars
            IdentifierError::InvalidLength {
                expected: LengthExpectation::Exact(11),
                actual: 10
            }
        ));
        assert!(matches!(
            NeloId::new("E12345678901").unwrap_err(), // 12 chars
            IdentifierError::InvalidLength {
                expected: LengthExpectation::Exact(11),
                actual: 12
            }
        ));
        assert!(matches!(
            NeloId::new("").unwrap_err(),
            IdentifierError::InvalidLength {
                expected: LengthExpectation::Exact(11),
                actual: 0
            }
        ));
    }

    #[test]
    fn wrong_type_char_fails() {
        // F = NeBe-ID, C = SR-ID, D = TR-ID — none are valid NeLo-ID prefixes
        for s in ["F0000000003", "C0000000011", "D0000000002"] {
            assert!(
                matches!(
                    NeloId::new(s).unwrap_err(),
                    IdentifierError::InvalidFormat { .. }
                ),
                "{s} should fail with InvalidFormat"
            );
        }
    }

    #[test]
    fn lowercase_body_fails() {
        let err = NeloId::new("E000a000019").unwrap_err();
        assert!(matches!(
            err,
            IdentifierError::InvalidCharacter {
                position: 4,
                character: 'a'
            }
        ));
    }

    #[test]
    fn hyphen_in_body_fails() {
        let err = NeloId::new("E000-000019").unwrap_err();
        assert!(matches!(
            err,
            IdentifierError::InvalidCharacter {
                position: 4,
                character: '-'
            }
        ));
    }

    #[test]
    fn wrong_check_digit_fails() {
        // "E0000000019" is valid; "E0000000010" has wrong check digit
        assert!(matches!(
            NeloId::new("E0000000010").unwrap_err(),
            IdentifierError::InvalidChecksum
        ));
    }

    #[test]
    fn eic_code_rejected() {
        // EIC code (16 chars) is not a NeLo-ID
        assert!(NeloId::new("10YDE-EON------1").is_err());
    }

    #[test]
    fn mp_id_codenummer_rejected() {
        // 13-digit BDEW Marktpartner-Codenummer is not a NeLo-ID
        assert!(NeloId::new("9900000000001").is_err());
    }

    // ── from_base error paths ────────────────────────────────────────────

    #[test]
    fn from_base_wrong_length_fails() {
        assert!(matches!(
            NeloId::from_base("E00000001").unwrap_err(),
            IdentifierError::InvalidLength { .. }
        ));
    }

    #[test]
    fn from_base_wrong_type_char_fails() {
        assert!(matches!(
            NeloId::from_base("F000000001").unwrap_err(),
            IdentifierError::InvalidFormat { .. }
        ));
    }

    #[test]
    fn from_base_lowercase_body_fails() {
        assert!(matches!(
            NeloId::from_base("E000a00001").unwrap_err(),
            IdentifierError::InvalidCharacter { .. }
        ));
    }

    // ── ASCII-Verfahren reference check ──────────────────────────────────

    #[test]
    fn bdew_reference_example_ascii_verfahren() {
        // BDEW doc §8.2 reference: base "A113735592" → check digit 5.
        // NeLo-IDs use the same algorithm with 'E' prefix; verify the
        // raw function produces the correct result on the doc example.
        use super::super::checksum::ascii_check_digit;
        let base = b"A113735592";
        assert_eq!(ascii_check_digit(base), 5);
    }
}
