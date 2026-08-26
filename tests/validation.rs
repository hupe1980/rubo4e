//! Integration tests for `#[cfg(feature = "validate")]` functionality.
//!
//! Run with:
//! ```text
//! cargo test --test validation --features validate,versioned,time,decimal
//! ```

#[cfg(all(feature = "validate", feature = "versioned"))]
mod identifier_tests {
    use garde::Validate as _;
    use rubo4e::identifiers::MaloId;

    #[test]
    fn malo_id_valid_passes_garde() {
        // 51238696781 is a valid BDEW 11-digit ID (passes checksum).
        let id = MaloId::new("51238696781").unwrap();
        assert!(id.validate().is_ok());
    }

    #[test]
    fn malo_id_invalid_rejected_at_construction() {
        // Wrong length → construction fails, so garde never sees it.
        assert!(MaloId::new("12345").is_err());
    }
}

#[cfg(all(feature = "validate", feature = "versioned"))]
mod marktlokation_tests {
    use garde::Validate as _;
    use rubo4e::v202607::{Adresse, Geokoordinaten, Katasteradresse, Marktlokation};

    fn base() -> Marktlokation {
        Marktlokation::default()
    }

    #[test]
    fn one_ortsangabe_is_fine() {
        let mut m = base();
        m.lokationsadresse = Some(Adresse::default());
        assert!(m.validate().is_ok());
    }

    /// BO4E states mutual exclusivity, not presence: no `required` array, no
    /// `oneOf`, all three properties default to `null`.
    ///
    /// BO4E has no reference type, so a location referenced from a `Rechnung`, a
    /// `Vertrag`, or an `Angebot` is a full `Marktlokation` carrying little more
    /// than its ID — which makes the empty case the common one.
    #[test]
    fn no_ortsangabe_at_all_is_fine() {
        assert!(
            base().validate().is_ok(),
            "a Marktlokation with no Ortsangabe conforms — BO4E marks none of \
             the three required"
        );

        let referenced = Marktlokation {
            marktlokations_id: Some(
                rubo4e::identifiers::MaloId::new("51238696781").expect("valid"),
            ),
            ..Default::default()
        };
        assert!(
            referenced.validate().is_ok(),
            "an ID-only reference conforms"
        );
    }

    /// Two is what the rule is actually about: they would disagree about where
    /// the location is.
    #[test]
    fn two_ortsangaben_conflict() {
        let mut m = base();
        m.lokationsadresse = Some(Adresse::default());
        m.geoadresse = Some(Geokoordinaten::default());

        let report = m.validate().expect_err("two Ortsangaben must conflict");
        let text = report.to_string();
        assert!(
            text.contains("lokationsadresse") && text.contains("geoadresse"),
            "the message must name which two are set: {text}"
        );
        assert!(
            !text.contains("katasterinformation"),
            "…and must not name the one that is not: {text}"
        );
    }

    /// All three is a conflict too, and every one of them is named.
    #[test]
    fn three_ortsangaben_conflict() {
        let mut m = base();
        m.lokationsadresse = Some(Adresse::default());
        m.geoadresse = Some(Geokoordinaten::default());
        m.katasterinformation = Some(Katasteradresse::default());

        let text = m.validate().expect_err("three must conflict").to_string();
        for field in ["lokationsadresse", "geoadresse", "katasterinformation"] {
            assert!(text.contains(field), "{field} missing from: {text}");
        }
    }

    #[test]
    fn kataster_alone_is_fine() {
        let mut m = base();
        m.katasterinformation = Some(Katasteradresse::default());
        assert!(m.validate().is_ok());
    }

    #[cfg(feature = "json")]
    #[test]
    fn golden_payload_with_lokationsadresse_validates_ok() {
        // Deserialize a realistic Marktlokation JSON payload that has
        // `lokationsadresse` set and confirm that validate() returns Ok.
        let json = r#"{
            "lokationsadresse": {
                "strasse": "Musterstraße",
                "hausnummer": "1",
                "postleitzahl": "12345",
                "ort": "Berlin",
                "landescode": "DE"
            }
        }"#;
        let m: Marktlokation = serde_json::from_str(json).expect("deserialize failed");
        assert!(
            m.lokationsadresse.is_some(),
            "lokationsadresse should be set"
        );
        assert!(m.geoadresse.is_none());
        assert!(m.katasterinformation.is_none());
        assert!(
            m.validate().is_ok(),
            "validate() should be Ok with lokationsadresse set"
        );
    }
}

#[cfg(all(feature = "validate", feature = "versioned"))]
mod messlokation_tests {
    use garde::Validate as _;
    use rubo4e::v202607::{Adresse, Geokoordinaten, Messlokation};

    #[test]
    fn one_ortsangabe_is_fine() {
        let m = Messlokation {
            messadresse: Some(Adresse::default()),
            ..Default::default()
        };
        assert!(m.validate().is_ok());
    }

    /// The schema's own description of `messadresse` reads *"Nur angeben, wenn
    /// diese von der Adresse der Marktlokation abweicht."* A Messlokation whose
    /// address matches its Marktlokation is **supposed** to carry none, so
    /// rejecting the empty case rejected the documented common case.
    #[test]
    fn no_ortsangabe_at_all_is_fine() {
        assert!(
            Messlokation::default().validate().is_ok(),
            "the schema tells senders to omit messadresse when it matches the \
             Marktlokation's — that cannot then be a violation"
        );
    }

