//! SEPA bank identifiers: [`Iban`] and [`Bic`].
//!
//! Both validate at construction, so a value that exists has passed its check —
//! the same contract as [`MaloId`](crate::identifiers::MaloId).
//!
//! They are *not* the types of `Zahlungsinformation.iban` / `.bic`, which stay
//! `String`; see [`Zahlungsinformation::iban_checked`].
//!
//! [`Zahlungsinformation::iban_checked`]: crate::current::Zahlungsinformation::iban_checked

use super::char_at;
use crate::error::{IdentifierError, LengthExpectation};

// ─── IBAN ────────────────────────────────────────────────────────────────────

/// Maximum IBAN length permitted by ISO 13616.
///
/// No country reaches it: the longest registered format is Russia's 33, then
/// Saint Lucia's 32. 34 is the standard's own ceiling and the width every SEPA
/// field is sized for, so it is the bound to check against rather than
/// whichever country happens to be longest this year.
pub const IBAN_MAX_LEN: usize = 34;

/// Shortest IBAN in the ISO 13616 registry (Norway).
pub const IBAN_MIN_LEN: usize = 15;

/// An International Bank Account Number (ISO 13616), checksum-verified.
///
/// # Format
///
/// Two letters of ISO 3166-1 country code, two check digits, then up to 30
/// alphanumeric characters of country-specific BBAN. A German IBAN is 22
/// characters: `DE` + 2 check digits + 8-digit Bankleitzahl + 10-digit Kontonummer.
///
/// # What is verified
///
/// - **Length**: within ISO 13616's 15–34 envelope, and exactly the registered
///   length for country codes the built-in registry table pins. A country it does
///   not list is left to the checksum, so a stale table cannot refuse a valid
///   IBAN — the registry adds countries, and this crate ships on its own
///   schedule.
/// - **Check digits**: ISO 7064 MOD-97-10, which detects every single-character
///   error and every adjacent transposition.
///
/// Whether the account or the Bankleitzahl exists is not derivable from the
/// string and is not checked.
///
/// # Formatting
///
/// Grouping spaces and lowercase normalise away, so a value pasted from a bank
/// statement parses. `as_ref` and `Display` give the compact wire form;
/// [`to_grouped_string`](Iban::to_grouped_string) gives the print form.
///
/// # Examples
/// ```
/// use rubo4e::identifiers::Iban;
///
/// // The canonical ECBS test IBAN.
/// let iban = Iban::new("DE89370400440532013000").unwrap();
/// assert_eq!(iban.country_code(), "DE");
/// assert_eq!(iban.bban(), "370400440532013000");
/// assert_eq!(iban.to_grouped_string(), "DE89 3704 0044 0532 0130 00");
///
/// // Grouped and lowercase input normalises to the same value.
/// assert_eq!(Iban::new("de89 3704 0044 0532 0130 00").unwrap(), iban);
///
/// // A single transposed digit fails the MOD-97 check.
/// assert!(Iban::new("DE89370400440532013090").is_err());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "validate", derive(garde::Validate))]
#[cfg_attr(feature = "validate", garde(allow_unvalidated))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(
    feature = "schemars",
    schemars(schema_with = "crate::schema_helpers::iban_schema")
)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "utoipa", schema(value_type = String))]
pub struct Iban(#[cfg_attr(feature = "validate", garde(custom(check_iban)))] Box<str>);

#[cfg(feature = "validate")]
fn check_iban(value: &str, _: &()) -> Result<(), garde::Error> {
    validate_iban(value).map_err(garde::Error::from)
}

/// Registered IBAN lengths: the SEPA zone plus the non-SEPA countries that appear
/// on European invoices. Sorted, so the lookup can binary-search.
///
/// A country absent from this table is **not** rejected — ISO 13616 adds
/// countries, and a stale list must not refuse a valid IBAN. Its checksum is
/// still verified.
static IBAN_LENGTHS: &[(&str, usize)] = &[
    ("AD", 24),
    ("AE", 23),
    ("AL", 28),
    ("AT", 20),
    ("AZ", 28),
    ("BA", 20),
    ("BE", 16),
    ("BG", 22),
    ("BH", 22),
    ("BR", 29),
    ("BY", 28),
    ("CH", 21),
    ("CR", 22),
    ("CY", 28),
    ("CZ", 24),
    ("DE", 22),
    ("DK", 18),
    ("DO", 28),
    ("EE", 20),
    ("EG", 29),
    ("ES", 24),
    ("FI", 18),
    ("FO", 18),
    ("FR", 27),
    ("GB", 22),
    ("GE", 22),
    ("GI", 23),
    ("GL", 18),
    ("GR", 27),
    ("GT", 28),
    ("HR", 21),
    ("HU", 28),
    ("IE", 22),
    ("IL", 23),
    ("IS", 26),
    ("IT", 27),
    ("JO", 30),
    ("KW", 30),
    ("KZ", 20),
    ("LB", 28),
    ("LC", 32),
    ("LI", 21),
    ("LT", 20),
    ("LU", 20),
    ("LV", 21),
    ("MC", 27),
    ("MD", 24),
    ("ME", 22),
    ("MK", 19),
    ("MR", 27),
    ("MT", 31),
    ("MU", 30),
    ("NL", 18),
    ("NO", 15),
    ("PK", 24),
    ("PL", 28),
    ("PS", 29),
    ("PT", 25),
    ("QA", 29),
    ("RO", 24),
    ("RS", 22),
    ("SA", 24),
    ("SE", 24),
    ("SI", 19),
    ("SK", 24),
    ("SM", 27),
    ("TN", 24),
    ("TR", 26),
    ("UA", 29),
    ("VA", 22),
    ("VG", 24),
    ("XK", 20),
];

/// The registered length for `country`, if the registry pins one.
fn registered_iban_length(country: &str) -> Option<usize> {
    IBAN_LENGTHS
        .binary_search_by_key(&country, |&(c, _)| c)
        .ok()
        .map(|i| IBAN_LENGTHS[i].1)
}

/// Strips the grouping whitespace these identifiers are conventionally written
/// with and upper-cases the result.
///
/// Shared by [`Iban::new`] and [`Bic::new`]: both are written for people in
/// spaced, sometimes lower-cased groups, and both have a single canonical
/// compact form on the wire.
fn normalise_grouping(s: &str) -> String {
    s.chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| c.to_ascii_uppercase())
        .collect()
}

