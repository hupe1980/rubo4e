use super::checksum::{compute_numeric_id_from_base, validate_numeric_id};
use crate::error::IdentifierError;

/// Length of a Marktlokations-ID, including the check digit.
const LEN: usize = 11;

/// Minimum value of the first digit (Vergabestelle) — `0` is not assigned (§3.2).
const MIN_FIRST_DIGIT: u8 = 1;

/// Marktlokations-ID (MaLo-ID) — identifies a *Marktlokation* or *Tranche*.
///
/// Defined by BDEW "Identifikatoren in der Marktkommunikation" v1.2
/// (7 February 2025), §3, and by BNetzA-Festlegung BK6-16-200 / BK7-16-142.
/// In use across electricity and gas since 1 February 2018.
///
/// ## Format (§3.2)
///
/// | Position | Content | Character set |
/// |----------|---------|---------------|
/// | 1 | Vergabestelle — `1`–`3` = DVGW, `4`–`9` = BDEW | `[1-9]` |
/// | 2–10 | Automatically assigned body | `[0-9]` |
/// | 11 | Check digit (§8.1) | `[0-9]` |
///
/// The Vergabestelle digit says **nothing** about the commodity: a MaLo-ID issued
/// by either office can be assigned to an electricity Marktlokation, a gas
/// Marktlokation, or a Tranche.
///
/// ## Check digit (§8.1 — Lok- und Waggon-Kennzeichnungsverfahren)
///
/// Sum the digits at odd positions, add twice the sum of the digits at even
/// positions, and take the difference to the next multiple of 10. The BDEW
/// document's own worked example:
///
/// ```text
/// MaLo-ID base:  4 1 3 7 3 5 5 9 2 4
/// a) odd:        4 + 3 + 3 + 5 + 2       = 17
/// b) even:      (1 + 7 + 5 + 9 + 4) * 2  = 52
/// c) sum:        17 + 52                 = 69
/// d) check:      70 - 69                 = 1
/// → 41373559241
/// ```
///
/// This is the same procedure that validates BDEW- and DVGW-Codenummern; see
/// [`MarktpartnerId`](super::MarktpartnerId).
///
/// ## What the check digit does and does not catch
///
/// It catches every adjacent transposition of two distinct digits, and every
/// single-digit typo **except** one class: changing a digit by exactly 5 at an
/// even position, because that position carries weight 2 and `2 · 5 ≡ 0 (mod 10)`.
/// For example `41373559241` and `46373559241` share a check digit.
///
/// That blind spot is inherent to the BDEW specification, not to this
/// implementation. Treat the check digit as a typo guard, not as authentication —
/// only the issuing office can confirm that an ID was actually assigned.
///
/// # Examples
/// ```
/// use rubo4e::identifiers::MaloId;
///
/// // The worked example from the BDEW specification.
/// let id = MaloId::new("41373559241").unwrap();
/// assert_eq!(id.as_ref(), "41373559241");
///
/// // The check digit is derived, so it never has to be typed by hand.
/// assert_eq!(MaloId::from_base("4137355924").unwrap(), id);
/// assert_eq!(MaloId::check_digit("4137355924").unwrap(), 1);
///
/// // A wrong check digit is rejected.
/// assert!(MaloId::new("41373559242").is_err());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "validate", derive(garde::Validate))]
#[cfg_attr(feature = "validate", garde(allow_unvalidated))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(
    feature = "schemars",
    schemars(schema_with = "crate::schema_helpers::malo_id_schema")
)]
#[cfg_attr(
    feature = "schemars",
    schemars(description = crate::identifiers::schema::MALO_ID.description)
)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "utoipa", schema(
    value_type = String,
    pattern = r"^[1-9][0-9]{10}$",
    example = "41373559241",
    description = crate::identifiers::schema::MALO_ID.description
))]
pub struct MaloId(#[cfg_attr(feature = "validate", garde(custom(check_malo_id)))] Box<str>);

#[cfg(feature = "validate")]
fn check_malo_id(value: &str, _: &()) -> Result<(), garde::Error> {
    validate_numeric_id(value, LEN, MIN_FIRST_DIGIT).map_err(garde::Error::from)
}

/// Issuing office (Vergabestelle) encoded in the first digit of a MaLo-ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MaloVergabestelle {
    /// Digits `1`–`3` — DVGW Services und Consult GmbH.
    Dvgw,
    /// Digits `4`–`9` — Energie Codes und Services GmbH (BDEW).
    Bdew,
}

impl std::fmt::Display for MaloVergabestelle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Dvgw => "DVGW",
            Self::Bdew => "BDEW",
        })
    }
}

