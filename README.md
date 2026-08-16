# rubo4e

Rust implementation of [BO4E](https://www.bo4e.de/) — *Geschäftsobjekte für die
Energiewirtschaft*, the object model the German energy industry uses to exchange
contracts, metering points, invoices, and the parties involved.

`rubo4e` generates the full object model from the official JSON Schema, then adds
what the schema cannot express: market identifiers that verify their own BDEW
check digits, enums you can parse strictly at an ingest boundary, and JSON that
stays byte-compatible with the Python, Go, and .NET implementations.

[![Crates.io](https://img.shields.io/crates/v/rubo4e.svg)](https://crates.io/crates/rubo4e)
[![Docs](https://img.shields.io/badge/docs-hupe1980.github.io%2Frubo4e-blue.svg)](https://hupe1980.github.io/rubo4e)
[![API](https://img.shields.io/badge/api-docs.rs-blue.svg)](https://docs.rs/rubo4e)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)
[![Rust 1.87+](https://img.shields.io/badge/rust-1.87%2B-orange.svg)](https://www.rust-lang.org/)

> **Independent implementation.** Not affiliated with or endorsed by the BO4E
> project or BDEW; the reference implementation is
> [BO4E-python](https://github.com/bo4e/BO4E-python).

## Features

- **Generated types** from the official BO4E JSON Schema (v202607)
- **Strong domain identifiers** — the complete BDEW identifier family (`MaloId`, `MeloId`, `NeloId`, `NebeId`, `CrId`, `SgId`, `SrId`, `TrId`, `PaketId`, `EicCode`, `ObisCode`, `MarktpartnerId`, …) with spec-accurate check digits and domain helpers
- **Three-layer validation** — constructor checks, `garde` struct rules, cross-field business logic
- **Strict enum parsing & introspection** — `from_wire` (reject out-of-schema values), `VARIANTS` / `COUNT` / `iter_known`, `Display` / `AsRef<str>`, `is_unknown`, unified by the `Bo4eEnum` trait — all **without** the `strum` feature
- **Recursive strict decoding** — `Bo4eStrict::ensure_known_enums()` rejects any `Unknown` enum value anywhere in a deserialized payload, with JSON-paths — one call replaces hand-written per-field checks
- **Typed builders** — readable, diffable construction via `typed-builder`; setters accept both `T` and `Option<T>` (note: BO4E BO fields are schema-optional, so AHB-mandatory contracts are enforced by your ingest layer, not the type system)
- **German / snake_case / canonical JSON** — BO4E wire format out of the box
- **Ergonomic convenience API** — extension traits, billing-period helpers, EDIFACT agency codes
- **JSON Schema** via `schemars`, OpenAPI via `utoipa`, PostgreSQL via `sqlx`
- **Golden corpus** and **fuzz harnesses** included; proptest round-trip tests run as dev tests

---

## Installation

```sh
cargo add rubo4e
```

That gives you the identifier types only. Add the features you need:

```sh
cargo add rubo4e --features versioned,time,decimal,json,validate
```

---

## Quick Start

```rust
use rubo4e::prelude::*;          // identifiers, BetragExt, MengeExt, PreisExt, Bo4eJsonExt
use rubo4e::v202607::{Vertrag, Sparte};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Typed builder — readable, diffable construction (requires `builder` feature).
    // BO4E BO fields are schema-optional, so any field you omit defaults to None.
    let vertrag = Vertrag::builder()
        .sparte(Sparte::Strom)
        .beschreibung("Jahresvertrag Strom".to_string())
        .vertragsnummer("VN-2026-001".to_string())
        .build();

    // Cross-field struct validation (requires `validate` feature).
    // The prelude re-exports the `Validate` trait, so no direct garde dependency.
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
| `sqlx`      |         | `Type`/`Encode`/`Decode`/`PgHasArrayType` for every identifier **and** every enum (PostgreSQL) |
| `utoipa`    |         | `ToSchema` with pattern/example/description for OpenAPI |
| `strum`     |         | Enum iteration and string conversion              |
| `versioned` |         | Versioned schema modules (`v202607`, `current`)   |
| `tracing`   |         | Structured diagnostics via the `tracing` crate    |
| `metrics`   |         | Counter export hooks (metrics ecosystem)          |

> **Typical full setup:**
> ```sh
> cargo add rubo4e --features versioned,time,decimal,json,validate,builder
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

**Versioning contract.** `rubo4e::current` re-exports the newest stable schema
series. A minor `rubo4e` bump *can* therefore change enum membership or codelist
coverage under `current` (a new variant, a new code) without a source change on
your side. **Pin to the version module (`rubo4e::v202607::…`) for anything whose
shape you guard** (SQL `CHECK` lists, exhaustive mappings, variant-count
assertions); use `current` for code that should always track the latest series.
Every release that changes schema-derived membership records it in the
[CHANGELOG](CHANGELOG.md) **Schema deltas** section. See
[Schema Versioning](https://hupe1980.github.io/rubo4e/docs/versioning/) for the full contract.

---

## Enum Introspection & Strict Parsing

Every generated BO4E enum carries an `Unknown` forward-compatibility catch-all,
so the lenient `serde` / `FromStr` path never fails on an unrecognized wire value
— it maps to `Unknown`. That is the right default for forward-compatibility, but
the wrong default at an ingest boundary that must reject typos, legacy codes, or
values from a newer schema. Every enum therefore also exposes a uniform,
**`strum`-free** surface (also unified by the [`Bo4eEnum`] trait for generic use):

```rust
use rubo4e::{Bo4eEnum, current::Zaehlertyp};

// Introspection without `strum` — drift-guard SQL CHECK lists & mappings:
assert_eq!(Zaehlertyp::COUNT, Zaehlertyp::VARIANTS.len());
for v in Zaehlertyp::iter_known() {           // never yields Unknown
    println!("{}", v.as_wire());              // canonical BO4E wire string
}

// Strict parsing at the boundary — Err instead of a silent Unknown:
assert_eq!(Zaehlertyp::from_wire("WASSERZAEHLER"), Ok(Zaehlertyp::Wasserzaehler));
assert!(Zaehlertyp::from_wire("LFG").is_err());          // legacy/typo rejected
assert!(Zaehlertyp::from_wire("UNKNOWN").is_err());      // catch-all is not a real value

// Detect lenient-decode fall-through after a serde round-trip, in one call:
let z: Zaehlertyp = serde_json::from_value(serde_json::json!("BOGUS")).unwrap();
assert!(z.is_unknown());
```

| Member                       | Feature | Purpose                                                        |
|------------------------------|---------|----------------------------------------------------------------|
| `T::VARIANTS`                | none    | `&'static [T]` of known variants (excludes `Unknown`)          |
| `T::COUNT`                   | none    | stable per-version variant count                               |
| `T::iter_known()`            | none    | iterator over known variants                                   |
| `T::as_wire(&self)`          | none    | canonical BO4E wire string                                     |
| `T::from_wire(s)`            | none    | **strict** parse → `Result<T, UnknownVariant>`                 |
| `T::is_known` / `is_unknown` | none    | detect the `Unknown` catch-all                                 |
| `Display`, `AsRef<str>`      | none    | canonical wire string — now available **without** `strum`      |
| `Bo4eEnum` trait             | `versioned` | the above, generic over the enum type                      |

> `Display`, `AsRef<str>`, `as_wire`, `from_wire`, `VARIANTS`, `COUNT`, and
> `iter_known` are all feature-independent. The `strum` feature now only adds
> `FromStr`, `EnumIter`, and `Into<&'static str>`.

### Strict decoding of whole payloads (`Bo4eStrict`)

Per-enum `from_wire` is strict at the *field* level. But the common pattern is a
lenient whole-object decode (`serde_json::from_value::<Rechnung>()`) used as a
schema gate — and that decode silently turns every unrecognized enum value into
`Unknown`, anywhere in the tree. `Bo4eStrict` closes that gap: **one call** finds
every out-of-schema enum value in a nested value and reports its JSON-path.

```rust
use rubo4e::{Bo4eStrict, current::Netzlokation};

let nelo: Netzlokation = serde_json::from_value(body)?;   // lenient decode (never fails on enums)
nelo.ensure_known_enums()?;                               // Err lists e.g. ["zaehler[1].zaehlertyp"]
```

`ensure_known_enums()` returns [`StrictError`] with the dotted, index-bracketed
paths of every `Unknown` enum value; `unknown_enum_paths()` returns them directly.
Implemented for every BO, COM, enum, and `AnyBo`. This replaces the hand-written
`record.field == T::Unknown` re-checks a strict ingest boundary would otherwise
need. Unlike `Bo4eObject`/`Bo4eEnum`, `Bo4eStrict` is **not sealed**, so you can
implement it on your own domain wrappers to extend the recursive check.

[`Bo4eEnum`]: https://docs.rs/rubo4e/latest/rubo4e/trait.Bo4eEnum.html
[`StrictError`]: https://docs.rs/rubo4e/latest/rubo4e/strict/struct.StrictError.html

---

## Identifiers

All domain identifiers validate their format at construction time. There are no panicking constructors.

| Type                | Format / Rule                                              |
|---------------------|------------------------------------------------------------|
| `MaloId`            | 11 digits, first `1`–`9`, BDEW §8.1 check digit — Marktlokation / Tranche |
| `NeloId`            | Codetyp `'E'` + 9 `[A-Z0-9]` + §8.2 check digit — Netzlokation (BK6-22-128) |
| `NebeId`            | Codetyp `'F'` + 9 `[A-Z0-9]` + §8.2 check digit — Netzbereich (BK6-22-300) |
| `CrId`              | Codetyp `'A'` + 9 `[A-Z0-9]` + §8.2 check digit — Cluster Ressource |
| `SgId`              | Codetyp `'B'` + 9 `[A-Z0-9]` + §8.2 check digit — Steuergruppe |
| `SrId`              | Codetyp `'C'` + 9 `[A-Z0-9]` + §8.2 check digit — Steuerbare Ressource |
| `TrId`              | Codetyp `'D'` + 9 `[A-Z0-9]` + §8.2 check digit — Technische Ressource |
| `PaketId`           | Codetyp `'P9'` + 8 `[A-Z0-9]` + §8.2 check digit — Netzbetreiberwechsel |
| `MeloId`            | 33 chars: 2-char ISO country code + 31 alphanumeric        |
| `EicCode`           | 16-char EIC with ENTSO-E check character and object type   |
| `BilanzkreisId`     | 16-char EIC restricted to object type `'X'` (Party) — Bilanzkreis, MaBiS / GaBi Gas |
| `BilanzierungsgebietId` | 16-char EIC restricted to object type `'Y'` (Area) — Bilanzierungsgebiet, MaBiS |
| `ObisCode`          | `[A-B:]C.D[.E][*F]`, value groups are octets; C=0 permitted (IEC 62056-61 general metering group) |
| `MarktpartnerId`    | 13 decimal digits — BDEW (99), DVGW (98), or GS1 GLN; check digit opt-in |
| `AkivId`            | 1–36 printable ASCII chars — Aktivierungsidentifikator Redispatch 2.0 (BK6-24-174) |
| `TranchennummerId`  | 1–6 decimal digits, no leading zeros — MABIS Bilanzkreisabrechnung (PID 13003) |

Section numbers refer to the BDEW Anwendungshilfe *"Identifikatoren in der
Marktkommunikation"* v1.2 (7 February 2025). Chapter 8 defines a single
check-digit arithmetic — sum the mapped values at odd positions, add twice the sum
at even positions, take the difference to the next multiple of 10 — in two flavours:
§8.1 for numeric IDs and §8.2 (the "ASCII-Verfahren", where `A`–`Z` map to their
ASCII codes) for alphanumeric ones. Both are implemented once and pinned to the
worked examples printed in the specification.

```rust
// Build from base — the check digit is computed, never typed by hand.
let malo = MaloId::from_base("4137355924")?;   // → "41373559241"  (BDEW §8.1 example)
let c    = MaloId::check_digit("4137355924")?; // → 1u8
assert_eq!(malo.vergabestelle(), MaloVergabestelle::Bdew);

// Every §8.2 identifier shares the same API and enforces its own Codetyp.
let nelo  = NeloId::from_base("E000000001")?;  // → "E0000000019"
let tr    = TrId::from_base("D000000001")?;    // → "D0000000010"
let paket = PaketId::from_base("P900000001")?; // → "P9000000010"
assert!(NeloId::new("D0000000010").is_err());  // Codetyp mismatch — that is a TrId

// Country code extraction (MeloId)
let melo = MeloId::new("DE0000000000000000000000000000001")?;
assert_eq!(melo.country_code(), "DE");
assert!(melo.is_german());

// EDIFACT agency codes (MarktpartnerId) — eliminates duplicate mapping tables
let mp = MarktpartnerId::new("9900357000003")?;
assert_eq!(mp.authority(), MpIdAuthority::Bdew);
assert_eq!(mp.nad_agency_code(), "293");  // EDIFACT NAD DE3055
assert_eq!(mp.unb_agency_code(), "500");  // EDIFACT UNB DE0007

// MP-IDs carry either a BDEW (§8.1) or a GS1/EAN-13 check digit; opt in explicitly.
assert!(mp.has_valid_bdew_check_digit());
assert!(MarktpartnerId::new_checked("9900357000000").is_err());

// Integer round-trip for legacy systems
assert_eq!(mp.to_i64(), 9_900_357_000_003_i64);

// Serde as integer (opt-in, field-level)
#[serde(with = "rubo4e::identifiers::marktpartner_id_as_i64")]
pub partner_id: MarktpartnerId,
```

### EIC codes and object types

Position 3 of an EIC carries the ENTSO-E **object type**, and the German market
leans on it: a Bilanzkreis is a market party (`11X…`), while a Bilanzierungsgebiet
is an area (`11Y…`). `EicType` exposes all seven types, and the two restricted
newtypes make the roles unswappable at a call site.

```rust
use rubo4e::identifiers::{BilanzierungsgebietId, BilanzkreisId, EicCode, EicType};

let area = EicCode::new("10YDE-EON------1")?;   // TenneT control area
assert_eq!(area.eic_type(), EicType::Area);     // 'Y'

let party = EicCode::new("11XSUEDWESTSTRO8")?;  // a Bilanzkreis
assert_eq!(party.eic_type(), EicType::Party);   // 'X'

// The restricted types pin position 3, so the two cannot be confused.
let bk = BilanzkreisId::new("11XSUEDWESTSTRO8")?;
assert!(BilanzierungsgebietId::new("11XSUEDWESTSTRO8").is_err());

let bg = BilanzierungsgebietId::new("11YN-0000-0001-Q")?;
assert!(BilanzkreisId::new("11YN-0000-0001-Q").is_err());

// The check character is derived, never typed by hand.
assert_eq!(BilanzkreisId::from_prefix("11XSUEDWESTSTRO")?.as_ref(), "11XSUEDWESTSTRO8");

// Widening is infallible; narrowing is checked.
let eic: EicCode = bk.into();
assert!(BilanzkreisId::try_from(eic).is_ok());
```

### OBIS codes (EDIFACT support)

`ObisCode` parses once at construction and stores a **canonical** form, so two
spellings of the same code are equal and hash alike. Value groups are single
octets, as IEC 62056-61 specifies.

```rust
// Standard OBIS codes
let obis = ObisCode::new("1-0:1.8.0")?;       // active energy total
let obis = ObisCode::new("0-0:0.0.0")?;       // C=0 — general metering group (IEC 62056-61)

// Canonicalisation: `&` becomes `*`, and leading zeros are dropped.
assert_eq!(ObisCode::new("1.8.1&255")?,      ObisCode::new("1.8.1*255")?);
assert_eq!(ObisCode::new("01-00:01.08.00")?, ObisCode::new("1-0:1.8.0")?);
assert_eq!(ObisCode::new("01-00:01.08.00")?.as_str(), "1-0:1.8.0");

// Value groups are octets — 256 is not an OBIS value.
assert!(ObisCode::new("1-0:1.8.256").is_err());

// Components are stored, so this neither re-parses nor allocates.
let parts = ObisCode::new("1-0:1.8.0*255")?.components();
assert_eq!((parts.a, parts.c, parts.f), (Some(1), 1, Some(255)));

// PIA item-number form drops the F component.
assert_eq!(ObisCode::new("1-0:1.8.0*255")?.to_pia_string(), "1-0:1.8.0");
```

---

## Multi-version Dispatch

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

See [Schema Versioning](https://hupe1980.github.io/rubo4e/docs/versioning/) for the full upgrade workflow.

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

The snake_case mapping is an exact table emitted by the code generator, not a
runtime heuristic, so `from_json_snake_case(to_json_snake_case(x)) == x` holds
for every generated type. That is not achievable algorithmically: BO4E names like
`hoechstpreisHT`, `kundengruppeKA`, and `Sigmoidparameter`'s `A`/`B`/`C`/`D`
render to a snake form a heuristic maps back to a *different* camelCase name,
which silently diverts the value into `_additional`. BO4E metadata keys (`_typ`,
`_version`, `_id`) and unknown extension keys pass through byte-for-byte in both
directions. See [Serialization](https://hupe1980.github.io/rubo4e/docs/serialization/#how-snake-case-keys-are-mapped).

### Parsing untrusted input

Preserving unknown fields is a memory-growth surface, so every deserialization
path — hardened or not — caps extension fields at 128 per struct and extension
keys at 256 bytes, and rejects documents nested deeper than 128 levels.

For payloads from outside your trust boundary, the `_hardened` entry points add
four opt-in budgets on top:

```rust
use rubo4e::json::{Bo4eJsonExt, JsonParseLimits};

let malo = Marktlokation::from_json_german_hardened(
    &body,
    JsonParseLimits::untrusted_defaults(),   // 1 MB / depth 64 / 64 KB ext / 32 fields
)?;
```

All four limits are enforced **during** parsing and at **every** nesting level —
extension data buried in a nested COM is charged to the same budget as extension
data on the root — so an oversized payload is rejected while it is being read,
not after the object tree has been allocated. Every limit that fires bumps a
process-wide counter readable via `json_limit_hit_counters()`, exported to the
`metrics` ecosystem when the `metrics` feature is on.

See [Serialization](https://hupe1980.github.io/rubo4e/docs/serialization/#hardened-deserialization-for-untrusted-inputs)
for the exact scope of each limit.

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
// MaloId → { type: string, pattern: "^[0-9]{11}$", example: "51238696781" }
```

---

## SQLx Integration

```rust
// Requires the `sqlx` feature — implements Type, Encode, Decode and
// PgHasArrayType for every identifier and every generated enum.
// No `json` feature needed: everything round-trips through &str.

// Bind directly as a typed identifier
sqlx::query("INSERT INTO malo (id) VALUES ($1)")
    .bind(&malo_id)
    .execute(&pool).await?;

// Decode directly — runs the same validation as new()
let id: MaloId = row.try_get("malo_id")?;

// Vec<Id> binds to a TEXT[] column
sqlx::query("SELECT * FROM malo WHERE id = ANY($1)")
    .bind(&malo_ids)
    .fetch_all(&pool).await?;

// Works in FromRow structs too
#[derive(sqlx::FromRow)]
struct MpRow {
    mp_id: MarktpartnerId,
}
```

Identifiers reject invalid values on decode. Enums decode **leniently** —
an out-of-schema string becomes `Unknown`, mirroring the serde path — so use
`from_wire` on a `String` column where that must be an error instead.

---

## Documentation

**[hupe1980.github.io/rubo4e](https://hupe1980.github.io/rubo4e)** — guides and design notes.
**[docs.rs/rubo4e](https://docs.rs/rubo4e)** — per-item API reference.
**[CHANGELOG](CHANGELOG.md)** — release history and upgrade notes.

| Guide | Covers |
|---|---|
| [Architecture](https://hupe1980.github.io/rubo4e/docs/architecture/) | Workspace layout, module tree, feature-gate reference |
| [Identifiers](https://hupe1980.github.io/rubo4e/docs/identifiers/) | Every identifier type, its validation rules, and the BDEW check-digit procedures |
| [Serialization](https://hupe1980.github.io/rubo4e/docs/serialization/) | JSON output modes, extension data, hardened parsing |
| [Validation](https://hupe1980.github.io/rubo4e/docs/validation/) | The three validation layers and `Validated<T>` |
| [Schema Versioning](https://hupe1980.github.io/rubo4e/docs/versioning/) | Version modules, `current`, and the upgrade workflow |
| [Ecosystem](https://hupe1980.github.io/rubo4e/docs/ecosystem/) | sqlx, schemars, utoipa, strum integrations |
| [Code Generator](https://hupe1980.github.io/rubo4e/docs/generator/) | How generation works and how to re-run it |
| [Testing](https://hupe1980.github.io/rubo4e/docs/testing/) | The seven testing layers and how to run each |

The site sources live in [`site/`](site/) and are built with [Zola](https://www.getzola.org).

---

## MSRV

The minimum supported Rust version is **1.87**, declared as `rust-version` in
`Cargo.toml` and verified in CI on every push. MSRV advances only when the
current floor is two stable releases behind, and a bump is a **minor** version
change, never a patch.

---

## License

Licensed under either of [Apache License 2.0](LICENSE-APACHE) or [MIT License](LICENSE-MIT),
at your option.
