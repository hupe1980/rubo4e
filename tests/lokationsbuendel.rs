//! The Lokationsbündelstruktur: reading a bundle, and checking one.
//!
//! The unit tests in `src/lokationsbuendel` pin the codelist itself — the check
//! digits, the cross-references, the counts. This file exercises the layer above:
//! a decoded `Lokationszuordnung`, read as the bundle it describes.

#![cfg(all(feature = "versioned", feature = "json"))]

use rubo4e::current::{
    Lokationszuordnung, Marktlokation, Messlokation, Netzlokation, SteuerbareRessource,
    TechnischeRessource,
};
use rubo4e::identifiers::{LokationsbuendelObjektcode, Lokationsbuendelcode};
use rubo4e::lokationsbuendel::{
    Befund, Flussrichtung, LokationsbuendelExt, LokationsbuendelObjekt, Objekttyp,
};

/// "Verbrauch mit einer Messlokation (Standard)" — one MeLo, one MaLo, any number
/// of TR, at most one NeLo.
const STANDARD: &str = "9992000000026";

const MELO_NETZUEBERGABE_VERBRAUCH: &str = "9992000001032";
const MALO_VERBRAUCH_E1: &str = "9992000001016";
const TR_VERBRAUCH_E1: &str = "9992000001024";
const NELO_E1: &str = "9992000001256";

fn malo(code: &str) -> Box<Marktlokation> {
    Box::new(Marktlokation {
        lokationsbuendel_objektcode: Some(code.to_owned()),
        ..Default::default()
    })
}

fn melo(code: &str) -> Box<Messlokation> {
    Box::new(Messlokation {
        lokationsbuendel_objektcode: Some(code.to_owned()),
        ..Default::default()
    })
}

fn tr(code: &str) -> Box<TechnischeRessource> {
    Box::new(TechnischeRessource {
        lokationsbuendel_objektcode: Some(code.to_owned()),
        ..Default::default()
    })
}

/// A § 14a EnWG household: grid meter, one Marktlokation, a heat pump and a
/// wallbox behind it, plus the Netzlokation.
fn conformant_standard_bundle() -> Lokationszuordnung {
    Lokationszuordnung {
        lokationsbuendelcode: Some(STANDARD.to_owned()),
        messlokationen: Some(vec![melo(MELO_NETZUEBERGABE_VERBRAUCH)]),
        marktlokationen: Some(vec![malo(MALO_VERBRAUCH_E1)]),
        technische_ressourcen: Some(vec![tr(TR_VERBRAUCH_E1), tr(TR_VERBRAUCH_E1)]),
        netzlokationen: Some(vec![Box::new(Netzlokation {
            lokationsbuendel_objektcode: Some(NELO_E1.to_owned()),
            ..Default::default()
        })]),
        ..Default::default()
    }
}

#[test]
fn a_conformant_bundle_reports_nothing() {
    let zuordnung = conformant_standard_bundle();
    let report = zuordnung.audit_buendel();
    assert!(
        report.is_conformant(),
        "expected a clean report, got:\n{report}"
    );
    assert_eq!(
        report.struktur.map(|s| s.bezeichnung),
        Some("Verbrauch mit einer Messlokation (Standard)")
    );
}

#[test]
fn the_bundle_view_reaches_the_fourteen_a_resources() {
    let zuordnung = conformant_standard_bundle();
    let buendel = zuordnung.buendel();

    assert_eq!(buendel.len(), 5);
    assert!(!buendel.is_empty());
    assert_eq!(buendel.verbrauchs_ressourcen().count(), 2);

    // Everything in a Standard bundle sits on level 1.
    let ebene1 = buendel.objekte_auf_ebene(1);
    assert_eq!(ebene1.len(), 5);
    assert!(buendel.objekte_auf_ebene(2).is_empty());
    assert!(ebene1.contains(&(Objekttyp::Marktlokation, MALO_VERBRAUCH_E1)));

    let code = buendel.code().expect("declared").expect("valid");
    assert_eq!(code, Lokationsbuendelcode::new(STANDARD).unwrap());
    assert_eq!(code.grouped(), "9992 00000 002 6");
}

