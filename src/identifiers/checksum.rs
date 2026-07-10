//! Shared BDEW checksum algorithm for MaloId, SrId, and TrId.

/// Computes the BDEW alternating-weight check digit used by Marktlokations-ID,
/// Steuerbare-Ressource-ID, and Technische-Ressource-ID.
///
/// Algorithm:
/// 1. Apply weights `[2, 1, 2, 1, …]` to the 10 input digit values.
/// 2. Products ≥ 10 are reduced by subtracting 9.
/// 3. `check = (10 − (Σ % 10)) % 10`.
///
/// Exposed `pub(super)` so that `MaloId`, `SrId`, and `TrId` can call it from
/// their `from_base` constructors.
pub(super) fn bdew_check_digit(digits: &[u8; 10]) -> u8 {
    const WEIGHTS: [u8; 10] = [2, 1, 2, 1, 2, 1, 2, 1, 2, 1];
    let sum: u32 = digits
        .iter()
        .zip(WEIGHTS.iter())
        .map(|(&d, &w)| {
            let p = u32::from(d) * u32::from(w);
            if p >= 10 {
                p - 9
            } else {
                p
            }
        })
        .sum();
    ((10 - (sum % 10)) % 10) as u8
}

/// Validates an 11-digit string against the BDEW check-digit algorithm.
pub(super) fn validate_11digit_bdew(s: &str) -> Result<(), crate::error::IdentifierError> {
    use crate::error::{IdentifierError, LengthExpectation};

    if s.len() != 11 {
        return Err(IdentifierError::InvalidLength {
            expected: LengthExpectation::Exact(11),
            actual: s.len(),
        });
    }
    let mut digits = [0u8; 11];
    for (i, c) in s.chars().enumerate() {
        match c.to_digit(10) {
            Some(d) => digits[i] = d as u8,
            None => {
                return Err(IdentifierError::InvalidCharacter {
                    position: i,
                    character: c,
                })
            }
        }
    }
    let expected = bdew_check_digit(
        digits[..10]
            .try_into()
            .expect("slice has exactly 10 elements; checked above"),
    );
    if digits[10] != expected {
        return Err(IdentifierError::InvalidChecksum);
    }
    Ok(())
}

// ─── Test helpers (available to sibling modules in #[cfg(test)]) ─────────────

#[cfg(test)]
pub(super) fn make_valid_11digit(prefix: &[u8; 10]) -> String {
    let check = bdew_check_digit(prefix);
    prefix
        .iter()
        .chain(std::iter::once(&check))
        .map(|&d| char::from_digit(u32::from(d), 10).unwrap())
        .collect()
}

// ─── Public construction helpers ─────────────────────────────────────────────

/// Parses a 10-ASCII-digit base string, computes the BDEW check digit, and
/// returns the full 11-character ID string.
///
/// # Errors
/// - [`IdentifierError::InvalidLength`] if `base` is not exactly 10 characters.
/// - [`IdentifierError::InvalidCharacter`] if any character is not a decimal digit.
pub(super) fn compute_11digit_from_base(
    base: &str,
) -> Result<String, crate::error::IdentifierError> {
    use crate::error::{IdentifierError, LengthExpectation};

    if base.len() != 10 {
        return Err(IdentifierError::InvalidLength {
            expected: LengthExpectation::Exact(10),
            actual: base.len(),
        });
    }
    let mut digits = [0u8; 10];
    for (i, c) in base.chars().enumerate() {
        match c.to_digit(10) {
            Some(d) => digits[i] = d as u8,
            None => {
                return Err(IdentifierError::InvalidCharacter {
                    position: i,
                    character: c,
                })
            }
        }
    }
    let check = bdew_check_digit(&digits);
    let mut result = base.to_owned();
    result.push(char::from_digit(u32::from(check), 10).expect("check digit is 0..=9"));
    Ok(result)
}

// ─── ASCII-Verfahren (NeLo-ID, SR-ID, TR-ID, NeBe-ID, Paket-ID) ─────────────

/// Maps a byte to its numeric value for the BDEW ASCII-Verfahren (§8.2).
///
/// - Uppercase letters (`A`–`Z`): raw ASCII code value (A = 65 … Z = 90).
/// - Decimal digits (`0`–`9`): numeric value (0–9).
#[inline]
fn ascii_val(b: u8) -> u32 {
    if b.is_ascii_digit() {
        u32::from(b - b'0')
    } else {
        u32::from(b)
    }
}

