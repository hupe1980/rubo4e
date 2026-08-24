//! Time-of-day parsing for BO4E's `format: "time"` fields.
//!
//! `Zeitraum.startuhrzeit`, `.enduhrzeit`, and `Umschaltzeit.umschaltzeit` are
//! annotated `"format": "time"` with the example `"18:00:00+01:00"` — a time of
//! day **with a UTC offset**. No `time` type holds both (`Time` carries no zone,
//! `OffsetDateTime` demands a date), so the fields stay `String` rather than
//! dropping the offset, and this reads them.
//!
//! The offset is load-bearing: a Zählzeit window or a Doppeltarif switch written
//! `18:00:00+01:00` is a different wall-clock moment in summer than in winter.
//!
//! ```
//! # #[cfg(feature = "time")] {
//! use rubo4e::offset_time::parse;
//! use time::macros::{offset, time};
//!
//! let (t, utc_offset) = parse("18:00:00+01:00").unwrap();
//! assert_eq!(t, time!(18:00:00));
//! assert_eq!(utc_offset, Some(offset!(+1)));
//!
//! // The offset is optional, and `Z` means UTC.
//! assert_eq!(parse("06:30:00Z").unwrap(), (time!(06:30:00), Some(offset!(UTC))));
//! assert_eq!(parse("06:30").unwrap(), (time!(06:30:00), None));
//! # }
//! ```

use thiserror::Error;
use time::{Time, UtcOffset};

/// Why a `format: "time"` value could not be parsed.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum OffsetTimeError {
    /// The time-of-day part is not `HH:MM[:SS[.fraction]]`.
    #[error("not a valid time of day: {value:?}")]
    InvalidTime {
        /// The part that failed to parse.
        value: String,
    },
    /// The UTC offset is not `Z`, `±HH`, `±HH:MM`, or `±HH:MM:SS`.
    #[error("not a valid UTC offset: {value:?}")]
    InvalidOffset {
        /// The part that failed to parse.
        value: String,
    },
}

/// Parses a `format: "time"` value into a time of day and its UTC offset.
///
/// Accepts `HH:MM`, `HH:MM:SS`, and `HH:MM:SS.fraction`, each optionally
/// followed by `Z` or `±HH[:MM[:SS]]`.
///
/// The offset is `Option`, not defaulted to UTC: BO4E does not require one, and
/// "local time, zone not stated" is a different claim from "UTC".
///
/// # Errors
///
/// [`OffsetTimeError`] naming whichever half did not parse.
pub fn parse(s: &str) -> Result<(Time, Option<UtcOffset>), OffsetTimeError> {
    let s = s.trim();

    // The offset starts at the first `Z`, or at a `+`/`-` after the time — never
    // at index 0, and never at the `:` separators.
    let split = s
        .char_indices()
        .skip(1)
        .find(|&(_, c)| c == 'Z' || c == 'z' || c == '+' || c == '-')
        .map(|(i, _)| i);

    let (time_part, offset_part) = match split {
        Some(i) => (&s[..i], Some(&s[i..])),
        None => (s, None),
    };

    let time = parse_time_of_day(time_part).ok_or_else(|| OffsetTimeError::InvalidTime {
        value: time_part.to_owned(),
    })?;

    let offset = match offset_part {
        None => None,
        Some(raw) => Some(
            parse_offset(raw).ok_or_else(|| OffsetTimeError::InvalidOffset {
                value: raw.to_owned(),
            })?,
        ),
    };

    Ok((time, offset))
}

/// `HH:MM`, `HH:MM:SS`, or `HH:MM:SS.fff…`.
fn parse_time_of_day(s: &str) -> Option<Time> {
    let mut parts = s.split(':');
    let hour: u8 = two_digits(parts.next()?)?;
    let minute: u8 = two_digits(parts.next()?)?;

    let (second, nanos) = match parts.next() {
        None => (0, 0),
        Some(sec) => {
            let (whole, frac) = match sec.split_once(['.', ',']) {
                Some((w, f)) => (w, Some(f)),
                None => (sec, None),
            };
            let second = two_digits(whole)?;
            let nanos = match frac {
                None => 0,
                Some(f) => {
                    if f.is_empty() || !f.bytes().all(|b| b.is_ascii_digit()) {
                        return None;
                    }
                    // Left-align to nanosecond precision, truncating past 9 digits.
                    let digits: String = f.chars().take(9).collect();
                    let scale = 10u32.pow(9 - u32::try_from(digits.len()).ok()?);
                    digits.parse::<u32>().ok()? * scale
                }
            };
            (second, nanos)
        }
    };
    if parts.next().is_some() {
        return None;
    }

    // `24:00:00` is a legal ISO 8601 end-of-day but not a `time::Time`; BO4E's
    // `enduhrzeit` is exclusive, so a window ending at midnight is written
    // `00:00:00` on the following day rather than `24:00`.
    Time::from_hms_nano(hour, minute, second, nanos).ok()
}

