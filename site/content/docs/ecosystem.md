+++
title = "Ecosystem Integrations"
description = "Optional serde, sqlx, schemars, utoipa, strum, and proptest integrations — each behind a feature gate, each costing nothing when disabled."
weight = 60
+++

`rubo4e` provides optional integrations with common Rust ecosystem crates.
Every integration is behind a feature gate and adds zero overhead when disabled.

> **Scope:** This library provides **types**. It does not contain HTTP handler code,
> Axum extractors, Actix-web guards, or any framework-specific glue code.
> Consumers compose `rubo4e` types with their own HTTP and persistence layers.

## schemars — JSON Schema Generation

**Feature flag:** `schemars`  
**Dependency:** `schemars = "1"`

Derive `JsonSchema` on all types to generate JSON Schema documents from Rust code.
Useful for API documentation, input validation pipelines, and tooling integration.

```sh
cargo add rubo4e --features schemars
```

```rust
use schemars::{schema_for, JsonSchema};

let schema = schema_for!(rubo4e::v202607::Vertrag);
let json = serde_json::to_string_pretty(&schema)?;
println!("{json}");
```

Identifier types appear as `{ "type": "string" }` in the schema — not as JSON objects.
This matches the wire format and keeps schemas interoperable with non-Rust consumers.

### The `Decimal` and `time` schema impls hang off `decimal` / `time`

`schemars`'s `rust_decimal1` feature — the one that provides
`impl JsonSchema for rust_decimal::Decimal` — is enabled by rubo4e's **`decimal`**
feature, as `schemars?/rust_decimal1`, not by `schemars`. `utoipa`'s `decimal`
and `time` features work the same way. They are needed exactly when the generated
fields *are* those types, and the `?` means neither feature drags the integration
in.

That placement matters beyond tidiness. Cargo unifies features across a
workspace, so a crate enabling `rust_decimal1` unconditionally becomes the sole
provider of that impl for everything beside it: a sibling crate deriving
`JsonSchema` over its own `Decimal` field compiles only for as long as something
in the graph keeps `rubo4e/schemars` on.

If you need the impl, declare it yourself:

```toml
schemars = { version = "1", features = ["rust_decimal1"] }
```

## sqlx — Database Type Impls

**Feature flag:** `sqlx`  
**Dependency:** `sqlx = "0.8"`  
**Primary target:** PostgreSQL (TEXT column type)

Store and query BO4E identifiers and enums directly in SQL queries without manual
string conversion.

```sh
cargo add rubo4e --features sqlx
```

### Identifier Storage

Identifiers bind and decode directly — no manual `.parse()` step, and no
`as _` override. Validation runs inside `Decode`, so a malformed value already in
the database surfaces as an error rather than a silently-accepted bad ID.

```rust
use rubo4e::identifiers::MaloId;
use sqlx::Row as _;

sqlx::query("INSERT INTO locations (malo_id) VALUES ($1)")
    .bind(&malo_id)                       // Encode: binds as TEXT, zero-copy
    .execute(&pool).await?;

let row = sqlx::query("SELECT malo_id FROM locations WHERE id = $1")
    .bind(id)
    .fetch_one(&pool).await?;
let malo: MaloId = row.try_get("malo_id")?;   // Decode: validates the check digit
```

`Vec<Id>` binds to a `TEXT[]` column, so `= ANY($1)` lookups work directly:

```rust
sqlx::query("SELECT * FROM locations WHERE malo_id = ANY($1)")
    .bind(&malo_ids)                      // Vec<MaloId> -> TEXT[]
    .fetch_all(&pool).await?;
```

### Enum Storage

BO4E enums are stored as their canonical wire string (e.g. `Sparte::Strom` → `"STROM"`).

```rust
let row = sqlx::query("SELECT sparte FROM contracts WHERE id = $1")
    .bind(id)
    .fetch_one(&pool).await?;
let sparte: Sparte = row.try_get("sparte")?;
```

Note the asymmetry with the JSON path: an unrecognized string decodes to the
`Unknown` catch-all rather than failing, matching the lenient serde behaviour.
Use `Sparte::from_wire(...)` on a `String` column, or check `is_known()`, when a
value from outside the schema must be rejected.

