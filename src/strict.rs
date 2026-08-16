//! Strict-decoding support for BO4E payloads.
//!
//! Every BO4E enum carries an `Unknown` forward-compatibility catch-all, so the
//! lenient `serde` deserialization path never fails on an unrecognized wire value
//! — it maps to `Unknown`. That is the right default for forward-compatibility,
//! but the wrong default at an **ingest boundary** that must reject typos, legacy
//! codes, or values from a newer schema.
//!
//! The [`Bo4eStrict`](crate::Bo4eStrict) trait — implemented by every generated BO,
//! COM, enum, and by `AnyBo` — walks a value **recursively** and reports the
//! JSON-path of every enum field that decoded to `Unknown`. This turns the MaKo
//! "round-trip as validation" pattern into an actually-strict one:
//!
//! ```
//! # #[cfg(feature = "json")] {
//! use rubo4e::{Bo4eStrict, current::Messlokation};
//!
//! // `sparte` carries a value this schema version does not define.
//! let body = r#"{"messlokationsId":"DE0123456789012345678901234567890","sparte":"PLASMA"}"#;
//! let melo: Messlokation = serde_json::from_str(body).unwrap();  // lenient decode
//!
//! // One call finds it, wherever it sits in the tree:
//! let err = melo.ensure_known_enums().unwrap_err();
//! assert_eq!(err.paths, ["sparte"]);
//! # }
//! ```
//!
//! One call replaces the hand-written `field == T::Unknown` checks scattered across
//! every handler.

/// The set of JSON-paths at which a payload holds an out-of-schema (`Unknown`)
/// enum value, produced by [`Bo4eStrict::ensure_known_enums`](crate::Bo4eStrict::ensure_known_enums).
///
/// Paths are dotted, with array indices in brackets, relative to the value that
/// was checked — e.g. `zaehler[0].zaehlertyp` or `bilanzierung.aggregationsverantwortung`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StrictError {
    /// JSON-paths of every field that holds the `Unknown` catch-all.
    pub paths: Vec<String>,
}

impl std::fmt::Display for StrictError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} enum field(s) hold out-of-schema (Unknown) values: {}",
            self.paths.len(),
            self.paths.join(", ")
        )
    }
}

impl std::error::Error for StrictError {}

#[cfg(feature = "validate")]
impl From<StrictError> for garde::Error {
    fn from(e: StrictError) -> Self {
        garde::Error::new(e.to_string())
    }
}

/// Joins a child field name onto a parent JSON-path.
///
/// Used by generated [`Bo4eStrict`](crate::Bo4eStrict) impls; rarely called
/// directly. The root path is the empty string, so the first segment carries no
/// leading dot.
#[inline]
pub fn field_path(parent: &str, field: &str) -> String {
    if parent.is_empty() {
        field.to_owned()
    } else {
        format!("{parent}.{field}")
    }
}

/// Joins an array index onto a parent JSON-path (`parent[i]`).
///
/// Used by generated [`Bo4eStrict`](crate::Bo4eStrict) impls; rarely called directly.
#[inline]
pub fn index_path(parent: &str, index: usize) -> String {
    format!("{parent}[{index}]")
}
