+++
title = "Time Series & Units"
description = "Interval series and register series, the coverage audit, Zeitraum's instant mode, and the unit dimensions behind sum, integrate and consumption."
weight = 45
+++

BO4E carries readings over time in **two** shapes, and they are not
interchangeable:

| Shape | Carried by | Each entry is | Read with |
|---|---|---|---|
| **Interval series** | `Lastgang`, `Zeitreihe` | a `Zeitreihenwert`: a value **over** a `Zeitraum` | `Bo4eTimeSeries` |
| **Register series** | `Zaehlwerk` | a `Messwert`: the meter's cumulative state **at** an instant | `Zaehlwerk::readings` |

The distinction decides the arithmetic: interval values are quantities you sum or
integrate, register values are *states* you difference. Everything up to
[Register readings](#register-readings) is about the first half.

Interval series are the highest-volume payload in German market communication — a
year of quarter-hours is 35 040 entries — and the ones the schema leaves most
open. Each entry states its own `Zeitraum`, and nothing requires the entries to
be sorted, contiguous, disjoint, or the length the `Lastgang` declares.

Two modules cover this: [`rubo4e::timeseries`](https://docs.rs/rubo4e/latest/rubo4e/timeseries/)
and [`rubo4e::units`](https://docs.rs/rubo4e/latest/rubo4e/units/).

---

## The interval a `Zeitreihenwert` states

`Zeitreihenwert.zeitraum` is documented *"Zeitraum für das Messintervall"*, and
for a quarter-hourly series that is BO4E's **third** `Zeitraum` mode — *"Zeitraum:
Startzeitpunkt (Datum und Uhrzeit) bis Endzeitpunkt (Datum und Uhrzeit)"*. All
four fields are in play:

```json
{
  "wert": "400.0",
  "zeitraum": {
    "startdatum": "2026-01-01", "startuhrzeit": "00:00:00+01:00",
    "enddatum":   "2026-01-01", "enduhrzeit":   "00:15:00+01:00"
  }
}
```

The schema states the two inclusivities separately, and they do **not** agree
with the date pair on the same struct:

| Pair | Convention |
|---|---|
| `startdatum` / `enddatum` | `[start, end]` — **closed** |
| `startuhrzeit` / `enduhrzeit` | `[start, end)` — **half-open** |
| all four together (an instant range) | `[start, end)` — **half-open** |

So `as_instant_range()` returns a `Range`, not a `RangeInclusive`. Consecutive
quarter-hours abut without overlapping, and `00:15` belongs to exactly one of
them — the property a load profile has to have before it can be summed.

```rust
use rubo4e::current::Zeitraum;
use time::macros::datetime;

let start = datetime!(2026-01-01 00:00 +01:00);
let slot = Zeitraum::from_instants(start, start + time::Duration::minutes(15));

assert_eq!(slot.startuhrzeit.as_deref(), Some("00:00:00+01:00"));
assert!(slot.contains_instant(start));                                    // inclusive
assert!(!slot.contains_instant(start + time::Duration::minutes(15)));     // exclusive
assert_eq!(slot.instant_duration(), Some(Ok(time::Duration::minutes(15))));
```

### The date accessors read the date pair, and only that

A `Zeitraum` carrying all four fields still answers `as_inclusive_range()`,
`contains()` and `whole_days()` — about **whole days**. A 15-minute slot inside
one day is `whole_days() == Some(1)` and `contains()` that entire day. That is the
correct reading of the date pair and the wrong reading of the value, so route on
`is_instant_range()`:

```rust
if z.is_instant_range() {
    let range = z.as_instant_range().unwrap()?;   // Range<OffsetDateTime>, half-open
} else {
    let range = z.as_inclusive_range();           // Option<RangeInclusive<Date>>, closed
}
```

### The offset is not optional here

`start_instant()` fails with `ZeitpunktError::MissingOffset` when the time of day
carries no UTC offset. A wall-clock reading is not a moment, and Germany changes
offset twice a year: assuming `+01:00` is wrong for half of it and assuming UTC is
wrong for all of it. BO4E's own examples all carry one.

Because the offset travels with each boundary, slots written in different offsets
land on one timeline: `01:30+02:00` and `00:30+01:00` are the same instant, and
the audit below reports them as an overlap rather than as two adjacent slots.

`contains_instant()` distinguishes the two open ends. An **absent** bound is
unbounded on that side, matching `contains()` on the date pair. A **malformed**
one answers `false` outright: a bound you cannot read is not one you can
establish you are inside, and dropping the record is the safe direction for the
`.filter()` this predicate exists for. Reach for `as_instant_range()` when a
malformed value has to stay distinguishable from an out-of-range one.

---

## Auditing a series

`Bo4eTimeSeries::audit()` walks the entries once and returns a `CoverageReport`
of facts — not a verdict. A gappy Lastgang is still a valid Lastgang (a meter
that was not installed yet produces one), and whether to accept it is your call.

```rust
use rubo4e::timeseries::Bo4eTimeSeries;

let report = lastgang.audit();

report.reference;      // Option<Range<OffsetDateTime>> — the span audited
report.covered;        // Duration actually covered, an overlap counted once
report.gaps;           // Vec<Range<..>> — stretches nothing covers
report.overlaps;       // Vec<Range<..>> — stretches more than one entry covers
report.unplaced;       // entries with no resolvable interval, each with a reason
report.wrong_length;   // indices whose length is not zeitIntervallLaenge
report.unusable;       // indices whose status is FEHLT / NICHT_VERWENDBAR
report.out_of_order;   // the entries are not listed in ascending start order

report.is_complete();      // the timeline is covered exactly once
report.is_usable();        // …and every entry carries a usable value
report.coverage_ratio();   // Option<f64> in 0.0 ..= 1.0
report.missing();          // total length of every gap
```

**`audit()` measures the series against itself.** A series missing its whole last
day looks complete that way, so pass the period it was *supposed* to cover:

```rust
let day = lastgang.audit_over(start..start + time::Duration::days(1));
```

Entries outside the reference are clipped where they overlap it and ignored where
they do not, so an extra day at the front cannot mask a missing day at the back.

### Two questions, two answers

`is_complete()` is a claim about the **timeline**. A `Zeitreihenwert` whose status
is `FEHLT` still occupies its slot, so a series where every reading is declared
absent covers its span exactly once and contains nothing:

```rust
assert!(report.is_complete());     // no hole in the timeline
assert_eq!(report.unusable, [1]);  // …but this slot says FEHLT
assert!(!report.is_usable());      // so it is not data
```

`Messwertstatus` gets three predicates that partition the enum — every variant is
exactly one of them, and a drift guard fails the build if a schema release breaks
that:

| Predicate | Variants |
|---|---|
| `is_measured()` | `ABGELESEN` |
| `is_substitute()` | `ERSATZWERT`, `VORSCHLAGSWERT`, `PROGNOSEWERT`, `VORLAEUFIGERWERT`, `ENERGIEMENGESUMMIERT`, `ANGABE_FUER_LIEFERSCHEIN` |
| `!is_usable()` | `FEHLT`, `NICHT_VERWENDBAR`, and the `Unknown` catch-all |

The audit only refuses the third group. Whether an `ERSATZWERT` is good enough for
what you are doing is a settlement question, not a data-quality one, so it stays
yours.

### `audit()` is not `validate()`

Nothing here is wired into `.validate()`. BO4E states none of these properties,
so a `Validated<Lastgang>` says nothing about gaps — the same line
[Validation](@/docs/validation.md) draws between conformance rules and this
crate's own judgements.

---

## Units have dimensions

BO4E puts every quantity it knows into one flat `Mengeneinheit`: energies, powers,
their reactive counterparts, a volume, eleven durations, a percentage, a frequency
and a dimensionless marker, side by side. Nothing in the schema says which may be
added, which convert into which, or how long a `MONAT` is — so every consumer
re-derives it, and the derivations disagree.

```rust
use rubo4e::units::Dimension;
use rubo4e::current::Mengeneinheit;

Mengeneinheit::Kwh.dimension();        // Some(Dimension::Energy)
Mengeneinheit::Kw.dimension();         // Some(Dimension::Power)
Mengeneinheit::Unknown.dimension();    // None

Mengeneinheit::Mwh.conversion_factor(Mengeneinheit::Kwh);  // Some(1000)
Mengeneinheit::Kwh.conversion_factor(Mengeneinheit::Kw);   // None — a different dimension
```

| Dimension | Units | Base |
|---|---|---|
| `Energy` | `WH`, `KWH`, `MWH` | `WH` |
| `Power` | `W`, `KW`, `MW` | `W` |
| `ReactiveEnergy` | `VARH`, `KVARH` | `VARH` |
| `ReactivePower` | `VAR`, `KVAR` | `VAR` |
| `Volume` | `KUBIKMETER` | `KUBIKMETER` |
| `Time` | `SEKUNDE` … `JAHR` | `SEKUNDE` |
| `Count` | `STUECK` | `STUECK` |
| `Ratio` | `PROZENT` | `PROZENT` |
| `Frequency` | `HZ` | `HZ` |
| `EnergyPerTemperature` | `KWHK` (kWh/K) | `KWHK` |
| `Dimensionless` | `DIMENSIONSLOS` | `DIMENSIONSLOS` |

### Calendar units are refused, not averaged

`MONAT`, `QUARTAL`, `HALBJAHR` and `JAHR` are in `Dimension::Time` and compare as
time units, but they have **no** `factor_to_base()` and **no** `exact_duration()`:
converting one needs a start date the `Menge` does not carry. `is_calendar()` names
them. This is the same call [`iso8601_duration`](https://docs.rs/rubo4e/latest/rubo4e/iso8601_duration/)
makes about `P1Y` / `P1M`, for the same reason — and a drift guard checks that
exactly the four lack a factor.

### Convert through the base unit, not by a scalar

`conversion_factor` returns a `Decimal`, and `SEKUNDE → MINUTE` is `1/60`, which
has no exact decimal form. Multiplying by the rounded value leaves 120 seconds as
`2.000…004` minutes. `Menge::convert_to` scales up then down instead, in one
multiply and one divide, and lands on `2`:

```rust
let two_minutes = Menge { wert: Some(dec!(120)), einheit: Some(Mengeneinheit::Sekunde), ..d() }
    .convert_to(Mengeneinheit::Minute)
    .unwrap();
assert_eq!(two_minutes.wert, Some(dec!(2)));
```

### Extensive and intensive

`is_extensive()` separates the units that may be **summed over a period** —
energy, reactive energy, volume, count, duration — from the ones that may not:
power, frequency, percentage. Adding 96 quarter-hourly kW readings produces a
number with no physical meaning, and `sum()` refuses to produce it:

```rust
lastgang.sum();              // None — messgroesse is KW
lastgang.integrate();        // Some(450) — Σ value × interval_hours
lastgang.integrated_unit();  // Some(Mengeneinheit::Kwh) — what that 450 is in
```

`integrate()` is the step from a `Lastgang` carrying power to the energy an
invoice bills. The unit of the result follows `Mengeneinheit::energy_unit()`:
`W → WH`, `KW → KWH`, `MW → MWH`, and the reactive pair likewise.

**For a stated unit, exactly one of the two answers.** `KWH` sums and does not
integrate; `KW` integrates and does not sum. Integrating an energy would give
kWh·h, which is why `integrate()` refuses a unit that has no energy counterpart.
A series that states no unit answers to both, on the caller's word — nothing
contradicts it.

For a single quantity the same arithmetic is on `Menge`:

```rust
let load = Menge { wert: Some(dec!(400)), einheit: Some(Mengeneinheit::Kw), ..d() };
let energy = load.energy_over(time::Duration::minutes(15)).unwrap();
assert_eq!(energy.wert, Some(dec!(100)));                 // 100 kWh
assert_eq!(energy.einheit, Some(Mengeneinheit::Kwh));
```

And `Menge::as_duration()` is what turns `Lastgang.zeitIntervallLaenge`
(`{"wert": 15, "einheit": "MINUTE"}`) into the 15 minutes the payload means —
which is where `expected_interval()` gets the length it checks each entry against.

---

## Register readings

A `Zaehlwerk` holds `Messwert`s — cumulative meter states, not quantities. The
consumption between two of them is what BO4E states on the `wandlerfaktor` field
itself: *"Mit diesem Faktor wird eine Zählerstandsdifferenz multipliziert, um zum
eigentlichen Verbrauch im Zeitraum zu kommen."*

```text
consumption = (to − from) × wandlerfaktor
```

```rust
let register = Zaehlwerk {
    vorkommastelle: Some(6),                  // a six-digit display
    wandlerfaktor: Some(dec!(40)),            // an indirectly-measuring meter
    ..Default::default()
};

register.consumption_between(dec!(1_000), dec!(1_050));   // Ok(2_000)  — 50 × 40
register.readings();                                      // chronological, usable only
register.register_capacity();                             // Some(1_000_000)
register.total_consumption();                             // the whole series
```

### Two corrections the bare subtraction gets wrong

**A wrap-around.** A six-digit register going `999998 → 000012` has not consumed
`−999 986`; it has consumed `14`. `vorkommastelle` states where the display wraps,
and `consumption_between` adds the capacity back — once:

```rust
register.consumption_between(dec!(999_998), dec!(12));    // Ok(560)  — 14 × 40
dec!(12) - dec!(999_998);                                 // -999_986 — the naive answer
```

**A fall it cannot explain.** With no `vorkommastelle` stated, a wrap-around
cannot be told from a fault, so it is refused rather than picked. And a fall
*larger* than one whole revolution is not a wrap-around at all — adding the
capacity back still leaves it negative — so that is refused too.

### Where it stops rather than guessing

`total_consumption()` returns `Result<Decimal, ConsumptionError>`, and every
variant is a place where continuing would produce a number that means nothing:

| Variant | Why |
|---|---|
| `TooFewReadings` | a consumption needs a pair |
| `MeterExchange` | a reading is marked `Z78_GERAETEWECHSEL` — the register restarted from an unrelated state, so the difference across that boundary is not a consumption. Split the series there; only you know which meter the second half belongs to |
| `DecreasedWithoutRegisterWidth` | see above |
| `IncompatibleUnit` | a reading is in a unit that does not convert to the register's, so the total would silently span a gap it does not admit to |
| `Overflow` | the arithmetic left `Decimal`'s range |

A reading marked `FEHLT` or `NICHT_VERWENDBAR` is not a zero, so it is left out of
the series rather than differenced against — the same call
`Bo4eTimeSeries` makes.

### Units are reconciled first

`Messwert.wert` is a `Menge`, so a reading carries its own unit, which need not
be the register's. A reading in MWh on a kWh register is brought onto the
register's scale before it is differenced — otherwise `1.1 MWh − 1000 kWh` reads
as `0.1`. That is [`Menge::convert_to`](#convert-through-the-base-unit-not-by-a-scalar)
doing the work; a unit that does not convert is an error, not an omission.

---

## Feature gates

| Module | Requires | Without it |
|---|---|---|
| `rubo4e::units` — `Dimension`, `dimension()`, `is_extensive()` | `versioned` | — |
| `factor_to_base`, `conversion_factor`, `Menge::convert_to` | `versioned` + `decimal` | absent; `Menge.wert` is a `String` |
| `exact_duration`, `Menge::as_duration`, `energy_over` | `+ time` | absent |
| `rubo4e::timeseries` — the whole timeline walk | `versioned` + `time` | absent |
| `sum()`, `integrate()`, `Lastgang::expected_interval()` | `+ decimal` | `expected_interval()` is `None`, so `wrong_length` stays empty |
| `Zaehlwerk::readings` / `consumption_between` / `total_consumption` | `+ decimal` | absent; register states are `String` without it |

---

## Cost

`placed()` parses each entry's `Zeitraum` as it goes and allocates nothing.
`audit()` sorts a `Vec` of the resulting ranges, so it is `O(n log n)` in the
number of entries and allocates once. A year of quarter-hours is 35 040 ranges —
one allocation of roughly a megabyte, and a single pass.

Parsing the two `format: "time"` strings dominates, so each entry is parsed
exactly once: `audit()` derives its reference span from the ranges it has already
placed rather than calling `span()` first. On a 2026 laptop that is ≈ 10 M
entries/s — a settlement year in about 3.4 ms — for every shape, clean or
pathological. `benches/timeseries_perf.rs` measures all four:

```text
cargo bench --bench timeseries_perf --features versioned,time,decimal
```

If the timeline is not in question and only the readings are, `all_values_usable()`
answers that in one pass with no allocation at all.

`Bo4eTimeSeries` is deliberately **not** sealed: implementing it takes two methods
(`werte` and `expected_interval`, plus `einheit`), and everything else is a
default, so a downstream type that wraps a series of its own opts in the same way
it can into `Bo4eStrict`.
