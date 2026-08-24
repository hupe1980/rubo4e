+++
title = "Architecture"
description = "Workspace layout, module tree, and the feature-gate taxonomy — plus where the boundary sits between this library and your application code."
weight = 10
+++

This page describes the workspace layout, module structure, feature gate taxonomy,
and the design boundary between this library and application code.

## Workspace Layout

```
rubo4e/
├── Cargo.toml               — workspace root ([workspace] + [package name = "rubo4e"])
├── deny.toml                — cargo-deny policy (licences, advisories)
├── justfile                 — build / generate / test recipes
├── site/                    — this documentation site (Zola)
├── src/                     — bo4e crate source
│   ├── lib.rs               — crate root; feature-gated re-exports, prelude, Bo4eObject trait
│   ├── error.rs             — IdentifierError and LengthExpectation types
│   ├── json/                — Bo4eJsonExt, Bo4eExtensionData, LimitedExtensionMap
│   ├── schema_helpers.rs    — schemars schema_with= helpers for OffsetDateTime and Date
│   ├── time_serde.rs        — date_serde / opt_date_serde modules (time feature)
│   ├── convenience.rs       — hand-written ergonomic methods on generated types
│   ├── identifiers/         — validated domain newtypes
│   │   ├── macros.rs        — trait boilerplate + §8.2 identifier generator
│   │   ├── checksum.rs      — BDEW chapter-8 check-digit arithmetic (one impl)
│   │   ├── ascii_ids.rs     — NeloId, NebeId, CrId, SgId, SrId, TrId, PaketId
│   │   ├── malo_id.rs, marktpartner_id.rs, melo_id.rs
│   │   └── eic_code.rs, bilanzkreis_id.rs, obis_code.rs, akiv_id.rs, …
│   ├── validation/          — garde-based cross-field validators
│   └── generated/           — written by generator; never pub outside crate
│       ├── mod.rs           — re-exports v202607
│       └── v202607/         — flat .rs files, one per BO/COM/enum type
│           ├── mod.rs       — re-exports all types + BoTyp / ComTyp discriminants
│           ├── marktlokation.rs
│           ├── vertrag.rs
│           └── …
│
├── generator/               — internal code generator; never published
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs
│   │   ├── parser.rs        — JSON Schema → AST
│   │   ├── inference.rs     — semantic type inference, where the schema is vague
│   │   ├── naming.rs        — BO4E wire values → Rust identifiers
│   │   └── emitter.rs       — AST → Rust source
│   ├── schemas/
│   │   └── v202607.1.0/     — pinned schema snapshot
│   └── tests/
│       ├── round_trip.rs    — generator snapshot tests
│       └── snapshots/       — expected generator output
│
├── fuzz/                    — cargo-fuzz targets
│   └── fuzz_targets/
│       └── fuzz_deserialize_vertrag.rs
│
├── examples/                — runnable usage examples
│   ├── builder.rs
│   └── serialize.rs
│
└── tests/
    ├── generated_contract.rs — drift guards: schemas ↔ committed generated code
    ├── golden/              — official JSON payloads for round-trip tests (flat, no version subdir)
    ├── compat/              — cross-implementation compatibility vectors
    │   ├── python/
    │   └── go/
    └── snapshots/           — insta snapshots (schemars JSON Schema output)
```

## From schema to your imports

The generator is the only thing that writes `src/generated/`. Everything below it
is a re-export, which is why a schema bump never requires hand-editing types.

