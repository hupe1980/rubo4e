use crate::error::IdentifierError;

// ─── Grammar ──────────────────────────────────────────────────────────────────
//
// OBIS ID (IEC 62056-61 / BDEW):
//   [A-B:]C.D[.E][*F]
//   [A:]C.D[.E][*F]    (A-only prefix: A present, B absent)
//
// Components:
//   A  – value group A (energy type), optional (default 1)
//   B  – value group B (channel), optional (default 0)
//   C  – value group C (physical quantity), mandatory
//   D  – value group D (measurement type), mandatory
//   E  – value group E (tariff), optional
//   F  – value group F (billing period), optional (separator '*' or '&')
//
// All components are non-negative integers (no leading zeros enforced).
//
// Prefix forms:
//   A-B:   both A and B are present (e.g. "1-0:1.8.0")
//   A:     A is present, B is absent — `ObisComponents::b` will be `None`
//          (e.g. "1:1.8")
//   <none> both A and B absent (e.g. "1.8.1")
// ─────────────────────────────────────────────────────────────────────────────

/// Parsed representation of an OBIS identifier.
///
/// The `F` component uses `'*'` as its separator when re-serialised; `'&'`
/// is accepted on input but normalised to `'*'`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ObisComponents {
    /// Value group A (energy type). `None` means the full `A-B:` prefix was omitted.
    pub a: Option<u32>,
    /// Value group B (channel).
    /// - `Some(b)` when both A and B appear in the `A-B:` prefix.
    /// - `None` when only the `A:` prefix was given (A present, B absent).
    /// - `None` when the entire `A-B:` / `A:` prefix was omitted.
    pub b: Option<u32>,
    /// Value group C (physical quantity) – mandatory.
    pub c: u32,
    /// Value group D (measurement type) – mandatory.
    pub d: u32,
    /// Value group E (tariff) – optional.
    pub e: Option<u32>,
    /// Value group F (billing period) – optional.
    pub f: Option<u32>,
}

// ─── Parser helpers ───────────────────────────────────────────────────────────

/// Parses a non-negative integer from the beginning of `s`, returning `(value, rest)`.
/// Returns `None` if `s` is empty or does not start with a digit.
fn parse_uint(s: &str) -> Option<(u32, &str)> {
    let end = s.find(|c: char| !c.is_ascii_digit()).unwrap_or(s.len());
    if end == 0 {
        return None;
    }
    let n = s[..end].parse::<u32>().ok()?;
    Some((n, &s[end..]))
}

fn validate_and_parse(s: &str) -> Result<ObisComponents, IdentifierError> {
    if s.is_empty() {
        return Err(IdentifierError::InvalidFormat {
            description: "OBIS code must not be empty".into(),
        });
    }

    // ── Split off optional F component (*F or &F) ────────────────────────────
    let (s, f) = if let Some(idx) = s.rfind(['*', '&']) {
        let f_str = &s[idx + 1..];
        let f_val = f_str
            .parse::<u32>()
            .map_err(|_| IdentifierError::InvalidFormat {
                description: "F component must be a non-negative integer".into(),
            })?;
        (&s[..idx], Some(f_val))
    } else {
        (s, None)
    };

    // ── Split off optional A-B: prefix ───────────────────────────────────────
    let (s, a, b) = if let Some(colon_pos) = s.find(':') {
        let prefix = &s[..colon_pos];
        let rest = &s[colon_pos + 1..];
        // Prefix is either "A" or "A-B"
        if let Some(dash_pos) = prefix.find('-') {
            let a =
                prefix[..dash_pos]
                    .parse::<u32>()
                    .map_err(|_| IdentifierError::InvalidFormat {
                        description: "A component must be a non-negative integer".into(),
                    })?;
            let b = prefix[dash_pos + 1..].parse::<u32>().map_err(|_| {
                IdentifierError::InvalidFormat {
                    description: "B component must be a non-negative integer".into(),
                }
            })?;
            (rest, Some(a), Some(b))
        } else {
            let a = prefix
                .parse::<u32>()
                .map_err(|_| IdentifierError::InvalidFormat {
                    description: "A component must be a non-negative integer".into(),
                })?;
            (rest, Some(a), None)
        }
    } else {
        (s, None, None)
    };

    // ── Parse mandatory C.D[.E] ──────────────────────────────────────────────
    let (c, rest) = parse_uint(s).ok_or(IdentifierError::InvalidFormat {
        description: "C component (mandatory) must be a non-negative integer".into(),
    })?;

    // BDEW / IEC 62056-61: C (physical quantity) must be ≥ 1.
    // C=0 is reserved and not a valid electricity-metering quantity code.
    if c == 0 {
        return Err(IdentifierError::InvalidFormat {
            description:
                "C component (physical quantity) must be ≥ 1 per IEC 62056-61 / BDEW specification"
                    .into(),
        });
    }

    if !rest.starts_with('.') {
        return Err(IdentifierError::InvalidFormat {
            description: "expected '.' separator between C and D".into(),
        });
    }
    let rest = &rest[1..];

    let (d, rest) = parse_uint(rest).ok_or(IdentifierError::InvalidFormat {
        description: "D component (mandatory) must be a non-negative integer".into(),
    })?;

    let e = if let Some(rest) = rest.strip_prefix('.') {
        let (e_val, remainder) = parse_uint(rest).ok_or(IdentifierError::InvalidFormat {
            description: "E component must be a non-negative integer after '.'".into(),
        })?;
        if !remainder.is_empty() {
            return Err(IdentifierError::InvalidFormat {
                description: "unexpected trailing characters after E component".into(),
            });
        }
        Some(e_val)
    } else if rest.is_empty() {
        None
    } else {
        return Err(IdentifierError::InvalidFormat {
            description: "unexpected trailing characters after D component".into(),
        });
    };

    Ok(ObisComponents { a, b, c, d, e, f })
}

