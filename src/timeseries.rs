//! Placing a BO4E time series on a timeline, and reading what it means.
//!
//! BO4E carries readings over time in **two** shapes, and they are not
//! interchangeable:
//!
//! | Shape | Carried by | Each entry is | Read with |
//! |---|---|---|---|
//! | **Interval series** | [`Lastgang`], [`Zeitreihe`] | a [`Zeitreihenwert`]: a value **over** a `Zeitraum` | [`Bo4eTimeSeries`] |
//! | **Register series** | [`Zaehlwerk`] | a [`Messwert`]: the meter's cumulative state **at** an instant | [`Zaehlwerk::readings`] |
//!
//! The distinction decides the arithmetic. Interval values are quantities you
//! sum or integrate; register values are *states* you difference — and the bare
//! subtraction is wrong in two ways the schema itself tells you how to fix. See
//! [`Zaehlwerk::consumption_between`].
//!
//! # Interval series
//!
//! [`Lastgang`] and [`Zeitreihe`] carry a `Vec<`[`Zeitreihenwert`]`>`, and a year
//! of quarter-hours is 35 040 entries. Each states its own [`Zeitraum`], so the
//! series is a *bag* of intervals: nothing requires them to be sorted,
//! contiguous, disjoint, or the length the `Lastgang` declares.
//! [`audit`](Bo4eTimeSeries::audit) walks them once and says which they are.
//!
//! ```
//! # #[cfg(all(feature = "versioned", feature = "time", feature = "decimal"))] {
//! use rubo4e::current::{Lastgang, Menge, Mengeneinheit, Zeitraum, Zeitreihenwert};
//! use rubo4e::timeseries::Bo4eTimeSeries;
//! use rust_decimal::Decimal;
//! use time::macros::datetime;
//!
//! fn quarter_hour(from: time::OffsetDateTime, kw: i64) -> Zeitreihenwert {
//!     Zeitreihenwert {
//!         wert: Some(Decimal::from(kw)),
//!         zeitraum: Some(Zeitraum::from_instants(from, from + time::Duration::minutes(15))),
//!         ..Default::default()
//!     }
//! }
//!
//! let start = datetime!(2026-01-01 00:00 +01:00);
//! let quarter = Menge {
//!     wert: Some(Decimal::from(15)),
//!     einheit: Some(Mengeneinheit::Minute),
//!     ..Default::default()
//! };
//! let lastgang = Lastgang {
//!     messgroesse: Some(Mengeneinheit::Kw),
//!     // One 15-minute slot is missing: 00:15 – 00:30.
//!     werte: Some(vec![
//!         quarter_hour(start, 400),
//!         quarter_hour(start + time::Duration::minutes(30), 480),
//!     ]),
//!     ..Lastgang::new(quarter)
//! };
//!
//! let report = lastgang.audit();
//! assert!(!report.is_complete());
//! assert_eq!(report.gaps, [datetime!(2026-01-01 00:15 +01:00)
//!                          ..datetime!(2026-01-01 00:30 +01:00)]);
//!
//! // 30 minutes of data across a 45-minute span.
//! assert_eq!(report.covered, time::Duration::minutes(30));
//!
//! // The values are a power, so integrate rather than sum: 400 kW and 480 kW
//! // each held for a quarter of an hour is 220 kWh.
//! assert_eq!(lastgang.integrate(), Some(Decimal::from(220)));
//! assert_eq!(lastgang.sum(), None, "kW is not summable");
//! # }
//! ```
//!
//! # What is checked, and what is not
//!
//! [`audit`](Bo4eTimeSeries::audit) is a data-quality report, not a conformance
//! check. BO4E states none of these properties — a gappy Lastgang is a valid
//! Lastgang — so nothing here is wired into `.validate()`, the same line
//! [`validation::current::quality`] draws.
//!
//! # Register series
//!
//! A [`Zaehlwerk`]'s `messwerte` are cumulative meter states, so the consumption
//! is the difference between two of them, times the `wandlerfaktor` BO4E defines
//! on the field itself, corrected for a wrap-around `vorkommastelle` reveals:
//!
//! ```
//! # #[cfg(all(feature = "versioned", feature = "time", feature = "decimal"))] {
//! use rubo4e::current::Zaehlwerk;
//! use rust_decimal::Decimal;
//!
//! let register = Zaehlwerk {
//!     vorkommastelle: Some(6),                  // a six-digit display
//!     wandlerfaktor: Some(Decimal::from(40)),   // an indirectly-measuring meter
//!     ..Default::default()
//! };
//!
//! // 999 998 → 000 012 is 14 register steps, not −999 986.
//! assert_eq!(
//!     register.consumption_between(Decimal::from(999_998), Decimal::from(12)),
//!     Ok(Decimal::from(560)),                   // 14 × 40
//! );
//! # }
//! ```
//!
//! [`total_consumption`](Zaehlwerk::total_consumption) walks a whole register in
//! time order, and refuses rather than guessing where the arithmetic stops
//! meaning anything — a meter exchange, a fall no register width explains, a
//! reading in a unit that does not convert.
//!
//! [`validation::current::quality`]: crate::validation::current::quality
//! [`Lastgang`]: crate::current::Lastgang
//! [`Zeitreihe`]: crate::current::Zeitreihe
//! [`Zeitreihenwert`]: crate::current::Zeitreihenwert
//! [`Zeitraum`]: crate::current::Zeitraum
//! [`Zaehlwerk`]: crate::current::Zaehlwerk
//! [`Messwert`]: crate::current::Messwert

use crate::convenience::ZeitpunktError;
use crate::generated::v202607::{Lastgang, Zeitreihe, Zeitreihenwert};
// The register half needs a numeric register state, so it exists only with
// `decimal`; without it `Messwert.wert` is a `String` and a difference between
// two of them is not arithmetic this crate can do.
#[cfg(feature = "decimal")]
use crate::generated::v202607::{Messwert, Zaehlwerk};
use std::ops::Range;
use time::{Duration, OffsetDateTime};

// ─── Per-entry placement ─────────────────────────────────────────────────────

/// One entry of a time series, placed on the timeline.
///
/// The range is **half-open**, `[start, end)`: BO4E declares `startuhrzeit`
/// inclusive and `enduhrzeit` exclusive, so consecutive quarter-hours abut
/// without overlapping and `00:15` belongs to exactly one of them.
#[derive(Debug, Clone, PartialEq)]
pub struct PlacedValue<'a> {
    /// Index of this entry in the series' `werte` vector.
    pub index: usize,
    /// The measurement interval, `[start, end)`.
    pub range: Range<OffsetDateTime>,
    /// The entry itself.
    pub value: &'a Zeitreihenwert,
}

/// Why one entry could not be placed on the timeline.
///
/// Each carries the index of the offending entry, so a report names the row
/// rather than just the count.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct UnplacedValue {
    /// Index of this entry in the series' `werte` vector.
    pub index: usize,
    /// What stopped it from resolving to an interval.
    pub reason: UnplacedReason,
}

/// The reason an entry has no place on the timeline.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum UnplacedReason {
    /// The entry states no `zeitraum` at all.
    MissingZeitraum,
    /// The `zeitraum` does not encode a full instant on one or both ends — a
    /// date without a time of day, or a time of day without a date. See
    /// [`Zeitraum::as_instant_range`](crate::current::Zeitraum::as_instant_range).
    NotAnInstantRange,
    /// One end parsed as an instant but not correctly.
    Malformed(ZeitpunktError),
    /// The interval ends before it starts.
    Reversed,
}

impl std::fmt::Display for UnplacedReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnplacedReason::MissingZeitraum => f.write_str("no zeitraum"),
            UnplacedReason::NotAnInstantRange => {
                f.write_str("zeitraum does not state both a date and a time of day with an offset")
            }
            UnplacedReason::Malformed(e) => write!(f, "{e}"),
            UnplacedReason::Reversed => f.write_str("interval ends before it starts"),
        }
    }
}

// ─── Report ──────────────────────────────────────────────────────────────────

/// What [`audit`](Bo4eTimeSeries::audit) found.
///
/// Every field is a fact about the series, not a verdict: a Lastgang with gaps
/// is common and sometimes correct (a meter that was not installed yet), and
/// whether to reject it is the caller's call. [`is_complete`](Self::is_complete)
/// is the one convenience judgement, and it is deliberately strict.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[non_exhaustive]
pub struct CoverageReport {
    /// The span the series was audited against: the interval the entries
    /// themselves span, or the reference passed to
    /// [`audit_over`](Bo4eTimeSeries::audit_over).
    ///
    /// `None` when no entry could be placed at all.
    pub reference: Option<Range<OffsetDateTime>>,
    /// Total time actually covered by entries, counting an overlap once.
    pub covered: Duration,
    /// Stretches of [`reference`](Self::reference) that no entry covers, in
    /// ascending order.
    pub gaps: Vec<Range<OffsetDateTime>>,
    /// Stretches covered by more than one entry, in ascending order.
    ///
    /// Duplicate or double-counted readings — the failure mode that silently
    /// inflates a billed quantity, and the reason a plain sum over `werte` is
    /// not enough.
    pub overlaps: Vec<Range<OffsetDateTime>>,
    /// Entries that could not be placed on the timeline at all.
    pub unplaced: Vec<UnplacedValue>,
    /// Indices of entries whose length is not the one the series declares.
    ///
    /// Always empty when [`expected_interval`](Bo4eTimeSeries::expected_interval)
    /// is `None` — a [`Zeitreihe`] declares no
    /// interval, so an irregular one is not an error there.
    pub wrong_length: Vec<usize>,
    /// Indices of entries whose `status` says they carry no usable value —
    /// `FEHLT`, `NICHT_VERWENDBAR`, or an out-of-schema status.
    ///
    /// These still **occupy** their slot, so they close a gap without filling
    /// it: a series where every reading is `FEHLT` covers its span exactly once
    /// and contains nothing. [`is_complete`](Self::is_complete) reports the
    /// timeline and says nothing about this; [`is_usable`](Self::is_usable)
    /// requires both. See
    /// [`Messwertstatus::is_usable`](crate::current::Messwertstatus::is_usable).
    pub unusable: Vec<usize>,
    /// `true` when the entries are **not** listed in ascending start order.
    ///
    /// Not an error — the schema imposes no order — but a series that arrives
    /// sorted can be consumed as a stream, and one that does not cannot.
    pub out_of_order: bool,
}

