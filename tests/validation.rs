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
    fn xor_exactly_one_address_ok() {
        let mut m = base();
        m.lokationsadresse = Some(Adresse::default());
        assert!(m.validate().is_ok());
    }

    #[test]
    fn xor_no_address_fails() {
        let m = base();
        assert!(m.validate().is_err());
    }

    #[test]
    fn xor_two_addresses_fails() {
        let mut m = base();
        m.lokationsadresse = Some(Adresse::default());
        m.geoadresse = Some(Geokoordinaten::default());
        assert!(m.validate().is_err());
    }

    #[test]
    fn xor_kataster_ok() {
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
    fn xor_messadresse_ok() {
        let m = Messlokation {
            messadresse: Some(Adresse::default()),
            ..Default::default()
        };
        assert!(m.validate().is_ok());
    }

    #[test]
    fn xor_no_address_fails() {
        let m = Messlokation::default();
        assert!(m.validate().is_err());
    }

    #[test]
    fn xor_two_addresses_fails() {
        let m = Messlokation {
            messadresse: Some(Adresse::default()),
            geoadresse: Some(Geokoordinaten::default()),
            ..Default::default()
        };
        assert!(m.validate().is_err());
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

    #[test]
    fn partial_totals_fails() {
        // Only two of the three totals — validation must reject this.
        let r = Rechnung {
            gesamtnetto: betrag(dec("100.00")),
            gesamtsteuer: betrag(dec("19.00")),
            ..Default::default()
        };
        assert!(r.validate().is_err());
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

#[cfg(all(feature = "validate", feature = "versioned"))]
mod report_errors_tests {
    use garde::Validate as _;
    use rubo4e::v202607::Marktlokation;
    use rubo4e::validation::report_errors;

    #[test]
    fn report_errors_returns_structured_failures() {
        let malo = Marktlokation::default(); // no address → validation fails
        let report = malo
            .validate()
            .expect_err("default Marktlokation should be invalid");
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
        // At least one failure should mention the expected address fields.
        let combined: String = failures
            .iter()
            .map(|f| f.message.as_str())
            .collect::<Vec<_>>()
            .join(" ");
        assert!(
            combined.contains("address")
                || combined.contains("adresse")
                || combined.contains("exactly one"),
            "validation message should mention address constraint; got: {combined}"
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