#[test]
fn an_object_reads_its_own_place() {
    let heat_pump = tr(TR_VERBRAUCH_E1);
    let rolle = heat_pump.objektrolle().expect("catalogued");
    assert_eq!(rolle.objekttyp, Objekttyp::TechnischeRessource);
    assert_eq!(heat_pump.flussrichtung(), Some(Flussrichtung::Verbrauch));
    assert_eq!(heat_pump.ebene(), Some(1));
    assert_eq!(
        heat_pump.lokationsbuendel_objektcode(),
        Some(Ok(LokationsbuendelObjektcode::new(TR_VERBRAUCH_E1).unwrap()))
    );
}

/// The BDEW codelist covers NeLo, MeLo, MaLo and TR only, so a
/// `SteuerbareRessource` has no object code to be checked against.
#[test]
fn steuerbare_ressourcen_are_listed_but_not_audited() {
    let mut zuordnung = conformant_standard_bundle();
    zuordnung.steuerbare_ressourcen = Some(vec![Box::new(SteuerbareRessource {
        // Deliberately nonsense as an *object* code — the audit must not care.
        lokationsbuendel_objektcode: Some("9992000009991".to_owned()),
        ..Default::default()
    })]);

    assert_eq!(
        <SteuerbareRessource as LokationsbuendelObjekt>::codelist_objekttyp(),
        None
    );
    assert_eq!(zuordnung.buendel().steuerbare_ressourcen().count(), 1);
    assert!(zuordnung.audit_buendel().is_conformant());
}

#[test]
fn a_missing_mandatory_object_is_reported() {
    let mut zuordnung = conformant_standard_bundle();
    zuordnung.messlokationen = None;

    let report = zuordnung.audit_buendel();
    assert!(!report.is_conformant());
    assert!(report.befunde.contains(&Befund::AnzahlVerletzt {
        code: MELO_NETZUEBERGABE_VERBRAUCH.to_owned(),
        objekttyp: Objekttyp::Messlokation,
        gefunden: 0,
        erwartet: "1".to_owned(),
    }));
}

#[test]
fn too_many_of_a_rigid_object_is_reported() {
    let mut zuordnung = conformant_standard_bundle();
    zuordnung.marktlokationen = Some(vec![malo(MALO_VERBRAUCH_E1), malo(MALO_VERBRAUCH_E1)]);

    let report = zuordnung.audit_buendel();
    assert!(report.befunde.contains(&Befund::AnzahlVerletzt {
        code: MALO_VERBRAUCH_E1.to_owned(),
        objekttyp: Objekttyp::Marktlokation,
        gefunden: 2,
        erwartet: "1".to_owned(),
    }));
}

/// A Marktlokation carrying a Messlokation's object code — the mistake a
/// hand-written mapping makes, and the one a check-digit test cannot see.
#[test]
fn an_object_code_filed_under_the_wrong_type_is_reported() {
    let mut zuordnung = conformant_standard_bundle();
    zuordnung.marktlokationen = Some(vec![malo(MELO_NETZUEBERGABE_VERBRAUCH)]);

    let report = zuordnung.audit_buendel();
    assert!(report.befunde.contains(&Befund::ObjekttypWiderspruch {
        gefunden: Objekttyp::Marktlokation,
        index: 0,
        erwartet: Objekttyp::Messlokation,
        code: MELO_NETZUEBERGABE_VERBRAUCH.to_owned(),
    }));
    // …and the MaLo the structure requires is then missing.
    assert!(report
        .befunde
        .iter()
        .any(|b| matches!(b, Befund::AnzahlVerletzt { code, .. } if code == MALO_VERBRAUCH_E1)));
}

#[test]
fn a_code_from_another_structure_is_reported() {
    let mut zuordnung = conformant_standard_bundle();
    // A level-2 hinterschaltete Erzeugungs-MeLo has no place in the Standard
    // structure.
    zuordnung
        .messlokationen
        .as_mut()
        .unwrap()
        .push(melo("9992000001090"));

    let report = zuordnung.audit_buendel();
    assert!(report.befunde.contains(&Befund::ObjektcodeNichtInStruktur {
        objekttyp: Objekttyp::Messlokation,
        index: 1,
        code: "9992000001090".to_owned(),
    }));
}

