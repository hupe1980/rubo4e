#![deny(unsafe_code)]
#![warn(missing_docs, clippy::all)]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! # rubo4e
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
//! | `time`         |         | `time` crate for timestamps; also `utoipa/time`                |
//! | `decimal`      |         | `rust_decimal::Decimal` for amounts/prices (see note below); also `schemars/rust_decimal1` and `utoipa/decimal` |
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
//! Every identifier type **always** provides `Display`, `FromStr`,
//! `TryFrom<&str>`, `TryFrom<String>`, `Into<String>`, `AsRef<str>`,
//! `Borrow<str>`, and `Deref<Target = str>` without any feature flag — the
//! minimum needed for EDIFACT wire-format encoding and decoding.
//!
//! To use only identifier types without pulling in the versioned BO4E schema:
//! ```toml
//! rubo4e = { version = "...", default-features = false, features = ["identifiers"] }
//! ```
//! This gives `serde` support on all identifiers with zero versioned-schema overhead.
//!
//! ## `serde` is enabled by default
//!
//! The default feature set is `default = ["identifiers"]`, and `identifiers`
//! enables `serde`.  Targets that only need the type definitions for in-memory
//! processing can opt out:
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
//! The string fallback keeps the value's lexical form, so nothing is lost when
//! these features are absent.
//!
//! Decimal fields read a JSON number as well as a JSON string, because BO4E
//! producers use both — but only the string spelling is exact. A number has
//! already passed through `f64` before this crate sees it, losing its scale
//! (`119.00` → `119`) and any precision past ~15 significant digits.
//! [`decimal_serde`] documents the whole picture and counts every such read.
//!
//! ## `Eq` and `Hash` on generated structs
//!
//! Generated BO and COM structs always derive `PartialEq`. They additionally
//! derive **`Eq` and `Hash` when the `json` feature is off**, which is what lets
//! them key a `HashMap` or a `HashSet`.
//!
//! One type blocks both: `serde_json::Value`, which appears in a generated
//! struct twice when `json` is on — inside `LimitedExtensionMap` (the
//! `_additional` field) and as `ZusatzAttribut::wert`. `Value` is neither `Eq`
//! nor `Hash`, because it wraps `f64` and `NaN != NaN`. With `json` off both
//! degrade to a ZST stub and a `String`, and the whole tree becomes `Eq + Hash`.
//!
//! Generated **enums** are always `Eq + Ord + Hash`, whatever the features.
//!
//! For content-addressed equality across every feature set, compare
//! `to_json_canonical()` (from `Bo4eJsonExt` in the `json` module), which
//! produces a deterministic byte string.

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
///
/// The module is pure serde glue, so it needs `serde` as well as `time`; the
/// generated `serde(with = …)` attributes that reference it are themselves
/// gated on `serde`.
#[cfg(all(feature = "time", feature = "serde"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "time", feature = "serde"))))]
pub mod time_serde;

/// Time-of-day parsing for BO4E's `format: "time"` fields.
///
/// `Zeitraum.startuhrzeit`, `.enduhrzeit`, and `Umschaltzeit.umschaltzeit` carry
/// a UTC offset (`"18:00:00+01:00"`), which no `time` type holds alongside a
/// time of day — so the fields stay `String` and this reads them.
#[cfg(feature = "time")]
#[cfg_attr(docsrs, doc(cfg(feature = "time")))]
pub mod offset_time;

/// ISO 8601 duration parsing for BO4E's `dauer` fields.
///
/// `Zeitraum.dauer` is a string like `"P1DT30H4S"` that neither `serde` nor
/// `time` parses. Years and months are refused rather than approximated — see
/// the module docs.
#[cfg(feature = "time")]
#[cfg_attr(docsrs, doc(cfg(feature = "time")))]
pub mod iso8601_duration;

/// Decimal deserialization, and what BO4E's two spellings of a number cost.
///
/// BO4E-python writes `"wert": "119.00"`, go-bo4e writes `"wert": 119.00`.
/// Both are read; only the string spelling is exact. See the module docs, and
/// [`decimal_serde::decimal_from_json_number_count`] for the counter that tells
/// you which one your producers use.
#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
pub mod decimal_serde;

