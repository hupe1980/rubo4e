#![deny(unsafe_code)]
#![warn(missing_docs, clippy::all)]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! # bo4e
//!
//! Rust implementation of the **BO4E** energy-market data standard.
//!
//! ## Feature gates
//!
//! | Feature        | Default | Description                                                    |
//! |----------------|---------|----------------------------------------------------------------|
//! | `identifiers`  | ✓       | Identifier types (`MaloId`, `EicCode`, `ObisCode`, …) + serde  |
//! | `serde`        | ✓       | Serde derives + extension-data map                             |
//! | `json`         |         | `serde_json` helpers (`to_json_*`, `from_json_*`)              |
//! | `simd-json`    |         | SIMD parser backend for `from_json_*` (workload-dependent)    |
//! | `time`         |         | `time` crate for timestamps                                    |
//! | `decimal`      |         | `rust_decimal::Decimal` for amounts/prices (see note below)   |
//! | `builder`      |         | `typed-builder` derives with `setter(into)` — accepts both `T` and `Option<T>`  |
//! | `validate`     |         | `garde` validation                                             |
//! | `schemars`     |         | JSON Schema generation                                         |
//! | `sqlx`         |         | `sqlx` type integrations                                       |
//! | `utoipa`       |         | `utoipa` OpenAPI integrations                                  |
//! | `strum`        |         | Enum iteration and string conversion                           |
//! | `versioned`    |         | Expose the versioned schema module (`v202607`)                 |
//! | `tracing`      |         | Structured diagnostics via the `tracing` crate                 |
//! | `metrics`      |         | Optional export hooks via the `metrics` crate                  |
//!
//! ## Identifiers without schema overhead
//!
//! All identifier types (`MaloId`, `MeloId`, `NeloId`, `EicCode`, `ObisCode`,
//! `MarktpartnerId`, `SrId`, `TrId`) **always** provide `Display`, `FromStr`,
//! `TryFrom<&str>`, `TryFrom<String>`, and `AsRef<str>` without any feature
//! flags — the minimum needed for EDIFACT wire-format encoding/decoding.
//!
//! To use only identifier types without pulling in the versioned BO4E schema:
//! ```toml
//! rubo4e = { version = "...", default-features = false, features = ["identifiers"] }
//! ```
//! This gives `serde` support on all identifiers with zero versioned-schema overhead.
//!
//! ## `serde` is enabled by default
//!
//! The `serde` feature is included in `default = ["serde"]`.  Targets that only
//! need the type definitions for in-memory processing can opt out:
//! ```toml
//! rubo4e = { version = "...", default-features = false, features = ["versioned"] }
//! ```
//!
//! ## Feature-conditional field types (`decimal` and `time`)
//!
//! Enabling `decimal` or `time` **changes the Rust type** of certain struct fields:
//!
//! | Feature   | Without feature     | With feature                                        | Affected fields                        |
//! |-----------|---------------------|-----------------------------------------------------|----------------------------------------|
//! | `decimal` | `Option<String>`    | `Option<rust_decimal::Decimal>`                     | `wert`, `preis`, amounts, quantities   |
//! | `time`    | `Option<String>`    | `Option<time::OffsetDateTime>` or `Option<time::Date>` | `beginn`/`ende` fields → `OffsetDateTime`; `*datum` fields → `Date` |
//!
//! This means code that compiles under one feature configuration may not compile
//! under the other.  For code that must be feature-agnostic, either:
//! - Always enable `decimal`/`time` and use the strong types, **or**
//! - Access fields through JSON round-trip (`to_json_german` / `from_json_german`)
//!   which is feature-independent.
//!
//! The string fallback preserves the ISO-8601 / decimal string value from JSON
//! so data is never lost when these features are absent.
//!
//! ## Why generated structs do not implement `Eq`
//!
//! Generated BO and COM structs derive `PartialEq` but **not `Eq`**.  The
//! `_additional` extension-data field (present when the `json` feature is active)
//! has type `LimitedExtensionMap` whose inner map contains `serde_json::Value`.
//! `serde_json::Value` does not implement `Eq` because it wraps `f64` (JSON
//! numbers), and `f64` is not `Eq` (`NaN ≠ NaN`).  This is intentional and
//! correct behaviour.
//!
//! For content-addressed equality comparisons, use `to_json_canonical()`
//! (from `Bo4eJsonExt` in the `json` module) which produces a deterministic
//! byte string that can be compared with `==`.