    #[test]
    fn two_ortsangaben_conflict() {
        let m = Messlokation {
            messadresse: Some(Adresse::default()),
            geoadresse: Some(Geokoordinaten::default()),
            ..Default::default()
        };
        let text = m.validate().expect_err("two must conflict").to_string();
        assert!(
            text.contains("messadresse") && text.contains("geoadresse"),
            "the message must name which two are set: {text}"
        );
    }
}

#[cfg(all(feature = "validate", feature = "versioned", feature = "time"))]
mod vertrag_tests {
    use garde::Validate as _;
    use rubo4e::v202607::Vertrag;
    use time::OffsetDateTime;

    #[test]
    fn valid_date_range_ok() {
        let v = Vertrag {
            vertragsbeginn: Some(OffsetDateTime::from_unix_timestamp(0).unwrap()),
            vertragsende: Some(OffsetDateTime::from_unix_timestamp(3600).unwrap()),
            ..Default::default()
        };
        assert!(v.validate().is_ok());
    }

    #[test]
    fn inverted_date_range_fails() {
        let v = Vertrag {
            vertragsbeginn: Some(OffsetDateTime::from_unix_timestamp(3600).unwrap()),
            vertragsende: Some(OffsetDateTime::from_unix_timestamp(0).unwrap()),
            ..Default::default()
        };
        assert!(v.validate().is_err());
    }

    #[test]
    fn one_date_missing_ok() {
        let v = Vertrag {
            vertragsbeginn: Some(OffsetDateTime::from_unix_timestamp(0).unwrap()),
            // vertragsende = None → no constraint
            ..Default::default()
        };
        assert!(v.validate().is_ok());
    }
}

#[cfg(all(feature = "validate", feature = "versioned", feature = "decimal"))]
mod rechnung_tests {
    use garde::Validate as _;
    use rubo4e::v202607::{Betrag, Rechnung};
    use rust_decimal::prelude::FromStr as _;
    use rust_decimal::Decimal;

    fn dec(s: &str) -> Decimal {
        Decimal::from_str(s).unwrap()
    }

    fn betrag(wert: Decimal) -> Option<Betrag> {
        Some(Betrag {
            wert: Some(wert),
            ..Default::default()
        })
    }

    #[test]
    fn balanced_invoice_ok() {
        let r = Rechnung {
            gesamtnetto: betrag(dec("100.00")),
            gesamtsteuer: betrag(dec("19.00")),
            gesamtbrutto: betrag(dec("119.00")),
            ..Default::default()
        };
        assert!(r.validate().is_ok());
    }

    #[test]
    fn netto_steuer_mismatch_fails() {
        let r = Rechnung {
            gesamtnetto: betrag(dec("100.00")),
            gesamtsteuer: betrag(dec("19.00")),
            gesamtbrutto: betrag(dec("120.00")), // off by 1
            ..Default::default()
        };
        assert!(r.validate().is_err());
    }

    /// Two of the three totals is a *quality* judgement, not a conformance one:
    /// BO4E marks none of the three `required` and says nothing about stating
    /// them together, so `.validate()` must stay silent about it.
    #[test]
    fn two_of_three_totals_conforms_but_fails_the_quality_check() {
        use rubo4e::validation::current::quality;

        let r = Rechnung {
            gesamtnetto: betrag(dec("100.00")),
            gesamtsteuer: betrag(dec("19.00")),
            ..Default::default()
        };
        assert!(
            r.validate().is_ok(),
            "BO4E requires none of the three totals — this conforms"
        );

        let text = quality::rechnung_totals_are_complete(&r)
            .expect_err("the opt-in house rule does flag it")
            .to_string();
        assert!(
            text.contains("gesamtbrutto"),
            "the message must name the derivable total: {text}"
        );
    }

    /// …and the quality check is silent on the shapes it has nothing to say
    /// about: all three stated, or fewer than two.
    #[test]
    fn the_completeness_check_only_fires_on_exactly_two() {
        use rubo4e::validation::current::quality;

        let all_three = Rechnung {
            gesamtnetto: betrag(dec("100.00")),
            gesamtsteuer: betrag(dec("19.00")),
            gesamtbrutto: betrag(dec("119.00")),
            ..Default::default()
        };
        assert!(quality::rechnung_totals_are_complete(&all_three).is_ok());

        let gross_only = Rechnung {
            gesamtbrutto: betrag(dec("119.00")),
            ..Default::default()
        };
        assert!(
            quality::rechnung_totals_are_complete(&gross_only).is_ok(),
            "an invoice may legitimately state only a gross total"
        );

        assert!(quality::rechnung_totals_are_complete(&Rechnung::default()).is_ok());
    }

    /// `zuZahlen` is deliberately unchecked: the equation its schema names
    /// references a `rabattBrutto` field v202607 does not have, and deriving it
    /// from the **net** discount instead is off by the VAT on that discount.
    /// This invoice was rejected by that derived rule, and is perfectly valid.
    #[test]
    fn net_discount_does_not_constrain_zu_zahlen() {
        let r = Rechnung {
            gesamtnetto: betrag(dec("100.00")),
            gesamtsteuer: betrag(dec("19.00")),
            gesamtbrutto: betrag(dec("119.00")),
            rabatt_netto: betrag(dec("10.00")),
            // 119 − (10 net + 1.90 VAT on the discount) = 107.10.
            zu_zahlen: betrag(dec("107.10")),
            ..Default::default()
        };
        assert!(
            r.validate().is_ok(),
            "a correctly discounted invoice must not be rejected: {:?}",
            r.validate().unwrap_err()
        );
    }

