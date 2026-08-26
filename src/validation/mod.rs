//! Cross-field business-rule validators for BO4E types, plus the [`Validated`](crate::validation::Validated) wrapper.
//!
//! ## Validation is recursive
//!
//! Calling `.validate()` on a BO checks that BO's own cross-field rules **and
//! descends into every nested BO, COM, and identifier below it**. A failure is
//! reported at its path — `rechnungsperiode`,
//! `kostenbloecke[0].kostenpositionen[0]` — so a report names where the problem
//! is, not just that there is one.
//!
//! What is *not* checked is presence: BO4E declares almost every field optional,
//! so `garde` cannot enforce "required" for you. The rules below are the
//! invariants BO4E states in prose, and they only fire on values that are there.
//!
//! ## `Validated<T>`
//!
//! [`Validated<T>`](crate::validation::Validated) is a zero-cost newtype wrapper that can only be
//! constructed by running the garde validation rules on `T`.  It implements
//! `Deref<Target = T>` and `AsRef<T>` for transparent field access, and
//! [`into_inner`](crate::validation::Validated::into_inner) to unwrap.
//!
//! With `serde` it is also `Serialize` and `Deserialize`, and the `Deserialize`
//! impl **validates**: decoding a `Validated<T>` and getting a value back is the
//! proof, so a handler can take one as its request body and never have a
//! forgotten `.validate()` call.
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
//! use rubo4e::current::{Adresse, Geokoordinaten, Marktlokation};
//!
//! // A Marktlokation may carry at most one of the three Ortsangaben.
//! let malo = Marktlokation {
//!     lokationsadresse: Some(Adresse { ort: Some("Bremen".into()), ..Default::default() }),
//!     ..Default::default()
//! };
//!
//! let validated = Validated::new(malo).expect("one Ortsangabe is fine");
//! assert!(validated.lokationsadresse.is_some());  // Deref to &Marktlokation
//! let inner: Marktlokation = validated.into_inner();
//!
//! // None at all is fine too — a referenced location often carries only its ID.
//! assert!(Validated::new(Marktlokation::default()).is_ok());
//!
//! // Two is not: they would disagree about where the location is.
//! let conflicting = Marktlokation {
//!     lokationsadresse: Some(Adresse::default()),
//!     geoadresse: Some(Geokoordinaten::default()),
//!     ..Default::default()
//! };
//! assert!(Validated::new(conflicting).is_err());
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
//! Static error messages (e.g. "Zeitraum must have at least one of: …") are
//! stored as `Cow::Borrowed(&'static str)` inside `garde::Error` — zero
//! allocation on the failure path.  Messages that name the offending values
//! (which fields conflicted, which timestamps, which amounts) use a single
//! `format!` on the failure path — unavoidable for a diagnosis worth reading.
//! The **happy path is always zero-allocation** for every validator here.

/// A zero-cost wrapper around a value that has been checked against all garde validation
/// rules.
///
/// `Validated<T>` is the only way to get a value that is guaranteed to satisfy
/// every business-rule invariant declared on `T` — and, because validation
/// descends into nested BOs, COMs, and identifiers, on everything `T` contains.
///
/// It is a proof about the *rules*, not about completeness: BO4E declares almost
/// every field optional, so a `Validated<Rechnung>` may still be missing fields
/// your AHB requires. Enforce those at your ingest boundary.
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
/// use rubo4e::current::{Adresse, Geokoordinaten, Marktlokation};
///
/// // Two Ortsangaben — they would disagree about where the location is.
/// let conflicting = Marktlokation {
///     lokationsadresse: Some(Adresse::default()),
///     geoadresse: Some(Geokoordinaten::default()),
///     ..Default::default()
/// };
/// match Validated::new(conflicting) {
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

