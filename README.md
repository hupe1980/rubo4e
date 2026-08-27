# rubo4e

Rust implementation of [BO4E](https://www.bo4e.de/) — *Geschäftsobjekte für die
Energiewirtschaft*, the object model the German energy industry uses to exchange
contracts, metering points, invoices, and the parties involved.

`rubo4e` generates the full object model from the official JSON Schema, then adds
what the schema cannot express: market identifiers that verify their own BDEW
check digits, enums you can parse strictly at an ingest boundary, and JSON that
reads what Python, Go, and .NET write — and writes what the reference Python
implementation does.

[![Crates.io](https://img.shields.io/crates/v/rubo4e.svg)](https://crates.io/crates/rubo4e)
[![Docs](https://img.shields.io/badge/docs-hupe1980.github.io%2Frubo4e-blue.svg)](https://hupe1980.github.io/rubo4e)
[![API](https://img.shields.io/badge/api-docs.rs-blue.svg)](https://docs.rs/rubo4e)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)
[![Rust 1.88+](https://img.shields.io/badge/rust-1.88%2B-orange.svg)](https://www.rust-lang.org/)

> **Independent implementation.** Not affiliated with or endorsed by the BO4E
> project or BDEW; the reference implementation is
> [BO4E-python](https://github.com/bo4e/BO4E-python).

## Features

- **Generated types** from the official BO4E JSON Schema, generated from a committed snapshot (`v202607.1.0`) so the codegen is reproducible
- **Strong domain identifiers** — the complete BDEW identifier family (`MaloId`, `MeloId`, `NeloId`, `NebeId`, `CrId`, `SgId`, `SrId`, `TrId`, `PaketId`, `EicCode`, `ObisCode`, `MarktpartnerId`, …) plus the SEPA pair (`Iban`, `Bic`), with spec-accurate check digits and domain helpers
- **Three-layer validation** — constructor checks, `garde` struct rules, cross-field business logic
- **Strict enum parsing & introspection** — `from_wire` (reject out-of-schema values), `VARIANTS` / `COUNT` / `iter_known`, `Display` / `AsRef<str>`, `is_unknown`, unified by the `Bo4eEnum` trait — all **without** the `strum` feature
- **Recursive strict decoding** — `Bo4eStrict::ensure_known_enums()` rejects any `Unknown` enum value anywhere in a deserialized payload, with JSON-paths — one call replaces hand-written per-field checks
- **Typed builders** — readable, diffable construction via `typed-builder`; setters accept both `T` and `Option<T>` (note: BO4E BO fields are schema-optional, so AHB-mandatory contracts are enforced by your ingest layer, not the type system); `Lastgang` and `Tarif`, the two the schema marks `required`, get a feature-free `new(…)`
- **Type-level `_typ` facts** — `T::TYP`, `T::TYP_WIRE`, `T::SCHEMA_VERSION`, `T::SCHEMA_SERIES` as associated constants on every BO **and** COM, so generic code needs no value and no `Default` bound
- **German / snake_case / canonical JSON** — BO4E wire format out of the box, with a hardened path for untrusted input
- **`Eq` + `Hash` on generated types** without the `json` feature, so a BO can key a `HashMap`; enums are always `Eq + Ord + Hash`
- **Ergonomic convenience API** — extension traits, billing-period helpers, EDIFACT agency codes
- **JSON Schema** via `schemars`, OpenAPI via `utoipa`, PostgreSQL via `sqlx`
- **Golden corpus**, **fuzz harnesses**, and **drift guards** that fail the build when the committed codegen stops matching the pinned schema

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
| `time`      |         | `time` crate — `Date` for date fields, `OffsetDateTime` for timestamps; also turns on `utoipa/time` |
| `decimal`   |         | `rust_decimal::Decimal` for amounts and prices; also turns on `schemars/rust_decimal1` and `utoipa/decimal` |
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

`JsonSchema`/`ToSchema` for `Decimal` and the `time` types ride on `decimal` and
`time`, not on `schemars`/`utoipa`, so this crate does not become your
workspace's accidental sole provider of them. If you derive `JsonSchema` over a
`Decimal` of your own, declare `schemars = { features = ["rust_decimal1"] }`
yourself — see [Ecosystem](https://hupe1980.github.io/rubo4e/docs/ecosystem/).

---

## Schema Versions

| Module    | Built from  | Status         |
|-----------|-------------|----------------|
| `v202607` | v202607.1.0 | Current stable |

```rust
use rubo4e::v202607::Marktlokation;  // the v202607 series
use rubo4e::current::Marktlokation;  // whichever series is newest — moves with crate updates
```

**Three spellings of a release.** The Rust module is the *series* (`v202607`).
The git tag carries a `v` and the full triple (`v202607.1.0`). The `_version`
field *inside a payload* has the triple without the `v` (`202607.1.0`).
`Bo4eTyped::SCHEMA_VERSION` is the wire spelling, `SCHEMA_SERIES` the series.

**The `_typ` facts are associated constants** on [`Bo4eTyped`], carried by every
BO *and* COM, so one bound reaches both and generic code needs no value — hence
no `Default` bound, which is what admits `Lastgang` and `Tarif`, the two types
the schema marks `required`:

```rust
use rubo4e::{current::{Adresse, BoTyp, Lastgang, Vertrag}, Bo4eObject, Bo4eTyped};

assert_eq!(Vertrag::TYP, BoTyp::Vertrag);
assert_eq!(Vertrag::SCHEMA_SERIES, "202607");

fn wire_typ<T: Bo4eTyped>() -> &'static str { T::TYP_WIRE }
assert_eq!(wire_typ::<Lastgang>(), "LASTGANG");   // a BO
assert_eq!(wire_typ::<Adresse>(),  "ADRESSE");    // a COM

// `Bo4eObject` / `Bo4eComponent` narrow it and bind the discriminant enum.
fn bo_typ<T: Bo4eObject<Typ = BoTyp>>() -> BoTyp { T::TYP }
```

`T::TYP` is what the type **is**, never the `_typ` a payload claimed — the public
`typ` field holds that.

[`Bo4eTyped`]: https://docs.rs/rubo4e/latest/rubo4e/trait.Bo4eTyped.html

**Dispatch on the series, not the release.** BO4E ships patch releases inside a
series, so a sender one patch ahead stamps a `_version` that an equality match
rejects — for a payload these types read perfectly:

```rust
match incoming_version.split('.').next() {
    Some("202607") => { /* rubo4e::v202607::… */ }
    _ => return Err(unsupported),
}
```

**Versioning contract, stated honestly.** The module path pins the *series*; the
`rubo4e` version pins the *values*. Enum membership can move inside a series
because BO4E moves it — `v202607.1.0` removed `Messgroesse::PREISE` and dropped
two enums outright. Importing `rubo4e::v202607::…` rather than `rubo4e::current::…`
means you will not silently cross a format-version cutover, but it does not
freeze a variant set: for that, pin the crate version and upgrade deliberately.
Guard the rest structurally with `T::VARIANTS` / `T::COUNT` so a schema bump
fails in CI. Every release that changes schema-derived membership records it in
the [CHANGELOG](CHANGELOG.md) **Schema deltas** section, removals included. See
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
| `Display`, `AsRef<str>`      | none    | canonical wire string, **without** `strum`                     |
| `Bo4eEnum` trait             | `versioned` | the above, generic over the enum type                      |

> `Display`, `AsRef<str>`, `as_wire`, `from_wire`, `VARIANTS`, `COUNT`, and
> `iter_known` are all feature-independent. The `strum` feature adds only
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
need. Unlike `Bo4eTyped`/`Bo4eEnum`, `Bo4eStrict` is **not sealed**, so you can
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

### IBAN and BIC

`Zahlungsinformation.iban` and `.bic` are the two fields on a BO4E invoice that
money moves against, and the schema declares both as bare strings. An IBAN's
ISO 7064 MOD-97-10 check digits catch **every** single-character error and
**every** adjacent transposition.

```rust
// Grouping spaces and lowercase normalise away, so a value pasted from a bank
// statement parses; `as_ref()` always returns the compact wire form.
let iban = Iban::new("de89 3704 0044 0532 0130 00")?;
assert_eq!(iban.as_ref(), "DE89370400440532013000");
assert_eq!(iban.to_grouped_string(), "DE89 3704 0044 0532 0130 00");
assert_eq!(iban.bankleitzahl(), Some("37040044"));
assert!(Iban::new("DE89370400440532013090").is_err());   // transposed digits

let bic = Bic::new("GENODEF1S04")?;
assert_eq!((bic.institution_code(), bic.country_code()), ("GENO", "DE"));
assert!(bic.is_passive());   // location code ending in 1, per ISO 9362
```

The generated `Zahlungsinformation` keeps both fields as `String`, deliberately:
it hangs off `Rechnung` and nothing else, so a **masked** IBAN
(`DE89 **** **** 3000`, routine on an invoice) would take the whole invoice down
with it. Run the check on demand instead — the error costs you the field, not the
invoice:

```rust
match zahlungsinformation.iban_checked() {
    None           => { /* no IBAN stated */ }
    Some(Ok(iban)) => { /* verified */ }
    Some(Err(e))   => { /* stated but invalid — masked, mistyped, truncated */ }
}
```

---

## Multi-version Dispatch

When a storage layer (e.g. PostgreSQL `JSONB`) writes a `bo4e_version` column alongside
BO4E JSON, the idiomatic dispatch is a plain `match` — on the **series**, not the
exact release:

```rust
use rubo4e::{v202607, Bo4eTyped as _};

fn process_rechnung(
    json: &str,
    bo4e_version: &str,          // the payload's own `_version`, e.g. "202607.1.0"
) -> Result<(), Box<dyn std::error::Error>> {
    match bo4e_version.split('.').next() {
        Some("202607") => {
            let r: v202607::Rechnung = serde_json::from_str(json)?;
            // r.schema_series() == "202607" — always matches this arm
            handle_v202607(r)
        }
        // When the v202801 series ships, add one arm:
        // Some("202801") => handle_v202801(serde_json::from_str::<v202801::Rechnung>(json)?),
        _ => Err(format!("unsupported schema series: {bo4e_version}").into()),
    }
}
```

Matching the full `_version` string instead would reject a payload from a sender
one BO4E patch ahead of you — `"202607.2.0"` against a `"202607.1.0"` arm — even
though the `v202607` types read it perfectly. `schema_series()` returns exactly
the value the `match` keys on, so a test can assert the two agree.

This pattern:
- Requires no new rubo4e API — `schema_series()` is already on every BO and COM via `Bo4eTyped`
- Is trivially extensible: each new schema series is one `match` arm, and patches inside a series need none
- Localises migration to the storage layer; business logic only handles the series it was written for
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

// Rechnung — closed billing period, as a RangeInclusive<Date>
if let Some(period) = rechnung.billing_period() {
    println!("Invoice period: {} – {} inclusive", period.start(), period.end());
    let billed = period.contains(&date!(2026-01-31));   // the end date is inside
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

// Zeitraum — BO4E declares *both* dates inclusive: the period is [start, end]
let range    = z.as_inclusive_range();        // Option<RangeInclusive<Date>>
let bounds   = z.bounds();                    // (Option<Date>, Option<Date>)
let days     = z.whole_days();                // Option<i64> — January is 31
let contains = z.contains(date!(2026-01-31)); // bool — the end date is inside
let dauer    = z.duration();                  // Option<Result<time::Duration, _>>
let start    = z.startuhrzeit_parsed();       // Option<Result<(Time, Option<UtcOffset>), _>>
```

**Interval conventions are not uniform in BO4E**, and this is the trap:

| Kind | Interval |
|---|---|
| `date-time` pairs (`vertragsbeginn`/`vertragsende`, `von`/`bis`) | `[start, end)` |
| `Zeitraum`'s **date** pair | `[start, end]` |
| `Zeitraum`'s **time** pair (`startuhrzeit`/`enduhrzeit`) | `[start, end)` |
| price-tier bounds (`staffelgrenzeVon`/`Bis`) | `[von, bis]`, plus a gap rule |

`enddatum` is inclusive — *"Enddatum des betrachteten Zeitraums ist
**inklusiv**"*, with `'2025-01-01'` given as the example for *both* date fields,
so `start == end` is a valid one-day period. Reading it exclusively drops a day
from every period. `as_inclusive_range` returns a `RangeInclusive` so the
convention travels with the value.
[`tests/interval_conventions.rs`](tests/interval_conventions.rs) reads each
convention out of the committed schema and checks it against the code.

Three `Zeitraum` values have no `time` type that holds them, so they keep the
wire string and an accessor parses on demand: `dauer` is an ISO 8601 duration
(`duration()` refuses `P1Y`/`P1M` rather than guessing their length), and the two
`*uhrzeit` fields are times of day **with a UTC offset**.
`PreisstaffelSliceExt::select_for` picks a price tier, including BO4E's rule that
a value between two tiers *"rutscht in die obere Zone"* — which a plain
`von <= x <= bis` scan misses entirely.

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
handling of new BO4E fields without library updates. Keys and values come back
unchanged, and the top-level ones keep their arrival order; key order *inside* a
nested extension object does not survive, because `serde_json::Value` stores an
object in a sorted map.

**Decimal amounts serialize as JSON strings** (`"wert": "119.00"`), matching
BO4E-python. Deserialization accepts JSON numbers too, the way go-bo4e writes
them — but only the string spelling is exact. A JSON number has already passed
through `f64` before any Rust deserializer sees it, so `119.00` arrives as `119`
(scale lost) and anything past ~15 significant digits is rounded. Nothing in the
German energy market comes near that many digits, so this is a fidelity question
rather than a correctness one; `decimal_serde::decimal_from_json_number_count()`
counts every such read so you can tell which spelling your producers use. See
[Serialization](https://hupe1980.github.io/rubo4e/docs/serialization/#decimal-amounts-are-written-as-json-strings).

The snake_case mapping is an exact table emitted by the generator, not a runtime
heuristic, so `from_json_snake_case(to_json_snake_case(x)) == x` holds for every
generated type — which a heuristic cannot: `hoechstpreisHT`, `kundengruppeKA`, and
`Sigmoidparameter`'s `A`/`B`/`C`/`D` all invert to a *different* camelCase name.
BO4E metadata keys (`_typ`, `_version`, `_id`) pass through byte-for-byte, and so
does extension data **including everything nested under it** — the transform
switches off at the edge of the schema, so a vendor blob holding `{"a": 3}` is not
rewritten to `{"A": 3}`. One ambiguity it cannot resolve: a *top-level* extension
key that is a field's own snake spelling is indistinguishable from that field, so
prefer the German mode whenever extension data is in play. See
[Serialization](https://hupe1980.github.io/rubo4e/docs/serialization/#how-snake-case-keys-are-mapped).

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

// …or narrowed, where you know your own payloads:
let strict = JsonParseLimits::untrusted_defaults()
    .with_max_payload_bytes(Some(64 * 1024))
    .with_max_extension_field_count(Some(0));  // reject any unknown field
```

`max_payload_bytes` is checked before a byte is parsed; the other three are
enforced during the single parse pass, at **every** nesting level — extension
data buried in a nested COM is charged to the same budget as extension data on
the root. Every limit that fires bumps a process-wide counter readable via
`json_limit_hit_counters()`, exported to the `metrics` ecosystem when the
`metrics` feature is on.

These bound what a payload *retains*, not what parsing it allocates:
`#[serde(flatten)]` buffers a struct's unrecognised fields before the extension
map sees them, so `max_payload_bytes` is the cap that bounds peak memory — set it
first. Nor do any of them bound the object graph: `[{},{},{}…]` is three wire
bytes and a full struct per element, so size `max_payload_bytes` against the
expanded cost and keep a concurrency limit in front of the endpoint.

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

// …and it validates on the way *in*, so a request body cannot skip the check:
let malo: Validated<Marktlokation> = serde_json::from_str(&body)?;
```

`.validate()` is **recursive**: it checks the value's own rules and descends into
every nested BO, COM, and identifier, reporting each failure at its path
(`rechnungsperiode`, `kostenbloecke[0].kostenpositionen[0]`). One call covers the
tree.

Cross-field rules run automatically via `#[garde(custom(...))]` attributes on the
generated types:

| Type | Rule |
|---|---|
| `Marktlokation`, `Messlokation` | **at most one** of `lokationsadresse`(`messadresse`) / `geoadresse` / `katasterinformation` |
| `Vertrag` | `vertragsbeginn` strictly before `vertragsende` |
| `Bilanzierung` | `bilanzierungsbeginn` ≤ `bilanzierungsende` |
| `Zeitraum` | at least one temporal field; `startdatum` **on or before** `enddatum` (both bounds inclusive) |
| `Rechnung` | one currency throughout; `gesamtnetto + gesamtsteuer == gesamtbrutto`; `steuerbetraege` sum to `gesamtsteuer` |
| `Kostenposition` | `einzelpreis × menge` rounds to `betrag_kostenposition` at its own scale |

**Every rule traces to a sentence in the BO4E schema, and only those do**, so
`.validate()` answers *"does this conform to BO4E"* — a claim you can make about
a counterparty's document. This crate's own judgements live in
`validation::current::quality` and are called by name:

```rust
use rubo4e::validation::current::quality;

rechnung.validate()?;                               // conformance
quality::rechnung_totals_are_complete(&rechnung)?;  // opt-in house rule
```

*At most one* Ortsangabe, not exactly one: BO4E states mutual exclusivity, not
presence. And it has no reference type, so a location referenced from a
`Rechnung` or a `Vertrag` is a full `Marktlokation` carrying little more than its
ID — which makes the empty case the common one.

Not asserted: **presence** (BO4E marks almost every field optional, so a
`Validated<T>` does not prove your AHB's mandatory fields are there) and
**`zuZahlen`** (its equation names a `rabattBrutto` field v202607 does not ship).

Import from `rubo4e::validation::current`, the counterpart of `rubo4e::current`,
so no file has to name a schema version.
See [Validation](https://hupe1980.github.io/rubo4e/docs/validation/).

---

## OpenAPI / JSON Schema

```rust
// schemars — JSON Schema (requires `schemars` feature)
let schema = schemars::schema_for!(rubo4e::v202607::Marktlokation);

// utoipa — OpenAPI 3.1 (requires `utoipa` feature)
// All identifier types emit pattern, description, and example values:
// MaloId → { type: string, pattern: "^[1-9][0-9]{10}$", example: "41373559241" }
// (the leading digit is the Vergabestelle, and 0 is not assigned — see §3.2)
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

The minimum supported Rust version is **1.88**, declared as `rust-version` in
`Cargo.toml` and verified in CI on every push. MSRV advances only when the
current floor is two stable releases behind, and a bump is a **minor** version
change, never a patch.

The floor is set by the dependency tree rather than by this crate's own source:
`time` and `home` (via `sqlx`) both require 1.88. Because Cargo's
default resolver picks the newest semver-compatible dependency without regard to
`rust-version`, a toolchain below the floor fails at *resolution* time with
`rustc 1.87.0 is not supported by the following packages` rather than at compile
time. On an older toolchain, either pin those dependencies back with
`cargo update <crate> --precise <version>` or enable Cargo's MSRV-aware resolver.

---

## License

Licensed under either of [Apache License 2.0](LICENSE-APACHE) or [MIT License](LICENSE-MIT),
at your option.
