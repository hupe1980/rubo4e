+++
title = "Testing Strategy"
description = "The eight testing layers that back this crate: schema drift guards, golden corpus, snapshots, property tests, fuzzing, cross-implementation compatibility, doctests, and the feature matrix."
weight = 80
+++

`rubo4e` uses eight distinct testing layers. Each layer has its own purpose, test corpus,
and command to run.

## Test Layer Summary

| Layer | Purpose | Feature flags | Location | Approx. runtime |
|-------|---------|---------------|----------|-----------------|
| 0. Drift guard | Committed codegen matches the schemas | `versioned` | `tests/generated_contract.rs` | < 1 s |
| 1. Golden | Wire compatibility | `json`, `versioned` | `tests/golden/` | < 5 s |
| 2. Snapshot | Serialization stability | `schemars` | `tests/snapshots/` | < 5 s |
| 3. Property | Identifier invariants | (dev dep only) | `tests/proptest_roundtrips.rs` | 30–60 s |
| 4. Fuzz | Panic safety | nightly + `json` | `fuzz/` | minutes (CI: 1M runs) |
| 5. Compat | Cross-impl interop | `json`, `versioned` | `tests/compat/` | < 10 s |
| 6. Doctest | Documentation is executable | all | `src/**` rustdoc comments | ~50 s |
| 7. Feature matrix | Every feature builds warning-free | (per combination) | CI job / `just lint-features` | minutes |

Two Criterion benches sit alongside them, measured rather than asserted:
`benches/json_perf.rs` for the three serialization modes and
`benches/timeseries_perf.rs` for the coverage audit at a day, a month and a
settlement year, across clean, gappy, duplicated and reversed series.

## Layer 0 — Schema Drift Guards

`src/generated/` is committed, so nothing at build time forces it to agree with
`generator/schemas/`. These tests read both sides and compare.

**Run:**
```bash
cargo test --features versioned --test generated_contract
```

**What the tests check:**
- Every schema emitted exactly one Rust module, and no module is left over
- Every struct stamps the `_typ` its schema pins with a `const` — BOs *and* COMs
- Every struct stamps the `_version` its schema declares as a default
- `BoTyp` / `ComTyp` variants are named after the structs they discriminate
- No two BO4E wire values collapse onto one Rust enum variant

`just check-docs-drift` complements this by regenerating into a scratch copy and
diffing, which catches changes these assertions do not name.

The pinned schema tag is not written out anywhere: every test, recipe, workflow
step, and the site config derive it from the single committed snapshot directory
under `generator/schemas/`. `pinned_tag.rs` fails the build if one starts pinning
a literal, and checks the same for the MSRV. A within-series bump therefore
touches the snapshot, the codegen, and the changelog — not a scattering of
strings.

More guards of the same kind sit alongside them, in `tests/`:

| Test | Catches |
|---|---|
| `prelude_surface.rs` | an identifier type reachable via `rubo4e::identifiers::` but missing from the prelude — and any identifier whose `Borrow<str>` disagrees with its `Hash` / `Ord`, which silently breaks `HashMap::get(&str)` |
| `extension_round_trip.rs` | the snake_case key transform renaming keys inside extension data |
| `hash_keys.rs` | generated types deriving `Hash` without `Eq`, which no `HashMap` key can use |
| `json_strictness.rs` | a JSON entry point that stops at the end of the first value and ignores the rest, or one that skips the nesting-depth cap — either makes this crate accept a payload `serde_json` rejects |
| `site_examples.rs` | a README or site snippet that stopped compiling, or stopped meaning what the surrounding prose says |
| `interval_conventions.rs` | a validator or accessor reading an interval bound the opposite way to the schema it came from |
| `pinned_tag.rs` | a schema tag or MSRV written out in a workflow, recipe, or the site config instead of derived |

## Layer 1 — Golden Schema Tests

Deserialize official BO4E JSON payloads and re-serialize; compare field values.

**Run:**
```bash
cargo test --features json,versioned --test golden
```

**Corpus location:** `tests/golden/`

```
tests/golden/
├── vertrag_minimal.json        # only _typ + _version
├── vertrag_typical.json        # common fields populated
├── marktlokation_minimal.json
├── marktlokation_typical.json
├── messlokation_minimal.json
├── messlokation_typical.json
├── netzlokation_minimal.json
├── netzlokation_typical.json
├── rechnung_minimal.json
├── rechnung_typical.json
├── lastgang_minimal.json       # only the required zeitIntervallLaenge
├── lastgang_typical.json       # a clean quarter-hourly hour in kW
├── zeitreihe_minimal.json
└── zeitreihe_typical.json      # the same hour as kWh readings
```

