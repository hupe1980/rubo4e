use super::eic_code::{compute_check_char, EicCode};
use crate::error::IdentifierError;

/// Bilanzkreis-ID: 16-character EIC code restricted to type character `'Z'`
/// (Bilanzierungszone / Scheduling Area).
///
/// A `BilanzkreisId` is structurally identical to an [`EicCode`] but position 3
/// (the EIC type character) is always `'Z'`, identifying a Bilanzkreis in the
/// German gas and power balance-group settlement processes:
///
/// - **GaBi Gas** (BK7-14-020) — Bilanzkreis identification for gas-settlement
///   processes (BDEW INVOIC 13007, REQOTE 13001, etc.)
/// - **MABIS** (BK6-06-009) — power Bilanzkreis in spot and balance-group accounting
/// - **EDIFACT messages**: appears in the `NAD` and `LOC` segments where the
///   DE3227 qualifier is `Z01` or `Z02` (Bilanzkreis / Bilanzierungsgebiet).
///
/// ## Format
///
/// Same as [`EicCode`]:
/// - Positions 1–2: Local Issuing Office (LIO) identifier (alphanumeric)
/// - Position 3: **always `'Z'`** (Bilanzierungszone)
/// - Positions 4–15: LIO-specific code body (alphanumeric or `'-'` as padding)
/// - Position 16: ENTSO-E check character
///
/// ## Conversion
///
/// A `BilanzkreisId` can be infallibly converted into an [`EicCode`] via
/// [`From<BilanzkreisId>`].  The reverse requires `BilanzkreisId::new`, which
/// validates the `'Z'` type character.
///
/// ## Examples
/// ```
/// use rubo4e::identifiers::{BilanzkreisId, EicCode};
///
/// // Build from a 15-character prefix — check character is computed automatically
/// let bk = BilanzkreisId::from_prefix("11ZVEW---------").unwrap();
/// assert_eq!(&bk.as_ref()[2..3], "Z");
///
/// // Infallible conversion to the broader EicCode type
/// let eic: EicCode = bk.into();
/// assert_eq!(eic.type_char(), 'Z');
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "validate", derive(garde::Validate))]
#[cfg_attr(feature = "validate", garde(allow_unvalidated))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(
    feature = "schemars",
    schemars(schema_with = "crate::schema_helpers::bilanzkreis_id_schema")
)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "utoipa", schema(value_type = String))]
pub struct BilanzkreisId(
    #[cfg_attr(feature = "validate", garde(custom(check_bilanzkreis_id)))] Box<str>,
);

#[cfg(feature = "validate")]
fn check_bilanzkreis_id(value: &str, _: &()) -> Result<(), garde::Error> {
    validate(value).map_err(garde::Error::from)
}

fn validate(s: &str) -> Result<(), IdentifierError> {
    // Re-use the full EicCode validation (length, charset, checksum).
    EicCode::new(s)?;
    // Additional constraint: type character (position 3, index 2) must be 'Z'.
    // SAFETY: EicCode::new guarantees s.len() == 16 and s.is_ascii().
    let type_char = s.as_bytes()[2] as char;
    if type_char != 'Z' {
        return Err(IdentifierError::InvalidFormat {
            description: format!(
                "Bilanzkreis-ID requires EIC type character 'Z' at position 3, found '{type_char}'"
            )
            .into(),
        });
    }
    Ok(())
}

impl BilanzkreisId {
    /// Creates a new `BilanzkreisId` after full EIC validation plus type-character
    /// constraint (`position 3 == 'Z'`).
    ///
    /// # Errors
    /// - [`IdentifierError::InvalidLength`] if `s` is not exactly 16 characters.
    /// - [`IdentifierError::InvalidCharacter`] if any character is outside `[A-Z0-9-]`.
    /// - [`IdentifierError::InvalidFormat`] if position 3 is not `'Z'`.
    /// - [`IdentifierError::InvalidChecksum`] if position 16 is not the correct ENTSO-E check character.
    #[must_use = "the validated identifier is returned; ignoring it discards the result"]
    pub fn new(s: &str) -> Result<Self, IdentifierError> {
        validate(s)?;
        Ok(Self(Box::from(s)))
    }

