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
#[cfg_attr(
    feature = "schemars",
    schemars(schema_with = "crate::schema_helpers::marktpartner_id_schema")
)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "utoipa", schema(
    value_type = String,
    pattern = r"^[0-9]{13}$",
    example = "9900357000004",
    description = "13-stellige Marktpartner-ID: BDEW-Codenummer Strom (Prefix 99), DVGW-Codenummer Gas (Prefix 98) oder GS1 GLN"
))]
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

    /// Converts this `MarktpartnerId` to its numeric `i64` representation.
    ///
    /// A validated 13-digit numeric string always fits in `i64`
    /// (`9_999_999_999_999 < i64::MAX`), so the conversion is infallible for
    /// any value that passed construction validation.
    ///
    /// # Examples
    /// ```
    /// use rubo4e::identifiers::MarktpartnerId;
    ///
    /// let id = MarktpartnerId::new("9900357000004").unwrap();
    /// assert_eq!(id.to_i64(), 9_900_357_000_004_i64);
    /// ```
    #[must_use]
    pub fn to_i64(&self) -> i64 {
        // All 13-digit decimal strings fit in i64; parse cannot fail on validated input.
        self.0
            .parse::<i64>()
            .expect("MarktpartnerId is validated as 13 ASCII digits; parse to i64 cannot fail")
    }

    /// Returns `true` if this ID was issued by BDEW (prefix `"99"`).
    ///
    /// BDEW-Codenummern for the German electricity market start with `99`.
    ///
    /// # Examples
    /// ```
    /// use rubo4e::identifiers::MarktpartnerId;
    ///
    /// assert!(MarktpartnerId::new("9900357000004").unwrap().is_bdew());
    /// assert!(!MarktpartnerId::new("9812345000003").unwrap().is_bdew());
    /// ```
    #[must_use]
    pub fn is_bdew(&self) -> bool {
        self.0.starts_with("99")
    }

    /// Returns `true` if this ID was issued by DVGW (prefix `"98"`).
    ///
    /// DVGW-Codenummern for the German gas market start with `98`.
    ///
    /// # Examples
    /// ```
    /// use rubo4e::identifiers::MarktpartnerId;
    ///
    /// assert!(MarktpartnerId::new("9812345000003").unwrap().is_dvgw());
    /// assert!(!MarktpartnerId::new("9900357000004").unwrap().is_dvgw());
    /// ```
    #[must_use]
    pub fn is_dvgw(&self) -> bool {
        self.0.starts_with("98")
    }

    /// Returns `true` if this ID is a GS1 Global Location Number (GLN).
    ///
    /// A 13-digit Marktpartner-ID that does **not** start with `"98"` or `"99"` is
    /// treated as a GS1 GLN.  Note that GS1 GLNs carry an EAN-13 check digit while
    /// BDEW/DVGW codes do not; `MarktpartnerId` validates only digit-only format and
    /// length, not the EAN-13 check.
    ///
    /// # Examples
    /// ```
    /// use rubo4e::identifiers::MarktpartnerId;
    ///
    /// assert!(MarktpartnerId::new("4012345678901").unwrap().is_gln());
    /// assert!(!MarktpartnerId::new("9900357000004").unwrap().is_gln());
    /// ```
    #[must_use]
    pub fn is_gln(&self) -> bool {
        !self.is_bdew() && !self.is_dvgw()
    }

    /// Returns the EDIFACT **NAD DE3055** agency code for this identifier.
    ///
    /// | Prefix | Authority          | NAD DE3055 |
    /// |--------|--------------------|------------|
    /// | `99…`  | BDEW Strom         | `"293"`    |
    /// | `98…`  | DVGW Gas           | `"332"`    |
    /// | other  | GS1 GLN            | `"9"`      |
    ///
    /// Use this code in EDIFACT NAD segments (e.g. `NAD+MS+<id>::293`).
    ///
    /// # Examples
    /// ```
    /// use rubo4e::identifiers::MarktpartnerId;
    ///
    /// let bdew = MarktpartnerId::new("9900357000004").unwrap();
    /// assert_eq!(bdew.nad_agency_code(), "293");
    ///
    /// let dvgw = MarktpartnerId::new("9812345000003").unwrap();
    /// assert_eq!(dvgw.nad_agency_code(), "332");
    ///
    /// let gln = MarktpartnerId::new("4012345678901").unwrap();
    /// assert_eq!(gln.nad_agency_code(), "9");
    /// ```
    #[must_use]
    pub fn nad_agency_code(&self) -> &'static str {
        if self.is_bdew() {
            "293"
        } else if self.is_dvgw() {
            "332"
        } else {
            "9"
        }
    }

    /// Returns the EDIFACT **UNB DE0007** sender/receiver qualifier for this identifier.
    ///
    /// | Prefix | Authority          | UNB DE0007 |
    /// |--------|--------------------|------------|
    /// | `99…`  | BDEW Strom         | `"500"`    |
    /// | `98…`  | DVGW Gas           | `"502"`    |
    /// | other  | GS1 GLN            | `"14"`     |
    ///
    /// Use this code in the EDIFACT UNB interchange header
    /// (e.g. `UNB+UNOC:3+<id>:500+...`).
    ///
    /// # Examples
    /// ```
    /// use rubo4e::identifiers::MarktpartnerId;
    ///
    /// let bdew = MarktpartnerId::new("9900357000004").unwrap();
    /// assert_eq!(bdew.unb_agency_code(), "500");
    ///
    /// let dvgw = MarktpartnerId::new("9812345000003").unwrap();
    /// assert_eq!(dvgw.unb_agency_code(), "502");
    ///
    /// let gln = MarktpartnerId::new("4012345678901").unwrap();
    /// assert_eq!(gln.unb_agency_code(), "14");
    /// ```
    #[must_use]
    pub fn unb_agency_code(&self) -> &'static str {
        if self.is_bdew() {
            "500"
        } else if self.is_dvgw() {
            "502"
        } else {
            "14"
        }
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
                f.write_str(
                    "a 13-digit Marktpartner-ID (BDEW-Codenummer, DVGW-Codenummer oder GS1 GLN)",
                )
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

    #[test]
    fn to_i64_conversion() {
        let id = MarktpartnerId::new("9900357000004").unwrap();
        assert_eq!(id.to_i64(), 9_900_357_000_004_i64);

        let zeros = MarktpartnerId::new("0000000000000").unwrap();
        assert_eq!(zeros.to_i64(), 0_i64);

        let max13 = MarktpartnerId::new("9999999999999").unwrap();
        assert_eq!(max13.to_i64(), 9_999_999_999_999_i64);
    }
}