fn validate_iban(s: &str) -> Result<(), IdentifierError> {
    if !(IBAN_MIN_LEN..=IBAN_MAX_LEN).contains(&s.len()) {
        return Err(IdentifierError::InvalidLength {
            expected: LengthExpectation::RangeInclusive {
                min: IBAN_MIN_LEN,
                max: IBAN_MAX_LEN,
            },
            actual: s.len(),
        });
    }

    let bytes = s.as_bytes();
    for (i, &b) in bytes.iter().enumerate() {
        let ok = match i {
            0 | 1 => b.is_ascii_uppercase(),
            2 | 3 => b.is_ascii_digit(),
            _ => b.is_ascii_uppercase() || b.is_ascii_digit(),
        };
        if !ok {
            return Err(IdentifierError::InvalidCharacter {
                position: i,
                character: char_at(s, i),
            });
        }
    }

    // A country the registry pins must use exactly that length. One it does not
    // pin is left to the checksum — see `IBAN_LENGTHS`.
    let country = &s[..2];
    if let Some(expected) = registered_iban_length(country) {
        if s.len() != expected {
            return Err(IdentifierError::InvalidLength {
                expected: LengthExpectation::Exact(expected),
                actual: s.len(),
            });
        }
    }

    if !mod97_is_one(s) {
        return Err(IdentifierError::InvalidChecksum);
    }
    Ok(())
}

/// ISO 7064 MOD-97-10: rotate the first four characters to the end, map letters
/// to `A`=10 … `Z`=35, and read the result as one long integer — which must be
/// congruent to 1 mod 97.
///
/// The integer is far wider than `u128` for a 34-character IBAN, so it is
/// reduced incrementally: `remainder` never exceeds `97 * 100 + 35`.
fn mod97_is_one(iban: &str) -> bool {
    let bytes = iban.as_bytes();
    let mut remainder: u32 = 0;
    // The rotation, expressed as an iteration order rather than an allocation.
    for &b in bytes[4..].iter().chain(&bytes[..4]) {
        remainder = match b {
            b'0'..=b'9' => remainder * 10 + u32::from(b - b'0'),
            b'A'..=b'Z' => {
                let v = u32::from(b - b'A') + 10;
                // A letter contributes two decimal digits.
                remainder * 100 + v
            }
            // `validate_iban` has already rejected everything else.
            _ => return false,
        } % 97;
    }
    remainder == 1
}