    /// Builds a `BilanzkreisId` from a 15-character prefix by computing and
    /// appending the ENTSO-E check character.
    ///
    /// `prefix[2]` (position 3) must be `'Z'`.
    ///
    /// # Errors
    /// - [`IdentifierError::InvalidLength`] if `prefix` is not exactly 15 characters.
    /// - [`IdentifierError::InvalidFormat`] if `prefix[2]` is not `'Z'`, or if the
    ///   computed check character would be `'-'` (prohibited by ENTSO-E).
    /// - [`IdentifierError::InvalidCharacter`] if any character is outside `[A-Z0-9-]`.
    /// - [`IdentifierError::InvalidChecksum`] if the check character cannot be computed
    ///   (input contains characters outside the EIC alphabet).
    ///
    /// # Examples
    /// ```
    /// use rubo4e::identifiers::BilanzkreisId;
    ///
    /// let bk = BilanzkreisId::from_prefix("11ZVEW---------").unwrap();
    /// assert_eq!(bk.as_ref().len(), 16);
    /// assert_eq!(&bk.as_ref()[2..3], "Z");
    /// ```
    pub fn from_prefix(prefix: &str) -> Result<Self, IdentifierError> {
        if prefix.len() != 15 {
            return Err(IdentifierError::InvalidLength {
                expected: crate::error::LengthExpectation::Exact(15),
                actual: prefix.len(),
            });
        }
        if !prefix.is_ascii() {
            return Err(IdentifierError::InvalidFormat {
                description: "EIC prefix must contain only ASCII characters".into(),
            });
        }
        // Validate the 'Z' type character early for a clear error message.
        let type_char = prefix.as_bytes()[2] as char;
        if type_char != 'Z' {
            return Err(IdentifierError::InvalidFormat {
                description: format!(
                    "Bilanzkreis-ID requires EIC type character 'Z' at position 3, found '{type_char}'"
                )
                .into(),
            });
        }
        let prefix_bytes: &[u8; 15] = prefix.as_bytes().try_into().expect("length checked above");
        let check = compute_check_char(prefix_bytes).ok_or(IdentifierError::InvalidChecksum)?;
        let full = format!("{prefix}{check}");
        Self::new(&full)
    }
}

// ─── Conversion to/from EicCode ───────────────────────────────────────────────

impl From<BilanzkreisId> for String {
    fn from(bk: BilanzkreisId) -> Self {
        bk.0.into()
    }
}

impl From<BilanzkreisId> for EicCode {
    fn from(bk: BilanzkreisId) -> Self {
        // SAFETY: BilanzkreisId::new validates the full EicCode constraints,
        // so this conversion is infallible.
        EicCode::new(bk.as_ref()).expect("BilanzkreisId is always a valid EicCode")
    }
}

impl TryFrom<EicCode> for BilanzkreisId {
    type Error = IdentifierError;
    fn try_from(eic: EicCode) -> Result<Self, Self::Error> {
        Self::new(eic.as_ref())
    }
}

// ─── Standard trait implementations ─────────────────────────────────────────

impl TryFrom<String> for BilanzkreisId {
    type Error = IdentifierError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::new(&s)
    }
}

impl TryFrom<&str> for BilanzkreisId {
    type Error = IdentifierError;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Self::new(s)
    }
}

impl AsRef<str> for BilanzkreisId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for BilanzkreisId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::str::FromStr for BilanzkreisId {
    type Err = IdentifierError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for BilanzkreisId {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&self.0)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for BilanzkreisId {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = BilanzkreisId;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a 16-character EIC code with type character 'Z' (Bilanzkreis-ID)")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<BilanzkreisId, E> {
                BilanzkreisId::new(v).map_err(E::custom)
            }
        }
        d.deserialize_str(Visitor)
    }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    const Z_PREFIX: &str = "11ZVEW---------"; // 15 chars, valid Z-type prefix

    fn make_bk() -> BilanzkreisId {
        BilanzkreisId::from_prefix(Z_PREFIX).expect("valid Z-type prefix")
    }

    #[test]
    fn valid_z_type() {
        let bk = make_bk();
        assert_eq!(bk.as_ref().len(), 16);
        assert_eq!(&bk.as_ref()[2..3], "Z");
    }

    #[test]
    fn rejects_non_z_type() {
        // Y-type EIC (control area, not Bilanzkreis)
        let area_eic = "10YDE-EON------1";
        assert!(BilanzkreisId::new(area_eic).is_err());
        // Also rejects via from_prefix with wrong type char
        assert!(BilanzkreisId::from_prefix("11YVEW---------").is_err());
    }

    #[test]
    fn rejects_invalid_length() {
        assert!(BilanzkreisId::new("11Z-----------").is_err()); // 14 chars
        assert!(BilanzkreisId::from_prefix("11Z-----------").is_err()); // 14 chars, needs 15
    }

    #[test]
    fn from_prefix_short_rejected() {
        // 14-char prefix must be rejected explicitly (not silently ignored)
        let err = BilanzkreisId::from_prefix("11Z-----------").unwrap_err();
        assert!(matches!(
            err,
            crate::error::IdentifierError::InvalidLength { .. }
        ));
    }

    #[test]
    fn from_prefix_non_z_rejected() {
        let err = BilanzkreisId::from_prefix("11YVEW---------").unwrap_err();
        assert!(matches!(
            err,
            crate::error::IdentifierError::InvalidFormat { .. }
        ));
    }

    #[test]
    fn roundtrip_serde() {
        let bk = make_bk();
        #[cfg(feature = "serde")]
        {
            let json = serde_json::to_string(&bk).unwrap();
            let restored: BilanzkreisId = serde_json::from_str(&json).unwrap();
            assert_eq!(bk, restored);
        }
    }

    #[test]
    fn from_eic_code_z_type() {
        let bk = make_bk();
        let eic: EicCode = bk.clone().into();
        assert_eq!(eic.type_char(), 'Z');
        let back = BilanzkreisId::try_from(eic).unwrap();
        assert_eq!(bk, back);
    }

    #[test]
    fn into_string() {
        let bk = make_bk();
        let s = String::from(bk.clone());
        assert_eq!(s, bk.as_ref());
    }

    #[test]
    fn display_equals_inner() {
        let bk = make_bk();
        assert_eq!(bk.to_string(), bk.as_ref());
    }
}
