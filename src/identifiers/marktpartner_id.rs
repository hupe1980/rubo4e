use crate::error::{IdentifierError, LengthExpectation};

const LEN: usize = 13;

fn validate(s: &str) -> Result<(), IdentifierError> {
    if s.len() != LEN {
        return Err(IdentifierError::InvalidLength {
            expected: LengthExpectation::Exact(LEN),
            actual: s.len(),
        });
    }
    for (i, c) in s.chars().enumerate() {
        if !c.is_ascii_digit() {
            return Err(IdentifierError::InvalidCharacter {
                position: i,
                character: c,
            });
        }
    }
    Ok(())
}

/// Marktpartner-ID (Rollencodenummer): 13-digit numeric string.
///
/// No checksum is applied; all 13 characters must be decimal digits `[0-9]`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "validate", derive(garde::Validate))]
#[cfg_attr(feature = "validate", garde(allow_unvalidated))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schemars", schemars(with = "String"))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "utoipa", schema(value_type = String))]
pub struct MarktpartnerId(
    #[cfg_attr(feature = "validate", garde(custom(check_marktpartner_id)))] Box<str>,
);

#[cfg(feature = "validate")]
fn check_marktpartner_id(value: &str, _: &()) -> Result<(), garde::Error> {
    validate(value).map_err(garde::Error::from)
}

impl MarktpartnerId {
    /// Creates a new `MarktpartnerId` after validating length and character set.
    ///
    /// # Errors
    /// - [`IdentifierError::InvalidLength`] if `s` is not exactly 13 characters.
    /// - [`IdentifierError::InvalidCharacter`] if any character is not a decimal digit.
    #[must_use = "the validated identifier is returned; ignoring it discards the result"]
    pub fn new(s: &str) -> Result<Self, IdentifierError> {
        validate(s)?;
        Ok(Self(Box::from(s)))
    }
}

impl TryFrom<String> for MarktpartnerId {
    type Error = IdentifierError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::new(&s)
    }
}

impl TryFrom<&str> for MarktpartnerId {
    type Error = IdentifierError;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Self::new(s)
    }
}

impl AsRef<str> for MarktpartnerId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for MarktpartnerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::str::FromStr for MarktpartnerId {
    type Err = IdentifierError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for MarktpartnerId {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&self.0)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MarktpartnerId {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MarktpartnerId;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a 13-digit Marktpartner-ID (ILN / GLN)")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<MarktpartnerId, E> {
                MarktpartnerId::new(v).map_err(|e| {
                    crate::identifiers::trace_identifier_deser_error("MarktpartnerId", v, &e);
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

    #[test]
    fn valid_13_digits_passes() {
        assert!(MarktpartnerId::new("1234567890123").is_ok());
        assert!(MarktpartnerId::new("0000000000000").is_ok());
        assert!(MarktpartnerId::new("9999999999999").is_ok());
    }

    #[test]
    fn too_short_fails() {
        assert!(matches!(
            MarktpartnerId::new("123456789012").unwrap_err(),
            IdentifierError::InvalidLength {
                expected: LengthExpectation::Exact(13),
                actual: 12
            }
        ));
    }

    #[test]
    fn too_long_fails() {
        assert!(matches!(
            MarktpartnerId::new("12345678901234").unwrap_err(),
            IdentifierError::InvalidLength {
                expected: LengthExpectation::Exact(13),
                actual: 14
            }
        ));
    }

    #[test]
    fn letter_fails() {
        let err = MarktpartnerId::new("123456789012A").unwrap_err();
        assert!(matches!(
            err,
            IdentifierError::InvalidCharacter {
                position: 12,
                character: 'A'
            }
        ));
    }

    #[test]
    fn hyphen_fails() {
        let err = MarktpartnerId::new("1234567890-23").unwrap_err();
        assert!(matches!(
            err,
            IdentifierError::InvalidCharacter {
                position: 10,
                character: '-'
            }
        ));
    }

    #[test]
    fn round_trip() {
        let id = MarktpartnerId::new("1234567890123").unwrap();
        assert_eq!(id.to_string().parse::<MarktpartnerId>().unwrap(), id);
    }
}
