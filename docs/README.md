# rubo4e Documentation

`rubo4e` is the definitive Rust SDK for the [BO4E](https://www.bo4e.de/) energy data
standard — the canonical data model for the German energy industry.

---

## Quick Start

```rust
use rubo4e::prelude::*;
use rubo4e::v202501::{Sparte, Vertrag};

// Builder with compile-time required-field enforcement
let vertrag = Vertrag::builder()
    .sparte(Sparte::Strom)
    .beschreibung("Jahresvertrag Strom".to_string())
    .vertragsnummer("VN-2025-001".to_string())
    .build();

// Layered struct validation (requires `validate` feature)
vertrag.validate()?;

// German camelCase JSON — BO4E wire format (requires `json` feature)
let json = vertrag.to_json_german()?;
```

---

## Documentation Pages

| Page | Description |
|------|-------------|
| [architecture.md](architecture.md) | Workspace layout, module tree, feature gates, Mermaid diagram |
| [generator.md](generator.md) | Internal code generator: how to run, schema pinning, type inference |
| [identifiers.md](identifiers.md) | All identifier types, validation rules, checksum algorithms |
| [versioning.md](versioning.md) | Schema version modules, `rubo4e::current` alias |
| [validation.md](validation.md) | Three-layer validation: constructor, struct (garde), schema |
| [serialization.md](serialization.md) | German / snake_case / Canonical output modes, round-trip safety |
| [ecosystem.md](ecosystem.md) | schemars, sqlx, utoipa, strum, proptest integrations |
| [testing.md](testing.md) | Test categories, commands, golden corpus, fuzz harness |

---

## Standard References

- [BO4E-Schemas](https://github.com/bo4e/BO4E-Schemas) — canonical JSON Schema source
- [BO4E-python](https://github.com/bo4e/BO4E-python) — reference implementation
- [go-bo4e](https://github.com/Hochfrequenz/go-bo4e) — most mature non-Python implementation
- [bo4e.de](https://www.bo4e.de/) — official standard website
