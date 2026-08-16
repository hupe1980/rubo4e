+++
title = "Serialization"
description = "German camelCase, snake_case, and canonical JSON output; round-trip preservation of unknown fields; and the limits that make parsing untrusted payloads safe."
weight = 30
+++

`rubo4e` supports three JSON output modes and ensures that unknown fields from
external payloads survive a round-trip.

**Required feature:** `json` (implies `serde`)

## Three Output Modes

### Side-by-Side Example

Given the same `Vertrag` value, the three methods produce:

**`to_json_german()`** — BO4E wire format (default):
```json
{
  "_typ": "VERTRAG",
  "_version": "v202607.0.0",
  "sparte": "STROM",
  "marktlokationsId": "51238696781"
}
```

**`to_json_snake_case()`** — snake_case BO4E keys:
```json
{
  "_typ": "VERTRAG",
  "_version": "v202607.0.0",
  "sparte": "STROM",
  "marktlokations_id": "51238696781"
}
```

**`to_json_canonical()`** — deterministic, sorted keys:
```json
{
  "_typ": "VERTRAG",
  "_version": "v202607.0.0",
  "marktlokationsId": "51238696781",
  "sparte": "STROM"
}
```

### The `_typ` and `_version` metadata keys

Both are populated for you when you construct a value — via `Default::default()`,
the typed builder, or `..Default::default()` struct-update syntax:

- **`_typ`** is set on BO types only (`BoTyp::Vertrag`, …). COM types leave it
  unset, matching the Python and Go implementations.
- **`_version`** is set on **every** BO *and* COM to the schema version the module
  was generated from, again matching Python and Go — their nested `Betrag` and
  `Adresse` objects carry `_version` too.

You never need to supply the version string yourself, and you should not
hardcode it: a literal in your code silently goes stale when you upgrade to a new
schema series, producing payloads that misreport their own version. If you do
need it programmatically, read it from `Bo4eObject::schema_version()`.

**Deserialization never overwrites it.** `_version` records the provenance of the
data, so a payload that arrives stamped `v202501.0.0` keeps that value through a
round-trip, and a payload that arrives without `_version` stays without one. Only
construction fills it in. The setter remains available if you need to re-stamp a
value deliberately.

## When to Use Each Mode

| Mode | Use case |
|------|----------|
| `to_json_german()` | BO4E ecosystem interoperability (Python, Go, .NET), EDIFACT-adjacent systems |
| `to_json_snake_case()` | Rust-centric APIs and internal integration formats |
| `to_json_canonical()` | Content-addressed signing, payload hashing, event sourcing, diffing, caching |

> **Note on `to_json_canonical` and RFC 8785 (JCS):** This method sorts object keys
> recursively and produces deterministic output, but is **not** a full RFC 8785
> implementation. Keys are sorted by UTF-8 byte order (not UTF-16 as JCS requires),
> and numeric values use serde_json formatting (not IEEE 754 as JCS requires).
> For BO4E data — ASCII-only field names and `Decimal`-as-string amounts — these
> differences are irrelevant in practice.

## API Reference

```rust
// Requires `json` feature
impl Vertrag {
    pub fn to_json_german(&self) -> Result<String, serde_json::Error>;
    pub fn to_json_snake_case(&self) -> Result<String, serde_json::Error>;
    pub fn to_json_canonical(&self) -> Result<String, serde_json::Error>;
}
```

All three methods:
- Return valid JSON as a `String`
- Skip `None` fields (no `null` values in output)
- Recursively serialize nested BO/COM types

There is no runtime `SerializeConfig` object. Mode is chosen at the call site.

## Deserialization

```rust
// Requires `json` feature
let vertrag: Vertrag = serde_json::from_str(&json_string)?;
```

Deserialization accepts both:
- BO4E German camelCase (`from_json_german`, `from_json_german_bytes`)
- Snake_case key form (`from_json_snake_case`, `from_json_snake_case_bytes`)

Snake_case mode transforms key style only. It is **not** a German->English translation.

### How snake_case keys are mapped

The mapping is an exact table the code generator emits from the same field data
it uses to emit the structs — not a runtime heuristic. `to_json_snake_case()`
followed by `from_json_snake_case()` therefore returns the value you started
with, for every generated type.

An algorithmic inverse cannot achieve that, because several BO4E names collapse
onto a snake form that maps back to a *different* camelCase name:

| Wire key | snake_case | What a heuristic maps back to |
|---|---|---|
| `hoechstpreisHT` | `hoechstpreis_ht` | `hoechstpreisHt` ✗ |
| `kundengruppeKA` | `kundengruppe_ka` | `kundengruppeKa` ✗ |
| `A` (`Sigmoidparameter`) | `a` | `a` ✗ |

With a heuristic those fields deserialize into `_additional` instead of their
typed field — a silent data loss that the table removes by construction.

Two kinds of key are deliberately **not** rewritten, in either direction:

- **BO4E metadata keys** — `_typ`, `_version`, `_id` keep their leading
  underscore in every output mode. They are wire metadata, not Rust field names.
- **Extension keys** — anything the schema does not define passes through
  byte-for-byte, so unknown fields round-trip exactly as they arrived rather than
  being renamed into something their producer would not recognise.

Because every lookup resolves to a `&'static str`, renaming a key allocates on
neither the serialize nor the deserialize path.

### `AnyBo` goes through the same pipeline

`AnyBo` cannot know its concrete type until it has read `"_typ"`, so it buffers
the payload first. It buffers **through the deserializer it was handed**, which is
what puts it on the same footing as a concrete BO type: the key transform above
applies to it, and so does the nesting-depth limit described below.

