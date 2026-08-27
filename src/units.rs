//! Physical dimensions and unit arithmetic for BO4E's [`Mengeneinheit`].
//!
//! `Mengeneinheit` is one flat enum over energies, powers, reactive
//! counterparts, a volume, eleven durations, a percentage and a frequency. The
//! schema says nothing about which may be added, which convert into which, or
//! how long a `MONAT` is. This states it once:
//!
//! - [`Mengeneinheit::dimension`] groups the units into [`Dimension`]s. Two units
//!   are convertible exactly when they share one.
//! - [`Mengeneinheit::factor_to_base`] gives each unit's size in its dimension's
//!   base unit (Wh, W, varh, var, second, m³, piece, …), and
//!   [`Mengeneinheit::conversion_factor`] turns that into a factor between any
//!   two compatible units.
//! - [`Mengeneinheit::is_extensive`] separates the units that may be summed over
//!   a period (energy, volume, count) from the ones that may not (power,
//!   frequency, percentage) — the distinction that makes
//!   [`sum`](crate::timeseries::Bo4eTimeSeries::sum) meaningful for one kind of
//!   time series and nonsense for the other.
//! - [`Mengeneinheit::energy_unit`] pairs a power unit with the energy unit an
//!   integration over time lands in — `KW` × hours → `KWH`.
//!
//! # Calendar units are refused, not averaged
//!
//! `MONAT`, `QUARTAL`, `HALBJAHR` and `JAHR` have no fixed length. They are in
//! [`Dimension::Time`] and compare as time units, but they have **no**
//! [`factor_to_base`](Mengeneinheit::factor_to_base) and **no**
//! [`exact_duration`](Mengeneinheit::exact_duration): converting one needs a
//! start date the `Menge` does not carry.
//! [`is_calendar`](Mengeneinheit::is_calendar) names them. This is the same call
//! [`iso8601_duration`](crate::iso8601_duration) makes about `P1Y` / `P1M`, for
//! the same reason.
//!
//! ```
//! # #[cfg(all(feature = "versioned", feature = "decimal"))] {
//! use rubo4e::current::Mengeneinheit;
//! use rubo4e::units::Dimension;
//! use rust_decimal::Decimal;
//!
//! assert_eq!(Mengeneinheit::Kwh.dimension(), Some(Dimension::Energy));
//! assert_eq!(Mengeneinheit::Kw.dimension(),  Some(Dimension::Power));
//!
//! // Compatible units convert; incompatible ones do not.
//! assert_eq!(
//!     Mengeneinheit::Mwh.conversion_factor(Mengeneinheit::Kwh),
//!     Some(Decimal::from(1000)),
//! );
//! assert_eq!(Mengeneinheit::Kwh.conversion_factor(Mengeneinheit::Kw), None);
//!
//! // A month has no length, so it has no factor.
//! assert!(Mengeneinheit::Monat.is_calendar());
//! assert_eq!(Mengeneinheit::Monat.factor_to_base(), None);
//! # }
//! ```

use crate::generated::v202607::Mengeneinheit;

// ─── Dimension ───────────────────────────────────────────────────────────────

