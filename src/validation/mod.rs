//! Cross-field business-rule validators for BO4E types, plus the [`Validated`](crate::validation::Validated) wrapper.
//!
//! ## `Validated<T>`
//!
//! [`Validated<T>`](crate::validation::Validated) is a zero-cost newtype wrapper that can only be constructed by
//! running the garde validation rules on `T`.  It implements `Deref<Target = T>` for
//! transparent field access and `Into<T>` / `From<Validated<T>>` for ergonomic
//! unwrapping.
//!
//! Requires only the `validate` feature (not `versioned`).
//!
//! ```rust,ignore
//! use rubo4e::validation::Validated;
//! use rubo4e::v202501::Marktlokation;
//!
//! let melo: Marktlokation = /* ... */;
//! let validated = Validated::new(melo)?;  // Err(garde::Report) if invariants violated
//! println!("{:?}", validated.lokations_id); // Deref to &Marktlokation
//! let inner: Marktlokation = validated.into_inner();
//! ```
//!
//! ## Cross-field validators
//!
//! Each function has the signature expected by [`garde`]:
//! ```text
//! fn validate_xxx(value: &T, context: &()) -> Result<(), garde::Error>
//! ```
//!
//! Validators are emitted via `#[garde(custom(...))]` on the generated structs.
//! Functions are only present when both `validate` and `versioned` features are active.
//!
//! ## Allocation behaviour
//!
//! Static error messages (e.g. "exactly one address field must be set") are stored
//! as `Cow::Borrowed(&'static str)` inside `garde::Error` — zero allocation on the
//! failure path.  Error messages that include runtime values (timestamps, decimal
//! amounts) use a single `format!` call on the failure path — unavoidable for
//! meaningful diagnostics.  The **happy path is always zero-allocation** for all
//! validators in this module.

/// A zero-cost wrapper around a value that has been checked against all garde validation
/// rules.
///
/// `Validated<T>` is the only way to get a value that is guaranteed to satisfy all
/// business-rule invariants declared on `T` via `#[derive(garde::Validate)]`.
///
/// # Construction
///
/// Use [`Validated::new`] to validate and wrap a value.  Unwrap with [`Validated::into_inner`]
/// or by dereferencing (`&*validated`).
///
/// # Examples
///
/// ```rust,ignore
/// # use rubo4e::validation::Validated;
/// # use rubo4e::v202501::Marktlokation;
/// let malo = Marktlokation::default();
/// match Validated::new(malo) {
///     Ok(v)  => println!("valid: {:?}", v.lokations_id),
///     Err(r) => eprintln!("invalid: {r}"),
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Validated<T>(T);

impl<T> Validated<T>
where
    T: garde::Validate,
    T::Context: Default,
{
    /// Validates `value` using its [`garde::Validate`] impl.
    ///
    /// Returns `Ok(Validated(value))` if all rules pass, or a [`garde::Report`]
    /// describing every failure.
    pub fn new(value: T) -> Result<Self, garde::Report> {
        value.validate()?;
        Ok(Self(value))
    }

    /// Consumes the wrapper and returns the inner (validated) value.
    #[inline]
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> std::ops::Deref for Validated<T> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> AsRef<T> for Validated<T> {
    #[inline]
    fn as_ref(&self) -> &T {
        &self.0
    }
}

#[cfg(feature = "serde")]
impl<T: serde::Serialize> serde::Serialize for Validated<T> {
    /// Serializes the inner (validated) value transparently.
    ///
    /// Consumers who receive a `Validated<T>` can serialize it without
    /// unwrapping, while retaining the type-level proof of validity.
    #[inline]
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(s)
    }
}

