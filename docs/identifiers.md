# Identifier Types

`rubo4e` wraps every BO4E domain identifier in a validated newtype. This prevents
passing an invalid ID where a valid one is required — at compile time, not at runtime.

All identifier types implement:

```
Display, FromStr, TryFrom<&str>, AsRef<str>,
Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd,
Serialize, Deserialize (behind `serde` feature)
```

Construction is always fallible. There are no panicking constructors.

---

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

---

## MaloId — Marktlokations-ID

**Format:** 11 decimal digits  
**Additional rule:** Luhn-like checksum on digits 1–10; result must equal digit 11

```rust
let malo = MaloId::new("51238696780")?;  // Ok
let bad  = MaloId::new("51238696782");   // Err(InvalidChecksum)
let _    = MaloId::new("123");           // Err(InvalidLength { expected: 11, actual: 3 })
```

### Checksum Algorithm

Given digits $d_1 d_2 \ldots d_{11}$:

1. Multiply each digit $d_i$ ($i = 1 \ldots 10$) by its weight $w_i$:

   | Position | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 |
   |----------|---|---|---|---|---|---|---|---|---|----|
   | Weight   | 2 | 1 | 2 | 1 | 2 | 1 | 2 | 1 | 2 | 1  |

2. For each product $p = d_i \times w_i$: if $p \geq 10$, replace with $p - 9$.
3. Sum all (possibly reduced) products: $S = \sum_{i=1}^{10} \text{reduce}(d_i \times w_i)$.
4. Check digit: $c = (10 - (S \bmod 10)) \bmod 10$.
5. Valid if $c = d_{11}$.

### Utilities

```rust
// Build from a 10-digit base — check digit is computed and appended.
let malo = MaloId::from_base("5123869678")?;  // → MaloId("51238696780")

// Compute just the check digit without building the full identifier.
let c = MaloId::check_digit("5123869678")?;   // → 0u8
```

### Display / FromStr

```rust
let malo = MaloId::new("51238696780")?;
assert_eq!(malo.to_string(), "51238696780");
assert_eq!("51238696780".parse::<MaloId>()?, malo);
```

---

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

---

## NeloId — Netzlokations-ID

**Source:** BDEW "Identifikatoren in der Marktkommunikation" v1.2 (February 2025), §4  
**Format:** 11 characters  
**Structure:**
- Position 1: Codetyp `'E'` (Netzlokation, per BNetzA-Festlegung BK6-22-128)
- Positions 2–10: uppercase alphanumeric body `[A-Z0-9]`
- Position 11: numeric check digit `[0-9]`  
**Checksum:** ASCII-Verfahren (§8.2)

> **Note:** Do not confuse `NeloId` with `EicCode`. EIC codes (e.g. `10YDE-EON------1`,
> 16 chars) identify Bilanzierungsgebiete / Regelzonen and appear on
> `Marktlokation.marktgebiet`. NeLo-IDs identify the physical grid asset
> (transformer, line segment) in Redispatch 2.0.

### ASCII-Verfahren Check Digit

Given positions $p_1 \ldots p_{11}$ where letters map to their ASCII code (A=65…Z=90)
and digits map to their numeric value (0–9):

$$\text{check} = \Bigl(10 - \bigl((\textstyle\sum_{i \text{ odd}} p_i + 2 \cdot \sum_{i \text{ even}} p_i) \bmod 10\bigr)\Bigr) \bmod 10$$

```rust
let nelo = NeloId::new("E0000000019")?;  // Ok
let bad  = NeloId::new("10YDE-EON------1"); // Err — that's an EicCode, not a NeloId
let bad  = NeloId::new("9900000000001");   // Err — that's a MarktpartnerId

// Build from 10-char base (Codetyp + 9 alphanumeric):
let nelo = NeloId::from_base("E000000001")?;  // → NeloId("E0000000019")
let c    = NeloId::check_digit("E111111111")?; // → 7u8
```

---

## SrId — Steuerbare-Ressource-ID

**Source:** BDEW "Identifikatoren in der Marktkommunikation" v1.2 (February 2025), §6.3/§6.6  
**Format:** 11 characters  
**Structure:**
- Position 1: Codetyp `'C'` (Steuerbare Ressource)
- Positions 2–10: uppercase alphanumeric body `[A-Z0-9]`
- Position 11: numeric check digit `[0-9]`  
**Checksum:** ASCII-Verfahren (§8.2) — same algorithm as `NeloId` and `TrId`

```rust
let sr = SrId::new("C0000000003")?;
let sr = SrId::from_base("C000000000")?;  // → SrId("C0000000003")
let c  = SrId::check_digit("C000000000")?; // → 3u8
```

---

## TrId — Technische-Ressource-ID

**Source:** BDEW "Identifikatoren in der Marktkommunikation" v1.2 (February 2025), §6.2/§6.6  
**Format:** 11 characters  
**Structure:**
- Position 1: Codetyp `'D'` (Technische Ressource)
- Positions 2–10: uppercase alphanumeric body `[A-Z0-9]`
- Position 11: numeric check digit `[0-9]`  
**Checksum:** ASCII-Verfahren (§8.2) — same algorithm as `NeloId` and `SrId`

```rust
let tr = TrId::new("D0000000002")?;
let tr = TrId::from_base("D000000000")?;  // → TrId("D0000000002")
let c  = TrId::check_digit("D000000000")?; // → 2u8
```

