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
    InvalidLength { expected: usize, actual: usize },

    #[error("invalid character '{character}' at position {position}")]
    InvalidCharacter { position: usize, character: char },

    #[error("invalid checksum")]
    InvalidChecksum,

    #[error("invalid format: {description}")]
    InvalidFormat { description: &'static str },
}
```

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

---

## NeloId — Netzlokations-ID

**Format:** 11 alphanumeric characters  
**Checksum:** none

```rust
let nelo = NeloId::new("NELO1234567")?;
```

---

## SrId — Steuerbare-Ressource-ID

**Format:** 11 decimal digits  
**Checksum:** same algorithm as `MaloId`

```rust
let sr = SrId::new("51238696780")?;
```

---

## TrId — Technische-Ressource-ID

**Format:** 11 decimal digits  
**Checksum:** same algorithm as `MaloId`

```rust
let tr = TrId::new("51238696780")?;
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

**Format:** `A-B:C.D.E` (with optional `*F` channel suffix)  
**Structure:** Media-Channel-Measurement.Type.Tariff groups

```
A = medium (1 = electricity, 7 = gas, …)
B = channel (0 = total)
C = physical quantity (1 = active energy forward, …)
D = measurement type
E = tariff
F = billing period (optional)
```

```rust
let obis = ObisCode::new("1-0:1.8.1")?;   // electricity, active energy forward, tariff 1
let obis = ObisCode::new("7-0:3.1.0")?;   // gas, volume
let bad  = ObisCode::new("not-an-obis");   // Err(InvalidFormat { … })
```

---

## MarktpartnerId — Marktpartner-ID (BDEW-code)

**Format:** 13 decimal digits  
**Checksum:** none (length + digit-only check)

```rust
let mp = MarktpartnerId::new("9900743000009")?;
```

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