Files are **not** nested in a version subdirectory — all live directly under
`tests/golden/`. The schema version is encoded in each file’s `"_version"` field.

**What the test checks:**
- Deserialization does not return an error
- Re-serialized output with `to_json_german()` deserializes back to a value equal
  to the original (field values identical; key ordering not required to match)
- Unknown fields in the payload are preserved in `_additional` and survive the round-trip

The two time-series fixtures carry a second assertion beyond the round-trip: the
`Lastgang` must audit clean (`is_usable()`, full coverage, 450 kWh integrated)
and the `Zeitreihe` must sum to the same 450 kWh. That pins them as the
*reference shape a producer should emit*, not merely as something that survives a
round-trip — see [Time Series & Units](@/docs/timeseries.md).

## Layer 2 — Snapshot Serialization Tests

Verify that canonical and German serialization output does not change unexpectedly.
Uses `insta` for snapshot management.

**Run:**
```bash
cargo test --features schemars --test schemars_snapshots
```

**Update snapshots** after intentional changes:
```bash
cargo insta review
```

Snapshots are committed to the repository. A changed snapshot in CI is a CI failure
that requires explicit review and acceptance with `cargo insta accept`.

## Layer 3 — Property-Based Tests

Verify identifier round-trip invariants, serde correctness for date types, and
enum `Display`/`FromStr` for all generated variants.

**Run:**
```bash
cargo test --all-features --test proptest_roundtrips
```

> **Note:** `proptest` is a plain dev-dependency, and the `Arbitrary` impls for
> identifier types are `#[cfg(test)]` only — not available to external crates.

**Properties covered:**

```rust
// Identifier: Display ↔ FromStr round-trip
proptest! {
    fn malo_id_display_from_str_roundtrip(s in valid_11digit()) {
        let id = MaloId::new(&s).unwrap();
        prop_assert_eq!(id.to_string().parse::<MaloId>().unwrap(), id);
    }
}

// Serde round-trip for required time::Date
proptest! {
    fn required_date_serde_roundtrip(date in any_date()) {
        // serializes as "YYYY-MM-DD", deserializes back to the same Date
    }
}

// Enum: Display ↔ FromStr round-trip over all known variants (strum)
proptest! {
    fn sparte_display_from_str_roundtrip(variant in any_sparte()) { … }
}
```

Also covered: `opt_date_serde` `None`/`Some` round-trips, JSON null → `None` deserialization,
ISO 8601 wire-format assertion (`"YYYY-MM-DD"`).

## Layer 4 — Fuzz Testing

Feed arbitrary bytes to the deserialization path and verify no panics occur.
Requires nightly Rust.

**Setup:**
```bash
cargo install cargo-fuzz
```

**Run (CI — 1 million iterations):**
```bash
cargo +nightly fuzz run fuzz_deserialize_vertrag -- -runs=1000000
```

**Run (continuous — local development):**
```bash
cargo +nightly fuzz run fuzz_deserialize_vertrag
```

**Targets:**
```
fuzz/fuzz_targets/
├── fuzz_deserialize_marktlokation.rs  — identifiers, Ortsangabe exclusivity
├── fuzz_deserialize_vertrag.rs        — date-time ordering
├── fuzz_deserialize_rechnung.rs       — multi-Betrag arithmetic, currency agreement
├── fuzz_deserialize_kosten.rs         — Kostenposition line-total arithmetic, two levels down; the `Value` reader and both recursive walks
├── fuzz_deserialize_bilanzierung.rs   — nested temporal ranges
├── fuzz_deserialize_lastgang.rs       — large Zeitreihenwert arrays; depth and budget limits; the coverage audit
├── fuzz_deserialize_zeitreihenwert.rs — the hot path in batch market-data processing
└── fuzz_parse_identifiers.rs          — every identifier parser, the duration and time-of-day parsers
```

Each BO target runs three separate code paths over the same bytes —
`serde_json::from_slice`, the hardened German reader, the hardened snake_case
reader — and **validates** whatever decoded. The `Lastgang` target additionally
runs the coverage audit: it parses every `startuhrzeit` off the wire, joins each
with its date, sorts the results and accumulates `time::Duration`s over them —
and `Duration` addition *panics* rather than saturating, so the accumulation is
fuzzed rather than reasoned about. The `Kosten` target runs the two recursive
walks — `extension_paths` and `unknown_enum_paths` — because
`collect_extension_paths` is the one place a JSON-path is assembled from bytes
the payload chose rather than from the schema, and it runs the
`serde_json::Value` reader, which is a fourth deserializer over the same input. The validators do `Decimal`
arithmetic over wire values, and `rust_decimal` panics rather than errors on
several of its constructors, so a validator that aborts on a decodable payload is
as exploitable as a deserializer that does. Hence `validate` in the fuzz build
alongside `time` and `decimal`.

