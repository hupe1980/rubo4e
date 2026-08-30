+++
title = "Lokationsbündel"
description = "There is no Lokationsbuendel Geschäftsobjekt in BO4E — there is a Lokationszuordnung and two BDEW codes. The published EDI@Energy codelist, shipped as data, and an audit that checks a bundle against the structure it declares."
weight = 25
+++

A German grid connection rarely holds one thing. Behind one Netzanschluss sit
Messlokationen, Marktlokationen, technische Ressourcen and — since § 14a EnWG —
steuerbare Verbrauchseinrichtungen, arranged in a shape that decides who bills
what. BDEW calls that shape a **Lokationsbündelstruktur** and exchanges it as a
code.

Two questions follow: where BO4E puts it, and what the code means.

---

## There is no `Lokationsbuendel` business object

Checked against `v202607.1.0`, the release this crate is generated from: there is **no** `Lokationsbuendel` schema, and `BoTyp`
has no `LOKATIONSBUENDEL` member. Looking for one and finding only
`Lokationszuordnung` is not a gap in `rubo4e` — it is the model.

The bundle is carried by one BO plus one field on each participant:

| Carries | Where | Type |
|---|---|---|
| the bundle itself | `Lokationszuordnung` | lists of MaLo / MeLo / NeLo / SR / TR |
| *which* structure | `Lokationszuordnung.lokationsbuendelcode` | `Lokationsbuendelcode` |
| *where in it* an object sits | `<BO>.lokationsbuendelObjektcode` | `LokationsbuendelObjektcode` |

The upstream schema says as much itself: `Lokationszuordnung` is documented
*"Modell für die Abbildung der Referenz auf die Lokationsbündelstruktur. Diese
gibt an welche Marktlokationen, Messlokationen, Netzlokationen,
technische/steuerbaren Ressourcen an einer Lokation vorhanden sind."*

`rubo4e` therefore gives the bundle a **name and an API** without giving it a new
schema. `Lokationsbuendel` is a borrowed view over a `Lokationszuordnung`; it
holds no data of its own and serialises as nothing, so nothing this crate writes
is a payload another BO4E implementation cannot read.

```rust
use rubo4e::current::Lokationszuordnung;
use rubo4e::lokationsbuendel::LokationsbuendelExt;

let zuordnung = Lokationszuordnung::default();
let buendel = zuordnung.buendel();     // a view, not a Geschäftsobjekt
assert!(buendel.is_empty());
```

---

## The two codes

Both are 13-digit BDEW Codenummern ending in a **§8.1 check digit** — the same
arithmetic as a `MaLo-ID`, described in
[Identifiers](@/docs/identifiers.md#bdew-check-digit-procedures). All 15 structure
codes and all 27 object codes the published list contains verify under it, and
`rubo4e` enforces it:

```rust
use rubo4e::identifiers::Lokationsbuendelcode;

let code = Lokationsbuendelcode::new("9992000000026").unwrap();

// The codelist prints codes grouped 4-5-3-1 for legibility; the wire never has
// the spaces, and neither does the stored value.
assert_eq!(code.grouped(), "9992 00000 002 6");
assert_eq!(code.as_str(), "9992000000026");

// A transposed digit fails.
assert!(Lokationsbuendelcode::new("9992000000062").is_err());
```

They are **two types**, not one, although the validation is identical. A
`Lokationszuordnung` carrying an object code where its structure code belongs
describes a bundle that does not exist, and no amount of check-digit arithmetic
notices — the type does.

### The codes stay `String` on the generated structs

`Marktlokation.lokationsbuendel_objektcode` is an `Option<String>`, deliberately.
This is the same trade the crate settles the same way for
`Zahlungsinformation.iban`: a newtype on the field would make one mistyped code
fail the deserialization of the **whole** `Marktlokation` it sits on — id,
address, Netzbetreiber and all. The checked accessor costs the caller one
`Result` instead:

```rust
use rubo4e::current::TechnischeRessource;
use rubo4e::lokationsbuendel::LokationsbuendelObjekt;

let tr = TechnischeRessource {
    lokationsbuendel_objektcode: Some("9992000001024".into()),
    ..Default::default()
};

let code = tr.lokationsbuendel_objektcode().unwrap().unwrap();
assert_eq!(code.as_str(), "9992000001024");
```

---

## What an object code means

The codelist assigns object codes *"in Abhängigkeit vom Objekt (NeLo, MeLo, MaLo,
TR), der Ebene (1, 2, 3), der Richtung"* — so one code is a complete coordinate,
and `9992 00000 101 6` is **the** consumption Marktlokation on level 1 in every
structure that uses it.

