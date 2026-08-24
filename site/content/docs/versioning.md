+++
title = "Schema Versioning"
description = "How BO4E schema releases map onto Rust modules, what rubo4e::current guarantees, and which imports to pin when enum membership must not move underneath you."
weight = 50
+++

`rubo4e` exposes a single stable BO4E schema series (`v202607`), compiled
conditionally behind the `versioned` feature flag.

## Three spellings of one release

BO4E names a release twice, and this crate needs a third name for it — the
*series*, which is the granularity at which a module exists:

| Where | Spelling | Example |
|---|---|---|
| Git tag and schema directory | with `v`, full triple | `v202607.1.0` |
| Rust module | with `v`, series only | `rubo4e::v202607` |
| The `_version` field inside a payload | no `v`, full triple | `202607.1.0` |

`Bo4eObject::schema_version()` returns the **wire** spelling, so it can be
compared against a `_version` read off a message without a normalisation step —
never against the tag.

The series — the bare `YYYYMM` prefix — is the one to dispatch on, because it is
the one that maps onto a set of Rust types. `Bo4eObject::schema_series()` returns
it.

## Multi-version Dispatch

When your storage layer persists a `bo4e_version` column alongside the JSON payload
(common in JSONB-column designs), the idiomatic dispatch is a plain `match` — **on
the series, not on the exact release**:

```rust
use rubo4e::{v202607, Bo4eObject as _};

/// The `YYYYMM` prefix of a `_version` value: `"202607.1.0"` → `"202607"`.
fn series_of(wire_version: &str) -> &str {
    wire_version.split('.').next().unwrap_or(wire_version)
}

fn process_rechnung(json: &str, bo4e_version: &str) -> Result<(), Box<dyn std::error::Error>> {
    // `bo4e_version` is the payload's own `_version` — no `v` prefix.
    match series_of(bo4e_version) {
        "202607" => {
            let r: v202607::Rechnung = serde_json::from_str(json)?;
            // r.schema_series() == "202607"  ← always matches this arm
            handle_v202607(r)
        }
        // When the v202801 series ships, add one arm and a migration shim if needed:
        // "202801" => handle_v202801(serde_json::from_str::<v202801::Rechnung>(json)?),
        _ => Err(format!("unsupported schema series: {bo4e_version}").into()),
    }
}
```

**Do not match on the full `_version`.** BO4E ships patch releases inside a
series — `202607.0.0`, then `202607.1.0` — and a sender one patch ahead of you
stamps a string an equality match rejects, for a payload the `v202607` types
deserialize perfectly. `Bo4eObject::schema_series()` returns exactly the value
this `match` keys on, so a test can assert the two agree.

Key points:
- `schema_series()` and `schema_version()` are on every BO type via the `Bo4eObject` trait — no new API needed
- Each new schema *series* is exactly one `match` arm; patches inside a series need none
- Business logic (`handle_v202607`, `handle_v202801`, …) only handles the series it was written for
- Older series can be migrated before the branch (`FROM v202607 TO v202801`) or handled by a thin shim inside the arm
- No trait objects, no `Any*` enums required for this straightforward branching

## Version Module Layout

With the `versioned` feature enabled:

```rust
rubo4e::v202607::Vertrag       // the v202607 series
rubo4e::v202607::Adresse
rubo4e::v202607::Sparte

rubo4e::current::Vertrag       // moving alias — whichever series is newest
```

Without the `versioned` feature, none of these module paths exist. The default
feature set (`identifiers`, which pulls in `serde`) does not include versioned types.

## Feature Gate

```sh
# Enable version modules (pure conditional compilation; no external deps)
cargo add rubo4e --features versioned
```

## Known Schema Series

| Series  | Snapshot in this release | Status         | Released  |
|---------|--------------------------|----------------|-----------|
| v202607 | v202607.1.0              | Current stable | July 2026 |