**What constitutes a fuzz failure:**
- Any panic (including `unwrap`, `expect`, index out of bounds)
- Stack overflow
- Memory safety violation

An `Err` return from `from_slice` is **not** a failure — malformed input is expected
to return an error, not panic.

**Reproducing a crash:**
```bash
cargo +nightly fuzz run fuzz_deserialize_vertrag fuzz/artifacts/fuzz_deserialize_vertrag/<id>
```

## Layer 5 — Cross-Implementation Compatibility

Verify that `rubo4e` correctly deserializes payloads produced by the Python and Go
reference implementations.

**Run:**
```bash
cargo test --features json,versioned --test compat
```

**Corpus location:**
```
tests/compat/
├── README.md           — how to regenerate vectors
├── python/
│   ├── marktlokation.json
│   ├── messlokation.json
│   ├── rechnung.json
│   └── vertrag.json
└── go/
    ├── marktlokation.json
    ├── messlokation.json
    ├── rechnung.json
    └── vertrag.json
```

**What the test checks:**
- Deserialization does not error
- Specific field values are asserted (not just "no error") — at least 3 fields per payload

**Regenerating vectors:**
See `tests/compat/README.md` for instructions on how to regenerate when either reference
implementation releases a new version.

## Layer 6 — Doctests

Every code block in a rustdoc comment is compiled and run. There are no
`rust,ignore` blocks anywhere in the crate: an example that cannot be executed
is written as a ```` ```text ```` block so it is never mistaken for verified code.

**Run:**
```bash
cargo test --all-features --doc
```

This matters more here than in most crates because the majority of the public
API is generated. The per-enum `iter_known` and `from_wire` examples are emitted
by the generator with real assertions — including a positive `wire → variant`
mapping taken from the schema — so a generator change that breaks the documented
behaviour fails the build instead of silently producing wrong documentation.

Examples that need a feature the doctest harness may not have are wrapped rather
than ignored, so they compile in every configuration:

```rust
/// ```
/// # #[cfg(feature = "json")] {
/// // …example needing `json`…
/// # }
/// ```
```

Examples needing external resources (a live database) use `no_run`: they are
still type-checked, just not executed.

## Layer 7 — Feature Matrix

`--all-features` is not sufficient to prove the crate builds. It cannot catch:

- code that is dead unless an optional dependency is enabled,
- bindings left unread when a feature compiles a function body away,
- a feature that does not build **at all** on its own.

Every feature is therefore checked in isolation and in realistic combinations,
with warnings denied.

**Run:**
```bash
just lint-features
```

In CI this is a matrix job, so a failing combination names itself in the job list.

## Keeping the fuzz targets alive

`fuzz/` declares its own `[workspace]`, so `cargo check --workspace` does not see
it. `just check-fuzz` (and a CI step) type-checks the targets on stable; only
*running* them needs nightly.

The targets build with `time` and `decimal` enabled: those two features replace
`String` fields with `time::OffsetDateTime`, `time::Date`, and
`rust_decimal::Decimal` — three parsers over attacker-controlled text that are
not compiled in at all without them.

## CI Safety Notes

When piping test output through `tee` in CI scripts, enable `set -o pipefail`
(or check `PIPESTATUS`) to prevent a failing test command from appearing to succeed:

```bash
set -o pipefail
cargo test --features json,versioned --test golden 2>&1 | tee test-output.log
```

Without `pipefail`, a non-zero exit from `cargo test` is masked by `tee`'s success.

## Running the Full Suite

```bash
# All unit and integration tests (default features)
cargo test --workspace

# All tests with all features
cargo test --workspace --all-features

# Just golden corpus tests
cargo test --features json,versioned --test golden

# Identifier + serde + enum property tests (no extra feature flag needed)
cargo test --all-features --test proptest_roundtrips

# Cross-impl compatibility
cargo test --features json,versioned --test compat

# schemars snapshot tests
cargo test --features schemars --test schemars_snapshots

# Validation integration tests
cargo test --all-features --test validation

# Doctests only
cargo test --all-features --doc

# Every feature combination, warnings denied
just lint-features

# Fuzz (nightly, 1M iterations)
cargo +nightly fuzz run fuzz_deserialize_vertrag -- -runs=1000000

# Everything CI runs
just ci
```
