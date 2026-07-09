# Schema Versioning

`rubo4e` exposes a single stable BO4E schema series (`v202607`), compiled
conditionally behind the `versioned` feature flag.

---

## Version Module Layout

With the `versioned` feature enabled:

```rust
rubo4e::v202607::Vertrag       // v202607 series — pinned, stable across crate updates
rubo4e::v202607::Adresse
rubo4e::v202607::Sparte

rubo4e::current::Vertrag       // moving alias — always the latest stable series
```

Without the `versioned` feature, none of these module paths exist.  The default
feature set (`serde` only) does not include versioned types.

---

## Feature Gate

```toml
# Enable version modules (pure conditional-compilation; no external deps)
rubo4e = { version = "0.4", features = ["versioned"] }
```

---

## Known Schema Series

| Series  | Pinned tag    | Status          | Released     |
|---------|---------------|-----------------|--------------|
| v202607 | v202607.0.0   | Current stable  | July 2026    |

### Versioning Scheme

BO4E uses `vYYYYMM.minor.patch`.  Module names use the `vYYYYMM` prefix only:

```
v202607.0.0  →  module: v202607
v202701.0.0  →  module: v202701   (hypothetical next series)
```

Within a series, minor/patch bumps (e.g. `v202607.0.0` → `v202607.1.0`) are
additive.  The generator pins the full semver tag for reproducibility but exposes
only the series prefix in the public API.

---

## `rubo4e::current` — Moving Alias

`rubo4e::current` is a type alias that always points to the latest stable schema
series.  Use it when you always want the newest types and do not need to pin to a
specific version.

```rust
use rubo4e::current::Vertrag;   // equivalent to rubo4e::v202607::Vertrag today
```

Pin to a concrete module if you need version-stability across crate updates:

```rust
use rubo4e::v202607::Vertrag;   // stable even if rubo4e::current advances
```

---

## Adding a New Schema Version

When a new BO4E schema release arrives with new or changed types:

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
5. Advance the `current` alias:
   ```rust
   pub use v202701 as current;  // was: v202607
   ```
6. Update the convenience module (`src/convenience.rs`) if schema-breaking changes
   require updating field references (e.g. renamed fields in `Rechnung`,
   `Rechnungsposition`).
7. Update the Known Schema Series table in this document.

---

## COM and Enum Versioning

COM and enum types live inside the versioned module alongside BO types.  They
follow exactly the same conditional-compilation rules.

---

## Schema Breaking Changes

The BO4E annual format-version cutover can introduce breaking changes.  Examples
of what changed between series:

| v202501 → v202607 | Change |
|-------------------|--------|
| `Rechnungsposition.lieferung_von` / `lieferung_bis` | Removed; replaced by `lieferungszeitraum: Zeitraum` |
| `Rechnungsposition.teilsumme_netto` | Renamed to `gesamtpreis` |
| `Rechnung.vorausgezahlt` / `rabatt_brutto` / `zu_zahlen` | Removed or restructured |
| `Tarif.registeranzahl` / `sparte` / `tariftyp` | Changed from optional to required |
| 14 types removed, 20 new types added | See schema diff in `generator/schemas/` |

The generator's `STRUCT_FIELD_MAP` in `inference.rs` can override schema-declared
types (e.g. fixing upstream `"format": "date-time"` fields that BDEW uses as
date-only).
