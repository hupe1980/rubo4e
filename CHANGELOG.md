# Changelog

All notable changes to `rubo4e` are documented in this file.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
This project uses [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0] — 2026-07-07

### Added
- `MaloId::from_base(base: &str)` — constructs a valid `MaloId` from a 10-digit base by
  computing and appending the BDEW check digit. Identical helpers added for `SrId` and `TrId`.
- `MaloId::check_digit(base: &str) -> Result<u8, _>` — returns the single check digit
  without constructing the full identifier. Identical helpers added for `SrId` and `TrId`.
- `MeloId::country_code() -> &str` — returns the 2-character ISO 3166-1 alpha-2 country
  code slice; zero-copy, always valid.
- `MeloId::is_german() -> bool` — convenience predicate (`country_code() == "DE"`).
- `MarktpartnerId::to_i64() -> i64` — infallible integer conversion; 13-digit decimal fits i64.
- `rubo4e::identifiers::serde_as_i64` module (`MarktpartnerId`) — serializes as JSON integer,
  deserializes from integer or string. Re-exported as `rubo4e::identifiers::marktpartner_id_as_i64`.
- `rubo4e::time_serde` module (`time` feature) — `date_serde` and `opt_date_serde` serde
  modules for `time::Date` ↔ `"YYYY-MM-DD"`. Both use zero-allocation `Visitor` patterns:
  `visit_str` borrows from the input; `opt_date_serde` uses `deserialize_option` so JSON `null`
  maps to `None` without an intermediate `String` allocation. Previously these lived inside
  `schema_helpers` (gated on `schemars`); they are now correctly gated on `time` only.
- `rubo4e::convenience` module (`versioned` + `time` features):
  - `Zeitraum::as_closed_range() -> Option<(Date, Date)>` — both boundary dates present.
  - `Zeitraum::as_half_open_range() -> Option<(Date, Option<Date>)>` — start required; open-ended allowed.
  - `Rechnung::billing_period() -> Option<(Date, Date)>` — reads `rechnungsperiode`.
  - `PreisblattNetznutzung::validity() -> Option<(Date, Option<Date>)>` — reads `gueltigkeit`.
- `rubo4e::current` alias — `pub use v202501 as current`; always points to the latest stable series.

### Changed *(breaking)*
- **`Zeitraum.startdatum` / `Zeitraum.enddatum`** now typed as `Option<time::Date>` (was
  `Option<time::OffsetDateTime>`). BO4E JSON Schema specifies `"format": "date"` (date-only)
  for these fields. Callers using `time::OffsetDateTime` for these fields must migrate to `time::Date`.
  Downstream: `validate_zeitraum` comparison updated accordingly.
- **`Person.geburtsdatum`** now typed as `Option<time::Date>` (was `Option<time::OffsetDateTime>`).
- **Generator: `"datum"` suffix inference rule removed.** The inference engine no longer maps
  field names ending in `"datum"` to `OffsetDateTime`. All BO4E date-only fields carry
  `"format": "date"` schema annotations; the parser reads these directly. The old rule was
  incorrect (German "datum" = date, not datetime) and could misfire on unannotated future fields.
- **`serde(with = ...)` paths for date fields** changed from `crate::schema_helpers::*_date_serde`
  to `crate::time_serde::*_date_serde` in all generated code. The `schema_helpers` module is
  now strictly schemars-only and gated on `feature = "schemars"`. Callers who referenced
  `rubo4e::schema_helpers::date_serde` directly must use `rubo4e::time_serde::date_serde`.

### Fixed
- `opt_date_serde::deserialize` previously allocated an intermediate `Option<String>` heap
  object for every optional date field deserialization. Replaced with a proper `Visitor`
  implementing `visit_none`, `visit_unit`, and `visit_some` — now fully zero-allocation.
- `schema_helpers` module was gated on `schemars` only. Any build with `time` enabled but
  without `schemars` would fail to compile (`crate::schema_helpers not found`). Fixed by
  moving date serde to `time_serde` (gated on `time`) and keeping `schema_helpers` strictly
  schemars.

## [0.2.0] — 2026-07-05

### Added
- `EicCode::type_char()` — raw EIC type character at position 3 (index 2).
- `EicDomain` enum (two variants: `Area`, `Party`) with `Display` impl.
- `EicCode::domain()` — heuristic domain classification based on ENTSO-E usage patterns.
- `ObisCode` C-component validation: C ≥ 1 is now enforced per IEC 62056-61 / BDEW specification.
- `peek_typ_field()` helper (`pub(crate)`) in `src/json/mod.rs` — zero-allocation `_typ` extraction from a raw JSON `&str` via a borrowed-field serde struct. Used by the generated `AnyBo` deserializer to avoid materialising a full `Value` tree.
- `Bo4eObject` trait is now sealed (`bo4e_object_sealed::Sealed` supertrait, versioned-feature only).
- `[package.metadata.docs.rs]` with `all-features = true` and `rustdoc-args = ["--cfg", "docsrs"]`.
- `#![cfg_attr(docsrs, feature(doc_cfg))]` + `#[cfg_attr(docsrs, doc(cfg(...)))]` on all feature-gated public items.
- Default nesting-depth guard (128 levels) in `deserialize_german_from_str` and `deserialize_german_from_slice`.
- Fuzz corpus seeds in `fuzz/corpus/` for all 7 fuzz targets.
- Integration tests for `AnyBo` dispatch (`tests/any_bo.rs`): 19 test cases covering dispatch, golden roundtrips, and edge cases.
- `LimitedExtensionMap` implements `utoipa::ToSchema` and `utoipa::PartialSchema` (utoipa feature).
- Inference regression tests in `generator/tests/round_trip.rs` (`v202501_inference_audit`): covers identifier newtypes, OffsetDateTime fields, Decimal scalars, and 9 schema-`$ref`-wins-over-suffix-inference cases (e.g. `Rechnungsposition.einzelpreis` → `Option<Preis>`).