// ─── Type ────────────────────────────────────────────────────────────────────

/// OBIS identifier (IEC 62056-61 / BDEW): compact reference for metering values.
///
/// Grammar: `[A-B:]C.D[.E][*F]`
///
/// All components are non-negative integers.  Components `A`, `B`, `E`, and `F`
/// are optional; `C` and `D` are mandatory.
///
/// The stored string is **normalised** at construction time: the alternative `&`
/// separator for the F component is converted to `*`.  Two codes that differ
/// only in separator are therefore equal.
///
/// Use [`ObisCode::components`] to access the parsed values.
///
/// # Examples
/// ```
/// use rubo4e::identifiers::ObisCode;
///
/// ObisCode::new("1-0:1.8.1").unwrap();   // A-B:C.D.E
/// ObisCode::new("1-0:1.8.0*255").unwrap(); // with F component
/// ObisCode::new("1:1.8").unwrap();           // A-only prefix (B absent)
/// ObisCode::new("1.8.1").unwrap();          // C.D.E only
/// ObisCode::new("1.8").unwrap();            // C.D only
/// // & separator is accepted and normalised to *
/// assert_eq!(ObisCode::new("1.8.1&255").unwrap(), ObisCode::new("1.8.1*255").unwrap());
/// // A-only: components() returns b = None
/// let c = ObisCode::new("1:1.8").unwrap().components();
/// assert_eq!(c.a, Some(1));
/// assert_eq!(c.b, None);
/// assert_eq!(c.c, 1);
/// assert_eq!(c.d, 8);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "validate", derive(garde::Validate))]
#[cfg_attr(feature = "validate", garde(allow_unvalidated))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schemars", schemars(with = "String"))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "utoipa", schema(value_type = String))]
pub struct ObisCode(#[cfg_attr(feature = "validate", garde(custom(check_obis_code)))] Box<str>);

#[cfg(feature = "validate")]
fn check_obis_code(value: &str, _: &()) -> Result<(), garde::Error> {
    validate_and_parse(value)
        .map(|_| ())
        .map_err(garde::Error::from)
}

impl ObisCode {
    /// Creates a new `ObisCode` after full structural validation.
    ///
    /// The `&` separator for the F component is normalised to `*` so that
    /// semantically identical codes compare equal.
    ///
    /// # Errors
    /// Returns [`IdentifierError::InvalidFormat`] if the input does not conform
    /// to the OBIS grammar.
    #[must_use = "the validated identifier is returned; ignoring it discards the result"]
    pub fn new(s: &str) -> Result<Self, IdentifierError> {
        validate_and_parse(s)?;
        // Normalise the alternative F-component separator.
        let normalised: Box<str> = if s.contains('&') {
            s.replace('&', "*").into()
        } else {
            Box::from(s)
        };
        Ok(Self(normalised))
    }

    /// Parses and returns the individual OBIS value groups.
    pub fn components(&self) -> ObisComponents {
        // SAFETY: `ObisCode` values are only constructable via `new()` / `TryFrom`,
        // both of which call `validate_and_parse` and return `Err` on invalid input.
        // A stored `ObisCode` is therefore always a valid OBIS string, so
        // `validate_and_parse` cannot return `Err` here.
        validate_and_parse(&self.0)
            .expect("ObisCode invariant violated: stored string is not a valid OBIS identifier")
    }
}

impl TryFrom<String> for ObisCode {
    type Error = IdentifierError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::new(&s)
    }
}