---

## EicCode — Energy Identification Code

**Format:** 16 characters  
**Structure:** `CC T XXXXXXXXXX C` where:

- `CC` (positions 1–2): ISO 3166-1 alpha-2 country code or `X` prefix for supranational
- `T` (position 3): EIC type character — one of `A`, `T`, `V`, `W`, `X`, `Y`, `Z`
- `XXXXXXXXXX` (positions 4–15): alphanumeric participant code
- `C` (position 16): check character computed per ENTSO-E specification

### Check Character Algorithm

1. Assign numeric values: `A`=10, `B`=11, …, `Z`=35; `0`=0, …, `9`=9; `-`=36.
2. Compute a weighted sum of the first 15 characters using modulo 37.
3. Check character is the character whose value equals `37 - (sum mod 37)` if non-zero,
   or `A` (value 10) if the remainder is zero.

```rust
let eic = EicCode::new("10XDE-EON-NETZ---W")?;
```

> EIC codes can be looked up at [entsoe.eu EIC browser](https://www.entsoe.eu/data/energy-identification-codes-eic/).

---

## ObisCode — OBIS Identification Code

**Format:** `[A-B:]C.D[.E][*F]`  
**Structure:**

```
A = medium (1 = electricity, 7 = gas, …)  [optional]
B = channel (0 = total)                   [optional, requires A]
C = physical quantity (0 = general metering group per IEC 62056-61; 1 = active energy fwd, …)
D = measurement type
E = tariff                                [optional]
F = billing period (*F or &F; & normalised to *)  [optional]
```

`C = 0` is **permitted** — it identifies the general metering data group per IEC 62056-21
§5.4 and IEC 62056-61 §4.2 (status, date/time, administrative objects).

```rust
let obis = ObisCode::new("1-0:1.8.1")?;   // electricity, active energy forward, tariff 1
let obis = ObisCode::new("7-0:3.1.0")?;   // gas, volume
let obis = ObisCode::new("0-0:0.0.0")?;   // C=0 — general metering group
let bad  = ObisCode::new("not-an-obis");   // Err(InvalidFormat { … })

// F-separator normalisation — & is accepted and stored as *
assert_eq!(ObisCode::new("1.8.1&255")?, ObisCode::new("1.8.1*255")?);

// Structured accessors (F6)
assert_eq!(ObisCode::new("1-0:1.8.0*255")?.to_pia_string(),  "1-0:1.8.0");    // F stripped
assert_eq!(ObisCode::new("1-0:1.8.0*255")?.to_bo4e_string(), "1-0:1.8.0*255"); // F kept
```
---

## MarktpartnerId — Marktpartner-ID (BDEW-code)

**Format:** 13 decimal digits  
**Checksum:** none for BDEW/DVGW codes; GS1 GLNs carry an EAN-13 check digit but
`MarktpartnerId` does not validate it (BDEW and DVGW codes share the same 13-digit
format without being GLNs)

The 13-digit Marktpartner-ID is issued by three different authorities with distinct
EDIFACT encoding:

| Prefix | Issued by              | NAD DE3055 | UNB DE0007 |
|--------|------------------------|-----------|------------|
| `99…`  | BDEW (Strom)           | `"293"`   | `"500"`    |
| `98…`  | DVGW (Gas)             | `"332"`   | `"502"`    |
| other  | GS1 (GLN)              | `"9"`     | `"14"`     |

```rust
let mp = MarktpartnerId::new("9900743000009")?;
```

### EDIFACT Agency Code Helpers

```rust
let mp = MarktpartnerId::new("9900357000004")?;

// Classification
assert!(mp.is_bdew());   // prefix "99" — BDEW Strom
assert!(!mp.is_dvgw());  // prefix "98" — DVGW Gas
assert!(!mp.is_gln());   // everything else — GS1 GLN

// EDIFACT NAD segment: NAD+MS+<id>::293
assert_eq!(mp.nad_agency_code(), "293");

// EDIFACT UNB header: UNB+UNOC:3+<id>:500+...
assert_eq!(mp.unb_agency_code(), "500");
```

### Integer Conversion

Legacy systems sometimes store Marktpartner-IDs as integers. Two helpers bridge the gap:

```rust
let mp = MarktpartnerId::new("9900357000004")?;
assert_eq!(mp.to_i64(), 9_900_357_000_004_i64);  // infallible; 13-digit decimal always fits i64
```

For fields in generated structs that a partner serializes as JSON numbers:

```rust
// In your own Serde-annotated struct:
#[serde(with = "rubo4e::identifiers::marktpartner_id_as_i64")]
pub partner_id: MarktpartnerId,
// Serializes as: 9900357000004  (integer, not "9900357000004")
// Deserializes from: integer or string — both accepted
```

`marktpartner_id_as_i64` is also re-exported from `rubo4e::identifiers` directly.

---

## Serialization

All identifiers serialize as plain JSON strings (no wrapper object):

```json
{ "marktlokationsId": "51238696780" }
```

The `#[serde(transparent)]` equivalent is used — the newtype is invisible in the
serialized form.

---

## Using Identifiers as Map Keys

All identifiers implement `Hash + Eq`, so they work as `HashMap` and `BTreeMap` keys:

```rust
use std::collections::HashMap;
let mut map: HashMap<MaloId, Vertrag> = HashMap::new();
```
