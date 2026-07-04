# Serialization

`rubo4e` supports three JSON output modes and ensures that unknown fields from
external payloads survive a round-trip.

**Required feature:** `json` (implies `serde`)

---

## Three Output Modes

### Side-by-Side Example

Given the same `Vertrag` value, the three methods produce:

**`to_json_german()`** — BO4E wire format (default):
```json
{
  "_typ": "VERTRAG",
  "sparte": "STROM",
  "marktlokationsId": "51238696780"
}
```

> **Note:** The `_version` field is `Option<String>` and is **not** auto-populated by
> the library.  Set it explicitly when your integration requires it.

**`to_json_snake_case()`** — snake_case BO4E keys:
```json
{
  "_typ": "VERTRAG",
  "sparte": "STROM",
  "marktlokations_id": "51238696780"
}
```

**`to_json_canonical()`** — deterministic, sorted keys:
```json
{
  "_typ": "VERTRAG",
  "marktlokationsId": "51238696780",
  "sparte": "STROM"
}
```

---

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

---

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

---

## Deserialization

```rust
// Requires `json` feature
let vertrag: Vertrag = serde_json::from_str(&json_string)?;
```

Deserialization accepts both:
- BO4E German camelCase (`from_json_german`, `from_json_german_bytes`)
- Snake_case key form (`from_json_snake_case`, `from_json_snake_case_bytes`)

Snake_case mode transforms key style only. It is **not** a German->English translation.

### Hardened Deserialization for Untrusted Inputs

For untrusted external payloads, use the hardened APIs with explicit limits:

```rust
use rubo4e::json::{Bo4eJsonExt, JsonParseLimits};
use rubo4e::v202501::Vertrag;

let limits = JsonParseLimits {
  max_payload_bytes: Some(1_000_000),
  max_nesting_depth: Some(64),
  max_extension_value_bytes: Some(64_000),
};

let vertrag = Vertrag::from_json_german_hardened(&json_string, limits)?;
```

Available hardened variants:
- `from_json_german_hardened`
- `from_json_snake_case_hardened`
- `from_json_german_bytes_hardened`
- `from_json_snake_case_bytes_hardened`

These methods enforce payload and structure budgets before returning typed values.

---

## Round-Trip Safety (ExtensionData)

Every BO and COM struct carries an `_additional` field that captures any JSON keys
not recognized by the struct definition:

```rust
pub struct Vertrag {
    // ... known fields ...

    #[serde(flatten)]
  #[serde(skip_serializing_if = "crate::json::ext_map_is_empty")]
  pub(crate) _additional: crate::json::LimitedExtensionMap,
}
```

This means a payload with custom extension fields (common in BO4E implementations that
extend the standard) survives a full round-trip:

```rust
let json = r#"{
  "_typ": "VERTRAG",
  "_version": "v202501.0.0",
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

---

## SIMD-Accelerated Deserialization

**Feature flag:** `simd-json`

For high-throughput scenarios, enable `simd-json` to use a SIMD parser backend for
JSON parsing:

```toml
rubo4e = { version = "...", features = ["simd-json"] }
```

| Platform | SIMD instruction set |
|----------|---------------------|
| x86_64   | SSE4.2 / AVX2       |
| ARM64    | NEON                |
| Other    | Falls back to `serde_json` |

The feature gate ensures builds remain portable. Performance is workload-dependent;
benchmark with your payload mix before adopting it globally.

Benchmark with your actual payload shapes before committing to `simd-json` in
production. The `benches/` directory provides a comparison benchmark against
`serde_json` and the Python reference implementation.

---

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
