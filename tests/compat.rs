//! Cross-implementation compatibility tests.
//!
//! These tests verify that JSON payloads conforming to the BO4E standard can be
//! deserialized by rubo4e regardless of which BO4E implementation produced them.
//!
//! The payloads in `tests/compat/python/` represent the JSON format emitted by
//! the reference Python implementation (`bo4e/BO4E-python`).
//!
//! The payloads in `tests/compat/go/` represent the JSON format emitted by the
//! Go implementation (`Hochfrequenz/go-bo4e`).
//!
//! See `tests/compat/README.md` for how to regenerate these vectors.

/// The committed schema snapshot for the `current` series, found rather than
/// hard-coded so a BO4E patch release inside the series does not have to be
/// chased through the test suite.
#[cfg(all(feature = "json", feature = "versioned"))]
fn pinned_schema_dir() -> std::path::PathBuf {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("generator/schemas");
    let mut matches: Vec<std::path::PathBuf> = std::fs::read_dir(&root)
        .unwrap_or_else(|e| panic!("reading {}: {e}", root.display()))
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| {
            p.is_dir()
                && p.file_name()
                    .and_then(|n| n.to_str())
                    .is_some_and(|n| n.starts_with("v202607"))
        })
        .collect();
    matches.sort();
    assert_eq!(
        matches.len(),
        1,
        "expected exactly one committed v202607 schema snapshot, found {matches:?}"
    );
    matches.pop().expect("checked len")
}

#[cfg(all(feature = "json", feature = "versioned"))]
mod compat_tests {
    use rubo4e::v202607::{Marktlokation, Messlokation, Rechnung, Vertrag};

    // ── Python vectors ────────────────────────────────────────────────────────

    #[test]
    fn python_marktlokation_deserializes() {
        let json = include_str!("compat/python/marktlokation.json");
        let malo: Marktlokation =
            serde_json::from_str(json).expect("python/marktlokation.json must deserialize");
        assert_eq!(
            malo.marktlokations_id.as_ref().map(|id| id.as_ref()),
            Some("51238696781")
        );
        assert!(malo.sparte.is_some(), "sparte must be present");
        assert!(
            malo.energierichtung.is_some(),
            "energierichtung must be present"
        );
    }

    #[test]
    fn python_messlokation_deserializes() {
        let json = include_str!("compat/python/messlokation.json");
        let melo: Messlokation =
            serde_json::from_str(json).expect("python/messlokation.json must deserialize");
        assert_eq!(
            melo.messlokations_id.as_ref().map(|id| id.as_ref()),
            Some("DE0000000000000000000000000000001")
        );
        assert!(melo.sparte.is_some(), "sparte must be present");
    }

    #[test]
    fn python_vertrag_deserializes() {
        let json = include_str!("compat/python/vertrag.json");
        let v: Vertrag = serde_json::from_str(json).expect("python/vertrag.json must deserialize");
        assert_eq!(v.vertragsnummer.as_deref(), Some("V-2025-001"));
        assert!(v.sparte.is_some(), "sparte must be present");
        assert!(v.vertragsart.is_some(), "vertragsart must be present");
    }

    /// The Python vector carries `rechnungsdatum` / `faelligkeitsdatum` in the
    /// `format: date-time` spelling the schema declares, so a Rust type narrower
    /// than that fails here rather than in production.
    #[test]
    fn python_rechnung_deserializes() {
        let json = include_str!("compat/python/rechnung.json");
        let r: Rechnung =
            serde_json::from_str(json).expect("python/rechnung.json must deserialize");
        assert_eq!(r.rechnungsnummer.as_deref(), Some("R-2025-0042"));
        assert!(r.gesamtbrutto.is_some(), "gesamtbrutto must be present");
        assert!(r.gesamtnetto.is_some(), "gesamtnetto must be present");
        assert!(r.gesamtsteuer.is_some(), "gesamtsteuer must be present");
        assert!(
            r.rechnungsdatum.is_some(),
            "rechnungsdatum must parse from its schema-declared date-time form"
        );
        assert!(
            r.faelligkeitsdatum.is_some(),
            "faelligkeitsdatum must parse"
        );
        assert!(
            r.rechnungsperiode.is_some(),
            "rechnungsperiode (date-only Zeitraum) must parse"
        );
    }

    // ── Go vectors ────────────────────────────────────────────────────────────

    #[test]
    fn go_marktlokation_deserializes() {
        let json = include_str!("compat/go/marktlokation.json");
        let malo: Marktlokation =
            serde_json::from_str(json).expect("go/marktlokation.json must deserialize");
        assert_eq!(
            malo.marktlokations_id.as_ref().map(|id| id.as_ref()),
            Some("51238696781")
        );
        assert!(malo.sparte.is_some(), "sparte must be present");
        assert!(
            malo.energierichtung.is_some(),
            "energierichtung must be present"
        );
    }