/// Stamps out a validation sub-module for a given schema version (e.g. `v202501`).
///
/// Each version gets its own `pub mod $ver { … }` so that future schema changes
/// (renamed fields, new rules) can diverge independently per version without
/// silently applying stale logic from an earlier release.
#[cfg(feature = "versioned")]
macro_rules! impl_validators {
    ($ver:ident) => {
        #[allow(missing_docs)]
        pub mod $ver {
            use crate::generated::$ver::*;

            /// Exactly one of `lokationsadresse`, `geoadresse`, or
            /// `katasterinformation` must be `Some`.
            pub fn validate_marktlokation(v: &Marktlokation, _: &()) -> Result<(), garde::Error> {
                let count = v.lokationsadresse.is_some() as usize
                    + v.geoadresse.is_some() as usize
                    + v.katasterinformation.is_some() as usize;
                if count == 1 {
                    Ok(())
                } else {
                    Err(garde::Error::new(
                        "exactly one address field must be set: \
                         lokationsadresse, geoadresse, or katasterinformation",
                    ))
                }
            }

            /// Exactly one of `messadresse`, `geoadresse`, or
            /// `katasterinformation` must be `Some`.
            pub fn validate_messlokation(v: &Messlokation, _: &()) -> Result<(), garde::Error> {
                let count = v.messadresse.is_some() as usize
                    + v.geoadresse.is_some() as usize
                    + v.katasterinformation.is_some() as usize;
                if count == 1 {
                    Ok(())
                } else {
                    Err(garde::Error::new(
                        "exactly one address field must be set: \
                         messadresse, geoadresse, or katasterinformation",
                    ))
                }
            }

            /// `vertragsbeginn` must be strictly before `vertragsende` when both
            /// are present.
            ///
            /// The ordering check is only performed when the `time` feature is active
            /// (fields are `time::OffsetDateTime`).  Without `time`, fields are `String`
            /// and lexicographic comparison is unsafe for partial ISO-8601 forms.
            pub fn validate_vertrag_dates(v: &Vertrag, _: &()) -> Result<(), garde::Error> {
                #[cfg(feature = "time")]
                if let (Some(start), Some(end)) = (v.vertragsbeginn, v.vertragsende) {
                    if start >= end {
                        return Err(garde::Error::new(format!(
                            "vertragsbeginn ({start}) must be before vertragsende ({end})"
                        )));
                    }
                }
                Ok(())
            }

            /// `bilanzierungsbeginn` must be ≤ `bilanzierungsende` when both are
            /// present.
            ///
            /// The ordering check is only performed when the `time` feature is active.
            pub fn validate_bilanzierung_dates(
                v: &Bilanzierung,
                _: &(),
            ) -> Result<(), garde::Error> {
                #[cfg(feature = "time")]
                if let (Some(start), Some(end)) = (v.bilanzierungsbeginn, v.bilanzierungsende) {
                    if start > end {
                        return Err(garde::Error::new(format!(
                            "bilanzierungsbeginn ({start}) must be ≤ bilanzierungsende ({end})"
                        )));
                    }
                }
                Ok(())
            }

            /// Invoice arithmetic checks:
            ///
            /// 1. If exactly two of `gesamtnetto`, `gesamtsteuer`, `gesamtbrutto` are
            ///    `Some`, all three must be present (partial amounts are not checkable).
            /// 2. When all three totals are present:
            ///    `gesamtnetto + gesamtsteuer == gesamtbrutto`
            /// 3. `gesamtbrutto - vorausgezahlt - rabatt_brutto == zu_zahlen`
            ///
            /// The arithmetic checks are gated on the `decimal` feature; without it
            /// `Betrag.wert` is `Option<String>` and numeric comparison is unsafe.
            pub fn validate_rechnung_arithmetic(v: &Rechnung, _: &()) -> Result<(), garde::Error> {
                #[cfg(feature = "decimal")]
                {
                    use rust_decimal::Decimal;

                    let wert =
                        |b: &Option<Betrag>| -> Option<Decimal> { b.as_ref().and_then(|b| b.wert) };

                    // I-8 (M-09): Currency-mismatch guard.
                    let waehrung = |b: &Option<Betrag>| b.as_ref().and_then(|b| b.waehrung);
                    let currencies = [
                        ("gesamtnetto", waehrung(&v.gesamtnetto)),
                        ("gesamtsteuer", waehrung(&v.gesamtsteuer)),
                        ("gesamtbrutto", waehrung(&v.gesamtbrutto)),
                        ("vorausgezahlt", waehrung(&v.vorausgezahlt)),
                        ("rabatt_brutto", waehrung(&v.rabatt_brutto)),
                        ("zu_zahlen", waehrung(&v.zu_zahlen)),
                    ];
                    let mut first_currency = None;
                    let mut first_field = "";
                    for (field, currency) in currencies {
                        if let Some(c) = currency {
                            match first_currency {
                                None => {
                                    first_currency = Some(c);
                                    first_field = field;
                                }
                                Some(fc) if fc != c => {
                                    return Err(garde::Error::new(format!(
                                        "currency mismatch: {first_field} uses {fc:?} \
                                         but {field} uses {c:?} — all Betrag fields in a \
                                         Rechnung must use the same Waehrungscode"
                                    )));
                                }
                                _ => {}
                            }
                        }
                    }

                    let netto = wert(&v.gesamtnetto);
                    let steuer = wert(&v.gesamtsteuer);
                    let brutto = wert(&v.gesamtbrutto);

                    let present_count = netto.is_some() as usize
                        + steuer.is_some() as usize
                        + brutto.is_some() as usize;
                    if present_count == 2 {
                        return Err(garde::Error::new(
                            "if any two invoice totals (gesamtnetto, gesamtsteuer, \
                             gesamtbrutto) are set, all three must be present",
                        ));
                    }

                    if let (Some(n), Some(s), Some(b)) = (netto, steuer, brutto) {
                        if n + s != b {
                            return Err(garde::Error::new(format!(
                                "gesamtnetto ({n}) + gesamtsteuer ({s}) must equal \
                                 gesamtbrutto ({b})"
                            )));
                        }
                    }

                    if let (Some(b), Some(z)) = (wert(&v.gesamtbrutto), wert(&v.zu_zahlen)) {
                        let expected = b
                            - wert(&v.vorausgezahlt).unwrap_or(Decimal::ZERO)
                            - wert(&v.rabatt_brutto).unwrap_or(Decimal::ZERO);
                        if expected != z {
                            return Err(garde::Error::new(format!(
                                "gesamtbrutto - vorausgezahlt - rabatt_brutto \
                                 ({expected}) must equal zu_zahlen ({z})"
                            )));
                        }
                    }
                } // end #[cfg(feature = "decimal")]
                Ok(())
            }

            /// `Zeitraum` must encode exactly one of the three valid modes:
            ///
            /// 1. **Duration**: `dauer` is set (ISO 8601 duration string, e.g. `"P1DT"`)
            /// 2. **Date range**: at least `startdatum` or `enddatum` is set
            /// 3. **Time range**: at least `startuhrzeit` or `enduhrzeit` is set
            ///
            /// When both `startdatum` and `enddatum` are present, `startdatum` must
            /// be strictly before `enddatum` (only checked when `time` feature is active).
            pub fn validate_zeitraum(v: &Zeitraum, _: &()) -> Result<(), garde::Error> {
                let has_duration = v.dauer.is_some();
                let has_date = v.startdatum.is_some() || v.enddatum.is_some();
                let has_time = v.startuhrzeit.is_some() || v.enduhrzeit.is_some();

                if !has_duration && !has_date && !has_time {
                    return Err(garde::Error::new(
                        "Zeitraum must have at least one of: dauer, startdatum/enddatum, \
                         or startuhrzeit/enduhrzeit",
                    ));
                }

                // Date-ordering invariant: only enforced when time feature provides
                // native OffsetDateTime comparison semantics.
                #[cfg(feature = "time")]
                if let (Some(start), Some(end)) = (v.startdatum, v.enddatum) {
                    if start >= end {
                        return Err(garde::Error::new(format!(
                            "startdatum ({start}) must be strictly before enddatum ({end})"
                        )));
                    }
                }

                Ok(())
            }

            /// Kostenposition arithmetic: `einzelpreis * menge == betrag_kostenposition.wert`
            /// when all three values are present.
            ///
            /// Gated on the `decimal` feature; without it the fields are `Option<String>`
            /// and numeric arithmetic is not available.
            pub fn validate_kostenposition_arithmetic(
                v: &Kostenposition,
                _: &(),
            ) -> Result<(), garde::Error> {
                #[cfg(feature = "decimal")]
                {
                    let betrag = v.betrag_kostenposition.as_ref().and_then(|b| b.wert);
                    let einzelpreis = v.einzelpreis;
                    let menge = v.menge;

                    if let (Some(ep), Some(m), Some(b)) = (einzelpreis, menge, betrag) {
                        let expected = (ep * m).round_dp(10);
                        let actual = b.round_dp(10);
                        if expected != actual {
                            return Err(garde::Error::new(format!(
                                "einzelpreis ({ep}) * menge ({m}) = {expected}, \
                                 but betrag_kostenposition.wert is {actual}"
                            )));
                        }
                    }
                } // end #[cfg(feature = "decimal")]
                Ok(())
            }
        }
    };
}

