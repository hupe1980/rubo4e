# rubo4e

Rust implementation of the [BO4E](https://www.bo4e.de/) energy-market data standard —
the canonical data model for the German energy industry.

> **Not an official BO4E implementation.** The reference implementation is
> [BO4E-python](https://github.com/bo4e/BO4E-python). This crate aims for idiomatic
> Rust ergonomics, strong domain types, and ecosystem integration.

[![Crates.io](https://img.shields.io/crates/v/rubo4e.svg)](https://crates.io/crates/rubo4e)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)
[![Rust 1.87+](https://img.shields.io/badge/rust-1.87%2B-orange.svg)](https://www.rust-lang.org/)

## MSRV Policy

The **minimum supported Rust version** is `1.87`, encoded in `Cargo.toml` as
`rust-version = "1.87"`.

We target **stable + 2 releases** — MSRV advances when the current floor has been
superseded by two stable releases.  MSRV bumps are treated as **minor** version
changes (not patch).  The `rust-version` field in `Cargo.toml` is the authoritative
source; the README badge is kept in sync.

---

## Features

- **Generated types** from the official BO4E JSON Schema (v202607)
- **Strong domain identifiers** — `MaloId`, `MeloId`, `EicCode`, `ObisCode`, `MarktpartnerId`, … with embedded validation and domain helpers
- **Three-layer validation** — constructor checks, `garde` struct rules, cross-field business logic
- **Typed builders** — compile-time required-field enforcement via `typed-builder`; optional-field setters accept both `T` and `Option<T>`
- **German / snake_case / canonical JSON** — BO4E wire format out of the box
- **Ergonomic convenience API** — extension traits, billing-period helpers, EDIFACT agency codes
- **JSON Schema** via `schemars`, OpenAPI via `utoipa`, PostgreSQL via `sqlx`
- **Golden corpus** and **fuzz harnesses** included; proptest round-trip tests run as dev tests

---

## Installation

```toml
[dependencies]
rubo4e = "0.6"
```

Enable optional features as needed:

```toml
rubo4e = { version = "0.6", features = ["versioned", "time", "decimal", "json", "validate"] }
```

---

## Quick Start

```rust
use rubo4e::prelude::*;          // identifiers, BetragExt, MengeExt, PreisExt, Bo4eJsonExt
use rubo4e::v202607::{Vertrag, Sparte};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Typed builder with compile-time required-field enforcement (requires `builder` feature)
    let vertrag = Vertrag::builder()
        .sparte(Sparte::Strom)
        .beschreibung("Jahresvertrag Strom".to_string())
        .vertragsnummer("VN-2026-001".to_string())
        .build();

    // Cross-field struct validation (requires `validate` feature)
    use garde::Validate as _;
    vertrag.validate()?;

    // German camelCase JSON — BO4E wire format (requires `json` feature)
    let json = vertrag.to_json_german()?;
    println!("{json}");

    Ok(())
}
```

---

## Feature Gates

| Feature       | Default | Description                                       |
|---------------|:-------:|--------------------------------------------------|
| `identifiers` | ✓       | Identifier types (`MaloId`, `EicCode`, `ObisCode`, …) + serde — zero schema overhead |
| `serde`       | ✓       | Serde derives + extension-data map                |
| `json`      |         | `serde_json` helpers (`to_json_german()`, …)      |
| `simd-json` |         | SIMD-accelerated JSON parsing backend             |
| `time`      |         | `time` crate — `Date` for date fields, `OffsetDateTime` for timestamps |
| `decimal`   |         | `rust_decimal::Decimal` for amounts and prices    |
| `builder`   |         | `typed-builder` derives on all BO/COM structs     |
| `validate`  |         | `garde` validation — constructor + cross-field rules |
| `schemars`  |         | JSON Schema generation with patterns and examples |
| `sqlx`      |         | `Type`/`Encode`/`Decode` for all identifier types (PostgreSQL) |
| `utoipa`    |         | `ToSchema` with pattern/example/description for OpenAPI |
| `strum`     |         | Enum iteration and string conversion              |
| `versioned` |         | Versioned schema modules (`v202607`, `current`)   |
| `tracing`   |         | Structured diagnostics via the `tracing` crate    |
| `metrics`   |         | Counter export hooks (metrics ecosystem)          |

> **Typical full setup:**
> ```toml
> rubo4e = { version = "0.6", features = ["versioned", "time", "decimal", "json", "validate", "builder"] }
> ```

---

## Schema Versions

| Module    | Schema tag    | Status            |
|-----------|---------------|-------------------|
| `v202607` | v202607.0.0   | Current stable    |

```rust
use rubo4e::v202607::Marktlokation;  // pin to v202607
use rubo4e::current::Marktlokation;  // always the latest stable — advances with crate updates
```

---

## Identifiers

All domain identifiers validate their format at construction time. There are no panicking constructors.

| Type                | Format / Rule                                              |
|---------------------|------------------------------------------------------------|
| `MaloId`            | 11 digits, BDEW alternating-weight check digit             |
| `NeloId`            | 11 chars: Codetyp `'E'` + 9 `[A-Z0-9]` + ASCII-Verfahren check digit (BDEW §4.2) |
| `SrId`              | 11 chars: Codetyp `'C'` + 9 `[A-Z0-9]` + ASCII-Verfahren check digit (BDEW §6.6) |
| `TrId`              | 11 chars: Codetyp `'D'` + 9 `[A-Z0-9]` + ASCII-Verfahren check digit (BDEW §6.6) |
| `MeloId`            | 33 chars: 2-char ISO country code + 31 alphanumeric        |
| `EicCode`           | 16-char EIC with ENTSO-E check character                   |
| `BilanzkreisId`     | 16-char EIC restricted to type `'Z'` (Bilanzierungszone) — GaBi Gas / MABIS |
| `ObisCode`          | `[A-B:]C.D[.E][*F]`; C=0 permitted (IEC 62056-61 general metering group) |
| `MarktpartnerId`    | 13 decimal digits — BDEW (prefix 99), DVGW (prefix 98), or GS1 GLN |
| `AkivId`            | 1–35 printable ASCII chars — Aktivierungsidentifikator Redispatch 2.0 (BK6-24-174) |
| `TranchennummerId`  | 1–6 decimal digits, no leading zeros — MABIS Bilanzkreisabrechnung (PID 13003) |

```rust
// Build from base (check digit computed automatically)
let malo = MaloId::from_base("5123869678")?;   // → "51238696780"
let c    = MaloId::check_digit("5123869678")?; // → 0u8

// NeloId / SrId / TrId — same from_base pattern
let nelo = NeloId::from_base("E000000001")?;  // → "E0000000019" (ASCII-Verfahren check)
let sr   = SrId::from_base("C000000000")?;   // → "C0000000003"
let tr   = TrId::from_base("D000000000")?;   // → "D0000000002"

// Country code extraction (MeloId)
let melo = MeloId::new("DE00001234567890123456789012345")?;
assert_eq!(melo.country_code(), "DE");
assert!(melo.is_german());

// EDIFACT agency codes (MarktpartnerId) — eliminates duplicate mapping tables
let mp = MarktpartnerId::new("9900357000004")?;
assert!(mp.is_bdew());
assert_eq!(mp.nad_agency_code(), "293");  // EDIFACT NAD DE3055
assert_eq!(mp.unb_agency_code(), "500");  // EDIFACT UNB DE0007

// Integer round-trip for legacy systems
assert_eq!(mp.to_i64(), 9_900_357_000_004_i64);

// Serde as integer (opt-in, field-level)
#[serde(with = "rubo4e::identifiers::marktpartner_id_as_i64")]
pub partner_id: MarktpartnerId,
```

### OBIS codes (EDIFACT support)

```rust
// Standard OBIS codes
let obis = ObisCode::new("1-0:1.8.0")?;       // active energy total
let obis = ObisCode::new("0-0:0.0.0")?;       // C=0 — general metering group (IEC 62056-61)

// F separator normalisation — & is accepted and stored as *
assert_eq!(ObisCode::new("1.8.1&255")?, ObisCode::new("1.8.1*255")?);

// Structured accessors
assert_eq!(ObisCode::new("1-0:1.8.0*255")?.to_pia_string(),  "1-0:1.8.0");    // F stripped
assert_eq!(ObisCode::new("1-0:1.8.0*255")?.to_bo4e_string(), "1-0:1.8.0*255"); // F kept
```

---

## Multi-version Dispatch (F4)

When a storage layer (e.g. PostgreSQL `JSONB`) writes a `bo4e_version` column alongside
BO4E JSON, the idiomatic dispatch pattern is a plain `match`:

```rust
use rubo4e::{v202607, Bo4eObject as _};

fn process_rechnung(
    json: &str,
    bo4e_version: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    match bo4e_version {
        "v202607.0.0" => {
            let r: v202607::Rechnung = serde_json::from_str(json)?;
            // r.schema_version() == "v202607.0.0"
            handle_v202607(r)
        }
        // When v202801 ships, add one arm:
        // "v202801.0.0" => handle_v202801(serde_json::from_str::<v202801::Rechnung>(json)?),
        _ => Err(format!("unsupported schema version: {bo4e_version}").into()),
    }
}
```

This pattern:
- Requires no new rubo4e API — `schema_version()` is already on every BO type via `Bo4eObject`
- Is trivially extensible: each new schema version is one `match` arm
- Localises migration to the storage layer; business logic only handles the current version
- Avoids over-engineering (`trait` objects, `Any*` enums) for a straightforward branch

See [docs/versioning.md](docs/versioning.md) for the full upgrade workflow.

---

## Convenience API

### Extension traits — flatten `Option<Com>` to `Option<Decimal>`

```rust
use rubo4e::prelude::*;  // brings BetragExt, MengeExt, PreisExt into scope

// Before (v0.3 — two levels of unwrap):
let net = pos.gesamtpreis.as_ref().and_then(|b| b.wert);

// After (v0.4):
let net  = pos.gesamtpreis.wert_decimal();          // Option<Decimal> via BetragExt
let qty  = pos.positions_menge.wert_decimal();      // Option<Decimal> via MengeExt
let unit = pos.einzelpreis.wert_decimal();          // Option<Decimal> via PreisExt
```

### Billing and validity helpers

```rust
use rubo4e::v202607::{Rechnung, PreisblattNetznutzung, Zeitraum};
use time::macros::date;

// Rechnung — closed billing period
if let Some((from, to)) = rechnung.billing_period() {
    println!("Invoice period: {from} – {to}");
}

// Navigate rechnungsperiode fields directly
let start: Option<time::Date> = rechnung.period_start();
let end:   Option<time::Date> = rechnung.period_end();

// Iterate line items
for pos in rechnung.positions() {
    println!("  pos {:?}: {:?}", pos.positionsnummer, pos.gesamtpreis_decimal());
}

// Decimal totals — direct access
let net   = rechnung.gesamtnetto_decimal();    // Option<Decimal>
let tax   = rechnung.gesamtsteuer_decimal();   // Option<Decimal>
let gross = rechnung.gesamtbrutto_decimal();   // Option<Decimal>
let pay   = rechnung.zu_zahlen_decimal();      // Option<Decimal> — final amount due
let disc  = rechnung.rabatt_netto_decimal();   // Option<Decimal> — net discount
let next  = rechnung.zukuenftiger_abschlag_decimal(); // Option<Decimal>
let adv   = rechnung.vorauszahlungen_summe();  // Option<Decimal> — sum of advance payments

// Invoice flags — unwrap_or(false), no Option juggling
if rechnung.is_storno() { /* handle cancellation */ }
if rechnung.is_original() { /* handle original */ }

// Date fields
let due: Option<time::Date> = rechnung.faelligkeitsdatum_date();

// Rechnungsposition — delivery period from embedded Zeitraum
let von: Option<time::Date> = pos.lieferung_von_date();  // reads lieferungszeitraum.startdatum
let bis: Option<time::Date> = pos.lieferung_bis_date();  // reads lieferungszeitraum.enddatum
let in_period: bool = pos.lieferungszeitraum_contains(date!(2026-10-01));

// PreisblattNetznutzung — point-in-time validity check
let valid = preisblatt.is_valid_at(date!(2026-10-01));

// Zeitraum — open/closed range helpers
let closed    = z.as_closed_range();     // Option<(Date, Date)>
let half_open = z.as_half_open_range();  // Option<(Date, Option<Date>)>
let contains  = z.contains(date!(2026-01-15)); // bool — [start, end) half-open
```

---

## JSON Handling

```rust
use rubo4e::json::Bo4eJsonExt;
use rubo4e::v202607::Marktlokation;

let malo: Marktlokation = todo!();

// Serialize
let german     = malo.to_json_german()?;     // {"marktlokationsId":"…","sparte":"…",…}
let snake_case = malo.to_json_snake_case()?; // {"marktlokations_id":"…","sparte":"…",…}
let canonical  = malo.to_json_canonical()?;  // sorted keys, stable for hashing/signing

// Deserialize
let restored = Marktlokation::from_json_german(&german)?;
```

Unknown JSON fields are **preserved through round-trips** via the `_additional`
extension-data map (requires `json` feature). This allows forward-compatible
handling of new BO4E fields without library updates.

---

## Validation

```rust
use garde::Validate as _;
use rubo4e::validation::Validated;
use rubo4e::v202607::Marktlokation;

// Direct validation — returns garde::Report on failure
let malo: Marktlokation = todo!();
malo.validate()?;

// Type-safe wrapper — only constructible via validation
let validated = Validated::new(malo)?;     // Err(garde::Report) if invalid
let inner: &Marktlokation = &validated;    // Deref to inner type
```

Cross-field rules (e.g. exactly one of `lokationsadresse` / `geoadresse` /
`katasterinformation` must be set) run automatically via `#[garde(custom(...))]`
attributes on the generated types.

---

## OpenAPI / JSON Schema

```rust
// schemars — JSON Schema (requires `schemars` feature)
let schema = schemars::schema_for!(rubo4e::v202607::Marktlokation);

// utoipa — OpenAPI 3.1 (requires `utoipa` feature)
// All identifier types emit pattern, description, and example values:
// MaloId → { type: string, pattern: "^[0-9]{11}$", example: "51238696780" }
```

---

## SQLx Integration

```rust
// Requires `sqlx` feature — implements Type, Encode, Decode for all identifiers

// Bind directly as typed identifier
sqlx::query("INSERT INTO malo (id) VALUES ($1)")
    .bind(&malo_id)
    .execute(&pool).await?;

// Decode directly — runs the same validation as new()
let id: MaloId = row.try_get("malo_id")?;

// Works in query_as! structs too
#[derive(sqlx::FromRow)]
struct MpRow {
    mp_id: MarktpartnerId,
}
```

---

## Documentation

- [docs/architecture.md](docs/architecture.md) — Workspace layout, module tree, feature gate reference
- [docs/generator.md](docs/generator.md) — Internal code generator — running it, pipeline, inference rules
- [docs/identifiers.md](docs/identifiers.md) — All identifier types, validation rules, algorithms
- [docs/validation.md](docs/validation.md) — Cross-field business rules and `Validated<T>`
- [docs/versioning.md](docs/versioning.md) — Schema versioning scheme and upgrade workflow
- [docs/serialization.md](docs/serialization.md) — JSON format variants, extension-data map, round-trip guarantees
- [docs/testing.md](docs/testing.md) — Golden corpus, fuzz targets, proptest strategies

---

## License

Licensed under either of [Apache License 2.0](LICENSE-APACHE) or [MIT License](LICENSE-MIT),
at your option.