/// Computes the BDEW ASCII-Verfahren check digit for a 10-byte base.
///
/// Used by NeLo-ID, NeBe-ID, Ressourcen-ID (TR-ID, SR-ID, SG-ID, CR-ID),
/// and Paket-ID.
///
/// Algorithm (BDEW "Identifikatoren in der Marktkommunikation" v1.2, §8.2):
/// 1. Map each character: digit → numeric value (0–9); letter → ASCII code value.
/// 2. Sum mapped values at **odd** positions (1-indexed: 1, 3, 5, 7, 9
///    → 0-indexed: 0, 2, 4, 6, 8).
/// 3. Sum mapped values at **even** positions (1-indexed: 2, 4, 6, 8, 10
///    → 0-indexed: 1, 3, 5, 7, 9), multiply by 2.
/// 4. check digit = (10 − ((step 2 + step 3) % 10)) % 10.
///
/// **Reference example** (§8.2): base `A113735592` → check digit `5`.
pub(super) fn ascii_check_digit(base: &[u8; 10]) -> u8 {
    let odd_sum: u32 = base.iter().step_by(2).map(|&b| ascii_val(b)).sum();
    let even_sum: u32 = base.iter().skip(1).step_by(2).map(|&b| ascii_val(b)).sum();
    ((10 - ((odd_sum + even_sum * 2) % 10)) % 10) as u8
}

/// Validates an 11-character alphanumeric BDEW ID using the ASCII-Verfahren.
///
/// Constraints:
/// - Length must be exactly 11.
/// - `s[0]` must equal `type_char` (Codetyp).
/// - `s[1..=9]` must be `[A-Z0-9]` (uppercase alphanumeric).
/// - `s[10]` must be a decimal digit `[0-9]` (the check digit).
/// - The check digit must match the ASCII-Verfahren result for the 10-byte base.
pub(super) fn validate_ascii_id(
    s: &str,
    type_char: u8,
) -> Result<(), crate::error::IdentifierError> {
    use crate::error::{IdentifierError, LengthExpectation};

    if s.len() != 11 {
        return Err(IdentifierError::InvalidLength {
            expected: LengthExpectation::Exact(11),
            actual: s.len(),
        });
    }
    let bytes = s.as_bytes();
    if bytes[0] != type_char {
        return Err(IdentifierError::InvalidFormat {
            description: format!(
                "first character (Codetyp) must be '{}', got '{}'",
                type_char as char, bytes[0] as char,
            )
            .into(),
        });
    }
    for (i, &b) in bytes.iter().enumerate().skip(1).take(9) {
        if !b.is_ascii_uppercase() && !b.is_ascii_digit() {
            return Err(IdentifierError::InvalidCharacter {
                position: i,
                character: b as char,
            });
        }
    }
    let last = bytes[10];
    if !last.is_ascii_digit() {
        return Err(IdentifierError::InvalidCharacter {
            position: 10,
            character: last as char,
        });
    }
    let base_arr: [u8; 10] = bytes[..10].try_into().expect("verified 10 bytes above");
    if last - b'0' != ascii_check_digit(&base_arr) {
        return Err(IdentifierError::InvalidChecksum);
    }
    Ok(())
}

/// Constructs a valid 11-character ASCII-Verfahren ID from a 10-character base.
///
/// - `base[0]` must equal `type_char` (Codetyp).
/// - `base[1..=9]` must be `[A-Z0-9]` (uppercase alphanumeric).
///
/// Returns the full 11-character string (base + computed check digit).
pub(super) fn compute_ascii_id_from_base(
    base: &str,
    type_char: u8,
) -> Result<String, crate::error::IdentifierError> {
    use crate::error::{IdentifierError, LengthExpectation};

    if base.len() != 10 {
        return Err(IdentifierError::InvalidLength {
            expected: LengthExpectation::Exact(10),
            actual: base.len(),
        });
    }
    let bytes = base.as_bytes();
    if bytes[0] != type_char {
        return Err(IdentifierError::InvalidFormat {
            description: format!(
                "base must start with '{}' (Codetyp), got '{}'",
                type_char as char, bytes[0] as char,
            )
            .into(),
        });
    }
    for (i, &b) in bytes.iter().enumerate().skip(1).take(9) {
        if !b.is_ascii_uppercase() && !b.is_ascii_digit() {
            return Err(IdentifierError::InvalidCharacter {
                position: i,
                character: b as char,
            });
        }
    }
    let arr: [u8; 10] = bytes.try_into().expect("verified 10 bytes above");
    let check = ascii_check_digit(&arr);
    let mut result = base.to_owned();
    result.push(char::from_digit(u32::from(check), 10).expect("check digit is 0..=9"));
    Ok(result)
}

#[cfg(test)]
pub(super) fn make_valid_ascii_id(type_char: u8, body: &[u8; 9]) -> String {
    let mut base = [0u8; 10];
    base[0] = type_char;
    base[1..].copy_from_slice(body);
    let check = ascii_check_digit(&base);
    let mut s = String::with_capacity(11);
    for &b in &base {
        s.push(b as char);
    }
    s.push(char::from_digit(u32::from(check), 10).unwrap());
    s
}
