use crate::error::{IdentifierError, LengthExpectation};

// ─── Character encoding ───────────────────────────────────────────────────────

/// Maps an EIC character to its numeric value for the check-character calculation.
///
/// Mapping (per ENTSO-E Reference Manual v5.5):
/// - `'0'`–`'9'` → 0–9
/// - `'A'`–`'Z'` → 10–35
/// - `'-'`       → 36  (used as right-padding)
fn char_value(c: char) -> Option<u32> {
    match c {
        '0'..='9' => Some(c as u32 - '0' as u32),
        'A'..='Z' => Some(c as u32 - 'A' as u32 + 10),
        '-' => Some(36),
        _ => None,
    }
}

/// Maps a numeric value (0–36) back to an EIC character.
fn value_to_char(v: u32) -> Option<char> {
    match v {
        0..=9 => char::from_digit(v, 10),
        10..=35 => Some((b'A' + (v - 10) as u8) as char),
        36 => Some('-'),
        _ => None,
    }
}

// ─── Check-character computation ─────────────────────────────────────────────

/// Computes the EIC check character for a 15-byte ASCII prefix.
///
/// Algorithm per ENTSO-E EIC Code Implementation Guide (§7.1) and
/// confirmed by the BO4E dotnet reference implementation:
/// 1. Assign numeric values to each of the 15 prefix characters.
/// 2. Multiply each value by its position weight: position 0 → weight 16,
///    position 1 → weight 15, …, position 14 → weight 2.
/// 3. `check_number = 36 − (Σ products − 1) mod 37`.
/// 4. Map `check_number` back to the corresponding EIC character.
///    Returns `None` if `check_number` would be 36 (i.e. the character
///    would be `'-'`, which ENTSO-E prohibits as a check character).
///
/// # Safety
/// `prefix_bytes` must contain only valid EIC alphabet bytes (`[A-Z0-9-]`);
/// invalid bytes contribute 0 to the sum (safe but may yield a wrong check char).
pub(crate) fn compute_check_char(prefix_bytes: &[u8; 15]) -> Option<char> {
    let sum: u32 = prefix_bytes
        .iter()
        .enumerate()
        // SAFETY: all bytes are validated ASCII EIC chars before this point;
        // casting u8 → char is sound for ASCII (code points 0x00–0x7F).
        .map(|(i, &b)| char_value(b as char).unwrap_or(0) * (16 - i as u32))
        .sum();
    // Guard against underflow: sum must be ≥ 1 for valid prefixes.
    if sum == 0 {
        return None;
    }
    let check_number = 36 - (sum - 1) % 37;
    // '-' (value 36) is not a valid check character per ENTSO-E spec.
    if check_number == 36 {
        return None;
    }
    value_to_char(check_number)
}

// ─── Validation ──────────────────────────────────────────────────────────────

/// Valid EIC type characters (position 3) per ENTSO-E EIC definitions.
const EIC_TYPE_CHARS: &[char] = &['A', 'T', 'V', 'W', 'X', 'Y', 'Z'];

fn validate(s: &str) -> Result<(), IdentifierError> {
    // EIC codes are ASCII-only; reject multi-byte UTF-8 early.
    if !s.is_ascii() {
        return Err(IdentifierError::InvalidFormat {
            description: "EIC code must contain only ASCII characters".into(),
        });
    }

    if s.len() != 16 {
        return Err(IdentifierError::InvalidLength {
            expected: LengthExpectation::Exact(16),
            actual: s.len(),
        });
    }

    // Validate all characters are in the EIC alphabet [A-Z, 0-9, '-'].
    for (i, c) in s.chars().enumerate() {
        if char_value(c).is_none() {
            return Err(IdentifierError::InvalidCharacter {
                position: i,
                character: c,
            });
        }
    }

    // Position 3 (index 2) must be a valid EIC type character.
    let eic_type = s.as_bytes()[2] as char;
    if !EIC_TYPE_CHARS.contains(&eic_type) {
        return Err(IdentifierError::InvalidFormat {
            description: "position 3 must be a valid EIC type character (A/T/V/W/X/Y/Z)".into(),
        });
    }

    // Validate check character (position 16, index 15).
    // SAFETY: s.is_ascii() and s.len() == 16 are guaranteed by the checks above;
    // as_bytes() gives a length-16 slice of ASCII bytes with no allocation.
    let prefix: &[u8; 15] = s.as_bytes()[..15]
        .try_into()
        .expect("length is verified to be 16 above");
    let expected = compute_check_char(prefix).ok_or(IdentifierError::InvalidChecksum)?;
    // Index 15 is safe: s.len() == 16 and s.is_ascii().
    let actual = s.as_bytes()[15] as char;
    if actual != expected {
        return Err(IdentifierError::InvalidChecksum);
    }

    Ok(())
}