### Changed *(breaking)*
- **`AnyBo` deserialization** (generator + `src/generated/v202501/mod.rs`): replaced the two-pass `serde_json::Value::deserialize` + `from_value` strategy with a single `Box<serde_json::value::RawValue>` capture and `peek_typ_field` zero-allocation `_typ` scan. Each concrete type is now deserialized via `serde_json::from_str(raw.get())` directly from the original bytes — no intermediate `Value` tree is ever allocated for known BO types. The `Unknown` variant still converts to `serde_json::Value` for arbitrary-field preservation.
- **Generator inference priority** (`generator/src/parser.rs`): schema `$ref` references to BO or COM types (resolved to `FieldType::Bo` or `FieldType::Com`) now unconditionally take precedence over suffix-based inference. Previously, fields like `einzelpreis` (suffix `"preis"` → `Decimal`) were incorrectly typed as `Option<Decimal>` even when the schema said `$ref: Preis.json`. Corrects 16 fields across 8 structs: `Rechnungsposition`, `Kostenposition`, `Fremdkostenposition`, `Angebotsposition`, `Angebotsvariante`, `Ausschreibungslos`, `Bilanzierung`, `Tarifberechnungsparameter`, `Vertragsteil`.
- **Generated struct optional-field setters** (`builder` feature): `setter(strip_option)` replaced by `setter(into)` throughout. Setters now accept both `T` and `Option<T>` (via `impl Into<Option<T>>`). Callers passing `T` directly continue to work; callers that hold an `Option<T>` from a segment parser can now pass it without intermediate `if let`.
- **`_additional` extension-data field** on generated structs: changed from `pub(crate)` to `#[doc(hidden)] pub`. External crates can now use functional-update syntax (`Struct { field: value, ..Default::default() }`) without a compile error.
- **`testing` feature removed.** `proptest` is now a `[dev-dependency]` only.
  Proptest `Arbitrary` impls for identifiers are compiled with `#[cfg(test)]`.
  Integration tests in `tests/proptest_roundtrips.rs` use inline strategies via public APIs.
  Downstream crates depending on `features = ["testing"]` must remove that feature flag.
- **`EicDomain` simplified** from three variants (`Area`, `Party`, `Resource`) to two (`Area`, `Party`).
  All type chars except `T` and `V` now map to `Area`.
- **`ObisCode` rejects C = 0.** Previously accepted; now returns `IdentifierError::InvalidFormat`.
- Enum `Arbitrary` impls in generated files changed from `#[cfg(all(feature = "testing", feature = "strum"))]`
  to `#[cfg(all(test, feature = "strum"))]`.

### Fixed
- `SortedMap::end` and `SortedSeq::end` no longer clone buffered bytes — entries are drained instead.
- `deny.toml`: removed 11 stale Windows-target skip entries that no longer apply.
- Golden test corpus: fixed incorrect enum casing (title-case → SCREAMING_CASE) in 5 fixture files.
- Golden tests now assert `serde_json::Value` equality (structural, not string equality).

### Dependency Updates
- `strum` 0.27 → 0.28
- `time` pinned to 0.3.53
- `simd-json` 0.14 → 0.17
- `criterion` 0.5 → 0.8
- `utoipa` 4 → 5 (with `time`, `decimal`, `indexmap` features enabled)

## [0.1.0] — 2025-01-01

### Added
- Initial release of `rubo4e`.
- BO4E schema v202501.0.0 — full set of Business Objects (BO) and Component Objects (COM).
- Features: `serde` (default), `json`, `simd-json`, `time`, `decimal`, `builder`, `validate`,
  `schemars`, `sqlx`, `utoipa`, `strum`, `versioned`, `tracing`, `metrics`.
- MSRV: Rust 1.87.
- Identifier newtypes: `MaloId`, `MeloId`, `NeloId`, `SrId`, `TrId`, `MarktpartnerId`,
  `ObisCode`, `EicCode` — all with construction-time validation.
- `Bo4eJsonExt` trait: `to_json_german()`, `to_json_snake_case()`, `to_json_canonical()`,
  `from_json_german()`, `from_json_snake_case()`.
- `AnyBo` enum for schema-version-agnostic dispatch.
- `Bo4eObject` sealed trait for type-safe generic code over all BO types.
- `LimitedExtensionMap` for DoS-hardened unknown-field capture (max 128 fields, key ≤ 256 bytes).
- `Validated<T>` wrapper enforcing `garde` validation.
- Fuzz targets for 6 BO types and identifier parsing.