    #[test]
    fn zu_zahlen_after_advance_payment_ok() {
        use rubo4e::v202607::Vorauszahlung;
        let r = Rechnung {
            gesamtnetto: betrag(dec("100.00")),
            gesamtsteuer: betrag(dec("19.00")),
            gesamtbrutto: betrag(dec("119.00")),
            vorauszahlungen: Some(vec![Vorauszahlung {
                betrag: betrag(dec("20.00")),
                ..Default::default()
            }]),
            zu_zahlen: betrag(dec("99.00")),
            ..Default::default()
        };
        assert!(r.validate().is_ok());
    }

    // ── steuerbetraege must sum to gesamtsteuer ──────────────────────────────

    #[test]
    fn tax_lines_summing_to_total_ok() {
        use rubo4e::v202607::Steuerbetrag;
        let r = Rechnung {
            gesamtnetto: betrag(dec("100.00")),
            gesamtsteuer: betrag(dec("19.00")),
            gesamtbrutto: betrag(dec("119.00")),
            steuerbetraege: Some(vec![
                Steuerbetrag {
                    steuerwert: Some(dec("11.40")),
                    ..Default::default()
                },
                Steuerbetrag {
                    steuerwert: Some(dec("7.60")),
                    ..Default::default()
                },
            ]),
            ..Default::default()
        };
        assert!(r.validate().is_ok());
    }

    #[test]
    fn tax_lines_not_summing_to_total_fails() {
        use rubo4e::v202607::Steuerbetrag;
        let r = Rechnung {
            gesamtnetto: betrag(dec("100.00")),
            gesamtsteuer: betrag(dec("19.00")),
            gesamtbrutto: betrag(dec("119.00")),
            steuerbetraege: Some(vec![Steuerbetrag {
                steuerwert: Some(dec("18.00")),
                ..Default::default()
            }]),
            ..Default::default()
        };
        let err = r.validate().unwrap_err().to_string();
        assert!(err.contains("steuerbetraege"), "unexpected error: {err}");
    }

    /// A tax list with an unstated `steuerwert` is incomplete, not inconsistent —
    /// summing the rest would report a mismatch that is not there.
    #[test]
    fn tax_lines_with_missing_amounts_are_skipped() {
        use rubo4e::v202607::Steuerbetrag;
        let r = Rechnung {
            gesamtnetto: betrag(dec("100.00")),
            gesamtsteuer: betrag(dec("19.00")),
            gesamtbrutto: betrag(dec("119.00")),
            steuerbetraege: Some(vec![
                Steuerbetrag {
                    steuerwert: Some(dec("11.40")),
                    ..Default::default()
                },
                Steuerbetrag::default(),
            ]),
            ..Default::default()
        };
        assert!(r.validate().is_ok());
    }
}

/// `Kostenposition` line-total arithmetic: the product must round to the stated
/// amount at its own scale, and time-proportional positions are out of scope.
#[cfg(all(
    feature = "validate",
    feature = "versioned",
    feature = "decimal",
    feature = "time"
))]
mod kostenposition_tests {
    use garde::Validate as _;
    use rubo4e::v202607::{Kostenposition, Menge, Preis};
    use rust_decimal::Decimal;
    use std::str::FromStr as _;

    fn dec(s: &str) -> Decimal {
        Decimal::from_str(s).unwrap()
    }

    fn position(einzelpreis: &str, menge: &str, betrag: &str) -> Kostenposition {
        Kostenposition {
            einzelpreis: Some(Preis {
                wert: Some(dec(einzelpreis)),
                ..Default::default()
            }),
            menge: Some(Menge {
                wert: Some(dec(menge)),
                ..Default::default()
            }),
            betrag_kostenposition: Some(rubo4e::v202607::Betrag {
                wert: Some(dec(betrag)),
                ..Default::default()
            }),
            ..Default::default()
        }
    }

    #[test]
    fn exact_product_ok() {
        assert!(position("2.00", "3", "6.00").validate().is_ok());
    }

    /// The case that mattered: a real unit price times a real consumption gives
    /// a product with more decimals than any invoice prints.
    #[test]
    fn product_rounded_to_the_stated_scale_ok() {
        // 0.2843 €/kWh × 3333 kWh = 947.5719 → printed as 947.57
        let p = position("0.2843", "3333", "947.57");
        assert!(
            p.validate().is_ok(),
            "rounding to the stated scale must be accepted: {:?}",
            p.validate().unwrap_err()
        );
    }

    #[test]
    fn genuinely_wrong_total_fails() {
        let err = position("0.2843", "3333", "900.00")
            .validate()
            .unwrap_err()
            .to_string();
        assert!(err.contains("betrag_kostenposition"), "unexpected: {err}");
    }

    /// The tolerance is half a unit in the amount's last stated place, which at
    /// `Decimal`'s maximum scale of 28 would ask for a scale of 29 — and
    /// `Decimal::new` **panics** on that rather than returning an error.
    ///
    /// A payload can carry a 28-scale amount, and this validator is what an
    /// ingest boundary runs on untrusted input, so it must return a verdict
    /// instead of taking the process down.
    #[test]
    fn an_amount_at_the_maximum_decimal_scale_does_not_panic() {
        let tiny = "0.0000000000000000000000000001"; // scale 28
        assert_eq!(dec(tiny).scale(), 28, "the fixture must sit at the ceiling");

        // 1 × that amount is exactly that amount: still accepted.
        assert!(position("1", tiny, tiny).validate().is_ok());

        // …and a genuinely wrong total at that scale is still a failure, not a
        // crash — at scale 28 there is no room for a tolerance, so the
        // comparison is exact.
        assert!(position("2", tiny, tiny).validate().is_err());
    }

