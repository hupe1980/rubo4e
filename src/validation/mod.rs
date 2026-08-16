//! Cross-field business-rule validators for BO4E types, plus the [`Validated`](crate::validation::Validated) wrapper.
//!
//! ## `Validated<T>`
//!
//! [`Validated<T>`](crate::validation::Validated) is a zero-cost newtype wrapper that can only be
//! constructed by running the garde validation rules on `T`.  It implements
//! `Deref<Target = T>` and `AsRef<T>` for transparent field access, and
//! [`into_inner`](crate::validation::Validated::into_inner) to unwrap.
//!
//! A blanket `impl From<Validated<T>> for T` is deliberately absent: `T` is a type
//! parameter, so such an impl is uncovered and rejected by the orphan rule.
//! `into_inner()` is the unwrapping path.
//!
//! Requires only the `validate` feature (not `versioned`).
//!
//! ```
//! # #[cfg(feature = "versioned")] {
//! use rubo4e::validation::Validated;
//! use rubo4e::current::{Marktlokation, Adresse};
//!
//! // A Marktlokation must carry exactly one of the three address fields.
//! let malo = Marktlokation {
//!     lokationsadresse: Some(Adresse { ort: Some("Bremen".into()), ..Default::default() }),
//!     ..Default::default()
//! };
//!
//! let validated = Validated::new(malo).expect("exactly one address field is set");
//! assert!(validated.lokationsadresse.is_some());  // Deref to &Marktlokation
//! let inner: Marktlokation = validated.into_inner();
//!
//! // A Marktlokation with no address at all is rejected.
//! assert!(Validated::new(Marktlokation::default()).is_err());
//! # }
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
/// ```
/// # #[cfg(feature = "versioned")] {
/// use rubo4e::validation::Validated;
/// use rubo4e::current::Marktlokation;
///
/// // No address field set — violates the "exactly one" rule.
/// match Validated::new(Marktlokation::default()) {
///     Ok(v)  => panic!("unexpectedly valid: {:?}", v.marktlokations_id),
///     Err(r) => assert!(r.iter().count() > 0),
/// }
/// # }
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

/// Stamps out a validation sub-module for a given schema version (e.g. `v202607`).
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
            // Without `time` the body compiles away and `v` goes unread.
            #[cfg_attr(not(feature = "time"), allow(unused_variables))]
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
            // Without `time` the body compiles away and `v` goes unread.
            #[cfg_attr(not(feature = "time"), allow(unused_variables))]
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
            /// 3. When `gesamtbrutto` and `zu_zahlen` are both present:
            ///    `gesamtbrutto - rabatt_netto - sum(vorauszahlungen) == zu_zahlen`
            ///
            /// The arithmetic checks are gated on the `decimal` feature; without it
            /// `Betrag.wert` is `Option<String>` and numeric comparison is unsafe.
            // Without `decimal` the body compiles away and `v` goes unread.
            #[cfg_attr(not(feature = "decimal"), allow(unused_variables))]
            pub fn validate_rechnung_arithmetic(v: &Rechnung, _: &()) -> Result<(), garde::Error> {
                #[cfg(feature = "decimal")]
                {
                    use rust_decimal::Decimal;

                    let wert =
                        |b: &Option<Betrag>| -> Option<Decimal> { b.as_ref().and_then(|b| b.wert) };

                    // Currency-mismatch guard — all monetary fields must use the same Waehrungscode.
                    let waehrung = |b: &Option<Betrag>| b.as_ref().and_then(|b| b.waehrung);
                    let currencies = [
                        ("gesamtnetto", waehrung(&v.gesamtnetto)),
                        ("gesamtsteuer", waehrung(&v.gesamtsteuer)),
                        ("gesamtbrutto", waehrung(&v.gesamtbrutto)),
                        ("rabatt_netto", waehrung(&v.rabatt_netto)),
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

                    // zu_zahlen = gesamtbrutto - rabatt_netto - sum(vorauszahlungen)
                    if let (Some(b), Some(z)) = (wert(&v.gesamtbrutto), wert(&v.zu_zahlen)) {
                        let rabatt = wert(&v.rabatt_netto).unwrap_or(Decimal::ZERO);
                        let vorauszahlungen: Decimal = v
                            .vorauszahlungen
                            .as_deref()
                            .unwrap_or_default()
                            .iter()
                            .filter_map(|p| p.betrag.as_ref().and_then(|b| b.wert))
                            .fold(Decimal::ZERO, |acc, v| acc + v);
                        let expected = b - rabatt - vorauszahlungen;
                        if expected != z {
                            return Err(garde::Error::new(format!(
                                "gesamtbrutto ({b}) - rabatt_netto ({rabatt}) \
                                 - vorauszahlungen ({vorauszahlungen}) = {expected}, \
                                 but zu_zahlen is {z}"
                            )));
                        }
                    }
                } // end #[cfg(feature = "decimal")]
                Ok(())
            }

            /// `Zeitraum` must encode **at least one** of the three modes:
            ///
            /// 1. **Duration**: `dauer` is set (ISO 8601 duration string, e.g. `"P1DT"`)
            /// 2. **Date range**: `startdatum` or `enddatum` is set
            /// 3. **Time range**: `startuhrzeit` or `enduhrzeit` is set
            ///
            /// Combinations are *not* rejected — the BO4E schema permits them, and a
            /// stricter "exactly one" rule would reject payloads that real senders
            /// produce (e.g. a date range annotated with an explicit duration).
            /// An entirely empty `Zeitraum` carries no information and is rejected.
            ///
            /// When both `startdatum` and `enddatum` are present, `startdatum` must
            /// be strictly before `enddatum` (only checked when `time` is active).
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
            // Without `decimal` the body compiles away and `v` goes unread.
            #[cfg_attr(not(feature = "decimal"), allow(unused_variables))]
            pub fn validate_kostenposition_arithmetic(
                v: &Kostenposition,
                _: &(),
            ) -> Result<(), garde::Error> {
                #[cfg(feature = "decimal")]
                {
                    // einzelpreis and menge are now typed structs (Preis / Menge) whose
                    // `.wert` holds the numeric amount as a Decimal.  Extract it with
                    // `and_then` so we skip the arithmetic check when the sub-field is absent.
                    let betrag = v.betrag_kostenposition.as_ref().and_then(|b| b.wert);
                    let einzelpreis = v.einzelpreis.as_ref().and_then(|p| p.wert);
                    let menge = v.menge.as_ref().and_then(|m| m.wert);

                    if let (Some(ep), Some(m), Some(b)) = (einzelpreis, menge, betrag) {
                        let expected = (ep * m).round_dp(10);
                        let actual = b.round_dp(10);
                        if expected != actual {
                            return Err(garde::Error::new(format!(
                                "einzelpreis.wert ({ep}) * menge.wert ({m}) = {expected}, \
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
impl_validators!(v202607);

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
/// ```
/// # #[cfg(feature = "versioned")] {
/// use rubo4e::validation::{report_errors, Validated};
/// use rubo4e::current::Marktlokation;
///
/// let report = Validated::new(Marktlokation::default()).unwrap_err();
/// let failures = report_errors(&report);
/// assert!(!failures.is_empty());
/// for failure in &failures {
///     eprintln!("  {}: {}", failure.path, failure.message);
/// }
/// # }
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
