//! Convenience methods and extension traits on generated BO4E types.
//!
//! These are hand-written extension `impl` blocks on generated structs.
//! The generated files are annotated `// @generated — do not edit by hand`,
//! so all ergonomic additions live here instead.
//!
//! All items are guarded by the feature flags that make their return types
//! available:
//!
//! - `versioned` — the generated BO/COM structs
//! - `time` — `time::Date` / `time::OffsetDateTime` return types
//! - `decimal` — `rust_decimal::Decimal` return types
//!
//! ## Extension traits
//!
//! [`BetragExt`][be], [`MengeExt`][me], and [`PreisExt`][pe] are the primary ergonomic
//! entry points for the common `Option<Com> → Option<Decimal>` pattern.  They
//! eliminate the repetitive `.as_ref().and_then(|x| x.wert)` chain:
//!
//! ```
//! # #[cfg(feature = "decimal")] {
//! use rubo4e::prelude::*;  // re-exports BetragExt, MengeExt, PreisExt
//! use rubo4e::current::{Betrag, Waehrungscode};
//!
//! let gesamtpreis = Some(Betrag {
//!     wert: Some(rust_decimal::Decimal::new(1250, 2)),
//!     waehrung: Some(Waehrungscode::Eur),
//!     ..Default::default()
//! });
//!
//! // Before: `gesamtpreis.as_ref().and_then(|b| b.wert)`
//! assert_eq!(gesamtpreis.wert_decimal(), Some(rust_decimal::Decimal::new(1250, 2)));
//! # }
//! ```
//!
//! Import via `use rubo4e::prelude::*` or individually via
//! `use rubo4e::convenience::{BetragExt, MengeExt, PreisExt}`.
//!
//! [be]: crate::convenience::BetragExt
//! [me]: crate::convenience::MengeExt
//! [pe]: crate::convenience::PreisExt

// ── Extension traits: Option<Com> → Option<Decimal> ─────────────────────────
//
// Flatten `Option<Betrag/Menge/Preis>` to `Option<Decimal>` in one method call.
// Gated on both `versioned` (the struct) and `decimal` (the type).

/// Flattens an [`Option<Betrag>`][crate::v202607::Betrag] to its `wert`, replacing
/// the `.as_ref().and_then(|b| b.wert)` chain. In the prelude.
#[cfg(all(feature = "versioned", feature = "decimal"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "versioned", feature = "decimal"))))]
pub trait BetragExt {
    /// `None` when the outer `Option` is empty or `wert` is.
    fn wert_decimal(&self) -> Option<rust_decimal::Decimal>;
}

#[cfg(all(feature = "versioned", feature = "decimal"))]
impl BetragExt for Option<crate::generated::v202607::Betrag> {
    fn wert_decimal(&self) -> Option<rust_decimal::Decimal> {
        self.as_ref().and_then(|b| b.wert)
    }
}

/// Flattens an [`Option<Menge>`][crate::v202607::Menge] to its `wert`, replacing
/// the `.as_ref().and_then(|m| m.wert)` chain. In the prelude.
#[cfg(all(feature = "versioned", feature = "decimal"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "versioned", feature = "decimal"))))]
pub trait MengeExt {
    /// `None` when the outer `Option` is empty or `wert` is.
    fn wert_decimal(&self) -> Option<rust_decimal::Decimal>;
}

#[cfg(all(feature = "versioned", feature = "decimal"))]
impl MengeExt for Option<crate::generated::v202607::Menge> {
    fn wert_decimal(&self) -> Option<rust_decimal::Decimal> {
        self.as_ref().and_then(|m| m.wert)
    }
}

/// Flattens an [`Option<Preis>`][crate::v202607::Preis] to its `wert`, replacing
/// the `.as_ref().and_then(|p| p.wert)` chain. In the prelude.
#[cfg(all(feature = "versioned", feature = "decimal"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "versioned", feature = "decimal"))))]
pub trait PreisExt {
    /// `None` when the outer `Option` is empty or `wert` is.
    fn wert_decimal(&self) -> Option<rust_decimal::Decimal>;
}

#[cfg(all(feature = "versioned", feature = "decimal"))]
impl PreisExt for Option<crate::generated::v202607::Preis> {
    fn wert_decimal(&self) -> Option<rust_decimal::Decimal> {
        self.as_ref().and_then(|p| p.wert)
    }
}

/// What the `format: "time"` accessors return: a time of day plus the UTC offset
/// it was written with, or the reason it did not parse.
///
/// The offset is `Option` because BO4E does not require one — see
/// [`offset_time`](crate::offset_time).
#[cfg(all(feature = "versioned", feature = "time"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "versioned", feature = "time"))))]
pub type OffsetTimeResult =
    Result<(time::Time, Option<time::UtcOffset>), crate::offset_time::OffsetTimeError>;

// ── Zeitraum ─────────────────────────────────────────────────────────────────

#[cfg(all(feature = "versioned", feature = "time"))]
mod zeitraum_impl {
    use super::{OffsetTimeResult, ZeitpunktError};
    use crate::generated::v202607::Zeitraum;
    use std::ops::{Range, RangeInclusive};
    use time::{Date, OffsetDateTime};

    /// # Boundary conventions
    ///
    /// BO4E states these on the fields themselves, and they are not the same for
    /// all three pairs:
    ///
    /// | Pair | Type | Interval | Read by |
    /// |---|---|---|---|
    /// | `startdatum` / `enddatum` | `time::Date` | `[start, end]` — **closed** | [`as_inclusive_range`](Zeitraum::as_inclusive_range) |
    /// | `startuhrzeit` / `enduhrzeit` | time of day + offset | `[start, end)` — **half-open** | [`startuhrzeit_parsed`](Zeitraum::startuhrzeit_parsed) |
    /// | all four together | `time::OffsetDateTime` | `[start, end)` — **half-open** | [`as_instant_range`](Zeitraum::as_instant_range) |
    /// | `dauer` | ISO 8601 duration | — | [`duration`](Zeitraum::duration) |
    ///
    /// The date pair being **closed** means `2026-01-01 … 2026-01-31` is all 31
    /// days of January and `startdatum == enddatum` is a valid one-day period —
    /// the schema gives `'2025-01-01'` as the example for both fields. Reading
    /// `enddatum` exclusively drops a day from every period.
    ///
    /// **The date accessors read the date pair, and only that.** A value stating
    /// all four fields is a moment inside a day, but
    /// [`whole_days`](Zeitraum::whole_days) still answers `Some(1)` and
    /// [`contains`](Zeitraum::contains) still covers the whole date. Route on
    /// [`is_instant_range`](Zeitraum::is_instant_range).
    ///
    /// The time pair keeps its wire `String`: it carries a UTC offset
    /// (`"18:00:00+01:00"`) and no `time` type holds both.
    ///
    impl Zeitraum {
        /// Returns the period as an inclusive range, when **both** dates are present.
        ///
        /// `start..=end`, matching BO4E's *"Enddatum … ist **inklusiv**"*. The
        /// return type is a [`RangeInclusive`] so the convention travels with the
        /// value: `range.contains(&d)` is correct by construction.
        ///
        /// Use this where an open-ended interval means "not yet determined" and
        /// should be filtered out; [`bounds`](Self::bounds) keeps it instead.
        ///
        /// Reads the **date pair only**. A value that also states a time of day
        /// on both ends names a moment inside a day, not the day —
        /// [`as_instant_range`](Self::as_instant_range) is the accessor for that,
        /// and [`is_instant_range`](Self::is_instant_range) tells them apart.
        ///
        /// ```
        /// # #[cfg(all(feature = "versioned", feature = "time"))] {
        /// use rubo4e::v202607::Zeitraum;
        /// use time::macros::date;
        ///
        /// let january = Zeitraum {
        ///     startdatum: Some(date!(2026-01-01)),
        ///     enddatum: Some(date!(2026-01-31)),
        ///     ..Default::default()
        /// };
        /// let range = january.as_inclusive_range().expect("both dates present");
        /// assert!(range.contains(&date!(2026-01-31)), "the end date is inside");
        /// # }
        /// ```
        #[must_use]
        pub fn as_inclusive_range(&self) -> Option<RangeInclusive<Date>> {
            Some(self.startdatum?..=self.enddatum?)
        }

