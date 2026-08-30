+++
title = "Identifiers"
description = "Every BO4E market identifier as a validated newtype: MaLo-ID, MeLo-ID, the Zählpunktbezeichnung, EIC, OBIS, Marktpartner-ID, the Lokationsbündel codes, the Redispatch 2.0 resource IDs and the SEPA bank identifiers, with the check-digit procedures behind them."
weight = 20
+++

`rubo4e` wraps every BO4E domain identifier in a validated newtype. This prevents
passing an invalid ID where a valid one is required — at compile time, not at runtime.

All identifier types implement:

```
Display, FromStr, TryFrom<&str>, TryFrom<String>, Into<String>,
AsRef<str>, Borrow<str>, Deref<Target = str>,
Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd,
Serialize, Deserialize (behind `serde` feature)
```

Construction is always fallible. There are no panicking constructors, and
`Deserialize` routes through the same `new`, so a value that exists has been
validated.

Everything up to `Deref` is unconditional — no feature flag turns it off — because
that is the minimum an EDIFACT encoder or decoder needs.

## IdentifierError

All construction failures return `IdentifierError`:

```rust
#[derive(Debug, thiserror::Error)]
pub enum IdentifierError {
    #[error("invalid length: expected {expected}, got {actual}")]
    InvalidLength { expected: LengthExpectation, actual: usize },

    #[error("invalid character {character:?} at position {position}")]
    InvalidCharacter { position: usize, character: char },

    #[error("invalid checksum")]
    InvalidChecksum,

    #[error("invalid format: {description}")]
    InvalidFormat { description: Cow<'static, str> },
}
```

`LengthExpectation` encodes the accepted length contract:

```rust
pub enum LengthExpectation {
    Exact(usize),
    RangeInclusive { min: usize, max: usize },
}
```

`Cow<'static, str>` in `InvalidFormat` allows both zero-allocation static messages
and runtime-constructed strings that include the actual invalid data.

## BDEW check-digit procedures

Chapter 8 of the BDEW Anwendungshilfe **"Identifikatoren in der Marktkommunikation"**
v1.2 (7 February 2025) defines two procedures. They are the *same* arithmetic:

1. Map each character of the base to a number.
2. Sum the values at **odd** positions (1-indexed).
3. Sum the values at **even** positions and multiply by 2.
4. The check digit is the difference from (2) + (3) to the next multiple of 10.
   If the total is already a multiple of 10 the check digit is `0`.

$$\text{check} = \Bigl(10 - \bigl((\textstyle\sum_{i \text{ odd}} v_i + 2 \cdot \sum_{i \text{ even}} v_i) \bmod 10\bigr)\Bigr) \bmod 10$$

The two differ only in step 1:

| § | Name | Character mapping | Used by |
|---|------|-------------------|---------|
| 8.1 | Lok- und Waggon-Kennzeichnungsverfahren | digit → its value | `MaloId`, BDEW-/DVGW-Codenummern |
| 8.2 | ASCII-Verfahren | digit → its value; `A`–`Z` → ASCII code (65–90) | `NeloId`, `NebeId`, `CrId`, `SgId`, `SrId`, `TrId`, `PaketId` |

Because a digit maps identically under both, §8.1 is exactly §8.2 restricted to
numeric input. `rubo4e` implements the arithmetic once, in
`src/identifiers/checksum.rs`, pinned to both worked examples from the
specification.

> **Note on strength.** This procedure catches every adjacent transposition of two
> distinct characters, but it misses one class of single-character typo: a change of
> exactly ±5 at an even position, since that position carries weight 2 and
> `2 · 5 ≡ 0 (mod 10)`. For instance `41373559241` and `46373559241` share a check
> digit. Treat check digits as typo guards, not as proof that an ID was issued.

## MaloId — Marktlokations-ID

**Source:** BDEW §3; BNetzA-Festlegung BK6-16-200 / BK7-16-142  
**Format:** 11 decimal digits  
**Structure:**

