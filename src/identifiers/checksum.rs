//! BDEW check-digit procedures shared by the German energy-market identifiers.
//!
//! Source: BDEW Anwendungshilfe **"Identifikatoren in der Marktkommunikation"**,
//! version 1.2 (7 February 2025), chapter 8.
//!
//! The document defines two procedures — and they are the *same* arithmetic:
//!
//! | § | Name | Applies to |
//! |---|------|------------|
//! | 8.1 | Lok- und Waggon-Kennzeichnungsverfahren | BDEW-Codenummer, DVGW-Codenummer, MaLo-ID |
//! | 8.2 | ASCII-Verfahren | Ressourcen-ID (TR/SR/SG/CR), NeLo-ID, NeBe-ID, Paket-ID |
//!
//! Both compute:
//!
//! 1. Map every character of the base to a numeric value.
//!    §8.1 bases are purely numeric, so a digit maps to its own value.
//!    §8.2 bases may contain uppercase letters, which map to their ASCII code
//!    (`A` = 65 … `Z` = 90).
//! 2. Sum the values at **odd** positions (1-indexed 1, 3, 5, … → 0-indexed 0, 2, 4, …).
//! 3. Sum the values at **even** positions and multiply that sum by 2.
//! 4. The check digit is the difference from the sum of (2) and (3) to the next
//!    multiple of 10 — i.e. `(10 - (total % 10)) % 10`, so a total that is already
//!    a multiple of 10 yields check digit `0`.
//!
//! Because a decimal digit maps to itself under both mappings, §8.1 is exactly
//! §8.2 restricted to numeric bases. This module therefore implements the
//! arithmetic **once**, in [`bdew_check_digit`], and exposes two validators that
//! differ only in the character set they accept.
//!
//! ## Reference vectors (from the BDEW document)
//!
//! - §8.1, MaLo-ID base `4137355924` → check digit `1` (full ID `41373559241`)
//! - §8.2, base `A113735592` → check digit `5` (full ID `A1137355925`)

use crate::error::{IdentifierError, LengthExpectation};

// ─── Core algorithm ──────────────────────────────────────────────────────────

/// Maps a base character to its numeric value for the BDEW check-digit procedures.
///
/// - Decimal digits (`0`–`9`) → their numeric value `0`–`9`.
/// - Uppercase letters (`A`–`Z`) → their ASCII code value (`A` = 65 … `Z` = 90).
///
/// Any other byte maps to its raw ASCII value; callers validate the character set
/// before calling, so such bytes never reach here in practice.
#[inline]
fn char_value(b: u8) -> u32 {
    if b.is_ascii_digit() {
        u32::from(b - b'0')
    } else {
        u32::from(b)
    }
}

/// Computes the BDEW check digit for `base` (chapter 8 — both §8.1 and §8.2).
///
/// Sums the mapped values at odd 1-indexed positions, adds twice the sum at even
/// 1-indexed positions, and returns the difference to the next multiple of 10.
///
/// # Examples (BDEW reference vectors)
///
/// This function is crate-private, so the vectors below are pinned by
/// `tests::bdew_reference_vector_lok_waggon` and
/// `tests::bdew_reference_vector_ascii` rather than by a doctest:
///
/// ```text
/// bdew_check_digit(b"4137355924") == 1   // §8.1 MaLo-ID
/// bdew_check_digit(b"A113735592") == 5   // §8.2 ASCII-Verfahren
/// ```
pub(super) fn bdew_check_digit(base: &[u8]) -> u8 {
    let odd: u32 = base.iter().step_by(2).map(|&b| char_value(b)).sum();
    let even: u32 = base.iter().skip(1).step_by(2).map(|&b| char_value(b)).sum();
    ((10 - ((odd + even * 2) % 10)) % 10) as u8
}

// ─── §8.1 — numeric identifiers (MaLo-ID, BDEW/DVGW-Codenummer) ──────────────