/// The physical dimension a [`Mengeneinheit`] measures.
///
/// Two units are convertible exactly when
/// [`dimension`](Mengeneinheit::dimension) returns the same variant for both;
/// [`conversion_factor`](Mengeneinheit::conversion_factor) enforces that.
///
/// Each dimension has a **base unit** — the one whose
/// [`factor_to_base`](Mengeneinheit::factor_to_base) is `1` — named by
/// [`base_unit`](Dimension::base_unit).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum Dimension {
    /// Active energy — `WH`, `KWH`, `MWH`. Base unit `WH`.
    Energy,
    /// Active power — `W`, `KW`, `MW`. Base unit `W`.
    Power,
    /// Reactive energy — `VARH`, `KVARH`. Base unit `VARH`.
    ReactiveEnergy,
    /// Reactive power — `VAR`, `KVAR`. Base unit `VAR`.
    ReactivePower,
    /// Volume — `KUBIKMETER`, the gas market's metered quantity before it is
    /// converted to energy with a Zustandszahl and a Brennwert.
    Volume,
    /// Duration — `SEKUNDE` … `JAHR`. Base unit `SEKUNDE`.
    ///
    /// The four calendar units are in this dimension but have no factor; see
    /// [`is_calendar`](Mengeneinheit::is_calendar).
    Time,
    /// A count of things — `STUECK`.
    Count,
    /// A ratio expressed in percent — `PROZENT`.
    Ratio,
    /// Frequency — `HZ`.
    Frequency,
    /// Energy per temperature difference — `KWHK` (kWh/K), the heating-degree
    /// coefficient a temperature-corrected consumption forecast is built on.
    EnergyPerTemperature,
    /// A quantity with no unit — `DIMENSIONSLOS`.
    Dimensionless,
}

impl Dimension {
    /// The unit every [`factor_to_base`](Mengeneinheit::factor_to_base) in this
    /// dimension is expressed in.
    #[must_use]
    pub const fn base_unit(self) -> Mengeneinheit {
        match self {
            Dimension::Energy => Mengeneinheit::Wh,
            Dimension::Power => Mengeneinheit::W,
            Dimension::ReactiveEnergy => Mengeneinheit::Varh,
            Dimension::ReactivePower => Mengeneinheit::Var,
            Dimension::Volume => Mengeneinheit::Kubikmeter,
            Dimension::Time => Mengeneinheit::Sekunde,
            Dimension::Count => Mengeneinheit::Stueck,
            Dimension::Ratio => Mengeneinheit::Prozent,
            Dimension::Frequency => Mengeneinheit::Hz,
            Dimension::EnergyPerTemperature => Mengeneinheit::Kwhk,
            Dimension::Dimensionless => Mengeneinheit::Dimensionslos,
        }
    }

    /// Whether a quantity in this dimension may be **summed over a period**.
    ///
    /// `true` for energy, reactive energy, volume and count: adding two of them
    /// gives the total over the union of their intervals. `false` for power,
    /// frequency, percentage and the rest: those are instantaneous or
    /// normalising, and adding them produces a number with no meaning. Integrate
    /// a power series instead — see
    /// [`integrate`](crate::timeseries::Bo4eTimeSeries::integrate).
    #[must_use]
    pub const fn is_extensive(self) -> bool {
        matches!(
            self,
            Dimension::Energy
                | Dimension::ReactiveEnergy
                | Dimension::Volume
                | Dimension::Count
                | Dimension::Time
        )
    }
}

// ─── Mengeneinheit ───────────────────────────────────────────────────────────

impl Mengeneinheit {
    /// The [`Dimension`] this unit measures, or `None` for the forward-compatibility
    /// `Unknown` catch-all.
    ///
    /// ```
    /// # #[cfg(feature = "versioned")] {
    /// use rubo4e::current::Mengeneinheit;
    /// use rubo4e::units::Dimension;
    ///
    /// assert_eq!(Mengeneinheit::Mwh.dimension(), Some(Dimension::Energy));
    /// assert_eq!(Mengeneinheit::Kvar.dimension(), Some(Dimension::ReactivePower));
    /// assert_eq!(Mengeneinheit::Unknown.dimension(), None);
    /// # }
    /// ```
    #[must_use]
    pub const fn dimension(self) -> Option<Dimension> {
        Some(match self {
            Mengeneinheit::Wh | Mengeneinheit::Kwh | Mengeneinheit::Mwh => Dimension::Energy,
            Mengeneinheit::W | Mengeneinheit::Kw | Mengeneinheit::Mw => Dimension::Power,
            Mengeneinheit::Varh | Mengeneinheit::Kvarh => Dimension::ReactiveEnergy,
            Mengeneinheit::Var | Mengeneinheit::Kvar => Dimension::ReactivePower,
            Mengeneinheit::Kubikmeter => Dimension::Volume,
            Mengeneinheit::Sekunde
            | Mengeneinheit::Minute
            | Mengeneinheit::ViertelStunde
            | Mengeneinheit::Stunde
            | Mengeneinheit::Tag
            | Mengeneinheit::Woche
            | Mengeneinheit::Monat
            | Mengeneinheit::Quartal
            | Mengeneinheit::Halbjahr
            | Mengeneinheit::Jahr => Dimension::Time,
            Mengeneinheit::Stueck => Dimension::Count,
            Mengeneinheit::Prozent => Dimension::Ratio,
            Mengeneinheit::Hz => Dimension::Frequency,
            Mengeneinheit::Kwhk => Dimension::EnergyPerTemperature,
            Mengeneinheit::Dimensionslos => Dimension::Dimensionless,
            Mengeneinheit::Unknown => return None,
        })
    }

