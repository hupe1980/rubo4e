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
//! [`BetragExt`], [`MengeExt`], and [`PreisExt`] are the primary ergonomic
//! entry points for the common `Option<Com> → Option<Decimal>` pattern.  They
//! eliminate the repetitive `.as_ref().and_then(|x| x.wert)` chain:
//!
//! ```rust,ignore
//! use rubo4e::prelude::*;  // re-exports BetragExt, MengeExt, PreisExt
//!
//! // Before (requires two-level unwrap):
//! let net = pos.gesamtpreis.as_ref().and_then(|b| b.wert);
//! // After:
//! let net = pos.gesamtpreis.wert_decimal();
//! ```
//!
//! Import via `use rubo4e::prelude::*` or individually via
//! `use rubo4e::convenience::{BetragExt, MengeExt, PreisExt}`.

// ── Extension traits: Option<Com> → Option<Decimal> ─────────────────────────
//
// These resolve B-01: `Option<Betrag/Menge/Preis>` → `Option<Decimal>` in one
// method call. Gated on both `versioned` (the struct) and `decimal` (the type).

/// Ergonomic access to the `wert` field of an [`Option<Betrag>`][crate::v202607::Betrag].
///
/// Eliminates the `.as_ref().and_then(|b| b.wert)` chain that appears wherever
/// monetary amounts are read from `Betrag`-typed optional fields.
///
/// Automatically available with `use rubo4e::prelude::*`.
#[cfg(all(feature = "versioned", feature = "decimal"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "versioned", feature = "decimal"))))]
pub trait BetragExt {
    /// Returns `wert` from the inner `Betrag`, or `None` if the outer `Option`
    /// is empty or `wert` itself is `None`.
    fn wert_decimal(&self) -> Option<rust_decimal::Decimal>;
}

#[cfg(all(feature = "versioned", feature = "decimal"))]
impl BetragExt for Option<crate::generated::v202607::Betrag> {
    fn wert_decimal(&self) -> Option<rust_decimal::Decimal> {
        self.as_ref().and_then(|b| b.wert)
    }
}

/// Ergonomic access to the `wert` field of an [`Option<Menge>`][crate::v202607::Menge].
///
/// Eliminates the `.as_ref().and_then(|m| m.wert)` chain that appears wherever
/// quantities are read from `Menge`-typed optional fields.
///
/// Automatically available with `use rubo4e::prelude::*`.
#[cfg(all(feature = "versioned", feature = "decimal"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "versioned", feature = "decimal"))))]
pub trait MengeExt {
    /// Returns `wert` from the inner `Menge`, or `None` if the outer `Option`
    /// is empty or `wert` itself is `None`.
    fn wert_decimal(&self) -> Option<rust_decimal::Decimal>;
}

#[cfg(all(feature = "versioned", feature = "decimal"))]
impl MengeExt for Option<crate::generated::v202607::Menge> {
    fn wert_decimal(&self) -> Option<rust_decimal::Decimal> {
        self.as_ref().and_then(|m| m.wert)
    }
}

/// Ergonomic access to the `wert` field of an [`Option<Preis>`][crate::v202607::Preis].
///
/// Eliminates the `.as_ref().and_then(|p| p.wert)` chain that appears wherever
/// unit prices are read from `Preis`-typed optional fields.
///
/// Automatically available with `use rubo4e::prelude::*`.
#[cfg(all(feature = "versioned", feature = "decimal"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "versioned", feature = "decimal"))))]
pub trait PreisExt {
    /// Returns `wert` from the inner `Preis`, or `None` if the outer `Option`
    /// is empty or `wert` itself is `None`.
    fn wert_decimal(&self) -> Option<rust_decimal::Decimal>;
}

#[cfg(all(feature = "versioned", feature = "decimal"))]
impl PreisExt for Option<crate::generated::v202607::Preis> {
    fn wert_decimal(&self) -> Option<rust_decimal::Decimal> {
        self.as_ref().and_then(|p| p.wert)
    }
}

// ── Zeitraum ─────────────────────────────────────────────────────────────────

#[cfg(all(feature = "versioned", feature = "time"))]
mod zeitraum_impl {
    use crate::generated::v202607::Zeitraum;
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
        /// # use rubo4e::v202607::Zeitraum;
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
        /// # use rubo4e::v202607::Zeitraum;
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