    #[test]
    fn go_messlokation_deserializes() {
        let json = include_str!("compat/go/messlokation.json");
        let melo: Messlokation =
            serde_json::from_str(json).expect("go/messlokation.json must deserialize");
        assert_eq!(
            melo.messlokations_id.as_ref().map(|id| id.as_ref()),
            Some("DE0000000000000000000000000000002")
        );
        assert!(melo.sparte.is_some(), "sparte must be present");
    }

    #[test]
    fn go_vertrag_deserializes() {
        let json = include_str!("compat/go/vertrag.json");
        let v: Vertrag = serde_json::from_str(json).expect("go/vertrag.json must deserialize");
        assert_eq!(v.vertragsnummer.as_deref(), Some("VT-2025-9999"));
        assert!(v.sparte.is_some(), "sparte must be present");
        assert!(v.vertragsart.is_some(), "vertragsart must be present");
    }

    /// The Go vector uses the `Z` offset spelling and encodes decimals as JSON
    /// numbers rather than strings — both are valid BO4E, and both must parse.
    #[test]
    fn go_rechnung_deserializes() {
        let json = include_str!("compat/go/rechnung.json");
        let r: Rechnung = serde_json::from_str(json).expect("go/rechnung.json must deserialize");
        assert_eq!(r.rechnungsnummer.as_deref(), Some("INV-2025-007"));
        assert!(r.gesamtbrutto.is_some(), "gesamtbrutto must be present");
        assert!(r.gesamtnetto.is_some(), "gesamtnetto must be present");
        assert!(r.gesamtsteuer.is_some(), "gesamtsteuer must be present");
        assert!(r.rechnungsdatum.is_some(), "rechnungsdatum must parse");
        assert!(
            r.faelligkeitsdatum.is_some(),
            "faelligkeitsdatum must parse"
        );
    }
}

/// Fields BO4E declares as a plain string must stay `String`, whatever their
/// name resembles.
///
/// A wrong type here is never local to the field: a `Kontaktweg` whose
/// `kontaktwert` will not parse takes the whole `Geschaeftspartner` with it —
/// name, address, and VAT ID included.
#[cfg(all(feature = "json", feature = "versioned"))]
mod schema_string_fields {
    use rubo4e::current::{
        Geschaeftspartner, Kontaktweg, MarktgebietInfo, StandorteigenschaftenStrom,
    };

    /// `Kontaktweg.kontaktwert` is *"Die Nummer oder E-Mail-Adresse"*.
    #[test]
    fn contact_value_accepts_an_email_address() {
        let kw: Kontaktweg =
            serde_json::from_str(r#"{"kontaktart":"E_MAIL","kontaktwert":"info@example.de"}"#)
                .expect("an e-mail address is a valid kontaktwert");
        assert_eq!(kw.kontaktwert.as_deref(), Some("info@example.de"));
    }

    /// The enclosing object is what actually breaks, so assert on that.
    #[test]
    fn geschaeftspartner_with_a_contact_method_deserializes_whole() {
        let body = r#"{"_typ":"GESCHAEFTSPARTNER",
            "organisationsname":"Stadtwerke Musterstadt GmbH",
            "umsatzsteuerId":"DE123456789",
            "kontaktwege":[
                {"kontaktart":"E_MAIL","kontaktwert":"info@example.de"},
                {"kontaktart":"FAX","kontaktwert":"+49 30 1234567-89"}
            ]}"#;
        let gp: Geschaeftspartner = serde_json::from_str(body).expect("must deserialize");
        assert_eq!(
            gp.organisationsname.as_deref(),
            Some("Stadtwerke Musterstadt GmbH")
        );
        assert_eq!(gp.umsatzsteuer_id.as_deref(), Some("DE123456789"));
        assert_eq!(gp.kontaktwege.as_deref().map(<[_]>::len), Some(2));
    }

