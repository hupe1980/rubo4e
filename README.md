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
- **Recursive unknown-field detection** — `Bo4eExtensions::ensure_no_extension_data()` finds every field BO4E does not define, at any depth; a decode cannot, since a misspelled key decodes cleanly and reads back as `None`
- **Typed builders** — readable, diffable construction via `typed-builder`; setters accept both `T` and `Option<T>` (note: BO4E BO fields are schema-optional, so AHB-mandatory contracts are enforced by your ingest layer, not the type system); `Lastgang` and `Tarif`, the two the schema marks `required`, get a feature-free `new(…)`
- **Type-level `_typ` facts** — `T::TYP`, `T::TYP_WIRE`, `T::SCHEMA_VERSION`, `T::SCHEMA_SERIES` as associated constants on every BO **and** COM, so generic code needs no value and no `Default` bound
- **German / snake_case / canonical JSON** — BO4E wire format out of the box, with a hardened path for untrusted input
- **`Eq` + `Hash` on generated types** without the `json` feature, so a BO can key a `HashMap`; enums are always `Eq + Ord + Hash`
- **Time-series audit** — `Lastgang` / `Zeitreihe` placed on a timeline in one call: gaps, overlaps, wrong-length intervals, unusable readings, coverage ratio — and `integrate()`, the step from a load profile in kW to the energy an invoice bills
- **One reading shape for every interval series** — `Lastgang`, `Zeitreihe` **and** `Energiemenge` all produce an `IntervalReading`, and all three read it back; `total_energy()` answers in kWh from a kW load profile or a kWh series alike
- **Lokationsbündelstrukturen** — BO4E has no `Lokationsbuendel` BO, so this ships the published EDI@Energy codelist (15 structures, 27 object codes) as data, plus `audit_buendel()`: unknown codes, an object filed under the wrong type, every cardinality the structure states
- **Namespaced `ZusatzAttribut`s** — a `hems:` / `mako:` convention with typed get/set on every BO **and** COM, so two producers writing what BO4E does not model cannot overwrite each other
- **Market rules beyond the schema** — the `Zählpunkt` that is deliberately not a Messlokation, the resting Aggregationsverantwortung, the Bilanzierungsgebiet EIC typed as an area code — read off BO4E fields, never as a forked enum
- **Register arithmetic** — `Zaehlwerk` consumption per BO4E's own formula, correcting a meter wrap-around (`999998 → 000012` is 14, not −999 986) and refusing where a meter exchange makes the difference meaningless
- **Unit dimensions** — `Mengeneinheit` grouped into eleven physical dimensions, with exact conversion, the energy ↔ power pairing, and calendar units refused rather than averaged
- **Ergonomic convenience API** — extension traits, billing-period helpers, EDIFACT agency codes
- **JSON Schema** via `schemars`, **OpenAPI** via `utoipa` — every identifier with a pattern, a German description and a check-digit-valid example, identical in both — **PostgreSQL** via `sqlx`
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

### A decode does **not** validate field names (`Bo4eExtensions`)

Serde ignores keys a struct does not declare, and this crate keeps them in
`_additional` so a payload from a newer schema survives. So decoding a document
is not a check on it — a misspelled key decodes cleanly and reads back as `None`:

```rust
let body = serde_json::json!({
    "_typ": "KOSTEN",
    "kostenbloecke": [{ "kostenblockBEZEICHNUNG": "x" }]   // misspelled
});
let kosten: Kosten = serde_json::from_value(body.clone())?;   // cannot fail
assert_eq!(kosten.kostenbloecke.unwrap()[0].kostenblockbezeichnung, None);
```

`Bo4eExtensions` is the recursive check that answers, with JSON-paths:

```rust
use rubo4e::json::Bo4eExtensions;

assert_eq!(kosten.extension_paths(), ["kostenbloecke[0].kostenblockBEZEICHNUNG"]);
kosten.ensure_no_extension_data()?;   // Err(UnknownFieldError { paths })
```

Or make the decode itself the check — `from_json_value` and
`from_json_value_hardened` are the `serde_json::Value` counterparts of the `&str`
readers, with the same depth and extension budgets:

```rust
let closed = JsonParseLimits::unlimited().with_max_extension_field_count(Some(0));
Kosten::from_json_value_hardened(body, closed)?;   // Err on any stray key
```

A payload can leave the schema in two ways, and neither check sees the other's
finding:

| Question | Call |
|---|---|
| Does it use a **value** this schema version does not define? | `ensure_known_enums()` — `Bo4eStrict` |
| Does it use a **field** this schema version does not define? | `ensure_no_extension_data()` — `Bo4eExtensions` |

