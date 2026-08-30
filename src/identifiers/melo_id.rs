use crate::error::IdentifierError;

use super::zaehlpunkt::validate_zaehlpunktbezeichnung as validate;

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
#[cfg_attr(
    feature = "schemars",
    schemars(schema_with = "crate::schema_helpers::melo_id_schema")
)]
#[cfg_attr(
    feature = "schemars",
    schemars(description = crate::identifiers::schema::MELO_ID.description)
)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "utoipa", schema(
    value_type = String,
    pattern = r"^[A-Z]{2}[A-Za-z0-9]{31}$",
    example = "DE0000000000000000000000000000001",
    description = crate::identifiers::schema::MELO_ID.description
))]
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

    /// Returns the ISO 3166-1 alpha-2 country code prefix (first two characters).
    ///
    /// Guaranteed to be two uppercase ASCII letters for any successfully
    /// constructed `MeloId`.
    ///
    /// # Examples
    /// ```
    /// use rubo4e::identifiers::MeloId;
    ///
    /// let id = MeloId::new("DE0000000000000000000000000000001").unwrap();
    /// assert_eq!(id.country_code(), "DE");
    /// ```
    #[must_use]
    pub fn country_code(&self) -> &str {
        // Safety: validated at construction — first 2 chars are always ASCII uppercase.
        &self.0[..2]
    }

    /// Returns `true` if the country code is `"DE"` (Germany).
    ///
    /// Useful for EDIFACT NAD routing and MaStR query filtering.
    ///
    /// # Examples
    /// ```
    /// use rubo4e::identifiers::MeloId;
    ///
    /// let de = MeloId::new("DE0000000000000000000000000000001").unwrap();
    /// assert!(de.is_german());
    ///
    /// let at = MeloId::new("AT0000000000000000000000000000001").unwrap();
    /// assert!(!at.is_german());
    /// ```
    #[must_use]
    pub fn is_german(&self) -> bool {
        self.country_code() == "DE"
    }
}

impl_identifier_traits!(MeloId, "a 33-character Messlokations-ID");

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::LengthExpectation;

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
