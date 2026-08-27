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
/// The `− 1` is applied *inside* the modulus as `+ 36`: the same congruence
/// class, with no intermediate that can go negative. Spelled `sum - 1` it needs
/// a `sum == 0` guard, and that guard answered `None` for an all-`'0'` prefix
/// whose check character the algorithm defines as `'0'`.
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
    let check_number = 36 - (sum + 36) % 37;
    // '-' (value 36) is not a valid check character per ENTSO-E spec.
    if check_number == 36 {
        return None;
    }
    value_to_char(check_number)
}

// ─── Validation ──────────────────────────────────────────────────────────────

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

    // Position 3 (index 2) must be a valid EIC object-type character.
    if EicType::from_char(s.as_bytes()[2] as char).is_none() {
        return Err(IdentifierError::InvalidFormat {
            description: "position 3 must be a valid EIC object-type character (A/T/V/W/X/Y/Z)"
                .into(),
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

// ─── Object type ─────────────────────────────────────────────────────────────

/// The ENTSO-E EIC **object type**, encoded in position 3 (index 2) of the code.
///
/// The seven object types are defined by the ENTSO-E EIC Reference Manual and are
/// exhaustive: an EIC whose position-3 character is not one of them is malformed,
/// so [`EicCode`] rejects it at construction and [`EicCode::eic_type`] is total.
///
/// | Char | Variant | Meaning |
/// |------|---------|---------|
/// | `A` | [`Substation`](EicType::Substation) | Substation |
/// | `T` | [`Tieline`](EicType::Tieline) | Tie line between two areas |
/// | `V` | [`Location`](EicType::Location) | Physical location |
/// | `W` | [`ResourceObject`](EicType::ResourceObject) | Resource object (generation/consumption unit) |
/// | `X` | [`Party`](EicType::Party) | Market participant — **including Bilanzkreise** |
/// | `Y` | [`Area`](EicType::Area) | Area or domain — control areas, bidding zones, Bilanzierungsgebiete |
/// | `Z` | [`MeasurementPoint`](EicType::MeasurementPoint) | Measurement point |
///
/// # German market note
///
/// BDEW issues `11X…` codes for **Bilanzkreise** (balance groups) and `11Y…` codes
/// for **Bilanzierungsgebiete** (balancing areas).  Those are [`Party`](EicType::Party)
/// and [`Area`](EicType::Area) respectively — see [`BilanzkreisId`] and
/// [`BilanzierungsgebietId`].
///
/// [`BilanzkreisId`]: crate::identifiers::BilanzkreisId
/// [`BilanzierungsgebietId`]: crate::identifiers::BilanzierungsgebietId
///
/// # Examples
/// ```
/// use rubo4e::identifiers::{EicCode, EicType};
///
/// // 10YDE-EON------1 is the TenneT German control area (type Y = Area).
/// let area = EicCode::new("10YDE-EON------1").expect("valid EIC area code");
/// assert_eq!(area.eic_type(), EicType::Area);
///
/// // 11XSUEDWESTSTRO8 is a Bilanzkreis, which is a market party (type X).
/// let party = EicCode::new("11XSUEDWESTSTRO8").expect("valid EIC party code");
/// assert_eq!(party.eic_type(), EicType::Party);
/// assert_eq!(party.eic_type().as_char(), 'X');
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EicType {
    /// `A` — Substation.
    Substation,
    /// `T` — Tie line between two areas.
    Tieline,
    /// `V` — Physical location.
    Location,
    /// `W` — Resource object (generation or consumption unit).
    ResourceObject,
    /// `X` — Market participant (party). German Bilanzkreise use this type.
    Party,
    /// `Y` — Area or domain: control areas, bidding zones, Bilanzierungsgebiete.
    Area,
    /// `Z` — Measurement point.
    MeasurementPoint,
}

impl EicType {
    /// Every EIC object type, in ENTSO-E documentation order.
    ///
    /// This is the single source of truth for which position-3 characters
    /// [`EicCode`] accepts.
    pub const ALL: [EicType; 7] = [
        EicType::Substation,
        EicType::Tieline,
        EicType::Location,
        EicType::ResourceObject,
        EicType::Party,
        EicType::Area,
        EicType::MeasurementPoint,
    ];

    /// Returns the position-3 character for this object type.
    #[must_use]
    pub const fn as_char(self) -> char {
        match self {
            EicType::Substation => 'A',
            EicType::Tieline => 'T',
            EicType::Location => 'V',
            EicType::ResourceObject => 'W',
            EicType::Party => 'X',
            EicType::Area => 'Y',
            EicType::MeasurementPoint => 'Z',
        }
    }

    /// Parses a position-3 character into an object type.
    ///
    /// Returns `None` for any character outside `A`, `T`, `V`, `W`, `X`, `Y`, `Z`.
    ///
    /// # Examples
    /// ```
    /// use rubo4e::identifiers::EicType;
    ///
    /// assert_eq!(EicType::from_char('X'), Some(EicType::Party));
    /// assert_eq!(EicType::from_char('Y'), Some(EicType::Area));
    /// assert_eq!(EicType::from_char('B'), None);
    /// // Lowercase is not part of the EIC alphabet.
    /// assert_eq!(EicType::from_char('x'), None);
    /// ```
    #[must_use]
    pub const fn from_char(c: char) -> Option<EicType> {
        match c {
            'A' => Some(EicType::Substation),
            'T' => Some(EicType::Tieline),
            'V' => Some(EicType::Location),
            'W' => Some(EicType::ResourceObject),
            'X' => Some(EicType::Party),
            'Y' => Some(EicType::Area),
            'Z' => Some(EicType::MeasurementPoint),
            _ => None,
        }
    }

    /// Returns the ENTSO-E English name of this object type.
    #[must_use]
    pub const fn description(self) -> &'static str {
        match self {
            EicType::Substation => "Substation",
            EicType::Tieline => "Tieline",
            EicType::Location => "Location",
            EicType::ResourceObject => "Resource Object",
            EicType::Party => "Party",
            EicType::Area => "Area or Domain",
            EicType::MeasurementPoint => "Measurement Point",
        }
    }
}

impl std::fmt::Display for EicType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.description())
    }
}

