use crate::error::IdentifierError;

// ─── Grammar ──────────────────────────────────────────────────────────────────
//
// OBIS ID (IEC 62056-61 / BDEW):
//   [A-B:]C.D[.E][*F]
//   [A:]C.D[.E][*F]   (A-only prefix: A present, B absent)
//
// Components:
//   A  – value group A (energy type), optional (default 1)
//   B  – value group B (channel), optional (default 0)
//   C  – value group C (physical quantity), mandatory; C=0 = general metering group
//   D  – value group D (measurement type), mandatory
//   E  – value group E (tariff), optional
//   F  – value group F (billing period), optional (separator '*' or '&')
//
// Every value group is a single octet (0–255) per IEC 62056-61 §4; a group that
// does not fit in a byte is not an OBIS value group. Leading zeros are accepted
// on input and canonicalised away.
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
///
/// This struct is `#[non_exhaustive]` — new fields may be added in future
/// minor versions without a major-version bump.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct ObisComponents {
    /// Value group A (energy type). `None` means the full `A-B:` prefix was omitted.
    ///
    /// Common values: `1` electricity, `6` heat, `7` gas, `8` water.
    pub a: Option<u8>,
    /// Value group B (channel).
    /// - `Some(b)` when both A and B appear in the `A-B:` prefix.
    /// - `None` when only the `A:` prefix was given (A present, B absent).
    /// - `None` when the entire `A-B:` / `A:` prefix was omitted.
    pub b: Option<u8>,
    /// Value group C (physical quantity) – mandatory.
    ///
    /// `C = 0` identifies the general metering data group per IEC 62056-21 §5.4
    /// and IEC 62056-61 §4.2 (status, date/time, administrative objects).
    pub c: u8,
    /// Value group D (measurement type) – mandatory.
    pub d: u8,
    /// Value group E (tariff) – optional.
    pub e: Option<u8>,
    /// Value group F (billing period) – optional.
    ///
    /// `255` conventionally marks "not used".
    pub f: Option<u8>,
}

// ─── Parser helpers ───────────────────────────────────────────────────────────

/// Parses one OBIS value group from the beginning of `s`, returning `(value, rest)`.
///
/// Returns `None` if `s` does not start with a digit, or if the digits do not
/// denote a single octet — value groups are one byte wide (IEC 62056-61 §4), so
/// `256` and up are not OBIS values regardless of how many digits are written.
/// Leading zeros are accepted: `007` and `7` are the same value group.
fn parse_group(s: &str) -> Option<(u8, &str)> {
    let end = s.find(|c: char| !c.is_ascii_digit()).unwrap_or(s.len());
    if end == 0 {
        return None;
    }
    let n = s[..end].parse::<u8>().ok()?;
    Some((n, &s[end..]))
}

/// Parses a complete value group, i.e. `parse_group` that must consume all of `s`.
fn parse_whole_group(s: &str) -> Option<u8> {
    match parse_group(s) {
        Some((n, "")) => Some(n),
        _ => None,
    }
}

fn validate_and_parse(s: &str) -> Result<ObisComponents, IdentifierError> {
    if s.is_empty() {
        return Err(IdentifierError::InvalidFormat {
            description: "OBIS code must not be empty".into(),
        });
    }

    /// Every value group failure reads the same way, so build the message once.
    fn bad_group(group: char) -> IdentifierError {
        IdentifierError::InvalidFormat {
            description: format!(
                "{group} component must be a single octet (0-255) per IEC 62056-61"
            )
            .into(),
        }
    }

    // ── Split off optional F component (*F or &F) ────────────────────────────
    let (s, f) = if let Some(idx) = s.rfind(['*', '&']) {
        let f_val = parse_whole_group(&s[idx + 1..]).ok_or_else(|| bad_group('F'))?;
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
            let a = parse_whole_group(&prefix[..dash_pos]).ok_or_else(|| bad_group('A'))?;
            let b = parse_whole_group(&prefix[dash_pos + 1..]).ok_or_else(|| bad_group('B'))?;
            (rest, Some(a), Some(b))
        } else {
            let a = parse_whole_group(prefix).ok_or_else(|| bad_group('A'))?;
            (rest, Some(a), None)
        }
    } else {
        (s, None, None)
    };

    // ── Parse mandatory C.D[.E] ──────────────────────────────────────────────
    let (c, rest) = parse_group(s).ok_or_else(|| bad_group('C'))?;

    if !rest.starts_with('.') {
        return Err(IdentifierError::InvalidFormat {
            description: "expected '.' separator between C and D".into(),
        });
    }
    let rest = &rest[1..];

    let (d, rest) = parse_group(rest).ok_or_else(|| bad_group('D'))?;

    let (e, rest_after_e) = if let Some(after_dot) = rest.strip_prefix('.') {
        let (e_val, remainder) = parse_group(after_dot).ok_or_else(|| bad_group('E'))?;
        (Some(e_val), remainder)
    } else {
        (None, rest)
    };

    if !rest_after_e.is_empty() {
        return Err(IdentifierError::InvalidFormat {
            description: "unexpected trailing characters after OBIS code".into(),
        });
    }

    Ok(ObisComponents { a, b, c, d, e, f })
}