impl Iban {
    /// Creates an `Iban`, normalising grouping spaces and case, then verifying
    /// the ISO 7064 MOD-97-10 check digits.
    ///
    /// # Errors
    /// - [`IdentifierError::InvalidLength`] if the normalised value is outside
    ///   15–34 characters, or is not the length its country code registers.
    /// - [`IdentifierError::InvalidCharacter`] for a non-alphanumeric character,
    ///   a non-letter in the country code, or a non-digit in the check digits.
    /// - [`IdentifierError::InvalidChecksum`] if the check digits do not verify.
    #[must_use = "the validated identifier is returned; ignoring it discards the result"]
    pub fn new(s: &str) -> Result<Self, IdentifierError> {
        let normalised = normalise_grouping(s);
        validate_iban(&normalised)?;
        Ok(Self(Box::from(normalised.as_str())))
    }

    /// The ISO 3166-1 alpha-2 country code (the first two characters).
    #[must_use]
    pub fn country_code(&self) -> &str {
        &self.0[..2]
    }

    /// The two check digits (characters 3 and 4).
    #[must_use]
    pub fn check_digits(&self) -> &str {
        &self.0[2..4]
    }

    /// The Basic Bank Account Number — everything after the check digits.
    ///
    /// Its internal structure is country-specific; for `DE` it is an 8-digit
    /// Bankleitzahl followed by a 10-digit Kontonummer.
    #[must_use]
    pub fn bban(&self) -> &str {
        &self.0[4..]
    }

    /// Returns `true` for a German IBAN.
    #[must_use]
    pub fn is_german(&self) -> bool {
        self.country_code() == "DE"
    }

    /// The 8-digit Bankleitzahl of a German IBAN, or `None` for any other country.
    ///
    /// ```
    /// use rubo4e::identifiers::Iban;
    ///
    /// let de = Iban::new("DE89370400440532013000").unwrap();
    /// assert_eq!(de.bankleitzahl(), Some("37040044"));
    ///
    /// let at = Iban::new("AT611904300234573201").unwrap();
    /// assert_eq!(at.bankleitzahl(), None, "only DE has a Bankleitzahl here");
    /// ```
    #[must_use]
    pub fn bankleitzahl(&self) -> Option<&str> {
        self.is_german().then(|| &self.0[4..12])
    }

    /// The 10-digit Kontonummer of a German IBAN, or `None` for any other country.
    #[must_use]
    pub fn kontonummer(&self) -> Option<&str> {
        self.is_german().then(|| &self.0[12..])
    }

    /// Renders the IBAN in the conventional four-character groups.
    ///
    /// The print form, for anything a person reads. BO4E, SEPA, and every other
    /// machine interface take the compact form `as_ref` returns.
    #[must_use]
    pub fn to_grouped_string(&self) -> String {
        let mut out = String::with_capacity(self.0.len() + self.0.len() / 4);
        for (i, c) in self.0.chars().enumerate() {
            if i > 0 && i % 4 == 0 {
                out.push(' ');
            }
            out.push(c);
        }
        out
    }
}

// ─── BIC ─────────────────────────────────────────────────────────────────────