        /// Returns both boundary dates as they stand, either of which may be absent.
        ///
        /// An absent boundary is an **open end**, not a missing value: no
        /// `startdatum` means "since forever", no `enddatum` means "until further
        /// notice". [`contains`](Self::contains) reads them the same way.
        ///
        /// ```
        /// # #[cfg(all(feature = "versioned", feature = "time"))] {
        /// use rubo4e::v202607::Zeitraum;
        /// use time::macros::date;
        ///
        /// let ongoing = Zeitraum {
        ///     startdatum: Some(date!(2026-01-01)),
        ///     ..Default::default()
        /// };
        /// match ongoing.bounds() {
        ///     (Some(start), Some(end)) => println!("{start} through {end} inclusive"),
        ///     (Some(start), None)      => println!("{start} onwards"),
        ///     (None, Some(end))        => println!("until {end} inclusive"),
        ///     (None, None)             => println!("unbounded"),
        /// }
        /// # }
        /// ```
        #[must_use]
        pub fn bounds(&self) -> (Option<Date>, Option<Date>) {
            (self.startdatum, self.enddatum)
        }

        /// Returns `true` if `date` falls in `[startdatum, enddatum]` — **both
        /// boundaries included**.
        ///
        /// An absent boundary is unbounded on that side, so a
        /// default-constructed `Zeitraum` answers `true` to everything. Filter on
        /// [`as_inclusive_range`](Self::as_inclusive_range) where a period must
        /// actually be stated.
        ///
        /// Reads the **date pair only**, so a value carrying a time of day
        /// contains the whole day either boundary falls in. Use
        /// [`contains_instant`](Self::contains_instant) for those.
        ///
        /// ```
        /// # #[cfg(all(feature = "versioned", feature = "time"))] {
        /// use rubo4e::v202607::Zeitraum;
        /// use time::macros::date;
        ///
        /// let january = Zeitraum {
        ///     startdatum: Some(date!(2026-01-01)),
        ///     enddatum: Some(date!(2026-01-31)),
        ///     ..Default::default()
        /// };
        /// assert!(january.contains(date!(2026-01-01)));   // start: inside
        /// assert!(january.contains(date!(2026-01-31)));   // end:   inside
        /// assert!(!january.contains(date!(2026-02-01)));
        ///
        /// // A single-day period is `start == end`, exactly as BO4E's own
        /// // examples show.
        /// let one_day = Zeitraum {
        ///     startdatum: Some(date!(2026-03-15)),
        ///     enddatum: Some(date!(2026-03-15)),
        ///     ..Default::default()
        /// };
        /// assert!(one_day.contains(date!(2026-03-15)));
        /// assert_eq!(one_day.whole_days(), Some(1));
        /// # }
        /// ```
        #[must_use]
        pub fn contains(&self, date: Date) -> bool {
            let start_ok = self.startdatum.is_none_or(|d| date >= d);
            let end_ok = self.enddatum.is_none_or(|d| date <= d);
            start_ok && end_ok
        }

        /// Returns the number of days the period covers, when both dates are present.
        ///
        /// Both boundaries count, so January 2026 (`2026-01-01` … `2026-01-31`)
        /// is 31 days and a one-day period is 1 — the count a day-proportional
        /// network charge or an abrechnungsrelevante Zeitmenge is computed from.
        ///
        /// Returns `None` if either bound is absent, and `0` for a reversed pair
        /// (which [`validate_zeitraum`] rejects).
        ///
        /// Counts **calendar days**, from the date pair alone: a 15-minute slot
        /// inside one day is `Some(1)`, not a fraction.
        /// [`instant_duration`](Self::instant_duration) measures such a value.
        ///
        /// [`validate_zeitraum`]: crate::validation::v202607::validate_zeitraum
        ///
        /// ```
        /// # #[cfg(all(feature = "versioned", feature = "time"))] {
        /// use rubo4e::v202607::Zeitraum;
        /// use time::macros::date;
        ///
        /// let january = Zeitraum {
        ///     startdatum: Some(date!(2026-01-01)),
        ///     enddatum: Some(date!(2026-01-31)),
        ///     ..Default::default()
        /// };
        /// assert_eq!(january.whole_days(), Some(31));
        /// # }
        /// ```
        #[must_use]
        pub fn whole_days(&self) -> Option<i64> {
            let (start, end) = (self.startdatum?, self.enddatum?);
            Some(((end - start).whole_days() + 1).max(0))
        }

        /// Parses `dauer` as an exact [`time::Duration`].
        ///
        /// BO4E stores it as an ISO 8601 duration string (`"P1DT30H4S"`), which
        /// neither `serde` nor `time` parses.
        ///
        /// Returns `None` when `dauer` is absent, so a missing value and an
        /// unparsable one stay distinguishable.
        ///
        /// # Errors
        ///
        /// [`Iso8601DurationError`](crate::iso8601_duration::Iso8601DurationError)
        /// — including for a `P1Y` / `P1M` whose length depends on when it
        /// starts, which is refused rather than approximated. See the
        /// [module docs](crate::iso8601_duration).
        ///
        /// ```
        /// # #[cfg(all(feature = "versioned", feature = "time"))] {
        /// use rubo4e::v202607::Zeitraum;
        ///
        /// use time::Duration;
        ///
        /// let z = Zeitraum { dauer: Some("P1DT30H4S".into()), ..Default::default() };
        /// assert_eq!(
        ///     z.duration(),
        ///     Some(Ok(Duration::days(1) + Duration::hours(30) + Duration::seconds(4))),
        /// );
        ///
        /// assert_eq!(Zeitraum::default().duration(), None);
        /// # }
        /// ```
        #[must_use]
        pub fn duration(
            &self,
        ) -> Option<Result<time::Duration, crate::iso8601_duration::Iso8601DurationError>> {
            Some(crate::iso8601_duration::parse(self.dauer.as_deref()?))
        }

