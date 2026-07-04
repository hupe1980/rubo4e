use crate::error::{IdentifierError, LengthExpectation};

const LEN: usize = 33;

fn validate(s: &str) -> Result<(), IdentifierError> {
    if s.len() != LEN {
        return Err(IdentifierError::InvalidLength {
            expected: LengthExpectation::Exact(LEN),
            actual: s.len(),
        });
    }
    // Positions 1–2: ISO 3166-1 alpha-2 country code — must be uppercase ASCII letters.
    // Real-world examples: "DE", "AT", "CH", "LU", "CZ".
    for c in s.chars().take(2) {
        if !c.is_ascii_uppercase() {
            return Err(IdentifierError::InvalidFormat {
                description:
                    "first two characters must be uppercase ISO 3166-1 country code (e.g. \"DE\")"
                        .into(),
            });
        }
    }
    // Positions 3–33: alphanumeric body [A–Z, a–z, 0–9].
    for (i, c) in s.chars().enumerate().skip(2) {
        if !c.is_ascii_alphanumeric() {
            return Err(IdentifierError::InvalidCharacter {
                position: i,
                character: c,
            });
        }
    }
    Ok(())
}

/// Messlokations-ID (MeLo-ID): 33-character string.
///
/// Structure (per BDEW specification):
/// - **Positions 1–2**: ISO 3166-1 alpha-2 country code, uppercase (e.g. `"DE"`, `"AT"`)
/// - **Positions 3–33**: alphanumeric body `[A-Za-z0-9]` (31 characters)
///
/// No checksum is applied.
///
/// # Examples
/// ```
/// use rubo4e::identifiers::MeloId;
///
/// let id = MeloId::new("DE0000000000000000000000000000001").unwrap();
/// assert_eq!(id.as_ref(), "DE0000000000000000000000000000001");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "validate", derive(garde::Validate))]
#[cfg_attr(feature = "validate", garde(allow_unvalidated))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schemars", schemars(with = "String"))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "utoipa", schema(value_type = String))]
pub struct MeloId(#[cfg_attr(feature = "validate", garde(custom(check_melo_id)))] Box<str>);

#[cfg(feature = "validate")]
fn check_melo_id(value: &str, _: &()) -> Result<(), garde::Error> {
    validate(value).map_err(garde::Error::from)
}

impl MeloId {
    /// Creates a new `MeloId` after validating length and format.
    ///
    /// # Errors
    /// - [`IdentifierError::InvalidLength`] if `s` is not exactly 33 characters.
    /// - [`IdentifierError::InvalidFormat`] if the first two characters are not uppercase ASCII letters.
    /// - [`IdentifierError::InvalidCharacter`] if any character beyond position 2 is not ASCII alphanumeric.
    #[must_use = "the validated identifier is returned; ignoring it discards the result"]
    pub fn new(s: &str) -> Result<Self, IdentifierError> {
        validate(s)?;
        Ok(Self(Box::from(s)))
    }
}

impl TryFrom<String> for MeloId {
    type Error = IdentifierError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::new(&s)
    }
}

impl TryFrom<&str> for MeloId {
    type Error = IdentifierError;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Self::new(s)
    }
}

impl AsRef<str> for MeloId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for MeloId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::str::FromStr for MeloId {
    type Err = IdentifierError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for MeloId {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&self.0)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MeloId {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MeloId;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a 33-character Messlokations-ID")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<MeloId, E> {
                MeloId::new(v).map_err(|e| {
                    crate::identifiers::trace_identifier_deser_error("MeloId", v, &e);
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

    const VALID_33: &str = "DE0000000000000000000000000000001";

    #[test]
    fn valid_de_prefix_passes() {
        assert!(MeloId::new(VALID_33).is_ok());
        // Any 2-uppercase-letter country code is accepted
        assert!(MeloId::new("AB0000000000000000000000000000000").is_ok());
        // Alphanumeric body with uppercase letters is valid
        assert!(MeloId::new("DEABCDEFGHIJKLMNOPQRSTUVWXYZ12345").is_ok());
        // Mixed-case body (positions 3–33) is accepted
        assert!(MeloId::new("DEabcdefghijklmnopqrstuvwxyz12345").is_ok());
    }

    #[test]
    fn lowercase_country_prefix_fails() {
        // First char lowercase
        let err = MeloId::new("de0000000000000000000000000000001").unwrap_err();
        assert!(
            matches!(err, IdentifierError::InvalidFormat { .. }),
            "expected InvalidFormat, got {err:?}"
        );
        // Second char lowercase
        let err2 = MeloId::new("De0000000000000000000000000000001").unwrap_err();
        assert!(
            matches!(err2, IdentifierError::InvalidFormat { .. }),
            "expected InvalidFormat, got {err2:?}"
        );
        // Digit at position 1
        let err3 = MeloId::new("1E0000000000000000000000000000001").unwrap_err();
        assert!(
            matches!(err3, IdentifierError::InvalidFormat { .. }),
            "expected InvalidFormat, got {err3:?}"
        );
    }

    #[test]
    fn too_short_fails() {
        let short: String = "A".repeat(32);
        assert!(matches!(
            MeloId::new(&short).unwrap_err(),
            IdentifierError::InvalidLength {
                expected: LengthExpectation::Exact(33),
                actual: 32
            }
        ));
    }

    #[test]
    fn too_long_fails() {
        let long: String = "A".repeat(34);
        assert!(matches!(
            MeloId::new(&long).unwrap_err(),
            IdentifierError::InvalidLength {
                expected: LengthExpectation::Exact(33),
                actual: 34
            }
        ));
    }

    #[test]
    fn hyphen_in_value_fails() {
        let with_hyphen = "DE000000000000000000000000000000-";
        let err = MeloId::new(with_hyphen).unwrap_err();
        assert!(matches!(
            err,
            IdentifierError::InvalidCharacter {
                position: 32,
                character: '-'
            }
        ));
    }

    #[test]
    fn space_fails() {
        // 33 chars: "DE" + 29 zeros + " " + "1"
        let with_space = "DE00000000000000000000000000000 1";
        assert_eq!(with_space.len(), 33);
        let err = MeloId::new(with_space).unwrap_err();
        assert!(matches!(err, IdentifierError::InvalidCharacter { .. }));
    }

    #[test]
    fn round_trip() {
        let id = MeloId::new(VALID_33).unwrap();
        assert_eq!(id.to_string().parse::<MeloId>().unwrap(), id);
    }
}