#[test]
fn a_bad_check_digit_is_reported_once_and_not_counted() {
    let mut zuordnung = conformant_standard_bundle();
    zuordnung.marktlokationen = Some(vec![malo("9992000001017")]); // last digit wrong

    let report = zuordnung.audit_buendel();
    assert!(report
        .befunde
        .iter()
        .any(|b| matches!(b, Befund::ObjektcodeUngueltig { index: 0, .. })));
    // Exactly one shortfall for the MaLo, not a second complaint about the same
    // object.
    assert_eq!(
        report
            .befunde
            .iter()
            .filter(
                |b| matches!(b, Befund::AnzahlVerletzt { code, .. } if code == MALO_VERBRAUCH_E1)
            )
            .count(),
        1
    );
}

/// Object-level checks that do not need the structure still run when the
/// structure code is missing.
#[test]
fn a_missing_structure_code_does_not_silence_everything() {
    let mut zuordnung = conformant_standard_bundle();
    zuordnung.lokationsbuendelcode = None;
    zuordnung.marktlokationen = Some(vec![malo(MELO_NETZUEBERGABE_VERBRAUCH)]);

    let report = zuordnung.audit_buendel();
    assert_eq!(report.struktur, None);
    assert!(report.befunde.contains(&Befund::StrukturcodeFehlt));
    assert!(report
        .befunde
        .iter()
        .any(|b| matches!(b, Befund::ObjekttypWiderspruch { .. })));
    // No cardinality findings without a structure to measure against.
    assert!(!report
        .befunde
        .iter()
        .any(|b| matches!(b, Befund::AnzahlVerletzt { .. })));
}

#[test]
fn an_unpublished_structure_code_is_reported() {
    let zuordnung = Lokationszuordnung {
        lokationsbuendelcode: Some(
            Lokationsbuendelcode::from_base("999200000999")
                .unwrap()
                .as_str()
                .to_owned(),
        ),
        ..Default::default()
    };
    let report = zuordnung.audit_buendel();
    assert!(report
        .befunde
        .iter()
        .any(|b| matches!(b, Befund::StrukturUnbekannt { .. })));
}

#[test]
fn an_object_with_no_code_is_reported() {
    let mut zuordnung = conformant_standard_bundle();
    zuordnung
        .technische_ressourcen
        .as_mut()
        .unwrap()
        .push(Box::new(TechnischeRessource::default()));

    let report = zuordnung.audit_buendel();
    assert!(report.befunde.contains(&Befund::ObjektcodeFehlt {
        objekttyp: Objekttyp::TechnischeRessource,
        index: 2,
    }));
}

/// The codes survive a JSON round trip as plain strings, so a bundle read off the
/// wire audits the same way one built in Rust does.
#[test]
fn a_bundle_survives_the_wire() {
    use rubo4e::prelude::Bo4eJsonExt;

    let original = conformant_standard_bundle();
    let json = original.to_json_german().unwrap();
    assert!(json.contains("\"lokationsbuendelcode\":\"9992000000026\""));
    assert!(json.contains("\"lokationsbuendelObjektcode\":\"9992000001016\""));

    let decoded = Lokationszuordnung::from_json_german(&json).unwrap();
    assert!(decoded.audit_buendel().is_conformant());
}

/// Every published structure is internally consistent as an *audit target*: a
/// bundle built from exactly its mandatory objects passes.
#[test]
fn every_published_structure_accepts_its_own_minimum() {
    for struktur in rubo4e::lokationsbuendel::STRUKTUREN {
        let mut zuordnung = Lokationszuordnung {
            lokationsbuendelcode: Some(struktur.code.to_owned()),
            ..Default::default()
        };
        for row in struktur.objekte {
            for _ in 0..row.min {
                match row.rolle().objekttyp {
                    Objekttyp::Marktlokation => zuordnung
                        .marktlokationen
                        .get_or_insert_with(Vec::new)
                        .push(malo(row.code)),
                    Objekttyp::Messlokation => zuordnung
                        .messlokationen
                        .get_or_insert_with(Vec::new)
                        .push(melo(row.code)),
                    Objekttyp::TechnischeRessource => zuordnung
                        .technische_ressourcen
                        .get_or_insert_with(Vec::new)
                        .push(tr(row.code)),
                    Objekttyp::Netzlokation => zuordnung
                        .netzlokationen
                        .get_or_insert_with(Vec::new)
                        .push(Box::new(Netzlokation {
                            lokationsbuendel_objektcode: Some(row.code.to_owned()),
                            ..Default::default()
                        })),
                }
            }
        }
        let report = zuordnung.audit_buendel();
        assert!(
            report.is_conformant(),
            "{}: minimal bundle rejected:\n{report}",
            struktur.code
        );
    }
}