        /// Parses `startuhrzeit` into a time of day and its UTC offset.
        ///
        /// Returns `None` when the field is absent. The offset is itself
        /// optional — BO4E does not require one, and a missing offset means
        /// "local time, zone not stated", which is not the same claim as UTC.
        ///
        /// Unlike the date pair, `startuhrzeit` is inclusive and `enduhrzeit`
        /// **exclusive**: the window is `[start, end)`. See the
        /// [boundary convention](#boundary-convention-the-date-pair-is-closed).
        ///
        /// # Errors
        ///
        /// [`OffsetTimeError`](crate::offset_time::OffsetTimeError).
        ///
        /// ```
        /// # #[cfg(all(feature = "versioned", feature = "time"))] {
        /// use rubo4e::v202607::Zeitraum;
        /// use time::macros::{offset, time};
        ///
        /// let z = Zeitraum {
        ///     startuhrzeit: Some("18:00:00+01:00".into()),
        ///     enduhrzeit: Some("19:00:00+01:00".into()),
        ///     ..Default::default()
        /// };
        /// assert_eq!(z.startuhrzeit_parsed(), Some(Ok((time!(18:00:00), Some(offset!(+1))))));
        /// assert_eq!(z.enduhrzeit_parsed(),   Some(Ok((time!(19:00:00), Some(offset!(+1))))));
        /// # }
        /// ```
        #[must_use]
        pub fn startuhrzeit_parsed(&self) -> Option<OffsetTimeResult> {
            Some(crate::offset_time::parse(self.startuhrzeit.as_deref()?))
        }

        /// Parses `enduhrzeit` into a time of day and its UTC offset.
        ///
        /// The end of a time window is **exclusive**, the opposite of
        /// `enddatum` on the same type. See
        /// [`startuhrzeit_parsed`](Self::startuhrzeit_parsed).
        ///
        /// # Errors
        ///
        /// [`OffsetTimeError`](crate::offset_time::OffsetTimeError).
        #[must_use]
        pub fn enduhrzeit_parsed(&self) -> Option<OffsetTimeResult> {
            Some(crate::offset_time::parse(self.enduhrzeit.as_deref()?))
        }

        /// Resolves `startdatum` **and** `startuhrzeit` into one instant.
        ///
        /// BO4E's third `Zeitraum` mode, and the one every quarter-hourly
        /// [`Zeitreihenwert`] uses. It needs both halves: a date alone is a whole
        /// day and a time of day alone is a daily recurring window, so either on
        /// its own answers `None`.
        ///
        /// [`Zeitreihenwert`]: crate::current::Zeitreihenwert
        ///
        /// # Errors
        ///
        /// [`ZeitpunktError::Time`] when `startuhrzeit` does not parse, and
        /// [`ZeitpunktError::MissingOffset`] when it carries no UTC offset —
        /// without one there is a wall-clock reading, not an instant.
        ///
        /// ```
        /// # #[cfg(all(feature = "versioned", feature = "time"))] {
        /// use rubo4e::v202607::Zeitraum;
        /// use time::macros::{date, datetime};
        ///
        /// let slot = Zeitraum {
        ///     startdatum: Some(date!(2026-01-01)),
        ///     startuhrzeit: Some("00:00:00+01:00".into()),
        ///     enddatum: Some(date!(2026-01-01)),
        ///     enduhrzeit: Some("00:15:00+01:00".into()),
        ///     ..Default::default()
        /// };
        /// assert_eq!(slot.start_instant(), Some(Ok(datetime!(2026-01-01 00:00 +01:00))));
        ///
        /// // A date without a time of day is a day, not an instant.
        /// let day = Zeitraum { startdatum: Some(date!(2026-01-01)), ..Default::default() };
        /// assert_eq!(day.start_instant(), None);
        /// # }
        /// ```
        #[must_use]
        pub fn start_instant(&self) -> Option<Result<OffsetDateTime, ZeitpunktError>> {
            Some(combine(self.startdatum?, self.startuhrzeit.as_deref()?))
        }

        /// Resolves `enddatum` **and** `enduhrzeit` into one instant.
        ///
        /// The counterpart of [`start_instant`](Self::start_instant). BO4E
        /// declares `enduhrzeit` **exclusive**, so this is the instant the period
        /// stops rather than its last moment — see
        /// [`as_instant_range`](Self::as_instant_range).
        ///
        /// # Errors
        ///
        /// As [`start_instant`](Self::start_instant).
        #[must_use]
        pub fn end_instant(&self) -> Option<Result<OffsetDateTime, ZeitpunktError>> {
            Some(combine(self.enddatum?, self.enduhrzeit.as_deref()?))
        }

        /// Returns the period as a **half-open** instant range, `[start, end)`.
        ///
        /// `Range`, not `RangeInclusive`: `startuhrzeit` is *"inklusiv"* and
        /// `enduhrzeit` *"exklusiv"*, the opposite of the date pair on the same
        /// struct. Consecutive quarter-hours therefore abut without overlapping.
        ///
        /// `None` unless all four fields are present —
        /// [`is_instant_range`](Self::is_instant_range) tells the two shapes
        /// apart.
        ///
        /// # Errors
        ///
        /// As [`start_instant`](Self::start_instant), reporting whichever end
        /// failed first.
        ///
        /// ```
        /// # #[cfg(all(feature = "versioned", feature = "time"))] {
        /// use rubo4e::v202607::Zeitraum;
        /// use time::macros::datetime;
        ///
        /// let slot = Zeitraum::from_instants(
        ///     datetime!(2026-01-01 00:00 +01:00),
        ///     datetime!(2026-01-01 00:15 +01:00),
        /// );
        /// let range = slot.as_instant_range().unwrap().unwrap();
        /// assert!(range.contains(&datetime!(2026-01-01 00:00 +01:00)));   // start: inside
        /// assert!(!range.contains(&datetime!(2026-01-01 00:15 +01:00)));  // end:   outside
        /// # }
        /// ```
        #[must_use]
        pub fn as_instant_range(&self) -> Option<Result<Range<OffsetDateTime>, ZeitpunktError>> {
            let start = self.start_instant()?;
            let end = self.end_instant()?;
            Some(match (start, end) {
                (Err(e), _) | (Ok(_), Err(e)) => Err(e),
                (Ok(s), Ok(e)) => Ok(s..e),
            })
        }

