use super::checksum::{compute_ascii_id_from_base, validate_ascii_id};
use crate::error::IdentifierError;
#[cfg(test)]
use crate::error::LengthExpectation;

/// Technische-Ressource-ID (TR-ID): 11-character identifier for a technical resource.
///
/// Defined by BDEW "Identifikatoren in der Marktkommunikation" v1.2
/// (February 2025), §6.2 and §6.6.  Used in Redispatch 2.0 to identify
/// technische Ressourcen (physical generation/consumption units).
///
/// ## Format (§6.6)
///
/// | Position | Content | Character set |
/// |----------|---------|---------------|
/// | 1        | Codetyp — always `'D'` (Technische Ressource) | `{D}` |
/// | 2–10     | Alphanumeric body | `[A-Z0-9]` |
/// | 11       | Check digit (ASCII-Verfahren, §8.2) | `[0-9]` |
///
/// Uses the same ASCII-Verfahren check digit algorithm as [`NeloId`](super::NeloId)
/// and [`SrId`](super::SrId).
///
/// # Examples
/// ```
/// use rubo4e::identifiers::TrId;
///
/// let id = TrId::new("D0000000002").unwrap();
/// assert_eq!(id.to_string(), "D0000000002");
///
/// let id = TrId::from_base("D000000000").unwrap();
/// assert_eq!(id.as_ref(), "D0000000002");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "validate", derive(garde::Validate))]
#[cfg_attr(feature = "validate", garde(allow_unvalidated))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(
    feature = "schemars",
    schemars(schema_with = "crate::schema_helpers::tr_id_schema")
)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "utoipa", schema(value_type = String))]
pub struct TrId(#[cfg_attr(feature = "validate", garde(custom(check_tr_id)))] Box<str>);

#[cfg(feature = "validate")]
fn check_tr_id(value: &str, _: &()) -> Result<(), garde::Error> {
    validate_ascii_id(value, b'D').map_err(garde::Error::from)
}

impl TrId {
    /// Creates a new `TrId` after full validation.
    ///
    /// # Errors
    /// - [`IdentifierError::InvalidLength`] if `s` is not exactly 11 characters.
    /// - [`IdentifierError::InvalidFormat`] if the first character is not `'D'`.
    /// - [`IdentifierError::InvalidCharacter`] if positions 2–10 contain characters
    ///   outside `[A-Z0-9]`, or position 11 is not a decimal digit.
    /// - [`IdentifierError::InvalidChecksum`] if the 11th digit does not match the
    ///   ASCII-Verfahren check digit.
    #[must_use = "the validated identifier is returned; ignoring it discards the result"]
    pub fn new(s: &str) -> Result<Self, IdentifierError> {
        validate_ascii_id(s, b'D')?;
        Ok(Self(Box::from(s)))
    }

    /// Constructs a valid `TrId` from a 10-character base string by computing
    /// and appending the ASCII-Verfahren check digit.
    ///
    /// `base[0]` must be `'D'`; `base[1..=9]` must be `[A-Z0-9]`.
    ///
    /// # Errors
    /// - [`IdentifierError::InvalidLength`] if `base` is not exactly 10 characters.
    /// - [`IdentifierError::InvalidFormat`] if `base[0]` is not `'D'`.
    /// - [`IdentifierError::InvalidCharacter`] if positions 2–10 are not `[A-Z0-9]`.
    ///
    /// # Examples
    /// ```
    /// use rubo4e::identifiers::TrId;
    ///
    /// let id = TrId::from_base("D000000000").unwrap();
    /// assert_eq!(id.as_ref(), "D0000000002");
    /// ```
    pub fn from_base(base: &str) -> Result<Self, IdentifierError> {
        let full = compute_ascii_id_from_base(base, b'D')?;
        Ok(Self(Box::from(full.as_str())))
    }

    /// Computes the ASCII-Verfahren check digit for a 10-character TR-ID base.
    ///
    /// Returns the check digit value (`0`–`9`) without constructing a `TrId`.
    ///
    /// # Errors
    /// - [`IdentifierError::InvalidLength`] if `base` is not exactly 10 characters.
    /// - [`IdentifierError::InvalidFormat`] if `base[0]` is not `'D'`.
    /// - [`IdentifierError::InvalidCharacter`] if positions 2–10 are not `[A-Z0-9]`.
    pub fn check_digit(base: &str) -> Result<u8, IdentifierError> {
        let full = compute_ascii_id_from_base(base, b'D')?;
        Ok(full.as_bytes().last().copied().expect("11 chars") - b'0')
    }
}

impl TryFrom<String> for TrId {
    type Error = IdentifierError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::new(&s)
    }
}