/// Strict-decoding support: reject out-of-schema (`Unknown`) enum values anywhere
/// in a deserialized payload. See [`Bo4eStrict`] and [`strict::StrictError`].
#[cfg(feature = "versioned")]
#[cfg_attr(docsrs, doc(cfg(feature = "versioned")))]
pub mod strict;

// Generator output.  Run `just generate` to populate or refresh it.
//
// This holds two independently gated things: the versioned schema modules
// (`versioned`) and the wire-key ↔ snake_case map the JSON key transforms need
// (`json`).  Neither feature implies the other, so the module is compiled when
// either is on and its contents are gated individually in the generated
// `mod.rs`.
#[cfg(any(feature = "versioned", feature = "json"))]
#[allow(missing_docs)]
mod generated;

/// Hand-written convenience methods on generated BO4E types.
///
/// Provides ergonomic accessors such as [`Zeitraum::as_inclusive_range`][az],
/// [`Rechnung::billing_period`][bp], and [`PreisblattNetznutzung::validity`][va].
///
/// All methods are gated on the feature flags that make their return types
/// available (`versioned`, `time`, `decimal`).
///
/// [az]: crate::current::Zeitraum::as_inclusive_range
/// [bp]: crate::current::Rechnung::billing_period
/// [va]: crate::current::PreisblattNetznutzung::validity
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

/// The `_typ` discriminant, as constants — implemented by every generated BO
/// **and** COM.
///
/// BO4E pins each schema's `_typ` with a `const`, and this exposes it without a
/// value, so a gate at a wire boundary can read the discriminant of a type it
/// only knows generically:
///
/// ```
/// use rubo4e::{current::{Adresse, Marktlokation}, Bo4eTyped};
///
/// fn wire_typ<T: Bo4eTyped>() -> &'static str { T::TYP_WIRE }
///
/// assert_eq!(wire_typ::<Marktlokation>(), "MARKTLOKATION");   // a BO
/// assert_eq!(wire_typ::<Adresse>(), "ADRESSE");               // a COM
/// ```
///
/// [`Bo4eObject`] and [`Bo4eComponent`] narrow this to the Geschäftsobjekte and
/// the components; bind [`Typ`](Bo4eTyped::Typ) through either when you need the
/// discriminant as its own enum rather than as a string.
///
/// Every generated struct implements it except `ZusatzAttribut`, the one BO4E
/// schema that declares no `_typ`. Sealed: not implementable outside this crate.
///
/// # Not `dyn`-compatible
///
/// Associated constants make a trait dyn-incompatible. For a heterogeneous
/// collection of BOs use [`AnyBo`](crate::current::AnyBo), the sum type over
/// exactly [`Bo4eObject`]'s implementors — it carries the same facts and is
/// `Clone + PartialEq + Serialize + Deserialize` besides.
#[cfg(feature = "versioned")]
#[cfg_attr(docsrs, doc(cfg(feature = "versioned")))]
pub trait Bo4eTyped: bo4e_typed_sealed::Sealed {
    /// The discriminant enum this type's `_typ` draws from — `BoTyp` for a
    /// Geschäftsobjekt, `ComTyp` for a component.
    ///
    /// An associated type, so one trait definition serves every schema version
    /// while each version's enums stay strongly typed.
    type Typ: Bo4eEnum;

    /// The [`Typ`](Bo4eTyped::Typ) discriminant for **this Rust type** — the
    /// value the BO4E schema pins with a `const`.
    ///
    /// Not the `_typ` a payload carried. A `Marktlokation` decoded from
    /// `{"_typ":"VERTRAG", …}` is still a `Marktlokation`, and a `match` on the
    /// discriminant must not take the branch the sender named. The `typ` field
    /// stays public for the payload's own claim, so comparing the two finds a
    /// payload whose discriminant disagrees with the type it was read into:
    ///
    /// ```
    /// # #[cfg(feature = "json")] {
    /// use rubo4e::{current::{BoTyp, Marktlokation}, Bo4eTyped};
    ///
    /// let body = r#"{"_typ":"VERTRAG","marktlokationsId":"51238696781"}"#;
    /// let malo: Marktlokation = serde_json::from_str(body).unwrap();
    ///
    /// assert_eq!(Marktlokation::TYP, BoTyp::Marktlokation);   // what it *is*
    /// assert_eq!(malo.typ, Some(BoTyp::Vertrag));             // what it *claimed*
    /// # }
    /// ```
    ///
    /// There is deliberately no `typ()` method beside the `typ` field — two
    /// spellings a keystroke apart with those two meanings is the confusion this
    /// constant exists to prevent. To dispatch on what a payload says it is,
    /// decode [`AnyBo`](crate::current::AnyBo).
    const TYP: Self::Typ;

