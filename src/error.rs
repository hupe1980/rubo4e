use std::borrow::Cow;

use thiserror::Error;

/// Expected input-length contract for an identifier type, in bytes.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LengthExpectation {
    /// Identifier must be exactly this many bytes long.
    Exact(usize),
    /// Identifier length must fall inside this inclusive byte range.
    RangeInclusive {
        /// Minimum accepted length in bytes (inclusive).
        min: usize,
        /// Maximum accepted length in bytes (inclusive).
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
    /// The input has a wrong length.
    ///
    /// Lengths are measured in **bytes**. Every identifier this crate defines is
    /// ASCII-only, so for any input that could have been valid this equals the
    /// character count; a non-ASCII input is rejected either way.
    #[error("invalid length: expected {expected}, got {actual}")]
    InvalidLength {
        /// The accepted length contract for this identifier type.
        expected: LengthExpectation,
        /// The actual length of the input, in bytes.
        actual: usize,
    },

    /// A character at the given position is not permitted.
    #[error("invalid character {character:?} at position {position}")]
    InvalidCharacter {
        /// Zero-based **byte** offset of the offending character within the input.
        ///
        /// Byte offset rather than character index so the value can be used to
        /// slice the original input directly. The two coincide for the ASCII
        /// prefix that every identifier requires.
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

/// Error returned by an enum's `from_wire` **strict** parser when the input
/// string does not correspond to any variant defined in this schema version.
///
/// Unlike the `serde` / `FromStr` deserialization path — which maps every
/// unrecognized value to the forward-compatibility `Unknown` catch-all — the
/// generated `T::from_wire` associated function returns this error for values
/// that are not part of the current BO4E schema.  Use it at the ingest boundary
/// to reject typos, legacy codes, or values from a newer schema, instead of
/// having them silently degrade to `Unknown`.
///
/// # Example
/// ```
/// # #[cfg(feature = "versioned")] {
/// use rubo4e::current::Marktrolle;
///
/// assert!(Marktrolle::from_wire("LF").is_ok());
/// // Legacy / typo'd values are rejected rather than silently accepted:
/// assert!(Marktrolle::from_wire("LFG").is_err());
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("unknown enum value {value:?}: not a variant defined in this BO4E schema version")]
pub struct UnknownVariant {
    /// The unrecognized wire value that was rejected.
    pub value: String,
}

impl UnknownVariant {
    /// Constructs an [`UnknownVariant`] from the offending wire value.
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
}

#[cfg(feature = "validate")]
impl From<UnknownVariant> for garde::Error {
    /// Converts an [`UnknownVariant`] into a [`garde::Error`] so strict enum
    /// parsing can participate in `garde`-driven validation pipelines.
    fn from(e: UnknownVariant) -> Self {
        garde::Error::new(e.to_string())
    }
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
