//! ISO 8601 duration parsing for BO4E's `dauer` fields.
//!
//! `Zeitraum.dauer` is an ISO 8601 string (`"P1DT30H4S"`) that neither serde nor
//! the `time` crate parses, so the field stays a `String` and this reads it.
//!
//! # Years and months are refused, not guessed
//!
//! `P1Y` and `P1M` have no fixed length — a year is 365 or 366 days, a month 28
//! to 31 — so converting either needs a start date the `dauer` does not carry.
//! They return
//! [`CalendarComponent`](crate::iso8601_duration::Iso8601DurationError::CalendarComponent)
//! rather than a
//! nominal average. Weeks and below are exact and parse fine.
//!
//! ```
//! # #[cfg(feature = "time")] {
//! use rubo4e::iso8601_duration::{parse, Iso8601DurationError};
//!
//! // BO4E's own example: 1 day + 30 hours + 4 seconds.
//! use time::Duration;
//! assert_eq!(
//!     parse("P1DT30H4S").unwrap(),
//!     Duration::days(1) + Duration::hours(30) + Duration::seconds(4),
//! );
//! assert_eq!(parse("PT15M").unwrap(), Duration::minutes(15));
//! assert_eq!(parse("P1W").unwrap(), Duration::days(7), "a week is exactly 7 days");
//!
//! // A calendar component has no fixed length, so it is refused.
//! assert_eq!(parse("P1M"), Err(Iso8601DurationError::CalendarComponent { unit: 'M' }));
//! # }
//! ```

use thiserror::Error;

/// Why an ISO 8601 duration string could not be turned into a [`time::Duration`].
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum Iso8601DurationError {
    /// The string is not an ISO 8601 duration at all.
    #[error("not an ISO 8601 duration: {reason}")]
    Malformed {
        /// What specifically did not parse.
        reason: &'static str,
    },

    /// The duration carries a years or months component, whose length depends on
    /// when it starts. Resolve it against a concrete date instead.
    #[error(
        "'{unit}' has no fixed length ({}); resolve it against a start date instead",
        if *unit == 'Y' { "a year is 365 or 366 days" } else { "a month is 28 to 31 days" }
    )]
    CalendarComponent {
        /// `'Y'` or `'M'`.
        unit: char,
    },

    /// The value is real but larger than [`time::Duration`] can hold.
    #[error("duration is out of range for time::Duration")]
    OutOfRange,
}

/// Parses an ISO 8601 duration into an exact [`time::Duration`].
///
/// Accepts `[-]P[nW]` and `[-]P[nD][T[nH][nM][nS]]`. A decimal fraction is
/// permitted on the **last** component present (`PT0.5S`, `P1.5D`, `PT1.5H`) and
/// rejected anywhere else (`P1.5DT1H`), as ISO 8601 requires. Both `,` and `.`
/// are accepted as the decimal separator, which ISO 8601 also allows.
///
/// # Errors
///
/// - [`Iso8601DurationError::CalendarComponent`] for `Y` or `M` in the date part
///   — see the [module docs](self) for why those are refused rather than
///   approximated. Note that `M` *after* the `T` is minutes and parses normally.
/// - [`Iso8601DurationError::Malformed`] for anything that is not a duration.
/// - [`Iso8601DurationError::OutOfRange`] if the value overflows.
pub fn parse(s: &str) -> Result<time::Duration, Iso8601DurationError> {
    fn malformed(reason: &'static str) -> Iso8601DurationError {
        Iso8601DurationError::Malformed { reason }
    }

    let (negative, rest) = match s.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, s.strip_prefix('+').unwrap_or(s)),
    };
    let body = rest
        .strip_prefix('P')
        .ok_or_else(|| malformed("must start with 'P'"))?;
    if body.is_empty() {
        return Err(malformed("'P' with no components"));
    }

    let (date_part, time_part) = match body.split_once('T') {
        Some((d, t)) => {
            if t.is_empty() {
                return Err(malformed("'T' with no time components"));
            }
            (d, Some(t))
        }
        None => (body, None),
    };

    let mut seconds = 0f64;
    let mut any = false;
    // ISO 8601 allows a decimal fraction on the **smallest** component only, so
    // once one appears nothing may follow it. `P1.5DT1H` states a day and a half
    // and then an hour, which is not a duration any other implementation reads
    // the same way.
    let mut after_fraction = false;

    for (value, unit, fractional) in Components::new(date_part) {
        if after_fraction {
            return Err(malformed(
                "a decimal fraction is only allowed on the last component",
            ));
        }
        let value = value?;
        any = true;
        after_fraction = fractional;
        seconds += match unit {
            'Y' | 'M' => return Err(Iso8601DurationError::CalendarComponent { unit }),
            'W' => value * 604_800.0, // a week is exactly 7 days
            'D' => value * 86_400.0,
            _ => return Err(malformed("unknown unit in the date part")),
        };
    }

    if let Some(time_part) = time_part {
        for (value, unit, fractional) in Components::new(time_part) {
            if after_fraction {
                return Err(malformed(
                    "a decimal fraction is only allowed on the last component",
                ));
            }
            let value = value?;
            any = true;
            after_fraction = fractional;
            seconds += match unit {
                'H' => value * 3_600.0,
                // After the 'T', 'M' is minutes — the one place the letter is
                // unambiguous, and the reason the split has to happen first.
                'M' => value * 60.0,
                'S' => value,
                _ => return Err(malformed("unknown unit in the time part")),
            };
        }
    }

    if !any {
        return Err(malformed("no components"));
    }
    if !seconds.is_finite() {
        return Err(Iso8601DurationError::OutOfRange);
    }

    let signed = if negative { -seconds } else { seconds };
    time::Duration::checked_seconds_f64(signed).ok_or(Iso8601DurationError::OutOfRange)
}

