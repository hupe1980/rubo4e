use crate::error::IdentifierError;

/// Aktivierungs-ID (AkivId): identifier for a Redispatch 2.0 activation event.
///
/// Defined in **BDEW WiM AHB BK6-24-174** (§4.3) for §14a EnWG Modul 3
/// and the broader Redispatch 2.0 framework (BK6-20-059). The Aktivierungsidentifikator
/// uniquely identifies a single activation of a controllable resource (SteuerbareRessource)
/// within the MSB's product registry.
///
/// ## Format
///
/// 1–35 printable ASCII characters (graphic characters, `0x21`–`0x7E`).  The
/// content is free-form within this envelope — in practice it is assigned by the
/// MSB and often takes the form of a UUID or a composite of SR-ID + sequence number.
///
/// ## Usage context
///
/// - BDEW UTILTS (PID 55168) — Verpflichtungsanfrage: `AkivId` appears in the
///   `RFF+ACD` segment as the reference to the triggering activation event.
/// - BDEW ORDERS/ORDRSP (Steuerungsauftrag) — references the activation being
///   acknowledged.
///
/// ## Examples
/// ```
/// use rubo4e::identifiers::AkivId;
///
/// // UUID-style activation ID
/// let id = AkivId::new("550e8400-e29b-41d4-a716-446655440000").unwrap();
/// assert_eq!(id.as_ref(), "550e8400-e29b-41d4-a716-446655440000");
///
/// // Short form
/// let id = AkivId::new("AKIV-2026-00001").unwrap();
/// assert_eq!(id.as_ref(), "AKIV-2026-00001");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "validate", derive(garde::Validate))]
#[cfg_attr(feature = "validate", garde(allow_unvalidated))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(
    feature = "schemars",
    schemars(schema_with = "crate::schema_helpers::akiv_id_schema")
)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "utoipa", schema(value_type = String))]
pub struct AkivId(#[cfg_attr(feature = "validate", garde(custom(check_akiv_id)))] Box<str>);

/// Maximum length for an Aktivierungs-ID.
/// 36 accommodates UUID-format activation IDs (e.g. `550e8400-e29b-41d4-a716-446655440000`);
/// EDIFACT DE1004 supports up to 35 chars but BDEW WiM AHB BK6-24-174 permits
/// UUID-style identifiers in UTILTS PID 55168 (`RFF+ACD` reference field).
pub const AKIV_ID_MAX_LEN: usize = 36;

#[cfg(feature = "validate")]
fn check_akiv_id(value: &str, _: &()) -> Result<(), garde::Error> {
    validate(value).map_err(garde::Error::from)
}

fn validate(s: &str) -> Result<(), IdentifierError> {
    if s.is_empty() || s.len() > AKIV_ID_MAX_LEN {
        return Err(IdentifierError::InvalidLength {
            expected: crate::error::LengthExpectation::RangeInclusive {
                min: 1,
                max: AKIV_ID_MAX_LEN,
            },
            actual: s.len(),
        });
    }
    // Only printable ASCII (graphic characters, 0x21–0x7E) are permitted.
    // Control characters and DEL are rejected; space (0x20) is also rejected
    // to avoid leading/trailing whitespace issues.
    for (i, c) in s.chars().enumerate() {
        if !c.is_ascii() || c.is_ascii_control() || c == ' ' {
            return Err(IdentifierError::InvalidCharacter {
                position: i,
                character: c,
            });
        }
    }
    Ok(())
}

impl AkivId {
    /// Creates a new `AkivId` after validation (1–35 printable ASCII characters).
    ///
    /// # Errors
    /// - [`IdentifierError::InvalidLength`] if `s` is empty or longer than 35 characters.
    /// - [`IdentifierError::InvalidCharacter`] if any character is a control character, space,
    ///   or non-ASCII.
    #[must_use = "the validated identifier is returned; ignoring it discards the result"]
    pub fn new(s: &str) -> Result<Self, IdentifierError> {
        validate(s)?;
        Ok(Self(Box::from(s)))
    }
}

// ─── Standard trait implementations ─────────────────────────────────────────

impl TryFrom<String> for AkivId {
    type Error = IdentifierError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::new(&s)
    }
}

impl TryFrom<&str> for AkivId {
    type Error = IdentifierError;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Self::new(s)
    }
}

impl AsRef<str> for AkivId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for AkivId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::str::FromStr for AkivId {
    type Err = IdentifierError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl From<AkivId> for String {
    fn from(a: AkivId) -> Self {
        a.0.into()
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for AkivId {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&self.0)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for AkivId {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AkivId;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(
                    "a non-empty Aktivierungs-ID string of up to 35 printable ASCII characters",
                )
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<AkivId, E> {
                AkivId::new(v).map_err(E::custom)
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
    fn valid_uuid_style() {
        let id = AkivId::new("550e8400-e29b-41d4-a716-446655440000").unwrap();
        assert_eq!(id.as_ref(), "550e8400-e29b-41d4-a716-446655440000");
    }

    #[test]
    fn valid_short() {
        AkivId::new("A").unwrap();
    }

    #[test]
    fn valid_max_length() {
        let s: String = "X".repeat(AKIV_ID_MAX_LEN);
        AkivId::new(&s).unwrap();
    }

    #[test]
    fn rejects_too_long() {
        let s: String = "X".repeat(AKIV_ID_MAX_LEN + 1);
        assert!(AkivId::new(&s).is_err());
    }

    #[test]
    fn rejects_empty() {
        assert!(AkivId::new("").is_err());
    }

    #[test]
    fn rejects_space() {
        assert!(AkivId::new("AKIV 001").is_err());
    }

    #[test]
    fn rejects_control_char() {
        assert!(AkivId::new("AKIV\x01001").is_err());
    }

    #[test]
    fn display_roundtrip() {
        let id = AkivId::new("AKIV-2026-00001").unwrap();
        assert_eq!(id.to_string(), "AKIV-2026-00001");
    }

    #[test]
    fn from_str() {
        use std::str::FromStr;
        let id = AkivId::from_str("AKIV-2026-00001").unwrap();
        assert_eq!(id.as_ref(), "AKIV-2026-00001");
    }
}
