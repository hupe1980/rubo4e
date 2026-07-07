# Ecosystem Integrations

`rubo4e` provides optional integrations with common Rust ecosystem crates.
Every integration is behind a feature gate and adds zero overhead when disabled.

> **Scope:** This library provides **types**. It does not contain HTTP handler code,
> Axum extractors, Actix-web guards, or any framework-specific glue code.
> Consumers compose `rubo4e` types with their own HTTP and persistence layers.

---

## schemars — JSON Schema Generation

**Feature flag:** `schemars`  
**Dependency:** `schemars = "1"`

Derive `JsonSchema` on all types to generate JSON Schema documents from Rust code.
Useful for API documentation, input validation pipelines, and tooling integration.

```toml
rubo4e = { version = "...", features = ["schemars"] }
```

```rust
use schemars::{schema_for, JsonSchema};

let schema = schema_for!(rubo4e::v202501::Vertrag);
let json = serde_json::to_string_pretty(&schema)?;
println!("{json}");
```

Identifier types appear as `{ "type": "string" }` in the schema — not as JSON objects.
This matches the wire format and keeps schemas interoperable with non-Rust consumers.

---

## sqlx — Database Type Impls

**Feature flag:** `sqlx`  
**Dependency:** `sqlx = "0.8"`  
**Primary target:** PostgreSQL (TEXT column type)

Store and query BO4E identifiers and enums directly in SQL queries without manual
string conversion.

```toml
rubo4e = { version = "...", features = ["sqlx"] }
```

### Identifier Storage

```rust
// MaloId stored as TEXT; validation runs on decode
sqlx::query!("INSERT INTO locations (malo_id) VALUES ($1)", malo_id as _)
    .execute(&pool).await?;

let row = sqlx::query!("SELECT malo_id FROM locations WHERE id = $1", id)
    .fetch_one(&pool).await?;
let malo: MaloId = row.malo_id.parse()?;  // validates on retrieval
```

### Enum Storage

BO4E enums are stored as their variant name string (e.g. `Sparte::Strom` → `"STROM"`).

```rust
sqlx::query!("SELECT sparte FROM contracts WHERE id = $1", id)
    .fetch_one(&pool)
    .await?;
// Decoding an unknown string from DB returns sqlx::Error::Decode
```

**Implemented for:** `MaloId`, `MeloId`, `NeloId`, `SrId`, `TrId`, `EicCode`,
`ObisCode`, `MarktpartnerId`, and key enums (`Sparte`, `BoTyp`, `Kundentyp`, `Tarifart`, …).

---

## utoipa — OpenAPI Schema Derivation

**Feature flag:** `utoipa`  
**Dependency:** `utoipa = "5"`

Derive `ToSchema` on all BO, COM, enum, and identifier types to auto-generate
OpenAPI/Swagger documentation from Rust types.

```toml
rubo4e = { version = "...", features = ["utoipa"] }
```

```rust
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(components(schemas(rubo4e::v202501::Vertrag)))]
struct ApiDoc;

let openapi = ApiDoc::openapi();
```

Property names in the generated OpenAPI schema use German camelCase, consistent
with the serde rename attributes and the BO4E wire format.

---

## strum — Enum Display + FromStr

**Feature flag:** `strum`  
**Dependency:** `strum = "0.28"` (with `derive` feature)

Convert BO4E enum values to and from strings without a `match` expression.
Useful for logging, debug output, CSV export, and non-JSON serialization.

```toml
rubo4e = { version = "...", features = ["strum"] }
```

```rust
// Display — produces the BO4E SCREAMING_SNAKE_CASE wire value
assert_eq!(Sparte::Strom.to_string(), "STROM");

// FromStr — accepts the same wire value
let sparte: Sparte = "STROM".parse()?;
assert_eq!(sparte, Sparte::Strom);

// Static str reference (zero allocation)
let s: &'static str = Sparte::Strom.into();
```

Without the `strum` feature, `Sparte` does not implement `Display` or `FromStr`.
Serde-based JSON serialization works independently of `strum`.

### Unknown Variant

All enums include a catch-all `Unknown` variant (serializes as `"UNKNOWN"`)
that captures unrecognised values for forward-compatibility.

### Error Handling