That costs an intermediate buffer. Deserializing a concrete BO type skips it, so
prefer the concrete type on hot paths where you already know it:

```rust
use rubo4e::current::{AnyBo, Marktlokation};
use rubo4e::json::Bo4eJsonExt;

// Polymorphic ingest — type decided at runtime by `_typ`.
let bo = AnyBo::from_json_german(body)?;

// Known type — no buffering.
let malo = Marktlokation::from_json_german(body)?;
```

### Hardened Deserialization for Untrusted Inputs

For untrusted external payloads, use the hardened APIs with explicit limits:

```rust
use rubo4e::json::{Bo4eJsonExt, JsonParseLimits};
use rubo4e::current::Vertrag;

// `untrusted_defaults()` sets all four caps to conservative values.
let vertrag = Vertrag::from_json_german_hardened(
    &json_string,
    JsonParseLimits::untrusted_defaults(),
)?;

// Or tune individually. Start from `unlimited()` (or `..Default::default()`)
// so the literal stays valid as new caps are added.
let limits = JsonParseLimits {
    max_payload_bytes: Some(1_000_000),
    max_nesting_depth: Some(64),
    max_extension_value_bytes: Some(64_000),
    max_extension_field_count: Some(32),
};
let vertrag = Vertrag::from_json_german_hardened(&json_string, limits)?;
```

Available hardened variants:
- `from_json_german_hardened`
- `from_json_snake_case_hardened`
- `from_json_german_bytes_hardened`
- `from_json_snake_case_bytes_hardened`

#### What each limit means

| Limit | Scope | Enforced |
|---|---|---|
| `max_payload_bytes` | whole input | before parsing starts |
| `max_nesting_depth` | whole document | inline, during the single parse pass |
| `max_extension_value_bytes` | **cumulative across every struct in the payload** | charged per extension field as it is parsed |
| `max_extension_field_count` | **per struct**, at every nesting level | checked as each struct's extension fields are read |

The two extension limits apply at **every nesting level**, not just the root
object. Extension data hidden inside a nested COM — say
`marktlokation.lokationsadresse` — is charged to the same budget as extension
data on the root. Enforcement happens *during* parsing, so an oversized payload
is rejected while it is being read rather than after the whole object tree has
been allocated.

Independently of these opt-in limits, two hard caps always apply, on every
deserialization path including the non-hardened ones:

- `MAX_EXTENSION_FIELDS` (128) — extension fields per struct
- `MAX_EXTENSION_KEY_LEN` (256) — bytes per extension field key

`max_extension_field_count` can only tighten the 128 cap, never loosen it.

Counters for every limit that has fired are available process-wide via
`json_limit_hit_counters()`, and are exported to the `metrics` ecosystem when
the `metrics` feature is on.

## Round-Trip Safety (ExtensionData)

Every BO and COM struct carries an `_additional` field that captures any JSON keys
not recognized by the struct definition:

```rust
pub struct Vertrag {
    // ... known fields ...

    #[serde(flatten)]
    #[serde(skip_serializing_if = "crate::json::ext_map_is_empty")]
    pub _additional: crate::LimitedExtensionMap,
}
```

This means a payload with custom extension fields (common in BO4E implementations that
extend the standard) survives a full round-trip:

```rust
let json = r#"{
  "_typ": "VERTRAG",
  "_version": "v202607.0.0",
  "_customExtension": "some-value"
}"#;

let vertrag: Vertrag = serde_json::from_str(json)?;
assert!(rubo4e::json::Bo4eExtensionData::extension_data(&vertrag)
  .contains_key("_customExtension"));

let roundtripped = vertrag.to_json_german()?;
assert!(roundtripped.contains("_customExtension"));
```

`indexmap::IndexMap` is used (not `std::collections::HashMap`) to preserve the
original key insertion order.

## SIMD-Accelerated Deserialization

**Feature flag:** `simd-json`

For high-throughput scenarios, enable `simd-json` to use a SIMD parser backend for
JSON parsing:

```sh
cargo add rubo4e --features simd-json
```

| Platform | SIMD instruction set |
|----------|---------------------|
| x86_64   | SSE4.2 / AVX2       |
| ARM64    | NEON                |
| Other    | Falls back to `serde_json` |

The feature gate ensures builds remain portable. Performance is workload-dependent;
benchmark with your payload mix before adopting it globally.

> **`simd-json` and hardened parsing.** Setting `max_nesting_depth` pins the call
> to the `serde_json` backend, because depth is enforced by wrapping the visitor
> and `simd-json` does not support that. `untrusted_defaults()` sets a depth cap,
> so the recommended hardened configuration never takes the SIMD path — that is a
> deliberate choice of correctness over throughput on untrusted input. The
> non-hardened entry points still use SIMD above the size threshold and enforce
> the default depth cap with a pre-scan.

Benchmark with your actual payload shapes before committing to `simd-json` in
production. The `benches/` directory provides a comparison benchmark against
`serde_json` and the Python reference implementation.

## Scope Note

This library does not provide HTTP handler code. There are no Axum extractors,
no Actix-web request guards, and no framework-specific `FromRequest` implementations.

Consumers integrate `rubo4e` types into their own HTTP layer. For example, with Axum:

```rust
// In your application code (not in rubo4e):
async fn create_vertrag(
    axum::extract::Json(body): axum::extract::Json<serde_json::Value>,
) -> Result<axum::Json<Vertrag>, AppError> {
    let vertrag: Vertrag = serde_json::from_value(body)?;
    vertrag.validate()?;
    Ok(axum::Json(vertrag))
}
```