        /// Whether this `Zeitraum` states a full instant on **both** ends — the
        /// shape [`as_instant_range`](Self::as_instant_range) reads.
        ///
        /// Checks presence only, not that the times parse. Use it to route a
        /// value to the instant accessors or the date ones, since the date
        /// accessors answer for a whole day either way:
        ///
        /// ```
        /// # #[cfg(all(feature = "versioned", feature = "time"))] {
        /// use rubo4e::v202607::Zeitraum;
        /// use time::macros::{date, datetime};
        ///
        /// let slot = Zeitraum::from_instants(
        ///     datetime!(2026-01-01 00:00 +01:00),
        ///     datetime!(2026-01-01 00:15 +01:00),
        /// );
        /// assert!(slot.is_instant_range());
        /// // The date pair still answers — for the whole day, which is not the
        /// // period this value means.
        /// assert_eq!(slot.whole_days(), Some(1));
        ///
        /// let january = Zeitraum {
        ///     startdatum: Some(date!(2026-01-01)),
        ///     enddatum: Some(date!(2026-01-31)),
        ///     ..Default::default()
        /// };
        /// assert!(!january.is_instant_range());
        /// # }
        /// ```
        #[must_use]
        pub fn is_instant_range(&self) -> bool {
            self.startdatum.is_some()
                && self.startuhrzeit.is_some()
                && self.enddatum.is_some()
                && self.enduhrzeit.is_some()
        }

        /// The exact length of the instant range, `end - start`.
        ///
        /// Unlike [`duration`](Self::duration), which parses the `dauer` string a
        /// sender wrote, this measures what the boundaries actually say — the
        /// number an interval-length check is run against.
        ///
        /// # Errors
        ///
        /// As [`as_instant_range`](Self::as_instant_range).
        ///
        /// ```
        /// # #[cfg(all(feature = "versioned", feature = "time"))] {
        /// use rubo4e::v202607::Zeitraum;
        /// use time::{Duration, macros::datetime};
        ///
        /// let slot = Zeitraum::from_instants(
        ///     datetime!(2026-01-01 00:00 +01:00),
        ///     datetime!(2026-01-01 00:15 +01:00),
        /// );
        /// assert_eq!(slot.instant_duration(), Some(Ok(Duration::minutes(15))));
        /// # }
        /// ```
        #[must_use]
        pub fn instant_duration(&self) -> Option<Result<time::Duration, ZeitpunktError>> {
            Some(self.as_instant_range()?.map(|r| r.end - r.start))
        }

        /// Returns `true` if `at` falls in `[start_instant, end_instant)`.
        ///
        /// An **absent** boundary is open on that side, matching
        /// [`contains`](Self::contains) on the date pair: a `Zeitraum` stating
        /// only a start instant contains everything from it onwards, and one
        /// stating no instants at all contains everything. Filter on
        /// [`is_instant_range`](Self::is_instant_range) where a period must
        /// actually be stated.
        ///
        /// A **malformed** boundary is not open — it answers `false`. A bound you
        /// cannot read is not one you can establish you are inside, and dropping
        /// the record is the safe direction for the `.filter()` this predicate
        /// exists for. Use [`as_instant_range`](Self::as_instant_range) to tell a
        /// malformed value from an out-of-range one.
        ///
        /// ```
        /// # #[cfg(all(feature = "versioned", feature = "time"))] {
        /// use rubo4e::v202607::Zeitraum;
        /// use time::macros::datetime;
        ///
        /// let slot = Zeitraum::from_instants(
        ///     datetime!(2026-01-01 00:00 +01:00),
        ///     datetime!(2026-01-01 00:15 +01:00),
        /// );
        /// assert!(slot.contains_instant(datetime!(2026-01-01 00:00 +01:00)));
        /// assert!(!slot.contains_instant(datetime!(2026-01-01 00:15 +01:00)));
        /// // The same moment written in another offset compares equal.
        /// assert!(slot.contains_instant(datetime!(2026-01-01 00:10 +01:00)));
        /// # }
        /// ```
        #[must_use]
        pub fn contains_instant(&self, at: OffsetDateTime) -> bool {
            let after_start = match self.start_instant() {
                Some(Ok(s)) => at >= s,
                Some(Err(_)) => return false,
                None => true,
            };
            let before_end = match self.end_instant() {
                Some(Ok(e)) => at < e,
                Some(Err(_)) => return false,
                None => true,
            };
            after_start && before_end
        }

        /// Builds a `Zeitraum` for the half-open instant range `[start, end)`.
        ///
        /// Fills all four fields, writing each time of day with the offset its
        /// `OffsetDateTime` carries, so
        /// [`as_instant_range`](Self::as_instant_range) returns what went in.
        ///
        /// ```
        /// # #[cfg(all(feature = "versioned", feature = "time"))] {
        /// use rubo4e::v202607::Zeitraum;
        /// use time::macros::datetime;
        ///
        /// let start = datetime!(2026-01-01 00:00 +01:00);
        /// let slot = Zeitraum::from_instants(start, start + time::Duration::minutes(15));
        ///
        /// assert_eq!(slot.startuhrzeit.as_deref(), Some("00:00:00+01:00"));
        /// assert_eq!(slot.enduhrzeit.as_deref(),   Some("00:15:00+01:00"));
        /// assert_eq!(slot.as_instant_range(), Some(Ok(start..start + time::Duration::minutes(15))));
        /// # }
        /// ```
        #[must_use]
        pub fn from_instants(start: OffsetDateTime, end: OffsetDateTime) -> Zeitraum {
            Zeitraum {
                startdatum: Some(start.date()),
                startuhrzeit: Some(crate::offset_time::format(
                    start.time(),
                    Some(start.offset()),
                )),
                enddatum: Some(end.date()),
                enduhrzeit: Some(crate::offset_time::format(end.time(), Some(end.offset()))),
                ..Default::default()
            }
        }
    }

    /// Joins a date and a `format: "time"` string into one instant.
    fn combine(date: Date, uhrzeit: &str) -> Result<OffsetDateTime, ZeitpunktError> {
        let (time, offset) = crate::offset_time::parse(uhrzeit).map_err(ZeitpunktError::Time)?;
        let offset = offset.ok_or(ZeitpunktError::MissingOffset)?;
        Ok(date.with_time(time).assume_offset(offset))
    }
}