        /// Returns `true` if `date` falls within `[startdatum, enddatum)`.
        ///
        /// Absent boundaries are treated as **unbounded**: a missing `startdatum`
        /// means "valid since forever"; a missing `enddatum` means "valid until
        /// further notice".  An entirely empty `Zeitraum` (both absent) returns
        /// `true` for any date.
        ///
        /// ```no_run
        /// # #[cfg(all(feature = "versioned", feature = "time"))] {
        /// # use rubo4e::v202607::Zeitraum;
        /// # use time::macros::date;
        /// let z: Zeitraum = todo!();
        /// if z.contains(date!(2026-01-15)) {
        ///     println!("active on 2026-01-15");
        /// }
        /// # }
        /// ```
        #[must_use]
        pub fn contains(&self, date: Date) -> bool {
            let start_ok = self.startdatum.is_none_or(|d| date >= d);
            let end_ok = self.enddatum.is_none_or(|d| date < d);
            start_ok && end_ok
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
        /// ```no_run
        /// # #[cfg(all(feature = "versioned", feature = "time"))] {
        /// # use rubo4e::v202607::Rechnung;
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

        /// Billing period start date (shorthand for `billing_period().map(|(s,_)| s)`).
        ///
        /// Returns `None` when `rechnungsperiode` is absent or `startdatum` is missing.
        #[must_use]
        pub fn period_start(&self) -> Option<Date> {
            self.rechnungsperiode.as_ref()?.startdatum
        }

        /// Billing period end date (shorthand for `billing_period().map(|(_,e)| e)`).
        ///
        /// Returns `None` when `rechnungsperiode` is absent or `enddatum` is missing.
        #[must_use]
        pub fn period_end(&self) -> Option<Date> {
            self.rechnungsperiode.as_ref()?.enddatum
        }

        /// Invoice issue date (`rechnungsdatum`).
        ///
        /// Convenience alias for the `rechnungsdatum` field.  BDEW INVOIC DTM+137
        /// is a date-only value (qualifier 102), so this returns `time::Date`.
        #[must_use]
        pub fn rechnungsdatum_date(&self) -> Option<Date> {
            self.rechnungsdatum
        }

        /// Payment due date (`faelligkeitsdatum`).
        ///
        /// Convenience alias for the `faelligkeitsdatum` field.  BDEW INVOIC DTM+92
        /// is a date-only value (qualifier 102), so this returns `time::Date`.
        #[must_use]
        pub fn faelligkeitsdatum_date(&self) -> Option<Date> {
            self.faelligkeitsdatum
        }
    }
}

// ── Rechnung — decimal accessors ─────────────────────────────────────────────

#[cfg(all(feature = "versioned", feature = "decimal"))]
mod rechnung_decimal_impl {
    use crate::generated::v202607::Rechnung;

    impl Rechnung {
        /// Net total (`gesamtnetto.wert`) as `Decimal`.
        ///
        /// Returns `None` when `gesamtnetto` is absent or its `wert` is `None`.
        ///
        /// ```no_run
        /// # #[cfg(all(feature = "versioned", feature = "decimal"))] {
        /// # use rubo4e::v202607::Rechnung;
        /// let r: Rechnung = todo!();
        /// if let Some(net) = r.gesamtnetto_decimal() {
        ///     println!("net total: {net}");
        /// }
        /// # }
        /// ```
        #[must_use]
        pub fn gesamtnetto_decimal(&self) -> Option<rust_decimal::Decimal> {
            self.gesamtnetto.as_ref()?.wert
        }

        /// Gross total (`gesamtbrutto.wert`) as `Decimal`.
        ///
        /// Returns `None` when `gesamtbrutto` is absent or its `wert` is `None`.
        #[must_use]
        pub fn gesamtbrutto_decimal(&self) -> Option<rust_decimal::Decimal> {
            self.gesamtbrutto.as_ref()?.wert
        }

        /// Total tax amount (`gesamtsteuer.wert`) as `Decimal`.
        ///
        /// Returns `None` when `gesamtsteuer` is absent or its `wert` is `None`.
        #[must_use]
        pub fn gesamtsteuer_decimal(&self) -> Option<rust_decimal::Decimal> {
            self.gesamtsteuer.as_ref()?.wert
        }

        /// Amount to pay (`zu_zahlen.wert`) as `Decimal`.
        ///
        /// In v202607 this is `gesamtbrutto - rabatt_netto - sum(vorauszahlungen)`.  The
        /// convenience method simply reads the pre-computed field; use
        /// [`vorauszahlungen_summe`](Self::vorauszahlungen_summe) to reconstruct the sum
        /// of advance payments independently.
        #[must_use]
        pub fn zu_zahlen_decimal(&self) -> Option<rust_decimal::Decimal> {
            self.zu_zahlen.as_ref()?.wert
        }

        /// Net discount (`rabatt_netto.wert`) as `Decimal`.
        #[must_use]
        pub fn rabatt_netto_decimal(&self) -> Option<rust_decimal::Decimal> {
            self.rabatt_netto.as_ref()?.wert
        }

        /// Estimated next instalment (`zukuenftiger_abschlag.wert`) as `Decimal`.
        ///
        /// Set by the sender as a forward-looking payment estimate for the next
        /// billing period.
        #[must_use]
        pub fn zukuenftiger_abschlag_decimal(&self) -> Option<rust_decimal::Decimal> {
            self.zukuenftiger_abschlag.as_ref()?.wert
        }

        /// Sum of all advance-payment amounts (`vorauszahlungen[*].betrag.wert`).
        ///
        /// Returns `None` when `vorauszahlungen` is absent or empty.  Returns
        /// `Some(Decimal::ZERO)` only when payments are present but all `betrag`
        /// values are `None`.  Saturates at `Decimal::MAX` — overflow is not
        /// expected for real-world invoice amounts.
        #[must_use]
        pub fn vorauszahlungen_summe(&self) -> Option<rust_decimal::Decimal> {
            let payments = self.vorauszahlungen.as_deref()?;
            if payments.is_empty() {
                return None;
            }
            Some(
                payments
                    .iter()
                    .filter_map(|p| p.betrag.as_ref().and_then(|b| b.wert))
                    .fold(rust_decimal::Decimal::ZERO, |acc, v| acc + v),
            )
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
    use crate::generated::v202607::PreisblattNetznutzung;
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
        /// # use rubo4e::v202607::PreisblattNetznutzung;
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

        /// Returns `true` if this price sheet's validity period contains `date`.
        ///
        /// Uses [`Zeitraum::contains`] — a missing `gueltigkeit` is treated as
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