| Position | Content | Character set |
|----------|---------|---------------|
| 1 | Vergabestelle — `1`–`3` DVGW, `4`–`9` BDEW | `[1-9]` |
| 2–10 | Automatically assigned body | `[0-9]` |
| 11 | Check digit (§8.1) | `[0-9]` |

**Checksum:** §8.1 Lok- und Waggon-Kennzeichnungsverfahren

The Vergabestelle digit says nothing about the commodity — a MaLo-ID from either
office can identify an electricity Marktlokation, a gas Marktlokation, or a Tranche.

```rust
let malo = MaloId::new("41373559241")?;   // Ok — worked example from BDEW §8.1
let bad  = MaloId::new("41373559242");    // Err(InvalidChecksum)
let bad  = MaloId::new("01373559241");    // Err(InvalidFormat) — Vergabestelle 0 unassigned
let bad  = MaloId::new("123");            // Err(InvalidLength { expected: Exact(11), actual: 3 })
```

### Worked example (BDEW §8.1)

```text
MaLo-ID base:  4 1 3 7 3 5 5 9 2 4
a) odd:        4 + 3 + 3 + 5 + 2       = 17
b) even:      (1 + 7 + 5 + 9 + 4) * 2  = 52
c) sum:        17 + 52                 = 69
d) check:      70 - 69                 = 1   →  41373559241
```

### Utilities

```rust
// Build from a 10-digit base — the check digit is computed and appended.
let malo = MaloId::from_base("4137355924")?;   // → MaloId("41373559241")

// Compute just the check digit.
let c = MaloId::check_digit("4137355924")?;    // → 1u8

// Inspect the parts.
assert_eq!(malo.base(), "4137355924");
assert_eq!(malo.vergabestelle(), MaloVergabestelle::Bdew);
```

### Display / FromStr

```rust
let malo = MaloId::new("41373559241")?;
assert_eq!(malo.to_string(), "41373559241");
assert_eq!("41373559241".parse::<MaloId>()?, malo);
```

## MeloId — Messlokations-ID

**Format:** 33 characters  
**Structure:**
- Positions 1–2: ISO 3166-1 alpha-2 country code, uppercase (e.g. `DE`, `AT`, `CH`)
- Positions 3–33: alphanumeric body `[A–Za–z0–9]`  
**Checksum:** none

```rust
let melo = MeloId::new("DE0000123456789012345678901234561")?;
```

### Utilities

```rust
let melo = MeloId::new("DE0000123456789012345678901234561")?;
assert_eq!(melo.country_code(), "DE");  // always 2-char slice; zero-copy
assert!(melo.is_german());              // country_code() == "DE"

let at = MeloId::new("AT0000123456789012345678901234561")?;
assert!(!at.is_german());
```

## Zaehlpunktbezeichnung — a Zählpunkt that is not a Messlokation

