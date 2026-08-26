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
  "_version": "202607.1.0",
  "sparte": "STROM",
  "marktlokationsId": "51238696781"
}
```

**`to_json_snake_case()`** — snake_case BO4E keys:
```json
{
  "_typ": "VERTRAG",
  "_version": "202607.1.0",
  "sparte": "STROM",
  "marktlokations_id": "51238696781"
}
```

**`to_json_canonical()`** — deterministic, sorted keys:
```json
{
  "_typ": "VERTRAG",
  "_version": "202607.1.0",
  "marktlokationsId": "51238696781",
  "sparte": "STROM"
}
```

### The `_typ` and `_version` metadata keys

Both are populated for you when you construct a value — via `Default::default()`,
the typed builder, or `..Default::default()` struct-update syntax. Both are read
from the schema, so neither can drift from what the standard declares:

- **`_typ`** is set on **every** BO *and* COM — each BO4E schema pins its
  discriminant with a JSON Schema `const`, and every reference implementation
  stamps it, nested `Betrag` and `Adresse` components included.
- **`_version`** is set on every BO and COM to the release the schema declares.

#### `_version` has no `v`

BO4E tags its schema releases `v202607.1.0`, but the `_version` value **inside a
payload** is `202607.1.0`. `Bo4eObject::SCHEMA_VERSION` is the wire
spelling, so it compares directly against a `_version` read off a message.

Do not hardcode either string — a literal goes stale on the next schema series.

**Deserialization never overwrites it.** `_version` records the provenance of the
data, so a payload that arrives stamped `202501.0.0` keeps that value through a
round-trip, and a payload that arrives without `_version` stays without one. Only
construction fills it in. The setter remains available if you need to re-stamp a
value deliberately.

### Decimal amounts are written as JSON strings

`Betrag.wert`, `Preis.wert`, `Menge.wert` and every other `Decimal` field
serializes as a **quoted string**:

```json
{ "_typ": "BETRAG", "wert": "119.00", "waehrung": "EUR" }
```

This matches the reference implementation: BO4E-python models these fields as
`decimal.Decimal`, and pydantic v2 serializes `Decimal` to a string in JSON mode.
It also avoids the precision loss an IEEE-754 double would introduce.

Two consequences:

- **The published BO4E JSON Schema says `"type": "number"`** here, because
  pydantic generates it in *validation* mode. Output from BO4E-python and from
  rubo4e therefore both fail strict validation against BO4E's own schema —
  an upstream inconsistency, not one to work around.
- **Deserialization accepts both spellings.** A producer writing `"wert": 119.00`
  as a JSON number (go-bo4e does) is read fine; `tests/compat/` covers both.

#### …but only the string spelling is exact

Serde's data model has no arbitrary-precision number, so a JSON number is already
an `f64` before any deserializer here is called:

| Wire | Result |
|---|---|
| `"wert": "119.00"` | `119.00` — scale kept |
| `"wert": 119.00` | `119` — **scale lost** |
| `"wert": "12345678901234567890.12"` | exact — 28 significant digits fit |
| `"wert": 12345678901234567890.12` | `12345678901234567000` — **rounded** |
| `"wert": 9007199254740993` | exact — integers skip the `f64` path |

No amount in the German energy market reaches 15 significant digits, so this is a
fidelity question rather than a correctness one: a relayed go-bo4e payload comes
out as `"119"` where the sender wrote `119.00`, and the two compare equal as
`Decimal`.

The loss is unrecoverable, so it is made visible instead:

```rust
use rubo4e::decimal_serde::decimal_from_json_number_count;

// Process-wide and monotonic; also exported as
// `bo4e_decimal_from_json_number_total` with the `metrics` feature.
// Zero means every producer on this link spells decimals as strings.
gauge("bo4e_decimal_from_json_number", decimal_from_json_number_count());
```

Without the `decimal` feature the field is a `String` holding the lexical form,
so `"119.00"` survives as written — at the cost of having no arithmetic.

## When to Use Each Mode

| Mode | Use case |
|------|----------|
| `to_json_german()` | BO4E ecosystem interoperability (Python, Go, .NET), EDIFACT-adjacent systems |
| `to_json_snake_case()` | Rust-centric APIs and internal integration formats |
| `to_json_canonical()` | Content-addressed signing, payload hashing, event sourcing, diffing, caching |

> **Note on `to_json_canonical` and RFC 8785 (JCS):** This method sorts object keys
> recursively — through every serde shape, including sequences, tuples, and enum
> variants — and produces deterministic output, but it is **not** a full RFC 8785
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
- **Extension keys** — anything the schema does not define is never renamed, so
  unknown fields round-trip under the names their producer chose rather than
  under something it would not recognise.

Because every lookup resolves to a `&'static str`, renaming a key allocates on
neither the serialize nor the deserialize path.

#### The transform stops at the edge of the schema

The second bullet holds for the whole subtree under an extension key, not just
the key itself:

```json
{ "_typ": "MARKTLOKATION", "vendorBlob": { "a": 3, "marktlokations_id": "x" } }
```

`a` and `marktlokations_id` come back out spelled exactly that way, even though
`A` is `Sigmoidparameter`'s field and `marktlokations_id` is `Marktlokation`'s.
Keys are renamed as the parser yields them, before serde knows which struct they
belong to, so an unscoped transform would rewrite the producer's own JSON into
names it does not use. It therefore descends only under keys the schema defines,
and switches off for the rest of that subtree.

#### …with two ambiguities it cannot resolve