<figure>
<svg viewBox="0 0 860 200" xmlns="http://www.w3.org/2000/svg" role="img"
     aria-labelledby="pipeline-title pipeline-desc">
  <title id="pipeline-title">The rubo4e code-generation pipeline</title>
  <desc id="pipeline-desc">A pinned BO4E JSON Schema release is read by the generator
  binary, which writes Rust source into src/generated. That module is re-exported as
  rubo4e::v202607, which in turn is re-exported as the moving alias rubo4e::current.</desc>
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="9" refY="5"
            markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M0 0 10 5 0 10z" fill="currentColor"/>
    </marker>
    <style>
      .box  { fill: var(--bg-soft); stroke: var(--border); stroke-width: 1.5; }
      .gen  { fill: var(--accent-soft); stroke: var(--accent); stroke-width: 1.5; }
      .lbl  { fill: var(--fg); font: 600 13px var(--mono); }
      .sub  { fill: var(--fg-muted); font: 400 11px var(--sans); }
      .edge { stroke: var(--fg-muted); stroke-width: 1.5; fill: none; color: var(--fg-muted); }
      .edgelbl { fill: var(--fg-muted); font: 400 10.5px var(--sans); }
    </style>
  </defs>

  <rect class="box" x="4"   y="52" width="150" height="56" rx="9"/>
  <text class="lbl" x="79"  y="76"  text-anchor="middle">schemas/</text>
  <text class="sub" x="79"  y="94"  text-anchor="middle">pinned release</text>

  <rect class="gen" x="196" y="52" width="150" height="56" rx="9"/>
  <text class="lbl" x="271" y="76"  text-anchor="middle">generator/</text>
  <text class="sub" x="271" y="94"  text-anchor="middle">never published</text>

  <rect class="box" x="388" y="52" width="160" height="56" rx="9"/>
  <text class="lbl" x="468" y="76"  text-anchor="middle">src/generated/</text>
  <text class="sub" x="468" y="94"  text-anchor="middle">not public</text>

  <rect class="box" x="590" y="16"  width="176" height="56" rx="9"/>
  <text class="lbl" x="678" y="40"  text-anchor="middle">::v202607</text>
  <text class="sub" x="678" y="58"  text-anchor="middle">pin for stable membership</text>

  <rect class="box" x="590" y="106" width="176" height="56" rx="9"/>
  <text class="lbl" x="678" y="130" text-anchor="middle">::current</text>
  <text class="sub" x="678" y="148" text-anchor="middle">tracks newest series</text>

  <path class="edge" d="M154 80 H190"  marker-end="url(#arrow)"/>
  <text class="edgelbl" x="172" y="72" text-anchor="middle">reads</text>

  <path class="edge" d="M346 80 H382"  marker-end="url(#arrow)"/>
  <text class="edgelbl" x="364" y="72" text-anchor="middle">writes</text>

  <path class="edge" d="M548 80 H570 Q582 80 582 68 V56 Q582 44 594 44" marker-end="url(#arrow)"/>
  <path class="edge" d="M548 80 H570 Q582 80 582 92 V122 Q582 134 594 134" marker-end="url(#arrow)"/>
  <text class="edgelbl" x="566" y="104" text-anchor="middle">re-exported as</text>
</svg>
<figcaption>
  Both import paths resolve to the same types. Choose <code>::current</code> to follow
  the newest schema series automatically, or the version module to stay on one series
  across a format-version cutover.
</figcaption>
</figure>

## What a generated type gives you

Every generated struct and enum carries the same trait surface, so nothing about
a type has to be looked up per type.