/// Splits `"1DT30H"`-style text into `(value, unit)` pairs, left to right.
struct Components<'a> {
    rest: &'a str,
}

impl<'a> Components<'a> {
    fn new(rest: &'a str) -> Self {
        Self { rest }
    }
}

impl Iterator for Components<'_> {
    /// `(value, unit, carried_a_decimal_fraction)`.
    type Item = (Result<f64, Iso8601DurationError>, char, bool);

    fn next(&mut self) -> Option<Self::Item> {
        if self.rest.is_empty() {
            return None;
        }
        let split = self
            .rest
            .find(|c: char| c.is_ascii_alphabetic())
            .unwrap_or(self.rest.len());
        let (digits, tail) = self.rest.split_at(split);
        let mut chars = tail.chars();
        let Some(unit) = chars.next() else {
            self.rest = "";
            return Some((
                Err(Iso8601DurationError::Malformed {
                    reason: "a value with no unit",
                }),
                '?',
                false,
            ));
        };
        self.rest = chars.as_str();

        // ISO 8601 permits a comma as the decimal separator.
        let normalised = digits.replace(',', ".");
        let fractional = normalised.contains('.');
        let value = if normalised.is_empty() {
            Err(Iso8601DurationError::Malformed {
                reason: "a unit with no value",
            })
        } else if !normalised.bytes().all(|b| b.is_ascii_digit() || b == b'.') {
            Err(Iso8601DurationError::Malformed {
                reason: "a component value must be a non-negative number",
            })
        } else {
            normalised
                .parse::<f64>()
                .map_err(|_| Iso8601DurationError::Malformed {
                    reason: "a component value must be a non-negative number",
                })
        };
        Some((value, unit, fractional))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::Duration;

    #[test]
    fn parses_the_bo4e_example() {
        // "P1DT30H4S" — the string the schema itself gives for `Zeitraum.dauer`.
        assert_eq!(
            parse("P1DT30H4S"),
            Ok(Duration::days(1) + Duration::hours(30) + Duration::seconds(4))
        );
    }

    #[test]
    fn parses_each_exact_unit() {
        assert_eq!(parse("P3D"), Ok(Duration::days(3)));
        assert_eq!(parse("PT4H"), Ok(Duration::hours(4)));
        assert_eq!(parse("PT15M"), Ok(Duration::minutes(15)));
        assert_eq!(parse("PT30S"), Ok(Duration::seconds(30)));
        assert_eq!(parse("P1W"), Ok(Duration::days(7)));
        assert_eq!(parse("P2W3DT4H5M6S"), Ok(Duration::seconds(1_483_506)));
    }

    /// The one genuinely ambiguous letter: `M` is months before the `T` and
    /// minutes after it.
    #[test]
    fn m_means_months_before_t_and_minutes_after() {
        assert_eq!(
            parse("P1M"),
            Err(Iso8601DurationError::CalendarComponent { unit: 'M' })
        );
        assert_eq!(parse("PT1M"), Ok(Duration::minutes(1)));
        // …and a duration carrying both resolves each in its own half.
        assert_eq!(
            parse("P1MT1M"),
            Err(Iso8601DurationError::CalendarComponent { unit: 'M' })
        );
    }

    #[test]
    fn years_and_months_are_refused_rather_than_approximated() {
        assert_eq!(
            parse("P1Y"),
            Err(Iso8601DurationError::CalendarComponent { unit: 'Y' })
        );
        let err = parse("P1Y").unwrap_err().to_string();
        assert!(err.contains("365 or 366"), "unhelpful message: {err}");
        let err = parse("P2M").unwrap_err().to_string();
        assert!(err.contains("28 to 31"), "unhelpful message: {err}");
    }

    #[test]
    fn accepts_a_decimal_fraction_with_either_separator() {
        assert_eq!(parse("PT0.5S"), Ok(Duration::milliseconds(500)));
        assert_eq!(parse("PT0,5S"), Ok(Duration::milliseconds(500)));
        assert_eq!(parse("PT1.5H"), Ok(Duration::minutes(90)));
    }

    /// ISO 8601 allows a fraction on the smallest component only. Accepting one
    /// anywhere would make `P1.5DT1H` mean something no other implementation
    /// reads the same way.
    #[test]
    fn a_fraction_is_only_allowed_on_the_last_component() {
        assert_eq!(parse("P1.5D"), Ok(Duration::hours(36)));
        assert_eq!(parse("PT1.5H"), Ok(Duration::minutes(90)));
        assert_eq!(parse("PT1H30.5M"), Ok(Duration::seconds(5430)));
        // …but a fraction on the *only* component is fine, week form included.
        assert_eq!(parse("P1.5W"), Ok(Duration::days(10) + Duration::hours(12)));
        for bad in ["P1.5DT1H", "PT1.5H30M", "PT0.5M1S", "P1.5WT1H"] {
            let err = parse(bad).unwrap_err();
            assert_eq!(
                err,
                Iso8601DurationError::Malformed {
                    reason: "a decimal fraction is only allowed on the last component",
                },
                "{bad:?}"
            );
        }
    }

    #[test]
    fn accepts_a_sign() {
        assert_eq!(parse("-P1D"), Ok(Duration::days(-1)));
        assert_eq!(parse("+P1D"), Ok(Duration::days(1)));
    }

    #[test]
    fn rejects_what_is_not_a_duration() {
        for bad in [
            "",
            "1D",
            "p1d",
            "P",
            "PT",
            "PD",
            "PTS",
            "P1",
            "PT1",
            "P-1D",
            "PXY",
            "P1DX",
            "2026-01-01",
            "PT1H1",
        ] {
            assert!(
                parse(bad).is_err(),
                "{bad:?} must not parse as a duration, got {:?}",
                parse(bad)
            );
        }
    }

    /// A lowercase designator is not ISO 8601, and accepting it silently would
    /// make `p1d` mean something no other implementation reads that way.
    #[test]
    fn is_case_sensitive() {
        assert!(parse("p1dt30h4s").is_err());
        assert!(parse("P1dT30h4s").is_err());
    }

    #[test]
    fn reports_out_of_range_rather_than_saturating() {
        assert_eq!(
            parse("P999999999999999999999D"),
            Err(Iso8601DurationError::OutOfRange)
        );
    }

    /// Every string this accepts must round-trip through `Duration` without
    /// panicking, which is the property the fuzz target relies on.
    #[test]
    fn never_panics_on_arbitrary_input() {
        for s in [
            "P", "PT", "P1DT", "PW", "P,S", "P..1D", "PTT1S", "P1D1", "-", "+P",
        ] {
            let _ = parse(s);
        }
    }
}
