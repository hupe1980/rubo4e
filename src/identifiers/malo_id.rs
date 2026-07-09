#[cfg(test)]
use super::checksum::bdew_check_digit;
use super::checksum::{compute_11digit_from_base, validate_11digit_bdew};
use crate::error::IdentifierError;
#[cfg(test)]
use crate::error::LengthExpectation;

// ─── Type ────────────────────────────────────────────────────────────────────

/// Marktlokations-ID (MaLo-ID): 11-digit string validated by BDEW checksum.
///
/// The 11th digit is a check digit computed from the first ten digits using the
/// BDEW alternating-weight algorithm.
///
/// # Examples
/// ```
/// use rubo4e::identifiers::MaloId;
///
/// let id = MaloId::new("51238696780").unwrap();
/// assert_eq!(id.as_ref(), "51238696780");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "validate", derive(garde::Validate))]
#[cfg_attr(feature = "validate", garde(allow_unvalidated))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(
    feature = "schemars",
    schemars(schema_with = "crate::schema_helpers::malo_id_schema")
)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "utoipa", schema(
    value_type = String,
    pattern = r"^[0-9]{11}$",
    example = "51238696780",
    description = "11-stellige BDEW Marktlokations-ID mit BDEW-Prüfziffer (11. Stelle)"
))]
pub struct MaloId(#[cfg_attr(feature = "validate", garde(custom(check_malo_id)))] Box<str>);

#[cfg(feature = "validate")]
fn check_malo_id(value: &str, _: &()) -> Result<(), garde::Error> {
    validate_11digit_bdew(value).map_err(garde::Error::from)
}

impl MaloId {
    /// Creates a new `MaloId` after validating the BDEW checksum.
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

    /// Constructs a valid `MaloId` from a 10-digit numeric base string by computing
    /// the BDEW alternating-weight check digit and appending it.
    ///
    /// This is the canonical way to generate a valid `MaloId` when only the 10-digit
    /// base is known (e.g. during test data generation or synthetic MaLo assignment).
    ///
    /// # Errors
    /// - [`IdentifierError::InvalidLength`] if `base` is not exactly 10 characters.
    /// - [`IdentifierError::InvalidCharacter`] if any character is not a decimal digit.
    ///
    /// # Examples
    /// ```
    /// use rubo4e::identifiers::MaloId;
    ///
    /// let id = MaloId::from_base("5123869678").unwrap();
    /// assert_eq!(id.as_ref(), "51238696780");
    /// ```
    pub fn from_base(base: &str) -> Result<Self, IdentifierError> {
        let full = compute_11digit_from_base(base)?;
        Ok(Self(Box::from(full.as_str())))
    }

    /// Computes the BDEW check digit for a 10-digit numeric base string without
    /// constructing a full `MaloId`.
    ///
    /// Returns the single check digit (`0..=9`) on success.
    ///
    /// # Errors
    /// - [`IdentifierError::InvalidLength`] if `base` is not exactly 10 characters.
    /// - [`IdentifierError::InvalidCharacter`] if any character is not a decimal digit.
    ///
    /// # Examples
    /// ```
    /// use rubo4e::identifiers::MaloId;
    ///
    /// assert_eq!(MaloId::check_digit("5123869678").unwrap(), 0);
    /// ```
    pub fn check_digit(base: &str) -> Result<u8, IdentifierError> {
        let full = compute_11digit_from_base(base)?;
        let check = full.as_bytes().last().copied().expect("11 chars") - b'0';
        Ok(check)
    }
}

// ─── Standard trait impls ────────────────────────────────────────────────────

impl TryFrom<String> for MaloId {
    type Error = IdentifierError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::new(&s)
    }
}

impl TryFrom<&str> for MaloId {
    type Error = IdentifierError;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Self::new(s)
    }
}

impl AsRef<str> for MaloId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for MaloId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::str::FromStr for MaloId {
    type Err = IdentifierError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

// ─── Serde ───────────────────────────────────────────────────────────────────

#[cfg(feature = "serde")]
impl serde::Serialize for MaloId {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&self.0)
    }
}