    /// Whether this is one of the four **calendar** durations — `MONAT`,
    /// `QUARTAL`, `HALBJAHR`, `JAHR` — whose length depends on when they start.
    ///
    /// They have no [`factor_to_base`](Self::factor_to_base) and no
    /// [`exact_duration`](Self::exact_duration): resolve them against a concrete
    /// date instead of taking a nominal average.
    #[must_use]
    pub const fn is_calendar(self) -> bool {
        matches!(
            self,
            Mengeneinheit::Monat
                | Mengeneinheit::Quartal
                | Mengeneinheit::Halbjahr
                | Mengeneinheit::Jahr
        )
    }

    /// Whether a quantity in this unit may be summed over a period — see
    /// [`Dimension::is_extensive`].
    ///
    /// `false` for the `Unknown` catch-all: nothing is known about it, and
    /// treating it as summable is the dangerous default.
    #[must_use]
    pub const fn is_extensive(self) -> bool {
        match self.dimension() {
            Some(d) => d.is_extensive(),
            None => false,
        }
    }

    /// The energy unit a power quantity in this unit integrates into: `W` → `WH`,
    /// `KW` → `KWH`, `MW` → `MWH`, and the reactive pair likewise.
    ///
    /// `None` when this is not a power unit.
    ///
    /// ```
    /// # #[cfg(feature = "versioned")] {
    /// use rubo4e::current::Mengeneinheit;
    ///
    /// assert_eq!(Mengeneinheit::Kw.energy_unit(), Some(Mengeneinheit::Kwh));
    /// assert_eq!(Mengeneinheit::Kwh.energy_unit(), None);
    /// # }
    /// ```
    #[must_use]
    pub const fn energy_unit(self) -> Option<Mengeneinheit> {
        Some(match self {
            Mengeneinheit::W => Mengeneinheit::Wh,
            Mengeneinheit::Kw => Mengeneinheit::Kwh,
            Mengeneinheit::Mw => Mengeneinheit::Mwh,
            Mengeneinheit::Var => Mengeneinheit::Varh,
            Mengeneinheit::Kvar => Mengeneinheit::Kvarh,
            _ => return None,
        })
    }

    /// The power unit an energy quantity in this unit differentiates into — the
    /// inverse of [`energy_unit`](Self::energy_unit).
    ///
    /// `None` when this is not an energy unit. `MWH` has a partner (`MW`); `WH`
    /// and `KWH` do too.
    #[must_use]
    pub const fn power_unit(self) -> Option<Mengeneinheit> {
        Some(match self {
            Mengeneinheit::Wh => Mengeneinheit::W,
            Mengeneinheit::Kwh => Mengeneinheit::Kw,
            Mengeneinheit::Mwh => Mengeneinheit::Mw,
            Mengeneinheit::Varh => Mengeneinheit::Var,
            Mengeneinheit::Kvarh => Mengeneinheit::Kvar,
            _ => return None,
        })
    }