/// Serde module that serializes/deserializes a [`MarktpartnerId`] as a JSON
/// integer (`i64`) rather than a string.
///
/// Some BDEW REST APIs (e.g. API-Webdienste Strom) represent GLN / Rollencodenummern
/// as 64-bit integers in their JSON payloads.  Use this adapter when the target
/// endpoint mandates integer encoding:
///
/// ```rust,ignore
/// #[serde(with = "rubo4e::identifiers::serde_as_i64")]
/// pub market_partner_id: MarktpartnerId,
/// ```
///
/// The deserializer accepts **both** integer and string representations so that
/// data from string-typed upstream sources can be seamlessly decoded.
#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
pub mod serde_as_i64 {
    use super::MarktpartnerId;
    use crate::error::IdentifierError;

    /// Serialize a `MarktpartnerId` as a JSON integer.
    pub fn serialize<S: serde::Serializer>(id: &MarktpartnerId, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_i64(id.to_i64())
    }

    /// Deserialize a `MarktpartnerId` from either a JSON integer or a JSON string.
    pub fn deserialize<'de, D: serde::Deserializer<'de>>(d: D) -> Result<MarktpartnerId, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MarktpartnerId;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a 13-digit Marktpartner-ID as an integer or string")
            }
            fn visit_i64<E: serde::de::Error>(self, v: i64) -> Result<MarktpartnerId, E> {
                if v < 0 {
                    return Err(E::custom(IdentifierError::InvalidFormat {
                        description: "MarktpartnerId cannot be negative".into(),
                    }));
                }
                let s = format!("{v:013}");
                MarktpartnerId::new(&s).map_err(E::custom)
            }
            fn visit_u64<E: serde::de::Error>(self, v: u64) -> Result<MarktpartnerId, E> {
                let s = format!("{v:013}");
                MarktpartnerId::new(&s).map_err(E::custom)
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<MarktpartnerId, E> {
                MarktpartnerId::new(v).map_err(E::custom)
            }
        }
        d.deserialize_any(Visitor)
    }
}