/// Efficient deserialization using `visit_str` — avoids allocating an
/// intermediate `String` before constructing the validated `Box<str>`.
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MaloId {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MaloId;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("an 11-digit Marktlokations-ID (BDEW checksum)")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<MaloId, E> {
                MaloId::new(v).map_err(|e| {
                    crate::identifiers::trace_identifier_deser_error("MaloId", v, &e);
                    serde::de::Error::custom(e)
                })
            }
        }
        d.deserialize_str(Visitor)
    }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    /// Builds a valid 11-digit MaloId string from 10 digit values using the
    /// reference checksum algorithm.
    fn make_valid(prefix: &[u8; 10]) -> String {
        super::super::checksum::make_valid_11digit(prefix)
    }

    // ── Valid IDs ─────────────────────────────────────────────────────────

    #[test]
    fn valid_constructed_ids_pass() {
        let prefixes: &[[u8; 10]] = &[
            [5, 1, 2, 3, 8, 6, 9, 6, 7, 8], // check = 0 → "51238696780"
            [1, 2, 3, 4, 5, 6, 7, 8, 9, 0], // check = 7 → "12345678907"
            [0, 9, 8, 7, 6, 5, 4, 3, 2, 1], // check = 3 → "09876543213"
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1], // check = 5 → "11111111115"
            [2, 2, 2, 2, 2, 2, 2, 2, 2, 2], // check = 0 → "22222222220"
            [9, 9, 9, 9, 9, 9, 9, 9, 9, 9], // check = 0 → "99999999990"
            [1, 2, 3, 4, 5, 1, 2, 3, 4, 5], // check = 4 → "12345123454"
            [5, 6, 7, 8, 9, 5, 6, 7, 8, 9], // check = 0 → "56789567890"
            [1, 3, 5, 7, 9, 2, 4, 6, 8, 0], // check = 5 → "13579246805"
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0], // check = 8 → "10000000008"
        ];
        for prefix in prefixes {
            let id_str = make_valid(prefix);
            let id = MaloId::new(&id_str)
                .unwrap_or_else(|e| panic!("{id_str} should be valid but got: {e}"));
            // round-trip
            assert_eq!(id.to_string().parse::<MaloId>().unwrap(), id);
        }
    }

    // ── Invalid IDs ───────────────────────────────────────────────────────

    #[test]
    fn too_short_returns_invalid_length() {
        let err = MaloId::new("1234567890").unwrap_err();
        assert!(matches!(
            err,
            IdentifierError::InvalidLength {
                expected: LengthExpectation::Exact(11),
                actual: 10
            }
        ));
    }

    #[test]
    fn too_long_returns_invalid_length() {
        let err = MaloId::new("123456789012").unwrap_err();
        assert!(matches!(
            err,
            IdentifierError::InvalidLength {
                expected: LengthExpectation::Exact(11),
                actual: 12
            }
        ));
    }

    #[test]
    fn empty_returns_invalid_length() {
        let err = MaloId::new("").unwrap_err();
        assert!(matches!(
            err,
            IdentifierError::InvalidLength {
                expected: LengthExpectation::Exact(11),
                actual: 0
            }
        ));
    }

    #[test]
    fn non_digit_returns_invalid_character() {
        let err = MaloId::new("1234X678901").unwrap_err();
        assert!(matches!(
            err,
            IdentifierError::InvalidCharacter {
                position: 4,
                character: 'X'
            }
        ));
    }

    #[test]
    fn wrong_check_digit_returns_invalid_checksum() {
        // "51238696780" is valid; flip the check digit
        let err = MaloId::new("51238696781").unwrap_err();
        assert!(matches!(err, IdentifierError::InvalidChecksum));
    }

    #[test]
    fn multiple_wrong_check_digits() {
        for wrong in ["51238696782", "51238696783", "51238696789"] {
            let err = MaloId::new(wrong).unwrap_err();
            assert!(matches!(err, IdentifierError::InvalidChecksum));
        }
    }

    #[test]
    fn space_character_returns_invalid_character() {
        let err = MaloId::new("5123 696780").unwrap_err();
        assert!(matches!(
            err,
            IdentifierError::InvalidCharacter {
                position: 4,
                character: ' '
            }
        ));
    }

    #[test]
    fn leading_hyphen_returns_invalid_character() {
        let err = MaloId::new("-1238696780").unwrap_err();
        assert!(matches!(
            err,
            IdentifierError::InvalidCharacter {
                position: 0,
                character: '-'
            }
        ));
    }

    // ── Display / AsRef / TryFrom ────────────────────────────────────────

    #[test]
    fn display_equals_inner_string() {
        let id_str = make_valid(&[5, 1, 2, 3, 8, 6, 9, 6, 7, 8]);
        let id = MaloId::new(&id_str).unwrap();
        assert_eq!(id.to_string(), id_str);
        assert_eq!(id.as_ref(), id_str);
    }

    #[test]
    fn try_from_str_delegates_to_new() {
        let valid = make_valid(&[1, 2, 3, 4, 5, 1, 2, 3, 4, 5]);
        assert!(MaloId::try_from(valid.as_str()).is_ok());
        assert!(MaloId::try_from("bad").is_err());
    }

    #[test]
    fn try_from_string_delegates_to_new() {
        let valid = make_valid(&[9, 9, 9, 9, 9, 9, 9, 9, 9, 9]);
        assert!(MaloId::try_from(valid).is_ok());
    }

    // ── Checksum helper self-consistency ─────────────────────────────────

    #[test]
    fn compute_check_digit_is_deterministic() {
        let prefix = [1u8, 2, 3, 4, 5, 6, 7, 8, 9, 0];
        assert_eq!(bdew_check_digit(&prefix), bdew_check_digit(&prefix));
    }

    #[test]
    fn all_zeros_check_is_zero() {
        assert_eq!(bdew_check_digit(&[0u8; 10]), 0);
    }
}