impl CoverageReport {
    /// Whether the series covers its reference span exactly once, end to end.
    ///
    /// `true` requires a reference span, no gaps, no overlaps, no unplaced
    /// entries and no wrong-length ones. Order is not part of it — the schema
    /// imposes none.
    ///
    /// This is a claim about the **timeline**, not the readings on it: a series
    /// whose every entry is `FEHLT` is complete by this measure and empty by any
    /// other. [`is_usable`](Self::is_usable) is the stronger claim.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.reference.is_some()
            && self.gaps.is_empty()
            && self.overlaps.is_empty()
            && self.unplaced.is_empty()
            && self.wrong_length.is_empty()
    }

    /// Whether the series is [complete](Self::is_complete) **and** every entry
    /// carries a usable value.
    ///
    /// The check to gate on before consuming a series as data. It does not
    /// require the values to be *measured*: `ERSATZWERT` and `PROGNOSEWERT` are
    /// legitimate readings that settlement rules treat differently, so
    /// distinguishing them is a decision this leaves to you — see
    /// [`Messwertstatus::is_measured`](crate::current::Messwertstatus::is_measured).
    #[must_use]
    pub fn is_usable(&self) -> bool {
        self.is_complete() && self.unusable.is_empty()
    }

    /// The fraction of [`reference`](Self::reference) that is covered, in
    /// `0.0 ..= 1.0`.
    ///
    /// `None` when there is no reference span, or when it is empty. An overlap
    /// counts once, so this never exceeds `1.0`.
    // Both counts are nanoseconds under the same span, so the quotient is well
    // inside what `f64` represents exactly for any period a market message covers.
    #[allow(clippy::cast_precision_loss)]
    #[must_use]
    pub fn coverage_ratio(&self) -> Option<f64> {
        let reference = self.reference.as_ref()?;
        let span = (reference.end - reference.start).whole_nanoseconds();
        if span <= 0 {
            return None;
        }
        Some(self.covered.whole_nanoseconds() as f64 / span as f64)
    }

    /// The total length of every gap.
    #[must_use]
    pub fn missing(&self) -> Duration {
        self.gaps
            .iter()
            .map(|g| g.end - g.start)
            .fold(Duration::ZERO, |a, b| a + b)
    }
}

// ─── The trait ───────────────────────────────────────────────────────────────

/// Timeline operations shared by BO4E's two time-series Geschäftsobjekte.
///
/// Implemented for [`Lastgang`] and [`Zeitreihe`]. Implementing it takes three
/// methods — [`werte`](Self::werte),
/// [`expected_interval`](Self::expected_interval) and
/// [`einheit`](Self::einheit) — and everything else follows, so a downstream
/// type that wraps a series of its own can opt in the same way it can into
/// [`Bo4eStrict`](crate::Bo4eStrict).
///
/// # Cost
///
/// [`placed`](Self::placed) parses each entry's `Zeitraum` as it goes and
/// allocates nothing; [`audit`](Self::audit) sorts a `Vec` of the resulting
/// ranges, so it is `O(n log n)` in the number of entries and allocates once.
/// Parsing the two `format: "time"` strings dominates, and each entry is parsed
/// exactly once — [`audit`](Self::audit) derives its own reference span from the
/// ranges it has already placed rather than calling [`span`](Self::span) first.
/// A year of quarter-hours (35 040 entries) audits in a few milliseconds;
/// `benches/timeseries_perf.rs` measures it.
pub trait Bo4eTimeSeries {
    /// The entries, or an empty slice when the field is absent.
    fn werte(&self) -> &[Zeitreihenwert];

    /// The interval length every entry is expected to have, when the type
    /// declares one.
    ///
    /// `Lastgang` states it in `zeitIntervallLaenge`; `Zeitreihe` does not state
    /// it at all and returns `None`, which switches
    /// [`CoverageReport::wrong_length`] off.
    fn expected_interval(&self) -> Option<Duration>;

    /// The unit the values carry, when the type states one.
    ///
    /// This decides whether [`sum`](Self::sum) or [`integrate`](Self::integrate)
    /// is the meaningful aggregate — see
    /// [`Mengeneinheit::is_extensive`](crate::current::Mengeneinheit::is_extensive).
    fn einheit(&self) -> Option<crate::generated::v202607::Mengeneinheit>;

    /// Every entry that resolves to an interval, in the order they are listed.
    ///
    /// Entries that do not resolve are skipped silently; [`audit`](Self::audit)
    /// is where they are reported. The iterator borrows the series and allocates
    /// nothing.
    fn placed(&self) -> impl Iterator<Item = PlacedValue<'_>> {
        self.werte()
            .iter()
            .enumerate()
            .filter_map(|(index, value)| {
                let range = place(value).ok()?;
                Some(PlacedValue {
                    index,
                    range,
                    value,
                })
            })
    }

    /// The span from the earliest start to the latest end across all entries.
    ///
    /// `None` when no entry resolves to an interval.
    fn span(&self) -> Option<Range<OffsetDateTime>> {
        let mut iter = self.placed();
        let first = iter.next()?.range;
        Some(iter.fold(first, |acc, p| {
            acc.start.min(p.range.start)..acc.end.max(p.range.end)
        }))
    }

    /// Audits the series against the span its own entries cover.
    ///
    /// Use [`audit_over`](Self::audit_over) instead when you know the period the
    /// series was *supposed* to cover: a series missing its whole last day looks
    /// complete against itself.
    fn audit(&self) -> CoverageReport {
        audit_inner(self.werte(), self.expected_interval(), None)
    }

    /// Audits the series against an explicit reference period, `[start, end)`.
    ///
    /// Entries outside `reference` still count as coverage where they overlap it
    /// and are ignored where they do not, so an extra day at the front does not
    /// mask a missing day at the back.
    fn audit_over(&self, reference: Range<OffsetDateTime>) -> CoverageReport {
        audit_inner(self.werte(), self.expected_interval(), Some(reference))
    }

    /// Whether **every** entry carries a usable value.
    ///
    /// A shortcut past [`audit`](Self::audit) when the timeline is not in
    /// question — reading the statuses costs one pass and no allocation. Unlike
    /// [`CoverageReport::unusable`], this covers entries that have no resolvable
    /// interval too: it is a question about the readings, not about the timeline.
    ///
    /// An entry that states no `status` is taken at its word, the way
    /// [`sum`](Self::sum) takes it.
    fn all_values_usable(&self) -> bool {
        self.werte()
            .iter()
            .all(|v| v.status.is_none_or(|s| s.is_usable()))
    }

    /// The plain sum of every value.
    ///
    /// `None` when any entry omits its `wert`, when the sum overflows, or when
    /// the series' [`einheit`](Self::einheit) is one that must **not** be summed
    /// — a power, a frequency, a percentage. Summing 96 quarter-hourly kW
    /// readings produces a number with no physical meaning; use
    /// [`integrate`](Self::integrate) for those.
    ///
    /// A series that states no unit is summed on the caller's word, since
    /// nothing contradicts it. An entry whose `status` marks it
    /// [unusable](crate::current::Messwertstatus::is_usable) also yields `None`:
    /// a `FEHLT` slot carrying `0` is an absence, not a zero, and adding it in
    /// understates the total without saying so.
    #[cfg(feature = "decimal")]
    #[cfg_attr(docsrs, doc(cfg(feature = "decimal")))]
    fn sum(&self) -> Option<rust_decimal::Decimal> {
        if self.einheit().is_some_and(|u| !u.is_extensive()) {
            return None;
        }
        self.werte()
            .iter()
            .try_fold(rust_decimal::Decimal::ZERO, |acc, v| {
                if v.status.is_some_and(|s| !s.is_usable()) {
                    return None;
                }
                acc.checked_add(v.wert?)
            })
    }

    /// Integrates the series over time: `Σ value × interval_length_in_hours`.
    ///
    /// This is the step from a `Lastgang` carrying **power** to the **energy** it
    /// represents — 400 kW held for a quarter of an hour is 100 kWh — and the
    /// unit of the result follows
    /// [`Mengeneinheit::energy_unit`](crate::current::Mengeneinheit::energy_unit).
    ///
    /// Only entries that resolve to an interval contribute, so
    /// [`audit`](Self::audit) first if a silently dropped row would matter.
    ///
    /// `None` when the series' [`einheit`](Self::einheit) is not one that
    /// integrates into anything — see [`integrated_unit`](Self::integrated_unit)
    /// — when an entry that *does* resolve omits its `wert`, when its `status`
    /// marks it [unusable](crate::current::Messwertstatus::is_usable), or when
    /// the arithmetic overflows.
    ///
    /// For a **stated** unit, exactly one of [`sum`](Self::sum) and `integrate`
    /// answers: `KWH` sums and does not integrate, `KW` integrates and does not
    /// sum. A series that states no unit answers to both, on the caller's word.
    #[cfg(feature = "decimal")]
    #[cfg_attr(docsrs, doc(cfg(feature = "decimal")))]
    fn integrate(&self) -> Option<rust_decimal::Decimal> {
        if self.einheit().is_some_and(|u| u.energy_unit().is_none()) {
            return None;
        }
        self.placed()
            .try_fold(rust_decimal::Decimal::ZERO, |acc, p| {
                if p.value.status.is_some_and(|s| !s.is_usable()) {
                    return None;
                }
                let hours = crate::units::duration_to_hours(p.range.end - p.range.start)?;
                acc.checked_add(p.value.wert?.checked_mul(hours)?)
            })
    }

    /// The unit [`integrate`](Self::integrate) produces: the energy counterpart
    /// of the series' own unit, per
    /// [`Mengeneinheit::energy_unit`](crate::current::Mengeneinheit::energy_unit).
    ///
    /// `KW` → `KWH`, `MW` → `MWH`, `KVAR` → `KVARH`. `None` when the series
    /// states no unit — the integral is then a bare number the caller has to
    /// label — or when its unit is not a power, in which case `integrate` itself
    /// answers `None`.
    ///
    /// ```
    /// # #[cfg(all(feature = "versioned", feature = "time", feature = "decimal"))] {
    /// use rubo4e::current::{Lastgang, Menge, Mengeneinheit};
    /// use rubo4e::timeseries::Bo4eTimeSeries;
    ///
    /// let lg = Lastgang {
    ///     messgroesse: Some(Mengeneinheit::Kw),
    ///     ..Lastgang::new(Menge::default())
    /// };
    /// assert_eq!(lg.integrated_unit(), Some(Mengeneinheit::Kwh));
    /// # }
    /// ```
    fn integrated_unit(&self) -> Option<crate::generated::v202607::Mengeneinheit> {
        self.einheit()?.energy_unit()
    }
}