/// Error types returned by identifier construction.
pub mod error;
pub mod identifiers;

/// JSON serialization helpers: `json::Bo4eJsonExt` with `to_json_german()`,
/// `to_json_snake_case()`, and `to_json_canonical()`.
#[cfg(feature = "json")]
#[cfg_attr(docsrs, doc(cfg(feature = "json")))]
pub mod json;

/// Always-available re-export of `json::extension::LimitedExtensionMap`.
///
/// When the `json` feature is **active** this is the real DoS-hardened extension
/// map.  When `json` is **inactive** it degrades to a zero-sized stub that is
/// `Debug + Clone + Default + PartialEq` but carries no data.  All generated
/// BO/COM structs use `crate::LimitedExtensionMap` as their `_additional` field
/// type so that the field can be declared once without a `#[cfg]` branch.
#[cfg(feature = "json")]
#[cfg_attr(docsrs, doc(cfg(feature = "json")))]
pub use json::extension::LimitedExtensionMap;

/// Zero-sized stub used when the `json` feature is disabled.
///
/// See the `json`-feature variant for the full description.
#[cfg(not(feature = "json"))]
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
#[doc(hidden)]
pub struct LimitedExtensionMap;

/// Cross-field business-rule validators for BO4E types (requires `validate` + `versioned`).
/// Also exports `Validated<T>` which only requires `validate`.
#[cfg(feature = "validate")]
#[cfg_attr(docsrs, doc(cfg(feature = "validate")))]
pub mod validation;

/// Schema helper functions used by generated schemars attributes.
///
/// Provides `"format": "date-time"` and `"format": "date"` JSON Schema
/// annotations for `time::OffsetDateTime` and `time::Date` fields, which
/// schemars 1.x does not emit automatically.
#[cfg(feature = "schemars")]
#[cfg_attr(docsrs, doc(cfg(feature = "schemars")))]
pub mod schema_helpers;

/// Serde modules for `time::Date` fields in generated structs.
///
/// - [`time_serde::date_serde`] — required `time::Date` ↔ `"YYYY-MM-DD"`
/// - [`time_serde::opt_date_serde`] — `Option<time::Date>` ↔ `"YYYY-MM-DD"` or `null`
///
/// These are referenced from generated code via
/// `#[serde(with = "crate::time_serde::opt_date_serde")]`.
#[cfg(feature = "time")]
#[cfg_attr(docsrs, doc(cfg(feature = "time")))]
pub mod time_serde;

// Versioned schema modules — emitted by the generator; gated behind `versioned`.
// Run `just generate` to populate or refresh these modules.
#[cfg(feature = "versioned")]
#[allow(missing_docs)]
mod generated;

/// Hand-written convenience methods on generated BO4E types.
///
/// Provides ergonomic accessors such as [`Zeitraum::as_closed_range`],
/// [`Rechnung::billing_period`], and [`PreisblattNetznutzung::validity`].
/// All methods are gated on the feature flags that make their return types
/// available (`versioned`, `time`, `decimal`).
#[cfg(feature = "versioned")]
#[cfg_attr(docsrs, doc(cfg(feature = "versioned")))]
pub mod convenience;

/// BO4E schema v202607 types.
#[cfg(feature = "versioned")]
#[cfg_attr(docsrs, doc(cfg(feature = "versioned")))]
pub mod v202607 {
    pub use crate::generated::v202607::*;
}

/// Alias to the current BO4E schema version (`v202607`).
#[cfg(feature = "versioned")]
pub use v202607 as current;