/// Why a `Zeitraum` boundary could not be resolved to an instant.
///
/// Returned by [`Zeitraum::start_instant`](crate::current::Zeitraum::start_instant)
/// and its siblings.
#[cfg(all(feature = "versioned", feature = "time"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "versioned", feature = "time"))))]
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum ZeitpunktError {
    /// The `startuhrzeit` / `enduhrzeit` string is not a time of day.
    #[error("{0}")]
    Time(#[from] crate::offset_time::OffsetTimeError),

    /// The time of day parsed but states no UTC offset, so it names a
    /// wall-clock reading rather than a moment.
    ///
    /// BO4E's own examples all carry one (`"18:00:00+01:00"`). Germany changes
    /// offset twice a year, so assuming `+01:00` is wrong for half of it and
    /// assuming UTC is wrong for all of it.
    #[error("time of day carries no UTC offset, so it does not name an instant")]
    MissingOffset,
}

// ── Messwertstatus — what a reading actually is ──────────────────────────────

#[cfg(feature = "versioned")]
mod messwertstatus_impl {
    use crate::generated::v202607::Messwertstatus;

    impl Messwertstatus {
        /// Whether this status marks a value that carries usable data at all.
        ///
        /// `false` for exactly two of the nine: `FEHLT` — the value is declared
        /// absent — and `NICHT_VERWENDBAR` — it is present but the sender says
        /// not to use it. A `Zeitreihenwert` in either state still occupies its
        /// slot on the timeline, so a coverage check alone reports the series as
        /// contiguous while every reading in it is unusable. That is the gap
        /// [`CoverageReport::unusable`] closes.
        ///
        /// [`CoverageReport::unusable`]: crate::timeseries::CoverageReport::unusable
        ///
        /// The `Unknown` catch-all is **not** usable: an out-of-schema status is
        /// a claim this crate cannot read, and treating it as a clean reading is
        /// the dangerous default.
        ///
        /// ```
        /// # #[cfg(feature = "versioned")] {
        /// use rubo4e::current::Messwertstatus;
        ///
        /// assert!(Messwertstatus::Abgelesen.is_usable());
        /// assert!(Messwertstatus::Ersatzwert.is_usable());   // substituted, but a value
        /// assert!(!Messwertstatus::Fehlt.is_usable());
        /// assert!(!Messwertstatus::NichtVerwendbar.is_usable());
        /// assert!(!Messwertstatus::Unknown.is_usable());
        /// # }
        /// ```
        #[must_use]
        pub const fn is_usable(self) -> bool {
            !matches!(
                self,
                Messwertstatus::Fehlt | Messwertstatus::NichtVerwendbar | Messwertstatus::Unknown
            )
        }

        /// Whether this status marks a value that was **measured** rather than
        /// derived.
        ///
        /// `true` only for `ABGELESEN`. Everything else is a substitute, a
        /// forecast, a provisional figure or an absence — all legitimate on the
        /// wire, and none of them a meter reading. Settlement rules distinguish
        /// them, so a pipeline that bills on `ERSATZWERT` as if it were
        /// `ABGELESEN` is making a claim the sender did not.
        #[must_use]
        pub const fn is_measured(self) -> bool {
            matches!(self, Messwertstatus::Abgelesen)
        }

        /// Whether this status marks a value that stands **in place of** a
        /// measurement: `ERSATZWERT`, `VORSCHLAGSWERT`, `PROGNOSEWERT`,
        /// `VORLAEUFIGERWERT`, `ENERGIEMENGESUMMIERT`,
        /// `ANGABE_FUER_LIEFERSCHEIN`.
        ///
        /// A value, and a usable one — but not a reading. The three predicates
        /// partition the enum: every variant is measured, substituted, or
        /// unusable.
        #[must_use]
        pub const fn is_substitute(self) -> bool {
            matches!(
                self,
                Messwertstatus::Ersatzwert
                    | Messwertstatus::Vorschlagswert
                    | Messwertstatus::Prognosewert
                    | Messwertstatus::Vorlaeufigerwert
                    | Messwertstatus::Energiemengesummiert
                    | Messwertstatus::AngabeFuerLieferschein
            )
        }
    }

    #[cfg(test)]
    mod tests {
        use super::Messwertstatus;

        /// The three predicates must partition the enum: exactly one holds for
        /// every variant, so a new status in the next release fails here rather
        /// than falling through every branch unnoticed.
        #[test]
        fn the_predicates_partition_every_variant() {
            for status in Messwertstatus::VARIANTS {
                let flags = [
                    status.is_measured(),
                    status.is_substitute(),
                    !status.is_usable(),
                ];
                assert_eq!(
                    flags.iter().filter(|f| **f).count(),
                    1,
                    "{} is in {} of the three classes",
                    status.as_wire(),
                    flags.iter().filter(|f| **f).count(),
                );
            }
        }

        #[test]
        fn the_catch_all_is_not_treated_as_a_clean_reading() {
            assert!(!Messwertstatus::Unknown.is_usable());
            assert!(!Messwertstatus::Unknown.is_measured());
            assert!(!Messwertstatus::Unknown.is_substitute());
        }
    }
}

// ── Umschaltzeit — the HT/NT switching time ──────────────────────────────────

#[cfg(all(feature = "versioned", feature = "time"))]
mod umschaltzeit_impl {
    use super::OffsetTimeResult;
    use crate::generated::v202607::Umschaltzeit;

    impl Umschaltzeit {
        /// Parses `umschaltzeit` into a time of day and its UTC offset.
        ///
        /// This is the instant a Doppeltarifzähler switches between
        /// Hoch- and Niedertarif, so the offset is load-bearing: `06:00:00+01:00`
        /// is a different wall-clock moment in summer than in winter, and
        /// dropping it moves the tariff boundary by an hour.
        ///
        /// Returns `None` when the field is absent.
        ///
        /// # Errors
        ///
        /// [`OffsetTimeError`](crate::offset_time::OffsetTimeError).
        #[must_use]
        pub fn umschaltzeit_parsed(&self) -> Option<OffsetTimeResult> {
            Some(crate::offset_time::parse(self.umschaltzeit.as_deref()?))
        }
    }
}

// ── Rechnung ─────────────────────────────────────────────────────────────────

#[cfg(all(feature = "versioned", feature = "time"))]
mod rechnung_impl {
    use crate::generated::v202607::Rechnung;
    use time::Date;

    impl Rechnung {
        /// Returns the billing period as `(start, end)` dates.
        ///
        /// Reads from `rechnungsperiode.startdatum` / `rechnungsperiode.enddatum`.
        /// Returns `None` when `rechnungsperiode` is absent or either boundary
        /// date is missing.
        ///
        /// Both boundaries are **inclusive** — see the [boundary convention] on
        /// `Zeitraum`. The last day of the month a monthly invoice covers is
        /// inside its own billing period.
        ///
        /// [boundary convention]: crate::v202607::Zeitraum#boundary-convention-the-date-pair-is-closed
        ///
        /// ```no_run
        /// # #[cfg(all(feature = "versioned", feature = "time"))] {
        /// # use rubo4e::v202607::Rechnung;
        /// # use time::macros::date;
        /// let r: Rechnung = todo!();
        /// if let Some(period) = r.billing_period() {
        ///     println!("invoice period: {} – {} inclusive", period.start(), period.end());
        ///     let billed = period.contains(&date!(2026-01-31));
        /// }
        /// # }
        /// ```
        #[must_use]
        pub fn billing_period(&self) -> Option<std::ops::RangeInclusive<Date>> {
            self.rechnungsperiode.as_ref()?.as_inclusive_range()
        }

        /// Billing period start date — `rechnungsperiode.startdatum`.
        ///
        /// Returns `None` when `rechnungsperiode` is absent or `startdatum` is
        /// missing. Unlike [`billing_period`](Self::billing_period) this does
        /// **not** require the other bound: an invoice whose period is open at
        /// the top still answers here, and `billing_period()` returns `None`.
        #[must_use]
        pub fn period_start(&self) -> Option<Date> {
            self.rechnungsperiode.as_ref()?.startdatum
        }

        /// Billing period end date — `rechnungsperiode.enddatum`, **inclusive**.
        ///
        /// Returns `None` when `rechnungsperiode` is absent or `enddatum` is
        /// missing. As with [`period_start`](Self::period_start), the other
        /// bound need not be present.
        #[must_use]
        pub fn period_end(&self) -> Option<Date> {
            self.rechnungsperiode.as_ref()?.enddatum
        }

        /// Invoice issue date — the calendar date of `rechnungsdatum`.
        ///
        /// BO4E types the field as a timestamp (`format: date-time`), but BDEW
        /// INVOIC transmits it as DTM+137 qualifier 102, a bare `YYYYMMDD`, so
        /// senders pin it to midnight in their own offset. Dropping the
        /// time-of-day keeps date comparisons from comparing offsets.
        ///
        /// The date is taken **in the offset the payload carries**; use
        /// [`Rechnung::rechnungsdatum`] to normalise first.
        #[must_use]
        pub fn rechnungsdatum_date(&self) -> Option<Date> {
            self.rechnungsdatum.map(|dt| dt.date())
        }

        /// Payment due date — the calendar date of `faelligkeitsdatum`.
        ///
        /// Same date-only reading as [`rechnungsdatum_date`](Self::rechnungsdatum_date);
        /// BDEW INVOIC carries this as DTM+92 with qualifier 102.
        #[must_use]
        pub fn faelligkeitsdatum_date(&self) -> Option<Date> {
            self.faelligkeitsdatum.map(|dt| dt.date())
        }
    }
}

// ── Rechnung — decimal accessors ─────────────────────────────────────────────

#[cfg(all(feature = "versioned", feature = "decimal"))]
mod rechnung_decimal_impl {
    use crate::generated::v202607::Rechnung;

    /// The invoice totals, flattened from `Option<Betrag>` to `Option<Decimal>`.
    ///
    /// Each returns `None` when its `Betrag` is absent or carries no `wert`, and
    /// reads what the sender wrote — no total is re-derived from the others.
    impl Rechnung {
        /// Net total (`gesamtnetto.wert`).
        #[must_use]
        pub fn gesamtnetto_decimal(&self) -> Option<rust_decimal::Decimal> {
            self.gesamtnetto.as_ref()?.wert
        }

        /// Gross total (`gesamtbrutto.wert`).
        #[must_use]
        pub fn gesamtbrutto_decimal(&self) -> Option<rust_decimal::Decimal> {
            self.gesamtbrutto.as_ref()?.wert
        }

        /// Total tax (`gesamtsteuer.wert`).
        #[must_use]
        pub fn gesamtsteuer_decimal(&self) -> Option<rust_decimal::Decimal> {
            self.gesamtsteuer.as_ref()?.wert
        }

        /// Amount to pay (`zu_zahlen.wert`).
        ///
        /// BO4E describes it as `gesamtbrutto - vorausbezahlt - rabattBrutto`,
        /// but v202607 ships no `rabattBrutto`, so the equation is not
        /// reconstructible from the payload — see
        /// [`vorauszahlungen_summe`](Self::vorauszahlungen_summe) for the
        /// advance-payment total on its own.
        #[must_use]
        pub fn zu_zahlen_decimal(&self) -> Option<rust_decimal::Decimal> {
            self.zu_zahlen.as_ref()?.wert
        }

        /// Net discount (`rabatt_netto.wert`).
        #[must_use]
        pub fn rabatt_netto_decimal(&self) -> Option<rust_decimal::Decimal> {
            self.rabatt_netto.as_ref()?.wert
        }

        /// Estimated next instalment (`zukuenftiger_abschlag.wert`).
        #[must_use]
        pub fn zukuenftiger_abschlag_decimal(&self) -> Option<rust_decimal::Decimal> {
            self.zukuenftiger_abschlag.as_ref()?.wert
        }

        /// Sum of all advance-payment amounts (`vorauszahlungen[*].betrag.wert`).
        ///
        /// Returns `None` when `vorauszahlungen` is absent, empty, or when the
        /// amounts overflow `Decimal`'s range. Returns `Some(Decimal::ZERO)`
        /// when payments are present but every `betrag` is `None`.
        ///
        /// Summed with [`checked_add`] rather than `+`, which would panic: the
        /// values come straight off a deserialization boundary.
        ///
        /// [`checked_add`]: rust_decimal::Decimal::checked_add
        #[must_use]
        pub fn vorauszahlungen_summe(&self) -> Option<rust_decimal::Decimal> {
            let payments = self.vorauszahlungen.as_deref()?;
            if payments.is_empty() {
                return None;
            }
            payments
                .iter()
                .filter_map(|p| p.betrag.as_ref().and_then(|b| b.wert))
                .try_fold(rust_decimal::Decimal::ZERO, |acc, v| acc.checked_add(v))
        }
    }
}

// ── Rechnung — versioned-only accessors ──────────────────────────────────────

#[cfg(feature = "versioned")]
mod rechnung_versioned_impl {
    use crate::generated::v202607::{Rechnung, Rechnungsposition};

    impl Rechnung {
        /// Iterates over all invoice line items (`rechnungspositionen`).
        ///
        /// Yields nothing when `rechnungspositionen` is `None` or empty.
        /// Eliminates the repetitive `.as_deref().into_iter().flatten()` pattern.
        ///
        /// ```no_run
        /// # #[cfg(feature = "versioned")] {
        /// # use rubo4e::v202607::Rechnung;
        /// let r: Rechnung = todo!();
        /// for pos in r.positions() {
        ///     println!("pos {}: {:?}", pos.positionsnummer.unwrap_or(0), pos.positionstext);
        /// }
        /// # }
        /// ```
        pub fn positions(&self) -> impl Iterator<Item = &Rechnungsposition> {
            self.rechnungspositionen.as_deref().into_iter().flatten()
        }

        /// Returns `true` if this is a cancellation invoice (`ist_storno == Some(true)`).
        #[must_use]
        pub fn is_storno(&self) -> bool {
            self.ist_storno.unwrap_or(false)
        }

        /// Returns `true` if this is an original invoice (`ist_original == Some(true)`).
        #[must_use]
        pub fn is_original(&self) -> bool {
            self.ist_original.unwrap_or(false)
        }
    }
}

// ── Rechnungsposition — time accessors ───────────────────────────────────────

#[cfg(all(feature = "versioned", feature = "time"))]
mod rechnungsposition_time_impl {
    use crate::generated::v202607::Rechnungsposition;
    use time::Date;

    impl Rechnungsposition {
        /// Delivery period start date from `lieferungszeitraum.startdatum`.
        ///
        /// Returns `None` when `lieferungszeitraum` is absent or `startdatum` is
        /// not set on the embedded [`Zeitraum`][crate::v202607::Zeitraum].
        #[must_use]
        pub fn lieferung_von_date(&self) -> Option<Date> {
            self.lieferungszeitraum.as_ref()?.startdatum
        }

        /// Delivery period end date from `lieferungszeitraum.enddatum`.
        ///
        /// Returns `None` when `lieferungszeitraum` is absent or `enddatum` is
        /// not set on the embedded [`Zeitraum`][crate::v202607::Zeitraum].
        #[must_use]
        pub fn lieferung_bis_date(&self) -> Option<Date> {
            self.lieferungszeitraum.as_ref()?.enddatum
        }

        /// Returns `true` if `date` falls within this line item's delivery period.
        ///
        /// Delegates to [`Zeitraum::contains`][crate::v202607::Zeitraum::contains].
        /// Returns `false` when `lieferungszeitraum` is absent.
        #[must_use]
        pub fn lieferungszeitraum_contains(&self, date: Date) -> bool {
            self.lieferungszeitraum
                .as_ref()
                .is_some_and(|z| z.contains(date))
        }
    }
}

// ── Zahlungsinformation — checked bank identifiers on demand ─────────────────

#[cfg(feature = "versioned")]
mod zahlungsinformation_impl {
    use crate::error::IdentifierError;
    use crate::generated::v202607::Zahlungsinformation;
    use crate::identifiers::{Bic, Iban};

    /// # Why `iban` is a `String` and not an [`Iban`]
    ///
    /// `Zahlungsinformation` hangs off `Rechnung` and nothing else, so a
    /// newtype that refuses a masked IBAN (`DE89 **** **** 3000`, routine on an
    /// invoice) would take the whole invoice down with it. The check is one call
    /// away instead, and returns an error rather than the invoice.
    impl Zahlungsinformation {
        /// Parses `iban` as a checksum-verified [`Iban`].
        ///
        /// Returns `None` when the field is absent, so "not stated" and "stated
        /// but invalid" stay distinguishable. Grouping spaces and lowercase are
        /// normalised, so a value copied from a bank statement parses.
        ///
        /// # Errors
        ///
        /// [`IdentifierError`], typically
        /// [`InvalidChecksum`](IdentifierError::InvalidChecksum) for a mistyped
        /// or masked value.
        ///
        /// ```
        /// # #[cfg(feature = "versioned")] {
        /// use rubo4e::current::Zahlungsinformation;
        ///
        /// let stated = |iban: &str| Zahlungsinformation {
        ///     iban: Some(iban.into()),
        ///     ..Default::default()
        /// };
        ///
        /// let z = stated("DE89 3704 0044 0532 0130 00");
        /// assert_eq!(z.iban_checked().unwrap().unwrap().bankleitzahl(), Some("37040044"));
        ///
        /// assert!(stated("DE89 **** **** **** 3000").iban_checked().unwrap().is_err());
        /// assert!(Zahlungsinformation::default().iban_checked().is_none());
        /// # }
        /// ```
        #[must_use]
        pub fn iban_checked(&self) -> Option<Result<Iban, IdentifierError>> {
            Some(Iban::new(self.iban.as_deref()?))
        }

        /// Parses `bic` as a [`Bic`], verifying the ISO 9362 grammar.
        ///
        /// Returns `None` when the field is absent. ISO 9362 defines no
        /// checksum, so this verifies the shape and nothing more.
        ///
        /// # Errors
        ///
        /// [`IdentifierError`] if the value is neither 8 nor 11 characters, or
        /// carries a digit where the standard requires a letter.
        #[must_use]
        pub fn bic_checked(&self) -> Option<Result<Bic, IdentifierError>> {
            Some(Bic::new(self.bic.as_deref()?))
        }
    }
}

// ── Preisstaffel — tier bounds and tier selection ────────────────────────────

#[cfg(all(feature = "versioned", feature = "decimal"))]
pub use preisstaffel_impl::PreisstaffelSliceExt;

#[cfg(all(feature = "versioned", feature = "decimal"))]
mod preisstaffel_impl {
    use crate::generated::v202607::Preisstaffel;
    use rust_decimal::Decimal;

    impl Preisstaffel {
        /// Returns `true` if `value` lies within this tier's own stated bounds,
        /// `staffelgrenzeVon ..= staffelgrenzeBis`. An absent bound is unbounded
        /// on that side.
        ///
        /// # This is *not* how a tier is selected
        ///
        /// BO4E's tiers can leave gaps — the schema's own example is
        /// `0 – 1000, 1001 – 2000` — and it rules that a value in a gap
        /// (`1000.6`) *"rutscht in die obere Zone"*. No single tier can honour
        /// that; the decision needs the whole list. Use
        /// [`select_for`](PreisstaffelSliceExt::select_for) to pick a tier, and
        /// this only to ask about one tier in isolation.
        ///
        /// ```
        /// # #[cfg(all(feature = "versioned", feature = "decimal"))] {
        /// use rubo4e::current::Preisstaffel;
        /// use rust_decimal::Decimal;
        ///
        /// let tier = Preisstaffel {
        ///     staffelgrenze_von: Some(Decimal::ZERO),
        ///     staffelgrenze_bis: Some(Decimal::from(1000)),
        ///     ..Default::default()
        /// };
        /// assert!(tier.contains(Decimal::from(1000)), "both bounds are inclusive");
        /// assert!(!tier.contains(Decimal::from(1001)));
        /// # }
        /// ```
        #[must_use]
        pub fn contains(&self, value: Decimal) -> bool {
            self.staffelgrenze_von.is_none_or(|von| value >= von)
                && self.staffelgrenze_bis.is_none_or(|bis| value <= bis)
        }
    }

    /// Picking the price tier that applies to a quantity.
    ///
    /// Implemented for `[Preisstaffel]`, so it works on the `preisstaffeln` /
    /// `staffeln` field of every type that carries one — `Preisposition`,
    /// `LastvariablePreisposition`, `TarifPreisposition`, and `AufAbschlag`.
    #[cfg_attr(docsrs, doc(cfg(all(feature = "versioned", feature = "decimal"))))]
    pub trait PreisstaffelSliceExt {
        /// Returns the tier that applies to `value`, or `None` if it falls below
        /// every tier or above all of them.
        ///
        /// # The gap rule
        ///
        /// BO4E states tier bounds like `0 – 1000, 1001 – 2000` and rules that a
        /// value *between* two tiers *"rutscht in die obere Zone / Staffel"*, so
        /// `1000.6` bills at the `1001 – 2000` rate. A `von <= x <= bis` scan
        /// finds no tier for it at all.
        ///
        /// What satisfies both cases: **the tier with the smallest
        /// `staffelgrenzeBis` still ≥ `value`**, provided `value` reaches the
        /// lowest `staffelgrenzeVon`. Slice order is irrelevant. An absent bound
        /// is unbounded on that side, so an open-topped final tier catches
        /// everything above the last stated one.
        ///
        /// ```
        /// # #[cfg(all(feature = "versioned", feature = "decimal"))] {
        /// use rubo4e::convenience::PreisstaffelSliceExt;
        /// use rubo4e::current::Preisstaffel;
        /// use rust_decimal::Decimal;
        ///
        /// fn tier(von: i64, bis: i64, preis: i64) -> Preisstaffel {
        ///     Preisstaffel {
        ///         staffelgrenze_von: Some(Decimal::from(von)),
        ///         staffelgrenze_bis: Some(Decimal::from(bis)),
        ///         preis: Some(Decimal::from(preis)),
        ///         ..Default::default()
        ///     }
        /// }
        /// let staffeln = [tier(0, 1000, 30), tier(1001, 2000, 25)];
        ///
        /// let price = |v| staffeln.select_for(v).and_then(|s| s.preis);
        ///
        /// assert_eq!(price(Decimal::from(1000)), Some(Decimal::from(30)));  // upper bound: inside
        /// assert_eq!(price(Decimal::new(10006, 1)), Some(Decimal::from(25))); // in the gap: upward
        /// assert!(price(Decimal::from(5000)).is_none());                    // above every tier
        /// assert!(price(Decimal::from(-1)).is_none());                      // below every tier
        /// # }
        /// ```
        fn select_for(&self, value: Decimal) -> Option<&Preisstaffel>;
    }

    impl PreisstaffelSliceExt for [Preisstaffel] {
        fn select_for(&self, value: Decimal) -> Option<&Preisstaffel> {
            // Below the lowest stated floor there is no tier at all. A tier with
            // no `von` is unbounded below, so its presence removes the floor.
            let below_floor = self
                .iter()
                .all(|s| s.staffelgrenze_von.is_some_and(|von| value < von));
            if self.is_empty() || below_floor {
                return None;
            }

            // The tier whose ceiling is the tightest one still at or above the
            // value. An absent ceiling is +∞ and therefore always a candidate,
            // but loses to any stated ceiling that also fits.
            self.iter()
                .filter(|s| s.staffelgrenze_bis.is_none_or(|bis| value <= bis))
                .min_by(|a, b| match (a.staffelgrenze_bis, b.staffelgrenze_bis) {
                    (Some(x), Some(y)) => x.cmp(&y),
                    (Some(_), None) => std::cmp::Ordering::Less,
                    (None, Some(_)) => std::cmp::Ordering::Greater,
                    (None, None) => std::cmp::Ordering::Equal,
                })
        }
    }
}

// ── Rechnungsposition — decimal accessors ────────────────────────────────────

#[cfg(all(feature = "versioned", feature = "decimal"))]
mod rechnungsposition_decimal_impl {
    use crate::generated::v202607::Rechnungsposition;

    impl Rechnungsposition {
        /// Line item total (`gesamtpreis.wert`) as `Decimal`.
        ///
        /// Returns `None` when `gesamtpreis` is absent or its `wert` is `None`.
        ///
        /// ```no_run
        /// # #[cfg(all(feature = "versioned", feature = "decimal"))] {
        /// # use rubo4e::v202607::Rechnungsposition;
        /// let pos: Rechnungsposition = todo!();
        /// if let Some(net) = pos.gesamtpreis_decimal() {
        ///     println!("line total: {net}");
        /// }
        /// # }
        /// ```
        #[must_use]
        pub fn gesamtpreis_decimal(&self) -> Option<rust_decimal::Decimal> {
            self.gesamtpreis.as_ref()?.wert
        }

        /// Unit price (`einzelpreis.wert`) as `Decimal`.
        ///
        /// Returns `None` when `einzelpreis` is absent or its `wert` is `None`.
        #[must_use]
        pub fn einzelpreis_decimal(&self) -> Option<rust_decimal::Decimal> {
            self.einzelpreis.as_ref()?.wert
        }

        /// Quantity (`positions_menge.wert`) as `Decimal`.
        ///
        /// Returns `None` when `positions_menge` is absent or its `wert` is `None`.
        #[must_use]
        pub fn positions_menge_decimal(&self) -> Option<rust_decimal::Decimal> {
            self.positions_menge.as_ref()?.wert
        }

        /// Time-proportional quantity (`zeitbezogene_menge.wert`) as `Decimal`.
        ///
        /// Used for period-proportional pricing (e.g. 3 months of a yearly rate).
        /// Returns `None` when `zeitbezogene_menge` is absent or its `wert` is `None`.
        #[must_use]
        pub fn zeitbezogene_menge_decimal(&self) -> Option<rust_decimal::Decimal> {
            self.zeitbezogene_menge.as_ref()?.wert
        }
    }
}

// ── PreisblattNetznutzung ─────────────────────────────────────────────────────

#[cfg(all(feature = "versioned", feature = "time"))]
mod preisblatt_netznutzung_impl {
    use crate::generated::v202607::{PreisblattNetznutzung, Zeitraum};
    use time::Date;

    impl PreisblattNetznutzung {
        /// Returns the price sheet's validity bounds, either of which may be absent.
        ///
        /// Reads `gueltigkeit.startdatum` / `gueltigkeit.enddatum`. Both are
        /// **inclusive**: the sheet is still valid on its `enddatum`. An absent
        /// bound is an open end — an indefinitely-valid sheet has no `enddatum`.
        ///
        /// Returns `(None, None)` when `gueltigkeit` itself is absent, which is
        /// indistinguishable from an unbounded validity; use
        /// [`is_valid_at`](Self::is_valid_at) if you need the "no validity stated"
        /// case to read as *not valid*.
        ///
        /// ```no_run
        /// # #[cfg(all(feature = "versioned", feature = "time"))] {
        /// # use rubo4e::v202607::PreisblattNetznutzung;
        /// let p: PreisblattNetznutzung = todo!();
        /// match p.validity() {
        ///     (Some(start), Some(end)) => println!("valid {start} – {end} inclusive"),
        ///     (Some(start), None)      => println!("valid from {start} (open-ended)"),
        ///     (None, Some(end))        => println!("valid until {end} inclusive"),
        ///     (None, None)             => println!("no validity stated"),
        /// }
        /// # }
        /// ```
        #[must_use]
        pub fn validity(&self) -> (Option<Date>, Option<Date>) {
            self.gueltigkeit
                .as_ref()
                .map_or((None, None), Zeitraum::bounds)
        }

        /// Returns `true` if this price sheet's validity period contains `date`.
        ///
        /// Uses [`Zeitraum::contains`][crate::v202607::Zeitraum::contains] — a
        /// missing `gueltigkeit` is treated as
        /// "always invalid" (returns `false`).
        ///
        /// ```no_run
        /// # #[cfg(all(feature = "versioned", feature = "time"))] {
        /// # use rubo4e::v202607::PreisblattNetznutzung;
        /// # use time::macros::date;
        /// let sheets: Vec<PreisblattNetznutzung> = todo!();
        /// let billing_date = date!(2026-03-15);
        /// let valid = sheets.iter().find(|s| s.is_valid_at(billing_date));
        /// # }
        /// ```
        #[must_use]
        pub fn is_valid_at(&self, date: Date) -> bool {
            self.gueltigkeit.as_ref().is_some_and(|z| z.contains(date))
        }
    }
}
