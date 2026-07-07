//! Convenience methods on generated BO4E types.
//!
//! These are hand-written extension `impl` blocks on generated structs.
//! The generated files are annotated `// @generated — do not edit by hand`,
//! so all ergonomic additions live here instead.
//!
//! All methods are guarded by the feature flags that make the return type
//! available:
//!
//! - `versioned` — the generated BO/COM structs
//! - `time` — `time::Date` / `time::OffsetDateTime` return types

// ── Zeitraum ─────────────────────────────────────────────────────────────────

#[cfg(all(feature = "versioned", feature = "time"))]
mod zeitraum_impl {
    use crate::generated::v202501::Zeitraum;
    use time::Date;

    impl Zeitraum {
        /// Returns the closed date range `(start, end)` if **both** boundary dates
        /// are present.
        ///
        /// Useful for iterating over billing or validity periods where an open-ended
        /// interval should be treated as "not yet determined" and filtered out:
        ///
        /// ```no_run
        /// # #[cfg(all(feature = "versioned", feature = "time"))] {
        /// # use rubo4e::v202501::Zeitraum;
        /// let z: Zeitraum = todo!();
        /// if let Some((start, end)) = z.as_closed_range() {
        ///     println!("from {start} to {end}");
        /// }
        /// # }
        /// ```
        ///
        /// To handle open-ended ranges use [`as_half_open_range`](Self::as_half_open_range).
        #[must_use]
        pub fn as_closed_range(&self) -> Option<(Date, Date)> {
            Some((self.startdatum?, self.enddatum?))
        }

        /// Returns `(start, optional_end)` if `startdatum` is present.
        ///
        /// Unlike [`as_closed_range`](Self::as_closed_range), this method succeeds
        /// even when `enddatum` is absent — representing an open-ended (ongoing)
        /// period such as an indefinitely-valid price list:
        ///
        /// ```no_run
        /// # #[cfg(all(feature = "versioned", feature = "time"))] {
        /// # use rubo4e::v202501::Zeitraum;
        /// let z: Zeitraum = todo!();
        /// if let Some((start, end)) = z.as_half_open_range() {
        ///     println!("starts {start}, ends {:?}", end);
        /// }
        /// # }
        /// ```
        #[must_use]
        pub fn as_half_open_range(&self) -> Option<(Date, Option<Date>)> {
            Some((self.startdatum?, self.enddatum))
        }
    }
}

// ── Rechnung ─────────────────────────────────────────────────────────────────

#[cfg(all(feature = "versioned", feature = "time"))]
mod rechnung_impl {
    use crate::generated::v202501::Rechnung;
    use time::Date;

    impl Rechnung {
        /// Returns the billing period as `(start, end)` dates.
        ///
        /// Reads from `rechnungsperiode.startdatum` / `rechnungsperiode.enddatum`.
        /// Returns `None` when `rechnungsperiode` is absent or either boundary
        /// date is missing.
        ///
        /// ```no_run
        /// # #[cfg(all(feature = "versioned", feature = "time"))] {
        /// # use rubo4e::v202501::Rechnung;
        /// let r: Rechnung = todo!();
        /// if let Some((from, to)) = r.billing_period() {
        ///     println!("invoice period: {from} – {to}");
        /// }
        /// # }
        /// ```
        #[must_use]
        pub fn billing_period(&self) -> Option<(Date, Date)> {
            self.rechnungsperiode.as_ref()?.as_closed_range()
        }
    }
}

// ── PreisblattNetznutzung ─────────────────────────────────────────────────────

#[cfg(all(feature = "versioned", feature = "time"))]
mod preisblatt_netznutzung_impl {
    use crate::generated::v202501::PreisblattNetznutzung;
    use time::Date;

    impl PreisblattNetznutzung {
        /// Returns the price-list validity period as `(start, optional_end)`.
        ///
        /// Reads from `gueltigkeit.startdatum` / `gueltigkeit.enddatum`.
        /// Returns `None` when `gueltigkeit` is absent or `startdatum` is missing.
        /// The end date may be absent for open-ended (ongoing) price lists.
        ///
        /// ```no_run
        /// # #[cfg(all(feature = "versioned", feature = "time"))] {
        /// # use rubo4e::v202501::PreisblattNetznutzung;
        /// let p: PreisblattNetznutzung = todo!();
        /// match p.validity() {
        ///     Some((start, Some(end))) => println!("valid {start} – {end}"),
        ///     Some((start, None))      => println!("valid from {start} (open-ended)"),
        ///     None                     => println!("validity unknown"),
        /// }
        /// # }
        /// ```
        #[must_use]
        pub fn validity(&self) -> Option<(Date, Option<Date>)> {
            self.gueltigkeit.as_ref()?.as_half_open_range()
        }
    }
}
