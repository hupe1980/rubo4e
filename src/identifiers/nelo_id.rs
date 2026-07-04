use crate::error::{IdentifierError, LengthExpectation};

const LEN: usize = 11;

fn validate(s: &str) -> Result<(), IdentifierError> {
    if s.len() != LEN {
        return Err(IdentifierError::InvalidLength {
            expected: LengthExpectation::Exact(LEN),
            actual: s.len(),
        });
    }
    for (i, c) in s.chars().enumerate() {
        if !c.is_ascii_alphanumeric() {
            return Err(IdentifierError::InvalidCharacter {
                position: i,
                character: c,
            });
        }
    }
    Ok(())
}

/// Netzlokations-ID (NeLo-ID): 11-character alphanumeric string.
///
/// Characters must be in `[A-Za-z0-9]`. No checksum is applied.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "validate", derive(garde::Validate))]
#[cfg_attr(feature = "validate", garde(allow_unvalidated))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schemars", schemars(with = "String"))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "utoipa", schema(value_type = String))]
pub struct NeloId(#[cfg_attr(feature = "validate", garde(custom(check_nelo_id)))] Box<str>);

#[cfg(feature = "validate")]
fn check_nelo_id(value: &str, _: &()) -> Result<(), garde::Error> {
    validate(value).map_err(garde::Error::from)
}

impl NeloId {
    /// Creates a new `NeloId` after validating length and character set.
    ///
    /// # Errors
    /// - [`IdentifierError::InvalidLength`] if `s` is not exactly 11 characters.
    /// - [`IdentifierError::InvalidCharacter`] if any character is not ASCII alphanumeric.
    #[must_use = "the validated identifier is returned; ignoring it discards the result"]
    pub fn new(s: &str) -> Result<Self, IdentifierError> {
        validate(s)?;
        Ok(Self(Box::from(s)))
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
                f.write_str("an 11-character Netzlokations-ID")
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

    #[test]
    fn valid_alphanumeric_11_passes() {
        assert!(NeloId::new("A1B2C3D4E5F").is_ok());
        assert!(NeloId::new("12345678901").is_ok());
        assert!(NeloId::new("AAAAAAAAAAA").is_ok());
        assert!(NeloId::new("aaaaaaaaaaa").is_ok());
    }

    #[test]
    fn too_short_fails() {
        assert!(matches!(
            NeloId::new("A1B2C3D4E5").unwrap_err(),
            IdentifierError::InvalidLength {
                expected: LengthExpectation::Exact(11),
                actual: 10
            }
        ));
    }

    #[test]
    fn too_long_fails() {
        assert!(matches!(
            NeloId::new("A1B2C3D4E5F6").unwrap_err(),
            IdentifierError::InvalidLength {
                expected: LengthExpectation::Exact(11),
                actual: 12
            }
        ));
    }

    #[test]
    fn hyphen_fails() {
        let err = NeloId::new("A1B2C3D4E-F").unwrap_err();
        assert!(matches!(
            err,
            IdentifierError::InvalidCharacter {
                position: 9,
                character: '-'
            }
        ));
    }

    #[test]
    fn round_trip() {
        let id = NeloId::new("A1B2C3D4E5F").unwrap();
        assert_eq!(id.to_string().parse::<NeloId>().unwrap(), id);
    }
}