impl ObisComponents {
    /// Renders these components in the canonical BO4E form `[A-B:]C.D[.E][*F]`.
    ///
    /// This is the single renderer for the type: it defines the canonical string
    /// stored by [`ObisCode`], so a stored value and its re-rendered components can
    /// never disagree.
    fn render(&self, include_f: bool) -> String {
        use std::fmt::Write as _;

        // Longest realistic form is well under 32 bytes; one allocation, no reallocs.
        let mut out = String::with_capacity(32);
        match (self.a, self.b) {
            (Some(a), Some(b)) => {
                let _ = write!(out, "{a}-{b}:");
            }
            (Some(a), None) => {
                let _ = write!(out, "{a}:");
            }
            (None, _) => {}
        }
        let _ = write!(out, "{}.{}", self.c, self.d);
        if let Some(e) = self.e {
            let _ = write!(out, ".{e}");
        }
        if include_f {
            if let Some(f) = self.f {
                let _ = write!(out, "*{f}");
            }
        }
        out
    }
}

// ─── Type ────────────────────────────────────────────────────────────────────

/// OBIS identifier (IEC 62056-61 / BDEW): compact reference for metering values.
///
/// Grammar: `[A-B:]C.D[.E][*F]`
///
/// All numeric components are non-negative integers.  `C` and `D` are mandatory;
/// `A`, `B`, `E`, and `F` are optional.  `C = 0` is permitted and identifies the
/// general metering data group (IEC 62056-21 / IEC 62056-61).
///
/// # Canonicalisation
///
/// The input is parsed once at construction and stored in **canonical form**, so
/// two codes that denote the same value are always equal and hash alike:
///
/// - the alternative `&` separator for the F component becomes `*`;
/// - redundant leading zeros are dropped (`01.08` → `1.8`).
///
/// [`AsRef<str>`], [`Display`](std::fmt::Display), and the `serde` output all
/// yield this canonical string, so a round-trip through JSON is stable.
///
/// Use [`ObisCode::components`] to access the parsed values — they are stored, so
/// the accessor neither re-parses nor allocates.
/// Use [`ObisCode::to_pia_string`] to emit the `A-B:C.D[.E]` form (F stripped).
///
/// # Examples
/// ```
/// use rubo4e::identifiers::ObisCode;
///
/// ObisCode::new("1-0:1.8.1").unwrap();          // A-B:C.D.E
/// ObisCode::new("1-0:1.8.0*255").unwrap();      // with F component
/// ObisCode::new("0-0:0.0.0").unwrap();          // C=0 (general metering group)
/// ObisCode::new("1:1.8").unwrap();              // A-only prefix (B absent)
/// ObisCode::new("1.8.1").unwrap();              // C.D.E only
///
/// // `&` is accepted on input and canonicalised to `*`.
/// assert_eq!(ObisCode::new("1.8.1&255").unwrap(), ObisCode::new("1.8.1*255").unwrap());
/// // Leading zeros are canonicalised away.
/// assert_eq!(ObisCode::new("01-00:01.08.00").unwrap(), ObisCode::new("1-0:1.8.0").unwrap());
/// assert_eq!(ObisCode::new("01-00:01.08.00").unwrap().as_ref(), "1-0:1.8.0");
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "validate", derive(garde::Validate))]
#[cfg_attr(feature = "validate", garde(allow_unvalidated))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(
    feature = "schemars",
    schemars(schema_with = "crate::schema_helpers::obis_code_schema")
)]
pub struct ObisCode {
    /// Canonical rendering — the only value ever exposed as a string.
    #[cfg_attr(feature = "validate", garde(custom(check_obis_code)))]
    canonical: Box<str>,
    /// Parsed once at construction so `components()` is infallible and free.
    #[cfg_attr(feature = "validate", garde(skip))]
    components: ObisComponents,
}

