# Changelog

All notable changes to `rubo4e` are documented in this file.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
This project uses [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

Each release that changes schema-derived enum membership or codelist coverage
also carries a **Schema deltas** section (see [Versioning](https://hupe1980.github.io/rubo4e/docs/versioning/))
so downstream guards (SQL `CHECK` lists, variant-count assertions, coverage
tests) can be updated deliberately instead of discovering drift at runtime.

## [0.9.0] — unreleased

### Fixed *(breaking)*

- **`BilanzkreisId` required EIC object type `'Z'`, which no Bilanzkreis uses.**
  A Bilanzkreis is held by a Bilanzkreisverantwortlicher — a market participant —
  so its EIC carries object type **`'X'` (Party)**. ENTSO-E `'Z'` is a
  *measurement point*. The type therefore **rejected every Bilanzkreis-ID that
  exists**: `11XSUEDWESTSTRO8`, `11XENERGIE2----H`, and `11XENAGISME----J` are
  all real published codes, and all three failed to construct.

  `BilanzkreisId` now pins `'X'`, and the tests are pinned to those real codes
  rather than to a synthetic `11Z…` value that no registry would ever issue.

  **Action required:** any `11Z…` value your code constructed was not a
  Bilanzkreis-ID. Re-source the value from the ECS code registry.

- **`EicCode::domain()` classified EIC object types incorrectly.**
  It mapped `T`/`V` to "Party" and everything else to "Area". Per the ENTSO-E EIC
  Reference Manual, `X` is the party type, `Y` is area/domain, and `T`/`V` are
  *Tieline*/*Location*. Every market-participant code was reported as an area.

  `EicDomain` is **removed** and replaced by `EicType`, which carries all seven
  ENTSO-E object types (`Substation`, `Tieline`, `Location`, `ResourceObject`,
  `Party`, `Area`, `MeasurementPoint`) instead of collapsing them into two wrong
  buckets. Use `EicCode::eic_type()` in place of `EicCode::domain()`.

- **`AnyBo` discarded the deserializer it was given, causing silent data loss.**
  `AnyBo::deserialize` captured a `Box<RawValue>` and re-parsed it with
  `serde_json::from_str`, throwing away the wrapping deserializer. Two consequences:

  - `AnyBo::from_json_snake_case` **returned `Ok` with every typed field empty.**
    The snake_case → German key transform never ran, so values were diverted into
    `_additional` instead of their fields. A round-trip through `AnyBo` silently
    emptied the object rather than failing.
  - `from_json_*_hardened`'s `max_nesting_depth` was **not enforced** for `AnyBo`.
    A `RawValue` capture is one level deep as far as the depth limiter can see, so
    a configured limit silently did nothing on the polymorphic ingest path — the
    one that most often faces untrusted input.

  `AnyBo` now buffers through the caller's deserializer before dispatching on
  `"_typ"`, so both wrappers apply exactly as they do for a concrete BO type.
  This costs an intermediate buffer; deserialize the concrete type on hot paths.

- **`MaloId` check digits were computed with the wrong algorithm.** Releases
  through 0.8 used a Luhn-style variant (per-digit weights `2,1,2,1,…` with a
  `−9` reduction) instead of the *Lok- und Waggon-Kennzeichnungsverfahren* that
  BDEW §8.1 specifies. `MaloId::new` therefore **rejected virtually every real
  MaLo-ID** and accepted invalid ones. The tests generated their expectations
  with the same wrong function, which is why it went unnoticed.

  0.9 implements the specified algorithm, verified against the worked example in
  the BDEW document, the German Wikipedia article, the BO4E-python reference
  implementation, and 631 published Marktpartner-IDs (all 631 validate).

  **Action required:** MaLo-IDs your code previously stored as valid are almost
  certainly wrong. Recompute them with `MaloId::from_base(base)` and re-verify
  persisted data. `51238696780` is the old form of the fixture that appears
  throughout the previous docs and test data; `51238696781` is correct.

- **Hardened parse limits only applied to the root object.**
  `max_extension_value_bytes` and `max_extension_field_count` were checked after
  deserialization on the root's own `_additional` map, so extension data nested
  inside any COM escaped them entirely — a 50 KB payload hidden in
  `marktlokation.lokationsadresse` passed a 16-byte budget. Both limits are now
  enforced **during** parsing at **every** nesting level, which also makes
  rejection fail-fast instead of happening after the whole object tree has been
  allocated. Semantics are unchanged from what the fields always documented:
  `max_extension_value_bytes` is cumulative across the payload,
  `max_extension_field_count` is per struct.

- **`--features time` alone did not compile.** `time_serde` used `serde`
  unconditionally while being gated only on `time`; it is now gated on both.

- **`_version` was never populated, so Rust-built payloads were distinguishable
  from every other implementation's.** `Vertrag::default()` serialized to
  `{"_typ":"VERTRAG"}` where BO4E-python and go-bo4e both emit
  `{"_typ":"VERTRAG","_version":"v202607.0.0",…}` — including on nested COMs,
  which carry `_version` but no `_typ`. The docs told callers to set it
  explicitly, which meant hardcoding a version literal that silently goes stale
  on upgrade, even though the crate already knew the value statically via
  `Bo4eObject::schema_version()`.

  `_version` is now pre-filled on construction for **every** BO and COM — through
  `Default::default()`, the typed builder, and `..Default::default()` — matching
  the reference implementations. `_typ` remains BO-only, as before.

  Deserialization is unchanged and deliberately so: `_version` records the
  provenance of the data, so a payload arriving stamped `v202501.0.0` keeps that
  value, and one arriving without `_version` stays without one. Only construction
  fills it in; the setter is still available to re-stamp a value deliberately.

  Neither the golden corpus nor the compat vectors could catch this — both only
  round-trip existing JSON, which carries `_version` in from the input.
  `tests/compat.rs` now has an `outbound_tests` module covering the direction
  rubo4e *produces*.

  **Action required:** if you asserted on exact serialized output, those payloads
  now contain `_version`. If you were setting it manually, you can stop.

- **snake_case JSON silently moved fields into extension data.** The key
  transform derived the camelCase↔snake_case mapping with a heuristic, and a
  heuristic has no correct inverse: `hoechstpreis_ht` is an equally valid
  rendering of `hoechstpreisHt` and `hoechstpreisHT`, and `a` of both `a` and
  `A`. BO4E uses all of those shapes, so
  `from_json_snake_case(to_json_snake_case(x))` did not return `x` for
  `Tarifberechnungsparameter` (`hoechstpreisHT`, `hoechstpreisNT`),
  `PreisblattKonzessionsabgabe` (`kundengruppeKA`), and `Sigmoidparameter`
  (`A`, `B`, `C`, `D`). The values were not lost — they were deserialized into
  `_additional` instead of their typed field, so the round-trip looked
  successful while the typed accessors returned `None`.

  The generator now emits the exact bidirectional mapping
  (`src/generated/key_map.rs`) from the same field data it uses to emit the
  structs, so the round-trip is lossless by construction and cannot drift from
  the generated types. Lookups resolve to `&'static str`, so renaming a key now
  allocates on neither path.

  **Behaviour change:** keys the schema does not define — extension data — are
  no longer rewritten between modes. Previously `{"fooBAR": 1}` became
  `foo_bar` on the way out and `fooBar` on the way back in; it now passes
  through byte-for-byte in both directions. BO4E metadata keys (`_typ`,
  `_version`, `_id`) keep their leading underscore in every mode, as before.

  **Action required:** if you persisted snake_case JSON produced by 0.8 or
  earlier, extension keys in it carry heuristic-mangled names, and the four
  types above stored their affected fields under `_additional`. Re-serialize
  from the German wire format, which was never affected.

### Added

- **Four more BDEW identifier types**, completing the §8.2 ASCII-Verfahren
  family: `NebeId` (Netzbereich, Codetyp `F`), `CrId` (Cluster Ressource, `A`),
  `SgId` (Steuergruppe, `B`), and `PaketId` (Netzbetreiberwechsel, `P9`).
- **`AkivId`, `BilanzkreisId`, `TranchennummerId`** — Aktivierungsidentifikator
  (Redispatch 2.0), Bilanzkreis (EIC object type `X`), and MABIS Tranchennummer.
- **`EicType`** — all seven ENTSO-E EIC object types, with `as_char`,
  `from_char`, `description`, and an exhaustive `ALL` that is the single source of
  truth for which position-3 characters `EicCode` accepts.
- **`BilanzierungsgebietId`** — EIC pinned to object type `'Y'` (Area), the MaBiS
  Bilanzierungsgebiet counterpart to `BilanzkreisId`. Having both as distinct
  types means a balance group cannot be passed where a balancing area is
  expected. `StandorteigenschaftenStrom.bilanzierungsgebiet_eic` now generates as
  this type instead of `String`: the schema documents it as "Die EIC-Nummer des
  Bilanzierungsgebietes", and all 645 codes in the TSOs' published
  VNB-Bilanzierungsgebiete list carry object type `Y`.
- **`EicCode::new_from_prefix`** — builds a complete code from a 15-character
  prefix by computing the ENTSO-E check character.
- **`ObisCode::as_str`** — the canonical string, matching `as_ref`/`Display`.
- **Shared API across every §8.2 identifier**: `from_base()` computes and appends
  the check digit, `check_digit()` returns it without constructing the value,
  `base()` returns the 10-character body, and `CODETYP` exposes the fixed prefix.
- **`MaloId::vergabestelle()`** returning `MaloVergabestelle`, and
  **`MarktpartnerId::authority()`** returning `MpIdAuthority`, plus
  `nad_agency_code()` / `unb_agency_code()` for EDIFACT NAD DE3055 and UNB DE0007.
- **Opt-in MP-ID check-digit verification** — `MarktpartnerId::new_checked`,
  `has_valid_bdew_check_digit`, `has_valid_gln_check_digit`. Construction still
  does not enforce a check digit, because an MP-ID may carry either the BDEW
  (§8.1) or the GS1/EAN-13 procedure and the leading digits do not reliably
  separate them.
- **`sqlx::postgres::PgHasArrayType` for every identifier**, so `Vec<Id>` binds
  to a `TEXT[]` column. This has to live in this crate: both the trait and the
  types are foreign to any consumer, so the orphan rule rules out a local impl.
- **`prelude::Validate`** — the `garde` trait that provides `.validate()` is now
  re-exported, so callers no longer need a direct `garde` dependency to use the
  `validate` feature.
- **Documentation site** at <https://hupe1980.github.io/rubo4e>, built from
  `site/` with Zola.

### Changed *(breaking)*

- `NeloId`, `SrId`, and `TrId` moved into a shared `ascii_ids` module with the
  other §8.2 identifiers. The public paths (`rubo4e::identifiers::NeloId`, …) are
  unchanged; only the internal module layout moved.
- `MaloId` now enforces the leading Vergabestelle digit (`1`–`9`) per §3.2.
- **`sqlx` no longer implies `json`.** Identifier and enum SQL impls both
  round-trip through plain `&str` (`as_ref` / `as_wire` / `from_wire`), so
  `serde_json` is no longer pulled in. Enum `Decode` also stops allocating a
  `serde_json::Value` per row.
- The `*_hardened` methods on `Bo4eJsonExt` dropped their
  `where Self: Bo4eExtensionData` bound, which the parse-time budget made
  unnecessary.
- **MSRV raised from 1.87 to 1.88.** Not a source change — `time`, `simd-json`,
  and `home` (reached through `sqlx`) all now require 1.88, so the declared
  `rust-version` was no longer achievable and CI's MSRV job failed at dependency
  *resolution*, before compiling anything. The crate's own source still builds on
  1.87; only the dependency tree does not. `garde` is no longer the binding
  constraint, and the feature table no longer lists an MSRV impact for `validate`.
- **`deny.toml` records six known-safe duplicate-version splits.** All are
  transitive splits between upstream crates that each pin their own major
  (`windows-sys`, `hashbrown` ×2, `foldhash`, `redox_syscall`, `syn`), reachable
  only through the optional `sqlx` / `simd-json` features and dev-only tooling.
  None can be collapsed from this crate. Skips are pinned at minor-version
  granularity so a patch bump upstream does not silently re-break the gate.
- **`just deny-check` now runs `--all-features`**, matching what
  `cargo-deny-action` does in CI. The recipe previously checked only the default
  feature set, which left every optional dependency out of the graph — a
  duplicate-version ban that CI rejected passed cleanly on the same tree locally.
- **`ObisCode` stores a canonical form and its parsed value groups.**
  Previously the input string was stored verbatim apart from `&`→`*`, and
  `components()` re-parsed it — allocating on every call and carrying an
  `expect` that a stored value was still parseable. Now the value is parsed once
  at construction, so `components()` is infallible and free.

  Canonicalisation also drops redundant leading zeros, which makes equality
  semantic: `01-00:01.08.00` and `1-0:1.8.0` are now equal and hash alike, where
  before they were distinct. `as_ref`, `Display`, and `serde` all emit the
  canonical form, so a value may not round-trip byte-for-byte to its input.

  `ObisCode::to_bo4e_string()` is **removed** — the canonical form is what the
  type stores, so `as_str()` (or `as_ref`/`Display`) returns it without
  allocating. `to_pia_string()` is unchanged.
- **OBIS value groups are `u8` rather than `u32`.** IEC 62056-61 §4 defines each
  of A–F as a single octet, so `ObisComponents` fields are now `u8`/`Option<u8>`
  and a group above 255 is rejected with an error naming the offending group.
- **Uniform trait surface across all identifiers.** `EicCode`, `BilanzkreisId`,
  and `ObisCode` hand-rolled a subset of the conversions the macro-generated
  identifiers already had. All identifiers now share one implementation, so
  `Deref<Target = str>`, `Borrow<str>`, and `From<T> for String` are available on
  every one of them — previously `String::from(malo_id)` compiled but
  `String::from(eic_code)` did not.
- `ObisCode` now carries a real `schemars`/`utoipa` schema (grammar pattern,
  description, examples) instead of a bare `String`.

### Removed

- **`src/identifiers/proptest_impls.rs`** — 224 lines of `#[cfg(test)]`
  `Arbitrary` impls with no callers. The integration suite has its own strategy
  table, and the unused copy had silently rotted: its `EicCode` generator placed
  the object-type character at position 1 instead of 3 (so most draws were
  discarded), and its `BilanzkreisId` generator produced `'Z'` codes. The
  integration table now also covers `BilanzkreisId` and `BilanzierungsgebietId`.
- **`json::peek_typ_field`** — an internal helper that existed only for the
  `AnyBo` raw-capture path removed above.

### Documentation

- Every code example in the crate is now compiled and run: the 205 `rust,ignore`
  doctest blocks are gone, and the generated per-enum examples carry real
  assertions including a wire→variant mapping taken from the schema. Doctests
  went from 37 passing / 206 ignored to 242 passing / 0 ignored.
- CI gained a 24-configuration feature matrix, MSRV verification, generated-code
  drift detection, and rustdoc broken-link denial.

### Schema deltas

- Schema version unchanged: **v202607.0.0**. No enum membership or codelist
  changes in this release.
- Upstream **v202607.1.0** remains un-adopted; see the 0.8.0 notes below for the
  review. Regenerating against it is still a separate follow-up.

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
