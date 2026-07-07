use super::checksum::{compute_11digit_from_base, validate_11digit_bdew};
use crate::error::IdentifierError;
#[cfg(test)]
use crate::error::LengthExpectation;

/// Technische-Ressource-ID (TR-ID): 11-digit string validated by BDEW checksum.
///
/// Uses the same alternating-weight checksum algorithm as [`MaloId`](super::MaloId).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "validate", derive(garde::Validate))]
#[cfg_attr(feature = "validate", garde(allow_unvalidated))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schemars", schemars(with = "String"))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "utoipa", schema(value_type = String))]
pub struct TrId(#[cfg_attr(feature = "validate", garde(custom(check_tr_id)))] Box<str>);

#[cfg(feature = "validate")]
fn check_tr_id(value: &str, _: &()) -> Result<(), garde::Error> {
    validate_11digit_bdew(value).map_err(garde::Error::from)
}

impl TrId {
    /// Creates a new `TrId` after validating the BDEW checksum.
    ///
    /// # Errors
    /// - [`IdentifierError::InvalidLength`] if `s` is not exactly 11 characters.
    /// - [`IdentifierError::InvalidCharacter`] if any character is not a decimal digit.
    /// - [`IdentifierError::InvalidChecksum`] if the 11th digit does not match.
    #[must_use = "the validated identifier is returned; ignoring it discards the result"]
    pub fn new(s: &str) -> Result<Self, IdentifierError> {
        validate_11digit_bdew(s)?;
        Ok(Self(Box::from(s)))
    }

    /// Constructs a valid `TrId` from a 10-digit numeric base by computing the
    /// BDEW alternating-weight check digit and appending it.
    ///
    /// # Errors
    /// - [`IdentifierError::InvalidLength`] if `base` is not exactly 10 characters.
    /// - [`IdentifierError::InvalidCharacter`] if any character is not a decimal digit.
    pub fn from_base(base: &str) -> Result<Self, IdentifierError> {
        let full = compute_11digit_from_base(base)?;
        Ok(Self(Box::from(full.as_str())))
    }

    /// Computes the BDEW check digit for a 10-digit numeric base string.
    ///
    /// Returns the single check digit (`0..=9`) on success.
    ///
    /// # Errors
    /// - [`IdentifierError::InvalidLength`] if `base` is not exactly 10 characters.
    /// - [`IdentifierError::InvalidCharacter`] if any character is not a decimal digit.
    pub fn check_digit(base: &str) -> Result<u8, IdentifierError> {
        let full = compute_11digit_from_base(base)?;
        let check = full.as_bytes().last().copied().expect("11 chars") - b'0';
        Ok(check)
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
                f.write_str("an 11-digit Technische-Ressource-ID (BDEW checksum)")
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
    use super::super::checksum::make_valid_11digit;
    use super::*;

    #[test]
    fn valid_ids_pass() {
        let prefixes: &[[u8; 10]] = &[
            [5, 1, 2, 3, 8, 6, 9, 6, 7, 8],
            [1, 2, 3, 4, 5, 6, 7, 8, 9, 0],
            [9, 9, 9, 9, 9, 9, 9, 9, 9, 9],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        ];
        for prefix in prefixes {
            let s = make_valid_11digit(prefix);
            assert!(TrId::new(&s).is_ok(), "{s} should be valid");
        }
    }

    #[test]
    fn wrong_length_fails() {
        assert!(matches!(
            TrId::new("12345").unwrap_err(),
            IdentifierError::InvalidLength {
                expected: LengthExpectation::Exact(11),
                actual: 5
            }
        ));
    }

    #[test]
    fn too_long_fails() {
        assert!(matches!(
            TrId::new("123456789012").unwrap_err(),
            IdentifierError::InvalidLength {
                expected: LengthExpectation::Exact(11),
                actual: 12
            }
        ));
    }

    #[test]
    fn empty_string_fails() {
        assert!(matches!(
            TrId::new("").unwrap_err(),
            IdentifierError::InvalidLength {
                expected: LengthExpectation::Exact(11),
                actual: 0
            }
        ));
    }

    #[test]
    fn non_digit_fails() {
        let err = TrId::new("1234B678901").unwrap_err();
        assert!(matches!(
            err,
            IdentifierError::InvalidCharacter {
                position: 4,
                character: 'B'
            }
        ));
    }

    #[test]
    fn space_character_fails() {
        let err = TrId::new("1234 678901").unwrap_err();
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
        let valid = make_valid_11digit(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 0]);
        let correct_check = valid.chars().last().unwrap().to_digit(10).unwrap();
        let wrong_check = char::from_digit((correct_check + 1) % 10, 10).unwrap();
        let invalid: String = valid[..10].to_string() + &wrong_check.to_string();
        assert!(matches!(
            TrId::new(&invalid).unwrap_err(),
            IdentifierError::InvalidChecksum
        ));
    }

    #[test]
    fn round_trip() {
        let s = make_valid_11digit(&[3, 1, 4, 1, 5, 9, 2, 6, 5, 3]);
        let id = TrId::new(&s).unwrap();
        assert_eq!(id.to_string().parse::<TrId>().unwrap(), id);
    }
}