// ─── Type ────────────────────────────────────────────────────────────────────

/// Energy Identification Code (EIC): 16-character code issued by ENTSO-E.
///
/// Structure:
/// - Positions 1–2:  Local Issuing Office (LIO) identifier (alphanumeric)
/// - Position 3:     EIC object-type character (`A`, `T`, `V`, `W`, `X`, `Y`, or `Z`)
/// - Positions 4–15: LIO-specific code body (alphanumeric or `-` as padding)
/// - Position 16:    Check character computed by the ENTSO-E algorithm
///
/// # Object type
///
/// Use [`EicCode::eic_type`] to get the [`EicType`] encoded in position 3, or
/// [`EicCode::type_char`] for the raw character.
///
/// # Examples
/// ```
/// use rubo4e::identifiers::{EicCode, EicType};
///
/// // 10YDE-EON------1 = TenneT German control area (type Y = Area).
/// let area = EicCode::new("10YDE-EON------1").expect("valid area EIC");
/// assert_eq!(area.eic_type(), EicType::Area);
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

    /// Returns the [`EicType`] encoded in position 3 (index 2) of the code.
    ///
    /// Total: construction rejects any code whose position-3 character is not one
    /// of the seven ENTSO-E object types, so this never fails.
    ///
    /// # Examples
    /// ```
    /// use rubo4e::identifiers::{EicCode, EicType};
    ///
    /// let bk = EicCode::new("11XSUEDWESTSTRO8").unwrap();
    /// assert_eq!(bk.eic_type(), EicType::Party);
    /// ```
    #[must_use]
    pub fn eic_type(&self) -> EicType {
        EicType::from_char(self.type_char())
            .expect("EicCode invariant: position 3 is validated at construction")
    }

    /// Returns the raw EIC object-type character at position 3 (index 2).
    ///
    /// Always one of `A`, `T`, `V`, `W`, `X`, `Y`, `Z`.  Prefer
    /// [`eic_type`](EicCode::eic_type) unless you specifically need the character.
    #[must_use]
    pub fn type_char(&self) -> char {
        // Validated at construction — index 2 is always a valid ASCII EIC type char.
        self.0.as_bytes()[2] as char
    }

    /// Builds a complete 16-character `EicCode` from its 15-character prefix by
    /// computing and appending the ENTSO-E check character.
    ///
    /// # Errors
    /// - [`IdentifierError::InvalidLength`] if `prefix` is not exactly 15 characters.
    /// - [`IdentifierError::InvalidFormat`] if `prefix` is not ASCII.
    /// - [`IdentifierError::InvalidChecksum`] if the check character cannot be
    ///   computed — ENTSO-E prohibits `'-'` as a check character, so a prefix that
    ///   would produce one has no valid completion.
    /// - Any error from [`EicCode::new`] on the completed code.
    ///
    /// # Examples
    /// ```
    /// use rubo4e::identifiers::EicCode;
    ///
    /// let eic = EicCode::new_from_prefix("10YDE-EON------").unwrap();
    /// assert_eq!(eic.as_ref(), "10YDE-EON------1");
    /// ```
    pub fn new_from_prefix(prefix: &str) -> Result<Self, IdentifierError> {
        Self::new(&Self::complete_prefix(prefix)?)
    }

    /// Returns the 16-character code for a 15-character prefix, without
    /// constructing an `EicCode`.  Shared by the EIC-restricted identifier types.
    pub(super) fn complete_prefix(prefix: &str) -> Result<String, IdentifierError> {
        if prefix.len() != 15 {
            return Err(IdentifierError::InvalidLength {
                expected: LengthExpectation::Exact(15),
                actual: prefix.len(),
            });
        }
        if !prefix.is_ascii() {
            return Err(IdentifierError::InvalidFormat {
                description: "EIC prefix must contain only ASCII characters".into(),
            });
        }
        let bytes: &[u8; 15] = prefix.as_bytes().try_into().expect("length checked above");
        let check = compute_check_char(bytes).ok_or(IdentifierError::InvalidChecksum)?;
        let mut out = String::with_capacity(16);
        out.push_str(prefix);
        out.push(check);
        Ok(out)
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

impl_identifier_traits!(EicCode, "a 16-character ENTSO-E Energy Identification Code");

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

    /// An all-`'0'` prefix sums to zero, and the algorithm's check character for
    /// it is `'0'` — `36 - (0 - 1 mod 37) = 36 - 36 = 0`. Spelling the `- 1`
    /// outside the modulus needs an underflow guard, and that guard used to
    /// answer `None` here.
    #[test]
    fn a_zero_sum_prefix_has_a_check_character() {
        assert_eq!(EicCode::compute_check_char("000000000000000"), Some('0'));
    }

    /// A prefix whose weighted sum is ≡ 1 (mod 37) would need check number 36,
    /// which maps to `'-'` — prohibited as a check character — so it has no
    /// valid completion at all.
    ///
    /// The weights run 16, 15, … 2, so a `'8'` at position 13 and a `'7'` at
    /// position 14 sum to `3·8 + 2·7 = 38 ≡ 1 (mod 37)`.
    #[test]
    fn a_prefix_needing_a_dash_has_no_completion() {
        let prefix = "000000000000087";
        assert_eq!(prefix.len(), 15);
        assert_eq!(EicCode::compute_check_char(prefix), None);
        assert!(matches!(
            EicCode::new_from_prefix(prefix),
            Err(IdentifierError::InvalidChecksum)
        ));
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

    // ── Object type (position 3) ──────────────────────────────────────────

    /// The seven ENTSO-E object types, pinned against the reference manual.
    ///
    /// `X` is the **party** type (market participants, and every German
    /// Bilanzkreis); `T`/`V` are Tieline/Location.
    #[test]
    fn eic_type_char_mapping_matches_entso_e() {
        for (c, want, desc) in [
            ('A', EicType::Substation, "Substation"),
            ('T', EicType::Tieline, "Tieline"),
            ('V', EicType::Location, "Location"),
            ('W', EicType::ResourceObject, "Resource Object"),
            ('X', EicType::Party, "Party"),
            ('Y', EicType::Area, "Area or Domain"),
            ('Z', EicType::MeasurementPoint, "Measurement Point"),
        ] {
            assert_eq!(EicType::from_char(c), Some(want), "from_char({c:?})");
            assert_eq!(want.as_char(), c, "as_char() for {want:?}");
            assert_eq!(want.description(), desc);
            assert_eq!(want.to_string(), desc);
        }
    }

    /// `ALL` must be exactly the set of characters `validate` accepts — the two
    /// are the same source of truth, so they cannot drift apart.
    #[test]
    fn eic_type_all_round_trips_and_is_exhaustive() {
        assert_eq!(EicType::ALL.len(), 7);
        for t in EicType::ALL {
            assert_eq!(EicType::from_char(t.as_char()), Some(t));
        }
        // Every other ASCII uppercase letter must be rejected.
        for c in 'A'..='Z' {
            let accepted = EicType::ALL.iter().any(|t| t.as_char() == c);
            assert_eq!(
                EicType::from_char(c).is_some(),
                accepted,
                "from_char({c:?}) disagrees with ALL"
            );
        }
    }

    /// Real published codes, classified by object type.
    ///
    /// `11X…` codes are BDEW-issued Bilanzkreise — market parties, not areas.
    #[test]
    fn real_codes_classify_correctly() {
        for (code, want) in [
            ("10YDE-EON------1", EicType::Area),
            ("10YDE-RWENET---I", EicType::Area),
            ("10YDE-VE-------2", EicType::Area),
            ("10YDE-ENBW-----N", EicType::Area),
            ("10Y1001A1001A82H", EicType::Area),
            // BDEW-issued Bilanzkreise (balance groups) — party codes.
            ("11XSUEDWESTSTRO8", EicType::Party),
            ("11XENERGIE2----H", EicType::Party),
            ("11XENAGISME----J", EicType::Party),
        ] {
            let eic = EicCode::new(code).unwrap_or_else(|e| panic!("{code} should be valid: {e}"));
            assert_eq!(eic.eic_type(), want, "{code}");
            assert_eq!(eic.type_char(), want.as_char(), "{code}");
        }
    }

    // ── Shared trait surface ──────────────────────────────────────────────

    /// `EicCode` must expose the same conversions as every other identifier.
    #[test]
    fn shares_the_common_identifier_trait_surface() {
        use std::borrow::Borrow;

        let eic = EicCode::new("11XSUEDWESTSTRO8").unwrap();
        // Deref<Target = str>
        assert!(eic.starts_with("11X"));
        assert_eq!(eic.len(), 16);
        // Borrow<str>
        let borrowed: &str = eic.borrow();
        assert_eq!(borrowed, "11XSUEDWESTSTRO8");
        // Into<String>
        assert_eq!(String::from(eic.clone()), "11XSUEDWESTSTRO8");
        // TryFrom<String>
        assert_eq!(EicCode::try_from("11XSUEDWESTSTRO8".to_string()), Ok(eic));
    }
}