// ─── Domain ──────────────────────────────────────────────────────────────────

/// ENTSO-E EIC domain type, determined by position 3 (index 2) of the code.
///
/// This is a best-effort classification based on common ENTSO-E usage patterns
/// in the German and European energy market.  The EIC Registry is the authoritative
/// source; when in doubt, call [`EicCode::type_char`] to inspect the raw character.
///
/// Position-3 mapping (per ENTSO-E EIC Reference Manual v5.5 and practical BO4E usage):
/// - `A`, `Y`, `X`, `W`, `Z` → [`EicDomain::Area`] (control areas, bidding zones,
///   metering grids, market areas)
/// - `T`, `V` → [`EicDomain::Party`] (TSOs, DSOs, market participants)
///
/// # Examples
/// ```
/// use rubo4e::identifiers::{EicCode, EicDomain};
///
/// // 10YDE-EON------1 is the E.ON Germany control area (type Y = Area).
/// let code = EicCode::new("10YDE-EON------1").expect("valid EIC area code");
/// assert_eq!(code.domain(), EicDomain::Area);
/// assert_eq!(code.type_char(), 'Y');
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EicDomain {
    /// Area-type codes: control areas, bidding zones, market areas, metering grids.
    ///
    /// Type characters: `A`, `Y`, `X`, `W`, `Z`.
    Area,
    /// Party-type codes: market participants, TSOs, DSOs, suppliers.
    ///
    /// Type characters: `T`, `V`.
    Party,
}

impl std::fmt::Display for EicDomain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EicDomain::Area => f.write_str("Area"),
            EicDomain::Party => f.write_str("Party"),
        }
    }
}

// ─── Type ────────────────────────────────────────────────────────────────────

/// Energy Identification Code (EIC): 16-character code issued by ENTSO-E.
///
/// Structure:
/// - Positions 1–2:  Local Issuing Office (LIO) identifier (alphanumeric)
/// - Position 3:     EIC type character (`A`, `T`, `V`, `W`, `X`, `Y`, or `Z`)
/// - Positions 4–15: LIO-specific code body (alphanumeric or `-` as padding)
/// - Position 16:    Check character computed by the ENTSO-E algorithm
///
/// # Domain
///
/// Use [`EicCode::domain`] to query whether this code is an Area or Party code.
/// Use [`EicCode::type_char`] to get the raw type character (position 3).
///
/// # Examples
/// ```
/// use rubo4e::identifiers::{EicCode, EicDomain};
///
/// // 10YDE-EON------1 = E.ON Germany control area (type Y = Area).
/// let area = EicCode::new("10YDE-EON------1").expect("valid area EIC");
/// assert_eq!(area.domain(), EicDomain::Area);
/// assert_eq!(area.type_char(), 'Y');
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "validate", derive(garde::Validate))]
#[cfg_attr(feature = "validate", garde(allow_unvalidated))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schemars", schemars(with = "String"))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "utoipa", schema(value_type = String))]
pub struct EicCode(#[cfg_attr(feature = "validate", garde(custom(check_eic_code)))] Box<str>);

#[cfg(feature = "validate")]
fn check_eic_code(value: &str, _: &()) -> Result<(), garde::Error> {
    validate(value).map_err(garde::Error::from)
}

impl EicCode {
    /// Creates a new `EicCode` after full structural and checksum validation.
    ///
    /// # Errors
    /// - [`IdentifierError::InvalidLength`] if `s` is not exactly 16 characters.
    /// - [`IdentifierError::InvalidCharacter`] if any character is outside `[A-Z0-9-]`.
    /// - [`IdentifierError::InvalidFormat`] if position 3 is not a valid EIC type character.
    /// - [`IdentifierError::InvalidChecksum`] if position 16 does not match the computed check character.
    #[must_use = "the validated identifier is returned; ignoring it discards the result"]
    pub fn new(s: &str) -> Result<Self, IdentifierError> {
        validate(s)?;
        Ok(Self(Box::from(s)))
    }

    /// Returns the EIC domain (Area / Party) for this code.
    ///
    /// This is a heuristic based on position 3 (index 2) of the code.
    /// See [`EicDomain`] for the exact character-to-domain mapping used.
    ///
    /// # Panics
    ///
    /// Never panics on a validly-constructed `EicCode`.
    #[must_use]
    pub fn domain(&self) -> EicDomain {
        match self.type_char() {
            'T' | 'V' => EicDomain::Party,
            _ => EicDomain::Area, // A, W, X, Y, Z are all area types per ENTSO-E
        }
    }

    /// Returns the raw EIC type character at position 3 (index 2) of the code.
    ///
    /// Valid characters are `A`, `T`, `V`, `W`, `X`, `Y`, `Z` per ENTSO-E spec.
    /// Use this when you need the raw type rather than the aggregated [`EicDomain`].
    #[must_use]
    pub fn type_char(&self) -> char {
        // SAFETY: validated at construction — index 2 is always a valid ASCII EIC type char.
        self.0.as_bytes()[2] as char
    }

    /// Computes the check character for a 15-character ASCII prefix string.
    ///
    /// Returns `None` if `prefix` is not exactly 15 bytes, is not ASCII,
    /// or if the computed check number maps to `'-'` (prohibited per ENTSO-E).
    ///
    /// Useful for generating valid test vectors.
    pub fn compute_check_char(prefix: &str) -> Option<char> {
        if prefix.len() != 15 || !prefix.is_ascii() {
            return None;
        }
        // SAFETY: prefix.len() == 15 and prefix.is_ascii() verified above.
        let bytes: &[u8; 15] = prefix.as_bytes().try_into().ok()?;
        compute_check_char(bytes)
    }
}

impl TryFrom<String> for EicCode {
    type Error = IdentifierError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::new(&s)
    }
}

