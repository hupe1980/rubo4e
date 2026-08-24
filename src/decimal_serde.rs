//! Decimal deserialization for BO4E's two spellings of a number.
//!
//! The standard does not settle which to use, so both are read here:
//!
//! | Producer | `Betrag.wert` on the wire |
//! |---|---|
//! | BO4E-python | `"wert": "119.00"` — pydantic renders `Decimal` as a string |
//! | go-bo4e | `"wert": 119.00` — Go marshals `float64` as a number |
//!
//! # Only the string spelling is exact
//!
//! A JSON string carries the decimal's digits and they are parsed as digits:
//! the scale is kept, and 28 significant digits survive.
//!
//! A JSON number has already been through `f64` before any Rust deserializer
//! sees it — serde's data model has no arbitrary-precision number — so two
//! things are gone before this module runs, and nothing here can recover them:
//!
//! - **Scale**: `119.00` arrives as `119`.
//! - **Precision past ~15 significant digits**: `12345678901234567890.12`
//!   arrives as `12345678901234567000`.
//!
//! No amount in the German energy market reaches 15 significant digits, so this
//! is a fidelity question rather than a correctness one. Every decimal read from
//! a number is counted anyway — see [`decimal_from_json_number_count`] — because
//! a non-zero count on a link you believed was string-encoded means a producer
//! changed or a proxy re-encoded the payload.
//!
//! Serialization always writes a string, matching BO4E-python. Compare amounts
//! rather than their spellings: `119` and `119.00` are the same `Decimal`.
//!
//! # Without the `decimal` feature
//!
//! The field is a `String` holding the lexical form, so `"119.00"` does keep its
//! scale. Serde's own `String` impl accepts only the string spelling, which is
//! why this module covers that case too.
//!
//! Generated code refers to it as
//! `#[serde(deserialize_with = "crate::decimal_serde::deserialize_opt")]`; the
//! return type flips with the feature, so the attribute is one string in both
//! builds.
//!
//! [`decimal_from_json_number_count`]: crate::decimal_serde::decimal_from_json_number_count

use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};

static DECIMAL_FROM_JSON_NUMBER: AtomicU64 = AtomicU64::new(0);

/// Returns how many decimal fields this process has read from a JSON **number**
/// rather than a JSON string.
///
/// Every one of those went through `f64`, losing its scale and any precision
/// past ~15 significant digits — see the [module docs](crate::decimal_serde). The counter is
/// process-wide, monotonically non-decreasing, and uses `Ordering::Relaxed`:
/// suitable for an observability endpoint, not for synchronization.
///
/// A steady zero means every producer on the link spells decimals as strings,
/// the way BO4E-python does. A rising count identifies a go-bo4e-style producer
/// (or a proxy that re-encoded the payload) whose amounts reach you at `f64`
/// fidelity.
#[must_use]
pub fn decimal_from_json_number_count() -> u64 {
    DECIMAL_FROM_JSON_NUMBER.load(Ordering::Relaxed)
}

/// Records one decimal read from a JSON number.
#[inline]
fn note_json_number(#[allow(unused_variables)] rendered: &str) {
    DECIMAL_FROM_JSON_NUMBER.fetch_add(1, Ordering::Relaxed);

    #[cfg(feature = "metrics")]
    metrics::counter!("bo4e_decimal_from_json_number_total").increment(1);

    #[cfg(feature = "tracing")]
    tracing::debug!(
        value = rendered,
        "decimal read from a JSON number; scale and precision beyond f64 are already lost"
    );
}

// ─── With `decimal`: rust_decimal::Decimal ───────────────────────────────────

#[cfg(feature = "decimal")]
mod imp {
    use super::{fmt, note_json_number};
    use rust_decimal::Decimal;

    pub(super) type Value = Decimal;

    pub(super) struct DecimalVisitor;

    impl serde::de::Visitor<'_> for DecimalVisitor {
        type Value = Decimal;

        fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("a decimal as a JSON string or number")
        }

        /// The exact path: the digits are parsed as digits, so scale is kept and
        /// 28 significant digits survive.
        fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Decimal, E> {
            v.parse::<Decimal>().map_err(E::custom)
        }

        fn visit_u64<E: serde::de::Error>(self, v: u64) -> Result<Decimal, E> {
            Ok(Decimal::from(v))
        }

        fn visit_i64<E: serde::de::Error>(self, v: i64) -> Result<Decimal, E> {
            Ok(Decimal::from(v))
        }

        // Not the `From` impls clippy points at: `Decimal::from(u128::MAX)`
        // **panics** (`rust_decimal` unwraps an internal `Option`), and a
        // deserializer must not panic on a value an untrusted payload can carry.
        // `serde_json` never reaches these — it hands JSON integers to
        // `visit_u64` / `visit_i64` — but another `Deserializer` may.
        #[allow(clippy::unnecessary_fallible_conversions)]
        fn visit_u128<E: serde::de::Error>(self, v: u128) -> Result<Decimal, E> {
            Decimal::try_from(v).map_err(E::custom)
        }

        #[allow(clippy::unnecessary_fallible_conversions)]
        fn visit_i128<E: serde::de::Error>(self, v: i128) -> Result<Decimal, E> {
            Decimal::try_from(v).map_err(E::custom)
        }

        /// The lossy path. Convert through the shortest `f64` rendering rather
        /// than through the raw binary value: `0.1_f64` is
        /// `0.1000000000000000055511151231` in full, and `"0.1"` is what the
        /// sender meant and what every other implementation shows.
        ///
        /// An integer-valued float never reaches here — `serde_json` hands
        /// those to `visit_u64` / `visit_i64` — so this really is the
        /// fractional case, where the scale was lost.
        fn visit_f64<E: serde::de::Error>(self, v: f64) -> Result<Decimal, E> {
            let rendered = v.to_string();
            note_json_number(&rendered);
            rendered.parse::<Decimal>().map_err(E::custom)
        }
    }
}

// ─── Without `decimal`: the lexical form, as a String ────────────────────────

#[cfg(not(feature = "decimal"))]
mod imp {
    use super::{fmt, note_json_number};

    pub(super) type Value = String;

    pub(super) struct DecimalVisitor;

    impl serde::de::Visitor<'_> for DecimalVisitor {
        type Value = String;

        fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("a decimal as a JSON string or number")
        }

        /// Verbatim: this is the one path that keeps a producer's exact
        /// spelling, trailing zeros included.
        fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<String, E> {
            Ok(v.to_owned())
        }

        fn visit_string<E: serde::de::Error>(self, v: String) -> Result<String, E> {
            Ok(v)
        }

        fn visit_u64<E: serde::de::Error>(self, v: u64) -> Result<String, E> {
            Ok(v.to_string())
        }

        fn visit_i64<E: serde::de::Error>(self, v: i64) -> Result<String, E> {
            Ok(v.to_string())
        }

        fn visit_u128<E: serde::de::Error>(self, v: u128) -> Result<String, E> {
            Ok(v.to_string())
        }

        fn visit_i128<E: serde::de::Error>(self, v: i128) -> Result<String, E> {
            Ok(v.to_string())
        }

        /// A JSON float is already through `f64` by the time a visitor sees it,
        /// so this cannot recover digits the parser dropped.
        fn visit_f64<E: serde::de::Error>(self, v: f64) -> Result<String, E> {
            let rendered = v.to_string();
            note_json_number(&rendered);
            Ok(rendered)
        }
    }
}

use imp::{DecimalVisitor, Value};

/// Deserializes a required decimal field from a JSON string or number.
///
/// Yields `rust_decimal::Decimal` with the `decimal` feature and `String`
/// without it. See the [module docs](self) for which spellings are exact.
///
/// # Errors
/// Returns an error for any JSON type other than a string or a number, and —
/// with `decimal` — for a value outside `Decimal`'s range.
pub fn deserialize<'de, D: serde::Deserializer<'de>>(d: D) -> Result<Value, D::Error> {
    d.deserialize_any(DecimalVisitor)
}

struct OptDecimalVisitor;

impl<'de> serde::de::Visitor<'de> for OptDecimalVisitor {
    type Value = Option<Value>;

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("a decimal as a JSON string or number, or null")
    }

    fn visit_none<E: serde::de::Error>(self) -> Result<Option<Value>, E> {
        Ok(None)
    }
    fn visit_unit<E: serde::de::Error>(self) -> Result<Option<Value>, E> {
        Ok(None)
    }
    fn visit_some<D: serde::Deserializer<'de>>(self, d: D) -> Result<Option<Value>, D::Error> {
        d.deserialize_any(DecimalVisitor).map(Some)
    }
}

