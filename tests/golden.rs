//! Golden JSON corpus tests (E17-S58).
//!
//! Deserializes committed JSON payloads and re-serializes them to verify
//! wire compatibility with the BO4E standard.
//!
//! Each test asserts **structural equality** between the original `serde_json::Value`
//! and the re-serialized value tree.  This catches field-ordering changes,
//! null-emission regressions, and key-casing bugs that a simple re-parse cannot detect.
//!
//! Run with:
//! ```text
//! cargo test --test golden --features json,versioned
//! ```

#[cfg(all(feature = "json", feature = "versioned"))]
mod golden_tests {
    use rubo4e::v202607::{
        Lastgang, Marktlokation, Messlokation, Netzlokation, Rechnung, Vertrag, Zeitreihe,
    };

    macro_rules! golden_roundtrip {
        ($name:ident, $ty:ty, $file:literal) => {
            #[test]
            fn $name() {
                let json = include_str!(concat!("golden/", $file));
                // Parse into canonical Value tree first.
                let original: serde_json::Value = serde_json::from_str(json).expect(concat!(
                    "failed to parse ",
                    $file,
                    " into Value"
                ));
                // Deserialize into the typed struct.
                let typed: $ty = serde_json::from_value(original.clone()).expect(concat!(
                    "failed to deserialize ",
                    $file,
                    " into typed struct"
                ));
                // Re-serialize then re-parse back to a Value tree.
                let reserialized: serde_json::Value = serde_json::from_str(
                    &serde_json::to_string(&typed).expect(concat!("failed to serialize ", $file)),
                )
                .expect(concat!("failed to re-parse serialized ", $file));
                // Structural equality, to catch field-ordering,
                // null-emission, and key-casing regressions.
                assert_eq!(
                    original, reserialized,
                    "round-trip produced different JSON value tree for {}",
                    $file
                );
            }
        };
    }

    golden_roundtrip!(vertrag_minimal, Vertrag, "vertrag_minimal.json");
    golden_roundtrip!(vertrag_typical, Vertrag, "vertrag_typical.json");
    golden_roundtrip!(
        marktlokation_minimal,
        Marktlokation,
        "marktlokation_minimal.json"
    );
    golden_roundtrip!(
        marktlokation_typical,
        Marktlokation,
        "marktlokation_typical.json"
    );
    golden_roundtrip!(
        messlokation_minimal,
        Messlokation,
        "messlokation_minimal.json"
    );
    golden_roundtrip!(
        messlokation_typical,
        Messlokation,
        "messlokation_typical.json"
    );
    golden_roundtrip!(rechnung_minimal, Rechnung, "rechnung_minimal.json");
    golden_roundtrip!(rechnung_typical, Rechnung, "rechnung_typical.json");
    golden_roundtrip!(
        netzlokation_minimal,
        Netzlokation,
        "netzlokation_minimal.json"
    );
    golden_roundtrip!(
        netzlokation_typical,
        Netzlokation,
        "netzlokation_typical.json"
    );
    golden_roundtrip!(lastgang_minimal, Lastgang, "lastgang_minimal.json");
    golden_roundtrip!(lastgang_typical, Lastgang, "lastgang_typical.json");
    golden_roundtrip!(zeitreihe_minimal, Zeitreihe, "zeitreihe_minimal.json");
    golden_roundtrip!(zeitreihe_typical, Zeitreihe, "zeitreihe_typical.json");

    /// The golden `Lastgang` is a well-formed quarter-hourly profile, so the
    /// coverage audit must find nothing — which pins the fixture as the
    /// reference shape a producer should emit, not just as something that
    /// round-trips.
    #[cfg(feature = "time")]
    #[test]
    fn lastgang_typical_is_a_clean_hour() {
        use rubo4e::timeseries::Bo4eTimeSeries;

        let json = include_str!("golden/lastgang_typical.json");
        let lg: Lastgang = serde_json::from_str(json).unwrap();

        let report = lg.audit();
        assert!(report.is_usable(), "{report:?}");
        assert_eq!(report.covered, time::Duration::HOUR);
        assert_eq!(report.coverage_ratio(), Some(1.0));

        #[cfg(feature = "decimal")]
        {
            assert_eq!(lg.expected_interval(), Some(time::Duration::minutes(15)));
            // 400 + 480 + 512 + 408 kW, each held a quarter-hour → 450 kWh.
            assert_eq!(lg.integrate(), Some(rust_decimal::Decimal::from(450)));
            assert_eq!(lg.sum(), None, "KW is not an extensive quantity");
        }
    }

    /// The golden `Zeitreihe` states KWH, so the aggregate flips: those values
    /// are additive and `sum()` is the meaningful one.
    #[cfg(all(feature = "time", feature = "decimal"))]
    #[test]
    fn zeitreihe_typical_sums_its_energies() {
        use rubo4e::timeseries::Bo4eTimeSeries;

        let json = include_str!("golden/zeitreihe_typical.json");
        let zr: Zeitreihe = serde_json::from_str(json).unwrap();

        assert!(zr.audit().is_usable());
        assert_eq!(zr.sum(), Some(rust_decimal::Decimal::from(450)));
        assert_eq!(zr.expected_interval(), None, "a Zeitreihe declares none");
    }

    /// Verify specific field values survive deserialization for Marktlokation.
    #[test]
    fn marktlokation_typical_field_values() {
        let json = include_str!("golden/marktlokation_typical.json");
        let m: Marktlokation = serde_json::from_str(json).unwrap();
        assert_eq!(
            m.marktlokations_id.as_ref().map(|id| id.as_ref()),
            Some("51238696781"),
            "marktlokationsId field"
        );
        assert!(
            m.lokationsadresse.is_some(),
            "lokationsadresse should be Some"
        );
        let addr = m.lokationsadresse.unwrap();
        assert_eq!(addr.ort.as_deref(), Some("Berlin"));
        assert_eq!(addr.postleitzahl.as_deref(), Some("10115"));
    }

    /// Verify specific field values survive deserialization for Vertrag.
    #[test]
    fn vertrag_typical_field_values() {
        let json = include_str!("golden/vertrag_typical.json");
        let v: Vertrag = serde_json::from_str(json).unwrap();
        assert_eq!(v.vertragsnummer.as_deref(), Some("V-2025-001"));
        assert_eq!(
            v.beschreibung.as_deref(),
            Some("Strom-Liefervertrag Haushalt")
        );
    }

    /// Verify specific field values for Rechnung.
    #[test]
    fn rechnung_typical_field_values() {
        let json = include_str!("golden/rechnung_typical.json");
        let r: Rechnung = serde_json::from_str(json).unwrap();
        assert_eq!(r.rechnungsnummer.as_deref(), Some("RE-2025-00042"));
        assert_eq!(
            r.rechnungstitel.as_deref(),
            Some("Abschlagsrechnung Januar 2025")
        );
        assert!(r.gesamtbrutto.is_some(), "gesamtbrutto should be parsed");
    }
}