Rejecting an unknown *value* is usually right at an ingest boundary; rejecting an
unknown *field* usually is not — that is how a counterparty one release ahead
reaches you. Run the field check on documents you **produce**. Better still,
construct values typed, where a rename is a compile error. See
[Serialization](https://hupe1980.github.io/rubo4e/docs/serialization/#a-decode-does-not-validate-field-names).

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
| `Zaehlpunktbezeichnung` | the same 33 chars — a Zählpunkt that is **not** a Messlokation (MaBiS; BK6-20-160 §1.6.2) |
| `EicCode`           | 16-char EIC with ENTSO-E check character and object type   |
| `BilanzkreisId`     | 16-char EIC restricted to object type `'X'` (Party) — Bilanzkreis, MaBiS / GaBi Gas |
| `BilanzierungsgebietId` | 16-char EIC restricted to object type `'Y'` (Area) — Bilanzierungsgebiet, MaBiS |
| `ObisCode`          | `[A-B:]C.D[.E][*F]`, value groups are octets; C=0 permitted (IEC 62056-61 general metering group) |
| `MarktpartnerId`    | 13 decimal digits — BDEW (99), DVGW (98), or GS1 GLN; check digit opt-in |
| `Lokationsbuendelcode` | 13 decimal digits, §8.1 check digit — *which* Lokationsbündelstruktur (EDI@Energy Codeliste v1.0) |
| `LokationsbuendelObjektcode` | 13 decimal digits, §8.1 check digit — *where in it* an object sits |
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
        // When the v202701 series ships, add one arm:
        // Some("202701") => handle_v202701(serde_json::from_str::<v202701::Rechnung>(json)?),
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
- Keeps the branch a branch. `AnyBo` is the sum type over the *Geschäftsobjekte*, for a payload whose `_typ` is unknown until it is read — it is not a version abstraction, and there is deliberately no `AnyVersion`: two schema series have different field sets, so anything unifying them would have to erase the difference that made the dispatch necessary

See [Schema Versioning](https://hupe1980.github.io/rubo4e/docs/versioning/) for the full upgrade workflow.

---

## Convenience API

### Extension traits — flatten `Option<Com>` to `Option<Decimal>`

```rust
use rubo4e::prelude::*;  // brings BetragExt, MengeExt, PreisExt into scope

// Replaces the `.as_ref().and_then(|b| b.wert)` chain.
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
| `Zeitraum`'s **instant** pair (all four boundary fields) | `[start, end)` |
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

## Time Series and Units

BO4E carries readings over time in two shapes. A `Zeitreihenwert` on a `Lastgang`
or `Zeitreihe` is a value **over** an interval; a `Messwert` on a `Zaehlwerk` is
the meter's cumulative state **at** an instant. The first you sum or integrate,
the second you difference.

### Interval series — `Bo4eTimeSeries`

Nothing in the schema requires the entries to be sorted, contiguous, disjoint, or
the length the `Lastgang` declares. `audit()` walks them once and reports:

```rust
use rubo4e::timeseries::Bo4eTimeSeries;

let report = lg.audit();                 // against the span the entries cover
let report = lg.audit_over(start..end);  // …or the period it was meant to cover

report.gaps;           // stretches nothing covers
report.overlaps;       // …and stretches more than one entry covers
report.wrong_length;   // indices whose length is not zeitIntervallLaenge
report.unplaced;       // entries with no resolvable interval, each with a reason
report.unusable;       // indices whose status is FEHLT / NICHT_VERWENDBAR

report.is_complete();  // the timeline is covered exactly once
report.is_usable();    // …and every entry carries a usable value

lg.sum();              // None — messgroesse is KW, and adding kW is meaningless
lg.integrate();        // Some(450) — Σ value × interval_hours
lg.integrated_unit();  // Some(Mengeneinheit::Kwh)
```

`is_complete()` is a claim about the timeline, not the readings: a `FEHLT` entry
still occupies its slot. BO4E requires none of these properties, so nothing here
is wired into `.validate()`.

### One reading shape for all three — `Bo4eIntervals`

`Lastgang`, `Zeitreihe` and `Energiemenge` put a value on a stretch of time in
three shapes that look nothing alike — two of them a `Vec<Zeitreihenwert>` whose
unit lives on the enclosing BO, the third a single `Menge` over a `Zeitraum`.
`IntervalReading` is the one mapping, and it goes both ways:

```rust
use rubo4e::timeseries::{Bo4eIntervals, IntervalReading};

for r in lastgang.intervals() {
    r.range;       // [start, end) — half-open, so quarter-hours abut
    r.wert;        // Option<Decimal>
    r.einheit;     // lifted off the enclosing BO
    r.status;      // Abgelesen / Ersatzwert / Fehlt …
    r.energy();    // 400 kW over a quarter-hour → Some((100, Kwh))
}

// A power series and an energy series answer in the same unit.
lastgang.total_energy();      // Some((400, Mengeneinheit::Kwh))  — kW × hours
zeitreihe.total_energy();     // Some((400, Mengeneinheit::Kwh))  — already kWh
energiemenge.total_energy();  // Some((400, Mengeneinheit::Kwh))  — one interval

// …and back out again.
let zr = Zeitreihe::from_intervals(readings);
let lg = Lastgang::from_intervals(quarter_hour, readings);   // the required field, stated
```

Unusable readings are skipped rather than counted as zero — a `FEHLT` slot
carrying `0` is an absence — and `audit()` is where the gap they leave is
reported.

### Register series — `Zaehlwerk`

BO4E states the formula on `wandlerfaktor` itself: *"Mit diesem Faktor wird eine
Zählerstandsdifferenz multipliziert, um zum eigentlichen Verbrauch im Zeitraum zu
kommen."*

```rust
let register = Zaehlwerk {
    vorkommastelle: Some(6),        // a six-digit display
    wandlerfaktor: Some(dec!(40)),  // an indirectly-measuring meter
    ..Default::default()
};

register.consumption_between(dec!(1_000), dec!(1_050));  // Ok(2_000) — 50 × 40
register.consumption_between(dec!(999_998), dec!(12));   // Ok(560)   — 14 × 40
register.total_consumption();                            // the whole series
```

The wrap-around is the trap: `999998 → 000012` is 14 register steps, not
`−999 986`, and `vorkommastelle` is what BO4E gives you to know it.
`total_consumption()` refuses rather than guessing on a meter exchange
(`Z78_GERAETEWECHSEL`), a fall no register width explains, or a reading in a unit
that does not convert.

### `Zeitraum`'s third mode: an instant range

A quarter-hourly `Zeitreihenwert` states all four boundary fields — BO4E's
*"Startzeitpunkt (Datum und Uhrzeit) bis Endzeitpunkt"*. It is half-open,
`[start, end)`: `startuhrzeit` is *"inklusiv"*, `enduhrzeit` *"exklusiv"* — the
opposite of the date pair on the same struct.

```rust
let slot = Zeitraum::from_instants(start, start + Duration::minutes(15));

slot.as_instant_range();     // Option<Result<Range<OffsetDateTime>, _>>
slot.instant_duration();     // Some(Ok(15 minutes))
slot.contains_instant(t);    // [start, end)
slot.is_instant_range();     // does this value state all four fields?
```

Route on `is_instant_range()`: the date accessors read the date pair and only
that, so `whole_days()` on a 15-minute slot is `Some(1)`. A time of day with no
UTC offset is a wall-clock reading, not a moment, so `start_instant()` returns
`ZeitpunktError::MissingOffset` rather than guessing. `.validate()` enforces the
matching rule — with all four fields present, the start instant must be strictly
before the end.

### Units have dimensions

`Mengeneinheit` is one flat enum over energies, powers, a volume, eleven
durations, a percentage and a frequency. `rubo4e::units` says which may be added
and which convert:

```rust
Mengeneinheit::Kwh.dimension();                             // Some(Dimension::Energy)
Mengeneinheit::Mwh.conversion_factor(Mengeneinheit::Kwh);   // Some(1000)
Mengeneinheit::Kwh.conversion_factor(Mengeneinheit::Kw);    // None — another dimension
Mengeneinheit::ViertelStunde.exact_duration();              // Some(15 minutes)
Mengeneinheit::Monat.exact_duration();                      // None — no fixed length

menge.convert_to(Mengeneinheit::Kwh);                       // through the base unit
menge.energy_over(Duration::minutes(15));                   // 400 KW → 100 KWH
menge.as_duration();                                        // reads zeitIntervallLaenge
```

`MONAT` / `QUARTAL` / `HALBJAHR` / `JAHR` have no factor and no duration — the
same call `iso8601_duration` makes about `P1Y`. `is_extensive()` separates what
may be summed over a period from what may not, which is what makes `sum()` and
`integrate()` mean different things: for a stated unit, exactly one of them
answers.

See [Time Series & Units](https://hupe1980.github.io/rubo4e/docs/timeseries/).

---

## Lokationsbündelstrukturen

BO4E `v202607.1.0` defines **no** `Lokationsbuendel` Geschäftsobjekt, and `BoTyp`
has no `LOKATIONSBUENDEL` member. The bundle is a `Lokationszuordnung` plus two
13-digit BDEW codes: `lokationsbuendelcode` says *which* structure a Netzanschluss
has, and `lokationsbuendelObjektcode` on each participant says *where in it* that
object sits.

`rubo4e` ships EDI@Energy's **"Codeliste der Lokationsbündelstrukturen"** (BDEW
v1.0, applicable from 1 October 2024) as static data — 15 structures, 27 object
codes — and reads a decoded bundle through it:

```rust
use rubo4e::lokationsbuendel::{Flussrichtung, LokationsbuendelExt, LokationsbuendelObjekt, Objekttyp};

// An object code is a complete coordinate: type, direction, level.
let rolle = technische_ressource.objektrolle().unwrap();
assert_eq!(rolle.objekttyp, Objekttyp::TechnischeRessource);
assert_eq!(rolle.richtung, Some(Flussrichtung::Verbrauch));   // a § 14a SteuVE
assert_eq!(rolle.ebene, 1);

// A view over the Lokationszuordnung — not a new Geschäftsobjekt, and it
// serialises as nothing.
let buendel = zuordnung.buendel();
buendel.verbrauchs_ressourcen();      // heat pumps, wallboxes — not PV, not a battery
buendel.objekte_auf_ebene(2);         // everything hinterschaltet

// …checked against the structure it declares.
let report = zuordnung.audit_buendel();
report.is_conformant();               // false → report.befunde says why
```

`audit_buendel()` reports unknown or malformed codes, an object filed under the
wrong type, a code the declared structure does not use, and every cardinality —
including "exactly one Marktlokation" met by zero. Like `Bo4eTimeSeries::audit`
it is a data-quality report, not `.validate()`.

See [Lokationsbündel](https://hupe1980.github.io/rubo4e/docs/lokationsbuendel/).

---

## Namespaced `ZusatzAttribut`s

BO4E gives every BO and COM a `zusatzAttribute` list for what the standard has no
field for, and says nothing about how two systems writing into it stay out of each
other's way. `"id"` written by a market-communication layer and by a household
model is *one* entry, and the second write wins.

```rust
use rubo4e::zusatz_attribut::{Namespace, ZusatzAttributeExt};

sr.set_zusatz_attribut_in(&Namespace::HEMS, "eebus-ski", ski);
sr.set_zusatz_attribut_in(&Namespace::MAKO, "vorgangsnummer", "V-2026-0001");

sr.zusatz_attribut_str_in(&Namespace::HEMS, "eebus-ski");   // Some(ski)
sr.zusatz_attribut_namespaces();                            // ["hems", "mako"]
sr.remove_zusatz_attribute_in(&Namespace::HEMS);            // strip before handing on

// Typed values, so a code list BO4E has not published stays a type in your crate.
tr.set_zusatz_attribut_as_in(&Namespace::HEMS, "steuerungsvariante", &Steuerungsvariante::Ems)?;
let v: Steuerungsvariante = tr.zusatz_attribut_as_in(&Namespace::HEMS, "steuerungsvariante").unwrap()?;
```

The wire form is the flat BO4E name — `{"name": "hems:eebus-ski", "wert": "…"}` —
so any BO4E reader still sees an ordinary `ZusatzAttribut`, and a foreign prefix
round-trips untouched. `mako`, `hems`, `edmd` and `mabis` are registered;
`Namespace::new` takes any well-formed prefix. `AttributKey<T>` pins a key **and**
its value type as one `const` both sides import, and
`zusatz_attribut::well_known` holds the ones this crate registers.

`ZusatzAttributeExt` is on every BO4E type that declares the field —
`ZusatzAttribut` itself being the one that does not.

`rubo4e` supplies the mechanism, not the values: a `Steuerungsvariante` enum here
would invent a code list the market has not published. See
[Beyond the Schema](https://hupe1980.github.io/rubo4e/docs/beyond-the-schema/) for
what BO4E does and does not model.

---

## Beyond the schema: when a market rule outruns BO4E

The market rules keep moving; BO4E carries only what fits an existing
Geschäftsobjekt. `rubo4e` adds what reads a BO4E field — and says where BO4E
already has what you were about to add.

**A generated enum is never forked.** A value added to one emits a wire string
every other BO4E implementation decodes as `Unknown`, and which this crate's own
`ensure_known_enums()` then rejects. The answers, in order: BO4E already has it
elsewhere; the state is readable from the fields it does have; or it rides in a
registered `ZusatzAttribut` key.

```rust
use rubo4e::convenience::Aggregationszustaendigkeit;
use rubo4e::identifiers::{Zaehlpunkt, Zaehlpunktart};

// The Bilanzierungsgebiet EIC BO4E leaves as a String, checked as a Y-EIC (Area)
// — which is what tells it from a Bilanzkreis (`11X…`).
malo.bilanzierungsgebiet_checked();          // Some(Ok(BilanzierungsgebietId))

// A Zählpunkt (eMob) is *not* a MeLo-ID (BK6-20-160 §1.6.2), and cannot become one.
let zp = Zaehlpunkt::new(Zaehlpunktart::NetzgangzeitreiheEmob, zpb);
assert_eq!(zp.as_melo_id(), None);

// "Ruhende" Aggregationsverantwortung: an absent field plus Modell 2, not a
// `RUHEND` value no other implementation would read.
bilanzierung.aggregation_ruht();
bilanzierung.aggregationszustaendigkeit();   // Uenb | Vnb | Ruhend | Unbekannt
```

Every addition passes one test: **does it read, type, or guard a value that
arrives in a BO4E payload?** A domain aggregate of another standard does not — a
Bilanzierungsgebiet's Stammdaten read no BO4E field, so they are not modelled
here.

Three things that look missing from BO4E, and are not:

| Looks missing | Actually |
|---|---|
| `Zeitreihentyp::Ngz` | `Zeitreihentyp` is chapter 1 of the BDEW *Codeliste der Zeitreihentypen* — the **Summen**zeitreihentypen of DE7111. `NGZ` is not a code there in any published version (1.1a 2012 … 1.1d 2021); it appears only inside the explanation of `NZR`. A Netzgangzeitreihe is an MSCONS **PID 13018** payload — in BO4E a `Lastgang` at a `Zaehlpunkt`. |
| `Verbrauchsart::EMobilitaetsladesaeule` | BO4E models the charging point on the technische Ressource: `EMobilitaetsart::EMobilitaetsladesaeule` and `TechnischeRessourceVerbrauchsart::EMobilitaet`. `Verbrauchsart` is the Kraft/Licht/Wärme categorisation. **Do not** use a `ZusatzAttribut` for this. |
| mandatory fields blocking a mobile MaLo | `Marktlokation` has **no** `required` field in the schema, and this crate's only cross-field rule is *at most one* Ortsangabe — a conflict rule, not a presence rule. A Modell-2 MaLo with no address validates. |

See [Beyond the Schema](https://hupe1980.github.io/rubo4e/docs/beyond-the-schema/).

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
counts every decimal read from a number — integers included, since Go writes a
whole amount as `119` — so you can tell which spelling your producers use. See
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
| `Zeitraum` | at least one temporal field; `startdatum` **on or before** `enddatum` (both bounds inclusive); with all four boundary fields, start instant **strictly before** end instant (the end is exclusive) |
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
let schema = <rubo4e::identifiers::MaloId as utoipa::PartialSchema>::schema();
```

Every identifier emits a pattern, a German description and a valid example, and
both generators emit the same three:

```json
{
  "type": "string",
  "pattern": "^[1-9][0-9]{10}$",
  "example": "41373559241",
  "description": "11-stellige BDEW Marktlokations-ID: Vergabestelle (1-3 DVGW, 4-9 BDEW) + 9 Ziffern + Prüfziffer nach dem Lok- und Waggon-Kennzeichnungsverfahren (BDEW §8.1)"
}
```

They come from `rubo4e::identifiers::schema`, one table both derives read — and
each example is checked against the type's own constructor, so it carries a valid
check digit. See [Ecosystem](https://hupe1980.github.io/rubo4e/docs/ecosystem/#identifier-schemas-come-from-one-table).

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
| [Lokationsbündel](https://hupe1980.github.io/rubo4e/docs/lokationsbuendel/) | The EDI@Energy codelist, the two BDEW codes, and `audit_buendel()` |
| [Beyond the Schema](https://hupe1980.github.io/rubo4e/docs/beyond-the-schema/) | What happens when a market rule outruns BO4E — the test, the placement, and BK6-20-160 Modell 2 worked through |
| [Serialization](https://hupe1980.github.io/rubo4e/docs/serialization/) | JSON output modes, extension data, hardened parsing, namespaced `ZusatzAttribut`s |
| [Validation](https://hupe1980.github.io/rubo4e/docs/validation/) | The three validation layers and `Validated<T>` |
| [Time Series & Units](https://hupe1980.github.io/rubo4e/docs/timeseries/) | Interval and register series, `Zeitraum`'s instant mode, unit dimensions |
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