    /// The `_typ` wire string for this type (e.g. `"MARKTLOKATION"`).
    ///
    /// The same value as `Self::TYP.as_wire()`, without needing [`Bo4eEnum`] in
    /// scope or the concrete discriminant enum in the bound.
    const TYP_WIRE: &'static str;

    /// The exact BO4E schema release this type was generated from, in the
    /// spelling the `_version` wire field carries (e.g. `"202607.1.0"`).
    ///
    /// No `v`: BO4E prefixes its git tags with one, the value inside a payload
    /// never has it, so this compares against a `_version` read off a message
    /// directly.
    ///
    /// **Do not dispatch on it.** BO4E ships patch releases inside a series, so
    /// a producer one patch ahead sends `"202607.2.0"` and an equality match
    /// rejects a payload this module handles. Match on
    /// [`SCHEMA_SERIES`](Bo4eTyped::SCHEMA_SERIES) instead.
    const SCHEMA_VERSION: &'static str;

    /// The schema **series** — the `YYYYMM` prefix of the release, e.g.
    /// `"202607"`.
    ///
    /// The granularity at which this crate exposes a module, and the right key
    /// for version dispatch: every release within a series deserializes into the
    /// same types.
    ///
    /// ```
    /// use rubo4e::{current::Rechnung, Bo4eTyped};
    ///
    /// let incoming = "202607.4.0";   // a sender's own `_version`
    /// assert_eq!(incoming.split('.').next(), Some(Rechnung::SCHEMA_SERIES));
    /// ```
    const SCHEMA_SERIES: &'static str;

    /// Returns [`TYP_WIRE`](Bo4eTyped::TYP_WIRE).
    fn typ_wire(&self) -> &'static str {
        Self::TYP_WIRE
    }

    /// Returns [`SCHEMA_VERSION`](Bo4eTyped::SCHEMA_VERSION).
    fn schema_version(&self) -> &'static str {
        Self::SCHEMA_VERSION
    }

    /// Returns [`SCHEMA_SERIES`](Bo4eTyped::SCHEMA_SERIES).
    fn schema_series(&self) -> &'static str {
        Self::SCHEMA_SERIES
    }
}

/// Marks a generated **Geschäftsobjekt** — the BO4E types that stand on their
/// own as a message payload.
///
/// Everything it carries comes from [`Bo4eTyped`]; this narrows the set, and
/// pins [`Typ`](Bo4eTyped::Typ) to `BoTyp`:
///
/// ```
/// use rubo4e::{current::{BoTyp, Lastgang, Vertrag}, Bo4eObject};
///
/// // No value needed, so no `Default` bound — which is what admits `Lastgang`
/// // and `Tarif`, the two types the schema marks `required`.
/// fn discriminant_of<T: Bo4eObject<Typ = BoTyp>>() -> BoTyp { T::TYP }
///
/// assert_eq!(discriminant_of::<Vertrag>(), BoTyp::Vertrag);
/// assert_eq!(discriminant_of::<Lastgang>(), BoTyp::Lastgang);
/// ```
///
/// Sealed: not implementable outside this crate.
#[cfg(feature = "versioned")]
#[cfg_attr(docsrs, doc(cfg(feature = "versioned")))]
pub trait Bo4eObject: Bo4eTyped + bo4e_object_sealed::Sealed {}