**Implemented for:** every identifier type — `AkivId`, `Bic`,
`BilanzierungsgebietId`, `BilanzkreisId`, `CrId`, `EicCode`, `Iban`, `MaloId`,
`MarktpartnerId`, `MeloId`, `NebeId`, `NeloId`, `ObisCode`, `PaketId`, `SgId`,
`SrId`, `TrId`, `TranchennummerId` — and **every** generated enum.
`PgHasArrayType` is on both, so `Vec<Sparte>` binds to a `TEXT[]` column just as
`Vec<MaloId>` does. Neither needs the `json` feature: both directions go through
`&str`.

The list is hand-maintained in `impl_sqlx_text!`, so `tests/prelude_surface.rs`
compares it against what `src/identifiers/` exports — an identifier missing from
it compiles fine and simply cannot be a column, which is not a failure anything
else would notice.

## utoipa — OpenAPI Schema Derivation

**Feature flag:** `utoipa`  
**Dependency:** `utoipa = "5"`

Derive `ToSchema` on all BO, COM, enum, and identifier types to auto-generate
OpenAPI/Swagger documentation from Rust types.

```sh
cargo add rubo4e --features utoipa
```

```rust
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(components(schemas(rubo4e::v202607::Vertrag)))]
struct ApiDoc;

let openapi = ApiDoc::openapi();
```

Property names in the generated OpenAPI schema use German camelCase, consistent
with the serde rename attributes and the BO4E wire format.

## strum — Enum `FromStr` + iteration

**Feature flag:** `strum`  
**Dependency:** `strum = "0.28"` (with `derive` feature)

> **Note:** `Display`, `AsRef<str>`, `as_wire()`, `from_wire()`, `VARIANTS`,
> `COUNT`, and `iter_known()` are **always available** — no `strum` needed. See
> [strict enum parsing](@/docs/serialization.md).
> The `strum` feature adds only `FromStr`, `EnumIter` (iteration including
> `Unknown`), and `Into<&'static str>`.

```sh
cargo add rubo4e --features strum
```

```rust
// FromStr — accepts the BO4E SCREAMING_SNAKE_CASE wire value (strum feature)
let sparte: Sparte = "STROM".parse()?;
assert_eq!(sparte, Sparte::Strom);

// Static str reference (zero allocation) — strum's IntoStaticStr
let s: &'static str = Sparte::Strom.into();

// Iterate every variant, including the Unknown catch-all — strum's EnumIter
use strum::IntoEnumIterator as _;
let n = Sparte::iter().count();
```

Without `strum`, use the always-on equivalents instead: `to_string()` /
`as_wire()` for the wire string, `from_wire()` for **strict** parsing, and
`iter_known()` for iteration over the schema-defined variants.

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

## proptest — Property Testing

`proptest` is a **dev-dependency**, so no feature flag is involved. The
`Arbitrary` impls for identifier types are `#[cfg(test)]` only and are not
exposed to external crates.

To write property tests against BO4E-integrated code in your own crate, add both as
dev-dependencies:

```sh
cargo add --dev rubo4e --features versioned,serde
cargo add --dev proptest
```

Your property tests can use the same strategy pattern this crate's own use —
generate the **base**, and let `from_base` append the check digit:

```rust
use proptest::prelude::*;
use rubo4e::identifiers::MaloId;

/// A valid 11-digit MaLo-ID: 10 base digits (the first is the Vergabestelle,
/// never `0`) plus the BDEW §8.1 check digit `from_base` computes.
fn valid_malo_id() -> impl Strategy<Value = String> {
    prop::string::string_regex("[1-9][0-9]{9}")
        .expect("MaLo base regex")
        .prop_map(|base| MaloId::from_base(&base).expect("valid base").to_string())
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

Deriving the check digit rather than reimplementing it is the point. BDEW §8.1
weights **odd** positions by 1 and **even** positions by 2 and sums the products
whole — it is not Luhn, which halves the alphabet by reducing any product ≥ 10 by
9, and a strategy built on Luhn generates strings `MaloId::new` rejects. The same
shape works for every identifier that has a `from_base` or a `from_prefix`.

See `tests/proptest_roundtrips.rs` in the `rubo4e` source for complete reference
strategy implementations for all identifier types, enum variants, and date fields.

## time_serde — Date Serde Helpers

**Feature flag:** `time`  
**Module:** `rubo4e::time_serde`

When the `time` feature is enabled, generated structs use `rubo4e::time_serde` for
`time::Date` fields instead of raw strings.  The module is also available to
consumers who need the same `"YYYY-MM-DD"` serde behaviour on their own types:

```sh
cargo add rubo4e --features time
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

