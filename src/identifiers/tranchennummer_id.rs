use crate::error::IdentifierError;

/// Tranchennummer-ID: 1–6 digit numeric identifier for a billing tranche.
///
/// Used in **MABIS Bilanzkreisabrechnung** (PID 13003, BK6-06-009) to identify
/// tranches within a balance-group settlement period.  Also appears in BDEW
/// MaBiS MSCONS and INVOIC messages where a settlement is split across multiple
/// tranches (partial deliveries or corrections).
///
/// ## Format
///
/// 1–6 decimal digits (`[0-9]`), **no leading zeros** unless the value is `"0"`.
///
/// ## EDIFACT wire format
///
/// The Tranchennummer is encoded as a reference number in EDIFACT segment `RFF+TN:`
/// (reference qualifier `TN` = Tranche Number).
///
/// ## Examples
/// ```
/// use rubo4e::identifiers::TranchennummerId;
///
/// let t = TranchennummerId::new("1").unwrap();
/// assert_eq!(t.as_ref(), "1");
/// assert_eq!(t.value(), 1u32);
///
/// let t = TranchennummerId::new("42").unwrap();
/// assert_eq!(t.value(), 42u32);
///
/// // Construct from an integer
/// let t = TranchennummerId::from_value(42).unwrap();
/// assert_eq!(t.as_ref(), "42");
///
/// // Maximum value
/// let max = TranchennummerId::from_value(999_999).unwrap();
/// assert_eq!(max.as_ref(), "999999");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "validate", derive(garde::Validate))]
#[cfg_attr(feature = "validate", garde(allow_unvalidated))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(
    feature = "schemars",
    schemars(schema_with = "crate::schema_helpers::tranchennummer_id_schema")
)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "utoipa", schema(value_type = String))]
pub struct TranchennummerId(
    #[cfg_attr(feature = "validate", garde(custom(check_tranchennummer_id)))] Box<str>,
);

/// Maximum numeric value for a [`TranchennummerId`] (6 digits → 999 999).
pub const TRANCHENNUMMER_MAX: u32 = 999_999;

#[cfg(feature = "validate")]
fn check_tranchennummer_id(value: &str, _: &()) -> Result<(), garde::Error> {
    validate(value).map_err(garde::Error::from)
}

fn validate(s: &str) -> Result<(), IdentifierError> {
    use crate::error::LengthExpectation;
    if s.is_empty() || s.len() > 6 {
        return Err(IdentifierError::InvalidLength {
            expected: LengthExpectation::RangeInclusive { min: 1, max: 6 },
            actual: s.len(),
        });
    }
    // All characters must be decimal digits.
    for (i, c) in s.chars().enumerate() {
        if !c.is_ascii_digit() {
            return Err(IdentifierError::InvalidCharacter {
                position: i,
                character: c,
            });
        }
    }
    // No leading zeros (except for the literal value "0").
    if s.len() > 1 && s.starts_with('0') {
        return Err(IdentifierError::InvalidFormat {
            description: "TranchennummerId must not have leading zeros (except \"0\")".into(),
        });
    }
    Ok(())
}

impl TranchennummerId {
    /// Creates a new `TranchennummerId` from a decimal digit string.
    ///
    /// # Errors
    /// - [`IdentifierError::InvalidLength`] if `s` is empty or longer than 6 characters.
    /// - [`IdentifierError::InvalidCharacter`] if any character is not a decimal digit.
    /// - [`IdentifierError::InvalidFormat`] if `s` has leading zeros and `s != "0"`.
    #[must_use = "the validated identifier is returned; ignoring it discards the result"]
    pub fn new(s: &str) -> Result<Self, IdentifierError> {
        validate(s)?;
        Ok(Self(Box::from(s)))
    }

