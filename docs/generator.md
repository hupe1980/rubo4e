# Code Generator

The generator (`generator/`) is a standalone Rust binary that reads pinned BO4E JSON Schema
files and emits Rust source code into `src/generated/`. It is a workspace member but is
never published to crates.io.

---

## Running the Generator

```bash
# Generate from the latest stable schema (v202501.0.0)
cargo run -p bo4e-generator -- --schema-version v202501.0.0

# Generate from the previous schema series
cargo run -p bo4e-generator -- --schema-version v202501.0.0
```

The generator reads schema files from `generator/schemas/<TAG>/` relative to the workspace
root and writes output to `src/generated/<series>/` where `<series>` is the `vYYYYMM`
prefix (e.g. `v202501`).

---

## Updating to a New Schema Version

When BO4E releases a new schema:

1. **Download the schema tag** into `generator/schemas/<NEW_TAG>/`:
   ```bash
   # Example: v202501.1.0
   git clone --depth 1 --branch v202501.1.0 \
     https://github.com/bo4e/BO4E-Schemas \
     generator/schemas/v202501.1.0
   ```
2. **Run the generator:**
   ```bash
   cargo run -p bo4e-generator -- --schema-version v202501.1.0
   ```
3. **Inspect the diff** in `src/generated/v202501/`:
   - Added fields → inspect; update curated wrappers if needed
   - Removed fields → check migration impls are still correct
   - Type changes → update semantic inference map if applicable
4. **Update the `_version` constant** in `src/generated/v202501/mod.rs`:
   ```rust
   pub const SCHEMA_VERSION: &str = "v202501.1.0";
   ```
5. **Commit both** the new schema directory and the regenerated source.

> **v202402 does not exist.** See [versioning.md](versioning.md) for the full warning.

---

## Schema Directory Layout

The generator expects schemas in the layout used by `bo4e/BO4E-Schemas`:

```
generator/schemas/v202501.0.0/
├── BO-Objekte/
│   ├── Vertrag.json
│   ├── Marktlokation.json
│   └── ...
├── COM/
│   ├── Adresse.json
│   └── ...
└── enum/
    ├── Sparte.json
    └── ...
```

---

## Internal Architecture

### Pipeline

```
JSON Schema files
      │
      ▼
  parser.rs          — serde_json → SchemaNode AST
      │
      ▼
  inference.rs       — field name → strong Rust type mapping
      │
      ▼
  emitter.rs         — SchemaNode AST → formatted Rust source
      │
      ▼
src/generated/
```

### SchemaNode AST

The parser produces a typed AST:

```rust
enum SchemaNode {
    BoType(BoTypeDef),
    ComType(ComTypeDef),
    EnumType(EnumTypeDef),
}

struct BoTypeDef {
    name: String,
    description: Option<String>,
    fields: Vec<FieldDef>,
}

struct FieldDef {
    name: String,                // camelCase BO4E name
    rust_name: String,           // snake_case Rust name
    typ: RustType,               // after semantic inference
    description: Option<String>,
    is_optional: bool,           // always true for BO4E; wrapped in Option<T>
}
```

### `$ref` Resolution

All `$ref` references are resolved **within the same schema snapshot**. The generator
never makes network requests. If a `$ref` cannot be resolved (e.g. missing file), the
generator emits a warning and replaces the field type with `serde_json::Value`.

---

## Semantic Type Inference

The generator maps BO4E field names to strong Rust types using a hard-coded table.
When a field name matches an entry, the generator substitutes the semantic type.
All other fields default to the JSON Schema primitive type.

| BO4E field name pattern  | Rust type        | Notes                             |
|--------------------------|------------------|-----------------------------------|
| `marktlokationsId`       | `MaloId`         | 11-digit + checksum               |
| `messlokationsId`        | `MeloId`         | 33-char alphanumeric              |
| `netzlokationsId`        | `NeloId`         | 11-char alphanumeric              |
| `steuerbareRessourceId`  | `SrId`           | 11-digit + checksum               |
| `technischeRessourceId`  | `TrId`           | 11-digit + checksum               |
| `bilanzkreis`            | `EicCode`        | 16-char EIC format                |
| `obisKennzahl`           | `ObisCode`       | OBIS pattern `n-n:n.n.n`         |
| `rollencodenummer`       | `MarktpartnerId` | 13-digit numeric                  |
| ends in `Betrag`/`Preis`/`Menge` | `Decimal` | `rust_decimal`, not `f64`   |
| ends in `Datum`/`Zeitpunkt`/`Beginn`/`Ende` | `OffsetDateTime` | `time` crate |
| `prozentsatz`            | `Decimal`        | No custom newtype; value = `n%`   |
| `waehrungscode`          | `Waehrungscode`  | BO4E enum; no custom newtype      |

### Adding a New Mapping

Edit `generator/src/inference.rs`:

```rust
fn infer_type(field_name: &str) -> RustType {
    match field_name {
        "marktlokationsId" => RustType::Named("MaloId"),
        // Add here:
        "myNewId" => RustType::Named("MyNewId"),
        _ => infer_by_suffix(field_name),
    }
}
```

Then implement the corresponding newtype in `src/identifiers/` and add it
to the `identifiers/mod.rs` re-exports. See [identifiers.md](identifiers.md).

---

## Determinism Guarantee

Running the generator twice on the same input produces **byte-identical output**.
This is enforced by:

- Iterating schema fields in JSON key order (deterministic with `serde_json`)
- Sorting generated enum variants alphabetically
- Running `prettyplease` (or `rustfmt`) on each output file with a fixed configuration

CI verifies this with:

```bash
cargo run -p bo4e-generator -- --schema-version v202501.0.0
git diff --exit-code src/generated/
```

If this step fails, the committed generated code is stale.

---

## What the Generator Does NOT Do

- It does not generate identifier newtype implementations (those are hand-crafted)
- It does not generate validation logic (that lives in `src/validation/`)
- It does not generate the migration module (hand-written in `src/migration/`)
- It does not make network requests
- It does not modify any file outside `src/generated/`
