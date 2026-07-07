# Validation

`rubo4e` validates domain data at three distinct levels. Each level is independent —
you can use constructor validation without the full `garde`-based struct validation.

---

## Layer 1 — Constructor Validation (Identifier Types)

Identifier newtypes validate their invariants **at the point of construction**.
A valid `MaloId` value can never exist without a valid checksum.

```rust
let malo = MaloId::new("51238696780")?;  // validates: 11 digits + checksum
let eic  = EicCode::new("10XDE-EON-NETZ---W")?; // validates: EIC format
let obis = ObisCode::new("1-0:1.8.1")?;  // validates: OBIS pattern
```

These checks run without any feature flag. They use `thiserror`-derived errors:

```rust
match MaloId::new("bad") {
    Err(IdentifierError::InvalidLength { expected, actual }) => { /* ... */ }
    Err(IdentifierError::InvalidChecksum) => { /* ... */ }
    Ok(id) => { /* guaranteed valid */ }
}
```

See [identifiers.md](identifiers.md) for the complete error type and per-type rules.

---

## Layer 2 — Struct-Level Validation (garde)

**Feature flag:** `validate`  
**Dependency:** `garde` v0.23 (MSRV **1.87**)

Struct-level validation checks cross-field constraints that cannot be enforced
by the type system alone.

```rust
// Cargo.toml: features = ["validate"]

// Option A: validate in place
let result = vertrag.validate();
match result {
    Ok(()) => { /* all constraints satisfied */ }
    Err(report) => {
        for (path, error) in report.iter() {
            println!("{path}: {error}");
        }
    }
}

// Option B: Validated<T> wrapper — proves validity in the type system
use rubo4e::validation::Validated;
let validated: Validated<Vertrag> = Validated::new(vertrag)?;
```

### Newtypes with `#[garde(transparent)]`

The `transparent` attribute flattens the error path for newtypes, giving clean messages:

```rust
#[derive(garde::Validate)]
#[garde(transparent)]
pub struct MaloId(
    #[garde(length(equal = 11), custom(check_malo_checksum))]
    String,
);

#[derive(garde::Validate)]
pub struct Vertrag {
    #[garde(required, dive)]
    pub marktlokations_id: Option<MaloId>,
}
```

Error path for an invalid ID: `"marktlokations_id"` (not `"marktlokations_id[0]"`).

---

## Validation Rules Reference

### XOR Address Constraints

**Applies to:** `Marktlokation`, `Messlokation`

Exactly **one** of the following address fields must be `Some`:

| Field               | Type                 |
|---------------------|----------------------|
| `lokationsadresse`  | `Option<Adresse>`    |
| `geoadresse`        | `Option<Geokoordinaten>` |
| `katasterinformation` | `Option<Katasteradresse>` |

```
✓  lokationsadresse = Some, geoadresse = None, katasterinformation = None
✗  all three = None  →  "exactly one address field must be set"
✗  two fields = Some →  "exactly one address field must be set"
```

### Date Range Constraints

**Vertrag** (fields: `Option<time::OffsetDateTime>`):
```
vertragsbeginn < vertragsende   (when both are Some; strict — equal is invalid)
```

**Bilanzierung** (fields: `Option<time::OffsetDateTime>`):
```
bilanzierungsbeginn ≤ bilanzierungsende   (when both are Some; equal is valid)
```

**Zeitraum** (fields: `Option<time::Date>`):
```
startdatum < enddatum   (when both are Some; strict — same day is invalid)
```

Additionally, a `Zeitraum` must have at least one temporal attribute set (`dauer`,
`startdatum`, `enddatum`, `startuhrzeit`, or `enduhrzeit`). A completely empty
`Zeitraum` fails validation.

If only one boundary is `Some`, no ordering constraint is checked.
Date-ordering constraints require the `time` feature — without it the comparison
is not emitted.

### Rechnung Arithmetic Constraints

All arithmetic uses `rust_decimal::Decimal`. Tolerance is exactly zero.

```
gesamt_netto + gesamt_steuer == gesamt_brutto
gesamt_brutto - vorausgezahlt - rabatt_brutto == zuzahlen
```

Validation only runs when all four fields are `Some`. If any is `None`, no arithmetic
constraint is checked.

**Error format:**
```
rechnung.gesamt_brutto: expected 119.00, got 118.99 (netto=100.00, steuer=19.00)
```

### Avis Sum Constraint

```
sum(positionen[*].betrag) == zu_zahlen
```

All position amounts are summed using `Decimal`. The sum must exactly equal `zu_zahlen`.

```
✓  positions = [50.00, 30.00, 20.00], zu_zahlen = 100.00
✗  positions = [],                    zu_zahlen = 50.00   →  "sum 0.00 ≠ 50.00"
```

---

## Layer 3 — Schema Validation

**Feature flag:** `schemars` (for schema generation); validation against schema uses
an external JSON Schema validator (not bundled in this library).

This layer is for interoperability: generate a JSON Schema from the Rust types and
validate incoming JSON against it using any standard JSON Schema validator.

```rust
// Generate the schema (requires `schemars` feature)
let schema = schemars::schema_for!(Vertrag);
let schema_json = serde_json::to_string_pretty(&schema)?;
// Pass schema_json to a JSON Schema validator (e.g. jsonschema crate)
```

---

## Collecting All Errors at Once

`garde::Report` collects all constraint violations from a single call to `.validate()`.

### Structured errors with `report_errors()`

The `report_errors()` helper converts a `garde::Report` into a `Vec<ValidationFailure>`,
each carrying a `.path` (dot-separated field path) and `.message` string — ideal for
structured API responses or key-value logging:

```rust
use rubo4e::validation::{Validated, report_errors};

if let Err(report) = vertrag.validate() {
    for failure in report_errors(&report) {
        eprintln!("  {}: {}", failure.path, failure.message);
    }
}
```

### Raw iteration with `report.iter()`

For direct access to garde's path/error pairs:

```rust
if let Err(report) = vertrag.validate() {
    for (path, error) in report.iter() {
        eprintln!("  field '{}': {}", path, error);
    }
}
```

Both approaches collect all constraint violations simultaneously — useful for
form-validation style responses where every error should be shown at once.

### `Validated<T>` — type-level proof of validity

Wrap a validated value to carry the proof in the type system:

```rust
use rubo4e::validation::Validated;

let valid: Validated<Vertrag> = Validated::new(vertrag)?;
// Only Validated<Vertrag> implements this imaginary trait:
// fn persist(v: &Validated<Vertrag>) { ... }
let inner: Vertrag = valid.into_inner();
```

---

## When `validate` Feature Is Inactive

Without the `validate` feature:

- `.validate()` does not exist on any type
- No `garde` dependency is compiled into the binary
- Constructor-level validation (Layer 1) still works — it has no feature gate

There is **no stub** `validate()` that always returns `Ok`. Consumers who want
validation must enable the feature.
