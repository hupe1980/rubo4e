use super::checksum::validate_11digit_bdew;
use crate::error::IdentifierError;
#[cfg(test)]
use crate::error::LengthExpectation;

/// Steuerbare-Ressource-ID (SR-ID): 11-digit string validated by BDEW checksum.
///
/// Uses the same alternating-weight checksum algorithm as [`MaloId`](super::MaloId).
///
/// # Examples
/// ```
/// use rubo4e::identifiers::SrId;
///
/// let id_str = "51238696780"; // valid check digit = 0
/// // SrId::new(id_str) succeeds for any correctly-checksummed 11-digit string.
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "validate", derive(garde::Validate))]
#[cfg_attr(feature = "validate", garde(allow_unvalidated))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schemars", schemars(with = "String"))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "utoipa", schema(value_type = String))]
pub struct SrId(#[cfg_attr(feature = "validate", garde(custom(check_sr_id)))] Box<str>);

#[cfg(feature = "validate")]
fn check_sr_id(value: &str, _: &()) -> Result<(), garde::Error> {
    validate_11digit_bdew(value).map_err(garde::Error::from)
}

impl SrId {
    /// Creates a new `SrId` after validating the BDEW checksum.
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
                f.write_str("an 11-digit Steuerbare-Ressource-ID (BDEW checksum)")
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
    use super::super::checksum::make_valid_11digit;
    use super::*;

    #[test]
    fn valid_ids_pass() {
        let prefixes: &[[u8; 10]] = &[
            [5, 1, 2, 3, 8, 6, 9, 6, 7, 8],
            [1, 2, 3, 4, 5, 6, 7, 8, 9, 0],
            [9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        ];
        for prefix in prefixes {
            let s = make_valid_11digit(prefix);
            assert!(SrId::new(&s).is_ok(), "{s} should be valid");
        }
    }

    #[test]
    fn wrong_length_fails() {
        assert!(matches!(
            SrId::new("1234567890").unwrap_err(),
            IdentifierError::InvalidLength {
                expected: LengthExpectation::Exact(11),
                actual: 10
            }
        ));
    }

    #[test]
    fn too_long_fails() {
        assert!(matches!(
            SrId::new("123456789012").unwrap_err(),
            IdentifierError::InvalidLength {
                expected: LengthExpectation::Exact(11),
                actual: 12
            }
        ));
    }

    #[test]
    fn empty_string_fails() {
        assert!(matches!(
            SrId::new("").unwrap_err(),
            IdentifierError::InvalidLength {
                expected: LengthExpectation::Exact(11),
                actual: 0
            }
        ));
    }

    #[test]
    fn non_digit_fails() {
        let err = SrId::new("1234A678901").unwrap_err();
        assert!(matches!(
            err,
            IdentifierError::InvalidCharacter {
                position: 4,
                character: 'A'
            }
        ));
    }

    #[test]
    fn space_character_fails() {
        let err = SrId::new("1234 678901").unwrap_err();
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
        let valid = make_valid_11digit(&[5, 1, 2, 3, 8, 6, 9, 6, 7, 8]);
        // Flip the last digit
        let last = valid.chars().last().unwrap();
        let wrong_check = char::from_digit((last.to_digit(10).unwrap() + 1) % 10, 10).unwrap();
        let invalid: String = valid[..10].to_string() + &wrong_check.to_string();
        assert!(matches!(
            SrId::new(&invalid).unwrap_err(),
            IdentifierError::InvalidChecksum
        ));
    }

    #[test]
    fn round_trip() {
        let s = make_valid_11digit(&[2, 4, 6, 8, 0, 1, 3, 5, 7, 9]);
        let id = SrId::new(&s).unwrap();
        assert_eq!(id.to_string().parse::<SrId>().unwrap(), id);
    }
}