    /// `MarktgebietInfo.marktgebiet` is *"Der Name des Marktgebietes"* — a name.
    /// `Marktlokation.marktgebiet`, which shares the name, is *"Code vom EIC"*.
    #[test]
    fn market_area_name_is_not_an_eic_code() {
        let info: MarktgebietInfo = serde_json::from_str(r#"{"marktgebiet":"NetConnect Germany"}"#)
            .expect("a market-area name is not an EIC code");
        assert_eq!(info.marktgebiet.as_deref(), Some("NetConnect Germany"));
    }

    /// `StandorteigenschaftenStrom.regelzone` is *"Der Name der Regelzone"*; the
    /// EIC lives in the neighbouring `regelzoneEic`.
    #[test]
    fn control_zone_name_is_not_an_eic_code() {
        let props: StandorteigenschaftenStrom =
            serde_json::from_str(r#"{"regelzone":"TenneT TSO GmbH"}"#)
                .expect("a control-zone name is not an EIC code");
        assert_eq!(props.regelzone.as_deref(), Some("TenneT TSO GmbH"));
    }

    /// The code half of a name/code pair is validated, the name half is not.
    #[test]
    fn control_zone_name_and_code_are_typed_separately() {
        let props: StandorteigenschaftenStrom = serde_json::from_str(
            r#"{"regelzone":"TenneT TSO GmbH","regelzoneEic":"10YDE-EON------1"}"#,
        )
        .expect("a name in the name field and an EIC in the EIC field");
        assert_eq!(props.regelzone.as_deref(), Some("TenneT TSO GmbH"));
        assert_eq!(
            props.regelzone_eic.as_ref().map(|e| e.as_ref()),
            Some("10YDE-EON------1")
        );
        assert!(
            serde_json::from_str::<StandorteigenschaftenStrom>(
                r#"{"regelzoneEic":"TenneT TSO GmbH"}"#
            )
            .is_err(),
            "the EIC field is documented as an EIC and must validate"
        );
    }

    /// The fields that genuinely do carry an EIC keep their validated type.
    #[test]
    fn marktlokation_eic_fields_stay_typed() {
        use rubo4e::current::Marktlokation;
        let malo: Marktlokation =
            serde_json::from_str(r#"{"_typ":"MARKTLOKATION","regelzone":"10YDE-EON------1"}"#)
                .expect("a real EIC must parse");
        assert_eq!(
            malo.regelzone.as_ref().map(|e| e.as_ref()),
            Some("10YDE-EON------1")
        );
        assert!(
            serde_json::from_str::<Marktlokation>(
                r#"{"_typ":"MARKTLOKATION","regelzone":"TenneT TSO GmbH"}"#
            )
            .is_err(),
            "this field is documented as an EIC code and must still validate"
        );
    }
}

/// A decimal arrives as a JSON string or a JSON number depending on which BO4E
/// implementation wrote the payload. Both must parse in **every** feature
/// configuration — with `decimal` the field is a `rust_decimal::Decimal`, without
/// it a `String` that keeps the lexical form.
#[cfg(all(feature = "json", feature = "versioned"))]
mod decimal_spelling_tests {
    use rubo4e::current::Betrag;

    fn wert_of(json: &str) -> String {
        let b: Betrag = serde_json::from_str(json).expect("must deserialize");
        let wert = b.wert.expect("wert present");
        wert.to_string()
    }

    #[test]
    fn string_spelling_parses() {
        assert_eq!(wert_of(r#"{"wert":"119.00","waehrung":"EUR"}"#), "119.00");
    }

    #[test]
    fn number_spelling_parses() {
        // `119` and `119.5` cover the integer and float tokenizer paths.
        assert_eq!(wert_of(r#"{"wert":119,"waehrung":"EUR"}"#), "119");
        assert_eq!(wert_of(r#"{"wert":119.5,"waehrung":"EUR"}"#), "119.5");
    }

    /// Whichever spelling came in, the value goes back out as a string — the
    /// form BO4E-python emits.
    #[test]
    fn both_spellings_reserialize_as_strings() {
        use rubo4e::json::Bo4eJsonExt;
        for input in [r#"{"wert":"119.5"}"#, r#"{"wert":119.5}"#] {
            let b: Betrag = serde_json::from_str(input).expect("must deserialize");
            let out = b.to_json_german().expect("serialize");
            assert!(
                out.contains(r#""wert":"119.5""#),
                "unexpected output: {out}"
            );
        }
    }
}

/// Outbound direction: JSON that rubo4e *produces* must carry the same metadata
/// every other BO4E implementation emits.
///
/// The vectors above only prove rubo4e can read other implementations' output.
/// That leaves the opposite direction untested, which is how `_version` came to
/// be omitted from every freshly constructed value: the golden corpus round-trips
/// existing payloads, so it carried `_version` in from the input and never
/// exercised construction.
#[cfg(all(feature = "json", feature = "versioned"))]
mod outbound_tests {
    use rubo4e::current::{Betrag, Marktlokation, Rechnung, Vertrag};
    use rubo4e::json::Bo4eJsonExt;
    use rubo4e::Bo4eObject as _;

    /// Reference implementations stamp `_version` on every BO, and rubo4e knows
    /// the value statically — a caller should never have to supply it.
    #[test]
    fn constructed_bo_carries_typ_and_version() {
        let v = Vertrag::default();
        let json = v.to_json_german().expect("serialize");
        assert!(json.contains(r#""_typ":"VERTRAG""#), "missing _typ: {json}");
        assert!(
            json.contains(&format!(r#""_version":"{}""#, v.schema_version())),
            "missing _version: {json}"
        );
        // The series is what dispatch keys on and what this module promises;
        // the patch inside it moves with every BO4E release.
        assert_eq!(v.schema_series(), "202607");
        assert!(
            v.schema_version().starts_with("202607."),
            "schema_version {:?} left the series schema_series reports",
            v.schema_version()
        );
    }

    /// COMs carry `_typ` too.  Every BO4E COM schema pins `_typ` with a `const`,
    /// so pydantic stamps it on every component the reference implementation
    /// emits — a Rust-built `Betrag` that omits it is distinguishable from one
    /// produced anywhere else.
    #[test]
    fn constructed_com_carries_typ_and_version() {
        let json = Betrag::default().to_json_german().expect("serialize");
        assert!(json.contains(r#""_typ":"BETRAG""#), "missing _typ: {json}");
        assert!(
            json.contains(&format!(
                r#""_version":"{}""#,
                Marktlokation::default().schema_version()
            )),
            "missing _version: {json}"
        );
    }

    /// The `_version` wire value is the BO4E release **without** the `v` its git
    /// tag carries; read from the schema rather than hard-coded, so it stays true
    /// across releases.
    #[test]
    fn version_matches_the_schema_declared_default() {
        let declared =
            std::fs::read_to_string(super::pinned_schema_dir().join("bo/Marktlokation.json"))
                .expect("schema snapshot is committed");
        let schema: serde_json::Value = serde_json::from_str(&declared).expect("valid schema");
        let expected = schema["properties"]["_version"]["default"]
            .as_str()
            .expect("schema declares a _version default");

        assert_eq!(Marktlokation::default().version.as_deref(), Some(expected));
        assert_eq!(Marktlokation::default().schema_version(), expected);
        assert!(
            !expected.starts_with('v'),
            "BO4E declares _version without the tag's `v` prefix; got {expected:?}"
        );
    }

    /// Every generated `_typ` default must equal the `const` its schema pins,
    /// for BOs and COMs alike.  A discriminant derived from the type name instead
    /// would silently drift the day BO4E names one differently.
    #[test]
    fn typ_defaults_match_the_schema_constants() {
        use rubo4e::current::{Adresse, Menge, Vorauszahlung, Zeitraum};

        for (json, want) in [
            (
                Marktlokation::default().to_json_german().unwrap(),
                "MARKTLOKATION",
            ),
            (Rechnung::default().to_json_german().unwrap(), "RECHNUNG"),
            (Vertrag::default().to_json_german().unwrap(), "VERTRAG"),
            (Adresse::default().to_json_german().unwrap(), "ADRESSE"),
            (Menge::default().to_json_german().unwrap(), "MENGE"),
            (Zeitraum::default().to_json_german().unwrap(), "ZEITRAUM"),
            (
                Vorauszahlung::default().to_json_german().unwrap(),
                "VORAUSZAHLUNG",
            ),
        ] {
            assert!(
                json.contains(&format!(r#""_typ":"{want}""#)),
                "expected _typ {want}: {json}"
            );
        }
    }

    /// `_version` must reach nested COMs too, not just the root object.
    #[test]
    fn nested_com_carries_version() {
        let r = Rechnung {
            gesamtnetto: Some(Betrag::default()),
            ..Default::default()
        };
        let value: serde_json::Value =
            serde_json::from_str(&r.to_json_german().expect("serialize")).expect("valid json");
        assert!(
            value["gesamtnetto"]["_version"].is_string(),
            "nested COM lost _version: {value}"
        );
    }

    /// A payload that arrived stamped with a different series keeps that stamp:
    /// `_version` records the provenance of the data, and deserialization must
    /// not overwrite it with the version this crate was generated from.
    #[test]
    fn deserialized_version_is_not_overwritten() {
        let body =
            r#"{"_typ":"MARKTLOKATION","_version":"202501.0.0","marktlokationsId":"51238696781"}"#;
        let malo: Marktlokation = serde_json::from_str(body).expect("deserialize");
        assert_eq!(malo.version.as_deref(), Some("202501.0.0"));
        assert!(malo
            .to_json_german()
            .expect("serialize")
            .contains(r#""_version":"202501.0.0""#));
    }

    /// An inbound payload without `_version` must not gain one: absence is
    /// information, and `Default` must not leak into the deserialization path.
    #[test]
    fn absent_version_stays_absent() {
        let malo: Marktlokation =
            serde_json::from_str(r#"{"_typ":"MARKTLOKATION"}"#).expect("deserialize");
        assert_eq!(malo.version, None);
        assert!(!malo
            .to_json_german()
            .expect("serialize")
            .contains("_version"));
    }
}