`rubo4e` ships that as data:

```rust
use rubo4e::current::TechnischeRessource;
use rubo4e::lokationsbuendel::{Flussrichtung, LokationsbuendelObjekt, Objekttyp};

let heat_pump = TechnischeRessource {
    lokationsbuendel_objektcode: Some("9992000001024".into()),
    ..Default::default()
};

let rolle = heat_pump.objektrolle().unwrap();
assert_eq!(rolle.objekttyp, Objekttyp::TechnischeRessource);
assert_eq!(rolle.richtung, Some(Flussrichtung::Verbrauch));
assert_eq!(rolle.ebene, 1);
```

The four `Objekttyp` values are exactly the four objects chapter 2.1 of the
codelist names — NeLo, MeLo, MaLo, TR. Wandler, Trafo and Tranche are listed there
as explicitly **not** part of a structure: they travel in UTILMD / UTILTS and do
not change the shape. The enum is closed for that reason, so an exhaustive `match`
in your mapper breaks loudly if BDEW ever adds a fifth.

### `SteuerbareRessource` is outside the codelist

BO4E puts `lokationsbuendelObjektcode` on `SteuerbareRessource` too, but the BDEW
codelist has no object code that *means* "steuerbare Ressource" — the four objects
above are the whole set. `Lokationsbuendel::steuerbare_ressourcen()` lists them;
`audit_buendel()` leaves them alone rather than reporting every entry as unknown.

---

## The published codelist, as data

`rubo4e` carries EDI@Energy's **"Codeliste der Lokationsbündelstrukturen"** (BDEW,
version 1.0, published 31 March 2023, applicable from 1 October 2024): 15
structures, 27 object codes, each structure's rows with their cardinalities and
the object references the flexible ones require.

```rust
use rubo4e::lokationsbuendel::{Lokationsbuendelstruktur, Objektfunktion};

let s = Lokationsbuendelstruktur::from_wire("9992000000026").unwrap();
assert_eq!(s.bezeichnung, "Verbrauch mit einer Messlokation (Standard)");

// Exactly one Messlokation, measuring at the Netzübergabe.
let melo = s.objekt("9992000001032").unwrap();
assert_eq!(melo.cardinality(), "1");
assert!(melo.is_mandatory());
assert_eq!(melo.rolle().funktion, Some(Objektfunktion::Netzuebergabe));

// Any number of technische Ressourcen behind it.
let tr = s.objekt("9992000001024").unwrap();
assert_eq!(tr.cardinality(), "0-N");
assert!(tr.permits(0) && tr.permits(9_999));
```

The 15 structures, by code:

| Code | Bezeichnung |
|---|---|
| `9992 00000 001 8` | Verbrauch ohne Messlokation (Pauschal) |
| `9992 00000 002 6` | Verbrauch mit einer Messlokation (Standard) |
| `9992 00000 003 4` | Erzeugung mit einer Messlokation |
| `9992 00000 004 2` | Erzeugung ohne getrennt gemessene Erzeugung |
| `9992 00000 006 8` | Verbrauch mit flexibler Hinterschaltung ohne Erzeugung |
| `9992 00000 007 6` | Verbrauch mit flexibler Hinterschaltung und nicht getrennt gemessener Erzeugung |
| `9992 00000 008 4` | Verbrauch mit flexibler Hinterschaltung und getrennt gemessener flexibler Erzeugung |
| `9992 00000 010 9` | Summenmessung Verbrauch ohne Erzeugung |
| `9992 00000 011 7` | Summenmessung mit mindestens einer ungemessenen Erzeugung |
| `9992 00000 012 5` | Summenmessung mit mindestens einer separat gemessenen Erzeugung |
| `9992 00000 013 3` | Erzeugung mit getrennt gemessener Erzeugung |
| `9992 00000 015 9` | Erzeugungskaskade mit ungemessener Erzeugung |
| `9992 00000 016 7` | Erzeugungskaskade mit gemessener und ungemessener Erzeugung |
| `9992 00000 017 5` | Verbrauchskaskade mit ungemessenen TR (Wärmepumpenkaskade) |
| `9992 00000 018 3` | Verbrauchskaskade mit ungemessenem Verbrauch und gemessener Erzeugung |

A well-formed code outside this list resolves to `None`, not to an error: the
codelist's own introduction says complex or special structures are agreed
bilaterally rather than coded, and BDEW may extend the list.