## convenience — Ergonomic Helpers on Generated Types

**Feature flag:** `versioned` + `time`  
**Module:** `rubo4e::convenience`

Hand-written extension methods on generated BO4E types — useful accessor shortcuts
that keep application code concise:

```rust
use rubo4e::v202607::{Rechnung, PreisblattNetznutzung, Zeitraum};

// Rechnung — closed billing period, as a RangeInclusive<Date>
// (`None` unless both `rechnungsperiode` dates are present)
let r: Rechnung = todo!();
if let Some(period) = r.billing_period() {
    println!("Invoice period: {} – {} inclusive", period.start(), period.end());
}

// PreisblattNetznutzung — validity bounds, either of which may be open.
// `validity()` returns a pair, not an Option: a missing `gueltigkeit` reads as
// (None, None), the same shape as "stated, but unbounded on both sides".
// Use `is_valid_at(date)` where "no validity stated" must read as *not* valid.
let p: PreisblattNetznutzung = todo!();
match p.validity() {
    (Some(start), Some(end)) => println!("valid {start} – {end} inclusive"),
    (Some(start), None)      => println!("valid from {start} (open-ended)"),
    (None, Some(end))        => println!("valid until {end} inclusive"),
    (None, None)             => println!("no validity stated"),
}

// Zeitraum — range accessors (also works for all 18+ types with gueltigkeit)
let z: Zeitraum = todo!();
let range   = z.as_inclusive_range();  // Option<RangeInclusive<Date>>  — both bounds stated
let bounds  = z.bounds();              // (Option<Date>, Option<Date>)  — either may be open
let days    = z.whole_days();          // Option<i64>                   — January is 31
let active  = z.contains(some_date);   // bool                          — end date included
let dauer   = z.duration();            // Option<Result<time::Duration, _>>
```

**Both dates are inclusive**, as BO4E states on the fields themselves:
*"Enddatum des betrachteten Zeitraums ist **inklusiv**"*. The schema gives
`'2025-01-01'` as the example for `startdatum` *and* `enddatum`, so
`start == end` is a valid one-day period.

`as_inclusive_range` returns a `RangeInclusive<Date>` rather than a tuple: the
type carries the convention, so `range.contains(&d)` is right by construction and
there is no `start..end` / `start..=end` decision to get wrong.

### The three fields that stay `String`, and how to read them

`Zeitraum` carries three values no `time` type can hold directly, so the
generated fields keep the wire string and an accessor parses on demand:

```rust
// `dauer` — an ISO 8601 duration, e.g. "P1DT30H4S"
let d: Option<Result<time::Duration, _>> = z.duration();

// `startuhrzeit` / `enduhrzeit` — a time of day *with a UTC offset*
let start: Option<Result<(time::Time, Option<time::UtcOffset>), _>> = z.startuhrzeit_parsed();
let end = z.enduhrzeit_parsed();

// …and the same format on Umschaltzeit, the HT/NT switching time
let switch = umschaltzeit.umschaltzeit_parsed();
```

**`duration()`** refuses `P1Y` and `P1M` rather than approximating them: a year
is 365 or 366 days and a month 28 to 31, so converting either without a start
date is a guess. Weeks and below are exact. See
[`iso8601_duration`](https://docs.rs/rubo4e/latest/rubo4e/iso8601_duration/).

**The time-of-day accessors** return the UTC offset separately because `time`
has no offset-bearing time-of-day type — `Time` carries no zone and
`OffsetDateTime` demands a date. The offset is load-bearing: a Zählzeit window
or a Doppeltarif switch written `06:00:00+01:00` is a different wall-clock
moment in summer than in winter. It is `Option`, not defaulted to UTC, because
BO4E does not require one and "local time, zone not stated" is a different claim.

Note that these two use the **opposite** convention to the date pair on the same
type: `startuhrzeit` is inclusive and `enduhrzeit` **exclusive**.
