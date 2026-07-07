//! Serde modules for `time::Date` fields in generated BO4E structs.
//!
//! These are referenced from generated code via
//! `#[serde(with = "crate::time_serde::date_serde")]` and
//! `#[serde(with = "crate::time_serde::opt_date_serde")]`.

/// Serde module for serializing/deserializing a required `time::Date` as an
/// ISO 8601 date-only string (`YYYY-MM-DD`).
///
/// Used with `#[serde(with = "crate::time_serde::date_serde")]`
/// on `time::Date` fields in generated structs when the `time` feature is active.
pub mod date_serde {
    use time::{format_description::BorrowedFormatItem, macros::format_description, Date};

    const DATE_FMT: &[BorrowedFormatItem<'_>] = format_description!("[year]-[month]-[day]");

    /// Serialize a `time::Date` as `"YYYY-MM-DD"`.
    pub fn serialize<S: serde::Serializer>(date: &Date, s: S) -> Result<S::Ok, S::Error> {
        let text = date.format(DATE_FMT).map_err(serde::ser::Error::custom)?;
        s.serialize_str(&text)
    }

    /// Deserialize a `"YYYY-MM-DD"` string into `time::Date`.
    ///
    /// Uses a zero-allocation visitor: the input is borrowed as `&str`
    /// without allocating an intermediate `String`.
    pub fn deserialize<'de, D: serde::Deserializer<'de>>(d: D) -> Result<Date, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Date;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("an ISO 8601 date string (YYYY-MM-DD)")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Date, E> {
                Date::parse(v, DATE_FMT).map_err(E::custom)
            }
        }
        d.deserialize_str(Visitor)
    }
}

/// Serde module for serializing/deserializing an `Option<time::Date>` as a
/// nullable ISO 8601 date-only string (`YYYY-MM-DD`).
///
/// Used with `#[serde(with = "crate::time_serde::opt_date_serde")]`
/// on `Option<time::Date>` fields in generated structs when the `time` feature is active.
pub mod opt_date_serde {
    use time::{format_description::BorrowedFormatItem, macros::format_description, Date};

    const DATE_FMT: &[BorrowedFormatItem<'_>] = format_description!("[year]-[month]-[day]");

    /// Serialize an `Option<time::Date>` as `null` or `"YYYY-MM-DD"`.
    pub fn serialize<S: serde::Serializer>(val: &Option<Date>, s: S) -> Result<S::Ok, S::Error> {
        match val {
            None => s.serialize_none(),
            Some(d) => {
                let text = d.format(DATE_FMT).map_err(serde::ser::Error::custom)?;
                s.serialize_some(&text)
            }
        }
    }

    /// Deserialize a nullable `"YYYY-MM-DD"` string into `Option<time::Date>`.
    ///
    /// Uses a zero-allocation visitor: `visit_str` borrows from the input
    /// without allocating an intermediate `String`.
    pub fn deserialize<'de, D: serde::Deserializer<'de>>(
        d: D,
    ) -> Result<Option<Date>, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Option<Date>;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("an ISO 8601 date string (YYYY-MM-DD) or null")
            }
            fn visit_none<E: serde::de::Error>(self) -> Result<Option<Date>, E> {
                Ok(None)
            }
            fn visit_unit<E: serde::de::Error>(self) -> Result<Option<Date>, E> {
                Ok(None)
            }
            fn visit_some<D2: serde::Deserializer<'de>>(
                self,
                d: D2,
            ) -> Result<Option<Date>, D2::Error> {
                super::date_serde::deserialize(d).map(Some)
            }
        }
        d.deserialize_option(Visitor)
    }
}