    /// A time-proportional position is computed by the schema's other formula,
    /// which needs a day count this COM does not carry — so it is not measured
    /// against the product formula at all.
    #[test]
    fn time_proportional_position_is_not_checked() {
        let mut p = position("55.00", "1", "13.75"); // a quarter of a yearly rate
        p.zeitmenge = Some(Menge {
            wert: Some(dec("3")),
            ..Default::default()
        });
        assert!(p.validate().is_ok());
    }
}

/// Validation descends into nested BO/COM values, so `Validated<T>` is a proof
/// about the whole tree rather than only about `T`'s own fields.
#[cfg(all(feature = "validate", feature = "versioned"))]
mod recursion_tests {
    use garde::Validate as _;
    use rubo4e::v202607::*;

    /// A `Zeitraum` with no temporal field at all is invalid standalone, and
    /// must stay invalid when it hangs off an invoice.
    #[test]
    fn a_nested_zeitraum_is_checked() {
        assert!(Zeitraum::default().validate().is_err(), "standalone");

        let r = Rechnung {
            rechnungsperiode: Some(Zeitraum::default()),
            ..Default::default()
        };
        let report = r
            .validate()
            .expect_err("an empty Zeitraum must invalidate the Rechnung carrying it");
        let text = report.to_string();
        assert!(
            text.contains("rechnungsperiode"),
            "the path must name the field it came from: {text}"
        );
    }

    /// Two levels down, through a `Vec` at each — the shape a real cost
    /// breakdown has.
    #[test]
    #[cfg(feature = "decimal")]
    fn a_position_two_levels_down_is_checked() {
        use rust_decimal::Decimal;

        let wrong = Kostenposition {
            einzelpreis: Some(Preis {
                wert: Some(Decimal::from(2)),
                ..Default::default()
            }),
            menge: Some(Menge {
                wert: Some(Decimal::from(3)),
                ..Default::default()
            }),
            betrag_kostenposition: Some(Betrag {
                wert: Some(Decimal::from(999)), // 2 × 3 is not 999
                ..Default::default()
            }),
            ..Default::default()
        };
        assert!(wrong.validate().is_err(), "standalone");

        let kosten = Kosten {
            kostenbloecke: Some(vec![Kostenblock {
                kostenpositionen: Some(vec![wrong]),
                ..Default::default()
            }]),
            ..Default::default()
        };
        let text = kosten
            .validate()
            .expect_err("a wrong line total must invalidate the Kosten carrying it")
            .to_string();
        assert!(
            text.contains("kostenbloecke[0].kostenpositionen[0]"),
            "the report must locate the offending position: {text}"
        );
    }

    /// A nested BO's own cross-field rules run wherever it sits.
    #[test]
    fn a_nested_bos_rules_are_checked() {
        use rubo4e::v202607::Geokoordinaten;

        let malo = Marktlokation {
            lokationsadresse: Some(Adresse::default()),
            ..Default::default()
        };
        assert!(malo.validate().is_ok(), "the outer value alone is valid");

        // …and one nested inside it carrying two conflicting Ortsangaben is not.
        let conflicting = Marktlokation {
            lokationsadresse: Some(Adresse::default()),
            geoadresse: Some(Geokoordinaten::default()),
            ..Default::default()
        };
        let outer = Marktlokation {
            lokationsadresse: Some(Adresse::default()),
            lokationszuordnungen: Some(vec![Box::new(Lokationszuordnung {
                marktlokationen: Some(vec![Box::new(conflicting)]),
                ..Default::default()
            })]),
            ..Default::default()
        };
        let text = outer
            .validate()
            .expect_err("the nested Marktlokation carries two Ortsangaben")
            .to_string();
        assert!(
            text.contains("lokationszuordnungen[0].marktlokationen[0]"),
            "the report must locate the nested value: {text}"
        );
    }

    /// An identifier below the top level is validated too. The newtypes reject
    /// at construction, so the decode is what enforces it — which is the path
    /// that actually matters for a payload.
    #[test]
    #[cfg(feature = "json")]
    fn a_nested_identifier_is_checked() {
        use rubo4e::json::Bo4eJsonExt;

        let ok = r#"{"lokationszuordnungen":[{"marktlokationen":[
            {"marktlokationsId":"51238696781"}]}]}"#;
        let m = Marktlokation::from_json_german(ok).expect("valid check digit");
        assert!(m.validate().is_ok());

        let bad = r#"{"lokationszuordnungen":[{"marktlokationen":[
            {"marktlokationsId":"51238696782"}]}]}"#;
        assert!(
            Marktlokation::from_json_german(bad).is_err(),
            "a nested identifier is validated on the way in"
        );
    }

    /// Recursion follows the data, so the depth it can reach is bounded by the
    /// parser's nesting cap. The deepest payload a reader accepts must not blow
    /// the stack on the way back through it.
    #[test]
    #[cfg(feature = "json")]
    fn the_deepest_accepted_payload_does_not_overflow_the_stack() {
        use rubo4e::json::{Bo4eJsonExt, JsonParseLimits};

        // Each Marktlokation → lokationszuordnungen[0] → marktlokationen[0] is
        // two object levels plus two array levels, so 25 links is ~100 — just
        // under the 128 the readers cap at.
        let mut body = String::from(r#"{"lokationsadresse":{"ort":"Bremen"}}"#);
        for _ in 0..25 {
            body = format!(
                r#"{{"lokationsadresse":{{"ort":"Bremen"}},"lokationszuordnungen":[{{"marktlokationen":[{body}]}}]}}"#
            );
        }

        let m = Marktlokation::from_json_german_hardened(&body, JsonParseLimits::unlimited())
            .expect("within the depth cap");
        assert!(
            m.validate().is_ok(),
            "{:?}",
            m.validate().err().map(|e| e.to_string())
        );
    }

    /// The golden corpus must still validate — recursion must not have made a
    /// real-world payload fail.
    #[test]
    #[cfg(feature = "json")]
    fn the_golden_marktlokation_still_validates() {
        use rubo4e::json::Bo4eJsonExt;

        let body = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("tests/golden/marktlokation_typical.json"),
        )
        .expect("golden payload");
        let malo = Marktlokation::from_json_german(&body).expect("valid BO4E payload");
        assert!(
            malo.validate().is_ok(),
            "a real payload must survive recursive validation: {:?}",
            malo.validate().unwrap_err().to_string()
        );
    }
}