#[cfg(feature = "versioned")]
/// Marker trait implemented by every generated BO4E business object (Geschäftsobjekt).
///
/// Provides runtime access to the BO type discriminant and the schema version that
/// was used to generate this type.  COM types and enums do NOT implement this trait.
///
/// # Sealed trait
///
/// `Bo4eObject` is sealed — it cannot be implemented by types outside this crate.
/// This allows the library to add new methods in future releases without breaking
/// downstream code that merely *uses* the trait.
///
/// # Design note — associated type over bare return type
///
/// `bo_type()` returns `Self::BoTyp` (an associated type) so that the single trait
/// definition in `src/lib.rs` can serve all schema versions while keeping each
/// version's `BoTyp` enum strongly typed.  For `dyn` usage, bind the associated type:
///
/// ```rust,ignore
/// use rubo4e::v202607::BoTyp;
/// let objects: Vec<Box<dyn rubo4e::Bo4eObject<BoTyp = BoTyp>>> = vec![
///     Box::new(Vertrag::default()),
///     Box::new(Marktlokation::default()),
/// ];
/// for obj in &objects {
///     println!("{:?} schema={}", obj.bo_type(), obj.schema_version());
/// }
/// ```
///
/// # Example
/// ```rust,ignore
/// use rubo4e::prelude::*;
/// let v = Vertrag::default();
/// assert_eq!(v.bo_type(), BoTyp::Vertrag);
/// assert_eq!(v.schema_version(), "v202607.0.0");
/// ```
pub trait Bo4eObject: bo4e_object_sealed::Sealed {
    /// The BO type discriminant enum for this schema version (e.g. `v202607::BoTyp`).
    type BoTyp;
    /// Returns the [`Self::BoTyp`] discriminant identifying this business object.
    fn bo_type(&self) -> Self::BoTyp;
    /// Returns the BO4E schema version tag used to generate this type (e.g. `"v202607.0.0"`).
    fn schema_version(&self) -> &'static str;
}

#[cfg(feature = "versioned")]
#[doc(hidden)]
pub mod bo4e_object_sealed {
    /// Sealing supertrait for [`crate::Bo4eObject`].
    ///
    /// Only generated BO types carry this impl.  External crates cannot implement
    /// `Bo4eObject` because `Sealed` is not accessible outside `rubo4e`.
    pub trait Sealed {}
}

/// Re-exports the most commonly used types.
///
/// `use rubo4e::prelude::*;` gives you all identifiers, the `Bo4eJsonExt` trait
/// (when `json` feature is active), the [`Bo4eObject`] marker trait (when
/// `versioned` feature is active), and the ergonomic COM extension traits
/// [`BetragExt`](crate::convenience::BetragExt),
/// [`MengeExt`](crate::convenience::MengeExt),
/// [`PreisExt`](crate::convenience::PreisExt)
/// (when `versioned` + `decimal` features are active).
pub mod prelude {
    pub use crate::error::IdentifierError;
    pub use crate::identifiers::{
        EicCode, EicDomain, MaloId, MarktpartnerId, MeloId, NeloId, ObisCode, ObisComponents, SrId,
        TrId,
    };

    #[cfg(feature = "validate")]
    pub use crate::validation::Validated;

    #[cfg(feature = "validate")]
    pub use crate::validation::{report_errors, ValidationFailure};

    #[cfg(feature = "json")]
    pub use crate::json::Bo4eExtensionData;

    #[cfg(feature = "json")]
    pub use crate::json::Bo4eJsonExt;

    #[cfg(feature = "versioned")]
    pub use crate::Bo4eObject;

    /// Flatten `Option<Betrag>` → `Option<Decimal>` in one call.
    #[cfg(all(feature = "versioned", feature = "decimal"))]
    pub use crate::convenience::BetragExt;

    /// Flatten `Option<Menge>` → `Option<Decimal>` in one call.
    #[cfg(all(feature = "versioned", feature = "decimal"))]
    pub use crate::convenience::MengeExt;

    /// Flatten `Option<Preis>` → `Option<Decimal>` in one call.
    #[cfg(all(feature = "versioned", feature = "decimal"))]
    pub use crate::convenience::PreisExt;
}
