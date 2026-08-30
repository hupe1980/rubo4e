+++
title = "Beyond the Schema"
description = "What rubo4e does when a market rule needs something BO4E has not modelled: the one test every addition passes, where each kind of addition lands, and BK6-20-160 Modell 2 worked through end to end as the example."
weight = 47
+++

The rules that govern BO4E's objects — GPKE, MaBiS, WiM, the BNetzA Festlegungen
— keep moving, and BO4E carries only what fits on an existing Geschäftsobjekt.
Sooner or later you need a fact the schema has no field for.

[The test](#the-test) decides what happens then, and
[the placement table](#where-additions-live) says where it goes. The e-mobility
**Modell 2** of BNetzA **BK6-20-160** is the worked example below: it asks for six
such facts at once, and four turn out to be there already.

---

## A generated enum is never forked

`src/generated/` is produced from the pinned BO4E JSON Schema, and
[a drift guard](@/docs/testing.md) fails the build when it stops matching. A value
added to a generated enum emits a wire string every other BO4E implementation
decodes as its `Unknown` catch-all, and which
[`Bo4eStrict`](@/docs/serialization.md) — this crate's own strict decoder — then
rejects.

So a market rule BO4E has not modelled has three honest answers, in order:

1. **BO4E already has it, somewhere else.** Say where, and test it.
2. **The state is expressible in what BO4E has.** Read it from the fields, don't
   invent a value.
3. **It genuinely is not there, and it qualifies a BO4E field.** Carry it as a
   value type in a
   [registered `ZusatzAttribut` key](@/docs/serialization.md#typed-keys-and-the-keys-this-crate-registers),
   never as a new `_typ` and never as a new enum member.

And a fourth that is not an answer at all: **it is a domain aggregate of another
standard.** Then it does not belong in `rubo4e`, whatever the escape hatch would
allow.

### The test

> **Does it read, type, or guard a value that arrives in a BO4E payload?**

Everything on this page passes it. A Bilanzierungsgebiet's Stammdaten do not, and
so are [not modelled here](#the-bilanzierungsgebiet-and-where-this-crate-stops) —
that is the boundary in one sentence, and it is the same one that makes
[`Lokationsbuendel`](@/docs/lokationsbuendel.md#there-is-no-lokationsbuendel-business-object)
a *view* rather than a Geschäftsobjekt.

It decides **naming** too. There is no `rubo4e::mabis` module and no `mabis` page:
a module or page named for a *standard* invites everything that standard covers.
Naming by what the code does *to BO4E* keeps the boundary self-enforcing.

---

## Where additions live

Each item sits where the crate's existing precedent puts its kind — never in a
module of its own named after the rule that motivated it:

| Item | Module | Precedent |
|---|---|---|
| `Zaehlpunktbezeichnung` | `identifiers` | `MeloId` — a validated string newtype |
| `Zaehlpunktart`, `Zaehlpunkt` | `identifiers` | `EicType`, `MpIdAuthority` — helper types beside the identifier they classify |
| `Marktlokation::bilanzierungsgebiet_checked()` | `convenience` | `Zahlungsinformation::iban_checked()` — the same shape, field for field |
| `Bilanzierung::aggregationszustaendigkeit()`, `Aggregationszustaendigkeit` | `convenience` | `Rechnung::billing_period()` — a derived reading of the fields |
| `TechnischeRessource::is_emobilitaetsladesaeule()` | `convenience` | `Messwertstatus::is_usable()` |

All three accessors are **inherent** methods, so none of them costs you a trait
import.

---

## Modell 2 in one table

BK6-20-160 Anlage 6 defines *"Netzzugangsregeln zur Ermöglichung einer
ladevorgangscharfen bilanziellen Energiemengenzuordnung für Elektromobilität"*.
The BDEW Anwendungshilfe (v1.3, 1 April 2025) calls the resulting alternative
settlement path **Modell 2**: the Marktlokation is no longer settled in the
Bilanzierungsgebiet of the VNB; its energy crosses as a Netzgangzeitreihe into
the Bilanzierungsgebiet of the NB (LPB), where charging sessions are settled one
by one.

| Fact a Modell-2 implementation needs | Carrier | Answer |
|---|---|---|
| the MaLo runs under Modell 1 or Modell 2 | `Bilanzierung.abwicklungsmodell` | ① already there |
| the MaLo is a charging point | `TechnischeRessource.emobilitaetsart` = `E_MOBILITAETSLADESAEULE` | ① already there |
| which Bilanzierungsgebiet | `Marktlokation.bilanzierungsgebiet` | ① already there — see below |
| the Aggregationsverantwortung **ruht** | absent field + `Abwicklungsmodell::Modell2` | ② read from the pair |
| the Netzgangzeitreihe | a `Lastgang` at a Zählpunkt | ② already expressible |
| which kind of Zählpunkt an ID names | `identifiers::Zaehlpunkt` | ③ a value type that **guards** `messlokationsId` |
| the Bilanzierungsgebiet's own Stammdaten | *not here* — see [below](#the-bilanzierungsgebiet-and-where-this-crate-stops) | a MaBiS aggregate, not a BO4E fact |

---

## The Bilanzierungsgebiet — and where this crate stops

BO4E `v202607.1.0` has **no** `Bilanzierungsgebiet` schema and `BoTyp` has no
member for one. It carries the EIC alone — as a `String` on
`Marktlokation.bilanzierungsgebiet`, and typed on
`StandorteigenschaftenStrom.bilanzierungsgebiet Eic`.

The field stays a `String` on the generated struct, because BO4E documents it only
as *"Bilanzierungsgebiet, dem das Netzgebiet zugeordnet ist"* without naming a
format, and this crate
[types a field only where the schema names one](@/docs/generator.md#semantic-field-typing).
MaBiS Kapitel 3.5 does name one — *"Jedes BG ist durch einen eindeutigen Energy
Identification Code (EIC) zu kennzeichnen"* — so the checked accessor exists
instead, and pins the ENTSO-E object type:

```rust
use rubo4e::current::Marktlokation;

let malo = Marktlokation {
    bilanzierungsgebiet: Some("11YN-0000-0001-Q".into()),
    ..Default::default()
};
assert!(malo.bilanzierungsgebiet_checked().unwrap().is_ok());

// A Bilanzkreis is a *party* code (`11X…`), not an area code, and does not pass.
let wrong = Marktlokation {
    bilanzierungsgebiet: Some("11XSUEDWESTSTRO8".into()),
    ..Default::default()
};
assert!(wrong.bilanzierungsgebiet_checked().unwrap().is_err());
```

**That is where `rubo4e` stops.** The rest of what MaBiS says a Bilanzierungsgebiet
is — the Regelzone, a Gültigkeitsbeginn that may fall only on the first of a month,
the owning Marktpartner-ID, the e-mobility Zusatzinformation, the Bilanzkreis named
to carry the Netzbetreiber-Deltazeitreihe — reads **no BO4E field**. Modelling it
here would fail the crate's own test above: it would be a new aggregate wrapped in a
`ZusatzAttribut` that no other BO4E implementation understands, which is the very
thing [`Lokationsbuendel`](@/docs/lokationsbuendel.md#there-is-no-lokationsbuendel-business-object)
is a *view* rather than a Geschäftsobjekt to avoid.

Those Stammdaten belong in the crate that owns the MaBiS processes, which can
depend on `rubo4e` for the identifier layer — `BilanzierungsgebietId` already
tells an area code (`11Y…`) from a Bilanzkreis (`11X…`), which MSCONS carries as
free text — and register its own key in its own namespace with
`AttributKey::new`.

## The Zählpunkt that is not a Messlokation

BO4E calls `Messlokation.messlokationsId` *"Die Messlokations-Identifikation; Das
ist die frühere Zählpunktbezeichnung"* — one grammar, and one assumed meaning.
The Anwendungshilfe § 1.6.2 is explicit that the assumption breaks:

> Für den Zählpunkt (eMob) wird eine ID (Zählpunktbezeichnung) vergeben.
> **Hinweis: Für den Zählpunkt (eMob) wird nicht die ID der Messlokation
> (Zählpunktbezeichnung) verwendet.**

`Zaehlpunktbezeichnung` validates exactly as `MeloId` does and is deliberately a
different type, so the mistake the standard warns against is a compile error
rather than a settlement incident:

```rust
use rubo4e::identifiers::Zaehlpunktbezeichnung;
use rubo4e::identifiers::{Zaehlpunkt, Zaehlpunktart};

let zp = Zaehlpunkt::new(
    Zaehlpunktart::NetzgangzeitreiheEmob,
    Zaehlpunktbezeichnung::new("DE0000000000000000000000000000042").unwrap(),
);

assert!(zp.is_emobilitaet());
assert_eq!(zp.as_melo_id(), None);   // …and never will be
```

`Zaehlpunktart` covers the four MaBiS names: `Messlokation`, `Netzuebergabe`
(the NGZ between two VNB Bilanzierungsgebiete), `NetzgangzeitreiheEmob` (the
Zählpunkt (eMob)), `NetzzeitreiheEmob` (the MaBiS-Zählpunkt für NZR (eMob)) and
the generic `MabisZaehlpunkt`. It is `#[non_exhaustive]` — unlike the closed BDEW
codelists this crate ships, MaBiS gains Zählpunktarten as processes are added, and
the eMob pair arrived with BK6-20-160 itself.

A Zählpunkt rides on the Messlokation under a registered key, so a
[Lokationsbündel](@/docs/lokationsbuendel.md) carries it without a new BO:

```rust
use rubo4e::current::Messlokation;
use rubo4e::zusatz_attribut::{well_known, ZusatzAttributeExt};

let mut melo = Messlokation::default();
melo.set_zusatz_attribut_key(&well_known::ZAEHLPUNKT, &zp).unwrap();
assert_eq!(well_known::ZAEHLPUNKT.name(), "mabis:zaehlpunkt");
```

---

## `Zeitreihentyp` is not missing the Netzgangzeitreihe

`Zeitreihentyp` is chapter 1 of the BDEW **"Codeliste der Zeitreihentypen"** —
its schema description says so: *"Codes der **Summen**zeitreihentypen. Die
nachfolgenden Codes sind in DE7111 zu nutzen."* In **every** published version of
that list — 1.1a of 1 October 2012 through 1.1d of 1 April 2021 — `NGZ` appears
only inside the explanation of `NZR`:

> **NZR** — Netzzeitreihe = Übergabesumme zwischen Bilanzierungsgebieten,
> Summendifferenz der NGZ zwischen zwei Bilanzierungsgebieten.
> NGZ = Netzgangzeitreihe = gemessene Netzübergabe

It is not itself a DE7111 code, so BO4E is right not to have it, and adding
`Zeitreihentyp::Ngz` would put a non-code into a code list.

The two are different *kinds* of thing:

| | `NZR` | NGZ |
|---|---|---|
| What | the summed transfer between two Bilanzierungsgebiete | one measured series at one Zählpunkt |
| Where it lives | a `Zeitreihentyp` value | an MSCONS message, Prüfidentifikator **13018** — *"Lastgang Messlokation / Netzgang / Netzkoppelpunkt Strom"* |
| In BO4E | `Bilanzierung.zeitreihentyp` | a `Lastgang`, at a `Zaehlpunkt` |

So a Netzgangzeitreihe is read with
[`Bo4eIntervals`](@/docs/timeseries.md#one-reading-shape-for-all-three), and
`Zaehlpunktart` is what says which series a given `Lastgang` belongs to.

---

## E-Mobilitätsladesäule: BO4E already has it

`Verbrauchsart` is the Kraft/Licht/Wärme categorisation of a Marktlokation — `KL`,
`KLW`, `KLWS`, `W`, `WS` — and it has no charging-point member. That is not a gap:
BO4E models the charging point on the **technische Ressource**, where it has both
values already.

```rust
use rubo4e::current::{EMobilitaetsart, TechnischeRessource, TechnischeRessourceVerbrauchsart};

let ladesaeule = TechnischeRessource {
    emobilitaetsart: Some(EMobilitaetsart::EMobilitaetsladesaeule),
    technische_ressource_verbrauchsart: Some(TechnischeRessourceVerbrauchsart::EMobilitaet),
    ..Default::default()
};
assert!(ladesaeule.is_emobilitaetsladesaeule());
```

`EMobilitaetsart` is `WALLBOX` / `E_MOBILITAETSLADESAEULE` / `LADEPARK`, so the
distinction the Anwendungshilfe draws — the MaLo carries the Verbrauchsart
*"E-Mobilitätsladesäule"* — is expressible, and **no `ZusatzAttribut` should be
used for it**. A downstream crate reaching for a namespaced key here would be
inventing a second spelling of a value the standard already has.

---

## The resting Aggregationsverantwortung

Anwendungshilfe § 1.6.2:

> Beim Wechsel in das Modell 2 **ruht** die Aggregationsverantwortung für die
> Energiemenge der MaLo (NB oder ÜNB).

`Aggregationsverantwortung` has two members, `UENB` and `VNB`, and MaBiS needs a
third state. The wire encoding for it is an **absent** field — not a new enum
value. Writing `"RUHEND"` there produces a string every other implementation
decodes as `Unknown`, and which this crate's own `ensure_known_enums()` rejects.

So the state is read from the *pair* of fields:

```rust
use rubo4e::current::{Abwicklungsmodell, Bilanzierung};
use rubo4e::convenience::Aggregationszustaendigkeit;

let ruhend = Bilanzierung {
    abwicklungsmodell: Some(Abwicklungsmodell::Modell2),
    ..Default::default()
};
assert!(ruhend.aggregation_ruht());

// An absent field *alone* says nothing — BO4E declares it optional and most
// payloads omit it.
assert_eq!(
    Bilanzierung::default().aggregationszustaendigkeit(),
    Aggregationszustaendigkeit::Unbekannt,
);
```

Four states, because `None` is genuinely ambiguous and only `abwicklungsmodell`
disambiguates it: `Uebertragungsnetzbetreiber`, `Verteilnetzbetreiber`, `Ruhend`,
`Unbekannt`.

---

## The mobile Marktlokation validates

A Modell-2 Marktlokation has no fixed Lokationsadresse and no Messlokation of its
own — its energy arrives as charging sessions. Nothing in this crate rejects that,
and nothing should:

- BO4E declares **no** `required` array on `Marktlokation`. Every field is
  optional in the schema.
- The only cross-field rule `rubo4e` enforces is *at most one* Ortsangabe —
  a **conflict** rule, not a presence rule. Zero Ortsangaben conforms, and is
  common: BO4E has no reference type, so a Marktlokation referenced from a
  `Rechnung` is a full `Marktlokation` carrying little more than its ID.

```rust
use rubo4e::current::Marktlokation;
use rubo4e::prelude::*;

let mobil = Marktlokation {
    marktlokations_id: Some(MaloId::new("51238696781").unwrap()),
    bilanzierungsgebiet: Some("11YN-0000-0001-Q".into()),
    ..Default::default()          // no address, no Zählwerk, no Verbrauchsart
};
assert!(mobil.validate().is_ok());
```

`tests/modell2.rs` pins all of this. See [Validation](@/docs/validation.md) for the
three layers and what each is allowed to reject.

---

## Feature gates

| Item | Needs |
|---|---|
| `Zaehlpunktbezeichnung` | `identifiers` (default) |
| `Zaehlpunkt`, `Zaehlpunktart` | `identifiers` (default) |
| `Marktlokation::bilanzierungsgebiet_checked`, `Bilanzierung::aggregationszustaendigkeit`, `TechnischeRessource::is_emobilitaetsladesaeule`, `Aggregationszustaendigkeit` | `versioned` |
| `well_known::ZAEHLPUNKT`, `AttributKey` get/set | `versioned` + `json` |

---

## Sources

- BNetzA **BK6-20-160**, Anlage 6 — *Netzzugangsregeln zur Ermöglichung einer
  ladevorgangscharfen bilanziellen Energiemengenzuordnung für Elektromobilität
  (NZR-EMob)*.
- BDEW — *Anwendungshilfe zum Modell 2 zur ladevorgangscharfen bilanziellen
  Energiemengenzuordnungsmöglichkeit*, Version 1.3, 1 April 2025.
- BNetzA **BK6-24-174**, Anlage 3 — MaBiS, Kapitel 3.5 *"Bilanzierungsgebiete"*.
- BDEW EDI@Energy — *Codeliste der Zeitreihentypen*, versions 1.1a (2012) and
  1.1d (2021).