// `Eq`, `Ord`, and `Hash` read the canonical string and nothing else.
//
// Deriving them would fold `components` in too, breaking the `Borrow<str>`
// contract every identifier here carries: `Borrow` requires `x.borrow()` to hash
// and compare exactly as `x` does, and a two-field hash makes
// `HashMap<ObisCode, _>::get("1-0:1.8.0")` miss a key that is present.
// `components` is a pure function of `canonical`, so nothing is lost.
impl PartialEq for ObisCode {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.canonical == other.canonical
    }
}

impl Eq for ObisCode {}

impl std::hash::Hash for ObisCode {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.canonical.hash(state);
    }
}

impl PartialOrd for ObisCode {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Lexicographic on the canonical string — a total order for `BTreeMap` keys and
/// `sort()`, not a numeric ordering of the value groups (`"10.1"` sorts before
/// `"2.1"`). Compare [`components`](ObisCode::components) where the numbers matter.
impl Ord for ObisCode {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.canonical.cmp(&other.canonical)
    }
}

#[cfg(feature = "validate")]
fn check_obis_code(value: &str, _: &()) -> Result<(), garde::Error> {
    validate_and_parse(value)
        .map(|_| ())
        .map_err(garde::Error::from)
}

impl ObisCode {
    /// Creates a new `ObisCode` after full structural validation, storing the
    /// value in canonical form.
    ///
    /// # Errors
    /// Returns [`IdentifierError::InvalidFormat`] if the input does not conform
    /// to the OBIS grammar.
    #[must_use = "the validated identifier is returned; ignoring it discards the result"]
    pub fn new(s: &str) -> Result<Self, IdentifierError> {
        let components = validate_and_parse(s)?;
        Ok(Self {
            canonical: components.render(true).into_boxed_str(),
            components,
        })
    }

    /// Returns the individual OBIS value groups.
    ///
    /// Parsed once at construction, so this neither re-parses nor allocates.
    #[must_use]
    pub fn components(&self) -> ObisComponents {
        self.components
    }

    /// Returns the canonical `[A-B:]C.D[.E][*F]` form — the same string as
    /// [`AsRef<str>`] and [`Display`](std::fmt::Display).
    ///
    /// # Examples
    /// ```
    /// use rubo4e::identifiers::ObisCode;
    ///
    /// assert_eq!(ObisCode::new("1-0:1.8.0*255").unwrap().as_str(), "1-0:1.8.0*255");
    /// assert_eq!(ObisCode::new("01-00:01.08").unwrap().as_str(),   "1-0:1.8");
    /// ```
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.canonical
    }

    /// Returns the `A-B:C.D[.E]` form of this OBIS code, without the F component.
    ///
    /// Useful for emitting the item-number composite in a PIA segment, where F
    /// is not included.
    ///
    /// # Examples
    /// ```
    /// use rubo4e::identifiers::ObisCode;
    ///
    /// assert_eq!(ObisCode::new("1-0:1.8.0").unwrap().to_pia_string(),     "1-0:1.8.0");
    /// assert_eq!(ObisCode::new("1-0:1.8.0*255").unwrap().to_pia_string(), "1-0:1.8.0");
    /// ```
    #[must_use]
    pub fn to_pia_string(&self) -> String {
        self.components.render(false)
    }
}

impl_identifier_traits!(
    ObisCode,
    "an OBIS code string (e.g. \"1-0:1.8.0*255\")",
    field = canonical
);

// `utoipa`'s `value_type = String` shortcut only applies to newtype structs, and
// `ObisCode` carries its parsed components alongside the string.  Writing the
// schema out by hand also lets it carry the grammar, which the derive could not.
#[cfg(feature = "utoipa")]
impl utoipa::PartialSchema for ObisCode {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        utoipa::openapi::ObjectBuilder::new()
            .schema_type(utoipa::openapi::schema::Type::String)
            .pattern(Some(OBIS_PATTERN))
            .description(Some(
                "OBIS-Kennzahl nach IEC 62056-61: [A-B:]C.D[.E][*F]. \
                 Wird kanonisiert gespeichert (führende Nullen entfallen, '&' wird zu '*').",
            ))
            // Not `serde_json::json!` — the `utoipa` feature does not enable
            // `serde_json` for this crate, and `&str` already converts.
            .examples(["1-0:1.8.0*255"])
            .into()
    }
}

#[cfg(feature = "utoipa")]
impl utoipa::ToSchema for ObisCode {
    fn name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("ObisCode")
    }
}

