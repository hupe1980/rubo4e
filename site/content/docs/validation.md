+++
title = "Validation"
description = "The three independent validation layers — constructor invariants, garde-based cross-field rules, and JSON Schema — and when to reach for each."
weight = 40
+++

`rubo4e` validates domain data at three distinct levels. Each level is independent —
you can use constructor validation without the full `garde`-based struct validation.

## Layer 1 — Constructor Validation (Identifier Types)

Identifier newtypes validate their invariants **at the point of construction**.
A valid `MaloId` value can never exist without a valid checksum.

```rust
let malo = MaloId::new("51238696781")?;  // validates: 11 digits + checksum
let eic  = EicCode::new("10YDE-EON------1")?;  // validates: EIC format + check char
let obis = ObisCode::new("1-0:1.8.1")?;  // validates: OBIS pattern
```

These checks run without any feature flag. They use `thiserror`-derived errors:

```rust
match MaloId::new("bad") {
    Ok(id) => { /* guaranteed valid */ }
    Err(IdentifierError::InvalidLength { expected, actual }) => { /* ... */ }
    Err(IdentifierError::InvalidChecksum) => { /* ... */ }
    // `IdentifierError` is #[non_exhaustive], so a catch-all arm is required.
    Err(other) => { /* ... */ }
}
```

See [Identifiers](@/docs/identifiers.md) for the complete error type and per-type rules.

## Layer 2 — Struct-Level Validation (garde)

**Feature flag:** `validate`  
**Dependency:** `garde` v0.23

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

### How the derives are wired

Identifier newtypes wrap a `Box<str>` and re-run the *same* validator garde saw
at construction, so `garde` and `Identifier::new` can never disagree:

```rust
#[derive(garde::Validate)]
#[garde(allow_unvalidated)]
pub struct MaloId(#[garde(custom(check_malo_id))] Box<str>);

fn check_malo_id(value: &str, _: &()) -> Result<(), garde::Error> {
    validate(value).map_err(garde::Error::from)   // the constructor's own check
}
```

Generated BO/COM structs use `#[garde(allow_unvalidated)]` too — a field with no
explicit attribute is simply accepted — and mark identifier-typed fields
`#[garde(dive)]` so the newtype's validator runs on the way through:

```rust
#[derive(garde::Validate)]
#[garde(allow_unvalidated)]
#[garde(custom(crate::validation::v202607::validate_marktlokation))]
pub struct Marktlokation {
    #[garde(dive)]
    pub marktlokations_id: Option<crate::identifiers::MaloId>,
    // … every other field: unvalidated
}
```

Note that BO4E declares almost every field optional, so `garde` cannot enforce
"required" for you. The cross-field validators below are where the invariants
BO4E states in prose become checkable.

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

BO4E states this rule without enforcing it — BO4E-python carries it as a source
comment (*"only one of the following three optional attributes can be set"*) with
no validator behind it. rubo4e checks it only when you call `.validate()`; a
violating payload still deserializes, as it does everywhere else.

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
startdatum <= enddatum   (when both are Some; equal is a valid one-day period)
```

Non-strict because BO4E declares **both** dates inclusive and gives
`'2025-01-01'` as the example for each: `startdatum == enddatum` is a one-day
period, not an empty one. The accessors in `rubo4e::convenience` read the
interval the same way — `Zeitraum::contains` includes both bounds, and
`whole_days()` counts a one-day period as 1 and January as 31.

Note the contrast with the two rules above it: `vertragsbeginn`/`vertragsende`
are `date-time` and BO4E declares *that* end **exclusive**, which is why
`Vertrag` requires a strict `<` and `Zeitraum` does not. The same release uses
three interval conventions; `tests/interval_conventions.rs` reads each one out of
the committed schema and checks it against the code, so a release that flips one
fails CI rather than an invoice.

Additionally, a `Zeitraum` must have at least one temporal attribute set (`dauer`,
`startdatum`, `enddatum`, `startuhrzeit`, or `enduhrzeit`). A completely empty
`Zeitraum` fails validation.

If only one boundary is `Some`, no ordering constraint is checked.
Date-ordering constraints require the `time` feature — without it the comparison
is not emitted.

### Rechnung Consistency Constraints

All arithmetic uses `rust_decimal::Decimal` (feature `decimal`); without it the
amount fields are `Option<String>` and the checks compile away. Every rule below
traces to a sentence in the BO4E schema.

**Currency agreement.** All `Betrag` fields on one invoice (`gesamtnetto`,
`gesamtsteuer`, `gesamtbrutto`, `rabatt_netto`, `zu_zahlen`) must name the same
`Waehrungscode`; amounts in different currencies cannot be summed.

**Totals.** The schema describes `gesamtbrutto` as *"Die Summe aus Netto- und
Steuerbetrag"*:

```
gesamtnetto + gesamtsteuer == gesamtbrutto      (when all three are Some)
```

If exactly **two** of the three are `Some`, validation fails — the third is
derivable, so its absence is a defect. Fewer than two present means no check.

**Tax lines.** The schema describes `steuerbetraege` as *"eine Liste mit
Steuerbeträgen … die Summe dieser Beträge ergibt den Wert für gesamtsteuer"*:

```
sum(steuerbetraege[*].steuerwert) == gesamtsteuer
```

Only checked when every entry states a `steuerwert`; a list with one omitted is
incomplete rather than inconsistent.

#### Not checked: `zuZahlen`

The schema describes `zuZahlen` as *"(gesamtbrutto - vorausbezahlt -
rabattBrutto)"*, but v202607 ships no `rabattBrutto` — only `rabattNetto`, a
**net** figure, which cannot be subtracted from a gross total. The equation is
not reconstructible from the payload, so nothing is asserted about it. Read
`zu_zahlen_decimal()` and check it against your own books.

### Kostenposition Line Totals

The schema computes `betragKostenposition` as *"<Menge * Einzelpreis>"* **or**
*"<Einzelpreis / (Anzahl Tage Jahr) * zeitmenge>"*. Only the first is checkable
from the COM alone, so a position that states a `zeitmenge` is skipped.

```
einzelpreis.wert * menge.wert  ≈  betrag_kostenposition.wert
```

The comparison allows **half a unit in the last stated decimal place** of the
amount, which accepts either rounding mode: `0.2843 €/kWh × 3333 kWh` is
`947.5719`, written on the invoice as `947.57`.

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

## When `validate` Feature Is Inactive

Without the `validate` feature:

- `.validate()` does not exist on any type
- No `garde` dependency is compiled into the binary
- Constructor-level validation (Layer 1) still works — it has no feature gate

There is **no stub** `validate()` that always returns `Ok`. Consumers who want
validation must enable the feature.