/// Shared body of [`Bo4eTimeSeries::audit`] and [`Bo4eTimeSeries::audit_over`].
///
/// `reference` of `None` means "measure the series against itself", and the span
/// is taken from the very ranges this function places — placing them a second
/// time through [`Bo4eTimeSeries::span`] would double the parse cost, which is
/// what dominates a 35 040-entry year.
fn audit_inner(
    werte: &[Zeitreihenwert],
    expected_interval: Option<Duration>,
    reference: Option<Range<OffsetDateTime>>,
) -> CoverageReport {
    let mut report = CoverageReport::default();

    // Place every entry, recording the ones that will not go.
    let mut ranges: Vec<Range<OffsetDateTime>> = Vec::with_capacity(werte.len());
    let mut last_start: Option<OffsetDateTime> = None;
    for (index, value) in werte.iter().enumerate() {
        match place(value) {
            Err(reason) => report.unplaced.push(UnplacedValue { index, reason }),
            Ok(range) => {
                if value.status.is_some_and(|s| !s.is_usable()) {
                    report.unusable.push(index);
                }
                if last_start.is_some_and(|prev| range.start < prev) {
                    report.out_of_order = true;
                }
                last_start = Some(range.start);
                if let Some(expected) = expected_interval {
                    if range.end - range.start != expected {
                        report.wrong_length.push(index);
                    }
                }
                ranges.push(range);
            }
        }
    }

    let reference = reference.or_else(|| {
        let first = ranges.first()?.clone();
        Some(
            ranges
                .iter()
                .fold(first, |acc, r| acc.start.min(r.start)..acc.end.max(r.end)),
        )
    });
    let Some(reference) = reference else {
        return report;
    };
    report.reference = Some(reference.clone());

    // Clip to the reference span and sweep left to right. Sorting by start makes
    // the gap pass and the overlap pass one shared linear scan. An empty
    // interval covers nothing and overlaps nothing, so it never enters the sweep.
    ranges.retain(|r| !r.is_empty() && r.start < reference.end && r.end > reference.start);
    for r in &mut ranges {
        r.start = r.start.max(reference.start);
        r.end = r.end.min(reference.end);
    }
    ranges.sort_unstable_by_key(|r| (r.start, r.end));

    let mut cursor = reference.start;
    for r in &ranges {
        if r.start > cursor {
            report.gaps.push(cursor..r.start);
            report.covered += r.end - r.start;
        } else if r.end > cursor {
            // Partially covered already: only the new tail is fresh ground.
            if r.start < cursor {
                report.overlaps.push(r.start..cursor);
            }
            report.covered += r.end - cursor;
        } else {
            // Wholly inside what is already covered.
            report.overlaps.push(r.start..r.end);
        }
        cursor = cursor.max(r.end);
    }
    if cursor < reference.end {
        report.gaps.push(cursor..reference.end);
    }

    merge_adjacent(&mut report.overlaps);
    report
}

/// Resolves one entry's `Zeitraum` to a half-open interval.
fn place(value: &Zeitreihenwert) -> Result<Range<OffsetDateTime>, UnplacedReason> {
    let zeitraum = value
        .zeitraum
        .as_ref()
        .ok_or(UnplacedReason::MissingZeitraum)?;
    let range = zeitraum
        .as_instant_range()
        .ok_or(UnplacedReason::NotAnInstantRange)?
        .map_err(UnplacedReason::Malformed)?;
    if range.end < range.start {
        return Err(UnplacedReason::Reversed);
    }
    Ok(range)
}

/// Coalesces touching or overlapping ranges in an already-sorted list.
fn merge_adjacent(ranges: &mut Vec<Range<OffsetDateTime>>) {
    if ranges.len() < 2 {
        return;
    }
    ranges.sort_unstable_by_key(|r| (r.start, r.end));
    let mut merged: Vec<Range<OffsetDateTime>> = Vec::with_capacity(ranges.len());
    for r in ranges.iter().cloned() {
        match merged.last_mut() {
            Some(last) if r.start <= last.end => last.end = last.end.max(r.end),
            _ => merged.push(r),
        }
    }
    *ranges = merged;
}

// ─── Impls ───────────────────────────────────────────────────────────────────

impl Bo4eTimeSeries for Lastgang {
    fn werte(&self) -> &[Zeitreihenwert] {
        self.werte.as_deref().unwrap_or_default()
    }

    /// From `zeitIntervallLaenge`, the one field the `Lastgang` schema marks
    /// `required`.
    ///
    /// `None` when its unit is not an exact duration — a `MONAT` interval has no
    /// fixed length, so no entry can be measured against it.
    fn expected_interval(&self) -> Option<Duration> {
        #[cfg(feature = "decimal")]
        {
            self.zeit_intervall_laenge.as_duration()
        }
        #[cfg(not(feature = "decimal"))]
        {
            // Without `decimal` the count is a string; the unit alone is not the
            // interval, so nothing is claimed.
            None
        }
    }

    /// From `messgroesse` — "Definition der gemessenen Größe anhand ihrer
    /// Einheit".
    fn einheit(&self) -> Option<crate::generated::v202607::Mengeneinheit> {
        self.messgroesse
    }
}

impl Bo4eTimeSeries for Zeitreihe {
    fn werte(&self) -> &[Zeitreihenwert] {
        self.werte.as_deref().unwrap_or_default()
    }

    /// Always `None`: `Zeitreihe` declares no interval length, so an irregular
    /// series is not a defect there.
    fn expected_interval(&self) -> Option<Duration> {
        None
    }

    /// From `einheit` — "Alle Werte in der Tabelle haben die Einheit, die hier
    /// angegeben ist".
    fn einheit(&self) -> Option<crate::generated::v202607::Mengeneinheit> {
        self.einheit
    }
}

// ─── One shape for every interval series ─────────────────────────────────────