    /// This unit's size in its dimension's [`base_unit`](Dimension::base_unit).
    ///
    /// `MWH` is `1_000_000` Wh; `VIERTEL_STUNDE` is `900` seconds. `None` for the
    /// `Unknown` catch-all and for the four [calendar](Self::is_calendar) units,
    /// which have no fixed size.
    #[cfg(feature = "decimal")]
    #[cfg_attr(docsrs, doc(cfg(feature = "decimal")))]
    #[must_use]
    pub fn factor_to_base(self) -> Option<rust_decimal::Decimal> {
        use rust_decimal::Decimal;
        Some(Decimal::from(match self {
            // Energy, power, and their reactive counterparts: SI prefixes.
            Mengeneinheit::Wh
            | Mengeneinheit::W
            | Mengeneinheit::Varh
            | Mengeneinheit::Var
            | Mengeneinheit::Kubikmeter
            | Mengeneinheit::Stueck
            | Mengeneinheit::Prozent
            | Mengeneinheit::Hz
            | Mengeneinheit::Kwhk
            | Mengeneinheit::Dimensionslos
            | Mengeneinheit::Sekunde => 1u32,
            Mengeneinheit::Kwh | Mengeneinheit::Kw | Mengeneinheit::Kvarh | Mengeneinheit::Kvar => {
                1_000
            }
            Mengeneinheit::Mwh | Mengeneinheit::Mw => 1_000_000,
            // Exact durations, in seconds.
            Mengeneinheit::Minute => 60,
            Mengeneinheit::ViertelStunde => 900,
            Mengeneinheit::Stunde => 3_600,
            Mengeneinheit::Tag => 86_400,
            Mengeneinheit::Woche => 604_800,
            // No fixed length, and the `Unknown` catch-all.
            Mengeneinheit::Monat
            | Mengeneinheit::Quartal
            | Mengeneinheit::Halbjahr
            | Mengeneinheit::Jahr
            | Mengeneinheit::Unknown => return None,
        }))
    }

    /// The factor that turns a quantity in `self` into one in `target`.
    ///
    /// `Some(f)` such that `value_in_self * f == value_in_target`, when the two
    /// units share a [`Dimension`] and both have a
    /// [`factor_to_base`](Self::factor_to_base). `None` otherwise — a
    /// dimension mismatch, a calendar unit, or the `Unknown` catch-all.
    ///
    /// # Precision
    ///
    /// Every factor between two SI-prefixed units is a power of ten and exact.
    /// A ratio between two *time* units need not be: `SEKUNDE` → `MINUTE` is
    /// `1/60`, which `Decimal` rounds at 28 significant digits, and multiplying
    /// by the rounded value leaves 120 seconds as `2.000…004` minutes. Prefer
    /// [`Menge::convert_to`](crate::current::Menge::convert_to), which scales
    /// through the base unit in one multiply and one divide and lands on `2`.
    ///
    /// ```
    /// # #[cfg(all(feature = "versioned", feature = "decimal"))] {
    /// use rubo4e::current::Mengeneinheit;
    /// use rust_decimal::Decimal;
    ///
    /// assert_eq!(
    ///     Mengeneinheit::Mwh.conversion_factor(Mengeneinheit::Kwh),
    ///     Some(Decimal::from(1000)),
    /// );
    /// // Energy is not power, however similar the spelling.
    /// assert_eq!(Mengeneinheit::Kwh.conversion_factor(Mengeneinheit::Kw), None);
    /// # }
    /// ```
    #[cfg(feature = "decimal")]
    #[cfg_attr(docsrs, doc(cfg(feature = "decimal")))]
    #[must_use]
    pub fn conversion_factor(self, target: Mengeneinheit) -> Option<rust_decimal::Decimal> {
        if self.dimension()? != target.dimension()? {
            return None;
        }
        self.factor_to_base()?.checked_div(target.factor_to_base()?)
    }

