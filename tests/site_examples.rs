//! Compiles and runs the examples shown on the landing page and in the README.
//!
//! Those snippets live in Markdown, so nothing else type-checks them. Keeping an
//! executable copy here means an API change breaks the build instead of quietly
//! leaving the published documentation wrong.
//!
//! Sources:
//! - `site/snippets/quickstart.md` — the landing page round-trip
//! - `README.md` § Quick Start — the builder walkthrough
//! - `README.md` § A decode does **not** validate field names
#![cfg(all(
    feature = "versioned",
    feature = "json",
    feature = "builder",
    feature = "validate"
))]

use rubo4e::current::{Marktlokation, Sparte, Vertrag};
use rubo4e::json::{Bo4eJsonExt, JsonParseLimits};
use rubo4e::prelude::*;
use rubo4e::Bo4eStrict;

/// `site/snippets/quickstart.md`
#[test]
fn landing_page_round_trip() -> Result<(), Box<dyn std::error::Error>> {
    let body = r#"{"_typ":"MARKTLOKATION","marktlokationsId":"51238696781","sparte":"STROM"}"#;

    let malo =
        Marktlokation::from_json_german_hardened(body, JsonParseLimits::untrusted_defaults())?;

    malo.ensure_known_enums()?;

    let id = malo.marktlokations_id.as_ref().expect("id present");
    assert_eq!(id.as_ref(), "51238696781");
    assert_eq!(malo.sparte, Some(Sparte::Strom));
    Ok(())
}

/// The same snippet must actually reject an out-of-schema enum value — otherwise
/// the `ensure_known_enums()` line above is decorative.
#[test]
fn landing_page_round_trip_rejects_unknown_enum() {
    let body = r#"{"_typ":"MARKTLOKATION","marktlokationsId":"51238696781","sparte":"PLASMA"}"#;
    let malo =
        Marktlokation::from_json_german_hardened(body, JsonParseLimits::untrusted_defaults())
            .expect("lenient decode succeeds");
    let err = malo
        .ensure_known_enums()
        .expect_err("PLASMA is not a Sparte in this schema version");
    assert_eq!(err.paths, ["sparte"]);
}