impl TryFrom<&str> for EicCode {
    type Error = IdentifierError;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Self::new(s)
    }
}

impl AsRef<str> for EicCode {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for EicCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::str::FromStr for EicCode {
    type Err = IdentifierError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for EicCode {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&self.0)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for EicCode {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = EicCode;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a 16-character ENTSO-E Energy Identification Code")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<EicCode, E> {
                EicCode::new(v).map_err(|e| {
                    crate::identifiers::trace_identifier_deser_error("EicCode", v, &e);
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

    /// Builds a valid 16-char EIC from a 15-char prefix by appending the computed check char.
    fn make_valid_eic(prefix: &str) -> String {
        let check = EicCode::compute_check_char(prefix)
            .unwrap_or_else(|| panic!("could not compute check char for prefix: {prefix}"));
        format!("{prefix}{check}")
    }

    // ── Valid codes ───────────────────────────────────────────────────────

    #[test]
    fn constructed_code_validates() {
        // Generate valid codes from different type-char prefixes and round-trip them.
        let prefixes = [
            "10XTEST--------", // type X, padded with '-'
            "11YTEST--------", // type Y
            "10ZFOO---------", // type Z
            "11WBAR---------", // type W
            "10VBAZ---------", // type V
            "11TQUX0--------", // type T (adjusted: 11TQUX--------- would yield '-' check char)
            "10ASUB---------", // type A (Substation)
        ];
        for prefix in prefixes {
            let eic = make_valid_eic(prefix);
            assert_eq!(eic.len(), 16, "{eic} should be 16 chars");
            let code =
                EicCode::new(&eic).unwrap_or_else(|e| panic!("{eic} should be valid but: {e}"));
            // round-trip
            assert_eq!(code.to_string().parse::<EicCode>().unwrap(), code);
        }
    }

    #[test]
    fn display_equals_input() {
        let eic = make_valid_eic("10XTEST--------");
        let code = EicCode::new(&eic).unwrap();
        assert_eq!(code.to_string(), eic);
        assert_eq!(code.as_ref(), eic.as_str());
    }

    // ── Invalid codes ─────────────────────────────────────────────────────

    #[test]
    fn wrong_length_fails() {
        assert!(matches!(
            EicCode::new("10XTEST").unwrap_err(),
            IdentifierError::InvalidLength {
                expected: LengthExpectation::Exact(16),
                actual: 7
            }
        ));
    }

    #[test]
    fn too_long_fails() {
        assert!(matches!(
            EicCode::new("10XTEST-----------X").unwrap_err(),
            IdentifierError::InvalidLength {
                expected: LengthExpectation::Exact(16),
                actual: 19
            }
        ));
    }

    #[test]
    fn invalid_character_fails() {
        // '!' is not in [A-Z0-9-]
        let err = EicCode::new("10XTEST!--------").unwrap_err();
        assert!(matches!(
            err,
            IdentifierError::InvalidCharacter {
                position: 7,
                character: '!'
            }
        ));
    }

    #[test]
    fn invalid_type_char_fails() {
        // Position 3 = 'B' which is not a valid EIC type
        let invalid_type = "10BTEST---------"; // 16 chars but type='B'
                                               // Must be invalid format, not length error
        match EicCode::new(invalid_type).unwrap_err() {
            IdentifierError::InvalidFormat { .. } => {}
            other => panic!("expected InvalidFormat, got: {other}"),
        }
    }

    #[test]
    fn wrong_check_char_fails() {
        let prefix = "10XTEST--------";
        let correct = make_valid_eic(prefix);
        // Replace last char with something different
        let wrong_last = if correct.ends_with('A') { 'B' } else { 'A' };
        let wrong: String = correct[..15].to_string() + &wrong_last.to_string();
        assert!(matches!(
            EicCode::new(&wrong).unwrap_err(),
            IdentifierError::InvalidChecksum
        ));
    }

    #[test]
    fn lowercase_input_fails() {
        let err = EicCode::new("10xtest---------").unwrap_err();
        // 'x' at position 2 is invalid (lowercase not in alphabet)
        assert!(matches!(
            err,
            IdentifierError::InvalidCharacter {
                position: 2,
                character: 'x'
            }
        ));
    }

    // ── compute_check_char helper ─────────────────────────────────────────

    #[test]
    fn compute_check_char_wrong_length_returns_none() {
        assert!(EicCode::compute_check_char("TOOSHORT").is_none());
        assert!(EicCode::compute_check_char("TOOLONGPREFIXHERE").is_none());
    }

    #[test]
    fn compute_check_char_is_deterministic() {
        let prefix = "10XTEST--------";
        assert_eq!(
            EicCode::compute_check_char(prefix),
            EicCode::compute_check_char(prefix)
        );
    }

    // ── Real-world public EIC codes (ENTSO-E transparency platform) ───────

    /// German TSO control area EIC codes, publicly listed on the ENTSO-E
    /// transparency platform and hardcoded in the BO4E-dotnet reference
    /// implementation (GermanControlAreas dictionary).
    #[test]
    fn real_entso_e_german_tso_codes() {
        let codes = [
            "10YDE-EON------1", // TenneT TSO GmbH & Co. KG (area Y)
            "10YDE-RWENET---I", // Amprion GmbH (area Y)
            "10YDE-VE-------2", // 50Hertz Transmission GmbH (area Y)
            "10YDE-ENBW-----N", // TransnetBW GmbH (area Y)
        ];
        for code in codes {
            assert!(
                EicCode::new(code).is_ok(),
                "Expected {code:?} to be a valid EIC code"
            );
        }
    }

    /// German bidding zone EIC code from the ENTSO-E transparency platform
    /// (DE-LU bidding zone — 10Y1001A1001A82H).
    #[test]
    fn real_entso_e_bidding_zone_code() {
        assert!(EicCode::new("10Y1001A1001A82H").is_ok());
    }

    /// Verify the check-character values match the ENTSO-E published codes
    /// so we catch any future algorithm regression immediately.
    #[test]
    fn check_char_matches_entso_e_published_codes() {
        assert_eq!(EicCode::compute_check_char("10YDE-EON------"), Some('1'));
        assert_eq!(EicCode::compute_check_char("10YDE-RWENET---"), Some('I'));
        assert_eq!(EicCode::compute_check_char("10YDE-VE-------"), Some('2'));
        assert_eq!(EicCode::compute_check_char("10YDE-ENBW-----"), Some('N'));
        assert_eq!(EicCode::compute_check_char("10Y1001A1001A82"), Some('H'));
    }
}