#[cfg(all(feature = "validate", feature = "versioned"))]
mod report_errors_tests {
    use garde::Validate as _;
    use rubo4e::v202607::Marktlokation;
    use rubo4e::validation::report_errors;

    #[test]
    fn report_errors_returns_structured_failures() {
        use rubo4e::v202607::{Adresse, Geokoordinaten};

        // Two conflicting Ortsangaben → validation fails.
        let malo = Marktlokation {
            lokationsadresse: Some(Adresse::default()),
            geoadresse: Some(Geokoordinaten::default()),
            ..Default::default()
        };
        let report = malo
            .validate()
            .expect_err("two Ortsangaben should be invalid");
        let failures = report_errors(&report);
        assert!(
            !failures.is_empty(),
            "report_errors must return at least one failure for invalid Marktlokation"
        );
        // Each failure should have a non-empty message.
        // (Paths may be empty for root-level garde errors on the struct itself.)
        for f in &failures {
            assert!(!f.message.is_empty(), "failure message must not be empty");
        }
        // The failure must name the conflicting Ortsangaben, not just say "invalid".
        let combined: String = failures
            .iter()
            .map(|f| f.message.as_str())
            .collect::<Vec<_>>()
            .join(" ");
        assert!(
            combined.contains("lokationsadresse") && combined.contains("geoadresse"),
            "the message should name which Ortsangaben conflict; got: {combined}"
        );
    }

    #[test]
    fn report_errors_empty_for_valid_type() {
        use rubo4e::v202607::Adresse;
        let malo = Marktlokation {
            lokationsadresse: Some(Adresse::default()),
            ..Default::default()
        };
        let failures = match malo.validate() {
            Ok(()) => vec![],
            Err(r) => report_errors(&r),
        };
        // An address set → the cross-field validator should pass.
        assert!(
            failures.is_empty(),
            "expected no failures for Marktlokation with address; got: {failures:?}"
        );
    }
}

// ── Zeitraum validation (date ordering) ─────────────────────────────────────

#[cfg(all(feature = "validate", feature = "versioned", feature = "time"))]
mod zeitraum_tests {
    use garde::Validate as _;
    use rubo4e::v202607::Zeitraum;
    use time::macros::date;

    fn zeitraum(start: Option<time::Date>, end: Option<time::Date>) -> Zeitraum {
        Zeitraum {
            startdatum: start,
            enddatum: end,
            ..Default::default()
        }
    }

    #[test]
    fn valid_closed_range_ok() {
        let z = zeitraum(Some(date!(2025 - 01 - 01)), Some(date!(2025 - 12 - 31)));
        assert!(z.validate().is_ok(), "closed range start < end must pass");
    }

    /// BO4E declares both dates **inclusive** and gives `'2025-01-01'` as the
    /// example for each, so `start == end` is a one-day period — valid, and the
    /// commonest shape in a daily-granularity payload.
    #[test]
    fn same_day_is_a_valid_one_day_period() {
        let z = zeitraum(Some(date!(2025 - 06 - 15)), Some(date!(2025 - 06 - 15)));
        assert!(
            z.validate().is_ok(),
            "start == end is a one-day period, not an empty one"
        );
    }

    #[test]
    fn inverted_range_fails() {
        let z = zeitraum(Some(date!(2025 - 12 - 31)), Some(date!(2025 - 01 - 01)));
        assert!(z.validate().is_err(), "end before start must be rejected");
    }

    #[test]
    fn only_start_ok() {
        // open-ended period (no enddatum) — validator only fires when both are present
        let z = zeitraum(Some(date!(2025 - 01 - 01)), None);
        assert!(z.validate().is_ok(), "open-ended period (no end) must pass");
    }

    #[test]
    fn only_end_ok() {
        let z = zeitraum(None, Some(date!(2025 - 12 - 31)));
        assert!(z.validate().is_ok(), "period with only end must pass");
    }

    #[test]
    fn all_absent_fails() {
        // A Zeitraum with NO temporal information is invalid by design.
        // The validator requires at least one of: dauer, startdatum/enddatum,
        // or startuhrzeit/enduhrzeit.
        let z = zeitraum(None, None);
        assert!(
            z.validate().is_err(),
            "empty Zeitraum (no temporal attributes at all) must fail validation"
        );
    }