/// Regex for the OBIS grammar, shared by the `schemars` and `utoipa` schemas so
/// the two can never describe different grammars.
///
/// This describes what deserialization **accepts**, not only what serialization
/// emits: both F-component separators are allowed, and leading zeros are
/// permitted on input even though they are canonicalised away on the way in.
#[cfg(any(feature = "schemars", feature = "utoipa"))]
pub(crate) const OBIS_PATTERN: &str = r"^(?:\d+(?:-\d+)?:)?\d+\.\d+(?:\.\d+)?(?:[*&]\d+)?$";

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
    fn c_zero_is_valid() {
        // C=0 identifies the general metering data group per IEC 62056-21 §5.4 /
        // IEC 62056-61 §4.2 (status, date/time, administrative objects).
        let c = ObisCode::new("0-0:0.0.0*0").unwrap();
        let p = c.components();
        assert_eq!(
            (p.a, p.b, p.c, p.d, p.e, p.f),
            (Some(0), Some(0), 0, 0, Some(0), Some(0))
        );
        // A=0, B=0 with C=1 are also valid.
        let c2 = ObisCode::new("0-0:1.0.0*0").unwrap();
        let p2 = c2.components();
        assert_eq!(
            (p2.a, p2.b, p2.c, p2.d, p2.f),
            (Some(0), Some(0), 1, 0, Some(0))
        );
    }

    #[test]
    fn display_preserves_input() {
        let input = "1-0:1.8.1";
        assert_eq!(ObisCode::new(input).unwrap().to_string(), input);
    }

    // ── Rendering ─────────────────────────────────────────────────────────────

    #[test]
    fn to_pia_drops_f() {
        let c = ObisCode::new("1-0:1.8.0*255").unwrap();
        assert_eq!(c.to_pia_string(), "1-0:1.8.0");
    }

    #[test]
    fn canonical_form_preserves_f() {
        let c = ObisCode::new("1-0:1.8.0*255").unwrap();
        assert_eq!(c.as_str(), "1-0:1.8.0*255");
    }

    #[test]
    fn pia_and_canonical_agree_when_there_is_no_f() {
        let s = "1-0:1.8.1";
        let c = ObisCode::new(s).unwrap();
        assert_eq!(c.to_pia_string(), s);
        assert_eq!(c.as_str(), s);
    }

    /// `as_str`, `as_ref`, `Display`, and `Deref` must all agree — they are the
    /// same canonical string, not independently rendered views.
    #[test]
    fn every_string_view_agrees() {
        let c = ObisCode::new("01-00:01.08.00&255").unwrap();
        assert_eq!(c.as_str(), "1-0:1.8.0*255");
        assert_eq!(c.as_ref(), c.as_str());
        assert_eq!(c.to_string(), c.as_str());
        assert_eq!(&*c, c.as_str());
    }

    #[test]
    fn round_trip() {
        let s = "1-0:1.8.0*255";
        let c = s.parse::<ObisCode>().unwrap();
        assert_eq!(c.to_string(), s);
    }

    // ── Canonicalisation ─────────────────────────────────────────────────────

    /// Codes that denote the same value must be equal and hash alike, whatever
    /// spelling they arrived in.
    #[test]
    fn equal_values_are_equal_regardless_of_spelling() {
        use std::collections::HashSet;

        for (a, b) in [
            ("1.8.1&255", "1.8.1*255"),          // separator
            ("01-00:01.08.00", "1-0:1.8.0"),     // leading zeros
            ("0001.0008", "1.8"),                // leading zeros, no prefix
            ("01:01.08.01*0255", "1:1.8.1*255"), // both, A-only prefix
        ] {
            let (x, y) = (ObisCode::new(a).unwrap(), ObisCode::new(b).unwrap());
            assert_eq!(x, y, "{a} vs {b}");
            assert_eq!(x.as_str(), y.as_str(), "{a} vs {b}");
            let set: HashSet<_> = [x, y].into_iter().collect();
            assert_eq!(set.len(), 1, "{a} and {b} must hash alike");
        }
    }

    /// Canonicalisation must be idempotent: re-parsing a canonical string is a
    /// no-op, so serialize → deserialize can never drift.
    #[test]
    fn canonicalisation_is_idempotent() {
        for input in [
            "1-0:1.8.0*255",
            "01-00:01.08.00&0255",
            "1:1.8",
            "0.0",
            "1.8.1",
        ] {
            let once = ObisCode::new(input).unwrap();
            let twice = ObisCode::new(once.as_str()).unwrap();
            assert_eq!(once, twice, "{input}");
            assert_eq!(once.as_str(), twice.as_str(), "{input}");
        }
    }

    /// `components()` must describe exactly the canonical string, so the two can
    /// never disagree about what the value is.
    #[test]
    fn components_and_canonical_string_agree() {
        for input in ["1-0:1.8.0*255", "01:01.08", "0000.0000", "1.8.1"] {
            let code = ObisCode::new(input).unwrap();
            let reparsed = ObisCode::new(code.as_str()).unwrap();
            assert_eq!(code.components(), reparsed.components(), "{input}");
        }
    }

    /// The accessor is a copy of stored data — repeated calls are stable and
    /// cannot panic on any constructable value.
    #[test]
    fn components_is_a_stable_accessor() {
        let c = ObisCode::new("1-0:1.8.0*255").unwrap();
        assert_eq!(c.components(), c.components());
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

    // ── Octet range (IEC 62056-61 §4) ────────────────────────────────────────

    /// Every value group is one byte, so 255 is the largest legal value and 256
    /// is not an OBIS value group at all.
    #[test]
    fn value_groups_are_octets() {
        // 255 is the documented maximum and must be accepted in every position.
        let max = ObisCode::new("255-255:255.255.255*255").unwrap();
        let p = max.components();
        assert_eq!(
            (p.a, p.b, p.c, p.d, p.e, p.f),
            (Some(255), Some(255), 255, 255, Some(255), Some(255))
        );

        // 256 overflows the octet in each position.
        for over in [
            "256-0:1.8.0",
            "1-256:1.8.0",
            "1-0:256.8.0",
            "1-0:1.256.0",
            "1-0:1.8.256",
            "1-0:1.8.0*256",
        ] {
            assert!(
                matches!(
                    ObisCode::new(over),
                    Err(IdentifierError::InvalidFormat { .. })
                ),
                "{over} must be rejected: OBIS value groups are single octets"
            );
        }
    }

    /// The out-of-range message must name the offending group, not just say
    /// "invalid" — otherwise a bad F reads the same as a bad A.
    #[test]
    fn octet_overflow_names_the_group() {
        for (input, group) in [
            ("256-0:1.8", 'A'),
            ("1-256:1.8", 'B'),
            ("300.8", 'C'),
            ("1.300", 'D'),
            ("1.8.300", 'E'),
            ("1.8*300", 'F'),
        ] {
            let msg = ObisCode::new(input).unwrap_err().to_string();
            assert!(
                msg.contains(group),
                "error for {input:?} should name group {group}: {msg}"
            );
        }
    }

    // ── Trait contracts ──────────────────────────────────────────────────────

    /// `Borrow<str>` promises that a borrowed key hashes and compares exactly as
    /// the owned one does — otherwise a `HashMap` lookup by `&str` misses keys
    /// that are present, with no error to show for it.
    #[test]
    fn borrowing_as_str_finds_the_same_entry() {
        use std::collections::HashMap;

        let mut by_code: HashMap<ObisCode, u32> = HashMap::new();
        by_code.insert(ObisCode::new("1-0:1.8.0").unwrap(), 7);

        assert_eq!(by_code.get("1-0:1.8.0"), Some(&7));
        // …and the canonical spelling is the one to look up with.
        assert_eq!(by_code.get("01-00:01.08.00"), None);
    }

    /// Every other identifier is `Ord`, and the module docs promise it for all of
    /// them; a `BTreeMap` keyed by OBIS code must compile and order stably.
    #[test]
    fn codes_order_totally_by_canonical_string() {
        use std::collections::BTreeMap;

        let mut m: BTreeMap<ObisCode, u32> = BTreeMap::new();
        for s in ["1-0:2.8.0", "1-0:1.8.0", "1-0:1.8.1"] {
            m.insert(ObisCode::new(s).unwrap(), 0);
        }
        assert_eq!(
            m.keys().map(ObisCode::as_str).collect::<Vec<_>>(),
            ["1-0:1.8.0", "1-0:1.8.1", "1-0:2.8.0"]
        );
    }

    /// Leading zeros are an accepted spelling of an in-range group, not an
    /// overflow — `0255` is 255, and `00001` is 1.
    #[test]
    fn leading_zeros_do_not_overflow_the_octet() {
        assert_eq!(
            ObisCode::new("0001-0000:0001.0008.0000*0255")
                .unwrap()
                .as_str(),
            "1-0:1.8.0*255"
        );
    }
}
