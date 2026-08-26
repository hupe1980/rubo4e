+++
title = "Code Generator"
description = "How the internal generator turns pinned BO4E JSON Schema releases into Rust source, including type inference, identifier naming, and the drift checks that keep output honest."
weight = 70
+++

The generator (`generator/`) is a standalone Rust binary that reads pinned BO4E JSON Schema
files and emits Rust source code into `src/generated/`. It is a workspace member but is
never published to crates.io.

Its output is **committed to the repository**, so nothing at build time forces it
to match the schemas — the [drift guards](#drift-guards) do that instead.

## Running the Generator

```bash
just generate                    # generator + cargo fmt, the supported entry point
```

or, directly:

```bash
cargo run -p bo4e-generator -- --schema-version v202607.1.0
cargo fmt --all                  # the generator emits prettyplease output
```

The generator reads schema files from `generator/schemas/<TAG>/` relative to the workspace
root and writes output to `src/generated/<series>/` where `<series>` is the `vYYYYMM`
prefix (e.g. `v202607`).

## Updating to a New Schema Version

When BO4E releases a new schema:

1. **Download the schema tag** into `generator/schemas/<NEW_TAG>/`:
   ```bash
   # Uses scripts/download_schemas.sh which pulls the release archive from GitHub
   just download-schemas v202701.0.0
   ```
2. **Retire the previous snapshot of the same series.** Exactly one snapshot per
   series is committed; the test suite reads the pinned tag off that directory
   rather than from a constant, and fails if it finds two.
   ```bash
   git rm -r generator/schemas/v202607.1.0    # only when staying in the series
   ```
3. **Run the generator** for the new tag:
   ```bash
   cargo run -p bo4e-generator -- --schema-version v202701.0.0 && cargo fmt --all
   ```
4. **Inspect the diff** in `src/generated/`:
   - Added fields → inspect; update the convenience wrappers if needed
   - Removed fields and removed types → check nothing in `src/convenience.rs` or
     `src/validation/` still names them. The generator deletes the module of a
     type the release retired, so a stale reference is a compile error, not a
     silently orphaned file.
   - New identifier-bearing fields → add them to `generator/src/inference.rs`
   - Enum membership → record **additions and removals** in the CHANGELOG's
     **Schema deltas** section; the removals are the half that breaks a
     downstream build
5. **Point `current` at the new series** in `src/lib.rs` if it supersedes the old one.
6. **Run `just ci`.** The drift guards below fail loudly if anything is stale.
7. **Commit both** the new schema directory and the regenerated source.

There is no version constant to hand-edit. `_typ` and `_version` are read out of
the schema files themselves (see [Metadata comes from the
schema](#metadata-comes-from-the-schema)), so a new release carries its own
values through with no manual step.

## Schema Directory Layout

The generator expects schemas in the layout used by `bo4e/BO4E-Schemas`:

```
generator/schemas/v202607.1.0/
├── bo/
│   ├── Vertrag.json
│   ├── Marktlokation.json
│   └── ...
├── com/
│   ├── Adresse.json
│   └── ...
├── enum/
│   ├── Sparte.json
│   └── ...
└── ZusatzAttribut.json   ← root-level COM (no category subdirectory)
```

## Internal Architecture

### Pipeline

```
JSON Schema files
      │
      ▼
  parser.rs          — serde_json → SchemaNode AST
      │
      ▼
  inference.rs       — (struct, field) → domain identifier type
      │
      ▼
  naming.rs          — BO4E wire values → Rust identifiers
      │
      ▼
  emitter.rs         — SchemaNode AST → formatted Rust source
      │
      ▼
src/generated/
```

### SchemaNode AST

A BO and a COM are structurally identical — they differ only in which
discriminant enum their `_typ` draws from and whether they implement
`Bo4eObject` — so the parser produces one node type for both:

```rust
enum SchemaNode {
    Struct(StructNode),
    Enum(EnumNode),
}

struct StructNode {
    name: String,
    kind: StructKind,               // Bo | Com
    fields: Vec<Field>,
    description: Option<String>,
    typ_const: Option<String>,      // the `_typ` wire value the schema pins
    version_default: Option<String>,// the `_version` wire value the schema declares
}

struct Field {
    name: String,                   // camelCase BO4E wire name
    rust_name: String,              // snake_case Rust name
    is_optional: bool,
    field_type: FieldType,          // after semantic inference
    description: Option<String>,
}
```

### `$ref` Resolution

All `$ref` references are resolved **within the same schema snapshot** — the
generator never makes network requests. A `$ref` to `bo/` becomes `FieldType::Bo`,
to `enum/` becomes `FieldType::BoEnum`, and anything else a `FieldType::Com`. A
field the generator cannot resolve to a concrete shape (an `anyOf` with more than
one non-null branch, a bare `object`) falls back to `serde_json::Value`.

## Metadata comes from the schema

`_typ` and `_version` are read out of each schema file rather than derived from
the type name or the release tag:

| Field | Source | Example |
|---|---|---|
| `_typ` | the property's `const`, falling back to its `default` | `"MARKTLOKATION"` |
| `_version` | the property's `default` | `"202607.1.0"` |

Note that `_version` is **not** the release tag: the tag is `v202607.1.0`, the
wire value `202607.1.0`.

The `BoTyp` / `ComTyp` variant a discriminant maps to is likewise looked up from
the struct that discriminant names, so `"AUFABSCHLAG"` yields
`ComTyp::AufAbschlag`, not the `Aufabschlag` a mechanical case conversion gives.

## Identifier naming (`naming.rs`)

BO4E enum values are SCREAMING_SNAKE_CASE. `screaming_to_camel` splits each value
into runs of letters and runs of digits, title-cases the letter runs, and keeps
the separator at a digit-to-digit boundary — the one place where dropping it
merges two distinct values (`MESSPREIS_G2_5` is meter size G 2.5, `MESSPREIS_G25`
is G 25):

```text
LEISTUNG_PAUSCHAL   → LeistungPauschal
G2KOMMA5            → G2Komma5
MESSPREIS_G2_5      → MesspreisG2_5
MESSPREIS_G25       → MesspreisG25
```

Enums that keep an underscore carry `#[allow(non_camel_case_types)]`. Two values
that would collapse onto one identifier fail generation, naming both.

## Semantic Field Typing

BO4E declares every identifier as a bare string. `generator/src/inference.rs` is
what turns `Marktlokation.marktlokationsId` into a `MaloId` that verifies its own
BDEW check digit. Four rules govern the table:

1. **Keyed on `(struct, field)`** — never a bare name, never a suffix. BO4E
   reuses names, and typing the wrong one takes the whole enclosing object down:
   a `Geschaeftspartner` whose `kontaktwert` will not parse loses its name,
   address, and VAT ID with it.
2. **The schema wins.** The table is consulted only for properties typed as a
   plain, unannotated `"string"`. A `$ref`, a `"format"`, or `"type": "number"`
   is authoritative — a Rust type narrower than the schema cannot read what the
   rest of the ecosystem emits.
3. **Type only what the schema names** — "EIC-Nummer", "OBIS-Kennzahl",
   "Codenummer des Netzbetreibers" — not a field the schema calls merely *a
   code*. A missing newtype costs the caller one `EicCode::try_from(&s)`.
4. **Weigh the blast radius even when rule 3 is satisfied.** How much is lost
   when the newtype rejects a value depends on what encloses the field, and on
   how often a legitimate payload carries something the type will refuse.

Under rule 1, these pairs share a name and differ in meaning:

| Typed | Left as `String` |
|---|---|
| `Marktlokation.marktgebiet` — *"Code vom EIC"* | `MarktgebietInfo.marktgebiet` — *"Der Name des Marktgebietes"* |
| `Marktlokation.regelzone` — *"Code vom EIC"* | `StandorteigenschaftenStrom.regelzone` — *"Der Name der Regelzone"* |

Where BO4E splits a thing into a name and a code they are separate properties, and
only the code half is typed: `StandorteigenschaftenStrom.regelzoneEic` is an
`EicCode`, `.regelzone` is not.

Under rule 2, `Rechnungsposition.einzelpreis` is `Preis` (`$ref`), `Betrag.wert` is
`Decimal` (`"type": "number"`), and `Rechnung.rechnungsdatum` is `OffsetDateTime`
(`"format": "date-time"`) — none need an entry.

Under rule 3, `MarktgebietInfo.marktgebietcode` (*"Die standardisierte Codenummer"*
— standardised by whom is not stated) and `Fremdkostenposition.marktpartnercode`
(*"Die Codenummer (z.B. BDEW-Codenummer)"* — the "z.B." admits other families)
stay `String`.

Under rule 4, two fields the schema *does* name stay `String` anyway:

- **`Zahlungsinformation.iban` / `.bic`** — named outright (*"Eine IBAN-Nummer"*),
  and [`Iban`](@/docs/identifiers.md#iban-and-bic-sepa-bank-identifiers) exists
  and verifies its MOD-97 check digits. But `Zahlungsinformation` hangs off
  `Rechnung` and nothing else, so a **masked** IBAN — `DE89 **** **** 3000`,
  routine on an invoice — would destroy the whole invoice. `iban_checked()` runs
  the check on demand instead.
- **`Bilanzierung.bilanzkreis`** — kept as the general `EicCode` rather than the
  tighter `BilanzkreisId`. A German electricity Bilanzkreis is a party code
  (`11X…`), but the same field carries gas Bilanzkreise whose object type is not
  established here; narrowing it would turn an unverified assumption into a hard
  deserialization failure. Callers opt in via `BilanzkreisId::try_from(eic)`.

### What is typed today

| Rust type | Fields |
|---|---|
| `MaloId` | `Marktlokation`, `Bilanzierung`, `Ausschreibungsdetail` — `marktlokationsId` |
| `MeloId` / `NeloId` | `Messlokation.messlokationsId`, `Netzlokation.netzlokationsId` |
| `SrId` / `TrId` | `SteuerbareRessource.steuerbareRessourceId`, `TechnischeRessource.technischeRessourceId` |
| `EicCode` | `Marktlokation.marktgebiet`, `Marktlokation.regelzone`, `Bilanzierung.bilanzkreis`, `StandorteigenschaftenStrom.regelzoneEic`, `Fremdkostenposition.gebietcodeEic` |
| `BilanzierungsgebietId` | `StandorteigenschaftenStrom.bilanzierungsgebietEic` |
| `MarktpartnerId` | `Marktteilnehmer.rollencodenummer` and the six `*Codenr` / `*Codenummer` fields |
| `ObisCode` | `Energiemenge`, `Lastgang`, `Zaehlwerk` — `obisKennzahl`; `Netzlokation.obiskennzahl` (upstream spells it with a lower-case `k`) |

### Adding an entry

Add the `(struct, field)` pair to `FIELD_TYPES` in `generator/src/inference.rs`,
then implement the newtype in `src/identifiers/` and re-export it from
`identifiers/mod.rs`. See [Identifiers](@/docs/identifiers.md).

Two generator tests keep the table honest: every entry must name a property that
exists and that the schema declares as a plain string, and the homonyms above
must stay untyped.

Before adding one, ask rule 4's question: what else is lost when this newtype
refuses a value, and how often will a legitimate payload carry one it refuses? If
the answer is "a whole invoice" and "routinely", ship the type and an
`*_checked()` accessor instead of an entry.

## Determinism Guarantee

Running the generator twice on the same input produces **byte-identical output**:

- Schema files are read in sorted path order
- Struct fields are sorted by Rust name; enum variants keep schema declaration order
- The wire-key map is emitted from a `BTreeMap`
- Every file is formatted through `prettyplease` before it is written
- Files are written only when their content actually changed
- Files this run did **not** emit are deleted, so the output directory mirrors the
  schema snapshot exactly. A type BO4E retires leaves nothing behind: an orphan
  module would be unreferenced by `mod.rs`, compiled nowhere, and would pass
  every drift check while looking exactly like a live type.

## Drift guards

Because `src/generated/` is committed, it can go stale. What is checked:

| Guard | Where | Catches |
|---|---|---|
| Regenerate-and-diff | `just check-docs-drift` | any stale generated file |
| Schema ↔ module coverage | `tests/generated_contract.rs` | a schema with no module, or a module with no schema |
| `_typ` / `_version` stamping | `tests/generated_contract.rs` | metadata that disagrees with the schema |
| Variant injectivity | `tests/generated_contract.rs` | two wire values collapsing onto one Rust variant |
| Key-map completeness | `src/json/key_transform.rs` tests | a property that does not survive a snake_case round-trip |
| Known-field-key table | `src/json/key_transform.rs` tests | an unsorted or incomplete `KNOWN_FIELD_KEYS`, which would scope the key transform wrongly |
| Extension round-trip | `tests/extension_round_trip.rs` | the key transform renaming keys inside somebody else's JSON |
| Prelude completeness | `tests/prelude_surface.rs` | an identifier type the prelude forgot to re-export |
| `sqlx` impl coverage | `tests/prelude_surface.rs` | an identifier missing from `impl_sqlx_text!`, which compiles fine and simply cannot be a column |
| `Borrow<str>` contract | `tests/prelude_surface.rs` | an identifier whose `Hash` / `Ord` disagrees with the string it borrows as, so a map lookup by `&str` silently misses |
| Field-typing table | `generator/tests/round_trip.rs` | a dead entry, or one overriding a type the schema states |
| Emitter snapshot | `generator/tests/round_trip.rs` | any change to emitted shape, as a reviewable diff |

CI runs all of them.

## What the Generator Does NOT Do

- It does not generate identifier newtype implementations (those are hand-written
  in `src/identifiers/`)
- It does not generate validation logic (that lives in `src/validation/`)
- It does not generate the convenience methods (those live in `src/convenience.rs`)
- It does not make network requests
- It does not modify any file outside `src/generated/`