impl MaloId {
    /// Creates a new `MaloId`, validating the format and the §8.1 check digit.
    ///
    /// # Errors
    /// - [`IdentifierError::InvalidLength`] if `s` is not exactly 11 characters.
    /// - [`IdentifierError::InvalidCharacter`] if any character is not a decimal digit.
    /// - [`IdentifierError::InvalidFormat`] if the first digit is `0`.
    /// - [`IdentifierError::InvalidChecksum`] if the 11th digit does not match.
    #[must_use = "the validated identifier is returned; ignoring it discards the result"]
    pub fn new(s: &str) -> Result<Self, IdentifierError> {
        validate_numeric_id(s, LEN, MIN_FIRST_DIGIT)?;
        Ok(Self(Box::from(s)))
    }

    /// Builds a `MaloId` from its 10-digit base by computing and appending the
    /// §8.1 check digit.
    ///
    /// # Errors
    /// - [`IdentifierError::InvalidLength`] if `base` is not exactly 10 characters.
    /// - [`IdentifierError::InvalidCharacter`] if any character is not a decimal digit.
    /// - [`IdentifierError::InvalidFormat`] if the first digit is `0`.
    ///
    /// # Examples
    /// ```
    /// use rubo4e::identifiers::MaloId;
    ///
    /// assert_eq!(MaloId::from_base("4137355924").unwrap().as_ref(), "41373559241");
    /// ```
    pub fn from_base(base: &str) -> Result<Self, IdentifierError> {
        let full = compute_numeric_id_from_base(base, LEN, MIN_FIRST_DIGIT)?;
        Ok(Self(full.into_boxed_str()))
    }

    /// Computes the §8.1 check digit (`0`–`9`) for a 10-digit base without
    /// constructing a `MaloId`.
    ///
    /// # Errors
    /// Same as [`from_base`](Self::from_base).
    ///
    /// # Examples
    /// ```
    /// use rubo4e::identifiers::MaloId;
    ///
    /// assert_eq!(MaloId::check_digit("4137355924").unwrap(), 1);
    /// ```
    pub fn check_digit(base: &str) -> Result<u8, IdentifierError> {
        let full = compute_numeric_id_from_base(base, LEN, MIN_FIRST_DIGIT)?;
        Ok(full.as_bytes()[LEN - 1] - b'0')
    }

    /// Returns the 10-digit base (everything except the check digit).
    #[must_use]
    pub fn base(&self) -> &str {
        &self.0[..LEN - 1]
    }

    /// Returns the issuing office encoded in the first digit (§3.2).
    ///
    /// # Examples
    /// ```
    /// use rubo4e::identifiers::{MaloId, MaloVergabestelle};
    ///
    /// let id = MaloId::new("41373559241").unwrap();
    /// assert_eq!(id.vergabestelle(), MaloVergabestelle::Bdew);
    /// ```
    #[must_use]
    pub fn vergabestelle(&self) -> MaloVergabestelle {
        // Validated at construction: the first byte is a digit in 1..=9.
        match self.0.as_bytes()[0] {
            b'1'..=b'3' => MaloVergabestelle::Dvgw,
            _ => MaloVergabestelle::Bdew,
        }
    }
}