/// One measurement interval, resolved: when it ran, what it measured, and
/// whether the value can be used.
///
/// BO4E carries interval data in three places that look nothing alike —
/// [`Lastgang`] and [`Zeitreihe`] hold a `Vec<Zeitreihenwert>` whose unit lives on
/// the enclosing BO, [`Energiemenge`] is a single `Menge` over a `Zeitraum` — and
/// a consumer that wants "a series of readings" ends up writing the third mapping
/// by hand. This is that mapping, written once: [`Bo4eIntervals`] produces it from
/// all three, and [`to_zeitreihenwert`] / [`to_energiemenge`] write it back.
///
/// It borrows only the OBIS code, so producing a year of quarter-hours allocates
/// nothing.
///
/// ```
/// # #[cfg(all(feature = "versioned", feature = "time", feature = "decimal"))] {
/// use rubo4e::current::{Energiemenge, Menge, Mengeneinheit, Zeitraum};
/// use rubo4e::timeseries::Bo4eIntervals;
/// use rust_decimal::Decimal;
/// use time::macros::datetime;
///
/// let menge = Energiemenge {
///     menge: Some(Menge {
///         wert: Some(Decimal::from(120)),
///         einheit: Some(Mengeneinheit::Kwh),
///         ..Default::default()
///     }),
///     zeitraum: Some(Zeitraum::from_instants(
///         datetime!(2026-01-01 00:00 +01:00),
///         datetime!(2026-01-01 01:00 +01:00),
///     )),
///     ..Default::default()
/// };
///
/// let readings: Vec<_> = menge.intervals().collect();
/// assert_eq!(readings.len(), 1);
/// assert_eq!(readings[0].wert, Some(Decimal::from(120)));
/// assert_eq!(readings[0].duration(), time::Duration::hours(1));
/// # }
/// ```
///
/// [`Lastgang`]: crate::current::Lastgang
/// [`Zeitreihe`]: crate::current::Zeitreihe
/// [`Energiemenge`]: crate::current::Energiemenge
/// [`to_zeitreihenwert`]: IntervalReading::to_zeitreihenwert
/// [`to_energiemenge`]: IntervalReading::to_energiemenge
#[cfg(feature = "decimal")]
#[cfg_attr(docsrs, doc(cfg(feature = "decimal")))]
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct IntervalReading<'a> {
    /// Index of the entry this came from, in the source's own list. Always `0`
    /// for an [`Energiemenge`](crate::current::Energiemenge), which holds one.
    pub index: usize,
    /// The measurement interval, `[start, end)` — the same half-open convention
    /// [`PlacedValue`] uses.
    pub range: Range<OffsetDateTime>,
    /// The measured value. `None` where the source states an interval but no
    /// number, which BO4E permits and a `FEHLT` slot routinely does.
    pub wert: Option<rust_decimal::Decimal>,
    /// The unit, taken from the enclosing BO for a `Lastgang` / `Zeitreihe` and
    /// from the `Menge` itself for an `Energiemenge`.
    pub einheit: Option<crate::generated::v202607::Mengeneinheit>,
    /// How the value is to be read — measured, substituted, missing.
    pub status: Option<crate::generated::v202607::Messwertstatus>,
    /// The reason behind the status, where one is stated.
    pub statuszusatz: Option<crate::generated::v202607::Messwertstatuszusatz>,
    /// The OBIS code of the register the series belongs to, where the source
    /// states one.
    pub obis_kennzahl: Option<&'a crate::identifiers::ObisCode>,
}

#[cfg(feature = "decimal")]
impl IntervalReading<'static> {
    /// Builds a reading from the three things it cannot be read without.
    ///
    /// The entry point for the other direction — a household or metering system
    /// that has its own intervals and wants BO4E out the far end. Chain
    /// [`with_status`](Self::with_status) and friends for the rest, then
    /// [`to_zeitreihenwert`](Self::to_zeitreihenwert) or
    /// [`Zeitreihe::from_intervals`](crate::current::Zeitreihe::from_intervals).
    ///
    /// ```
    /// # #[cfg(all(feature = "versioned", feature = "time", feature = "decimal"))] {
    /// use rubo4e::current::{Mengeneinheit, Messwertstatus, Zeitreihe};
    /// use rubo4e::timeseries::IntervalReading;
    /// use rust_decimal::Decimal;
    /// use time::macros::datetime;
    ///
    /// let start = datetime!(2026-03-01 00:00 +01:00);
    /// let readings = (0..4).map(|i| {
    ///     let from = start + time::Duration::minutes(15 * i);
    ///     IntervalReading::new(
    ///         from..from + time::Duration::minutes(15),
    ///         Some(Decimal::from(100 + i)),
    ///         Some(Mengeneinheit::Kwh),
    ///     )
    ///     .with_status(Messwertstatus::Abgelesen)
    /// });
    ///
    /// let zeitreihe = Zeitreihe::from_intervals(readings);
    /// assert_eq!(zeitreihe.einheit, Some(Mengeneinheit::Kwh));
    /// assert_eq!(zeitreihe.werte.as_ref().map(Vec::len), Some(4));
    /// # }
    /// ```
    #[must_use]
    pub const fn new(
        range: Range<OffsetDateTime>,
        wert: Option<rust_decimal::Decimal>,
        einheit: Option<crate::generated::v202607::Mengeneinheit>,
    ) -> Self {
        Self {
            index: 0,
            range,
            wert,
            einheit,
            status: None,
            statuszusatz: None,
            obis_kennzahl: None,
        }
    }
}

#[cfg(feature = "decimal")]
impl<'a> IntervalReading<'a> {
    /// Sets the status.
    #[must_use]
    pub fn with_status(mut self, status: crate::generated::v202607::Messwertstatus) -> Self {
        self.status = Some(status);
        self
    }

    /// Sets the status qualifier.
    #[must_use]
    pub fn with_statuszusatz(
        mut self,
        statuszusatz: crate::generated::v202607::Messwertstatuszusatz,
    ) -> Self {
        self.statuszusatz = Some(statuszusatz);
        self
    }

    /// Attaches the OBIS code of the register this reading belongs to.
    #[must_use]
    pub fn with_obis(mut self, obis: &'a crate::identifiers::ObisCode) -> Self {
        self.obis_kennzahl = Some(obis);
        self
    }

    /// Sets the index this reading reports.
    #[must_use]
    pub const fn with_index(mut self, index: usize) -> Self {
        self.index = index;
        self
    }

    /// How long the interval ran.
    #[must_use]
    pub fn duration(&self) -> Duration {
        self.range.end - self.range.start
    }

    /// Whether the value may be used in arithmetic.
    ///
    /// A reading with no `status` is taken at its word, the way
    /// [`Bo4eTimeSeries::sum`] takes it. A reading with no `wert` is not usable
    /// whatever its status says — there is nothing to use.
    #[must_use]
    pub fn is_usable(&self) -> bool {
        self.wert.is_some() && self.status.is_none_or(|s| s.is_usable())
    }

    /// The energy this interval represents, and the unit it is in.
    ///
    /// The one call the two BO4E spellings of "how much energy" collapse into:
    ///
    /// | The reading's unit is | Result |
    /// |---|---|
    /// | an energy (`KWH`, `MWH`, `KVARH`) | the value unchanged |
    /// | a power (`KW`, `MW`, `KVAR`) | value × interval length in hours, in the matching energy unit |
    /// | anything else, or absent | `None` |
    ///
    /// `None` too when the reading is not [usable](Self::is_usable) or when the
    /// arithmetic overflows. A `FEHLT` slot carrying `0` is an absence, not a
    /// zero, and this refuses to launder it into one.
    ///
    /// ```
    /// # #[cfg(all(feature = "versioned", feature = "time", feature = "decimal"))] {
    /// use rubo4e::current::Mengeneinheit;
    /// use rubo4e::timeseries::IntervalReading;
    /// use rust_decimal::Decimal;
    /// use time::macros::datetime;
    ///
    /// let start = datetime!(2026-01-01 00:00 +01:00);
    /// let quarter = start..start + time::Duration::minutes(15);
    ///
    /// // 400 kW held for a quarter of an hour is 100 kWh.
    /// let power = IntervalReading::new(quarter.clone(), Some(Decimal::from(400)), Some(Mengeneinheit::Kw));
    /// assert_eq!(power.energy(), Some((Decimal::from(100), Mengeneinheit::Kwh)));
    ///
    /// // An energy reading is already the answer.
    /// let energy = IntervalReading::new(quarter, Some(Decimal::from(100)), Some(Mengeneinheit::Kwh));
    /// assert_eq!(energy.energy(), Some((Decimal::from(100), Mengeneinheit::Kwh)));
    /// # }
    /// ```
    #[must_use]
    pub fn energy(
        &self,
    ) -> Option<(
        rust_decimal::Decimal,
        crate::generated::v202607::Mengeneinheit,
    )> {
        if !self.is_usable() {
            return None;
        }
        let einheit = self.einheit?;
        let wert = self.wert?;
        if einheit.power_unit().is_some() {
            // Already an energy — `power_unit` is the inverse map, so a unit that
            // has one *is* the energy side of the pair.
            return Some((wert, einheit));
        }
        let energy_unit = einheit.energy_unit()?;
        let hours = crate::units::duration_to_hours(self.duration())?;
        Some((wert.checked_mul(hours)?, energy_unit))
    }

    /// The same reading expressed in `target`.
    ///
    /// `None` when the reading states no unit or no value, when `target` is a
    /// different [`Dimension`](crate::units::Dimension), or when the arithmetic
    /// overflows. This converts *within* a dimension — `MWH` → `KWH`; crossing
    /// from power to energy is [`energy`](Self::energy)'s job, because that needs
    /// the interval length as well as the value.
    #[must_use]
    pub fn converted_to(&self, target: crate::generated::v202607::Mengeneinheit) -> Option<Self> {
        let source = self.einheit?;
        if source.dimension()? != target.dimension()? {
            return None;
        }
        let wert = self
            .wert?
            .checked_mul(source.factor_to_base()?)?
            .checked_div(target.factor_to_base()?)?;
        Some(Self {
            wert: Some(wert),
            einheit: Some(target),
            range: self.range.clone(),
            ..*self
        })
    }

