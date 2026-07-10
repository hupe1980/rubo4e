use super::checksum::{compute_ascii_id_from_base, validate_ascii_id};
use crate::error::IdentifierError;
#[cfg(test)]
use crate::error::LengthExpectation;

/// Steuerbare-Ressource-ID (SR-ID): 11-character identifier for a controllable resource.
///
/// Defined by BDEW "Identifikatoren in der Marktkommunikation" v1.2
/// (February 2025), §6.3 and §6.6.  Used in Redispatch 2.0 to identify
/// steuerbare Ressourcen (controllable generation/load assets).
///
/// ## Format (§6.6)
///
/// | Position | Content | Character set |
/// |----------|---------|---------------|
/// | 1        | Codetyp — always `'C'` (Steuerbare Ressource) | `{C}` |
/// | 2–10     | Alphanumeric body | `[A-Z0-9]` |
/// | 11       | Check digit (ASCII-Verfahren, §8.2) | `[0-9]` |
///
/// Uses the same ASCII-Verfahren check digit algorithm as [`NeloId`](super::NeloId)
/// and [`TrId`](super::TrId).
///
/// # Examples
/// ```
/// use rubo4e::identifiers::SrId;
///
/// let id = SrId::new("C0000000003").unwrap();
/// assert_eq!(id.to_string(), "C0000000003");
///
/// let id = SrId::from_base("C000000000").unwrap();
/// assert_eq!(id.as_ref(), "C0000000003");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "validate", derive(garde::Validate))]
#[cfg_attr(feature = "validate", garde(allow_unvalidated))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(
    feature = "schemars",
    schemars(schema_with = "crate::schema_helpers::sr_id_schema")
)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "utoipa", schema(value_type = String))]
pub struct SrId(#[cfg_attr(feature = "validate", garde(custom(check_sr_id)))] Box<str>);

#[cfg(feature = "validate")]
fn check_sr_id(value: &str, _: &()) -> Result<(), garde::Error> {
    validate_ascii_id(value, b'C').map_err(garde::Error::from)
}

impl SrId {
    /// Creates a new `SrId` after full validation.
    ///
    /// # Errors
    /// - [`IdentifierError::InvalidLength`] if `s` is not exactly 11 characters.
    /// - [`IdentifierError::InvalidFormat`] if the first character is not `'C'`.
    /// - [`IdentifierError::InvalidCharacter`] if positions 2–10 contain characters
    ///   outside `[A-Z0-9]`, or position 11 is not a decimal digit.
    /// - [`IdentifierError::InvalidChecksum`] if the 11th digit does not match the
    ///   ASCII-Verfahren check digit.
    #[must_use = "the validated identifier is returned; ignoring it discards the result"]
    pub fn new(s: &str) -> Result<Self, IdentifierError> {
        validate_ascii_id(s, b'C')?;
        Ok(Self(Box::from(s)))
    }

    /// Constructs a valid `SrId` from a 10-character base string by computing
    /// and appending the ASCII-Verfahren check digit.
    ///
    /// `base[0]` must be `'C'`; `base[1..=9]` must be `[A-Z0-9]`.
    ///
    /// # Errors
    /// - [`IdentifierError::InvalidLength`] if `base` is not exactly 10 characters.
    /// - [`IdentifierError::InvalidFormat`] if `base[0]` is not `'C'`.
    /// - [`IdentifierError::InvalidCharacter`] if positions 2–10 are not `[A-Z0-9]`.
    ///
    /// # Examples
    /// ```
    /// use rubo4e::identifiers::SrId;
    ///
    /// let id = SrId::from_base("C000000000").unwrap();
    /// assert_eq!(id.as_ref(), "C0000000003");
    /// ```
    pub fn from_base(base: &str) -> Result<Self, IdentifierError> {
        let full = compute_ascii_id_from_base(base, b'C')?;
        Ok(Self(Box::from(full.as_str())))
    }

    /// Computes the ASCII-Verfahren check digit for a 10-character SR-ID base.
    ///
    /// Returns the check digit value (`0`–`9`) without constructing an `SrId`.
    ///
    /// # Errors
    /// - [`IdentifierError::InvalidLength`] if `base` is not exactly 10 characters.
    /// - [`IdentifierError::InvalidFormat`] if `base[0]` is not `'C'`.
    /// - [`IdentifierError::InvalidCharacter`] if positions 2–10 are not `[A-Z0-9]`.
    pub fn check_digit(base: &str) -> Result<u8, IdentifierError> {
        let full = compute_ascii_id_from_base(base, b'C')?;
        Ok(full.as_bytes().last().copied().expect("11 chars") - b'0')
    }
}

impl TryFrom<String> for SrId {
    type Error = IdentifierError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::new(&s)
    }
}

impl TryFrom<&str> for SrId {
    type Error = IdentifierError;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Self::new(s)
    }
}