#[cfg(feature = "serde")]
impl<'de, T> serde::Deserialize<'de> for Validated<T>
where
    T: serde::Deserialize<'de> + garde::Validate,
    T::Context: Default,
{
    /// Deserializes `T`, then runs its rules — the same recursive check
    /// [`Validated::new`] runs. This is what makes the wrapper usable as a
    /// request body:
    ///
    /// ```
    /// # #[cfg(all(feature = "versioned", feature = "json"))] {
    /// use rubo4e::current::Marktlokation;
    /// use rubo4e::validation::Validated;
    ///
    /// // A Marktlokation may carry at most one of the three Ortsangaben.
    /// let ok = r#"{"marktlokationsId":"51238696781","lokationsadresse":{"ort":"Bremen"}}"#;
    /// let malo: Validated<Marktlokation> = serde_json::from_str(ok).unwrap();
    /// assert_eq!(malo.lokationsadresse.as_ref().unwrap().ort.as_deref(), Some("Bremen"));
    ///
    /// // …and one that carries two does not decode at all.
    /// let bad = r#"{"lokationsadresse":{"ort":"Bremen"},"geoadresse":{"breitengrad":"53.1"}}"#;
    /// let err = serde_json::from_str::<Validated<Marktlokation>>(bad).unwrap_err();
    /// assert!(err.to_string().contains("Ortsangabe"), "{err}");
    /// # }
    /// ```
    ///
    /// The failure arrives as the deserializer's error type, which renders the
    /// whole `garde::Report` into its message. Decode a plain `T` and call
    /// [`Validated::new`] where you need the structured report back.
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let value = T::deserialize(d)?;
        Self::new(value).map_err(serde::de::Error::custom)
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

            /// Reports the Ortsangabe fields that are set, when more than one is.
            ///
            /// `None` when the value conforms — nought or one of them present.
            fn conflicting_ortsangaben(present: [(&'static str, bool); 3]) -> Option<String> {
                let set: Vec<&str> = present
                    .iter()
                    .filter(|(_, is_set)| *is_set)
                    .map(|(name, _)| *name)
                    .collect();
                (set.len() > 1).then(|| set.join(", "))
            }

            /// **At most one** of `lokationsadresse`, `geoadresse`, or
            /// `katasterinformation` may be `Some`.
            ///
            /// BO4E states mutual exclusivity, not presence: *"Es darf immer nur
            /// eine Art der Ortsangabe vorhanden sein."* The schema declares no
            /// `required` array and no `oneOf`, and all three properties default
            /// to `null`.
            ///
            /// **No** Ortsangabe therefore conforms, and is common: BO4E has no
            /// reference type, so a location referenced from a `Rechnung`, a
            /// `Vertrag`, or an `Angebot` is a full `Marktlokation` carrying
            /// little more than its ID.
            ///
            /// Checked only when you call `.validate()`; a violating payload
            /// still deserializes.
            pub fn validate_marktlokation(v: &Marktlokation, _: &()) -> Result<(), garde::Error> {
                match conflicting_ortsangaben([
                    ("lokationsadresse", v.lokationsadresse.is_some()),
                    ("geoadresse", v.geoadresse.is_some()),
                    ("katasterinformation", v.katasterinformation.is_some()),
                ]) {
                    None => Ok(()),
                    Some(set) => Err(garde::Error::new(format!(
                        "at most one Ortsangabe may be set, but {set} are — BO4E allows \
                         either an Adresse, a Geokoordinate, or a Katasteradresse"
                    ))),
                }
            }

            /// **At most one** of `messadresse`, `geoadresse`, or
            /// `katasterinformation` may be `Some`.
            ///
            /// Same provenance as [`validate_marktlokation`]. The empty case is
            /// explicit here: `messadresse` is documented *"Nur angeben, wenn
            /// diese von der Adresse der Marktlokation abweicht"*, so a
            /// Messlokation matching its Marktlokation carries none by design.
            pub fn validate_messlokation(v: &Messlokation, _: &()) -> Result<(), garde::Error> {
                match conflicting_ortsangaben([
                    ("messadresse", v.messadresse.is_some()),
                    ("geoadresse", v.geoadresse.is_some()),
                    ("katasterinformation", v.katasterinformation.is_some()),
                ]) {
                    None => Ok(()),
                    Some(set) => Err(garde::Error::new(format!(
                        "at most one Ortsangabe may be set, but {set} are — BO4E allows \
                         either an Adresse, a Geokoordinate, or a Katasteradresse"
                    ))),
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

            /// Invoice consistency checks, each traceable to a sentence in the
            /// BO4E schema:
            ///
            /// 1. All monetary fields must agree on a currency. Two `Betrag`s in
            ///    one invoice denominated differently cannot be summed, so any
            ///    downstream total would be meaningless.
            /// 2. `gesamtbrutto` is *"Die Summe aus Netto- und Steuerbetrag"* —
            ///    so `gesamtnetto + gesamtsteuer == gesamtbrutto` when all three
            ///    are present.
            /// 3. `steuerbetraege` is *"eine Liste mit Steuerbeträgen … die Summe
            ///    dieser Beträge ergibt den Wert für gesamtsteuer"* — so the
            ///    line-level tax amounts must sum to `gesamtsteuer`.
            ///
            /// "If two totals are stated the third must be too" is a house rule
            /// BO4E does not state, so it lives in
            /// [`quality::rechnung_totals_are_complete`] instead.
            ///
            /// # Not checked: `zuZahlen`
            ///
            /// Its schema description reads *"(gesamtbrutto - vorausbezahlt -
            /// rabattBrutto)"*, but v202607 has no `rabattBrutto` — only
            /// `rabattNetto`, a **net** discount, which cannot be subtracted from
            /// a gross total. The equation is not reconstructible from the
            /// payload, so nothing is asserted about it.
            ///
            /// The arithmetic is gated on the `decimal` feature; without it
            /// `Betrag.wert` is `Option<String>` and numeric comparison is unsafe.
            // Without `decimal` the body compiles away and `v` goes unread.
            #[cfg_attr(not(feature = "decimal"), allow(unused_variables))]
            pub fn validate_rechnung_arithmetic(v: &Rechnung, _: &()) -> Result<(), garde::Error> {
                #[cfg(feature = "decimal")]
                {
                    use rust_decimal::Decimal;

                    let wert =
                        |b: &Option<Betrag>| -> Option<Decimal> { b.as_ref().and_then(|b| b.wert) };

                    // 1. Currency-mismatch guard.
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

                    // 2. gesamtbrutto = gesamtnetto + gesamtsteuer.
                    if let (Some(n), Some(s), Some(b)) = (netto, steuer, brutto) {
                        if n + s != b {
                            return Err(garde::Error::new(format!(
                                "gesamtnetto ({n}) + gesamtsteuer ({s}) must equal \
                                 gesamtbrutto ({b})"
                            )));
                        }
                    }

                    // 3. sum(steuerbetraege[*].steuerwert) = gesamtsteuer.
                    //
                    // Only checked when every entry states a `steuerwert`: a list
                    // that omits one is incomplete rather than wrong, and summing
                    // the rest would report a mismatch that is not there.
                    if let (Some(entries), Some(total)) = (v.steuerbetraege.as_deref(), steuer) {
                        let all_stated =
                            !entries.is_empty() && entries.iter().all(|e| e.steuerwert.is_some());
                        if all_stated {
                            let summed = entries
                                .iter()
                                .filter_map(|e| e.steuerwert)
                                .try_fold(Decimal::ZERO, |acc, v| acc.checked_add(v));
                            match summed {
                                Some(sum) if sum != total => {
                                    return Err(garde::Error::new(format!(
                                        "steuerbetraege sum to {sum}, but gesamtsteuer \
                                         is {total}"
                                    )));
                                }
                                None => {
                                    return Err(garde::Error::new(
                                        "steuerbetraege overflow the Decimal range when summed",
                                    ));
                                }
                                _ => {}
                            }
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
            /// When both dates are present, `startdatum` must be **on or before**
            /// `enddatum` (only checked when `time` is active).
            ///
            /// # Why `<=` and not `<`
            ///
            /// BO4E declares both dates **inclusive**, and gives `'2025-01-01'` as
            /// the example for *both* of them: `startdatum == enddatum` is a valid
            /// one-day period, not an empty one. Requiring a strict `<` — as an
            /// earlier revision did, on the assumption that `enddatum` was
            /// exclusive — rejected every single-day Zeitraum in circulation.
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

                // Date ordering is only checked with `time`, where the fields are
                // `time::Date` and compare chronologically.  Without it they are
                // `String`, and a lexicographic comparison of partial ISO-8601
                // forms is not the same order.
                #[cfg(feature = "time")]
                if let (Some(start), Some(end)) = (v.startdatum, v.enddatum) {
                    if start > end {
                        return Err(garde::Error::new(format!(
                            "startdatum ({start}) must be on or before enddatum ({end}); \
                             both bounds are inclusive, so a one-day period has start == end"
                        )));
                    }
                }

                Ok(())
            }

            /// Kostenposition arithmetic: the line total must be the product of
            /// unit price and quantity.
            ///
            /// The schema describes `betragKostenposition` as the result of
            /// *"<Menge * Einzelpreis>"* **or** *"<Einzelpreis / (Anzahl Tage
            /// Jahr) * zeitmenge"*. Only the first form is checkable from the
            /// fields alone — the second needs the day count of the billing
            /// year, which the COM does not carry — so a position that states a
            /// `zeitmenge` is skipped rather than measured against the wrong
            /// formula.
            ///
            /// # Rounding
            ///
            /// The product is compared at the **scale of the stated amount**.
            /// A unit price of `0.2843 €/kWh` over `3333 kWh` is `947.5719`,
            /// which every invoice in circulation writes as `947.57`; demanding
            /// exact equality (or equality at ten decimal places, as an earlier
            /// revision did) rejects the entire real-world corpus.
            ///
            /// Gated on the `decimal` feature; without it the fields are
            /// `Option<String>` and numeric arithmetic is not available.
            // Without `decimal` the body compiles away and `v` goes unread.
            #[cfg_attr(not(feature = "decimal"), allow(unused_variables))]
            pub fn validate_kostenposition_arithmetic(
                v: &Kostenposition,
                _: &(),
            ) -> Result<(), garde::Error> {
                #[cfg(feature = "decimal")]
                {
                    use rust_decimal::Decimal;

                    // A time-proportional position uses the other formula.
                    if v.zeitmenge.is_some() {
                        return Ok(());
                    }
                    let betrag = v.betrag_kostenposition.as_ref().and_then(|b| b.wert);
                    let einzelpreis = v.einzelpreis.as_ref().and_then(|p| p.wert);
                    let menge = v.menge.as_ref().and_then(|m| m.wert);

                    if let (Some(ep), Some(m), Some(b)) = (einzelpreis, menge, betrag) {
                        let Some(product) = ep.checked_mul(m) else {
                            return Err(garde::Error::new(format!(
                                "einzelpreis ({ep}) * menge ({m}) overflows the Decimal range"
                            )));
                        };
                        // Accept the amount if it is *a* correct rounding of the
                        // product to its own scale — i.e. within half a unit in
                        // the last stated place.  Comparing against one rounding
                        // mode would reject the other: invoices round halves up,
                        // `Decimal::round_dp` rounds them to even.
                        //
                        // `try_new`, not `new`: an amount already at `Decimal`'s
                        // maximum scale of 28 would ask for a scale of 29, and
                        // `Decimal::new` *panics* on that.  A payload can carry
                        // one, and a validator handed untrusted input must not be
                        // a way to bring the process down.  At that scale there
                        // is no room left for a tolerance anyway, so the
                        // comparison falls back to exact equality.
                        let scale = b.scale();
                        let half_ulp = scale
                            .checked_add(1)
                            .and_then(|s| Decimal::try_new(5, s).ok())
                            .unwrap_or(Decimal::ZERO);
                        if (product - b).abs() > half_ulp {
                            return Err(garde::Error::new(format!(
                                "einzelpreis.wert ({ep}) * menge.wert ({m}) = {product}, \
                                 which does not round to betrag_kostenposition.wert ({b}) \
                                 at its own scale of {scale} decimal place(s)"
                            )));
                        }
                    }
                } // end #[cfg(feature = "decimal")]
                Ok(())
            }

            /// Data-quality rules this crate considers sensible, which **BO4E
            /// does not state**.
            ///
            /// Nothing here is wired into `#[derive(garde::Validate)]`, so
            /// `.validate()` and [`Validated<T>`](crate::validation::Validated)
            /// never run it. That keeps `.validate()` answering *"does this
            /// conform to BO4E"* — a claim you can make about a document a
            /// counterparty sent.
            ///
            /// Call these by name: typically on documents you produce, or as a
            /// warning rather than a rejection on documents you receive.
            ///
            /// ```
            /// # #[cfg(all(feature = "versioned", feature = "decimal"))] {
            /// use rubo4e::current::{Betrag, Rechnung};
            /// use rubo4e::validation::current::quality;
            /// use garde::Validate as _;
            /// use rust_decimal::Decimal;
            ///
            /// let netto = Betrag { wert: Some(Decimal::from(100)), ..Default::default() };
            /// let partial = Rechnung {
            ///     gesamtnetto: Some(netto.clone()),
            ///     gesamtsteuer: Some(netto),
            ///     ..Default::default()   // gesamtbrutto omitted
            /// };
            ///
            /// // Conformance is unaffected — BO4E requires none of the three.
            /// assert!(partial.validate().is_ok());
            /// // The house rule is available separately.
            /// assert!(quality::rechnung_totals_are_complete(&partial).is_err());
            /// # }
            /// ```
            pub mod quality {
                use super::Rechnung;

                /// All three invoice totals must be present, or none of them.
                ///
                /// `gesamtbrutto = gesamtnetto + gesamtsteuer`, so any two
                /// determine the third, and stating exactly two makes the reader
                /// do arithmetic the sender could have. BO4E marks none of the
                /// three `required`, which is why this is not a conformance rule.
                ///
                /// Fewer than two stated is not reported: an invoice may
                /// legitimately carry only a gross total.
                ///
                /// # Errors
                /// A [`garde::Error`] naming the missing total, so this composes
                /// into a `garde` pipeline of your own.
                // Without `decimal` the amounts are `String` and the body
                // compiles away, so `v` goes unread.
                #[cfg_attr(not(feature = "decimal"), allow(unused_variables))]
                pub fn rechnung_totals_are_complete(v: &Rechnung) -> Result<(), garde::Error> {
                    #[cfg(feature = "decimal")]
                    {
                        let stated = [
                            ("gesamtnetto", v.gesamtnetto.as_ref().and_then(|b| b.wert)),
                            ("gesamtsteuer", v.gesamtsteuer.as_ref().and_then(|b| b.wert)),
                            ("gesamtbrutto", v.gesamtbrutto.as_ref().and_then(|b| b.wert)),
                        ];
                        let missing: Vec<&str> = stated
                            .iter()
                            .filter(|(_, amount)| amount.is_none())
                            .map(|(name, _)| *name)
                            .collect();
                        if missing.len() == 1 {
                            return Err(garde::Error::new(format!(
                                "two of the three invoice totals are stated, so {} is \
                                 derivable and should be stated too",
                                missing[0],
                            )));
                        }
                    }
                    Ok(())
                }
            }
        }
    };
}

#[cfg(feature = "versioned")]
impl_validators!(v202607);

/// The validators for the current stable BO4E schema series — the counterpart
/// of [`rubo4e::current`](crate::current).
///
/// Import from here for the same reason you import types from `crate::current`:
/// so that no downstream file has to name a schema version, and a CI guard that
/// greps for `rubo4e::v202607` stays clean. It resolves to the same functions as
/// [`v202607`], which `tests/validation.rs` pins.
///
/// ```
/// # #[cfg(all(feature = "versioned", feature = "validate"))] {
/// use rubo4e::current::Zeitraum;
/// use rubo4e::validation::current::validate_zeitraum;
///
/// // An empty Zeitraum carries no information, whichever path you call it by.
/// assert!(validate_zeitraum(&Zeitraum::default(), &()).is_err());
/// # }
/// ```
///
/// The [`quality`](current::quality) submodule comes along with it.
#[cfg(feature = "versioned")]
#[cfg_attr(docsrs, doc(cfg(feature = "versioned")))]
pub mod current {
    pub use super::v202607::*;
}

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

/// Flattens a [`garde::Report`] into one [`ValidationFailure`] per field error.
///
/// `garde::Report` only implements `Display` — one string with every failure in
/// it — which is unusable for anything but a log line. The flattened form lets
/// callers:
/// - render structured API error responses
/// - log individual field names with key-value pairs
/// - build test assertions per field
///
/// # Example
/// ```
/// # #[cfg(feature = "versioned")] {
/// use rubo4e::validation::{report_errors, Validated};
/// use rubo4e::current::{Adresse, Geokoordinaten, Marktlokation};
///
/// let conflicting = Marktlokation {
///     lokationsadresse: Some(Adresse::default()),
///     geoadresse: Some(Geokoordinaten::default()),
///     ..Default::default()
/// };
/// let report = Validated::new(conflicting).unwrap_err();
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