#[cfg(feature = "versioned")]
impl_validators!(v202501);

/// A single structured validation failure, extracted from a [`garde::Report`].
///
/// Use [`report_errors`] to convert a `garde::Report` into an iterator of these.
#[cfg(feature = "validate")]
#[cfg_attr(docsrs, doc(cfg(feature = "validate")))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationFailure {
    /// Dot-separated field path, e.g. `"betrag.wert"` or `"positionen[2].name"`.
    pub path: String,
    /// Human-readable error message for this field.
    pub message: String,
}

/// Converts a [`garde::Report`] into an iterator of structured [`ValidationFailure`]s.
///
/// `garde::Report` only implements `Display` (one big string), making it hard to
/// handle individual failures programmatically.  This function flattens the report
/// into one `ValidationFailure` per field error so callers can:
/// - render structured API error responses
/// - log individual field names with key-value pairs
/// - build test assertions per field
///
/// # Example
/// ```rust,ignore
/// use rubo4e::validation::{Validated, report_errors};
/// use rubo4e::v202501::Marktlokation;
///
/// let malo = Marktlokation::default();
/// if let Err(report) = Validated::new(malo) {
///     for failure in report_errors(&report) {
///         eprintln!("  {}: {}", failure.path, failure.message);
///     }
/// }
/// ```
#[cfg(feature = "validate")]
#[cfg_attr(docsrs, doc(cfg(feature = "validate")))]
pub fn report_errors(report: &garde::Report) -> Vec<ValidationFailure> {
    report
        .iter()
        .map(|(path, error)| ValidationFailure {
            path: path.to_string(),
            message: error.to_string(),
        })
        .collect()
}
