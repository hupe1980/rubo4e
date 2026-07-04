use std::borrow::Cow;

use thiserror::Error;

/// Expected input-length contract for an identifier type.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LengthExpectation {
    /// Identifier must have exactly this number of characters.
    Exact(usize),
    /// Identifier must have a number of characters inside this inclusive range.
    RangeInclusive {
        /// Minimum accepted number of characters (inclusive).
        min: usize,
        /// Maximum accepted number of characters (inclusive).
        max: usize,
    },
}

impl std::fmt::Display for LengthExpectation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Exact(n) => write!(f, "exactly {n}"),
            Self::RangeInclusive { min, max } => write!(f, "{min}..={max}"),
        }
    }
}

/// Errors that can occur when constructing or validating an identifier.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum IdentifierError {
    /// The input has a wrong number of characters.
    #[error("invalid length: expected {expected}, got {actual}")]
    InvalidLength {
        /// The accepted length contract for this identifier type.
        expected: LengthExpectation,
        /// The actual number of characters in the input.
        actual: usize,
    },

    /// A character at the given byte position is not permitted.
    #[error("invalid character {character:?} at position {position}")]
    InvalidCharacter {
        /// Zero-based character index of the offending character.
        position: usize,
        /// The offending character.
        character: char,
    },

    /// The check digit or check character does not match.
    #[error("invalid checksum")]
    InvalidChecksum,

    /// The overall structure of the input does not conform to the expected format.
    #[error("invalid format: {description}")]
    InvalidFormat {
        /// Human-readable explanation of what is wrong.
        ///
        /// `Cow<'static, str>` allows both compile-time constant messages and
        /// runtime-constructed strings that include the actual invalid data.
        description: Cow<'static, str>,
    },
}

#[cfg(feature = "validate")]
impl From<IdentifierError> for garde::Error {
    /// Converts an [`IdentifierError`] into a [`garde::Error`].
    ///
    /// `InvalidChecksum` maps to a `'static` string (zero allocation).
    /// All other variants use `Display` (one allocation instead of two).
    fn from(e: IdentifierError) -> Self {
        match e {
            IdentifierError::InvalidChecksum => garde::Error::new("invalid checksum"),
            other => garde::Error::new(other.to_string()),
        }
    }
}