    #[test]
    fn no_dates_but_dauer_ok() {
        // Zeitraum with only dauer (no start/end dates) passes the date-ordering
        // check, which only fires when BOTH startdatum and enddatum are present.
        let z = rubo4e::v202607::Zeitraum {
            dauer: Some("15".to_owned()),
            ..Default::default()
        };
        assert!(
            z.validate().is_ok(),
            "Zeitraum with only dauer must pass date-ordering check"
        );
    }

    #[test]
    fn error_message_contains_dates() {
        let z = zeitraum(Some(date!(2025 - 12 - 31)), Some(date!(2025 - 01 - 01)));
        let err = z.validate().expect_err("inverted range must fail");
        let msg = err.to_string();
        // The error message should include both boundary dates for diagnostics.
        assert!(
            msg.contains("2025"),
            "error message should include the year; got: {msg}"
        );
    }
}

// ── Convenience method tests ─────────────────────────────────────────────────

#[cfg(all(feature = "versioned", feature = "time"))]
mod convenience_tests {
    use rubo4e::v202607::{PreisblattNetznutzung, Rechnung, Zeitraum};
    use time::macros::date;

    fn closed_zeitraum() -> Zeitraum {
        Zeitraum {
            startdatum: Some(date!(2025 - 01 - 01)),
            enddatum: Some(date!(2025 - 12 - 31)),
            ..Default::default()
        }
    }

    fn open_zeitraum() -> Zeitraum {
        Zeitraum {
            startdatum: Some(date!(2025 - 01 - 01)),
            enddatum: None,
            ..Default::default()
        }
    }

    fn no_start_zeitraum() -> Zeitraum {
        Zeitraum {
            startdatum: None,
            enddatum: Some(date!(2025 - 12 - 31)),
            ..Default::default()
        }
    }

    // ── Zeitraum::as_inclusive_range ─────────────────────────────────────────
    //
    // BO4E declares both dates **inclusive**, so `closed_zeitraum()` is the whole
    // of 2025 and its `enddatum` is inside the period, not the first day after it.

    #[test]
    fn inclusive_range_both_present() {
        let r = closed_zeitraum()
            .as_inclusive_range()
            .expect("both dates present");
        assert_eq!(*r.start(), date!(2025 - 01 - 01));
        assert_eq!(*r.end(), date!(2025 - 12 - 31));
        assert!(r.contains(&date!(2025 - 12 - 31)), "the end date is inside");
    }

    #[test]
    fn inclusive_range_needs_both_bounds() {
        assert!(open_zeitraum().as_inclusive_range().is_none());
        assert!(no_start_zeitraum().as_inclusive_range().is_none());
    }

    // ── Zeitraum::bounds ─────────────────────────────────────────────────────

    #[test]
    fn bounds_reports_each_side_independently() {
        assert_eq!(
            closed_zeitraum().bounds(),
            (Some(date!(2025 - 01 - 01)), Some(date!(2025 - 12 - 31)))
        );
        assert_eq!(
            open_zeitraum().bounds(),
            (Some(date!(2025 - 01 - 01)), None)
        );
        assert_eq!(
            no_start_zeitraum().bounds(),
            (None, Some(date!(2025 - 12 - 31)))
        );
        assert_eq!(Zeitraum::default().bounds(), (None, None));
    }

    // ── Zeitraum::contains ───────────────────────────────────────────────────

    /// `enddatum` is **inclusive** per the schema, so the last day of a period
    /// is inside it. Reading it exclusively drops a day from every billing
    /// period and every price-sheet validity.
    #[test]
    fn contains_includes_both_boundaries() {
        let z = closed_zeitraum();
        assert!(z.contains(date!(2025 - 01 - 01)), "startdatum is inside");
        assert!(z.contains(date!(2025 - 07 - 01)));
        assert!(z.contains(date!(2025 - 12 - 31)), "enddatum is inside");

        assert!(!z.contains(date!(2024 - 12 - 31)));
        assert!(!z.contains(date!(2026 - 01 - 01)));
    }

    /// BO4E gives `'2025-01-01'` as the example for *both* dates, so a one-day
    /// period is `start == end`.
    #[test]
    fn a_single_day_period_contains_its_one_day() {
        let one_day = Zeitraum {
            startdatum: Some(date!(2025 - 03 - 15)),
            enddatum: Some(date!(2025 - 03 - 15)),
            ..Default::default()
        };
        assert!(one_day.contains(date!(2025 - 03 - 15)));
        assert!(!one_day.contains(date!(2025 - 03 - 14)));
        assert!(!one_day.contains(date!(2025 - 03 - 16)));
        assert_eq!(one_day.whole_days(), Some(1));

        // …and it must validate: the ordering rule is `<=`, not `<`.
        #[cfg(feature = "validate")]
        {
            use garde::Validate as _;
            assert!(one_day.validate().is_ok(), "a one-day period is valid BO4E");
        }
    }

    #[test]
    fn an_absent_boundary_is_unbounded_on_that_side() {
        assert!(open_zeitraum().contains(date!(2099 - 12 - 31)));
        assert!(!open_zeitraum().contains(date!(2024 - 12 - 31)));

        assert!(no_start_zeitraum().contains(date!(1900 - 01 - 01)));
        assert!(!no_start_zeitraum().contains(date!(2026 - 01 - 01)));
    }

    // ── Zeitraum::whole_days ─────────────────────────────────────────────────