/// A Business Identifier Code (ISO 9362) — the SWIFT code of a financial institution.
///
/// 8 or 11 characters: 4 letters of institution code, 2 of ISO 3166-1 country
/// code, 2 alphanumerics of location code, and — in the 11-character form — 3 of
/// branch code, conventionally `XXX` for a head office.
///
/// ISO 9362 defines no checksum, so construction verifies the grammar and
/// nothing more.
///
/// # Examples
/// ```
/// use rubo4e::identifiers::Bic;
///
/// let bic = Bic::new("COBADEFFXXX").unwrap();
/// assert_eq!(bic.institution_code(), "COBA");
/// assert_eq!(bic.country_code(), "DE");
/// assert_eq!(bic.location_code(), "FF");
/// assert_eq!(bic.branch_code(), Some("XXX"));
/// assert!(bic.is_head_office());
///
/// // The 8-character form omits the branch entirely.
/// let short = Bic::new("MARKDEFF").unwrap();
/// assert_eq!(short.branch_code(), None);
/// assert!(short.is_head_office(), "no branch code means head office");
///
/// // 9 or 10 characters is neither form.
/// assert!(Bic::new("COBADEFFX").is_err());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "validate", derive(garde::Validate))]
#[cfg_attr(feature = "validate", garde(allow_unvalidated))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(
    feature = "schemars",
    schemars(schema_with = "crate::schema_helpers::bic_schema")
)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "utoipa", schema(value_type = String))]
pub struct Bic(#[cfg_attr(feature = "validate", garde(custom(check_bic)))] Box<str>);

#[cfg(feature = "validate")]
fn check_bic(value: &str, _: &()) -> Result<(), garde::Error> {
    validate_bic(value).map_err(garde::Error::from)
}

fn validate_bic(s: &str) -> Result<(), IdentifierError> {
    if s.len() != 8 && s.len() != 11 {
        return Err(IdentifierError::InvalidFormat {
            description: format!("a BIC is 8 or 11 characters, got {}", s.len()).into(),
        });
    }
    for (i, &b) in s.as_bytes().iter().enumerate() {
        // Institution code (0–3) and country code (4–5) are letters; the
        // location and branch codes are alphanumeric.
        let ok = if i < 6 {
            b.is_ascii_uppercase()
        } else {
            b.is_ascii_uppercase() || b.is_ascii_digit()
        };
        if !ok {
            return Err(IdentifierError::InvalidCharacter {
                position: i,
                character: char_at(s, i),
            });
        }
    }
    Ok(())
}

impl Bic {
    /// Creates a `Bic`, upper-casing and stripping whitespace first.
    ///
    /// # Errors
    /// - [`IdentifierError::InvalidFormat`] if the length is neither 8 nor 11.
    /// - [`IdentifierError::InvalidCharacter`] for a digit in the institution or
    ///   country code, or a non-alphanumeric anywhere.
    #[must_use = "the validated identifier is returned; ignoring it discards the result"]
    pub fn new(s: &str) -> Result<Self, IdentifierError> {
        let normalised = normalise_grouping(s);
        validate_bic(&normalised)?;
        Ok(Self(Box::from(normalised.as_str())))
    }

    /// The 4-letter institution (bank) code.
    #[must_use]
    pub fn institution_code(&self) -> &str {
        &self.0[..4]
    }

    /// The ISO 3166-1 alpha-2 country code.
    #[must_use]
    pub fn country_code(&self) -> &str {
        &self.0[4..6]
    }

    /// The 2-character location code.
    #[must_use]
    pub fn location_code(&self) -> &str {
        &self.0[6..8]
    }

    /// The 3-character branch code, or `None` for the 8-character form.
    #[must_use]
    pub fn branch_code(&self) -> Option<&str> {
        (self.0.len() == 11).then(|| &self.0[8..])
    }

    /// Returns `true` if this BIC addresses the institution's head office —
    /// either the 8-character form, or the 11-character form with the
    /// conventional `XXX` branch.
    #[must_use]
    pub fn is_head_office(&self) -> bool {
        matches!(self.branch_code(), None | Some("XXX"))
    }

    /// Returns `true` if this is a passive (non-SWIFT-connected) BIC — location
    /// code ending in `1`, per ISO 9362.
    #[must_use]
    pub fn is_passive(&self) -> bool {
        self.location_code().ends_with('1')
    }

    /// Returns `true` for a German BIC.
    #[must_use]
    pub fn is_german(&self) -> bool {
        self.country_code() == "DE"
    }
}

impl_identifier_traits!(Iban, "an IBAN (ISO 13616) with valid MOD-97 check digits");
impl_identifier_traits!(Bic, "a BIC (ISO 9362) of 8 or 11 characters");

#[cfg(test)]
mod tests {
    use super::*;

    // ── IBAN ─────────────────────────────────────────────────────────────────

    /// Published test IBANs from the ECBS / national registries. Every one of
    /// these is a documented example, not a value invented here.
    #[test]
    fn accepts_published_test_ibans() {
        for iban in [
            "DE89370400440532013000",          // ECBS, Germany
            "GB82WEST12345698765432",          // ECBS, United Kingdom
            "FR1420041010050500013M02606",     // France
            "AT611904300234573201",            // Austria
            "CH9300762011623852957",           // Switzerland
            "NL91ABNA0417164300",              // Netherlands
            "BE68539007547034",                // Belgium
            "IT60X0542811101000000123456",     // Italy
            "ES9121000418450200051332",        // Spain
            "NO9386011117947",                 // Norway — the shortest registered format
            "MT84MALT011000012345MTLCAST001S", // Malta — 31 characters
        ] {
            assert!(Iban::new(iban).is_ok(), "{iban} is a published valid IBAN");
        }
    }

    /// The check exists to catch exactly these two kinds of typo.
    #[test]
    fn rejects_the_errors_mod97_is_designed_to_catch() {
        let valid = "DE89370400440532013000";
        assert!(Iban::new(valid).is_ok());

        // Every single-digit substitution in the BBAN must fail.
        for pos in 4..valid.len() {
            let mut bytes = valid.as_bytes().to_vec();
            let orig = bytes[pos];
            bytes[pos] = if orig == b'9' { b'8' } else { orig + 1 };
            let mutated = String::from_utf8(bytes).expect("ascii");
            assert!(
                Iban::new(&mutated).is_err(),
                "a single-digit error at {pos} slipped through: {mutated}"
            );
        }

        // …as must every transposition of two adjacent, differing characters.
        for pos in 4..valid.len() - 1 {
            let mut bytes = valid.as_bytes().to_vec();
            if bytes[pos] == bytes[pos + 1] {
                continue;
            }
            bytes.swap(pos, pos + 1);
            let mutated = String::from_utf8(bytes).expect("ascii");
            assert!(
                Iban::new(&mutated).is_err(),
                "a transposition at {pos} slipped through: {mutated}"
            );
        }
    }

    #[test]
    fn normalises_grouping_and_case() {
        let canonical = Iban::new("DE89370400440532013000").unwrap();
        for written in [
            "DE89 3704 0044 0532 0130 00",
            "de89370400440532013000",
            "  DE89 3704 0044 0532 0130 00  ",
            "DE89\t3704\n0044 0532 0130 00",
        ] {
            assert_eq!(Iban::new(written).unwrap(), canonical, "{written:?}");
        }
        // …and the stored form is always compact.
        assert_eq!(canonical.as_ref(), "DE89370400440532013000");
        assert_eq!(canonical.to_string(), "DE89370400440532013000");
    }

    #[test]
    fn renders_the_print_form_in_groups_of_four() {
        let iban = Iban::new("DE89370400440532013000").unwrap();
        assert_eq!(iban.to_grouped_string(), "DE89 3704 0044 0532 0130 00");
        // The print form parses back to the same value.
        assert_eq!(Iban::new(&iban.to_grouped_string()).unwrap(), iban);
    }

    #[test]
    fn splits_a_german_iban_into_its_parts() {
        let iban = Iban::new("DE89370400440532013000").unwrap();
        assert_eq!(iban.country_code(), "DE");
        assert_eq!(iban.check_digits(), "89");
        assert_eq!(iban.bban(), "370400440532013000");
        assert_eq!(iban.bankleitzahl(), Some("37040044"));
        assert_eq!(iban.kontonummer(), Some("0532013000"));
        assert!(iban.is_german());
    }

    #[test]
    fn a_country_specific_length_is_enforced() {
        // A German IBAN is 22 characters; 21 has the wrong length even before
        // the checksum gets a say.
        let err = Iban::new("DE8937040044053201300").unwrap_err();
        assert_eq!(
            err,
            IdentifierError::InvalidLength {
                expected: LengthExpectation::Exact(22),
                actual: 21,
            }
        );
    }

    /// An unregistered country code must not be rejected on length alone — the
    /// registry grows, and this table will lag it.
    #[test]
    fn an_unregistered_country_is_left_to_the_checksum() {
        assert_eq!(registered_iban_length("ZZ"), None);
        // A well-formed "ZZ" IBAN with correct check digits is accepted…
        let candidate = with_correct_check_digits("ZZ", "12345678901234");
        assert!(
            Iban::new(&candidate).is_ok(),
            "{candidate} has valid check digits and an unknown country"
        );
        // …but a wrong checksum still fails.
        let mut bad = candidate.into_bytes();
        bad[2] = if bad[2] == b'0' { b'1' } else { b'0' };
        assert!(Iban::new(&String::from_utf8(bad).unwrap()).is_err());
    }

    /// `registered_iban_length` binary-searches this table, which silently
    /// returns wrong answers on unsorted input — a valid IBAN would be rejected
    /// for having "the wrong length", or a wrong one accepted.
    #[test]
    fn the_registered_length_table_is_sorted_and_well_formed() {
        assert!(
            IBAN_LENGTHS.windows(2).all(|w| w[0].0 < w[1].0),
            "IBAN_LENGTHS must be strictly sorted by country code"
        );
        for &(country, len) in IBAN_LENGTHS {
            assert_eq!(country.len(), 2, "{country} is not an alpha-2 code");
            assert!(
                country.bytes().all(|b| b.is_ascii_uppercase()),
                "{country} must be uppercase"
            );
            assert!(
                (IBAN_MIN_LEN..=IBAN_MAX_LEN).contains(&len),
                "{country} registers {len}, outside the ISO 13616 envelope"
            );
            assert_eq!(registered_iban_length(country), Some(len));
        }
    }

    /// Builds a syntactically valid IBAN for `country` + `bban` by computing the
    /// check digits the ISO 7064 way, so the test does not hard-code one.
    fn with_correct_check_digits(country: &str, bban: &str) -> String {
        for candidate in 2..=98u32 {
            let iban = format!("{country}{candidate:02}{bban}");
            if mod97_is_one(&iban) {
                return iban;
            }
        }
        unreachable!("some two-digit checksum always satisfies MOD-97")
    }

    #[test]
    fn rejects_malformed_shapes() {
        for (bad, why) in [
            ("", "empty"),
            ("DE89", "far too short"),
            ("D189370400440532013000", "digit in the country code"),
            ("DEX9370400440532013000", "letter in the check digits"),
            ("DE89370400440532013-00", "punctuation in the BBAN"),
            ("DE8937040044053201300012345678", "too long for DE"),
            (
                "DE89370400440532013000DE89370400440532013000",
                "past the ISO ceiling",
            ),
        ] {
            assert!(Iban::new(bad).is_err(), "{bad:?} must be rejected ({why})");
        }
    }

    /// Non-ASCII input must be rejected without panicking on a byte index that
    /// falls inside a multi-byte character.
    #[test]
    fn rejects_non_ascii_without_panicking() {
        for bad in [
            "DE89370400440532013ÄÖÜ",
            "DEß9370400440532013000",
            "🏦🏦🏦🏦🏦🏦🏦🏦🏦🏦🏦🏦🏦🏦🏦",
        ] {
            assert!(Iban::new(bad).is_err(), "{bad:?}");
        }
    }

    // ── BIC ──────────────────────────────────────────────────────────────────

    #[test]
    fn accepts_real_german_bics() {
        for bic in [
            "COBADEFFXXX", // Commerzbank
            "MARKDEFF",    // Bundesbank, 8-character form
            "DEUTDEFF",    // Deutsche Bank
            "GENODEF1S04", // a cooperative bank, alphanumeric branch
            "PBNKDEFFXXX",
        ] {
            assert!(Bic::new(bic).is_ok(), "{bic} is a real BIC");
        }
    }

    #[test]
    fn splits_a_bic_into_its_parts() {
        let bic = Bic::new("GENODEF1S04").unwrap();
        assert_eq!(bic.institution_code(), "GENO");
        assert_eq!(bic.country_code(), "DE");
        assert_eq!(bic.location_code(), "F1");
        assert_eq!(bic.branch_code(), Some("S04"));
        assert!(bic.is_german());
        assert!(!bic.is_head_office());
        assert!(bic.is_passive(), "location code ending in 1 is passive");
    }

    #[test]
    fn head_office_is_either_form() {
        assert!(Bic::new("MARKDEFF").unwrap().is_head_office());
        assert!(Bic::new("COBADEFFXXX").unwrap().is_head_office());
        assert!(!Bic::new("COBADEFF100").unwrap().is_head_office());
    }

    #[test]
    fn rejects_malformed_bics() {
        for (bad, why) in [
            ("", "empty"),
            ("COBADEF", "7 characters"),
            ("COBADEFFX", "9 characters"),
            ("COBADEFFXX", "10 characters"),
            ("COBADEFFXXXX", "12 characters"),
            ("C0BADEFFXXX", "digit in the institution code"),
            ("COBAD1FFXXX", "digit in the country code"),
            ("COBA-EFFXXX", "punctuation"),
        ] {
            assert!(Bic::new(bad).is_err(), "{bad:?} must be rejected ({why})");
        }
    }

    #[test]
    fn normalises_bic_case_and_spacing() {
        assert_eq!(
            Bic::new("coba deff xxx").unwrap(),
            Bic::new("COBADEFFXXX").unwrap()
        );
    }
}