    /// Writes the reading back as a [`Zeitreihenwert`].
    ///
    /// The unit does not travel: `Zeitreihenwert` has no unit field, because a
    /// `Lastgang` and a `Zeitreihe` state it once for the whole series. Use
    /// [`Zeitreihe::from_intervals`](crate::current::Zeitreihe::from_intervals) or
    /// [`Lastgang::from_intervals`](crate::current::Lastgang::from_intervals),
    /// which carry it up to where it belongs.
    #[must_use]
    pub fn to_zeitreihenwert(&self) -> Zeitreihenwert {
        Zeitreihenwert {
            wert: self.wert,
            zeitraum: Some(crate::generated::v202607::Zeitraum::from_instants(
                self.range.start,
                self.range.end,
            )),
            status: self.status,
            statuszusatz: self.statuszusatz,
            ..Default::default()
        }
    }

    /// Writes the reading back as an [`Energiemenge`](crate::current::Energiemenge).
    ///
    /// Everything survives here — the unit rides along on the `Menge`, and the
    /// OBIS code on the BO — which is what makes `Energiemenge` the lossless
    /// single-interval form. The status does not: `Energiemenge` has no field for
    /// it.
    #[must_use]
    pub fn to_energiemenge(&self) -> crate::generated::v202607::Energiemenge {
        crate::generated::v202607::Energiemenge {
            menge: Some(crate::generated::v202607::Menge {
                wert: self.wert,
                einheit: self.einheit,
                ..Default::default()
            }),
            zeitraum: Some(crate::generated::v202607::Zeitraum::from_instants(
                self.range.start,
                self.range.end,
            )),
            obis_kennzahl: self.obis_kennzahl.cloned(),
            ..Default::default()
        }
    }
}

/// Reads any BO4E interval series as a stream of [`IntervalReading`]s.
///
/// Implemented for [`Lastgang`], [`Zeitreihe`] and [`Energiemenge`] — the three
/// BO4E shapes that put a value on a stretch of time. Not for [`Zaehlwerk`],
/// whose `messwerte` are cumulative register *states* at an instant rather than
/// quantities over an interval; [`Zaehlwerk::readings`] is that shape's reader,
/// and turning two of them into a consumption is
/// [`consumption_between`](Zaehlwerk::consumption_between)'s job.
///
/// Entries that state no resolvable interval are skipped, exactly as in
/// [`Bo4eTimeSeries::placed`]; run [`audit`](Bo4eTimeSeries::audit) when a
/// silently dropped row would matter.
///
/// [`Lastgang`]: crate::current::Lastgang
/// [`Zeitreihe`]: crate::current::Zeitreihe
/// [`Energiemenge`]: crate::current::Energiemenge
/// [`Zaehlwerk`]: crate::current::Zaehlwerk
#[cfg(feature = "decimal")]
#[cfg_attr(docsrs, doc(cfg(feature = "decimal")))]
pub trait Bo4eIntervals {
    /// Every interval the source resolves, in the order it lists them.
    fn intervals(&self) -> impl Iterator<Item = IntervalReading<'_>>;

    /// Every interval whose value may be used — see
    /// [`IntervalReading::is_usable`].
    fn usable_intervals(&self) -> impl Iterator<Item = IntervalReading<'_>> {
        self.intervals().filter(IntervalReading::is_usable)
    }

    /// The total energy across every usable interval, and its unit.
    ///
    /// Sums [`IntervalReading::energy`], so a `Lastgang` in kW and a `Zeitreihe`
    /// in kWh both answer in kWh — the number an invoice bills, from either
    /// spelling.
    ///
    /// `None` when the series states no unit, when its unit is neither an energy
    /// nor a power, when two readings answer in different units, or when the
    /// arithmetic overflows. Unusable readings are skipped rather than counted as
    /// zero; [`audit`](Bo4eTimeSeries::audit) is where the gap they leave is
    /// reported.
    ///
    /// Also `None` — not `Some((0, unit))` — for a series with **no** usable
    /// reading at all, empty or entirely `FEHLT`. The unit comes from the readings
    /// that were totalled, and with none there is nothing to name the zero in.
    fn total_energy(
        &self,
    ) -> Option<(
        rust_decimal::Decimal,
        crate::generated::v202607::Mengeneinheit,
    )> {
        let mut total = rust_decimal::Decimal::ZERO;
        let mut unit = None;
        for reading in self.usable_intervals() {
            let (value, u) = reading.energy()?;
            match unit {
                None => unit = Some(u),
                Some(seen) if seen != u => return None,
                Some(_) => {}
            }
            total = total.checked_add(value)?;
        }
        Some((total, unit?))
    }
}

#[cfg(feature = "decimal")]
impl Bo4eIntervals for Lastgang {
    fn intervals(&self) -> impl Iterator<Item = IntervalReading<'_>> {
        let einheit = self.messgroesse;
        let obis = self.obis_kennzahl.as_ref();
        self.placed()
            .map(move |p| reading_from_placed(&p, einheit, obis))
    }
}

#[cfg(feature = "decimal")]
impl Bo4eIntervals for Zeitreihe {
    fn intervals(&self) -> impl Iterator<Item = IntervalReading<'_>> {
        let einheit = self.einheit;
        // `Zeitreihe` states no OBIS code — `messgroesse` there is a
        // `Messgroesse`, a physical quantity, not a register.
        self.placed()
            .map(move |p| reading_from_placed(&p, einheit, None))
    }
}

#[cfg(feature = "decimal")]
impl Bo4eIntervals for crate::generated::v202607::Energiemenge {
    /// At most one interval: an `Energiemenge` is a single `Menge` over a single
    /// `Zeitraum`.
    ///
    /// Empty when either is absent, or when the `zeitraum` does not resolve to
    /// two instants — a period given as bare dates has no time of day, and a
    /// reading needs one.
    fn intervals(&self) -> impl Iterator<Item = IntervalReading<'_>> {
        let range = self
            .zeitraum
            .as_ref()
            .and_then(crate::generated::v202607::Zeitraum::as_instant_range)
            .and_then(Result::ok)
            .filter(|r| r.end >= r.start);
        let menge = self.menge.as_ref();
        range
            .zip(menge)
            .map(|(range, menge)| IntervalReading {
                index: 0,
                range,
                wert: menge.wert,
                einheit: menge.einheit,
                status: None,
                statuszusatz: None,
                obis_kennzahl: self.obis_kennzahl.as_ref(),
            })
            .into_iter()
    }
}

// ─── …and back again ─────────────────────────────────────────────────────────

/// What a stream of readings contributes to the BO that will carry it.
///
/// A `Lastgang` and a `Zeitreihe` state the unit once, for the whole series, so
/// building either means folding the per-reading unit up to the BO — and both
/// spell the field differently (`messgroesse`, `einheit`) while meaning the same
/// thing. This is that fold, once.
#[cfg(feature = "decimal")]
struct CollectedSeries {
    werte: Vec<Zeitreihenwert>,
    einheit: Option<crate::generated::v202607::Mengeneinheit>,
    obis_kennzahl: Option<crate::identifiers::ObisCode>,
}

/// Folds readings into the entries plus the two facts the enclosing BO states.
///
/// The unit is the first one any reading states; a reading in a different unit is
/// converted into it where the two share a [`Dimension`](crate::units::Dimension),
/// and carried through unconverted where they do not — dropping the row silently
/// would be worse than a series [`audit`](Bo4eTimeSeries::audit) can flag.
#[cfg(feature = "decimal")]
fn collect_series<'a>(readings: impl IntoIterator<Item = IntervalReading<'a>>) -> CollectedSeries {
    let mut einheit = None;
    let mut obis = None;
    let mut werte = Vec::new();
    for reading in readings {
        if obis.is_none() {
            obis = reading.obis_kennzahl.cloned();
        }
        if einheit.is_none() {
            einheit = reading.einheit;
        }
        werte.push(match (einheit, reading.einheit) {
            (Some(target), Some(source)) if source != target => reading
                .converted_to(target)
                .unwrap_or(reading)
                .to_zeitreihenwert(),
            _ => reading.to_zeitreihenwert(),
        });
    }
    CollectedSeries {
        werte,
        einheit,
        obis_kennzahl: obis,
    }
}

#[cfg(feature = "decimal")]
impl Zeitreihe {
    /// Builds a `Zeitreihe` from a stream of readings.
    ///
    /// The unit is taken from the first reading that states one and written to
    /// `einheit`, where BO4E puts it — *"Alle Werte in der Tabelle haben die
    /// Einheit, die hier angegeben ist"*. A reading in a **different** unit is
    /// converted into it where the two share a
    /// [`Dimension`](crate::units::Dimension), and carried through unconverted
    /// where they do not, because dropping the row silently would be worse than
    /// a series [`audit`](Bo4eTimeSeries::audit) can flag.
    ///
    /// Every other field is left at its default: a `Zeitreihe` says what it is
    /// through `bezeichnung`, `medium` and `messart`, and this cannot know them.
    #[must_use]
    pub fn from_intervals<'a>(
        readings: impl IntoIterator<Item = IntervalReading<'a>>,
    ) -> Zeitreihe {
        let series = collect_series(readings);
        Zeitreihe {
            einheit: series.einheit,
            werte: Some(series.werte),
            ..Default::default()
        }
    }
}