    #[test]
    fn whole_days_counts_both_boundaries() {
        // January 2026: 31 days, not 30 and not 32.
        let january = Zeitraum {
            startdatum: Some(date!(2026 - 01 - 01)),
            enddatum: Some(date!(2026 - 01 - 31)),
            ..Default::default()
        };
        assert_eq!(january.whole_days(), Some(31));

        // A full non-leap year.
        assert_eq!(closed_zeitraum().whole_days(), Some(365));

        // A leap year.
        let leap = Zeitraum {
            startdatum: Some(date!(2024 - 01 - 01)),
            enddatum: Some(date!(2024 - 12 - 31)),
            ..Default::default()
        };
        assert_eq!(leap.whole_days(), Some(366));
    }

    #[test]
    fn whole_days_needs_both_bounds() {
        assert_eq!(open_zeitraum().whole_days(), None);
        assert_eq!(no_start_zeitraum().whole_days(), None);
    }

    // ── Rechnung::billing_period ─────────────────────────────────────────────

    #[test]
    fn billing_period_with_full_period() {
        let r = Rechnung {
            rechnungsperiode: Some(closed_zeitraum()),
            ..Default::default()
        };
        let period = r.billing_period().expect("both dates present");
        assert_eq!(*period.start(), date!(2025 - 01 - 01));
        assert_eq!(*period.end(), date!(2025 - 12 - 31));
        assert!(
            period.contains(&date!(2025 - 12 - 31)),
            "the last day of the period is billed"
        );
    }

    #[test]
    fn billing_period_no_periode_returns_none() {
        let r = Rechnung {
            rechnungsperiode: None,
            ..Default::default()
        };
        assert!(r.billing_period().is_none());
    }

    #[test]
    fn billing_period_open_ended_returns_none() {
        let r = Rechnung {
            rechnungsperiode: Some(open_zeitraum()),
            ..Default::default()
        };
        assert!(r.billing_period().is_none());
    }

    // ── PreisblattNetznutzung::validity ──────────────────────────────────────

    #[test]
    fn validity_reports_both_bounds() {
        let p = PreisblattNetznutzung {
            gueltigkeit: Some(closed_zeitraum()),
            ..Default::default()
        };
        assert_eq!(
            p.validity(),
            (Some(date!(2025 - 01 - 01)), Some(date!(2025 - 12 - 31)))
        );
        assert!(
            p.is_valid_at(date!(2025 - 12 - 31)),
            "a price sheet is still valid on its enddatum"
        );
    }

    #[test]
    fn validity_open_ended() {
        let p = PreisblattNetznutzung {
            gueltigkeit: Some(open_zeitraum()),
            ..Default::default()
        };
        assert_eq!(p.validity(), (Some(date!(2025 - 01 - 01)), None));
    }

    #[test]
    fn validity_without_gueltigkeit_is_unstated_and_never_valid() {
        let p = PreisblattNetznutzung {
            gueltigkeit: None,
            ..Default::default()
        };
        assert_eq!(p.validity(), (None, None));
        assert!(
            !p.is_valid_at(date!(2025 - 06 - 01)),
            "an unstated validity must not read as valid"
        );
    }
}

/// `Validated<T>` validates on the way *in*, so a handler can take one as its
/// request body and cannot forget the check.
#[cfg(all(feature = "validate", feature = "versioned", feature = "json"))]
mod validated_deserialize_tests {
    use rubo4e::current::{Adresse, Marktlokation};
    use rubo4e::validation::Validated;

    #[test]
    fn a_valid_payload_decodes() {
        let body = r#"{"marktlokationsId":"51238696781","lokationsadresse":{"ort":"Bremen"}}"#;
        let malo: Validated<Marktlokation> = serde_json::from_str(body).expect("valid");
        assert_eq!(
            malo.lokationsadresse
                .as_ref()
                .and_then(|a| a.ort.as_deref()),
            Some("Bremen")
        );
    }

    /// The rule this exists for: an invalid value must not produce a
    /// `Validated<T>` at all.
    #[test]
    fn an_invalid_payload_does_not_decode() {
        // Two conflicting Ortsangaben.
        let body = r#"{"lokationsadresse":{"ort":"Bremen"},"geoadresse":{"breitengrad":"53.1"}}"#;
        assert!(
            serde_json::from_str::<Marktlokation>(body).is_ok(),
            "plain T still decodes — the serde path stays lenient"
        );

        let err = serde_json::from_str::<Validated<Marktlokation>>(body)
            .expect_err("the wrapper must refuse it");
        assert!(
            err.to_string().contains("Ortsangabe"),
            "the garde report must reach the deserializer error: {err}"
        );
    }

    /// An ID-only reference is the common shape, so it must decode.
    #[test]
    fn an_id_only_reference_decodes() {
        let body = r#"{"marktlokationsId":"51238696781"}"#;
        let malo: Validated<Marktlokation> =
            serde_json::from_str(body).expect("a referenced location carries only its ID");
        assert!(malo.lokationsadresse.is_none());
    }