/// `README.md` § Quick Start
#[test]
fn readme_quick_start() -> Result<(), Box<dyn std::error::Error>> {
    let vertrag = Vertrag::builder()
        .sparte(Sparte::Strom)
        .beschreibung("Jahresvertrag Strom".to_string())
        .vertragsnummer("VN-2026-001".to_string())
        .build();

    // Reached through the prelude — the README states no direct garde dependency
    // is needed, so this import path is part of the contract.
    vertrag.validate()?;

    let json = vertrag.to_json_german()?;
    assert!(
        json.contains(r#""sparte":"STROM""#),
        "unexpected JSON: {json}"
    );
    Ok(())
}

// ─── Identifier snippets ─────────────────────────────────────────────────────
//
// `README.md` § Identifiers and `site/content/docs/identifiers.md` show these
// verbatim. They document the ENTSO-E object-type distinction and the OBIS
// canonical form, both of which were wrong before — so an executable copy is
// what keeps the published text honest.

/// `README.md` § EIC codes and object types, and the site's
/// "Object type (position 3)" and "BilanzkreisId and BilanzierungsgebietId"
/// sections.
#[test]
fn docs_eic_object_types() -> Result<(), Box<dyn std::error::Error>> {
    use rubo4e::identifiers::{BilanzierungsgebietId, BilanzkreisId, EicCode, EicType};

    let area = EicCode::new("10YDE-EON------1")?; // TenneT control area
    assert_eq!(area.type_char(), 'Y');
    assert_eq!(area.eic_type(), EicType::Area);

    let party = EicCode::new("11XSUEDWESTSTRO8")?; // a Bilanzkreis
    assert_eq!(party.eic_type(), EicType::Party);

    assert_eq!(EicCode::compute_check_char("10XDE-EON-NETZ-"), Some('C'));
    assert_eq!(
        EicCode::new_from_prefix("10YDE-EON------")?.as_ref(),
        "10YDE-EON------1"
    );

    // The restricted types pin position 3, so the roles cannot be confused.
    let bk = BilanzkreisId::from_prefix("11XSUEDWESTSTRO")?;
    assert_eq!(bk.as_ref(), "11XSUEDWESTSTRO8");
    assert_eq!(bk.to_eic_code().eic_type(), EicType::Party);

    let eic: EicCode = bk.clone().into();
    assert_eq!(BilanzkreisId::try_from(eic)?, bk);

    let bg = BilanzierungsgebietId::new("11YN-0000-0001-Q")?;
    assert_eq!(bg.to_eic_code().eic_type(), EicType::Area);

    assert!(BilanzkreisId::new("11YN-0000-0001-Q").is_err());
    assert!(BilanzierungsgebietId::new("11XSUEDWESTSTRO8").is_err());
    assert!(BilanzkreisId::new("10YDE-EON------1").is_err());
    Ok(())
}

/// `README.md` § OBIS codes and the site's "Canonical form" section.
#[test]
fn docs_obis_canonical_form() -> Result<(), Box<dyn std::error::Error>> {
    use rubo4e::identifiers::ObisCode;

    ObisCode::new("1-0:1.8.1")?; // electricity, active energy forward, tariff 1
    ObisCode::new("7-0:3.1.0")?; // gas, volume
    ObisCode::new("0-0:0.0.0")?; // C=0 — general metering group
    assert!(ObisCode::new("not-an-obis").is_err());

    // Canonicalisation
    assert_eq!(ObisCode::new("1.8.1&255")?, ObisCode::new("1.8.1*255")?);
    assert_eq!(
        ObisCode::new("01-00:01.08.00")?,
        ObisCode::new("1-0:1.8.0")?
    );
    assert_eq!(ObisCode::new("01-00:01.08.00")?.as_str(), "1-0:1.8.0");

    // Value groups are octets
    assert!(ObisCode::new("1-0:1.8.256").is_err());

    // Stored components
    let parts = ObisCode::new("1-0:1.8.0*255")?.components();
    assert_eq!(
        (parts.a, parts.b, parts.c, parts.d, parts.e, parts.f),
        (Some(1), Some(0), 1, 8, Some(0), Some(255))
    );

    assert_eq!(ObisCode::new("1-0:1.8.0*255")?.to_pia_string(), "1-0:1.8.0");
    Ok(())
}

// ─── Claims added in this release ────────────────────────────────────────────

/// `README.md` § Schema Versions and the site's "Multi-version Dispatch".
///
/// The docs tell readers to key dispatch on the **series** and warn that keying
/// on the full `_version` breaks the day a sender is one BO4E patch ahead. Both
/// halves are asserted here so the advice cannot rot.
#[test]
fn docs_dispatch_on_the_series_not_the_release() {
    use rubo4e::Bo4eTyped as _;

    fn series_of(wire_version: &str) -> &str {
        wire_version.split('.').next().unwrap_or(wire_version)
    }

    let malo = Marktlokation::default();
    assert_eq!(malo.schema_series(), "202607");
    assert_eq!(series_of(malo.schema_version()), malo.schema_series());
    assert!(
        !malo.schema_version().starts_with('v'),
        "the wire form has no `v`"
    );

    // A sender one patch ahead still lands in this arm…
    assert_eq!(series_of("202607.4.0"), malo.schema_series());
    // …and would not have, matched on the full triple.
    assert_ne!("202607.4.0", malo.schema_version());
    // A different series does not.
    assert_ne!(series_of("202801.0.0"), malo.schema_series());
}

/// `README.md` § Convenience API and the site's ecosystem / validation pages:
/// BO4E declares **both** `Zeitraum` dates inclusive.
#[test]
#[cfg(feature = "time")]
fn docs_zeitraum_dates_are_both_inclusive() {
    use rubo4e::current::Zeitraum;
    use time::macros::date;

    let january = Zeitraum {
        startdatum: Some(date!(2026 - 01 - 01)),
        enddatum: Some(date!(2026 - 01 - 31)),
        ..Default::default()
    };

    assert!(
        january.contains(date!(2026 - 01 - 01)),
        "the start is inside"
    );
    assert!(january.contains(date!(2026 - 01 - 31)), "the end is inside");
    assert!(!january.contains(date!(2026 - 02 - 01)));
    assert_eq!(january.whole_days(), Some(31), "both bounds count");

    let range = january.as_inclusive_range().expect("both dates present");
    assert!(range.contains(&date!(2026 - 01 - 31)));

    // An absent boundary is unbounded, not empty.
    let open = Zeitraum {
        startdatum: Some(date!(2026 - 01 - 01)),
        ..Default::default()
    };
    assert!(open.contains(date!(2099 - 12 - 31)));
    assert!(
        open.as_inclusive_range().is_none(),
        "one bound is not a range"
    );
    assert_eq!(open.whole_days(), None);
    assert_eq!(open.bounds(), (Some(date!(2026 - 01 - 01)), None));
}

/// The site's "Hardened Deserialization" section builds limits with `with_*`
/// rather than a struct literal, because `JsonParseLimits` is `#[non_exhaustive]`.
#[test]
fn docs_json_parse_limits_are_built_not_declared() {
    let strict = JsonParseLimits::untrusted_defaults()
        .with_max_payload_bytes(Some(64 * 1024))
        .with_max_extension_field_count(Some(0));

    assert_eq!(strict.max_payload_bytes, Some(64 * 1024));
    assert_eq!(strict.max_extension_field_count, Some(0));
    assert_eq!(
        strict.max_nesting_depth,
        Some(64),
        "the profile's rest is kept"
    );

    assert_eq!(JsonParseLimits::default(), JsonParseLimits::unlimited());

    // `max_extension_field_count: Some(0)` really does reject any unknown field.
    let body = r#"{"_typ":"MARKTLOKATION","marktlokationsId":"51238696781","vendor":1}"#;
    assert!(Marktlokation::from_json_german_hardened(body, strict).is_err());

    // …and accepts the same payload without the extension field.
    let clean = r#"{"_typ":"MARKTLOKATION","marktlokationsId":"51238696781"}"#;
    assert!(Marktlokation::from_json_german_hardened(clean, strict).is_ok());
}

/// `README.md` § JSON Handling: decimals are written as strings, read from
/// either spelling, and the number path is counted because it is lossy.
#[test]
#[cfg(feature = "decimal")]
fn docs_decimal_wire_spellings() {
    use rubo4e::current::Menge;
    use rubo4e::decimal_serde::decimal_from_json_number_count;

    // Written as a string, matching BO4E-python.
    let m = Menge::from_json_german(r#"{"wert":"119.00"}"#).expect("string form");
    assert!(
        m.to_json_german()
            .expect("serialize")
            .contains(r#""wert":"119.00""#),
        "decimals must serialize as JSON strings"
    );

    // Read from a number too, the way go-bo4e writes them — and counted.
    let n = Menge::from_json_german(r#"{"wert":119.50}"#).expect("number form");
    assert!(n.wert.is_some());
    assert!(decimal_from_json_number_count() > 0);
}

/// `README.md` § IBAN and BIC, and the site's "Iban and Bic" section.
///
/// Both show the normalisation, the accessors, and the deliberate decision to
/// leave the generated fields as `String` — so all three are exercised here.
#[test]
fn docs_iban_and_bic() -> Result<(), Box<dyn std::error::Error>> {
    use rubo4e::current::Zahlungsinformation;
    use rubo4e::identifiers::{Bic, Iban};

    // Grouping spaces and lowercase normalise away; `as_ref` is the wire form.
    let iban = Iban::new("de89 3704 0044 0532 0130 00")?;
    assert_eq!(iban.as_ref(), "DE89370400440532013000");
    assert_eq!(iban.to_grouped_string(), "DE89 3704 0044 0532 0130 00");
    assert_eq!(iban.country_code(), "DE");
    assert_eq!(iban.bankleitzahl(), Some("37040044"));
    assert_eq!(iban.kontonummer(), Some("0532013000"));
    assert!(
        Iban::new("DE89370400440532013090").is_err(),
        "transposed digits must fail the MOD-97 check"
    );
    // A German IBAN is 22 characters — a 21-character one is rejected here.
    assert!(Iban::new("DE8937040044053201300").is_err());

    let bic = Bic::new("GENODEF1S04")?;
    assert_eq!((bic.institution_code(), bic.country_code()), ("GENO", "DE"));
    assert_eq!(bic.location_code(), "F1");
    assert_eq!(bic.branch_code(), Some("S04"));
    assert!(bic.is_passive());
    assert!(!bic.is_head_office());
    assert!(Bic::new("MARKDEFF")?.is_head_office(), "the 8-char form");

    // The generated field stays a `String`, and the check is on demand.
    let z = Zahlungsinformation {
        iban: Some("DE89 3704 0044 0532 0130 00".into()),
        ..Default::default()
    };
    assert_eq!(z.iban_checked().expect("stated").expect("valid"), iban);

    let masked = Zahlungsinformation {
        iban: Some("DE89 **** **** **** 3000".into()),
        ..Default::default()
    };
    assert!(
        masked.iban_checked().expect("stated").is_err(),
        "a masked IBAN fails the check…"
    );
    assert!(
        masked.iban.is_some(),
        "…but the invoice still carries the field it was written with"
    );
    assert!(Zahlungsinformation::default().iban_checked().is_none());
    Ok(())
}

/// `README.md` § Convenience API: BO4E's price-tier gap rule.
#[test]
#[cfg(feature = "decimal")]
fn docs_price_tier_gap_rule() {
    use rubo4e::convenience::PreisstaffelSliceExt;
    use rubo4e::current::Preisstaffel;
    use rust_decimal::Decimal;

    fn tier(von: i64, bis: i64, preis: i64) -> Preisstaffel {
        Preisstaffel {
            staffelgrenze_von: Some(Decimal::from(von)),
            staffelgrenze_bis: Some(Decimal::from(bis)),
            preis: Some(Decimal::from(preis)),
            ..Default::default()
        }
    }
    let staffeln = [tier(0, 1000, 30), tier(1001, 2000, 25)];
    let price = |v: Decimal| staffeln.select_for(v).and_then(|s| s.preis);

    assert_eq!(price(Decimal::from(1000)), Some(Decimal::from(30)));
    // BO4E: "Werte zwischen den Grenzen rutschen in die obere Zone / Staffel".
    assert_eq!(price(Decimal::new(10006, 1)), Some(Decimal::from(25)));
    assert!(staffeln.select_for(Decimal::from(9999)).is_none());
}

/// `README.md` § Convenience API and the site's ecosystem page: the `Rechnung`
/// and `PreisblattNetznutzung` accessor walkthroughs.
///
/// `billing_period()` returns a `RangeInclusive<Date>` and `validity()` a pair;
/// the snippets show both, and nothing else type-checks them.
#[test]
#[cfg(all(feature = "time", feature = "decimal"))]
fn docs_rechnung_and_validity_accessors() {
    use rubo4e::current::{
        Betrag, PreisblattNetznutzung, Rechnung, Rechnungsposition, Waehrungscode, Zeitraum,
    };
    use rust_decimal::Decimal;
    use time::macros::date;

    let betrag = |cents: i64| Betrag {
        wert: Some(Decimal::new(cents, 2)),
        waehrung: Some(Waehrungscode::Eur),
        ..Default::default()
    };
    let januar = Zeitraum {
        startdatum: Some(date!(2026 - 01 - 01)),
        enddatum: Some(date!(2026 - 01 - 31)),
        ..Default::default()
    };

    let rechnung = Rechnung {
        rechnungsperiode: Some(januar.clone()),
        gesamtnetto: Some(betrag(10_000)),
        gesamtsteuer: Some(betrag(1_900)),
        gesamtbrutto: Some(betrag(11_900)),
        ist_storno: Some(true),
        faelligkeitsdatum: Some(date!(2026 - 02 - 15).midnight().assume_utc()),
        rechnungspositionen: Some(vec![Rechnungsposition {
            gesamtpreis: Some(betrag(11_900)),
            lieferungszeitraum: Some(januar),
            ..Default::default()
        }]),
        ..Default::default()
    };

    // A `RangeInclusive<Date>`, so the convention travels with the value.
    let period = rechnung.billing_period().expect("both dates present");
    assert_eq!(
        (*period.start(), *period.end()),
        (date!(2026 - 01 - 01), date!(2026 - 01 - 31),)
    );
    assert!(period.contains(&date!(2026 - 01 - 31)), "the end is inside");

    assert_eq!(rechnung.period_start(), Some(date!(2026 - 01 - 01)));
    assert_eq!(rechnung.period_end(), Some(date!(2026 - 01 - 31)));
    assert_eq!(
        rechnung.faelligkeitsdatum_date(),
        Some(date!(2026 - 02 - 15))
    );

    assert_eq!(
        rechnung.gesamtnetto_decimal(),
        Some(Decimal::new(10_000, 2))
    );
    assert_eq!(
        rechnung.gesamtsteuer_decimal(),
        Some(Decimal::new(1_900, 2))
    );
    assert_eq!(
        rechnung.gesamtbrutto_decimal(),
        Some(Decimal::new(11_900, 2))
    );
    assert!(rechnung.zu_zahlen_decimal().is_none());
    assert!(rechnung.vorauszahlungen_summe().is_none(), "none stated");
    assert!(rechnung.is_storno());
    assert!(!rechnung.is_original());

    let pos = rechnung.positions().next().expect("one line item");
    assert_eq!(pos.gesamtpreis_decimal(), Some(Decimal::new(11_900, 2)));
    assert_eq!(pos.lieferung_von_date(), Some(date!(2026 - 01 - 01)));
    assert_eq!(pos.lieferung_bis_date(), Some(date!(2026 - 01 - 31)));
    assert!(pos.lieferungszeitraum_contains(date!(2026 - 01 - 31)));
    assert!(!pos.lieferungszeitraum_contains(date!(2026 - 02 - 01)));

    // `validity()` is the pair, and a missing `gueltigkeit` reads as (None, None).
    let sheet = PreisblattNetznutzung {
        gueltigkeit: Some(Zeitraum {
            startdatum: Some(date!(2026 - 01 - 01)),
            ..Default::default()
        }),
        ..Default::default()
    };
    assert_eq!(sheet.validity(), (Some(date!(2026 - 01 - 01)), None));
    assert!(sheet.is_valid_at(date!(2026 - 10 - 01)));
    assert_eq!(PreisblattNetznutzung::default().validity(), (None, None));
    assert!(
        !PreisblattNetznutzung::default().is_valid_at(date!(2026 - 10 - 01)),
        "no validity stated must read as *not* valid"
    );
}

/// `README.md` § Convenience API and the site's ecosystem page: the three
/// `Zeitraum` fields that stay `String` and are parsed on demand.
#[test]
#[cfg(feature = "time")]
fn docs_zeitraum_string_fields_parse_on_demand() {
    use rubo4e::current::Zeitraum;
    use time::macros::{offset, time};
    use time::Duration;

    let z = Zeitraum {
        dauer: Some("P1DT30H4S".into()),
        startuhrzeit: Some("18:00:00+01:00".into()),
        enduhrzeit: Some("19:00:00+01:00".into()),
        ..Default::default()
    };

    assert_eq!(
        z.duration(),
        Some(Ok(Duration::days(1)
            + Duration::hours(30)
            + Duration::seconds(4)))
    );
    assert_eq!(
        z.startuhrzeit_parsed(),
        Some(Ok((time!(18:00:00), Some(offset!(+1)))))
    );
    assert_eq!(
        z.enduhrzeit_parsed(),
        Some(Ok((time!(19:00:00), Some(offset!(+1)))))
    );

    // Absent stays distinguishable from unparsable.
    assert!(Zeitraum::default().duration().is_none());
    // A calendar component has no fixed length and is refused, not guessed.
    let calendar = Zeitraum {
        dauer: Some("P1M".into()),
        ..Default::default()
    };
    assert!(calendar.duration().expect("stated").is_err());
}

/// `README.md` § A decode does **not** validate field names.
///
/// The claim the section is built on is a negative one — that a decode round-trip
/// *cannot* fail on a renamed field — so if a future change made decoding strict,
/// the README would be wrong and nothing else would notice.
#[test]
fn readme_decode_does_not_validate_field_names() {
    use rubo4e::current::{Kosten, Kostenblock};
    use rubo4e::json::{Bo4eExtensions, JsonParseLimits};

    let body = serde_json::json!({
        "_typ": "KOSTEN",
        "kostenbloecke": [{ "kostenblockBEZEICHNUNG": "x" }]   // misspelled
    });

    // Built from a literal, so a field rename must fail here — right?
    let kosten: Kosten = serde_json::from_value(body.clone()).expect("it cannot fail");
    assert_eq!(
        kosten.kostenbloecke.as_ref().unwrap()[0].kostenblockbezeichnung,
        None
    );

    // The recursive check is the one that answers.
    assert_eq!(
        kosten.extension_paths(),
        ["kostenbloecke[0].kostenblockBEZEICHNUNG"]
    );
    assert!(kosten.ensure_no_extension_data().is_err());

    // …or make the decode itself the check.
    let closed = JsonParseLimits::unlimited().with_max_extension_field_count(Some(0));
    assert!(Kosten::from_json_value_hardened(body, closed).is_err());

    // Better still: construct it typed, where the rename is a compile error.
    let kosten = Kosten {
        kostenbloecke: Some(vec![Kostenblock {
            kostenblockbezeichnung: Some("x".into()),
            ..Default::default()
        }]),
        ..Default::default()
    };
    assert!(kosten.ensure_no_extension_data().is_ok());
}

// ─── Lokationsbündel, ZusatzAttribute, and the interval reading ──────────────
//
// `README.md` § Lokationsbündelstrukturen / § Namespaced `ZusatzAttribut`s /
// § One reading shape for all three, and the site's `lokationsbuendel.md`,
// `serialization.md` § ZusatzAttribute and namespaces, and `timeseries.md`
// § One reading shape for all three.

/// The site's Lokationsbündel page — the codelist lookups it prints verbatim.
#[test]
fn docs_lokationsbuendel_codelist() {
    use rubo4e::identifiers::Lokationsbuendelcode;
    use rubo4e::lokationsbuendel::{Lokationsbuendelstruktur, Objektfunktion};

    let code = Lokationsbuendelcode::new("9992000000026").unwrap();
    assert_eq!(code.grouped(), "9992 00000 002 6");
    assert_eq!(code.as_str(), "9992000000026");
    assert!(Lokationsbuendelcode::new("9992000000062").is_err());

    let s = Lokationsbuendelstruktur::from_code(&code).unwrap();
    assert_eq!(s.bezeichnung, "Verbrauch mit einer Messlokation (Standard)");

    let melo = s.objekt("9992000001032").unwrap();
    assert_eq!(melo.cardinality(), "1");
    assert!(melo.is_mandatory());
    assert_eq!(melo.rolle().funktion, Some(Objektfunktion::Netzuebergabe));

    let tr = s.objekt("9992000001024").unwrap();
    assert_eq!(tr.cardinality(), "0-N");
    assert!(tr.permits(0) && tr.permits(9_999));

    // The page prints all 15 structure codes; the table and the data must agree.
    assert_eq!(rubo4e::lokationsbuendel::STRUKTUREN.len(), 15);
    for printed in [
        "9992000000018",
        "9992000000026",
        "9992000000034",
        "9992000000042",
        "9992000000068",
        "9992000000076",
        "9992000000084",
        "9992000000109",
        "9992000000117",
        "9992000000125",
        "9992000000133",
        "9992000000159",
        "9992000000167",
        "9992000000175",
        "9992000000183",
    ] {
        assert!(
            Lokationsbuendelstruktur::from_wire(printed).is_some(),
            "{printed} is printed on the site but not in the codelist"
        );
    }
}

/// `README.md` § Lokationsbündelstrukturen and the site's audit example.
#[test]
fn docs_lokationsbuendel_audit() {
    use rubo4e::current::{Lokationszuordnung, Marktlokation, TechnischeRessource};
    use rubo4e::lokationsbuendel::{
        Befund, Flussrichtung, LokationsbuendelExt, LokationsbuendelObjekt, Objekttyp,
    };

    let technische_ressource = TechnischeRessource {
        lokationsbuendel_objektcode: Some("9992000001024".into()),
        ..Default::default()
    };
    let rolle = technische_ressource.objektrolle().unwrap();
    assert_eq!(rolle.objekttyp, Objekttyp::TechnischeRessource);
    assert_eq!(rolle.richtung, Some(Flussrichtung::Verbrauch));
    assert_eq!(rolle.ebene, 1);

    let zuordnung = Lokationszuordnung {
        lokationsbuendelcode: Some("9992000000026".into()),
        marktlokationen: Some(vec![Box::new(Marktlokation {
            lokationsbuendel_objektcode: Some("9992000001016".into()),
            ..Default::default()
        })]),
        ..Default::default()
    };

    let report = zuordnung.audit_buendel();
    assert!(!report.is_conformant());
    assert!(report.befunde.contains(&Befund::AnzahlVerletzt {
        code: "9992000001032".into(),
        objekttyp: Objekttyp::Messlokation,
        gefunden: 0,
        erwartet: "1".into(),
    }));

    // The view methods the docs list.
    let buendel = zuordnung.buendel();
    assert_eq!(buendel.verbrauchs_ressourcen().count(), 0);
    assert_eq!(buendel.objekte_auf_ebene(2).len(), 0);
    assert!(Lokationszuordnung::default().buendel().is_empty());
}

/// `README.md` § Namespaced `ZusatzAttribut`s and the site's serialization page.
#[test]
fn docs_namespaced_zusatz_attribute() {
    use rubo4e::current::{SteuerbareRessource, TechnischeRessource};
    use rubo4e::zusatz_attribut::{Namespace, ZusatzAttributeExt};

    #[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    enum Steuerungsvariante {
        Direktansteuerung,
        Ems,
    }

    let ski = "d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0";
    let mut sr = SteuerbareRessource::default();
    sr.set_zusatz_attribut_in(&Namespace::HEMS, "eebus-ski", ski);
    sr.set_zusatz_attribut_in(&Namespace::MAKO, "vorgangsnummer", "V-2026-0001");

    assert_eq!(
        sr.zusatz_attribut_str_in(&Namespace::HEMS, "eebus-ski"),
        Some(ski)
    );
    assert_eq!(sr.zusatz_attribut_namespaces(), ["hems", "mako"]);

    // The wire form is the flat BO4E name.
    let json = sr.to_json_german().unwrap();
    assert!(json.contains(r#""name":"hems:eebus-ski""#), "{json}");

    assert_eq!(sr.remove_zusatz_attribute_in(&Namespace::HEMS).len(), 1);

    let mut tr = TechnischeRessource::default();
    tr.set_zusatz_attribut_as_in(
        &Namespace::HEMS,
        "steuerungsvariante",
        &Steuerungsvariante::Ems,
    )
    .unwrap();
    let read: Steuerungsvariante = tr
        .zusatz_attribut_as_in(&Namespace::HEMS, "steuerungsvariante")
        .unwrap()
        .unwrap();
    assert_eq!(read, Steuerungsvariante::Ems);

    // The three registered prefixes the docs table lists.
    let registered: Vec<&str> = Namespace::REGISTERED
        .iter()
        .map(Namespace::as_str)
        .collect();
    assert_eq!(registered, ["mako", "hems", "edmd", "mabis"]);
    assert!(!Namespace::new("sap").unwrap().is_registered());
}

/// `README.md` § One reading shape for all three, and the site's timeseries page.
#[test]
#[cfg(all(feature = "time", feature = "decimal"))]
fn docs_one_reading_shape_for_all_three() {
    use rubo4e::current::{Energiemenge, Lastgang, Menge, Mengeneinheit, Zeitraum, Zeitreihe};
    use rubo4e::timeseries::{Bo4eIntervals, Bo4eTimeSeries};
    use rust_decimal::Decimal;
    use time::macros::datetime;

    let start = datetime!(2026-01-01 00:00 +01:00);
    let quarter = Menge {
        wert: Some(Decimal::from(15)),
        einheit: Some(Mengeneinheit::Minute),
        ..Default::default()
    };
    let entries = |values: [i64; 4]| {
        Some(
            values
                .into_iter()
                .enumerate()
                .map(|(i, v)| rubo4e::current::Zeitreihenwert {
                    wert: Some(Decimal::from(v)),
                    zeitraum: Some(Zeitraum::from_instants(
                        start + time::Duration::minutes(15 * i as i64),
                        start + time::Duration::minutes(15 * (i as i64 + 1)),
                    )),
                    ..Default::default()
                })
                .collect(),
        )
    };

    let lastgang = Lastgang {
        messgroesse: Some(Mengeneinheit::Kw),
        werte: entries([400, 400, 400, 400]),
        ..Lastgang::new(quarter)
    };
    let zeitreihe = Zeitreihe {
        einheit: Some(Mengeneinheit::Kwh),
        werte: entries([100, 100, 100, 100]),
        ..Default::default()
    };
    let energiemenge = Energiemenge {
        menge: Some(Menge {
            wert: Some(Decimal::from(400)),
            einheit: Some(Mengeneinheit::Kwh),
            ..Default::default()
        }),
        zeitraum: Some(Zeitraum::from_instants(
            start,
            start + time::Duration::hours(1),
        )),
        ..Default::default()
    };

    let expected = Some((Decimal::from(400), Mengeneinheit::Kwh));
    assert_eq!(lastgang.total_energy(), expected);
    assert_eq!(zeitreihe.total_energy(), expected);
    assert_eq!(energiemenge.total_energy(), expected);

    // 400 kW over a quarter-hour is 100 kWh — the per-reading step.
    let first = lastgang.intervals().next().unwrap();
    assert_eq!(
        first.energy(),
        Some((Decimal::from(100), Mengeneinheit::Kwh))
    );

    // …and back out again, into a series that audits clean against its own
    // interval length.
    let rebuilt = Zeitreihe::from_intervals(zeitreihe.intervals());
    assert_eq!(rebuilt.einheit, Some(Mengeneinheit::Kwh));
    assert!(rebuilt.audit().is_complete());
}

// ─── MaBiS & Modell 2 ────────────────────────────────────────────────────────
//
// `README.md` § Beyond the schema, and the site's `beyond-the-schema.md`.
// `tests/modell2.rs` carries the full argument; these are the snippets the
// two documents print verbatim.

/// The site's "The Bilanzierungsgebiet" section — the BO4E field, typed.
#[test]
fn docs_bilanzierungsgebiet_eic_is_checked() {
    use rubo4e::current::Marktlokation;

    let malo = Marktlokation {
        bilanzierungsgebiet: Some("11YN-0000-0001-Q".into()),
        ..Default::default()
    };
    assert!(malo.bilanzierungsgebiet_checked().unwrap().is_ok());

    // A Bilanzkreis is a *party* code (`11X…`), not an area code.
    let wrong = Marktlokation {
        bilanzierungsgebiet: Some("11XSUEDWESTSTRO8".into()),
        ..Default::default()
    };
    assert!(wrong.bilanzierungsgebiet_checked().unwrap().is_err());
}

/// The site's "The Zählpunkt that is not a Messlokation" section, and the
/// `Zaehlpunktbezeichnung` entry on the Identifiers page.
#[test]
fn docs_zaehlpunkt_is_not_a_messlokation() {
    use rubo4e::identifiers::{MeloId, Zaehlpunktbezeichnung};
    use rubo4e::identifiers::{Zaehlpunkt, Zaehlpunktart};

    let zpb = Zaehlpunktbezeichnung::new("DE0000000000000000000000000000042").unwrap();
    assert_eq!(zpb.country_code(), "DE");

    let zp = Zaehlpunkt::new(Zaehlpunktart::NetzgangzeitreiheEmob, zpb);
    assert!(zp.is_emobilitaet());
    assert_eq!(zp.as_melo_id(), None);

    // Widening is a fact; narrowing is a claim, and is spelled out.
    let melo = MeloId::new("DE0000000000000000000000000000001").unwrap();
    let widened = Zaehlpunktbezeichnung::from(melo.clone());
    assert_eq!(widened.clone().into_melo_id(), melo);

    #[cfg(feature = "versioned")]
    {
        use rubo4e::current::Messlokation;
        use rubo4e::zusatz_attribut::{well_known, ZusatzAttributeExt};

        let mut melo = Messlokation::default();
        melo.set_zusatz_attribut_key(&well_known::ZAEHLPUNKT, &zp)
            .unwrap();
        assert_eq!(well_known::ZAEHLPUNKT.name(), "mabis:zaehlpunkt");
    }
}

/// The site's "E-Mobilitätsladesäule: BO4E already has it" section.
#[test]
fn docs_charging_point_is_native_bo4e() {
    use rubo4e::current::{EMobilitaetsart, TechnischeRessource, TechnischeRessourceVerbrauchsart};

    let ladesaeule = TechnischeRessource {
        emobilitaetsart: Some(EMobilitaetsart::EMobilitaetsladesaeule),
        technische_ressource_verbrauchsart: Some(TechnischeRessourceVerbrauchsart::EMobilitaet),
        ..Default::default()
    };
    assert!(ladesaeule.is_emobilitaetsladesaeule());

    // The three values BO4E publishes, as the page lists them.
    assert_eq!(
        EMobilitaetsart::VARIANTS,
        [
            EMobilitaetsart::Wallbox,
            EMobilitaetsart::EMobilitaetsladesaeule,
            EMobilitaetsart::Ladepark,
        ]
    );
}

/// The site's "The resting Aggregationsverantwortung" section.
#[test]
fn docs_resting_aggregationsverantwortung() {
    use rubo4e::convenience::Aggregationszustaendigkeit;
    use rubo4e::current::{Abwicklungsmodell, Bilanzierung};

    let ruhend = Bilanzierung {
        abwicklungsmodell: Some(Abwicklungsmodell::Modell2),
        ..Default::default()
    };
    assert!(ruhend.aggregation_ruht());

    assert_eq!(
        Bilanzierung::default().aggregationszustaendigkeit(),
        Aggregationszustaendigkeit::Unbekannt,
    );
}

/// The site's "The mobile Marktlokation validates" section — and the README table
/// row that says no field is mandatory.
#[test]
fn docs_mobile_marktlokation_validates() {
    use rubo4e::current::Marktlokation;

    let mobil = Marktlokation {
        marktlokations_id: Some(MaloId::new("51238696781").unwrap()),
        bilanzierungsgebiet: Some("11YN-0000-0001-Q".into()),
        ..Default::default()
    };
    assert!(mobil.validate().is_ok());
}

/// Both documents claim `Zeitreihentyp` is exactly the DE7111 Summenzeitreihen
/// list and that `NGZ` is not in it.
#[test]
fn docs_zeitreihentyp_has_no_ngz() {
    use rubo4e::current::Zeitreihentyp;

    assert_eq!(
        Zeitreihentyp::VARIANTS
            .iter()
            .map(Zeitreihentyp::as_wire)
            .collect::<Vec<_>>(),
        ["EGS", "LGS", "NZR", "SES", "SLS", "TES", "TLS", "SLS_TLS", "SES_TES"],
    );
    assert!(Zeitreihentyp::from_wire("NGZ").is_err());
}