#[cfg(feature = "decimal")]
impl Lastgang {
    /// Builds a `Lastgang` from a stream of readings, over a stated interval
    /// length.
    ///
    /// `zeit_intervall_laenge` is the one field the `Lastgang` schema marks
    /// `required`, and nothing here can infer it: a series of four quarter-hours
    /// and a series with three of them missing look identical. Pass it, and
    /// [`audit`](Bo4eTimeSeries::audit) will then measure the readings against it.
    ///
    /// The unit goes to `messgroesse`, following the same rule as
    /// [`Zeitreihe::from_intervals`], and the first OBIS code any reading carries
    /// goes to `obisKennzahl`.
    #[must_use]
    pub fn from_intervals<'a>(
        zeit_intervall_laenge: crate::generated::v202607::Menge,
        readings: impl IntoIterator<Item = IntervalReading<'a>>,
    ) -> Lastgang {
        let series = collect_series(readings);
        Lastgang {
            messgroesse: series.einheit,
            obis_kennzahl: series.obis_kennzahl,
            werte: Some(series.werte),
            ..Lastgang::new(zeit_intervall_laenge)
        }
    }
}

/// Shared body of the `Lastgang` and `Zeitreihe` impls.
#[cfg(feature = "decimal")]
fn reading_from_placed<'a>(
    placed: &PlacedValue<'a>,
    einheit: Option<crate::generated::v202607::Mengeneinheit>,
    obis: Option<&'a crate::identifiers::ObisCode>,
) -> IntervalReading<'a> {
    IntervalReading {
        index: placed.index,
        range: placed.range.clone(),
        wert: placed.value.wert,
        einheit,
        status: placed.value.status,
        statuszusatz: placed.value.statuszusatz,
        obis_kennzahl: obis,
    }
}

// ─── Register readings: the other time-series shape ──────────────────────────

/// One usable reading off a [`Zaehlwerk`], resolved to an instant and a value.
///
/// A register reading is a **cumulative meter state**, not a quantity consumed:
/// `Messwert.wert` is what the display said at `zeitpunkt`. The consumption is
/// the difference between two of them — see
/// [`consumption_between`](Zaehlwerk::consumption_between).
#[cfg(feature = "decimal")]
#[cfg_attr(docsrs, doc(cfg(feature = "decimal")))]
#[derive(Debug, Clone, PartialEq)]
pub struct Reading<'a> {
    /// Index of this reading in the register's `messwerte` vector.
    pub index: usize,
    /// When the meter was read.
    pub at: OffsetDateTime,
    /// The register state, in [`Zaehlwerk::einheit`] where the register states
    /// one and the reading could be converted to it.
    pub value: rust_decimal::Decimal,
    /// The reading itself, for its status and any extension data.
    pub source: &'a Messwert,
}

/// Why a consumption could not be computed from a register's readings.
#[cfg(feature = "decimal")]
#[cfg_attr(docsrs, doc(cfg(feature = "decimal")))]
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum ConsumptionError {
    /// Fewer than two usable readings — a consumption needs a pair.
    #[error("a consumption needs two readings, and this register has {count}")]
    TooFewReadings {
        /// How many usable readings there are.
        count: usize,
    },

    /// A reading is lower than the one before it, and the register states no
    /// `vorkommastelle`, so there is no way to tell a wrap-around from an error.
    ///
    /// See [`Zaehlwerk::register_capacity`].
    #[error(
        "reading {index} fell from {from} to {to}, and the register states no \
         vorkommastelle, so a wrap-around cannot be told from a fault"
    )]
    DecreasedWithoutRegisterWidth {
        /// Index of the lower reading.
        index: usize,
        /// The preceding, higher state.
        from: rust_decimal::Decimal,
        /// The state that fell.
        to: rust_decimal::Decimal,
    },

    /// A reading is marked `Z78_GERAETEWECHSEL`: the meter was swapped, so the
    /// register started again from a state unrelated to the previous one.
    ///
    /// The difference across that boundary is not a consumption at whatever the
    /// arithmetic says, so it is refused rather than guessed. Split the series at
    /// the exchange and sum the halves.
    #[error("reading {index} is marked Z78_GERAETEWECHSEL; split the series there")]
    MeterExchange {
        /// Index of the reading that carries the marker.
        index: usize,
    },

    /// A reading's unit is not the register's, and the two do not convert.
    #[error("reading {index} is in a unit that does not convert to the register's")]
    IncompatibleUnit {
        /// Index of the offending reading.
        index: usize,
    },

    /// The arithmetic left `Decimal`'s range.
    #[error("the consumption arithmetic overflowed")]
    Overflow,
}