/// `Z`, `±HH`, `±HH:MM`, or `±HH:MM:SS`.
fn parse_offset(s: &str) -> Option<UtcOffset> {
    if s.eq_ignore_ascii_case("Z") {
        return Some(UtcOffset::UTC);
    }
    let (sign, rest) = match s.as_bytes().first()? {
        b'+' => (1i8, &s[1..]),
        b'-' => (-1i8, &s[1..]),
        _ => return None,
    };

    let mut parts = rest.split(':');
    let hours = two_digits(parts.next()?)?;
    let minutes = parts.next().map_or(Some(0), two_digits)?;
    let seconds = parts.next().map_or(Some(0), two_digits)?;
    if parts.next().is_some() {
        return None;
    }

    UtcOffset::from_hms(
        sign * i8::try_from(hours).ok()?,
        sign * i8::try_from(minutes).ok()?,
        sign * i8::try_from(seconds).ok()?,
    )
    .ok()
}

/// A one- or two-digit non-negative number. Rejects `+1`, `1_0`, and the empty
/// string, all of which `str::parse` would otherwise be lenient about.
fn two_digits(s: &str) -> Option<u8> {
    if s.is_empty() || s.len() > 2 || !s.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    s.parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::macros::{offset, time};

    #[test]
    fn parses_the_bo4e_examples() {
        // The two given on `Zeitraum.startuhrzeit` / `.enduhrzeit`.
        assert_eq!(
            parse("18:00:00+01:00"),
            Ok((time!(18:00:00), Some(offset!(+1))))
        );
        assert_eq!(
            parse("19:00:00+01:00"),
            Ok((time!(19:00:00), Some(offset!(+1))))
        );
    }

    #[test]
    fn the_offset_is_optional_and_z_is_utc() {
        assert_eq!(parse("06:30"), Ok((time!(06:30:00), None)));
        assert_eq!(parse("06:30:15"), Ok((time!(06:30:15), None)));
        assert_eq!(
            parse("06:30:00Z"),
            Ok((time!(06:30:00), Some(offset!(UTC))))
        );
        assert_eq!(
            parse("06:30:00z"),
            Ok((time!(06:30:00), Some(offset!(UTC))))
        );
    }

    /// A missing offset must not be reported as UTC — "local time, zone not
    /// stated" and "UTC" are different claims, and conflating them moves a
    /// tariff boundary by up to two hours in Germany.
    #[test]
    fn a_missing_offset_is_not_utc() {
        assert_eq!(parse("18:00:00").unwrap().1, None);
        assert_ne!(parse("18:00:00").unwrap().1, Some(offset!(UTC)));
    }

    #[test]
    fn parses_every_offset_shape() {
        assert_eq!(parse("00:00:00+02").unwrap().1, Some(offset!(+2)));
        assert_eq!(parse("00:00:00-05:30").unwrap().1, Some(offset!(-5:30)));
        assert_eq!(parse("00:00:00+05:45").unwrap().1, Some(offset!(+5:45)));
        // A negative offset applies its sign to every component.
        let (_, o) = parse("00:00:00-01:30").unwrap();
        assert_eq!(o.unwrap().whole_minutes(), -90);
    }

    #[test]
    fn parses_fractional_seconds() {
        assert_eq!(parse("12:00:00.5").unwrap().0, time!(12:00:00.500));
        assert_eq!(parse("12:00:00,5").unwrap().0, time!(12:00:00.500));
        assert_eq!(
            parse("12:00:00.123456789").unwrap().0,
            time!(12:00:00.123456789)
        );
        // Past nanosecond precision the extra digits are truncated, not rejected.
        assert_eq!(
            parse("12:00:00.1234567891").unwrap().0,
            time!(12:00:00.123456789)
        );
    }

    #[test]
    fn rejects_what_is_not_a_time() {
        for bad in [
            "",
            "18",
            "18:",
            ":00",
            "18:60",
            "25:00",
            "18:00:60",
            "18:00:00:00",
            "6:3:0:1",
            "abc",
            "18:00:00+",
            "18:00:00+99:00",
            "18:00:00±01",
            "-01:00",
            "18:00:00.",
            "18:00:00.abc",
            "1_8:00",
        ] {
            assert!(
                parse(bad).is_err(),
                "{bad:?} must not parse, got {:?}",
                parse(bad)
            );
        }
    }

    /// `24:00` is a legal ISO 8601 end-of-day, but not a wall-clock time, and
    /// BO4E's exclusive `enduhrzeit` never needs it.
    #[test]
    fn rejects_end_of_day_24_00() {
        assert!(parse("24:00:00").is_err());
    }

    #[test]
    fn never_panics_on_arbitrary_input() {
        for s in ["+", "-", "Z", "::", "18:00:00+", "ä:öü", "18:00:00+ä"] {
            let _ = parse(s);
        }
    }
}