/// Marks a generated **component** (COM) — the BO4E types that appear nested
/// inside a Geschäftsobjekt rather than on their own.
///
/// The counterpart of [`Bo4eObject`], with [`Typ`](Bo4eTyped::Typ) pinned to
/// `ComTyp`: `fn f<T: Bo4eComponent<Typ = ComTyp>>()` takes `Adresse` and
/// `Betrag` the way the example there takes `Vertrag` and `Lastgang`.
///
/// Sealed: not implementable outside this crate.
#[cfg(feature = "versioned")]
#[cfg_attr(docsrs, doc(cfg(feature = "versioned")))]
pub trait Bo4eComponent: Bo4eTyped + bo4e_component_sealed::Sealed {}

#[cfg(feature = "versioned")]
#[doc(hidden)]
pub mod bo4e_typed_sealed {
    /// Sealing supertrait for [`crate::Bo4eTyped`].
    pub trait Sealed {}
}

#[cfg(feature = "versioned")]
#[doc(hidden)]
pub mod bo4e_object_sealed {
    /// Sealing supertrait for [`crate::Bo4eObject`].
    pub trait Sealed {}
}

#[cfg(feature = "versioned")]
#[doc(hidden)]
pub mod bo4e_component_sealed {
    /// Sealing supertrait for [`crate::Bo4eComponent`].
    pub trait Sealed {}
}

/// Uniform introspection & strict-parsing surface implemented by **every**
/// generated BO4E enum (`Zaehlertyp`, `Marktrolle`, `BdewArtikelnummer`, …).
///
/// Every BO4E enum carries an `Unknown` forward-compatibility catch-all, so the
/// `serde` / `FromStr` path never fails on an unrecognized wire value — the
/// right default for forward-compatibility, and the wrong one at an ingest
/// boundary that must reject typos, legacy codes, or values from a newer schema.
/// The members below close that gap **without the `strum` feature**.
///
/// They are inherent methods on each enum too, so importing this trait is only
/// necessary for code generic over the enum type — a
/// `fn assert_covered<T: Bo4eEnum>()` proving a database `CHECK` list covers
/// `T::VARIANTS`, say.
///
/// # Example
/// ```
/// use rubo4e::{current::Zaehlertyp, Bo4eEnum};
///
/// // Introspection without `strum`:
/// assert_eq!(Zaehlertyp::COUNT, Zaehlertyp::VARIANTS.len());
///
/// // Strict parsing at the ingest boundary:
/// assert_eq!(Zaehlertyp::from_wire("WASSERZAEHLER"), Ok(Zaehlertyp::Wasserzaehler));
/// assert!(Zaehlertyp::from_wire("NOT_A_REAL_VALUE").is_err());
///
/// // Detect lenient-decode fall-through:
/// # #[cfg(feature = "json")] {
/// let z: Zaehlertyp = serde_json::from_str("\"BOGUS\"").unwrap();
/// assert!(z.is_unknown());
/// # }
/// ```
///
/// # Derived traits on every BO4E enum
///
/// `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `PartialOrd`, `Ord`, `Hash` —
/// so an enum can key a `HashMap` or a `BTreeMap`, be sorted, and let a caller's
/// own struct derive `Ord`.
///
/// **`Ord` is declaration order, not a business ranking.** The variants follow
/// the order the BO4E schema lists them in, with the `Unknown` catch-all last.
/// It is a *total* order — which is all `BTreeMap` and `sort()` need — but a
/// schema release may reorder the values, so never persist a sort key derived
/// from it or compare two variants expecting a domain meaning. Compare
/// [`as_wire`](Bo4eEnum::as_wire) when the order has to be stable across
/// releases.
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
/// ```
/// # #[cfg(feature = "json")] {
/// use rubo4e::{current::Messlokation, Bo4eStrict};
///
/// let body = r#"{"messlokationsId":"DE0123456789012345678901234567890","sparte":"PLASMA"}"#;
/// let melo: Messlokation = serde_json::from_str(body).unwrap();  // lenient decode
///
/// // 422 if any Unknown, anywhere:
/// assert_eq!(melo.unknown_enum_paths(), ["sparte"]);
/// assert!(melo.ensure_known_enums().is_err());
/// # }
/// ```
///
/// Paths are reported relative to the checked value, using dotted field names and
/// bracketed array indices (e.g. `zaehler[0].zaehlertyp`). The field names are the
/// BO4E wire (camelCase) names, matching the JSON you deserialized from.
///
/// # Not sealed
///
/// Unlike [`Bo4eTyped`] and [`Bo4eEnum`], `Bo4eStrict` is intentionally **not**
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
/// `use rubo4e::prelude::*;` gives you:
///
/// - **every** identifier type and its error type — always;
/// - [`Bo4eJsonExt`](crate::json::Bo4eJsonExt) and
///   [`Bo4eExtensionData`](crate::json::Bo4eExtensionData), with `json`;
/// - [`Bo4eTyped`], [`Bo4eObject`], [`Bo4eComponent`], [`Bo4eEnum`], and
///   [`Bo4eStrict`], with `versioned`;
/// - [`Validate`](garde::Validate), [`Validated`](crate::validation::Validated),
///   and the report helpers, with `validate`;
/// - the COM extension traits [`BetragExt`](crate::convenience::BetragExt),
///   [`MengeExt`](crate::convenience::MengeExt),
///   [`PreisExt`](crate::convenience::PreisExt), and
///   [`PreisstaffelSliceExt`](crate::convenience::PreisstaffelSliceExt), with
///   `versioned` + `decimal`.
///
/// It deliberately does **not** re-export the generated BO/COM types: they are
/// version-scoped, and `use rubo4e::current::*` is the import that says which
/// schema series you meant.
///
/// `tests/prelude_surface.rs` holds the guard that keeps the first bullet true.
pub mod prelude {
    pub use crate::error::{IdentifierError, LengthExpectation, UnknownVariant};
    /// Every identifier type, and the helper enums their accessors return.
    ///
    /// The BDEW Ressourcen-ID family (`CrId`, `NebeId`, `PaketId`, `SgId`, …) is
    /// here for the same reason `MaloId` is: a crate that touches Redispatch 2.0
    /// or a Netzbetreiberwechsel needs them, and having to remember which four of
    /// the fourteen the prelude forgot is not a distinction worth making.
    pub use crate::identifiers::{
        AkivId, Bic, BilanzierungsgebietId, BilanzkreisId, CrId, EicCode, EicType, Iban, MaloId,
        MaloVergabestelle, MarktpartnerId, MeloId, MpIdAuthority, NebeId, NeloId, ObisCode,
        ObisComponents, PaketId, SgId, SrId, TrId, TranchennummerId,
    };