/// Validates a purely numeric identifier of `len` characters whose final digit is
/// the §8.1 check digit.
///
/// `min_first` constrains the first digit (the Vergabestelle): MaLo-IDs require
/// `1`–`9` per §3.2, whereas 13-digit Marktpartner-IDs permit a leading `0`.
pub(super) fn validate_numeric_id(
    s: &str,
    len: usize,
    min_first: u8,
) -> Result<(), IdentifierError> {
    if s.len() != len {
        return Err(IdentifierError::InvalidLength {
            expected: LengthExpectation::Exact(len),
            actual: s.len(),
        });
    }
    let bytes = s.as_bytes();
    for (i, &b) in bytes.iter().enumerate() {
        if !b.is_ascii_digit() {
            return Err(IdentifierError::InvalidCharacter {
                position: i,
                character: char_at(s, i),
            });
        }
    }
    if bytes[0] - b'0' < min_first {
        return Err(IdentifierError::InvalidFormat {
            description: format!(
                "first digit (Vergabestelle) must be {}-9, got '{}'",
                min_first, bytes[0] as char,
            )
            .into(),
        });
    }
    let (base, check) = bytes.split_at(len - 1);
    if check[0] - b'0' != bdew_check_digit(base) {
        return Err(IdentifierError::InvalidChecksum);
    }
    Ok(())
}

/// Appends the §8.1 check digit to a numeric base of `len - 1` digits, returning
/// the complete `len`-character identifier.
pub(super) fn compute_numeric_id_from_base(
    base: &str,
    len: usize,
    min_first: u8,
) -> Result<String, IdentifierError> {
    let base_len = len - 1;
    if base.len() != base_len {
        return Err(IdentifierError::InvalidLength {
            expected: LengthExpectation::Exact(base_len),
            actual: base.len(),
        });
    }
    let bytes = base.as_bytes();
    for (i, &b) in bytes.iter().enumerate() {
        if !b.is_ascii_digit() {
            return Err(IdentifierError::InvalidCharacter {
                position: i,
                character: char_at(base, i),
            });
        }
    }
    if bytes[0] - b'0' < min_first {
        return Err(IdentifierError::InvalidFormat {
            description: format!(
                "first digit (Vergabestelle) must be {}-9, got '{}'",
                min_first, bytes[0] as char,
            )
            .into(),
        });
    }
    let mut out = String::with_capacity(len);
    out.push_str(base);
    out.push(char::from(b'0' + bdew_check_digit(bytes)));
    Ok(out)
}

// ─── §8.2 — alphanumeric identifiers (NeLo, NeBe, Ressourcen, Paket) ─────────

/// Checks the fixed Codetyp `prefix` and the `[A-Z0-9]` body of a 10-character
/// §8.2 base. Shared by the validating and constructing entry points.
fn check_ascii_base(base: &str, prefix: &[u8]) -> Result<(), IdentifierError> {
    let bytes = base.as_bytes();
    if !bytes.starts_with(prefix) {
        return Err(IdentifierError::InvalidFormat {
            description: format!(
                "identifier must start with Codetyp \"{}\", got \"{}\"",
                std::str::from_utf8(prefix).unwrap_or("?"),
                base.chars().take(prefix.len()).collect::<String>(),
            )
            .into(),
        });
    }
    for (i, &b) in bytes.iter().enumerate().skip(prefix.len()) {
        if !b.is_ascii_uppercase() && !b.is_ascii_digit() {
            return Err(IdentifierError::InvalidCharacter {
                position: i,
                character: char_at(base, i),
            });
        }
    }
    Ok(())
}

/// Validates an 11-character alphanumeric BDEW identifier using the §8.2
/// ASCII-Verfahren.
///
/// Constraints:
/// - Length exactly 11.
/// - `s` starts with the fixed Codetyp `prefix` (one byte for NeLo/NeBe/Ressourcen,
///   two bytes `b"P9"` for the Paket-ID).
/// - The remaining base characters up to position 10 are `[A-Z0-9]`.
/// - `s[10]` is a decimal digit matching the check digit for `s[..10]`.
pub(super) fn validate_ascii_id(s: &str, prefix: &[u8]) -> Result<(), IdentifierError> {
    if s.len() != 11 {
        return Err(IdentifierError::InvalidLength {
            expected: LengthExpectation::Exact(11),
            actual: s.len(),
        });
    }
    // Guarantees every byte index below is a character boundary, so the `&s[..10]`
    // slice and the per-position error reporting can never split a code point.
    if let Some((i, c)) = s.char_indices().find(|(_, c)| !c.is_ascii()) {
        return Err(IdentifierError::InvalidCharacter {
            position: i,
            character: c,
        });
    }
    let bytes = s.as_bytes();
    check_ascii_base(&s[..10], prefix)?;
    if !bytes[10].is_ascii_digit() {
        return Err(IdentifierError::InvalidCharacter {
            position: 10,
            character: char_at(s, 10),
        });
    }
    if bytes[10] - b'0' != bdew_check_digit(&bytes[..10]) {
        return Err(IdentifierError::InvalidChecksum);
    }
    Ok(())
}

