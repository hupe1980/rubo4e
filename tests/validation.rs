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
    use rubo4e::v202501::{Adresse, Geokoordinaten, Katasteradresse, Marktlokation};

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
    use rubo4e::v202501::{Adresse, Geokoordinaten, Messlokation};

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
    use rubo4e::v202501::Vertrag;
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
    use rubo4e::v202501::{Betrag, Rechnung};
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
            zu_zahlen: betrag(dec("119.00")),
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
            zu_zahlen: betrag(dec("120.00")),
            ..Default::default()
        };
        assert!(r.validate().is_err());
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

    #[test]
    fn invoice_with_prepayment_ok() {
        let r = Rechnung {
            gesamtnetto: betrag(dec("100.00")),
            gesamtsteuer: betrag(dec("19.00")),
            gesamtbrutto: betrag(dec("119.00")),
            vorausgezahlt: betrag(dec("20.00")),
            zu_zahlen: betrag(dec("99.00")),
            ..Default::default()
        };
        assert!(r.validate().is_ok());
    }
}

#[cfg(all(feature = "validate", feature = "versioned"))]
mod report_errors_tests {
    use garde::Validate as _;
    use rubo4e::v202501::Marktlokation;
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
        use rubo4e::v202501::Adresse;
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
