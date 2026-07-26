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

/// Strict-decoding support: reject out-of-schema (`Unknown`) enum values anywhere
/// in a deserialized payload. See [`Bo4eStrict`] and [`strict::StrictError`].
#[cfg(feature = "versioned")]
#[cfg_attr(docsrs, doc(cfg(feature = "versioned")))]
pub mod strict;

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

/// Current stable BO4E schema version — always resolves to the latest stable schema
/// (`v202607` in this release).
///
/// Prefer `use rubo4e::current::Foo` over `use rubo4e::v202607::Foo` so that
/// imports remain valid across schema version bumps without any code changes.
///
/// # rust-analyzer / IDE note
///
/// Because this is a true `pub mod` (not a `pub use … as` alias), IDE tooling
/// and rust-analyzer resolve hover types and auto-import suggestions as
/// `rubo4e::current::Foo` rather than the underlying versioned path.
#[cfg(feature = "versioned")]
#[cfg_attr(docsrs, doc(cfg(feature = "versioned")))]
pub mod current {
    pub use crate::generated::v202607::*;
}

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

/// Uniform introspection & strict-parsing surface implemented by **every**
/// generated BO4E enum (`Zaehlertyp`, `Marktrolle`, `BdewArtikelnummer`, …).
///
/// Every BO4E enum carries an `Unknown` forward-compatibility catch-all, so the
/// `serde` / `FromStr` deserialization path never fails on an unrecognized wire
/// value — it silently maps to `Unknown`.  That is the right default for
/// forward-compatibility, but the wrong default at an ingest boundary that must
/// reject typos, legacy codes, or values from a newer schema.  This trait gives
/// every enum, **without requiring the `strum` feature**:
///
/// - [`VARIANTS`](Bo4eEnum::VARIANTS) / [`COUNT`](Bo4eEnum::COUNT) — the known
///   variants and their count, for drift-guarding SQL `CHECK` lists and mappings.
/// - [`as_wire`](Bo4eEnum::as_wire) — the canonical BO4E wire string.
/// - [`from_wire`](Bo4eEnum::from_wire) — **strict** parsing that returns
///   [`Err`](crate::error::UnknownVariant) for out-of-schema values instead of
///   yielding `Unknown`.
/// - [`is_known`](Bo4eEnum::is_known) / [`is_unknown`](Bo4eEnum::is_unknown) —
///   detect a value that fell through to the `Unknown` catch-all after a lenient
///   `serde` round-trip.
///
/// The same members are also available as inherent methods on each enum, so you
/// rarely need to import this trait unless you are writing code generic over the
/// enum type (e.g. a generic `fn assert_covered<T: Bo4eEnum>()` that proves a
/// database `CHECK` list covers `T::VARIANTS`).
///
/// # Example
/// ```rust,ignore
/// use rubo4e::{Bo4eEnum, current::Zaehlertyp};
///
/// // Introspection without `strum`:
/// assert_eq!(Zaehlertyp::COUNT, Zaehlertyp::VARIANTS.len());
///
/// // Strict parsing at the ingest boundary:
/// assert_eq!(Zaehlertyp::from_wire("WASSERZAEHLER"), Ok(Zaehlertyp::Wasserzaehler));
/// assert!(Zaehlertyp::from_wire("NOT_A_REAL_VALUE").is_err());
///
/// // Detect lenient-decode fall-through:
/// let z: Zaehlertyp = serde_json::from_value(serde_json::json!("BOGUS")).unwrap();
/// assert!(z.is_unknown());
/// ```
///
/// # Sealed trait
///
/// `Bo4eEnum` is sealed — it cannot be implemented outside this crate.
#[cfg(feature = "versioned")]
#[cfg_attr(docsrs, doc(cfg(feature = "versioned")))]
pub trait Bo4eEnum: bo4e_enum_sealed::Sealed + Copy + Sized + 'static {
    /// All variants defined by the BO4E schema, in declaration order.
    ///
    /// Excludes the synthetic [`Unknown`](Bo4eEnum::is_unknown) catch-all, so
    /// this is exactly the set of values that appear on the wire.
    const VARIANTS: &'static [Self];

    /// Number of schema-defined variants (equal to `VARIANTS.len()`), excluding
    /// the `Unknown` catch-all.  Stable for a given schema version — use it to
    /// drift-guard hand-maintained variant counts.
    const COUNT: usize;

    /// Returns the canonical BO4E wire string (SCREAMING_SNAKE_CASE) for this value.
    fn as_wire(&self) -> &'static str;

    /// Strictly parses a BO4E wire string, returning
    /// [`Err`](crate::error::UnknownVariant) for any value not defined in this
    /// schema version — including the literal `"UNKNOWN"`.
    ///
    /// This is the opt-in strict counterpart to the lenient `serde` / `FromStr`
    /// path, which maps unrecognized values to the `Unknown` catch-all.
    fn from_wire(s: &str) -> Result<Self, crate::error::UnknownVariant>;

    /// Returns `true` if this value is the forward-compatibility `Unknown`
    /// catch-all — i.e. an out-of-schema value produced by a lenient decode.
    fn is_unknown(&self) -> bool;

    /// Returns `true` if this value is a known, schema-defined variant.
    fn is_known(&self) -> bool {
        !self.is_unknown()
    }
}

