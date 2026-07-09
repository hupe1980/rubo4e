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
        // 51238696780 is a valid BDEW 11-digit ID (passes checksum).
        let id = MaloId::new("51238696780").unwrap();
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

    #[test]
    fn zu_zahlen_simple_ok() {
        // no discount, no advance payments → zu_zahlen == gesamtbrutto
        let r = Rechnung {
            gesamtnetto: betrag(dec("100.00")),
            gesamtsteuer: betrag(dec("19.00")),
            gesamtbrutto: betrag(dec("119.00")),
            zu_zahlen: betrag(dec("119.00")),
            ..Default::default()
        };
        assert!(r.validate().is_ok());
    }

    #[test]
    fn zu_zahlen_with_discount_ok() {
        // gesamtbrutto 119 - rabatt_netto 10 = 109
        let r = Rechnung {
            gesamtnetto: betrag(dec("100.00")),
            gesamtsteuer: betrag(dec("19.00")),
            gesamtbrutto: betrag(dec("119.00")),
            rabatt_netto: betrag(dec("10.00")),
            zu_zahlen: betrag(dec("109.00")),
            ..Default::default()
        };
        assert!(r.validate().is_ok());
    }

    #[test]
    fn zu_zahlen_with_advance_ok() {
        use rubo4e::v202607::Vorauszahlung;
        // gesamtbrutto 119 - vorauszahlung 20 = 99
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

    #[test]
    fn zu_zahlen_mismatch_fails() {
        let r = Rechnung {
            gesamtnetto: betrag(dec("100.00")),
            gesamtsteuer: betrag(dec("19.00")),
            gesamtbrutto: betrag(dec("119.00")),
            zu_zahlen: betrag(dec("118.00")), // off by 1
            ..Default::default()
        };
        assert!(r.validate().is_err());
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

    #[test]
    fn same_day_fails() {
        // validate_zeitraum requires start < end (strict)
        let z = zeitraum(Some(date!(2025 - 06 - 15)), Some(date!(2025 - 06 - 15)));
        assert!(
            z.validate().is_err(),
            "start == end should be rejected (not a valid interval)"
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

    // ── Zeitraum::as_closed_range ────────────────────────────────────────────

    #[test]
    fn closed_range_both_present() {
        let r = closed_zeitraum().as_closed_range();
        assert_eq!(r, Some((date!(2025 - 01 - 01), date!(2025 - 12 - 31))));
    }

    #[test]
    fn closed_range_open_end_returns_none() {
        assert_eq!(open_zeitraum().as_closed_range(), None);
    }

    #[test]
    fn closed_range_no_start_returns_none() {
        assert_eq!(no_start_zeitraum().as_closed_range(), None);
    }

    // ── Zeitraum::as_half_open_range ─────────────────────────────────────────

    #[test]
    fn half_open_range_both_present() {
        let r = closed_zeitraum().as_half_open_range();
        assert_eq!(
            r,
            Some((date!(2025 - 01 - 01), Some(date!(2025 - 12 - 31))))
        );
    }

    #[test]
    fn half_open_range_open_end() {
        let r = open_zeitraum().as_half_open_range();
        assert_eq!(r, Some((date!(2025 - 01 - 01), None)));
    }

    #[test]
    fn half_open_range_no_start_returns_none() {
        assert_eq!(no_start_zeitraum().as_half_open_range(), None);
    }

    // ── Rechnung::billing_period ─────────────────────────────────────────────

    #[test]
    fn billing_period_with_full_period() {
        let r = Rechnung {
            rechnungsperiode: Some(closed_zeitraum()),
            ..Default::default()
        };
        assert_eq!(
            r.billing_period(),
            Some((date!(2025 - 01 - 01), date!(2025 - 12 - 31)))
        );
    }

    #[test]
    fn billing_period_no_periode_returns_none() {
        let r = Rechnung {
            rechnungsperiode: None,
            ..Default::default()
        };
        assert_eq!(r.billing_period(), None);
    }

    #[test]
    fn billing_period_open_ended_returns_none() {
        // open-ended rechnungsperiode → billing_period() returns None (no closed range)
        let r = Rechnung {
            rechnungsperiode: Some(open_zeitraum()),
            ..Default::default()
        };
        assert_eq!(r.billing_period(), None);
    }

    // ── PreisblattNetznutzung::validity ──────────────────────────────────────

    #[test]
    fn validity_closed_range() {
        let p = PreisblattNetznutzung {
            gueltigkeit: Some(closed_zeitraum()),
            ..Default::default()
        };
        assert_eq!(
            p.validity(),
            Some((date!(2025 - 01 - 01), Some(date!(2025 - 12 - 31))))
        );
    }

    #[test]
    fn validity_open_ended() {
        let p = PreisblattNetznutzung {
            gueltigkeit: Some(open_zeitraum()),
            ..Default::default()
        };
        assert_eq!(p.validity(), Some((date!(2025 - 01 - 01), None)));
    }

    #[test]
    fn validity_no_gueltigkeit_returns_none() {
        let p = PreisblattNetznutzung {
            gueltigkeit: None,
            ..Default::default()
        };
        assert_eq!(p.validity(), None);
    }
}