impl_identifier_traits!(MaloId, "an 11-digit Marktlokations-ID (BDEW check digit)");

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::LengthExpectation;

    /// Reference vectors that do **not** come from this implementation.
    ///
    /// - `41373559241` is the worked example printed in BDEW "Identifikatoren in
    ///   der Marktkommunikation" v1.2 §8.1 and on the German Wikipedia article
    ///   for the Marktlokations-Identifikationsnummer.
    /// - `51238696781` is the fixture used by the reference implementation
    ///   [BO4E-python](https://github.com/bo4e/BO4E-python) in its
    ///   `validate_marktlokations_id` tests.
    ///
    /// These pin the algorithm to an external source, so a regression cannot be
    /// masked by generating expectations from the code under test.
    const EXTERNAL_VECTORS: &[(&str, &str)] =
        &[("4137355924", "41373559241"), ("5123869678", "51238696781")];

    #[test]
    fn external_reference_vectors_validate() {
        for &(base, full) in EXTERNAL_VECTORS {
            let id = MaloId::new(full)
                .unwrap_or_else(|e| panic!("{full} must be a valid MaLo-ID, got: {e}"));
            assert_eq!(id.as_ref(), full);
            assert_eq!(MaloId::from_base(base).unwrap(), id);
            assert_eq!(id.base(), base);
        }
    }

    /// Every single-digit perturbation of the check digit must be rejected — this
    /// is what the check digit exists for.
    #[test]
    fn every_wrong_check_digit_is_rejected() {
        for &(base, full) in EXTERNAL_VECTORS {
            let correct = full.as_bytes()[10];
            for d in b'0'..=b'9' {
                if d == correct {
                    continue;
                }
                let candidate = format!("{base}{}", d as char);
                assert!(
                    matches!(
                        MaloId::new(&candidate),
                        Err(IdentifierError::InvalidChecksum)
                    ),
                    "{candidate} must be rejected as an invalid checksum"
                );
            }
        }
    }

    /// Pins the exact error-detection power of the §8.1 procedure.
    ///
    /// A single-digit typo in the body shifts the weighted sum by `δ` at an odd
    /// position and by `2δ` at an even position. The check digit therefore misses
    /// exactly one class of typo: `δ = ±5` at an even position, because
    /// `2 · 5 ≡ 0 (mod 10)`. Everything else is caught.
    ///
    /// This is a property of the BDEW specification, not of this implementation —
    /// see the note on [`MaloId`]. The test asserts it precisely so that a future
    /// change to the algorithm cannot quietly alter the guarantee.
    #[test]
    fn single_digit_typo_detection_matches_specification() {
        let full = "41373559241";
        for pos in 0..10 {
            let original = i32::from(full.as_bytes()[pos] - b'0');
            for d in b'0'..=b'9' {
                if full.as_bytes()[pos] == d || (pos == 0 && d == b'0') {
                    continue;
                }
                let mut bytes = full.as_bytes().to_vec();
                bytes[pos] = d;
                let candidate = String::from_utf8(bytes).unwrap();

                let delta = (i32::from(d - b'0') - original).rem_euclid(10);
                // 1-indexed even position → weight 2 → δ = ±5 is invisible mod 10.
                let undetectable = pos % 2 == 1 && delta == 5;

                assert_eq!(
                    MaloId::new(&candidate).is_err(),
                    !undetectable,
                    "{candidate} vs {full}: single-digit change at position {pos} \
                     (δ={delta}) — expected detected={}",
                    !undetectable
                );
            }
        }
    }

    /// Transposing two adjacent, distinct digits is always caught: the swap moves
    /// `a` from weight 1 to weight 2 and `b` the other way, shifting the sum by
    /// `a − b`, which is non-zero mod 10 whenever `a ≠ b`.
    #[test]
    fn adjacent_transpositions_are_caught() {
        let full = "41373559241";
        for pos in 0..9 {
            let bytes = full.as_bytes();
            if bytes[pos] == bytes[pos + 1] {
                continue;
            }
            let mut swapped = bytes.to_vec();
            swapped.swap(pos, pos + 1);
            if swapped[0] == b'0' {
                continue;
            }
            let candidate = String::from_utf8(swapped).unwrap();
            assert!(
                MaloId::new(&candidate).is_err(),
                "{candidate} transposes positions {pos}/{} of {full} and must be rejected",
                pos + 1
            );
        }
    }

    #[test]
    fn first_digit_zero_is_rejected() {
        // Structurally 11 digits with a correct check digit, but Vergabestelle 0
        // is not assigned (§3.2).
        let base = "0137355924";
        assert!(matches!(
            MaloId::from_base(base),
            Err(IdentifierError::InvalidFormat { .. })
        ));
    }

    #[test]
    fn vergabestelle_classification() {
        for (base, expected) in [
            ("1137355924", MaloVergabestelle::Dvgw),
            ("3137355924", MaloVergabestelle::Dvgw),
            ("4137355924", MaloVergabestelle::Bdew),
            ("9137355924", MaloVergabestelle::Bdew),
        ] {
            let id = MaloId::from_base(base).unwrap();
            assert_eq!(id.vergabestelle(), expected, "for {base}");
        }
    }

    #[test]
    fn wrong_length_is_rejected() {
        for (input, actual) in [("", 0usize), ("1234567890", 10), ("123456789012", 12)] {
            assert!(matches!(
                MaloId::new(input).unwrap_err(),
                IdentifierError::InvalidLength {
                    expected: LengthExpectation::Exact(11),
                    actual: a,
                } if a == actual
            ));
        }
    }

    #[test]
    fn non_digit_is_rejected_with_position() {
        assert!(matches!(
            MaloId::new("4137X559241").unwrap_err(),
            IdentifierError::InvalidCharacter {
                position: 4,
                character: 'X'
            }
        ));
        assert!(matches!(
            MaloId::new("4137 559241").unwrap_err(),
            IdentifierError::InvalidCharacter {
                position: 4,
                character: ' '
            }
        ));
        assert!(matches!(
            MaloId::new("-137355924").unwrap_err(),
            IdentifierError::InvalidLength { .. }
        ));
    }

    #[test]
    fn conversions_round_trip() {
        let id = MaloId::new("41373559241").unwrap();
        assert_eq!(id.to_string().parse::<MaloId>().unwrap(), id);
        assert_eq!(MaloId::try_from("41373559241").unwrap(), id);
        assert_eq!(MaloId::try_from(String::from("41373559241")).unwrap(), id);
        assert_eq!(String::from(id.clone()), "41373559241");
        assert_eq!(&*id, "41373559241");
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_round_trip_and_rejection() {
        let id = MaloId::new("41373559241").unwrap();
        let json = serde_json::to_string(&id).unwrap();
        assert_eq!(json, r#""41373559241""#);
        assert_eq!(serde_json::from_str::<MaloId>(&json).unwrap(), id);
        // Deserialization must not bypass validation.
        assert!(serde_json::from_str::<MaloId>(r#""41373559242""#).is_err());
    }
}