---

## Auditing a bundle

`audit_buendel()` checks a decoded `Lokationszuordnung` against the structure it
declares — the codes, the object types, and every cardinality:

```rust
use rubo4e::current::{Lokationszuordnung, Marktlokation, Messlokation};
use rubo4e::lokationsbuendel::{Befund, LokationsbuendelExt, Objekttyp};

let zuordnung = Lokationszuordnung {
    lokationsbuendelcode: Some("9992000000026".into()),
    marktlokationen: Some(vec![Box::new(Marktlokation {
        lokationsbuendel_objektcode: Some("9992000001016".into()),
        ..Default::default()
    })]),
    // The structure requires exactly one Messlokation, and there is none.
    ..Default::default()
};

let report = zuordnung.audit_buendel();
assert!(!report.is_conformant());
assert!(report.befunde.contains(&Befund::AnzahlVerletzt {
    code: "9992000001032".into(),
    objekttyp: Objekttyp::Messlokation,
    gefunden: 0,
    erwartet: "1".into(),
}));
```

What it finds:

| `Befund` | Means |
|---|---|
| `StrukturcodeFehlt` | no `lokationsbuendelcode` |
| `StrukturcodeUngueltig` | present, but not a valid BDEW code |
| `StrukturUnbekannt` | valid, but not one of the 15 |
| `ObjektcodeFehlt` | an object states no place in the structure |
| `ObjektcodeUngueltig` | its code fails the check digit |
| `ObjektcodeUnbekannt` | valid, but not in the catalogue |
| `ObjekttypWiderspruch` | a Marktlokation carrying a Messlokation's code |
| `ObjektcodeNichtInStruktur` | catalogued, but the declared structure does not use it |
| `AnzahlVerletzt` | the count is outside `Starr`/`Flexibel` bounds — including zero where one is required |

Object-level checks that do not need the structure still run when the structure
code is missing, so a report is never empty of everything it could have said.

### `audit_buendel()` is not `validate()`

The same line [Time Series](@/docs/timeseries.md#audit-is-not-validate) draws.
BO4E requires none of this — a `Lokationszuordnung` with no codes at all is a
valid `Lokationszuordnung` — so nothing here is wired into `.validate()`. It is a
data-quality report you run where you want one.

---

## § 14a EnWG

The § 14a case is what the levels and directions are *for*. A steuerbare
Verbrauchseinrichtung is a `TechnischeRessource` whose object code says
`Flussrichtung::Verbrauch`; the Marktlokation it is billed through is the one on
the same level.

```rust
use rubo4e::current::Lokationszuordnung;
use rubo4e::lokationsbuendel::{LokationsbuendelExt, Objekttyp};

# let zuordnung = Lokationszuordnung::default();
let buendel = zuordnung.buendel();

// Heat pumps, wallboxes, storage heaters — not a PV inverter, and not a battery,
// whose code says "Verbrauch & Erzeugung (Speicher)".
for tr in buendel.verbrauchs_ressourcen() {
    let _ = tr;
}

// Everything on one level, with the type its code stands for.
let level_one = buendel.objekte_auf_ebene(1);
let _: &[(Objekttyp, &str)] = &level_one;
```

The steering side of § 14a — Direktansteuerung versus a customer EMS, and the
identifier of the Steuerungseinrichtung — is **not** in BO4E `v202607.1.0` at all;
see [Serialization § ZusatzAttribute](@/docs/serialization.md#zusatzattribute-and-namespaces)
for where it goes instead.

---

## Feature gates

| Item | Needs |
|---|---|
| `Lokationsbuendelcode`, `LokationsbuendelObjektcode` | `identifiers` (default) |
| the codelist — `Lokationsbuendelstruktur`, `Objektrolle`, `STRUKTUREN`, `OBJEKTROLLEN` | nothing |
| `LokationsbuendelExt`, `LokationsbuendelObjekt`, `Lokationsbuendel`, `audit_buendel` | `versioned` |

The codelist is plain static data, so a transport crate that pulls in `rubo4e`
with `default-features = false` still gets it, and still gets the two validated
codes.

---

## Source

EDI@Energy / BDEW, **"Codeliste der Lokationsbündelstrukturen"**, version 1.0,
published 31 March 2023, to be applied from 1 October 2024 — chapters 2 (the
legend and the logic), 3 (the overview of structure codes) and 4 (the per-structure
tables). Published by the Bundesnetzagentur alongside BK6-22-024.