| | BO / COM structs | Enums |
|---|---|---|
| Always | `Debug`, `Clone`, `PartialEq`, `Default`&nbsp;\* | `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `PartialOrd`, `Ord`, `Hash` |
| Without `json` | **`Eq`, `Hash`** | — |
| `versioned` | `Bo4eObject` (BOs only), `Bo4eStrict` | `Bo4eEnum`, `VARIANTS` / `COUNT` / `as_wire` / `from_wire` / `iter_known` |
| `json` | `Bo4eJsonExt`, `Bo4eExtensionData`, `Display` | — |
| `serde` | `Serialize`, `Deserialize` | `Serialize`, `Deserialize` |
| `builder` | `TypedBuilder` | — |
| `validate` | `garde::Validate` | — |
| `schemars` / `utoipa` | `JsonSchema` / `ToSchema` | `JsonSchema` / `ToSchema` |
| `strum` | — | `EnumString`, `EnumIter`, `IntoStaticStr` |
| `sqlx` | — | `Type` / `Encode` / `Decode` / `PgHasArrayType` |

\* `Default` is absent on the two structs BO4E declares a *required* field on
(`Lastgang`, `Tarif`) — a required field's type need not have one. Those stamp
`_typ` and `_version` through the builder instead.

**`Eq` and `Hash` on structs move together**, and only without `json`. One type
blocks both: `serde_json::Value`, which reaches a generated struct through
`_additional` and `ZusatzAttribut.wert` and is neither, because it wraps `f64`.
With `json` off those become a zero-sized stub and a `String`, every remaining
field type is `Eq + Hash`, and a BO can key a `HashMap` — which a `Hash` without
an `Eq` could not.

**`Ord` on enums is declaration order**, `Unknown` last. A total order, which is
what `BTreeMap` and `sort()` need, but not a business ranking — and a release may
reorder the values, so never persist a sort key derived from it. Compare
`as_wire()` where the order must be stable across releases.

## Feature Gate Reference

| Feature | Default | External dep added | MSRV impact | Description |
|---------|---------|-------------------|-------------|-------------|
| `identifiers` | ✓ | `serde` | none | Identifier newtypes with serde, without the versioned schema |
| `serde` | ✓ (via `identifiers`) | `serde` | none | Derive `Serialize`/`Deserialize` on all types |
| `json` | — | `serde_json` | none | `to_json_*()` methods; `serde` implied |
| `time` | — | `time` | none | `OffsetDateTime` for datetime fields; `Date` for date-only fields; enables `rubo4e::time_serde` when `serde` is also on |
| `decimal` | — | `rust_decimal` | none | `Decimal` for all monetary/quantity fields; without it they are `String` and still accept a JSON number |
| `builder` | — | `typed-builder` | none | Typed builder derives on all BO/COM structs |
| `validate` | — | `garde` | none | `.validate()` method on all structs |
| `schemars` | — | `schemars` | none | `JsonSchema` derive on all types; enables `rubo4e::schema_helpers` |
| `versioned` | — | none | none | Conditional compilation of `v202607` and `current` modules; enables `rubo4e::convenience`, `rubo4e::strict`, and the `Bo4eEnum` / `Bo4eStrict` traits |
| `sqlx` | — | `sqlx` | none | `sqlx::Type`/`Encode`/`Decode`/`PgHasArrayType` for every identifier and every enum; no `json` required — both directions go through `&str` |
| `utoipa` | — | `utoipa` | none | `ToSchema` derive on all types |
| `strum` | — | `strum` | none | `FromStr`, `EnumIter`, `Into<&'static str>` on all enums (`Display`/`AsRef<str>`/introspection are always on) |
| `tracing` | — | `tracing` | none | Structured diagnostics (identifier failures, extension-data events) |
| `metrics` | — | `metrics` | none | Counter export hooks (metrics ecosystem) |

> **MSRV:** The library targets Rust ≥ **1.88** (set in `Cargo.toml` via `rust-version`). No feature raises the floor: the binding constraint is the always-available dependency tree (`time` and `home` via `sqlx` both require 1.88).

## Design Boundary

This library provides **types** and **domain logic**. It does not provide:

- HTTP handler code (no Axum extractors, no Actix-web guards)
- Database migration scripts
- gRPC service definitions
- Anything that requires knowledge of a specific application framework

Consumers compose `rubo4e` types with their own HTTP, persistence, or messaging layer.

## Code Generation Policy

The generator (`generator/`) is the only component that writes to `src/generated/`.
Generated code is **committed to the repository** so that:

1. `cargo build` works without running the generator
2. Code review can inspect schema-driven changes as an ordinary diff

Committing it means nothing at build time forces the output to match the schemas,
so `tests/generated_contract.rs` and `just check-docs-drift` supply that force:
they read both sides and compare. See [Code Generator](@/docs/generator.md#drift-guards).

The `generated/` subtree is never `pub` beyond the crate boundary. All public types
are flat-re-exported through `src/generated/v<version>/mod.rs`, which is then
re-exported from the version-gated module in `src/lib.rs` (e.g., `pub mod v202607`).

## MSRV Policy

- `rust-version = "1.88"` is set in the root `Cargo.toml`, and CI checks the crate on exactly
  that toolchain so the key is a verified claim rather than an unchecked promise.
- The floor tracks the **dependency tree**, not this crate's own source: `time`
  and `home` (via `sqlx`) require 1.88, and no individual feature raises it further.
- A below-floor toolchain fails during dependency *resolution*, not compilation, because the
  default resolver ignores `rust-version` when picking versions. The error names the offending
  packages; pin them with `cargo update <crate> --precise <version>` to build on an older
  toolchain.
- Raising the floor is a **minor** version bump, never a patch.
