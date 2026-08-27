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

### Validation is recursive

`.validate()` on a BO checks that BO's own cross-field rules **and descends into
every nested BO, COM, and identifier below it**. One call covers the tree:

```rust
let kosten: Kosten = todo!();

// Reports, at its path, a line total two levels down that does not add up:
//   kostenbloecke[0].kostenpositionen[0]: einzelpreis.wert (2) * menge.wert (3)
//   = 6, which does not round to betrag_kostenposition.wert (999) …
kosten.validate()?;
```

Paths use the Rust field names with bracketed indices, so a report locates the
offending value rather than only naming the root type. `report_errors()` (below)
turns that into one `ValidationFailure` per field.

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
explicit attribute is simply accepted — and the generator marks every field
whose type carries rules of its own `#[garde(dive)]`: the identifier newtypes,
and every nested BO and COM. That is what makes the recursion above happen.

```rust
#[derive(garde::Validate)]
#[garde(allow_unvalidated)]
#[garde(custom(crate::validation::v202607::validate_marktlokation))]
pub struct Marktlokation {
    #[garde(dive)]
    pub marktlokations_id: Option<crate::identifiers::MaloId>,   // identifier
    #[garde(dive)]
    pub lokationsadresse: Option<Adresse>,                       // nested COM
    #[garde(dive)]
    pub lokationszuordnungen: Option<Vec<Box<Lokationszuordnung>>>,
    pub sparte: Option<Sparte>,                                  // enum: no rules
    // … scalars: unvalidated
}
```

`garde` supplies the `Validate` impls for `Option`, `Vec`, and `Box`, so one
`dive` covers `Option<Vec<Box<T>>>`. Enums and scalars carry no rules and are
left alone. A self-referential type recurses over the *data*, which terminates
because the indirection is a `Box` and the payload is a finite tree.

Note that BO4E declares almost every field optional, so `garde` cannot enforce
"required" for you. The cross-field validators below are where the invariants
BO4E states in prose become checkable — and they only fire on values that are
actually present.

## Validation Rules Reference

### Ortsangabe exclusivity

**Applies to:** `Marktlokation`, `Messlokation`

**At most one** of the following may be `Some`:

| Field                                    | Type                      |
|------------------------------------------|---------------------------|
| `lokationsadresse` / `messadresse`       | `Option<Adresse>`         |
| `geoadresse`                             | `Option<Geokoordinaten>`  |
| `katasterinformation`                    | `Option<Katasteradresse>` |

```
✓  lokationsadresse = Some, geoadresse = None, katasterinformation = None
✓  all three = None
✗  two fields = Some →  "at most one Ortsangabe may be set, but
                         lokationsadresse, geoadresse are — …"
```

BO4E states **mutual exclusivity, not presence**: *"Es darf immer nur eine Art
der Ortsangabe vorhanden sein."* The schema backs that exactly — no `required`
array, no `oneOf`, all three properties `"default": null` — and BO4E-python
carries the rule as a comment over three `Optional[…] = None` fields.

The empty case has to be legal because BO4E has no reference type: a location
referenced from a `Rechnung`, a `Vertrag`, or an `Angebot` is a full
`Marktlokation` carrying little more than its ID. For `Messlokation` the schema
says so outright — `messadresse` is documented *"Nur angeben, wenn diese von der
Adresse der Marktlokation abweicht"*.

Checked only when you call `.validate()`; a violating payload still deserializes,
as everywhere else.

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

**Zeitraum, third mode** (all four boundary fields present):
```
start_instant < end_instant   (strict — equal encloses no time)
```

The same struct, the opposite rule, and both come straight out of the schema:
`startuhrzeit` is *"im betrachteten Zeitraum **inklusiv**"* and `enduhrzeit`
*"**exklusiv**"*, so an instant range is `[start, end)` and a start at or after
the end covers nothing. The date check cannot see it, because both instants can
fall on the same date — which is exactly the shape every quarter-hourly
`Zeitreihenwert` has. See [Time Series & Units](@/docs/timeseries.md).

An unparsable `startuhrzeit` is **not** reported here: rejecting it would make
`.validate()` answer a question about string syntax that the schema states no rule
for. Read it through `startuhrzeit_parsed()` and decide there.

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