/// Deserializes an optional decimal field from a JSON string, number, or `null`.
///
/// # Errors
/// Returns an error for any other JSON type.
pub fn deserialize_opt<'de, D: serde::Deserializer<'de>>(d: D) -> Result<Option<Value>, D::Error> {
    d.deserialize_option(OptDecimalVisitor)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(serde::Deserialize)]
    struct Holder {
        #[serde(default, deserialize_with = "deserialize_opt")]
        wert: Option<Value>,
    }

    fn read(json: &str) -> Option<Value> {
        serde_json::from_str::<Holder>(json).expect("valid").wert
    }

    #[test]
    fn accepts_both_wire_spellings_and_null() {
        assert!(read(r#"{"wert":"119.00"}"#).is_some());
        assert!(read(r#"{"wert":119.00}"#).is_some());
        assert!(read(r#"{"wert":119}"#).is_some());
        assert!(read(r#"{"wert":null}"#).is_none());
        assert!(read("{}").is_none());
    }

    #[test]
    fn rejects_a_non_numeric_json_type() {
        assert!(serde_json::from_str::<Holder>(r#"{"wert":[1]}"#).is_err());
        assert!(serde_json::from_str::<Holder>(r#"{"wert":true}"#).is_err());
    }

    /// The string spelling is the exact one: it keeps the scale a payment or an
    /// invoice line was written with.
    #[test]
    #[cfg(feature = "decimal")]
    fn a_json_string_keeps_its_scale() {
        assert_eq!(read(r#"{"wert":"119.00"}"#).unwrap().to_string(), "119.00");
        assert_eq!(read(r#"{"wert":"0.0725"}"#).unwrap().to_string(), "0.0725");
    }

    /// 28 significant digits survive the string path…
    #[test]
    #[cfg(feature = "decimal")]
    fn a_json_string_keeps_precision_f64_would_lose() {
        assert_eq!(
            read(r#"{"wert":"12345678901234567890.12"}"#)
                .unwrap()
                .to_string(),
            "12345678901234567890.12"
        );
    }

    /// …and the number path cannot: `serde_json` rounds to `f64` before this
    /// crate is called. Pinned so the limit stays a documented fact.
    #[test]
    #[cfg(feature = "decimal")]
    fn a_json_number_arrives_at_f64_fidelity() {
        let got = read(r#"{"wert":12345678901234567890.12}"#).unwrap();
        assert_eq!(got.to_string(), "12345678901234567000");
    }

    /// The counter is process-wide and monotonic, so this asserts only that a
    /// fractional JSON number moves it — never a delta, which another test
    /// running concurrently in this binary would race with.
    #[test]
    fn reading_a_decimal_from_a_json_number_is_counted() {
        assert!(read(r#"{"wert":0.5}"#).is_some());
        assert!(
            decimal_from_json_number_count() > 0,
            "a decimal read from a JSON number must be counted"
        );
    }

    /// `0.1` is not exactly representable in `f64`; going through the shortest
    /// rendering yields what the sender wrote instead of its binary expansion.
    #[test]
    #[cfg(feature = "decimal")]
    fn a_fractional_number_uses_the_shortest_rendering() {
        assert_eq!(read(r#"{"wert":0.1}"#).unwrap().to_string(), "0.1");
    }

    /// An integer arrives through `visit_u64` / `visit_i64`, which is exact —
    /// including values `f64` could not have carried.
    #[test]
    #[cfg(feature = "decimal")]
    fn an_integer_is_exact() {
        assert_eq!(read(r#"{"wert":119}"#).unwrap().to_string(), "119");
        assert_eq!(read(r#"{"wert":-42}"#).unwrap().to_string(), "-42");
        assert_eq!(
            read(r#"{"wert":9007199254740993}"#).unwrap().to_string(),
            "9007199254740993",
            "2^53 + 1 must survive: an integer must not go through the f64 path"
        );
    }

    /// Without `decimal` the lexical form is what is kept — including the scale
    /// the `Decimal` path cannot recover from a number.
    #[test]
    #[cfg(not(feature = "decimal"))]
    fn without_the_feature_the_lexical_form_is_kept() {
        assert_eq!(read(r#"{"wert":"119.00"}"#).unwrap(), "119.00");
        assert_eq!(read(r#"{"wert":119}"#).unwrap(), "119");
    }
}