    /// Creates a `TranchennummerId` from a `u32` value (0–999 999).
    ///
    /// # Errors
    /// - [`IdentifierError::InvalidFormat`] if `value > 999_999`.
    ///
    /// # Examples
    /// ```
    /// use rubo4e::identifiers::TranchennummerId;
    ///
    /// let t = TranchennummerId::from_value(42).unwrap();
    /// assert_eq!(t.as_ref(), "42");
    /// ```
    pub fn from_value(value: u32) -> Result<Self, IdentifierError> {
        if value > TRANCHENNUMMER_MAX {
            return Err(IdentifierError::InvalidFormat {
                description: format!(
                    "TranchennummerId value {value} exceeds maximum {TRANCHENNUMMER_MAX}"
                )
                .into(),
            });
        }
        Ok(Self(Box::from(value.to_string().as_str())))
    }

    /// Returns the numeric value of this Tranchennummer.
    ///
    /// Guaranteed to be in range `0..=999_999` for any validly constructed instance.
    #[must_use]
    pub fn value(&self) -> u32 {
        // SAFETY: validated at construction — only decimal digits, no leading zeros,
        // max 6 chars → max value 999_999 which fits in u32.
        self.0.parse().expect("validated digits")
    }
}

// ─── Standard trait implementations ─────────────────────────────────────────

impl TryFrom<String> for TranchennummerId {
    type Error = IdentifierError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::new(&s)
    }
}

impl TryFrom<&str> for TranchennummerId {
    type Error = IdentifierError;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Self::new(s)
    }
}

impl TryFrom<u32> for TranchennummerId {
    type Error = IdentifierError;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        Self::from_value(v)
    }
}

impl From<TranchennummerId> for u32 {
    /// Converts a [`TranchennummerId`] to its numeric value.
    ///
    /// This is infallible because `TranchennummerId` is always in range `0..=999_999`.
    fn from(t: TranchennummerId) -> Self {
        t.value()
    }
}

impl From<TranchennummerId> for String {
    fn from(t: TranchennummerId) -> Self {
        t.0.into()
    }
}

impl AsRef<str> for TranchennummerId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for TranchennummerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::str::FromStr for TranchennummerId {
    type Err = IdentifierError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for TranchennummerId {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&self.0)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for TranchennummerId {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TranchennummerId;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a 1–6 digit decimal string without leading zeros (Tranchennummer)")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<TranchennummerId, E> {
                TranchennummerId::new(v).map_err(E::custom)
            }
        }
        d.deserialize_str(Visitor)
    }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_single_digit() {
        let t = TranchennummerId::new("1").unwrap();
        assert_eq!(t.value(), 1);
    }

    #[test]
    fn valid_zero() {
        let t = TranchennummerId::new("0").unwrap();
        assert_eq!(t.value(), 0);
    }

    #[test]
    fn valid_max() {
        let t = TranchennummerId::from_value(TRANCHENNUMMER_MAX).unwrap();
        assert_eq!(t.value(), TRANCHENNUMMER_MAX);
        assert_eq!(t.as_ref(), "999999");
    }

    #[test]
    fn from_value_roundtrip() {
        for v in [0u32, 1, 42, 100, 9999, 999_999] {
            let t = TranchennummerId::from_value(v).unwrap();
            assert_eq!(t.value(), v);
        }
    }

    #[test]
    fn rejects_empty() {
        assert!(TranchennummerId::new("").is_err());
    }

    #[test]
    fn rejects_leading_zero() {
        assert!(TranchennummerId::new("01").is_err());
        assert!(TranchennummerId::new("007").is_err());
    }

    #[test]
    fn rejects_non_digit() {
        assert!(TranchennummerId::new("1A").is_err());
        assert!(TranchennummerId::new("T1").is_err());
    }

    #[test]
    fn rejects_too_long() {
        assert!(TranchennummerId::new("1234567").is_err()); // 7 digits
    }

    #[test]
    fn rejects_from_value_overflow() {
        assert!(TranchennummerId::from_value(1_000_000).is_err());
    }

    #[test]
    fn display_roundtrip() {
        let t = TranchennummerId::new("42").unwrap();
        assert_eq!(t.to_string(), "42");
    }

    #[test]
    fn try_from_u32() {
        let t = TranchennummerId::try_from(7u32).unwrap();
        assert_eq!(t.value(), 7);
    }
}