impl AsRef<str> for SrId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for SrId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::str::FromStr for SrId {
    type Err = IdentifierError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for SrId {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&self.0)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for SrId {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SrId;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(
                    "an 11-character Steuerbare-Ressource-ID \
                     (Codetyp 'C' + 9 alphanumeric + ASCII-Verfahren check digit)",
                )
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<SrId, E> {
                SrId::new(v).map_err(|e| {
                    crate::identifiers::trace_identifier_deser_error("SrId", v, &e);
                    serde::de::Error::custom(e)
                })
            }
        }
        d.deserialize_str(Visitor)
    }
}

#[cfg(test)]
mod tests {
    use super::super::checksum::make_valid_ascii_id;
    use super::*;

    // All expected values verified against BDEW ASCII-Verfahren §8.2
    // with Codetyp 'C' (ASCII 67).

    // ── from_base / check_digit ──────────────────────────────────────────

    #[test]
    fn from_base_all_zeros() {
        // C=67 at idx0; zeros elsewhere.  odd=67, even=0 → total=67 → check=3
        let id = SrId::from_base("C000000000").unwrap();
        assert_eq!(id.as_ref(), "C0000000003");
    }

    #[test]
    fn from_base_trailing_one() {
        // odd=67, even=(0+0+0+0+1)×2=2 → total=69 → check=1
        let id = SrId::from_base("C000000001").unwrap();
        assert_eq!(id.as_ref(), "C0000000011");
    }

    #[test]
    fn from_base_all_ones_body() {
        // C=67, body=1×9.  odd=67+1+1+1+1=71, even=(1+1+1+1+1)×2=10 → 81 → check=9
        let id = SrId::from_base("C111111111").unwrap();
        assert_eq!(id.as_ref(), "C1111111119");
    }

    #[test]
    fn check_digit_method() {
        assert_eq!(SrId::check_digit("C000000000").unwrap(), 3);
        assert_eq!(SrId::check_digit("C000000001").unwrap(), 1);
    }

    // ── make_valid_ascii_id helper ───────────────────────────────────────

    #[test]
    fn valid_ids_from_helper_pass() {
        let bodies: &[[u8; 9]] = &[
            *b"000000000",
            *b"000000001",
            *b"111111111",
            *b"987654321",
            *b"ABCDEF012",
        ];
        for body in bodies {
            let s = make_valid_ascii_id(b'C', body);
            SrId::new(&s).unwrap_or_else(|e| panic!("{s} should be valid: {e}"));
        }
    }

    // ── new / round-trip ─────────────────────────────────────────────────

    #[test]
    fn round_trip() {
        let s = "C0000000003";
        assert_eq!(s.parse::<SrId>().unwrap().to_string(), s);
    }

    // ── validation error paths ───────────────────────────────────────────

    #[test]
    fn wrong_length_fails() {
        assert!(matches!(
            SrId::new("C123456789").unwrap_err(),
            IdentifierError::InvalidLength {
                expected: LengthExpectation::Exact(11),
                actual: 10
            }
        ));
        assert!(matches!(
            SrId::new("C12345678901").unwrap_err(),
            IdentifierError::InvalidLength {
                expected: LengthExpectation::Exact(11),
                actual: 12
            }
        ));
        assert!(matches!(
            SrId::new("").unwrap_err(),
            IdentifierError::InvalidLength {
                expected: LengthExpectation::Exact(11),
                actual: 0
            }
        ));
    }

    #[test]
    fn wrong_type_char_fails() {
        // E = NeLo-ID, D = TR-ID — not valid SR-ID prefixes
        for s in ["E0000000019", "D0000000002"] {
            assert!(
                matches!(
                    SrId::new(s).unwrap_err(),
                    IdentifierError::InvalidFormat { .. }
                ),
                "{s} should fail with InvalidFormat"
            );
        }
        // A purely numeric 11-digit string (old format) is also rejected
        assert!(matches!(
            SrId::new("51238696780").unwrap_err(),
            IdentifierError::InvalidFormat { .. }
        ));
    }

    #[test]
    fn lowercase_body_fails() {
        let err = SrId::new("C000a000003").unwrap_err();
        assert!(matches!(
            err,
            IdentifierError::InvalidCharacter {
                position: 4,
                character: 'a'
            }
        ));
    }

    #[test]
    fn space_character_fails() {
        let err = SrId::new("C000 000003").unwrap_err();
        assert!(matches!(
            err,
            IdentifierError::InvalidCharacter {
                position: 4,
                character: ' '
            }
        ));
    }

    #[test]
    fn wrong_check_digit_fails() {
        // "C0000000003" is valid (check=3); other last digits must fail
        for wrong in ["C0000000000", "C0000000001", "C0000000002", "C0000000009"] {
            assert!(
                matches!(
                    SrId::new(wrong).unwrap_err(),
                    IdentifierError::InvalidChecksum
                ),
                "{wrong} should fail with InvalidChecksum"
            );
        }
    }
}