#[cfg(feature = "versioned")]
#[doc(hidden)]
pub mod bo4e_enum_sealed {
    /// Sealing supertrait for [`crate::Bo4eEnum`].
    ///
    /// Only generated enum types carry this impl.  External crates cannot
    /// implement `Bo4eEnum` because `Sealed` is not accessible outside `rubo4e`.
    pub trait Sealed {}
}

/// Recursive strict-decode check: find every out-of-schema (`Unknown`) enum value
/// anywhere inside a deserialized BO4E value.
///
/// Implemented by **every** generated BO, COM, and enum, and by `AnyBo`. Because
/// the `serde` path is intentionally lenient (unknown wire values decode to
/// `Unknown` for forward-compatibility), this trait is how you make the popular
/// "round-trip as validation" pattern *actually* strict at an ingest boundary,
/// with a single call instead of hand-checking every enum field:
///
/// ```rust,ignore
/// use rubo4e::{Bo4eStrict, current::Netzlokation};
///
/// let nelo: Netzlokation = serde_json::from_value(body)?;   // lenient decode
/// nelo.ensure_known_enums()?;                               // 422 if any Unknown, anywhere
/// ```
///
/// Paths are reported relative to the checked value, using dotted field names and
/// bracketed array indices (e.g. `zaehler[0].zaehlertyp`). The field names are the
/// BO4E wire (camelCase) names, matching the JSON you deserialized from.
///
/// # Not sealed
///
/// Unlike [`Bo4eObject`] and [`Bo4eEnum`], `Bo4eStrict` is intentionally **not**
/// sealed: downstream crates that wrap BO4E types in their own domain types can
/// implement it to make their wrappers participate in the same recursive check.
#[cfg(feature = "versioned")]
#[cfg_attr(docsrs, doc(cfg(feature = "versioned")))]
pub trait Bo4eStrict {
    /// Appends, to `out`, the [`field_path`](crate::strict::field_path)-formatted
    /// location of every enum field in `self` (recursively) that holds the
    /// forward-compatibility `Unknown` catch-all. `path` is the JSON-path of
    /// `self` relative to the root of the check (`""` at the top level).
    ///
    /// This is the low-level primitive; prefer [`ensure_known_enums`] or
    /// [`unknown_enum_paths`] unless you are composing the walk yourself.
    ///
    /// [`ensure_known_enums`]: Bo4eStrict::ensure_known_enums
    /// [`unknown_enum_paths`]: Bo4eStrict::unknown_enum_paths
    fn collect_unknown_enums(&self, path: &str, out: &mut Vec<String>);

    /// Returns the JSON-paths of every enum field (recursively) that decoded to
    /// `Unknown`. Empty when the value is fully in-schema.
    fn unknown_enum_paths(&self) -> Vec<String> {
        let mut out = Vec::new();
        self.collect_unknown_enums("", &mut out);
        out
    }

    /// Returns `Err(`[`StrictError`](crate::strict::StrictError)`)` listing every
    /// out-of-schema (`Unknown`) enum value in `self`, or `Ok(())` if there are
    /// none. Call this after a lenient `serde` decode to reject payloads that
    /// carry typos, legacy codes, or values from a newer schema.
    fn ensure_known_enums(&self) -> Result<(), crate::strict::StrictError> {
        let paths = self.unknown_enum_paths();
        if paths.is_empty() {
            Ok(())
        } else {
            Err(crate::strict::StrictError { paths })
        }
    }
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
    pub use crate::error::{IdentifierError, UnknownVariant};
    pub use crate::identifiers::{
        AkivId, BilanzkreisId, EicCode, EicDomain, MaloId, MarktpartnerId, MeloId, NeloId,
        ObisCode, ObisComponents, SrId, TrId, TranchennummerId,
    };

    /// Uniform enum introspection + strict parsing (`VARIANTS`, `from_wire`, …).
    #[cfg(feature = "versioned")]
    pub use crate::Bo4eEnum;

    /// Recursive strict-decode check (`ensure_known_enums`, `unknown_enum_paths`).
    #[cfg(feature = "versioned")]
    pub use crate::Bo4eStrict;

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