impl TryFrom<&str> for ObisCode {
    type Error = IdentifierError;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Self::new(s)
    }
}

impl AsRef<str> for ObisCode {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for ObisCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::str::FromStr for ObisCode {
    type Err = IdentifierError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for ObisCode {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&self.0)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for ObisCode {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ObisCode;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("an OBIS code string (e.g. \"1-0:1.8.0*255\")")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<ObisCode, E> {
                ObisCode::new(v).map_err(|e| {
                    crate::identifiers::trace_identifier_deser_error("ObisCode", v, &e);
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

    // ── Valid inputs ──────────────────────────────────────────────────────────

    #[test]
    fn c_dot_d_only() {
        let c = ObisCode::new("1.8").unwrap();
        let p = c.components();
        assert_eq!(
            (p.a, p.b, p.c, p.d, p.e, p.f),
            (None, None, 1, 8, None, None)
        );
    }

    #[test]
    fn c_dot_d_dot_e() {
        let c = ObisCode::new("1.8.1").unwrap();
        let p = c.components();
        assert_eq!(
            (p.a, p.b, p.c, p.d, p.e, p.f),
            (None, None, 1, 8, Some(1), None)
        );
    }

    #[test]
    fn a_b_colon_c_dot_d_dot_e() {
        let c = ObisCode::new("1-0:1.8.1").unwrap();
        let p = c.components();
        assert_eq!(
            (p.a, p.b, p.c, p.d, p.e, p.f),
            (Some(1), Some(0), 1, 8, Some(1), None)
        );
    }

    #[test]
    fn with_f_component_star() {
        let c = ObisCode::new("1-0:1.8.0*255").unwrap();
        let p = c.components();
        assert_eq!(
            (p.a, p.b, p.c, p.d, p.e, p.f),
            (Some(1), Some(0), 1, 8, Some(0), Some(255))
        );
    }

    #[test]
    fn with_f_component_ampersand() {
        let c = ObisCode::new("1-0:1.8.0&255").unwrap();
        let p = c.components();
        assert_eq!(p.f, Some(255));
    }

    #[test]
    fn a_colon_without_b() {
        // Some implementations omit B: "1:1.8.1"
        let c = ObisCode::new("1:1.8.1").unwrap();
        let p = c.components();
        assert_eq!((p.a, p.b, p.c, p.d, p.e), (Some(1), None, 1, 8, Some(1)));
    }

    #[test]
    fn zero_c_component_is_invalid() {
        // H-05 fix: C=0 is reserved / not a valid BDEW physical quantity code.
        assert!(
            matches!(
                ObisCode::new("0-0:0.0.0*0").unwrap_err(),
                IdentifierError::InvalidFormat { .. }
            ),
            "C=0 must be rejected"
        );
        // But A=0 and B=0 with C>=1 are valid (zero-values for channel/energy-type are allowed).
        let c = ObisCode::new("0-0:1.0.0*0").unwrap();
        let p = c.components();
        assert_eq!((p.a, p.b, p.c, p.d, p.f), (Some(0), Some(0), 1, 0, Some(0)));
    }

    #[test]
    fn display_preserves_input() {
        let input = "1-0:1.8.1";
        assert_eq!(ObisCode::new(input).unwrap().to_string(), input);
    }

    #[test]
    fn round_trip() {
        let s = "1-0:1.8.0*255";
        let c = s.parse::<ObisCode>().unwrap();
        assert_eq!(c.to_string(), s);
    }

    // ── Invalid inputs ────────────────────────────────────────────────────────

    #[test]
    fn empty_string_fails() {
        assert!(matches!(
            ObisCode::new("").unwrap_err(),
            IdentifierError::InvalidFormat { .. }
        ));
    }

    #[test]
    fn missing_d_component_fails() {
        assert!(matches!(
            ObisCode::new("1.").unwrap_err(),
            IdentifierError::InvalidFormat { .. }
        ));
    }

    #[test]
    fn missing_c_component_fails() {
        assert!(matches!(
            ObisCode::new(".8").unwrap_err(),
            IdentifierError::InvalidFormat { .. }
        ));
    }

    #[test]
    fn trailing_garbage_fails() {
        assert!(matches!(
            ObisCode::new("1.8.1.2").unwrap_err(),
            IdentifierError::InvalidFormat { .. }
        ));
    }

    #[test]
    fn non_numeric_c_fails() {
        assert!(matches!(
            ObisCode::new("A.8").unwrap_err(),
            IdentifierError::InvalidFormat { .. }
        ));
    }

    #[test]
    fn non_numeric_f_fails() {
        assert!(matches!(
            ObisCode::new("1.8*abc").unwrap_err(),
            IdentifierError::InvalidFormat { .. }
        ));
    }
}