impl TryFrom<&str> for TrId {
    type Error = IdentifierError;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Self::new(s)
    }
}

impl AsRef<str> for TrId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for TrId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::str::FromStr for TrId {
    type Err = IdentifierError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for TrId {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&self.0)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for TrId {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TrId;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(
                    "an 11-character Technische-Ressource-ID \
                     (Codetyp 'D' + 9 alphanumeric + ASCII-Verfahren check digit)",
                )
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<TrId, E> {
                TrId::new(v).map_err(|e| {
                    crate::identifiers::trace_identifier_deser_error("TrId", v, &e);
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
    // with Codetyp 'D' (ASCII 68).

    // ── from_base / check_digit ──────────────────────────────────────────

    #[test]
    fn from_base_all_zeros() {
        // D=68 at idx0; zeros elsewhere.  odd=68, even=0 → total=68 → check=2
        let id = TrId::from_base("D000000000").unwrap();
        assert_eq!(id.as_ref(), "D0000000002");
    }

    #[test]
    fn from_base_trailing_one() {
        // odd=68, even=(0+0+0+0+1)×2=2 → total=70 → check=0
        let id = TrId::from_base("D000000001").unwrap();
        assert_eq!(id.as_ref(), "D0000000010");
    }

    #[test]
    fn from_base_all_ones_body() {
        // D=68, body=1×9.  odd=68+1+1+1+1=72, even=(1+1+1+1+1)×2=10 → 82 → check=8
        let id = TrId::from_base("D111111111").unwrap();
        assert_eq!(id.as_ref(), "D1111111118");
    }

    #[test]
    fn check_digit_method() {
        assert_eq!(TrId::check_digit("D000000000").unwrap(), 2);
        assert_eq!(TrId::check_digit("D000000001").unwrap(), 0);
    }

    // ── make_valid_ascii_id helper ───────────────────────────────────────

    #[test]
    fn valid_ids_from_helper_pass() {
        let bodies: &[[u8; 9]] = &[
            *b"000000000",
            *b"000000001",
            *b"111111111",
            *b"987654321",
        ];
        for body in bodies {
            let s = make_valid_ascii_id(b'D', body);
            TrId::new(&s).unwrap_or_else(|e| panic!("{s} should be valid: {e}"));
        }
    }

    // ── new / round-trip ─────────────────────────────────────────────────

    #[test]
    fn round_trip() {
        let s = "D0000000002";
        assert_eq!(s.parse::<TrId>().unwrap().to_string(), s);
    }

    // ── validation error paths ───────────────────────────────────────────

    #[test]
    fn wrong_length_fails() {
        assert!(matches!(
            TrId::new("D123456789").unwrap_err(),
            IdentifierError::InvalidLength {
                expected: LengthExpectation::Exact(11),
                actual: 10
            }
        ));
        assert!(matches!(
            TrId::new("D12345678901").unwrap_err(),
            IdentifierError::InvalidLength {
                expected: LengthExpectation::Exact(11),
                actual: 12
            }
        ));
        assert!(matches!(
            TrId::new("").unwrap_err(),
            IdentifierError::InvalidLength {
                expected: LengthExpectation::Exact(11),
                actual: 0
            }
        ));
    }

    #[test]
    fn wrong_type_char_fails() {
        // C = SR-ID, E = NeLo-ID — not valid TR-ID prefixes
        for s in ["C0000000003", "E0000000019"] {
            assert!(
                matches!(
                    TrId::new(s).unwrap_err(),
                    IdentifierError::InvalidFormat { .. }
                ),
                "{s} should fail with InvalidFormat"
            );
        }
        // A purely numeric 11-digit string (old format) is also rejected
        assert!(matches!(
            TrId::new("51238696780").unwrap_err(),
            IdentifierError::InvalidFormat { .. }
        ));
    }

    #[test]
    fn lowercase_body_fails() {
        let err = TrId::new("D000b000002").unwrap_err();
        assert!(matches!(
            err,
            IdentifierError::InvalidCharacter {
                position: 4,
                character: 'b'
            }
        ));
    }

    #[test]
    fn space_character_fails() {
        let err = TrId::new("D000 000002").unwrap_err();
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
        // "D0000000002" is valid (check=2); other last digits must fail
        for wrong in ["D0000000000", "D0000000001", "D0000000003", "D0000000009"] {
            assert!(
                matches!(
                    TrId::new(wrong).unwrap_err(),
                    IdentifierError::InvalidChecksum
                ),
                "{wrong} should fail with InvalidChecksum"
            );
        }
    }
}