    /// Uniform enum introspection + strict parsing (`VARIANTS`, `from_wire`, …).
    #[cfg(feature = "versioned")]
    pub use crate::Bo4eEnum;

    /// Recursive strict-decode check (`ensure_known_enums`, `unknown_enum_paths`).
    #[cfg(feature = "versioned")]
    pub use crate::Bo4eStrict;

    /// The trait providing `.validate()`, re-exported so callers do not need a
    /// direct `garde` dependency just to run the derived rules.
    #[cfg(feature = "validate")]
    pub use garde::Validate;

    #[cfg(feature = "validate")]
    pub use crate::validation::Validated;

    #[cfg(feature = "validate")]
    pub use crate::validation::{report_errors, ValidationFailure};

    #[cfg(feature = "json")]
    pub use crate::json::Bo4eExtensionData;

    #[cfg(feature = "json")]
    pub use crate::json::Bo4eJsonExt;

    /// The `_typ` discriminant as constants, and the two markers that narrow it
    /// to the Geschäftsobjekte and the components.
    #[cfg(feature = "versioned")]
    pub use crate::{Bo4eComponent, Bo4eObject, Bo4eTyped};

    /// Flatten `Option<Betrag>` → `Option<Decimal>` in one call.
    #[cfg(all(feature = "versioned", feature = "decimal"))]
    pub use crate::convenience::BetragExt;

    /// Flatten `Option<Menge>` → `Option<Decimal>` in one call.
    #[cfg(all(feature = "versioned", feature = "decimal"))]
    pub use crate::convenience::MengeExt;

    /// Flatten `Option<Preis>` → `Option<Decimal>` in one call.
    #[cfg(all(feature = "versioned", feature = "decimal"))]
    pub use crate::convenience::PreisExt;

    /// Pick the price tier that applies to a quantity, honouring BO4E's gap rule.
    #[cfg(all(feature = "versioned", feature = "decimal"))]
    pub use crate::convenience::PreisstaffelSliceExt;
}