    /// Nested rules apply too — the check is the same recursive one
    /// `Validated::new` runs.
    #[test]
    fn a_nested_violation_is_refused() {
        let body = r#"{
            "marktlokationsId":"51238696781",
            "lokationsadresse":{"ort":"Bremen"},
            "lokationszuordnungen":[{"marktlokationen":[
                {"lokationsadresse":{"ort":"Bremen"},"geoadresse":{"breitengrad":"53.1"}}]}]
        }"#;
        // The nested Marktlokation carries two Ortsangaben.
        assert!(serde_json::from_str::<Validated<Marktlokation>>(body).is_err());
    }

    /// Round-trip: `Serialize` is transparent, so a `Validated<T>` re-encodes to
    /// the same bytes `T` would.
    #[test]
    fn serialization_is_transparent() {
        let malo = Marktlokation {
            lokationsadresse: Some(Adresse {
                ort: Some("Bremen".into()),
                ..Default::default()
            }),
            ..Default::default()
        };
        let wrapped = Validated::new(malo.clone()).expect("valid");
        assert_eq!(
            serde_json::to_string(&wrapped).unwrap(),
            serde_json::to_string(&malo).unwrap()
        );
    }
}

/// `rubo4e::validation::current` must resolve to the same series as
/// `rubo4e::current`, and expose the same items as the versioned module.
///
/// Downstream code imports validators from `current` for the same reason it
/// imports types from there — so no file names a schema version and a CI guard
/// grepping for `rubo4e::v202607` stays clean. That only holds if the two
/// aliases advance together.
#[cfg(all(feature = "validate", feature = "versioned"))]
mod current_alias_tests {
    use rubo4e::current::Zeitraum;
    use rubo4e::validation::{current, v202607};

    /// Function pointers compare equal only when they are the same function, so
    /// this proves the alias resolves to the versioned module rather than to a
    /// copy that could drift.
    #[test]
    fn current_is_the_versioned_module() {
        type Check<T> = fn(&T, &()) -> Result<(), garde::Error>;

        let via_current: Check<Zeitraum> = current::validate_zeitraum;
        let via_version: Check<Zeitraum> = v202607::validate_zeitraum;
        assert!(std::ptr::fn_addr_eq(via_current, via_version));

        let quality_current: fn(&rubo4e::current::Rechnung) -> Result<(), garde::Error> =
            current::quality::rechnung_totals_are_complete;
        let quality_version: fn(&rubo4e::current::Rechnung) -> Result<(), garde::Error> =
            v202607::quality::rechnung_totals_are_complete;
        assert!(std::ptr::fn_addr_eq(quality_current, quality_version));
    }

    /// …and every validator the versioned module exposes is reachable through
    /// the alias. Naming them costs one line each and is what fails the build
    /// when a new validator is added to only one of the two.
    #[test]
    fn every_validator_is_reachable_through_current() {
        use rubo4e::current::{
            Bilanzierung, Kostenposition, Marktlokation, Messlokation, Rechnung, Vertrag, Zeitraum,
        };

        fn is_validator<T>(_: fn(&T, &()) -> Result<(), garde::Error>) {}

        is_validator::<Marktlokation>(current::validate_marktlokation);
        is_validator::<Messlokation>(current::validate_messlokation);
        is_validator::<Vertrag>(current::validate_vertrag_dates);
        is_validator::<Bilanzierung>(current::validate_bilanzierung_dates);
        is_validator::<Rechnung>(current::validate_rechnung_arithmetic);
        is_validator::<Zeitraum>(current::validate_zeitraum);
        is_validator::<Kostenposition>(current::validate_kostenposition_arithmetic);
    }
}

/// The two BOs the schema marks `required` are constructible without the
/// `builder` feature, and stamp their metadata like every other type.
#[cfg(all(feature = "versioned", feature = "validate"))]
mod required_field_constructor_tests {
    use garde::Validate as _;
    use rubo4e::current::{BoTyp, Lastgang, Menge, Mengeneinheit};
    use rubo4e::Bo4eObject as _;

    #[test]
    fn lastgang_new_fills_in_everything_else() {
        let interval = Menge {
            einheit: Some(Mengeneinheit::Kwh),
            ..Default::default()
        };
        let lg = Lastgang::new(interval.clone());

        assert_eq!(lg.zeit_intervall_laenge, interval);
        assert_eq!(lg.typ, Some(BoTyp::Lastgang), "`_typ` is stamped");
        assert!(lg.marktlokation.is_none(), "everything else defaults");
        assert_eq!(lg.bo_type(), BoTyp::Lastgang);
        assert!(lg.validate().is_ok());
    }

    /// `Tarif` has ten required fields; `new` exists so the type is reachable
    /// without the `builder` feature, not because it reads better than one.
    #[test]
    #[cfg(feature = "decimal")]
    fn tarif_new_takes_every_required_field() {
        use rubo4e::current::{
            Energiemix, Kundentyp, Preisgarantie, Registeranzahl, Sparte, Tarif,
            Tarifberechnungsparameter, Tarifmerkmal, Tariftyp, Vertragskonditionen,
        };

        let t = Tarif::new(
            Tarifberechnungsparameter::default(),
            vec![Energiemix::default()],
            vec![Kundentyp::Gewerbe],
            Preisgarantie::default(),
            Registeranzahl::Eintarif,
            Sparte::Strom,
            vec![Tarifmerkmal::Vorkasse],
            Tariftyp::Grundversorgung,
            Vertragskonditionen::default(),
            "https://example.invalid/tarif".to_owned(),
        );

        assert_eq!(t.sparte, Sparte::Strom);
        assert_eq!(t.typ, Some(BoTyp::Tarif), "`_typ` is stamped");
        assert_eq!(
            t.version.as_deref(),
            Some(Tarif::SCHEMA_VERSION),
            "`_version` is stamped, and agrees with the constant"
        );
        assert!(t.bezeichnung.is_none(), "everything else defaults");
    }
}