```rust
match "INVALID_SPARTE".parse::<Sparte>() {
    Err(strum::ParseError::VariantNotFound) => { /* handle */ }
    Ok(sparte) => { /* ... */ }
}
```

---

## proptest — Property Testing

`proptest` is a **dev-dependency** of `rubo4e`. No feature flag is required —
there is no `testing` feature. `Arbitrary` impls for identifier types are compiled
`#[cfg(test)]` only and are not exposed to external crates.

To write property tests against BO4E-integrated code in your own crate, add both as
dev-dependencies:

```toml
# In your crate's Cargo.toml:
[dev-dependencies]
rubo4e = { version = "...", features = ["versioned", "serde"] }
proptest = "1"
```

Your property tests can use the same inline strategy pattern:

```rust
use proptest::prelude::*;

// Build a valid 11-digit BDEW ID string with the correct checksum.
fn valid_malo_id() -> impl Strategy<Value = String> {
    prop::array::uniform10(0u8..=9u8).prop_map(|prefix| {
        // Alternating-weight checksum: weights [2,1,2,1,…], reduce products ≥ 10 by −9
        let sum: u32 = prefix.iter().enumerate()
            .map(|(i, &d)| { let p = u32::from(d) * if i % 2 == 0 { 2 } else { 1 }; if p >= 10 { p - 9 } else { p } })
            .sum();
        let check = ((10 - (sum % 10)) % 10) as u8;
        prefix.iter().chain(std::iter::once(&check))
            .map(|&d| char::from_digit(u32::from(d), 10).unwrap())
            .collect::<String>()
    })
}

proptest! {
    #[test]
    fn my_service_handles_valid_malo(s in valid_malo_id()) {
        let malo = MaloId::new(&s).unwrap();
        let result = my_service.process_location(malo);
        prop_assert!(result.is_ok());
    }
}
```

See `tests/proptest_roundtrips.rs` in the `rubo4e` source for complete reference
strategy implementations for all identifier types, enum variants, and date fields.

---

## time_serde — Date Serde Helpers

**Feature flag:** `time`  
**Module:** `rubo4e::time_serde`

When the `time` feature is enabled, generated structs use `rubo4e::time_serde` for
`time::Date` fields instead of raw strings.  The module is also available to
consumers who need the same `"YYYY-MM-DD"` serde behaviour on their own types:

```toml
rubo4e = { version = "...", features = ["time"] }
```

```rust
use time::Date;

#[derive(serde::Serialize, serde::Deserialize)]
struct MyRecord {
    // Required date — "YYYY-MM-DD" wire format, zero-allocation deserializer
    #[serde(with = "rubo4e::time_serde::date_serde")]
    billing_date: Date,

    // Optional date — null or "YYYY-MM-DD"
    #[serde(with = "rubo4e::time_serde::opt_date_serde")]
    expiry_date: Option<Date>,
}
```

Both submodules use a proper `Visitor` pattern: `visit_str` borrows from the input
without heap allocation.  `opt_date_serde::deserialize` uses `deserialize_option`
so that JSON `null` maps to `None` without constructing an intermediate `String`.

---

## convenience — Ergonomic Helpers on Generated Types

**Feature flag:** `versioned` + `time`  
**Module:** `rubo4e::convenience`

Hand-written extension methods on generated BO4E types — useful accessor shortcuts
that keep application code concise:

```rust
use rubo4e::v202501::{Rechnung, PreisblattNetznutzung, Zeitraum};

// Rechnung — closed billing period (both dates must be present)
let r: Rechnung = todo!();
if let Some((from, to)) = r.billing_period() {
    println!("Invoice period: {from} – {to}");
}

// PreisblattNetznutzung — open-ended or closed validity
let p: PreisblattNetznutzung = todo!();
match p.validity() {
    Some((start, Some(end))) => println!("valid {start} – {end}"),
    Some((start, None))      => println!("valid from {start} (open-ended)"),
    None                     => println!("validity unknown"),
}

// Zeitraum — raw range accessors (also works for all 18+ types with gueltigkeit)
let z: Zeitraum = todo!();
let closed     = z.as_closed_range();     // Option<(Date, Date)>   — both bounds present
let half_open  = z.as_half_open_range();  // Option<(Date, Option<Date>)> — start required
```