    /// The length of **one** of this unit, when it is an exact duration.
    ///
    /// `Some` for `SEKUNDE`, `MINUTE`, `VIERTEL_STUNDE`, `STUNDE`, `TAG` and
    /// `WOCHE`; `None` for every other unit, the four
    /// [calendar](Self::is_calendar) ones included.
    ///
    /// ```
    /// # #[cfg(all(feature = "versioned", feature = "time"))] {
    /// use rubo4e::current::Mengeneinheit;
    /// use time::Duration;
    ///
    /// assert_eq!(Mengeneinheit::ViertelStunde.exact_duration(), Some(Duration::minutes(15)));
    /// assert_eq!(Mengeneinheit::Jahr.exact_duration(), None);
    /// assert_eq!(Mengeneinheit::Kwh.exact_duration(), None);
    /// # }
    /// ```
    #[cfg(feature = "time")]
    #[cfg_attr(docsrs, doc(cfg(feature = "time")))]
    #[must_use]
    pub const fn exact_duration(self) -> Option<time::Duration> {
        Some(time::Duration::seconds(match self {
            Mengeneinheit::Sekunde => 1,
            Mengeneinheit::Minute => 60,
            Mengeneinheit::ViertelStunde => 900,
            Mengeneinheit::Stunde => 3_600,
            Mengeneinheit::Tag => 86_400,
            Mengeneinheit::Woche => 604_800,
            _ => return None,
        }))
    }
}

// ─── Menge ───────────────────────────────────────────────────────────────────

#[cfg(feature = "decimal")]
mod menge_impl {
    use super::Dimension;
    use crate::generated::v202607::{Menge, Mengeneinheit};

    impl Menge {
        /// The [`Dimension`] of this quantity, or `None` when `einheit` is absent
        /// or `Unknown`.
        #[must_use]
        pub fn dimension(&self) -> Option<Dimension> {
            self.einheit?.dimension()
        }

        /// Converts this quantity into `target`, keeping every other field.
        ///
        /// `None` when `wert` or `einheit` is absent, when the two units are not
        /// [compatible](Mengeneinheit::conversion_factor), or when the product
        /// overflows `Decimal`.
        ///
        /// ```
        /// # #[cfg(all(feature = "versioned", feature = "decimal"))] {
        /// use rubo4e::current::{Menge, Mengeneinheit};
        /// use rust_decimal::Decimal;
        ///
        /// let mwh = Menge {
        ///     wert: Some(Decimal::from(2)),
        ///     einheit: Some(Mengeneinheit::Mwh),
        ///     ..Default::default()
        /// };
        /// let kwh = mwh.convert_to(Mengeneinheit::Kwh).unwrap();
        /// assert_eq!(kwh.wert, Some(Decimal::from(2000)));
        /// assert_eq!(kwh.einheit, Some(Mengeneinheit::Kwh));
        ///
        /// // Not a conversion — a different dimension.
        /// assert!(mwh.convert_to(Mengeneinheit::Kw).is_none());
        /// # }
        /// ```
        #[must_use]
        pub fn convert_to(&self, target: Mengeneinheit) -> Option<Menge> {
            let source = self.einheit?;
            if source.dimension()? != target.dimension()? {
                return None;
            }
            // Scale up then down, rather than by a pre-rounded
            // `conversion_factor`: `SEKUNDE` → `MINUTE` is 1/60, and rounding
            // that first turns 120 seconds into 2.000…004 minutes.
            let wert = self
                .wert?
                .checked_mul(source.factor_to_base()?)?
                .checked_div(target.factor_to_base()?)?;
            Some(Menge {
                wert: Some(wert),
                einheit: Some(target),
                ..self.clone()
            })
        }

