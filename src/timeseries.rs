//! Placing a BO4E time series on a timeline, and auditing what it covers.
//!
//! [`Lastgang`] and [`Zeitreihe`] are the two BO4E Geschäftsobjekte that carry a
//! `Vec<`[`Zeitreihenwert`]`>`, and between them they are the highest-volume
//! payload in German market communication: a year of quarter-hourly meter
//! readings is 35 040 entries, and MSCONS, MaBiS and Redispatch 2.0 all move
//! them by the million.
//!
//! Each entry states its own [`Zeitraum`], so the series is a *bag* of intervals,
//! not a sequence. Nothing in the schema requires them to be sorted, contiguous,
//! disjoint, or the length the `Lastgang` declares — and in practice they are
//! routinely none of those. The single question every consumer has to answer
//! before using one is therefore *"is this series actually complete?"*, and this
//! module answers it in one call.
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
//! [`audit`](Bo4eTimeSeries::audit) is a **data-quality** report, not a
//! conformance check. BO4E states none of these properties, so nothing here is
//! wired into `.validate()` — the same line [`validation::current::quality`]
//! draws. A counterparty's gappy Lastgang is still a valid Lastgang; whether you
//! accept it is your decision, and this gives you the facts to make it.
//!
//! [`validation::current::quality`]: crate::validation::current::quality
//! [`Lastgang`]: crate::current::Lastgang
//! [`Zeitreihe`]: crate::current::Zeitreihe
//! [`Zeitreihenwert`]: crate::current::Zeitreihenwert
//! [`Zeitraum`]: crate::current::Zeitraum

use crate::convenience::ZeitpunktError;
use crate::generated::v202607::{Lastgang, Zeitreihe, Zeitreihenwert};
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
