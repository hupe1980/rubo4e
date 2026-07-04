//! Shared BDEW checksum algorithm for MaloId, SrId, and TrId.

/// Computes the BDEW alternating-weight check digit used by Marktlokations-ID,
/// Steuerbare-Ressource-ID, and Technische-Ressource-ID.
///
/// Algorithm:
/// 1. Apply weights `[2, 1, 2, 1, …]` to the 10 input digit values.
/// 2. Products ≥ 10 are reduced by subtracting 9.
/// 3. `check = (10 − (Σ % 10)) % 10`.
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