        /// Reads this quantity as an exact [`time::Duration`] — `wert` many of
        /// `einheit`.
        ///
        /// This is what turns `Lastgang.zeit_intervall_laenge`
        /// (`{"wert": 15, "einheit": "MINUTE"}`) into the 15 minutes the payload
        /// means.
        ///
        /// `None` when either field is absent, when `einheit` is not a duration
        /// or is one of the four [calendar](Mengeneinheit::is_calendar) units, or
        /// when the result does not fit a `Duration`.
        ///
        /// ```
        /// # #[cfg(all(feature = "versioned", feature = "decimal", feature = "time"))] {
        /// use rubo4e::current::{Menge, Mengeneinheit};
        /// use rust_decimal::Decimal;
        /// use time::Duration;
        ///
        /// let interval = Menge {
        ///     wert: Some(Decimal::from(15)),
        ///     einheit: Some(Mengeneinheit::Minute),
        ///     ..Default::default()
        /// };
        /// assert_eq!(interval.as_duration(), Some(Duration::minutes(15)));
        ///
        /// // A month has no fixed length, so it is refused rather than averaged.
        /// let month = Menge {
        ///     wert: Some(Decimal::ONE),
        ///     einheit: Some(Mengeneinheit::Monat),
        ///     ..Default::default()
        /// };
        /// assert_eq!(month.as_duration(), None);
        /// # }
        /// ```
        #[cfg(feature = "time")]
        #[cfg_attr(docsrs, doc(cfg(feature = "time")))]
        #[must_use]
        pub fn as_duration(&self) -> Option<time::Duration> {
            if self.einheit?.dimension()? != Dimension::Time {
                return None;
            }
            let seconds = self.wert?.checked_mul(self.einheit?.factor_to_base()?)?;
            super::decimal_to_duration(seconds)
        }

        /// Integrates this **power** quantity over `duration`, giving the energy
        /// it delivers.
        ///
        /// `KW` over 15 minutes is `0.25 ×` the value in `KWH`; the unit follows
        /// [`Mengeneinheit::energy_unit`]. This is the step from a `Lastgang`
        /// carrying power to the energy an invoice bills.
        ///
        /// `None` when `wert` or `einheit` is absent, when `einheit` is not a
        /// power unit, or when the product overflows.
        ///
        /// ```
        /// # #[cfg(all(feature = "versioned", feature = "decimal", feature = "time"))] {
        /// use rubo4e::current::{Menge, Mengeneinheit};
        /// use rust_decimal::Decimal;
        /// use time::Duration;
        ///
        /// let load = Menge {
        ///     wert: Some(Decimal::from(400)),          // 400 kW
        ///     einheit: Some(Mengeneinheit::Kw),
        ///     ..Default::default()
        /// };
        /// let energy = load.energy_over(Duration::minutes(15)).unwrap();
        /// assert_eq!(energy.wert, Some(Decimal::from(100)));   // 100 kWh
        /// assert_eq!(energy.einheit, Some(Mengeneinheit::Kwh));
        /// # }
        /// ```
        #[cfg(feature = "time")]
        #[cfg_attr(docsrs, doc(cfg(feature = "time")))]
        #[must_use]
        pub fn energy_over(&self, duration: time::Duration) -> Option<Menge> {
            let target = self.einheit?.energy_unit()?;
            let hours = super::duration_to_hours(duration)?;
            let wert = self.wert?.checked_mul(hours)?;
            Some(Menge {
                wert: Some(wert),
                einheit: Some(target),
                ..self.clone()
            })
        }
    }
}

/// A `Decimal` count of seconds for `duration`, exact to the nanosecond.
///
/// `None` for a duration past ±292 years, whose nanosecond count leaves `i64`.
#[cfg(all(feature = "decimal", feature = "time"))]
pub(crate) fn duration_to_seconds(duration: time::Duration) -> Option<rust_decimal::Decimal> {
    let nanos = i64::try_from(duration.whole_nanoseconds()).ok()?;
    rust_decimal::Decimal::try_new(nanos, 9).ok()
}