All three present, or fewer than two, is all `.validate()` has to say about
them: BO4E marks none of the three `required`. The "if two are stated the third
should be too" rule is a *quality* judgement and lives in
[`quality`](#opt-in-quality-rules) instead.

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

An amount at `Decimal`'s maximum scale of 28 leaves no room for a tolerance, so
the comparison there is exact.

## What `.validate()` does not check: field names

`.validate()` runs rules over the value it is given. It cannot say anything about
keys that never reached a field — a misspelled `kostenblockBEZEICHNUNG` is not a
failed rule, it is a key that landed in extension data while the field it was
meant to fill stayed `None`. `Validated<Kosten>` proves the rules hold; it does
not prove the document said what its author meant.

There are three separate questions, and they need three separate answers:

| Question | Call |
|---|---|
| Do the **cross-field rules** hold? | `.validate()` / `Validated<T>` |
| Does it use a **value** BO4E does not define? | `Bo4eStrict::ensure_known_enums()` |
| Does it use a **field** BO4E does not define? | `Bo4eExtensions::ensure_no_extension_data()` |

The third is documented in full under
[Serialization](@/docs/serialization.md#a-decode-does-not-validate-field-names),
along with why a decode round-trip cannot answer it and why constructing values
typed is better than any of the three.

## Conformance rules vs. quality rules

`.validate()` runs **only rules traceable to a sentence of the BO4E schema**, so
it answers *"does this conform to BO4E"* — a claim you can make about a document
a **counterparty** sent. A rule this crate merely thought sensible would turn
that into *"…and satisfies `rubo4e`"*, and `Validated::new` is all-or-nothing, so
there would be no way to opt out.

The crate's own judgements therefore live in a separate module.

### Opt-in quality rules

**Module:** `rubo4e::validation::current::quality`

Nothing here is wired into `#[derive(garde::Validate)]`, so `.validate()` and
`Validated<T>` never run it.

| Function | Rule | Why it is not conformance |
|---|---|---|
| `rechnung_totals_are_complete` | all three of `gesamtnetto` / `gesamtsteuer` / `gesamtbrutto`, or none | BO4E marks none of them `required` and says nothing about stating them together |

The time-series audit is the same kind of judgement, and lives outside
`validation` for the same reason. `Bo4eTimeSeries::audit()` reports gaps,
overlaps, wrong-length intervals and unusable readings on a `Lastgang` or a
`Zeitreihe` — none of which the schema requires, so a gappy load profile is a
conforming one. See [Time Series & Units](@/docs/timeseries.md#audit-is-not-validate).

```rust
use rubo4e::validation::current::quality;
use garde::Validate as _;

rechnung.validate()?;   // conformance

// Typically on documents you produce, or as a warning on ones you receive.
if let Err(e) = quality::rechnung_totals_are_complete(&rechnung) {
    tracing::warn!(%e, "invoice states two of three totals");
}
```

Each returns a `garde::Error`, so it composes into a `garde` pipeline of your own.

## Version-agnostic imports

`rubo4e::validation::current` mirrors `rubo4e::current`, so no downstream file
has to name a schema version — and a CI guard that greps for `rubo4e::v202607`
stays clean:

```rust
use rubo4e::validation::current::{validate_zeitraum, quality};
```

It resolves to the same functions as `rubo4e::validation::v202607`. Reach for the
versioned path only to stay on one series across a format-version cutover — the
same trade-off as [`current` vs `v202607`](@/docs/versioning.md#rubo4e-current-moving-alias)
for the types.

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

#### It validates on the way in

With `serde`, `Validated<T>` is `Deserialize` — and the impl **runs the rules**.
Decoding one and getting a value back *is* the proof, so a handler can take one
as its request body and there is no `.validate()` call left to forget:

```rust
// In your own HTTP layer — rubo4e ships no extractors of its own.
async fn create(
    axum::Json(body): axum::Json<Validated<Marktlokation>>,
) -> Result<StatusCode, AppError> {
    // `body` cannot exist unless every rule held, nested ones included.
    persist(body.into_inner()).await?;
    Ok(StatusCode::CREATED)
}
```

An invalid payload fails at deserialization, with the whole `garde::Report`
rendered into the deserializer's error message. Where you want the *structured*
report back — one entry per failing field, for a 422 body — decode the plain `T`
and call `Validated::new` on it, or `.validate()` plus `report_errors()`.

`Serialize` is transparent, so a `Validated<T>` re-encodes to exactly the bytes
`T` would.

## When `validate` Feature Is Inactive

Without the `validate` feature:

- `.validate()` does not exist on any type
- No `garde` dependency is compiled into the binary
- Constructor-level validation (Layer 1) still works — it has no feature gate

There is **no stub** `validate()` that always returns `Ok`. Consumers who want
validation must enable the feature.