The snapshot column is the exact BO4E tag `src/generated/v202607/` was built
from; it lives under `generator/schemas/` and is committed, so the codegen is
reproducible. It advances when BO4E ships a release inside the series — see
[the contract below](#what-pinning-does-and-does-not-buy-you).

### Versioning Scheme

BO4E uses `vYYYYMM.minor.patch`.  Module names use the `vYYYYMM` prefix only:

```
v202607.1.0  →  module: v202607
v202701.0.0  →  module: v202701   (hypothetical next series)
```

The generator pins the full tag for reproducibility but exposes only the series
prefix in the public API.

**A minor bump inside a series is not necessarily additive.** BO4E removes enum
values and whole types within a series, so anything treating one as a frozen
value set will eventually be wrong. The contract below says what is stable.

## `rubo4e::current` — Moving Alias

`rubo4e::current` is a moving re-export module (a real `pub mod`, not a
`pub use … as` alias) that always points to the latest stable schema series.  Use
it when you always want the newest types and do not need to pin to a specific
version.

```rust
use rubo4e::current::Vertrag;   // equivalent to rubo4e::v202607::Vertrag today
```

Pin to a concrete module if you need version-stability across crate updates:

```rust
use rubo4e::v202607::Vertrag;   // stable even if rubo4e::current advances
```

### What pinning does, and does not, buy you

| Path                   | What a **minor `rubo4e` bump** can do to it |
|------------------------|---------------------------------------------|
| `rubo4e::v202607::Foo` | Keep the **series**. Field names and types stay put; enum membership can still move, because BO4E itself moves it inside a series. |
| `rubo4e::current::Foo` | Anything the above can do, **plus jump to a new series** — renamed fields, retyped fields, whole types added or removed. |

So the honest statement is:

> **The Rust module path pins the series. The `rubo4e` version pins the values.**

If a variant set must not move under you, pin the **crate version** in
`Cargo.toml` (`rubo4e = "=0.11.0"`) and upgrade deliberately. Importing
`rubo4e::v202607::Sparte` instead of `rubo4e::current::Sparte` narrows the blast
radius — you will not silently jump a format-version cutover — but it does not
freeze the enum.

The rest is a test, not a promise. Anything whose shape you guard should assert
it structurally, so a schema bump fails in CI instead of in production:

- SQL `CHECK (col IN (...))` lists generated from an enum's variants
- Exhaustive `match` / mapping tables over an enum
- Variant-count assertions (`assert_eq!(T::COUNT, N)`)

The `strum`-free introspection surface is there for exactly this:

```rust
use rubo4e::{Bo4eEnum, v202607::Zaehlertyp};

// Structural drift guard — no magic number to update by hand:
#[test]
fn sql_check_list_covers_every_variant() {
    let sql: Vec<&str> = load_check_list();     // your migration's CHECK list
    for v in Zaehlertyp::VARIANTS {
        assert!(sql.contains(&v.as_wire()), "CHECK list missing {}", v.as_wire());
    }
}
```

Note the direction: this asserts the CHECK list *covers* every variant, so an
**added** variant fails it. To catch a **removed** one too, assert set equality
instead.

### Schema-delta changelog

Every release that changes schema-derived enum membership or codelist coverage
records it in the [`CHANGELOG.md`](https://github.com/hupe1980/rubo4e/blob/main/CHANGELOG.md) **Schema deltas** section, in
the form:

```
### Schema deltas   (<old tag> → <new tag>)
- <Enum>         +2 (NEW_A, NEW_B)  -1 (REMOVED_C)
- <OtherEnum>    +1 (NEW_D)
- removed enums: <Type>, <Type>
```

Removals are listed as prominently as additions: they are the half that breaks a
build. `T::COUNT` and `T::VARIANTS` turn the drift into a test failure the moment
you upgrade.

## Upgrading within a series

BO4E ships a new patch inside the series the crate is already on. The module path
does not change, so this is three commands and a changelog entry.

```bash
just download-schemas v202607.1.0     # vendors the snapshot under generator/schemas/
git rm -r generator/schemas/v202607.0.0   # exactly one snapshot per series
just generate v202607.1.0
just ci
```

The generator rewrites every file in `src/generated/v202607/` **and deletes the
ones the release retired** — a type BO4E drops leaves no orphan module behind
pretending to be live. Nothing in the tree hard-codes the tag: the justfile
default and the test helpers read it off the snapshot directory, so the only
manual edits are the table above and the changelog.

Then read the diff. `git diff src/generated/` shows every membership change, and
the ones that matter are the removals — those are what break a downstream build,
and they belong in the **Schema deltas** entry.

## Adding a New Schema Series

When BO4E's annual format-version cutover lands, with new or renamed types:

1. **Download the schema snapshot** using the provided script:
   ```bash
   just download-schemas v202701.0.0
   ```
2. **Run the generator:**
   ```bash
   just generate v202701.0.0
   ```
3. The generator writes `src/generated/v202701/` with all types and automatically
   updates `src/generated/mod.rs` (by re-scanning the directory — no manual edit
   needed).
4. In `src/lib.rs`, add a versioned re-export module:
   ```rust
   #[cfg(feature = "versioned")]
   pub mod v202701 {
       pub use crate::generated::v202701::*;
   }
   ```
5. Advance the `current` module to re-export the new series (it is a real
   `pub mod`, not a `pub use … as` alias, so IDE tooling resolves hovers as
   `rubo4e::current::Foo`):
   ```rust
   #[cfg(feature = "versioned")]
   pub mod current {
       pub use crate::generated::v202701::*;  // was: v202607
   }
   ```
6. Update the convenience module (`src/convenience.rs`) if schema-breaking changes
   require updating field references (e.g. renamed fields in `Rechnung`,
   `Rechnungsposition`).
7. Update the Known Schema Series table in this document, and keep the
   retiring series listed until you remove its module.
8. Record a **Schema deltas** section in [`CHANGELOG.md`](https://github.com/hupe1980/rubo4e/blob/main/CHANGELOG.md) listing
   every enum whose membership changed and every codelist code added/removed
   (e.g. `Zaehlertyp +2 (…)`). Downstream projects rely on this to update pinned
   guards deliberately. Diffing `T::VARIANTS` between the old and new series makes
   this mechanical.

## COM and Enum Versioning

COM and enum types live inside the versioned module alongside BO types.  They
follow exactly the same conditional-compilation rules.

## Schema Breaking Changes

The BO4E annual format-version cutover can rename fields, change optionality, and
add or remove whole types. What a given cutover changed is recorded in the
[CHANGELOG](https://github.com/hupe1980/rubo4e/blob/main/CHANGELOG.md); the
authoritative diff is between the snapshots under `generator/schemas/`.

The generator does **not** paper over such changes: a `$ref`, a `"format"`, and
`"type": "number"` are always authoritative, so a renamed or retyped field
surfaces as a compile error rather than as a silent behaviour change. See
[Semantic Field Typing](@/docs/generator.md#semantic-field-typing).