/// A `Decimal` count of hours for `duration`, exact to the nanosecond.
#[cfg(all(feature = "decimal", feature = "time"))]
pub(crate) fn duration_to_hours(duration: time::Duration) -> Option<rust_decimal::Decimal> {
    duration_to_seconds(duration)?.checked_div(rust_decimal::Decimal::from(3_600))
}

/// The inverse of [`duration_to_seconds`], truncating below the nanosecond.
#[cfg(all(feature = "decimal", feature = "time"))]
pub(crate) fn decimal_to_duration(seconds: rust_decimal::Decimal) -> Option<time::Duration> {
    use rust_decimal::prelude::ToPrimitive;
    let nanos = seconds.checked_mul(rust_decimal::Decimal::from(1_000_000_000u32))?;
    Some(time::Duration::nanoseconds(nanos.trunc().to_i64()?))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every schema-defined unit must be classified — a new one in the next
    /// release fails here rather than silently answering `None`.
    #[test]
    fn every_known_unit_has_a_dimension() {
        for unit in Mengeneinheit::iter_known() {
            assert!(
                unit.dimension().is_some(),
                "{} has no Dimension",
                unit.as_wire()
            );
        }
        assert_eq!(Mengeneinheit::Unknown.dimension(), None);
    }

    /// A dimension's base unit must be in that dimension and have factor 1.
    #[cfg(feature = "decimal")]
    #[test]
    fn base_units_are_self_consistent() {
        use rust_decimal::Decimal;
        for unit in Mengeneinheit::iter_known() {
            let Some(dim) = unit.dimension() else {
                continue;
            };
            let base = dim.base_unit();
            assert_eq!(base.dimension(), Some(dim), "{base:?} left its dimension");
            assert_eq!(base.factor_to_base(), Some(Decimal::ONE), "{base:?}");
        }
    }

    /// Exactly the four calendar units lack a factor.
    #[cfg(feature = "decimal")]
    #[test]
    fn only_calendar_units_lack_a_factor() {
        for unit in Mengeneinheit::iter_known() {
            assert_eq!(
                unit.factor_to_base().is_none(),
                unit.is_calendar(),
                "{} disagrees with is_calendar()",
                unit.as_wire()
            );
        }
    }

    #[cfg(feature = "decimal")]
    #[test]
    fn conversion_round_trips_within_a_dimension() {
        use rust_decimal::Decimal;
        for a in Mengeneinheit::iter_known() {
            for b in Mengeneinheit::iter_known() {
                let factor = a.conversion_factor(b);
                if a.dimension() != b.dimension() || a.is_calendar() || b.is_calendar() {
                    assert!(factor.is_none(), "{a:?} -> {b:?} should not convert");
                    continue;
                }
                let there = factor.expect("same dimension, both exact");
                let back = b.conversion_factor(a).expect("symmetric");
                // `there * back` is 1 up to `Decimal`'s rounding: 1/60 has no
                // exact decimal form. The tolerance is the last of the 28
                // significant digits, which is what rounding can cost.
                assert!(
                    (there * back - Decimal::ONE).abs() < Decimal::new(1, 20),
                    "{a:?} <-> {b:?}: {there} * {back}"
                );
            }
        }
    }

    /// The energy/power pairing must be a bijection on the units that have one.
    #[test]
    fn energy_and_power_units_pair_up() {
        for unit in Mengeneinheit::iter_known() {
            if let Some(energy) = unit.energy_unit() {
                assert_eq!(energy.power_unit(), Some(unit), "{unit:?}");
                assert!(energy.is_extensive(), "{energy:?} must be summable");
            }
            if let Some(power) = unit.power_unit() {
                assert_eq!(power.energy_unit(), Some(unit), "{unit:?}");
            }
        }
    }

    /// A power unit is never summable and an energy unit always is — the
    /// distinction `Bo4eTimeSeries::sum` relies on.
    #[test]
    fn extensivity_matches_the_dimension() {
        assert!(Mengeneinheit::Kwh.is_extensive());
        assert!(Mengeneinheit::Kubikmeter.is_extensive());
        assert!(!Mengeneinheit::Kw.is_extensive());
        assert!(!Mengeneinheit::Hz.is_extensive());
        assert!(!Mengeneinheit::Prozent.is_extensive());
        // Nothing is known about the catch-all, so it is not summable.
        assert!(!Mengeneinheit::Unknown.is_extensive());
    }

    #[cfg(feature = "time")]
    #[test]
    fn exact_durations_agree_with_the_factor() {
        for unit in Mengeneinheit::iter_known() {
            let is_exact_time = unit.dimension() == Some(Dimension::Time) && !unit.is_calendar();
            assert_eq!(
                unit.exact_duration().is_some(),
                is_exact_time,
                "{} disagrees",
                unit.as_wire()
            );
            #[cfg(feature = "decimal")]
            if let Some(d) = unit.exact_duration() {
                assert_eq!(
                    super::duration_to_seconds(d),
                    unit.factor_to_base(),
                    "{}",
                    unit.as_wire()
                );
            }
        }
    }

    /// The two-step scaling in `convert_to` must land on the exact value even
    /// where the scalar factor cannot be written down.
    #[cfg(feature = "decimal")]
    #[test]
    fn converting_through_the_base_unit_stays_exact() {
        use crate::generated::v202607::Menge;
        use rust_decimal::Decimal;

        let two_minutes = Menge {
            wert: Some(Decimal::from(120)),
            einheit: Some(Mengeneinheit::Sekunde),
            ..Default::default()
        }
        .convert_to(Mengeneinheit::Minute)
        .unwrap();
        assert_eq!(two_minutes.wert, Some(Decimal::from(2)));

        // …and the scalar factor on its own is the one that cannot.
        let rounded = Decimal::from(120)
            * Mengeneinheit::Sekunde
                .conversion_factor(Mengeneinheit::Minute)
                .unwrap();
        assert_ne!(rounded, Decimal::from(2));
    }

    #[cfg(all(feature = "decimal", feature = "time"))]
    #[test]
    fn power_integrates_into_energy() {
        use crate::generated::v202607::Menge;
        use rust_decimal::Decimal;

        let load = Menge {
            wert: Some(Decimal::from(400)),
            einheit: Some(Mengeneinheit::Kw),
            ..Default::default()
        };
        let energy = load.energy_over(time::Duration::minutes(15)).unwrap();
        assert_eq!(energy.wert, Some(Decimal::from(100)));
        assert_eq!(energy.einheit, Some(Mengeneinheit::Kwh));

        // An energy quantity has no power partner, so it does not integrate.
        assert!(energy.energy_over(time::Duration::HOUR).is_none());
    }

    #[cfg(all(feature = "decimal", feature = "time"))]
    #[test]
    fn menge_reads_as_a_duration() {
        use crate::generated::v202607::Menge;
        use rust_decimal::Decimal;

        let quarter = Menge {
            wert: Some(Decimal::ONE),
            einheit: Some(Mengeneinheit::ViertelStunde),
            ..Default::default()
        };
        assert_eq!(quarter.as_duration(), Some(time::Duration::minutes(15)));

        // A fractional count is exact too.
        let half_hour = Menge {
            wert: Some(Decimal::new(5, 1)),
            einheit: Some(Mengeneinheit::Stunde),
            ..Default::default()
        };
        assert_eq!(half_hour.as_duration(), Some(time::Duration::minutes(30)));

        // Not a duration at all.
        let energy = Menge {
            wert: Some(Decimal::ONE),
            einheit: Some(Mengeneinheit::Kwh),
            ..Default::default()
        };
        assert_eq!(energy.as_duration(), None);
    }
}