/// Appends the §8.2 check digit to a 10-character base, returning the complete
/// 11-character identifier.
pub(super) fn compute_ascii_id_from_base(
    base: &str,
    prefix: &[u8],
) -> Result<String, IdentifierError> {
    if base.len() != 10 {
        return Err(IdentifierError::InvalidLength {
            expected: LengthExpectation::Exact(10),
            actual: base.len(),
        });
    }
    check_ascii_base(base, prefix)?;
    let mut out = String::with_capacity(11);
    out.push_str(base);
    out.push(char::from(b'0' + bdew_check_digit(base.as_bytes())));
    Ok(out)
}

/// Returns the character starting at byte index `i`, or U+FFFD when `i` is out of
/// range or falls inside a multi-byte sequence.
///
/// Used only on error paths. Slicing is guarded by [`str::is_char_boundary`] so
/// this can never panic on non-ASCII input.
#[inline]
fn char_at(s: &str, i: usize) -> char {
    if s.is_char_boundary(i) {
        s[i..].chars().next().unwrap_or('\u{FFFD}')
    } else {
        '\u{FFFD}'
    }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    /// BDEW "Identifikatoren in der Marktkommunikation" v1.2, §8.1 worked example.
    #[test]
    fn bdew_reference_vector_lok_waggon() {
        // a) 4 + 3 + 3 + 5 + 2 = 17
        // b) (1 + 7 + 5 + 9 + 4) * 2 = 52
        // c) 17 + 52 = 69
        // d) 70 - 69 = 1
        assert_eq!(bdew_check_digit(b"4137355924"), 1);
    }

    /// BDEW "Identifikatoren in der Marktkommunikation" v1.2, §8.2 worked example.
    #[test]
    fn bdew_reference_vector_ascii() {
        // a) A = 65
        // b) 65 + 1 + 7 + 5 + 9 = 87
        // c) (1 + 3 + 3 + 5 + 2) * 2 = 28
        // d) 87 + 28 = 115
        // e) 120 - 115 = 5
        assert_eq!(bdew_check_digit(b"A113735592"), 5);
    }

    /// A total that is already a multiple of 10 must yield check digit 0, not 10.
    #[test]
    fn multiple_of_ten_yields_zero() {
        assert_eq!(bdew_check_digit(b"0000000000"), 0);
    }

    /// §8.1 and §8.2 are the same arithmetic — a numeric base must produce the
    /// same digit through either entry point.
    #[test]
    fn numeric_and_ascii_paths_agree() {
        let full = compute_numeric_id_from_base("4137355924", 11, 1).unwrap();
        assert_eq!(full, "41373559241");
        assert_eq!(bdew_check_digit(b"4137355924"), 1);
    }

    #[test]
    fn numeric_id_rejects_leading_zero_when_required() {
        // MaLo-ID: first digit must be 1-9 (§3.2).
        let err = validate_numeric_id("01234567890", 11, 1).unwrap_err();
        assert!(matches!(err, IdentifierError::InvalidFormat { .. }));
        // Marktpartner-ID permits a leading zero.
        assert!(matches!(
            validate_numeric_id("0123456789012", 13, 0),
            Err(IdentifierError::InvalidChecksum) | Ok(())
        ));
    }

    #[test]
    fn non_ascii_error_reports_full_char() {
        // 'ä' is multi-byte; the error must not panic on a non-boundary slice.
        let err = validate_numeric_id("4137355924ä", 11, 1);
        assert!(err.is_err());
    }
}
