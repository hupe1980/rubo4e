# Changelog

All notable changes to `rubo4e` are documented in this file.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
This project uses [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

Each release that changes schema-derived enum membership or codelist coverage
also carries a **Schema deltas** section (see [Versioning](docs/versioning.md))
so downstream guards (SQL `CHECK` lists, variant-count assertions, coverage
tests) can be updated deliberately instead of discovering drift at runtime.

## [Unreleased]

## [0.8.0] — 2026-07-26

This release adds a uniform, feature-independent introspection and strict-parsing
surface to every generated BO4E enum, in response to downstream feedback from the
`mako` project.

### Added

- **`Bo4eStrict` trait + `strict` module** (`versioned` feature) — recursive
  strict decoding. `value.ensure_known_enums()` walks a decoded BO/COM/`AnyBo`
  and returns `Err(strict::StrictError)` listing the JSON-path of **every** enum
  field that fell through to `Unknown`, anywhere in the tree (e.g.
  `["zaehler[1].zaehlertyp"]`). `unknown_enum_paths()` returns them directly.
  One call replaces the hand-written `record.field == T::Unknown` re-checks a
  strict ingest boundary needs. Implemented for every generated BO, COM, enum,
  and `AnyBo`. **Not sealed** — downstream wrappers can implement it to extend
  the recursive check. Re-exported from `rubo4e::prelude`.
- **`Bo4eEnum` trait** (`versioned` feature) — implemented by every generated
  BO4E enum, giving a uniform surface for code that is generic over the enum type
  (e.g. proving a SQL `CHECK` list covers `T::VARIANTS`). Re-exported from
  `rubo4e::prelude`. Sealed — cannot be implemented downstream.
- **Feature-independent `Display` + `AsRef<str>` on every enum**, yielding the
  canonical BO4E wire string via `as_wire`. Previously these required `strum`.
- **Per-enum introspection, available WITHOUT the `strum` feature.** Every enum
  now exposes:
  - `const VARIANTS: &'static [Self]` — the known variants, excluding the
    `Unknown` catch-all, in schema declaration order.
  - `const COUNT: usize` — a stable per-version variant count. Replaces the
    hand-maintained magic-number guards downstream projects had to pin.
  - `fn iter_known() -> impl Iterator<Item = Self> + Clone` — previously gated on
    `strum`, now always available.
- **Strict enum parsing.** Every enum now exposes:
  - `fn from_wire(s: &str) -> Result<Self, UnknownVariant>` — the opt-in strict
    counterpart to the lenient `serde` / `FromStr` path. Returns `Err` for typos,
    legacy codes, and values from a newer schema (including the literal
    `"UNKNOWN"`) instead of silently mapping them to `Unknown`.
  - `fn as_wire(&self) -> &'static str` — the canonical BO4E wire string.
  - `const fn is_known(&self) -> bool` / `const fn is_unknown(&self) -> bool` —
    detect a value that fell through to `Unknown` after a lenient decode, in one
    call at the ingest boundary.
- **`error::UnknownVariant`** — the error returned by `from_wire`, with the
  offending value. Converts into `garde::Error` under the `validate` feature.
  Re-exported from `rubo4e::prelude`.
- **Type- and variant-level interop documentation** generated directly into the
  affected enums:
  - `Zaehlertyp::IntelligentesMesssystem` and `Geraetetyp::IntelligentesMessystem`
    now carry cross-referencing notes about the upstream `Messsystem`/`Messystem`
    spelling divergence.
  - `BdewArtikelnummer` documents its provenance and coverage signal.
  - `Gasqualitaet` documents the H2-blend forward-compatibility story.
  - `Rechnungstyp` documents the sanctioned representation for correction/reversal
    invoices.

### Changed *(breaking)*

- Enum `Display` and `AsRef<str>` are now hand-written (always on) rather than
  derived from `strum`. Behaviour is identical (canonical wire string); the
  `strum` derive set is reduced to `EnumString`, `EnumIter`, `IntoStaticStr`.
- The sqlx `Encode` impl for enums no longer has a separate `strum` fast-path —
  it always encodes via `as_wire()` (one fewer allocation, no `strum` needed).
- The proptest `Arbitrary` impls for generated enums no longer require the
  `strum` feature (they now sample from `VARIANTS`). This only affects the
  crate's own `#[cfg(test)]` builds.

### Notes / no-ops

- **Reactive-energy units already present.** `Mengeneinheit` already includes
  `Kvarh`, `Kvar`, `Var`, and `Varh` in v202607 — no change needed. Downstream
  code mapping `KVARH → KWH` / `KVAR → KW` can map directly to the reactive
  variants instead.
- **Mandatory-vs-optional fields.** AHB-mandatory status is a
  message/process-context property and is *not* present in the BO4E JSON Schema
  (`bo/*.json` carry no `required` array beyond `_typ`), so it cannot be derived
  here. For ergonomic, diffable construction, enable the `builder` feature
  (`typed-builder`, `setter(into)`); see `examples/builder.rs`.

### Schema deltas

- Schema version unchanged: **v202607.0.0**. No enum membership or codelist
  changes in this release — all changes above are library API additions.
- Reviewed upstream **v202607.1.0** (BO4E-Schemas): it removes two *unreferenced*
  enums (`Mengenoperator`, `Lokationstyp`), trims `Messgroesse`, and remodels
  `Zeitreihe`; it does **not** touch `Gasqualitaet`, `Zaehlertyp`, `Geraetetyp`,
  `Mengeneinheit`, `Rechnungstyp`, or `BdewArtikelnummer`. All the enum
  observations above therefore still hold against the newest upstream patch.
  Regenerating against v202607.1.0 is tracked as a separate follow-up.

---

Older history (0.1.0 – 0.7.0) is available in the git log. Notable prior
milestones: identifier types with BDEW check digits (`MaloId`, `MeloId`, `SrId`,
`TrId`, `AkivId`, `BilanzkreisId`, `TranchennummerId`, …), the `versioned` schema
modules, the `time`-crate version-bound relaxation in 0.7.0, and the
`decimal` / `time` typed-field backends.