Both follow from the same root — the transform runs before serde knows the type:

1. **A top-level extension key that *is* a field's snake spelling.** A
   `Marktlokation` carrying an unknown top-level `marktlokations_id` is
   indistinguishable from the real field once written in snake form, so
   `from_json_snake_case` reads it as the field — and rejects the payload if the
   value is not a valid MaLo-ID.
2. **`ZusatzAttribut.wert`.** Its value is free-form JSON, but `wert` is also
   `Betrag`'s decimal and `Messwert`'s nested COM, so the name cannot be excluded
   from the schema-key set without breaking the last of those. Object keys inside
   a `ZusatzAttribut.wert` are renamed like schema keys.

**Use `to_json_german` / `from_json_german` whenever extension data matters.**
The German mode renames nothing, so neither ambiguity exists there.

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

// Or start from a profile and narrow it.  `JsonParseLimits` is
// `#[non_exhaustive]`: new caps get added as new amplification paths are found,
// and a struct literal would make every one of those a breaking change.
let limits = JsonParseLimits::untrusted_defaults()
    .with_max_payload_bytes(Some(64 * 1024))
    .with_max_extension_field_count(Some(0));   // reject any unknown field
let vertrag = Vertrag::from_json_german_hardened(&json_string, limits)?;

// `unlimited()` turns every cap off — useful as a base when exactly one matters.
let depth_only = JsonParseLimits::unlimited().with_max_nesting_depth(Some(16));
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
data on the root, and both are checked as that struct's extension fields are
read rather than after the whole object tree has been built.

They bound what a payload leaves **retained**, not what parsing it allocates:
`#[serde(flatten)]` routes unknown keys into the extension map by buffering a
struct's unrecognised entries into an intermediate `Content` first, so those
fields exist in memory before the count cap fires. `max_payload_bytes` is the
only cap applied before any parsing, which makes it the one that bounds peak
memory — set it first.

Independently of these opt-in limits, two hard caps always apply, on every
deserialization path including the non-hardened ones:

- `MAX_EXTENSION_FIELDS` (128) — extension fields per struct
- `MAX_EXTENSION_KEY_LEN` (256) — bytes per extension field key

`max_extension_field_count` can only tighten the 128 cap, never loosen it.

Counters for every limit that has fired are available process-wide via
`json_limit_hit_counters()`, and are exported to the `metrics` ecosystem when
the `metrics` feature is on.

#### What these limits do *not* bound

They bound the parser, not the object graph it produces. A payload well inside
`max_payload_bytes` can still expand by a large factor: `[{},{},{}…]` is three
bytes per element on the wire and one fully-sized struct per element in memory,
so a 1 MB body can allocate on the order of a hundred megabytes of `Vec`.

Size `max_payload_bytes` against the *expanded* cost rather than the wire cost,
and put a concurrency limit in front of the endpoint — a per-request cap does
not bound what a thousand concurrent requests hold at once.

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
  "_version": "202607.1.0",
  "_customExtension": "some-value"
}"#;

let vertrag: Vertrag = serde_json::from_str(json)?;
assert!(rubo4e::json::Bo4eExtensionData::extension_data(&vertrag)
  .contains_key("_customExtension"));

let roundtripped = vertrag.to_json_german()?;
assert!(roundtripped.contains("_customExtension"));
```

`indexmap::IndexMap` is used (not `std::collections::HashMap`) so the
**top-level** extension keys keep the order they arrived in.

Everything nested under an extension key keeps its names and values, and is
never renamed — see [the transform's scoping
rule](#the-transform-stops-at-the-edge-of-the-schema). Key **order inside** a
nested object is not kept: below the top level a value is a `serde_json::Value`,
whose objects are a sorted map, so `{"b":1,"a":2}` comes back as `{"a":2,"b":1}`.
Enable `serde_json`'s `preserve_order` in your own `Cargo.toml` if that ordering
matters; feature unification applies it here too.

### Two caps you cannot turn off

Preserving unknown fields is a memory-growth surface, so the extension map
enforces two hard limits on **every** deserialization path, hardened or not:
`MAX_EXTENSION_FIELDS` (128) per struct, and `MAX_EXTENSION_KEY_LEN` (256 bytes)
per key. `JsonParseLimits::max_extension_field_count` can tighten the first,
never loosen it.

The same caps apply to programmatic writes: `LimitedExtensionMap::try_insert`
returns `Err(ExtensionInsertError)` rather than growing past either, and no
`&mut IndexMap` is exposed anywhere — handing one out would make both advisory.
Replacing an existing key is always allowed, even at capacity, since it does not
grow the map.

## Why there is no SIMD backend

A SIMD JSON parser does not help this crate, and measurement says so at every
payload size from 265 bytes to 166 KB:

| Payload | `serde_json` | `simd-json` |
|---|---|---|
| 1.7 KB | 5.65 µs | 8.89 µs |
| 16.7 KB | 55.7 µs | 75.6 µs |
| 166 KB | 544 µs | 676 µs |

The reason is structural. Every generated struct carries `#[serde(flatten)]` for
its extension map, so deserialization is dominated by serde's `Content`
buffering, not by the tokenizer SIMD accelerates. `simd-json`'s mutable-slice API
then forces a `Vec<u8>` copy of every payload, and its parser cannot wrap a
visitor, so the nesting-depth guard needs a second pass over the bytes.

`benches/json_perf.rs` tracks the `serde_json` path; re-run it before assuming
this conclusion still holds for your payload shapes.

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