#[cfg(feature = "decimal")]
impl Zaehlwerk {
    /// The register's usable readings, in chronological order.
    ///
    /// Only readings that state both a `zeitpunkt` and a numeric `wert` appear,
    /// and only ones whose status does not mark them
    /// [unusable](crate::current::Messwertstatus::is_usable). Values are converted
    /// into [`einheit`](crate::current::Zaehlwerk::einheit) where the register
    /// states one; a reading whose unit does not convert is dropped here and
    /// named by [`total_consumption`](Self::total_consumption).
    ///
    /// `messwerte` is a bag, not a sequence — nothing in the schema orders it —
    /// so this sorts, which is the one allocation on the path.
    #[must_use]
    pub fn readings(&self) -> Vec<Reading<'_>> {
        let target = self.einheit;
        let mut out: Vec<Reading<'_>> = self
            .messwerte
            .as_deref()
            .unwrap_or_default()
            .iter()
            .enumerate()
            .filter_map(|(index, m)| {
                if m.messwertstatus.is_some_and(|s| !s.is_usable()) {
                    return None;
                }
                let value = reading_value(m, target)?;
                Some(Reading {
                    index,
                    at: m.zeitpunkt?,
                    value,
                    source: m,
                })
            })
            .collect();
        out.sort_by_key(|r| r.at);
        out
    }

    /// The state at which this register wraps back to zero: `10^vorkommastelle`.
    ///
    /// A six-digit register runs to `999999` and then reads `000000`, so its
    /// capacity is `1_000_000` — the amount a wrapped difference is short by.
    ///
    /// `None` when the register states no `vorkommastelle`, or states one too
    /// large to represent, in which case a decrease cannot be resolved.
    ///
    /// ```
    /// # #[cfg(all(feature = "versioned", feature = "time", feature = "decimal"))] {
    /// use rubo4e::current::Zaehlwerk;
    /// use rust_decimal::Decimal;
    ///
    /// let zw = Zaehlwerk { vorkommastelle: Some(6), ..Default::default() };
    /// assert_eq!(zw.register_capacity(), Some(Decimal::from(1_000_000)));
    /// assert_eq!(Zaehlwerk::default().register_capacity(), None);
    /// # }
    /// ```
    #[must_use]
    pub fn register_capacity(&self) -> Option<rust_decimal::Decimal> {
        let digits = u32::try_from(self.vorkommastelle?).ok()?;
        // `Decimal` holds 28 significant digits; a wider register is not one this
        // arithmetic can resolve, and pretending otherwise would silently round.
        if digits > 28 {
            return None;
        }
        // `10u128.pow` rather than a `Decimal` power: `Decimal::checked_powu`
        // lives behind rust_decimal's `maths` feature, and this needs one exact
        // power of ten, not a general exponentiation.
        //
        // `try_from`, not the `From` impl clippy points at: `Decimal::from(u128)`
        // **panics** above `Decimal`'s range (`rust_decimal` unwraps an internal
        // `Option`), and `u128` reaches 3.4e38 against `Decimal`'s 7.9e28. The
        // `digits > 28` guard above makes it unreachable today; the fallible
        // conversion is what keeps it unreachable if that guard is ever widened.
        // Same reasoning as the visitors in `crate::decimal_serde`.
        #[allow(clippy::unnecessary_fallible_conversions)]
        rust_decimal::Decimal::try_from(10u128.checked_pow(digits)?).ok()
    }

    /// The consumption between two register states, exactly as the schema
    /// defines it.
    ///
    /// BO4E states the formula on `wandlerfaktor` itself: *"Mit diesem Faktor
    /// wird eine Zählerstandsdifferenz multipliziert, um zum eigentlichen
    /// Verbrauch im Zeitraum zu kommen."* So:
    ///
    /// ```text
    /// consumption = (to − from) × wandlerfaktor
    /// ```
    ///
    /// with two corrections the bare subtraction gets wrong:
    ///
    /// - **A wrap-around.** A six-digit register going `999998 → 000012` has not
    ///   consumed `−999986`; it has consumed `14`. When `to < from` and the
    ///   register states a [`register_capacity`](Self::register_capacity), the
    ///   capacity is added back, once.
    /// - **No stated width.** When `to < from` and there is no
    ///   `vorkommastelle`, a wrap-around cannot be told from a fault, so this
    ///   refuses rather than picking one.
    ///
    /// An absent `wandlerfaktor` is **1**: a directly-measuring meter has no
    /// transformer and states none. Check the field yourself where that
    /// distinction matters.
    ///
    /// # Errors
    ///
    /// [`ConsumptionError::DecreasedWithoutRegisterWidth`] or
    /// [`ConsumptionError::Overflow`].
    ///
    /// ```
    /// # #[cfg(all(feature = "versioned", feature = "time", feature = "decimal"))] {
    /// use rubo4e::current::Zaehlwerk;
    /// use rust_decimal::Decimal;
    ///
    /// let zw = Zaehlwerk {
    ///     vorkommastelle: Some(6),
    ///     wandlerfaktor: Some(Decimal::from(40)),   // an indirectly-measuring meter
    ///     ..Default::default()
    /// };
    ///
    /// // An ordinary pair.
    /// assert_eq!(
    ///     zw.consumption_between(Decimal::from(1_000), Decimal::from(1_050)),
    ///     Ok(Decimal::from(2_000)),                 // 50 × 40
    /// );
    ///
    /// // …and one that wrapped: 14 register steps, not −999 986.
    /// assert_eq!(
    ///     zw.consumption_between(Decimal::from(999_998), Decimal::from(12)),
    ///     Ok(Decimal::from(560)),                   // 14 × 40
    /// );
    /// # }
    /// ```
    pub fn consumption_between(
        &self,
        from: rust_decimal::Decimal,
        to: rust_decimal::Decimal,
    ) -> Result<rust_decimal::Decimal, ConsumptionError> {
        self.consumption_at(0, from, to)
    }

    /// [`consumption_between`](Self::consumption_between), carrying the index the
    /// sequence walk reports a failure at.
    fn consumption_at(
        &self,
        index: usize,
        from: rust_decimal::Decimal,
        to: rust_decimal::Decimal,
    ) -> Result<rust_decimal::Decimal, ConsumptionError> {
        use rust_decimal::Decimal;

        let mut delta = to.checked_sub(from).ok_or(ConsumptionError::Overflow)?;
        if delta < Decimal::ZERO {
            let capacity = self
                .register_capacity()
                .ok_or(ConsumptionError::DecreasedWithoutRegisterWidth { index, from, to })?;
            delta = delta
                .checked_add(capacity)
                .ok_or(ConsumptionError::Overflow)?;
            // Still negative: the fall is larger than one whole revolution, so it
            // is not a wrap-around at all.
            if delta < Decimal::ZERO {
                return Err(ConsumptionError::DecreasedWithoutRegisterWidth { index, from, to });
            }
        }
        delta
            .checked_mul(self.wandlerfaktor.unwrap_or(Decimal::ONE))
            .ok_or(ConsumptionError::Overflow)
    }

    /// The consumption across every consecutive pair of
    /// [`readings`](Self::readings).
    ///
    /// The whole period the register covers, in
    /// [`einheit`](crate::current::Zaehlwerk::einheit), with each step corrected
    /// for a wrap-around and multiplied by the `wandlerfaktor`.
    ///
    /// # Errors
    ///
    /// - [`ConsumptionError::TooFewReadings`] — a consumption needs a pair.
    /// - [`ConsumptionError::MeterExchange`] — a reading marks
    ///   `Z78_GERAETEWECHSEL`. The register restarted from an unrelated state, so
    ///   the difference across that boundary is not a consumption. Split the
    ///   series there and sum the halves; nothing here can do that for you,
    ///   because only you know which meter the second half belongs to.
    /// - [`ConsumptionError::IncompatibleUnit`] — a reading is in a unit that
    ///   does not convert to the register's, so it was dropped from
    ///   [`readings`](Self::readings) and the total would silently skip it.
    /// - [`ConsumptionError::DecreasedWithoutRegisterWidth`],
    ///   [`ConsumptionError::Overflow`] — as
    ///   [`consumption_between`](Self::consumption_between).
    pub fn total_consumption(&self) -> Result<rust_decimal::Decimal, ConsumptionError> {
        // A reading dropped for an unconvertible unit would leave a total that
        // silently spans a gap, so it is an error rather than an omission.
        if let Some(index) = self.unconvertible_reading() {
            return Err(ConsumptionError::IncompatibleUnit { index });
        }

        let readings = self.readings();
        if readings.len() < 2 {
            return Err(ConsumptionError::TooFewReadings {
                count: readings.len(),
            });
        }
        if let Some(r) = readings.iter().find(|r| {
            r.source.messwertstatuszusatz
                == Some(crate::generated::v202607::Messwertstatuszusatz::Z78Geraetewechsel)
        }) {
            return Err(ConsumptionError::MeterExchange { index: r.index });
        }

        readings
            .windows(2)
            .try_fold(rust_decimal::Decimal::ZERO, |acc, pair| {
                let step = self.consumption_at(pair[1].index, pair[0].value, pair[1].value)?;
                acc.checked_add(step).ok_or(ConsumptionError::Overflow)
            })
    }

    /// The index of the first reading whose unit does not convert to the
    /// register's, if there is one.
    fn unconvertible_reading(&self) -> Option<usize> {
        let target = self.einheit?;
        self.messwerte
            .as_deref()
            .unwrap_or_default()
            .iter()
            .enumerate()
            .find(|(_, m)| {
                m.messwertstatus.is_none_or(|s| s.is_usable())
                    && m.zeitpunkt.is_some()
                    && m.wert.is_some()
                    && reading_value(m, Some(target)).is_none()
            })
            .map(|(index, _)| index)
    }
}