**Source:** MaBiS (BK6-24-174, Anlage 3); BDEW Anwendungshilfe zu BK6-20-160 §1.6.2  
**Format:** the same 33 characters as a [`MeloId`](#meloid-messlokations-id)  
**Checksum:** none defined

BO4E calls `Messlokation.messlokationsId` *"Die Messlokations-Identifikation; Das
ist die frühere Zählpunktbezeichnung"* — one grammar, and one assumed meaning.
MaBiS names several points with a Zählpunktbezeichnung that are not
Messlokationen, and the Anwendungshilfe is explicit for the e-mobility case:

> Für den Zählpunkt (eMob) wird eine ID (Zählpunktbezeichnung) vergeben.
> **Hinweis: Für den Zählpunkt (eMob) wird nicht die ID der Messlokation
> (Zählpunktbezeichnung) verwendet.**

Same validation, deliberately different type — the pattern
[`Lokationsbuendelcode` and `LokationsbuendelObjektcode`](#lokationsbuendelcode-and-lokationsbuendelobjektcode)
follow for the same reason:

```rust
use rubo4e::identifiers::{MeloId, Zaehlpunktbezeichnung};

let zpb = Zaehlpunktbezeichnung::new("DE0000000000000000000000000000042").unwrap();
assert_eq!(zpb.country_code(), "DE");

// A MeLo-ID *is* a Zählpunktbezeichnung — BO4E says so on the field itself.
let melo = MeloId::new("DE0000000000000000000000000000001").unwrap();
let widened = Zaehlpunktbezeichnung::from(melo);

// The reverse is a claim, not a fact, so it is spelled out.
let narrowed = zpb.into_melo_id();
```

The one grammar is implemented once and shared by both types, so the two cannot
drift. `Zaehlpunktart` sits beside them the way `EicType` sits beside `EicCode` —
with one difference: an `EicType` is *read out of* its code (position 3), whereas
a Zählpunktart cannot be — a Zählpunkt (eMob) and a MeLo-ID are indistinguishable
as strings, so it has to be carried. [`Zaehlpunkt`](@/docs/beyond-the-schema.md#the-zahlpunkt-that-is-not-a-messlokation)
is what says *which* kind of Zählpunkt a given one names, and refuses
`as_melo_id()` for every kind that is not a Messlokation.

## The §8.2 ASCII-Verfahren family

Seven identifiers share one structure: a fixed Codetyp, an uppercase-alphanumeric
body, and a numeric check digit — 11 characters in total.

| Type | Codetyp | Object | § | Introduced by |
|------|---------|--------|---|---------------|
| `NeloId` | `E` | Netzlokation | 4 | BK6-22-128 |
| `NebeId` | `F` | Netzbereich | 5 | BK6-22-300, BK8-22/010-A |
| `CrId` | `A` | Cluster Ressource | 6.5 | Redispatch 2.0 |
| `SgId` | `B` | Steuergruppe | 6.4 | Redispatch 2.0 |
| `SrId` | `C` | Steuerbare Ressource | 6.3 | Redispatch 2.0 |
| `TrId` | `D` | Technische Ressource | 6.2 | Redispatch 2.0 |
| `PaketId` | `P9` | Paket (Netzbetreiberwechsel) | 7 | — |

The Paket-ID is the one variation: §7.2 fixes **two** leading characters — `P` for
"Paket" and `9` for BDEW/Strom — leaving an 8-character body.

Every type in the family exposes the same API:

```rust
// Validate a complete identifier.
let nelo = NeloId::new("E0000000019")?;

// Or build from the base and let the check digit be derived.
let nelo = NeloId::from_base("E000000001")?;   // → NeloId("E0000000019")
let c    = NeloId::check_digit("E111111111")?; // → 7u8

// Inspect the parts.
assert_eq!(nelo.base(), "E000000001");
assert_eq!(NeloId::CODETYP, "E");

// Same shape for every other member.
let tr    = TrId::from_base("D000000001")?;    // → TrId("D0000000010")
let sr    = SrId::from_base("C000000001")?;    // → SrId("C0000000011")
let nebe  = NebeId::from_base("F000000001")?;  // → NebeId("F0000000018")
let paket = PaketId::from_base("P900000001")?; // → PaketId("P9000000010")
```

The Codetyp is enforced, so the types cannot be confused with one another or with
identifiers from a different family:

```rust
assert!(NeloId::new("D0000000010").is_err());      // that is a TrId
assert!(NeloId::new("10YDE-EON------1").is_err()); // that is an EicCode
assert!(PaketId::from_base("P800000001").is_err()); // §7.2 fixes position 2 to '9'
```

> **Note:** Do not confuse `NeloId` with `EicCode`. EIC codes (16 chars, e.g.
> `10YDE-EON------1`) identify Bilanzierungsgebiete / Regelzonen and appear on
> `Marktlokation.marktgebiet`. NeLo-IDs identify the grid location itself.

## EicCode — Energy Identification Code

**Source:** ENTSO-E EIC Reference Manual  
**Format:** 16 characters  
**Structure:**

| Position | Content |
|----------|---------|
| 1–2 | Local Issuing Office (LIO) identifier |
| 3 | EIC type character — one of `A`, `T`, `V`, `W`, `X`, `Y`, `Z` |
| 4–15 | LIO-assigned body, `[A-Z0-9]` or `-` as right padding |
| 16 | Check character |

### Check-character algorithm

1. Map each character to a value: `0`–`9` → 0–9, `A`–`Z` → 10–35, `-` → 36.
2. Weight position *i* (0-indexed, over the first 15 characters) by `16 - i`,
   i.e. weights 16, 15, …, 2, and sum the products.
3. `check_value = 36 - ((sum - 1) mod 37)`.
4. Map `check_value` back to a character. A result of 36 would be `-`, which
   ENTSO-E prohibits as a check character; `compute_check_char` returns `None`
   for such prefixes.

### Object type (position 3)

Position 3 encodes the ENTSO-E **object type**. There are exactly seven, and
`EicCode` rejects anything else at construction, so `eic_type()` is total.

| Char | `EicType` | Meaning |
|------|-----------|---------|
| `A` | `Substation` | Substation |
| `T` | `Tieline` | Tie line between two areas |
| `V` | `Location` | Physical location |
| `W` | `ResourceObject` | Resource object (generation/consumption unit) |
| `X` | `Party` | Market participant — **including Bilanzkreise** |
| `Y` | `Area` | Area or domain — control areas, bidding zones, Bilanzierungsgebiete |
| `Z` | `MeasurementPoint` | Measurement point |

The German market relies on this distinction: BDEW/ECS issue `11X…` for
Bilanzkreise and `11Y…` for Bilanzierungsgebiete.

```rust
// Real ENTSO-E control-area code.
let eic = EicCode::new("10YDE-EON------1")?;  // TenneT TSO
assert_eq!(eic.type_char(), 'Y');
assert_eq!(eic.eic_type(), EicType::Area);

// A Bilanzkreis is a market party, not an area.
assert_eq!(EicCode::new("11XSUEDWESTSTRO8")?.eic_type(), EicType::Party);

// Compute the check character for a 15-character prefix.
assert_eq!(EicCode::compute_check_char("10XDE-EON-NETZ-"), Some('C'));

// Or build the whole code from a prefix.
assert_eq!(EicCode::new_from_prefix("10YDE-EON------")?.as_ref(), "10YDE-EON------1");
```

The implementation is pinned to published ENTSO-E codes (`10YDE-EON------1`,
`10YDE-RWENET---I`, `10YDE-VE-------2`, `10YDE-ENBW-----N`, `10Y1001A1001A82H`)
so an algorithm regression fails immediately.

> EIC codes can be looked up in the [ENTSO-E EIC browser](https://www.entsoe.eu/data/energy-identification-codes-eic/).

## ObisCode — OBIS Identification Code

**Format:** `[A-B:]C.D[.E][*F]`  
**Structure:**

```
A = medium (1 = electricity, 6 = heat, 7 = gas, 8 = water)  [optional]
B = channel (0 = total)                   [optional, requires A]
C = physical quantity (0 = general metering group per IEC 62056-61; 1 = active energy fwd, …)
D = measurement type
E = tariff                                [optional]
F = billing period (*F or &F; & canonicalised to *)  [optional]
```

`C = 0` is **permitted** — it identifies the general metering data group per IEC 62056-21
§5.4 and IEC 62056-61 §4.2 (status, date/time, administrative objects).

Every value group is a **single octet** (0–255) per IEC 62056-61 §4, so `256` and
above are rejected rather than silently accepted as a wider integer.

### Canonical form

The input is parsed once at construction and stored canonically, so two spellings
of the same code are equal, hash alike, and render identically:

- `&` becomes `*`;
- redundant leading zeros are dropped (`01.08` → `1.8`).

Because the components are stored, `components()` neither re-parses nor allocates.

```rust
let obis = ObisCode::new("1-0:1.8.1")?;   // electricity, active energy forward, tariff 1
let obis = ObisCode::new("7-0:3.1.0")?;   // gas, volume
let obis = ObisCode::new("0-0:0.0.0")?;   // C=0 — general metering group
let bad  = ObisCode::new("not-an-obis");  // Err(InvalidFormat { … })

// Canonicalisation
assert_eq!(ObisCode::new("1.8.1&255")?,      ObisCode::new("1.8.1*255")?);
assert_eq!(ObisCode::new("01-00:01.08.00")?, ObisCode::new("1-0:1.8.0")?);
assert_eq!(ObisCode::new("01-00:01.08.00")?.as_str(), "1-0:1.8.0");

// Value groups are octets
assert!(ObisCode::new("1-0:1.8.256").is_err());

// Stored components — no re-parse, no allocation
let parts = ObisCode::new("1-0:1.8.0*255")?.components();
assert_eq!((parts.a, parts.b, parts.c, parts.d, parts.e, parts.f),
           (Some(1), Some(0), 1, 8, Some(0), Some(255)));

// PIA item-number form drops F
assert_eq!(ObisCode::new("1-0:1.8.0*255")?.to_pia_string(), "1-0:1.8.0");
```

## MarktpartnerId — Marktpartner-ID (MP-ID)

**Source:** BDEW §2  
**Format:** 13 decimal digits  
**Checksum:** not enforced at construction — see below

An MP-ID identifies a market participant in one role and one commodity. It is a
BDEW-Codenummer (electricity), a DVGW-Codenummer (gas), or a GS1 Global Location
Number.

| Prefix | Issued by | NAD DE3055 | UNB DE0007 |
|--------|-----------|-----------|------------|
| `99…`  | BDEW (Strom) | `"293"` | `"500"` |
| `98…`  | DVGW (Gas)   | `"332"` | `"502"` |
| other  | GS1 (GLN)    | `"9"`   | `"14"`  |

### Why the check digit is not enforced by `new`

BDEW §2.3 specifies **two different** procedures depending on origin: BDEW- and
DVGW-Codenummern use §8.1, while a GS1-issued GLN uses the GS1/EAN-13 procedure
(weights `1, 3, 1, 3, …`). The two disagree, and the leading digits do not reliably
separate them — codes predating the `98`/`99` convention are still in circulation
and validate under §8.1 despite other prefixes. DB Energie's published Bahnstrom
MP-IDs, for example, all begin `19` and all satisfy §8.1.

Enforcing either procedure by default would reject valid production identifiers, so
`new` validates only what is unambiguous. Check digits are available on demand:

```rust
let mp = MarktpartnerId::new("9900357000003")?;

assert!(mp.has_valid_bdew_check_digit());   // §8.1
assert!(!mp.has_valid_gln_check_digit());   // GS1/EAN-13

// Accept only if one of the two procedures matches — useful at an ingest boundary.
assert!(MarktpartnerId::new_checked("9900357000003").is_ok());  // §8.1
assert!(MarktpartnerId::new_checked("4006381333931").is_ok());  // EAN-13
assert!(MarktpartnerId::new_checked("9900357000000").is_err()); // neither

// Build from a 12-digit base using §8.1.
assert_eq!(MarktpartnerId::from_base("990035700000")?.as_ref(), "9900357000003");
```

### Classification and EDIFACT agency codes

```rust
let mp = MarktpartnerId::new("9900357000003")?;

assert_eq!(mp.authority(), MpIdAuthority::Bdew);
assert!(mp.is_bdew());

// EDIFACT NAD segment: NAD+MS+<id>::293
assert_eq!(mp.nad_agency_code(), "293");

// EDIFACT UNB header: UNB+UNOC:3+<id>:500+...
assert_eq!(mp.unb_agency_code(), "500");

// The codes also live on the enum, for use without a concrete ID.
assert_eq!(MpIdAuthority::Dvgw.nad_agency_code(), "332");
```

### Integer conversion

Some BDEW REST APIs represent Rollencodenummern as JSON integers.

```rust
let mp = MarktpartnerId::new("9900357000003")?;
assert_eq!(mp.to_i64(), 9_900_357_000_003_i64);  // infallible; 13 digits always fit
```

For a field that a partner serializes as a number:

```rust
#[serde(with = "rubo4e::identifiers::marktpartner_id_as_i64")]
pub partner_id: MarktpartnerId,
// Serializes as: 9900357000003  (integer, not "9900357000003")
// Deserializes from: integer or string — both accepted, integers zero-padded to 13 digits
```

## Lokationsbuendelcode and LokationsbuendelObjektcode

**Source:** EDI@Energy / BDEW, *"Codeliste der Lokationsbündelstrukturen"* v1.0
(31 March 2023, applicable from 1 October 2024)  
**Format:** 13 decimal digits  
**Checksum:** §8.1 check digit at position 13 — the same arithmetic as a MaLo-ID

The two codes that carry a Lokationsbündelstruktur: *which* structure a
Netzanschluss has, and *where in it* one object sits.

```rust
use rubo4e::identifiers::{Lokationsbuendelcode, LokationsbuendelObjektcode};

let struktur = Lokationsbuendelcode::new("9992000000026").unwrap();
let objekt = LokationsbuendelObjektcode::new("9992000001016").unwrap();

// The codelist prints codes grouped 4-5-3-1 for legibility; the wire never has
// the spaces, and neither does the stored value.
assert_eq!(struktur.grouped(), "9992 00000 002 6");
assert_eq!(struktur.as_str(), "9992000000026");

// Derived rather than typed by hand.
assert_eq!(LokationsbuendelObjektcode::from_base("999200000101").unwrap(), objekt);
assert_eq!(LokationsbuendelObjektcode::check_digit("999200000101").unwrap(), 6);
```

Two things separate these from [`MarktpartnerId`](#marktpartnerid-marktpartner-id-mp-id),
the other 13-digit BDEW code:

- **The check digit is enforced.** An MP-ID may carry either the §8.1 digit or a
  GS1/EAN-13 one and the leading digits do not reliably separate them, so
  `MarktpartnerId` checks neither. A Lokationsbündel code has no such ambiguity:
  all 42 published codes verify under §8.1.
- **They are two types.** The validation is identical, but a structure code where
  an object code belongs describes a bundle that does not exist, and no checksum
  notices. The type does.

A well-formed code outside the published list still constructs — BDEW may extend
the list, and the document itself says complex structures are agreed bilaterally
rather than coded. Resolving one to its *meaning* is
[`rubo4e::lokationsbuendel`](@/docs/lokationsbuendel.md), which returns `None`
there.

Neither type is used on a generated struct field: `lokationsbuendelcode` and
`lokationsbuendelObjektcode` stay `Option<String>`, for the same reason
[`Zahlungsinformation.iban`](#these-two-fields-stay-string-on-the-generated-struct) does. The checked
accessors cost the caller one `Result` instead of costing them the whole BO.

## BilanzkreisId and BilanzierungsgebietId

**Source:** ENTSO-E EIC Reference Manual v5.5 + MaBiS BK6-06-009 + GaBi Gas BK7-14-020  
**Format:** 16-character EIC code  
**Checksum:** ENTSO-E check character at position 16 (same algorithm as [`EicCode`])

Two roles in the German market share the EIC namespace and are told apart *only*
by the object-type character:

| Type | Object type | Role |
|------|-------------|------|
| `BilanzkreisId` | `X` — Party | Bilanzkreis (balance group) |
| `BilanzierungsgebietId` | `Y` — Area | Bilanzierungsgebiet (balancing area) |

A Bilanzkreis is held by a Bilanzkreisverantwortlicher — a market participant —
so ENTSO-E classifies it as a **party**. A Bilanzierungsgebiet is a grid area, so
it is an **area**. Each gets its own Rust type, which is what stops one being
passed where the other is expected.

Used in:
- **MaBiS** (BK6-06-009) — electricity balance-group settlement and area assignment
- **GaBi Gas** (BK7-14-020) — gas balance-group settlement
- EDIFACT `NAD`/`LOC` segments with DE3227 qualifier `Z01`/`Z02`

```rust
// Build from a 15-character prefix — the check character is computed for you.
let bk = BilanzkreisId::from_prefix("11XSUEDWESTSTRO")?;   // → "11XSUEDWESTSTRO8"
assert_eq!(bk.to_eic_code().eic_type(), EicType::Party);

// Widening to EicCode is infallible; narrowing back is checked.
let eic: EicCode = bk.clone().into();
assert_eq!(BilanzkreisId::try_from(eic)?, bk);

// The two types reject each other's codes.
let bg = BilanzierungsgebietId::new("11YN-0000-0001-Q")?;
assert!(BilanzkreisId::new("11YN-0000-0001-Q").is_err());
assert!(BilanzierungsgebietId::new("11XSUEDWESTSTRO8").is_err());

// A control-area EIC (10Y…) is an area, so it is never a Bilanzkreis.
assert!(BilanzkreisId::new("10YDE-EON------1").is_err());
```

`StandorteigenschaftenStrom.bilanzierungsgebiet_eic` is generated as a
`BilanzierungsgebietId`: the BO4E schema declares it a bare string but documents
it as "Die EIC-Nummer des Bilanzierungsgebietes", and all 645 entries in the TSOs'
published VNB-Bilanzierungsgebiete list carry object type `Y`.

`Bilanzierung.bilanzkreis` stays a plain `EicCode`. Electricity Bilanzkreise are
`11X…`, but the same field also carries gas Bilanzkreise, whose object type this
crate has not verified — narrowing it would turn an assumption into a hard
deserialization failure on real payloads. Callers who know their domain can opt in
with `BilanzkreisId::try_from(eic)`.

## AkivId — Aktivierungsidentifikator

**Source:** BDEW WiM AHB BK6-24-174 (§14a EnWG Modul 3, Redispatch 2.0 BK6-20-059)  
**Format:** 1–36 printable ASCII characters (`!` through `~`, no spaces or control chars)  
**Checksum:** none — opaque reference identifier

The Aktivierungsidentifikator uniquely identifies a single activation event of a
controllable resource (`SteuerbareRessource`). It appears in:
- BDEW UTILTS PID 55168 (`RFF+ACD` segment) — Verpflichtungsanfrage
- BDEW ORDERS/ORDRSP Steuerungsauftrag — activation acknowledgement

In practice, UUIDs (36 chars) are commonly used:

```rust
// UUID-format activation ID
let id = AkivId::new("550e8400-e29b-41d4-a716-446655440000").unwrap();

// Shorter form
let id = AkivId::new("AKIV-2026-00001").unwrap();

// Into<String> for ergonomic conversion
let s: String = id.into();
```

## TranchennummerId — Tranchennummer

**Source:** MABIS Bilanzkreisabrechnung PID 13003 (BK6-06-009)  
**Format:** 1–6 decimal digits (`[0-9]`), no leading zeros (except the literal `"0"`)  
**Range:** `0`–`999 999`  
**Checksum:** none

The Tranchennummer identifies tranches within a balance-group settlement period.
It appears in EDIFACT `RFF+TN:` (reference qualifier `TN` = Tranche Number).

```rust
let t = TranchennummerId::new("1").unwrap();
assert_eq!(t.value(), 1u32);

// Build from an integer
let t = TranchennummerId::from_value(42).unwrap();
assert_eq!(t.as_ref(), "42");

// Infallible numeric conversion
let n: u32 = t.into();  // From<TranchennummerId> for u32
assert_eq!(n, 42u32);

// Reject leading zeros (except "0")
assert!(TranchennummerId::new("007").is_err());
assert!(TranchennummerId::new("0").is_ok());

// Reject out-of-range values
assert!(TranchennummerId::from_value(1_000_000).is_err());
```

## Iban and Bic — SEPA bank identifiers

**Source:** ISO 13616 (IBAN), ISO 9362 (BIC)
**Checksum:** IBAN — ISO 7064 MOD-97-10. BIC — none defined by the standard.

`Zahlungsinformation.iban` and `.bic` are the two fields on a BO4E invoice that
money actually moves against, and the schema declares both as bare strings. An
IBAN's check digits catch **every** single-character error and **every**
transposition of adjacent characters, so leaving them unverified was the one
obvious hole in the identifier family.

```rust
use rubo4e::identifiers::{Bic, Iban};

// Grouping spaces and lowercase normalise away — a value pasted from a bank
// statement parses.
let iban = Iban::new("de89 3704 0044 0532 0130 00").unwrap();
assert_eq!(iban.as_ref(), "DE89370400440532013000");   // wire form
assert_eq!(iban.to_grouped_string(), "DE89 3704 0044 0532 0130 00"); // print form

// German IBANs split into their Bundesbank parts.
assert_eq!(iban.country_code(), "DE");
assert_eq!(iban.bankleitzahl(), Some("37040044"));
assert_eq!(iban.kontonummer(), Some("0532013000"));

// A transposed digit fails the MOD-97 check.
assert!(Iban::new("DE89370400440532013090").is_err());

let bic = Bic::new("GENODEF1S04").unwrap();
assert_eq!(bic.institution_code(), "GENO");
assert_eq!(bic.country_code(), "DE");
assert_eq!(bic.branch_code(), Some("S04"));
assert!(bic.is_passive());        // location code ending in 1
assert!(!bic.is_head_office());   // …and not the XXX / 8-char head office
```

Country-specific lengths are enforced for the codes in the ISO 13616 registry, so
a 21-character German IBAN is rejected here rather than by the bank. A country
the crate's table does not yet list is **not** rejected on length — the registry
grows, and a stale table should not refuse a valid IBAN — but its checksum is
still verified.

### These two fields stay `String` on the generated struct

Deliberately, and against the usual rule. `Zahlungsinformation` hangs off
`Rechnung` and nothing else, so typing `iban` as a validated newtype would mean a
**masked** IBAN — `DE89 **** **** 3000`, routine on an invoice — destroys the
entire `Rechnung`: line items, amounts, periods and all. That is a bad trade for
a field most consumers never read.

The check is one call away instead, and it hands you an error rather than
costing you the invoice:

```rust
let z: Zahlungsinformation = todo!();

match z.iban_checked() {
    None            => { /* no IBAN stated */ }
    Some(Ok(iban))  => { /* verified */ }
    Some(Err(e))    => { /* stated but invalid — masked, mistyped, truncated */ }
}
```

See [Semantic Field Typing](@/docs/generator.md#semantic-field-typing) for the
rule this is an exception to, and why.

## Serialization

All identifiers serialize as plain JSON strings (no wrapper object):

```json
{ "marktlokationsId": "51238696781" }
```

The `#[serde(transparent)]` equivalent is used — the newtype is invisible in the
serialized form.

## Using Identifiers as Map Keys

All identifiers implement `Hash + Eq + Ord`, so they key a `HashMap` and a
`BTreeMap` alike:

```rust
use std::collections::HashMap;
let mut map: HashMap<MaloId, Vertrag> = HashMap::new();
```

They also implement `Borrow<str>`, so a lookup does not have to construct — and
re-validate — an identifier just to ask whether one is in the map:

```rust
map.insert(MaloId::new("41373559241")?, vertrag);
assert!(map.contains_key("41373559241"));   // no MaloId built, no checksum re-run
```

`Borrow` carries a contract — the borrowed form must hash and compare exactly as
the owned one does, or that lookup returns `None` for a key that is present.
`tests/prelude_surface.rs` checks it for every identifier.

For `ObisCode` the key is the **canonical** spelling: look up `"1-0:1.8.0"`, not
`"01-00:01.08.00"`. Both build the same value, but only the canonical form is the
string it borrows as.
