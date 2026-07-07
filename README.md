# rubo4e ⚡

A Rust implementation of the [BO4E](https://www.bo4e.de/) energy-market data standard —
the canonical data model for the German energy industry.

> ⚠️ **This is not an official BO4E implementation.**
> The official reference implementation is [BO4E-python](https://github.com/bo4e/BO4E-python).
> This crate aims for idiomatic Rust ergonomics, strong domain types, and ecosystem integration.

[![Crates.io](https://img.shields.io/crates/v/rubo4e.svg)](https://crates.io/crates/rubo4e)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE-MIT) [![License: Apache 2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE-APACHE)
[![Rust 1.87+](https://img.shields.io/badge/rust-1.87%2B-orange.svg)](https://www.rust-lang.org/)

---

## ✨ Features

- 🏗️ **Generated types** from the official BO4E JSON Schema (v202501)
- 🔒 **Strong domain identifiers** — `MaloId`, `MeloId`, `EicCode`, `ObisCode`, … with embedded validation
- ✅ **Three-layer validation** — constructor checks, `garde` struct rules, cross-field business logic
- 🔧 **Typed builders** — compile-time required-field enforcement via `typed-builder`; optional-field setters accept both `T` and `Option<T>`
- 🌍 **German / English / Canonical JSON** — BO4E wire format out of the box
- 📐 **JSON Schema** via `schemars`, OpenAPI via `utoipa`, DB via `sqlx`
- 🧪 **Golden corpus** and **fuzz harnesses** included; proptest round-trip tests run as dev tests

---

## 📦 Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
rubo4e = "0.3"
```

Enable optional features as needed:

```toml
rubo4e = { version = "0.3", features = ["json", "versioned", "validate", "builder"] }
```

---

## 🚀 Quick Start

```rust
use rubo4e::prelude::*;
use rubo4e::v202501::{Vertrag, Sparte};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Builder with compile-time required-field enforcement (requires `builder` feature)
    let vertrag = Vertrag::builder()
        .sparte(Sparte::Strom)
        .beschreibung("Jahresvertrag Strom".to_string())
        .vertragsnummer("VN-2025-001".to_string())
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

## 🎛️ Feature Gates

| Feature     | Default | Description                                      |
|-------------|:-------:|-------------------------------------------------|
| `serde`     | ✓       | Serde derives + extension-data map               |
| `json`      |         | `serde_json` helpers (`to_json_german()`, …)     |
| `simd-json` |         | SIMD-accelerated JSON parsing backend            |
| `time`      |         | `time` crate for timestamps                      |
| `decimal`   |         | `rust_decimal::Decimal` for amounts and prices   |
| `builder`   |         | `typed-builder` derives                          |
| `validate`  |         | `garde` validation                               |
| `schemars`  |         | JSON Schema generation                           |
| `sqlx`      |         | `sqlx` type integrations (PostgreSQL)            |
| `utoipa`    |         | `utoipa` OpenAPI integration                     |
| `strum`     |         | Enum iteration and string conversion             |
| `versioned` |         | Versioned schema modules (`v202501`, `current`) |
| `tracing`   |         | Structured diagnostics via the `tracing` crate   |
| `metrics`   |         | Counter export hooks (metrics ecosystem)         |

---

## 🗂️ Schema Versions

| Module    | Status              |
|-----------|---------------------|
| `v202501` | ✅ Current stable    |

Use the versioned module to pin a stable schema:

```rust
use rubo4e::v202501::Marktlokation;  // pin to v202501
use rubo4e::current::Marktlokation;  // always the latest stable — advances with crate updates
```

---

## 🏷️ Identifiers

All domain identifiers validate their format on construction:

| Type             | Format / Rule                                         |
|------------------|-------------------------------------------------------|
| `MaloId`         | 11 digits, BDEW alternating-weight checksum           |
| `SrId`           | 11 digits, BDEW alternating-weight checksum (same algorithm as `MaloId`) |
| `TrId`           | 11 digits, BDEW alternating-weight checksum (same algorithm as `MaloId`) |
| `MeloId`         | 33 characters: 2-char ISO country code + 31 alphanumeric |
| `NeloId`         | 11 alphanumeric characters                            |
| `EicCode`        | 16-character EIC with check character (types A, T, V, W, X, Y, Z) |
| `ObisCode`       | OBIS identifier (e.g. `1-0:1.8.1`); C ≥ 1 enforced  |
| `MarktpartnerId` | 13 decimal digits, no checksum                        |

### Identifier Utilities

Beyond construction, identifiers expose domain-specific helpers:

```rust
// Compute check digit / build from base (MaloId, SrId, TrId)
let check = MaloId::check_digit("5123869678")?;        // → 0
let id    = MaloId::from_base("5123869678")?;          // → "51238696780"

// Country code extraction (MeloId)
let melo = MeloId::new("DE0000123456789012345678901234561")?;
assert_eq!(melo.country_code(), "DE");
assert!(melo.is_german());

// Integer round-trip for legacy systems (MarktpartnerId)
let mp = MarktpartnerId::new("9900357000004")?;
assert_eq!(mp.to_i64(), 9_900_357_000_004_i64);

// Serde as integer instead of string (opt-in, field-level)
#[serde(with = "rubo4e::identifiers::marktpartner_id_as_i64")]
pub partner_id: MarktpartnerId,
```

### Convenience Methods on Generated Types

The `convenience` module adds ergonomic helpers on generated BO types
(requires `versioned` + `time` features):

```rust
use rubo4e::v202501::{Rechnung, PreisblattNetznutzung};

// Rechnung — closed billing period
if let Some((from, to)) = rechnung.billing_period() {
    println!("Invoice period: {from} – {to}");
}

// PreisblattNetznutzung — open-ended or closed validity
match preisblatt.validity() {
    Some((start, Some(end))) => println!("valid {start} – {end}"),
    Some((start, None))      => println!("valid from {start} (open-ended)"),
    None                     => println!("validity unknown"),
}

// Zeitraum — low-level range helpers (also available on all types with gueltigkeit)
let z: Zeitraum = todo!();
let closed    = z.as_closed_range();     // Option<(Date, Date)>
let half_open = z.as_half_open_range();  // Option<(Date, Option<Date>)>
```

---

## 📚 Documentation

- [docs/architecture.md](docs/architecture.md) — Workspace layout, module tree, Mermaid diagram
- [docs/generator.md](docs/generator.md) — Internal code generator
- [docs/identifiers.md](docs/identifiers.md) — All identifier types and validation rules
- [docs/versioning.md](docs/versioning.md) — Schema versions and migration
- [docs/validation.md](docs/validation.md) — Three-layer validation
- [docs/serialization.md](docs/serialization.md) — Serialization modes
- [docs/ecosystem.md](docs/ecosystem.md) — schemars, sqlx, utoipa, strum integrations
- [docs/testing.md](docs/testing.md) — Test categories, golden corpus, fuzz harness

---

## 🔗 Related Projects

| Project | Language | Notes |
|---------|----------|-------|
| [BO4E-python](https://github.com/bo4e/BO4E-python) | Python | Official reference implementation |
| [BO4E-Schemas](https://github.com/bo4e/BO4E-Schemas) | JSON Schema | Canonical schema source |
| [go-bo4e](https://github.com/Hochfrequenz/go-bo4e) | Go | Most mature non-Python implementation |
| [bo4e-rust](https://github.com/Hochfrequenz/bo4e-rust) | Rust | Hochfrequenz's Rust implementation |

---

## 📜 License

Dual-licensed under [MIT](LICENSE-MIT) or [Apache 2.0](LICENSE-APACHE) — your choice.

The BO4E standard itself is maintained by the
[Interessengemeinschaft Geschäftsobjekte Energiewirtschaft e. V.](https://www.bo4e.de/).