/// A reading's numeric value, converted into `target` when both are stated.
#[cfg(feature = "decimal")]
fn reading_value(
    m: &Messwert,
    target: Option<crate::generated::v202607::Mengeneinheit>,
) -> Option<rust_decimal::Decimal> {
    let menge = m.wert.as_ref()?;
    match (target, menge.einheit) {
        // The register names a unit and the reading names a different one: the
        // reading has to be brought onto the register's scale, or it is not
        // comparable with the reading beside it.
        (Some(t), Some(u)) if u != t => menge.convert_to(t)?.wert,
        _ => menge.wert,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generated::v202607::{Menge, Mengeneinheit, Zeitraum};
    use time::macros::datetime;

    #[cfg(feature = "decimal")]
    fn value(from: OffsetDateTime, minutes: i64, wert: i64) -> Zeitreihenwert {
        Zeitreihenwert {
            wert: Some(rust_decimal::Decimal::from(wert)),
            zeitraum: Some(Zeitraum::from_instants(
                from,
                from + Duration::minutes(minutes),
            )),
            ..Default::default()
        }
    }

    #[cfg(not(feature = "decimal"))]
    fn value(from: OffsetDateTime, minutes: i64, wert: i64) -> Zeitreihenwert {
        Zeitreihenwert {
            wert: Some(wert.to_string()),
            zeitraum: Some(Zeitraum::from_instants(
                from,
                from + Duration::minutes(minutes),
            )),
            ..Default::default()
        }
    }

    fn lastgang(werte: Vec<Zeitreihenwert>) -> Lastgang {
        // `Lastgang` is one of the two BOs the schema marks `required`, so it has
        // a `new` rather than a `Default`.
        Lastgang {
            messgroesse: Some(Mengeneinheit::Kw),
            werte: Some(werte),
            ..Lastgang::new(quarter_hour_interval())
        }
    }

    #[cfg(feature = "decimal")]
    fn quarter_hour_interval() -> Menge {
        Menge {
            wert: Some(rust_decimal::Decimal::from(15)),
            einheit: Some(Mengeneinheit::Minute),
            ..Default::default()
        }
    }

    #[cfg(not(feature = "decimal"))]
    fn quarter_hour_interval() -> Menge {
        Menge {
            wert: Some("15".to_owned()),
            einheit: Some(Mengeneinheit::Minute),
            ..Default::default()
        }
    }

    const T0: OffsetDateTime = datetime!(2026-01-01 00:00 +01:00);

    #[test]
    fn a_contiguous_series_is_complete() {
        let lg = lastgang(
            (0..4)
                .map(|i| value(T0 + Duration::minutes(15 * i), 15, 100))
                .collect(),
        );
        let report = lg.audit();
        assert!(report.is_complete(), "{report:?}");
        assert_eq!(report.covered, Duration::HOUR);
        assert_eq!(report.coverage_ratio(), Some(1.0));
        assert!(!report.out_of_order);
        assert_eq!(report.span_or_none(), lg.span());
    }

    #[test]
    fn a_hole_becomes_a_gap() {
        let lg = lastgang(vec![
            value(T0, 15, 100),
            value(T0 + Duration::minutes(30), 15, 100),
        ]);
        let report = lg.audit();
        assert!(!report.is_complete());
        assert_eq!(
            report.gaps,
            [T0 + Duration::minutes(15)..T0 + Duration::minutes(30)]
        );
        assert_eq!(report.missing(), Duration::minutes(15));
        assert_eq!(report.covered, Duration::minutes(30));
    }

    #[test]
    fn a_duplicate_reading_becomes_an_overlap_and_is_counted_once() {
        let lg = lastgang(vec![value(T0, 15, 100), value(T0, 15, 100)]);
        let report = lg.audit();
        assert!(!report.is_complete());
        assert_eq!(report.overlaps, [T0..T0 + Duration::minutes(15)]);
        // Counted once, so a double-booked quarter-hour does not read as two.
        assert_eq!(report.covered, Duration::minutes(15));
        assert!(report.gaps.is_empty());
    }

    #[test]
    fn a_partial_overlap_reports_only_the_shared_stretch() {
        let lg = lastgang(vec![
            value(T0, 15, 100),
            value(T0 + Duration::minutes(10), 15, 100),
        ]);
        let report = lg.audit();
        assert_eq!(
            report.overlaps,
            [T0 + Duration::minutes(10)..T0 + Duration::minutes(15)]
        );
        assert_eq!(report.covered, Duration::minutes(25));
    }

    #[test]
    fn an_entry_of_the_wrong_length_is_named_by_index() {
        let lg = lastgang(vec![
            value(T0, 15, 100),
            value(T0 + Duration::minutes(15), 30, 100),
        ]);
        let report = lg.audit();
        assert_eq!(report.wrong_length, [1]);
        // …and the coverage itself is still contiguous.
        assert!(report.gaps.is_empty());
    }

    #[test]
    fn a_zeitreihe_declares_no_interval_so_none_is_wrong() {
        let zr = Zeitreihe {
            einheit: Some(Mengeneinheit::Kwh),
            werte: Some(vec![
                value(T0, 15, 100),
                value(T0 + Duration::minutes(15), 45, 100),
            ]),
            ..Default::default()
        };
        let report = zr.audit();
        assert!(report.wrong_length.is_empty());
        assert!(report.is_complete(), "{report:?}");
    }

    #[test]
    fn an_unplaceable_entry_is_reported_with_its_reason() {
        let lg = lastgang(vec![
            value(T0, 15, 100),
            Zeitreihenwert::default(),
            Zeitreihenwert {
                zeitraum: Some(Zeitraum {
                    startdatum: Some(time::macros::date!(2026 - 01 - 01)),
                    ..Default::default()
                }),
                ..Default::default()
            },
        ]);
        let report = lg.audit();
        assert_eq!(
            report.unplaced,
            [
                UnplacedValue {
                    index: 1,
                    reason: UnplacedReason::MissingZeitraum
                },
                UnplacedValue {
                    index: 2,
                    reason: UnplacedReason::NotAnInstantRange
                },
            ]
        );
        assert!(!report.is_complete());
    }

    #[test]
    fn unsorted_entries_are_flagged_but_still_measured() {
        let lg = lastgang(vec![
            value(T0 + Duration::minutes(15), 15, 100),
            value(T0, 15, 100),
        ]);
        let report = lg.audit();
        assert!(report.out_of_order);
        assert!(report.gaps.is_empty());
        assert!(report.overlaps.is_empty());
        assert_eq!(report.covered, Duration::minutes(30));
    }

    #[test]
    fn auditing_over_a_reference_finds_a_missing_tail() {
        let lg = lastgang(vec![value(T0, 15, 100)]);
        // Against itself the series looks complete…
        assert!(lg.audit().is_complete());
        // …but it was meant to cover the whole hour.
        let report = lg.audit_over(T0..T0 + Duration::HOUR);
        assert_eq!(
            report.gaps,
            [T0 + Duration::minutes(15)..T0 + Duration::HOUR]
        );
        assert_eq!(report.coverage_ratio(), Some(0.25));
    }

    #[test]
    fn entries_outside_the_reference_are_clipped_not_counted() {
        let lg = lastgang(vec![
            value(T0 - Duration::HOUR, 15, 100),
            value(T0, 15, 100),
        ]);
        let report = lg.audit_over(T0..T0 + Duration::minutes(15));
        assert!(report.is_complete(), "{report:?}");
        assert_eq!(report.covered, Duration::minutes(15));
    }

    #[test]
    fn an_empty_series_has_no_reference() {
        let report = lastgang(vec![]).audit();
        assert_eq!(report.reference, None);
        assert_eq!(report.coverage_ratio(), None);
        assert!(!report.is_complete());
    }

    #[cfg(feature = "decimal")]
    #[test]
    fn power_integrates_but_does_not_sum() {
        use rust_decimal::Decimal;
        let lg = lastgang(vec![
            value(T0, 15, 400),
            value(T0 + Duration::minutes(15), 15, 480),
        ]);
        assert_eq!(lg.integrate(), Some(Decimal::from(220)));
        assert_eq!(lg.sum(), None, "kW is not an extensive quantity");
    }

    /// For a stated unit, exactly one of the two aggregates answers — which is
    /// what keeps a caller from silently getting kWh·h.
    #[cfg(feature = "decimal")]
    #[test]
    fn a_stated_unit_admits_exactly_one_aggregate() {
        let power = lastgang(vec![value(T0, 15, 400)]);
        assert!(power.sum().is_none() && power.integrate().is_some());
        assert_eq!(power.integrated_unit(), Some(Mengeneinheit::Kwh));

        let energy = Zeitreihe {
            einheit: Some(Mengeneinheit::Kwh),
            werte: Some(vec![value(T0, 15, 100)]),
            ..Default::default()
        };
        assert!(energy.sum().is_some() && energy.integrate().is_none());
        assert_eq!(energy.integrated_unit(), None);

        // A series that states no unit answers to both, on the caller's word.
        let unstated = Zeitreihe {
            werte: Some(vec![value(T0, 15, 100)]),
            ..Default::default()
        };
        assert!(unstated.sum().is_some() && unstated.integrate().is_some());
        assert_eq!(unstated.integrated_unit(), None);
    }

    #[cfg(feature = "decimal")]
    #[test]
    fn energy_sums() {
        use rust_decimal::Decimal;
        let zr = Zeitreihe {
            einheit: Some(Mengeneinheit::Kwh),
            werte: Some(vec![
                value(T0, 15, 100),
                value(T0 + Duration::minutes(15), 15, 120),
            ]),
            ..Default::default()
        };
        assert_eq!(zr.sum(), Some(Decimal::from(220)));
    }

    #[cfg(feature = "decimal")]
    #[test]
    fn a_missing_value_makes_the_sum_unavailable() {
        let zr = Zeitreihe {
            einheit: Some(Mengeneinheit::Kwh),
            werte: Some(vec![value(T0, 15, 100), Zeitreihenwert::default()]),
            ..Default::default()
        };
        assert_eq!(zr.sum(), None);
    }

    /// A `FEHLT` slot occupies the timeline without filling it, so the series is
    /// complete and not usable — two different questions, two different answers.
    #[test]
    fn a_declared_absence_keeps_the_timeline_but_not_the_data() {
        use crate::generated::v202607::Messwertstatus;

        let mut werte = vec![value(T0, 15, 100), value(T0 + Duration::minutes(15), 15, 0)];
        werte[0].status = Some(Messwertstatus::Abgelesen);
        werte[1].status = Some(Messwertstatus::Fehlt);
        let lg = lastgang(werte);

        let report = lg.audit();
        assert!(report.is_complete(), "the timeline has no hole: {report:?}");
        assert_eq!(report.unusable, [1]);
        assert!(!report.is_usable());

        // …and the aggregates refuse rather than adding a zero that is an
        // absence.
        #[cfg(feature = "decimal")]
        assert_eq!(lg.integrate(), None);
    }

    /// A substituted reading is a value, so it neither blocks the aggregates nor
    /// counts as unusable — the caller decides whether an `ERSATZWERT` is good
    /// enough for what they are doing.
    #[test]
    fn a_substitute_reading_is_usable() {
        use crate::generated::v202607::Messwertstatus;

        let mut werte = vec![value(T0, 15, 400)];
        werte[0].status = Some(Messwertstatus::Ersatzwert);
        let lg = lastgang(werte);

        let report = lg.audit();
        assert!(report.unusable.is_empty());
        assert!(report.is_usable());
        assert!(!Messwertstatus::Ersatzwert.is_measured());
        #[cfg(feature = "decimal")]
        assert_eq!(lg.integrate(), Some(rust_decimal::Decimal::from(100)));
    }

    /// The status-only shortcut must agree with the audit, without the sort or
    /// the allocation.
    #[test]
    fn all_values_usable_agrees_with_the_audit() {
        use crate::generated::v202607::Messwertstatus;

        let clean = lastgang(vec![value(T0, 15, 100)]);
        assert!(clean.all_values_usable());
        assert!(clean.audit().unusable.is_empty());

        let mut werte = vec![value(T0, 15, 100)];
        werte[0].status = Some(Messwertstatus::NichtVerwendbar);
        let dirty = lastgang(werte);
        assert!(!dirty.all_values_usable());
        assert_eq!(dirty.audit().unusable, [0]);

        // An entry with no interval at all still counts here: the shortcut asks
        // about statuses, not about the timeline.
        let mut werte = vec![Zeitreihenwert::default()];
        werte[0].status = Some(Messwertstatus::Fehlt);
        assert!(!lastgang(werte).all_values_usable());
    }

    #[test]
    fn expected_interval_reads_the_declared_menge() {
        let lg = lastgang(vec![]);
        #[cfg(feature = "decimal")]
        assert_eq!(lg.expected_interval(), Some(Duration::minutes(15)));
        #[cfg(not(feature = "decimal"))]
        assert_eq!(lg.expected_interval(), None);
    }

    impl CoverageReport {
        /// Test helper: the reference span, for comparing against `span()`.
        fn span_or_none(&self) -> Option<Range<OffsetDateTime>> {
            self.reference.clone()
        }
    }
}
