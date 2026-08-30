//! The e-mobility "Modell 2" of BK6-20-160, end to end over BO4E types.
//!
//! Every test here also pins an **answer**: the six questions a Modell-2
//! implementation asks of BO4E, and what this crate says the carrier is. Where
//! the answer is "BO4E already has it", the test is what stops that claim rotting.
//!
//! Sources: BNetzA BK6-20-160, Anlage 6 (NZR-EMob); BDEW Anwendungshilfe zum
//! Modell 2, v1.3 (1 April 2025); BNetzA BK6-24-174, Anlage 3 (MaBiS), Kap. 3.5;
//! BDEW EDI@Energy "Codeliste der Zeitreihentypen" v1.1d (1 April 2021).

#![cfg(all(feature = "versioned", feature = "json"))]

use rubo4e::convenience::Aggregationszustaendigkeit;
use rubo4e::current::{
    Abwicklungsmodell, Aggregationsverantwortung, Bilanzierung, EMobilitaetsart, Marktlokation,
    TechnischeRessource, TechnischeRessourceVerbrauchsart, Verbrauchsart, Zeitreihentyp,
};
use rubo4e::prelude::*;

// ─── The Bilanzierungsgebiet (request 1) ─────────────────────────────────────

/// BO4E carries the EIC alone, and does not name its format. The checked
/// accessor does — and pins it to ENTSO-E object type `'Y'` (Area), which is what
/// MaBiS 3.5 requires of a Bilanzierungsgebiet.
#[test]
fn the_bilanzierungsgebiet_eic_is_an_area_code() {
    let malo = Marktlokation {
        bilanzierungsgebiet: Some("11YN-0000-0001-Q".into()),
        ..Default::default()
    };
    let bg = malo.bilanzierungsgebiet_checked().unwrap().unwrap();
    assert_eq!(bg.to_eic_code().eic_type(), EicType::Area);

    // A Bilanzkreis is a *party* code and must not pass as a Bilanzierungsgebiet.
    let wrong = Marktlokation {
        bilanzierungsgebiet: Some("11XSUEDWESTSTRO8".into()),
        ..Default::default()
    };
    assert!(wrong.bilanzierungsgebiet_checked().unwrap().is_err());

    // Absent is absent, not an error.
    assert!(Marktlokation::default()
        .bilanzierungsgebiet_checked()
        .is_none());
}

/// The Stammdaten around that EIC — Regelzone, validity, owner, the e-mobility
/// Zusatzinformation, the Deltazeitreihen-Bilanzkreis — are **not** here, and this
/// is the test that says so deliberately rather than by omission.
///
/// They read no BO4E field, so a `Bilanzierungsgebiet` aggregate in this crate
/// would be a new object wrapped in a `ZusatzAttribut` that no other BO4E
/// implementation understands. What BO4E does carry is the EIC, and that is typed
/// above. The aggregate belongs in the crate that owns the MaBiS processes.
#[test]
fn the_bilanzierungsgebiet_stammdaten_are_not_modelled_here() {
    // The registry holds keys that qualify a BO4E field. There is no key for a
    // Bilanzierungsgebiet, and adding one would be the wrong call.
    use rubo4e::zusatz_attribut::{well_known, Namespace};

    assert_eq!(well_known::ZAEHLPUNKT.name(), "mabis:zaehlpunkt");
    assert_eq!(well_known::ZAEHLPUNKT.namespace(), &Namespace::MABIS);

    // …and the EIC, which *is* a BO4E field, needs no key at all.
    let malo = Marktlokation {
        bilanzierungsgebiet: Some("11YN-0000-0001-Q".into()),
        ..Default::default()
    };
    let json = malo.to_json_german().unwrap();
    assert!(
        json.contains(r#""bilanzierungsgebiet":"11YN-0000-0001-Q""#),
        "{json}"
    );
    assert!(
        !json.contains("zusatzAttribute"),
        "no escape hatch needed: {json}"
    );
}

// ─── The Zeitreihentyp (request 2) ───────────────────────────────────────────

/// `Zeitreihentyp` is chapter 1 of the BDEW "Codeliste der Zeitreihentypen" —
/// the **Summen**zeitreihentypen for DE7111 — and nothing is missing from it.
///
/// `NGZ` is not a code in that list, in any published version: it appears only
/// inside the explanation of `NZR` (*"Summendifferenz der NGZ zwischen zwei
/// Bilanzierungsgebieten"*). A Netzgangzeitreihe is a measured series at one
/// Zählpunkt, carried by MSCONS Prüfidentifikator 13018 — which in BO4E is a
/// `Lastgang`, not a `Zeitreihentyp`.
#[test]
fn zeitreihentyp_is_exactly_the_summenzeitreihen_codelist() {
    let published = [
        "EGS", "LGS", "NZR", "SES", "SLS", "TES", "TLS", "SLS_TLS", "SES_TES",
    ];
    let actual: Vec<&str> = Zeitreihentyp::VARIANTS
        .iter()
        .map(Zeitreihentyp::as_wire)
        .collect();
    assert_eq!(actual, published);

    // The summed network series is there…
    assert_eq!(Zeitreihentyp::from_wire("NZR"), Ok(Zeitreihentyp::Nzr));
    // …and the per-Zählpunkt one is not a member of this list at all.
    assert!(Zeitreihentyp::from_wire("NGZ").is_err());
}

/// What a Netzgangzeitreihe *is*, in BO4E: a `Lastgang` at a Zählpunkt whose kind
/// says which series it belongs to.
#[cfg(all(feature = "time", feature = "decimal"))]
#[test]
fn a_netzgangzeitreihe_is_a_lastgang_at_a_zaehlpunkt() {
    use rubo4e::current::{Lastgang, Menge, Mengeneinheit, Zeitraum, Zeitreihenwert};
    use rubo4e::identifiers::{Zaehlpunkt, Zaehlpunktart};
    use rubo4e::timeseries::Bo4eIntervals;
    use rust_decimal::Decimal;
    use time::macros::datetime;

    let zp = Zaehlpunkt::new(
        Zaehlpunktart::NetzgangzeitreiheEmob,
        Zaehlpunktbezeichnung::new("DE0000000000000000000000000000042").unwrap(),
    );
    assert!(zp.is_emobilitaet());

    let start = datetime!(2026-04-01 00:00 +02:00);
    let ngz = Lastgang {
        messgroesse: Some(Mengeneinheit::Kwh),
        werte: Some(vec![Zeitreihenwert {
            wert: Some(Decimal::from(11)),
            zeitraum: Some(Zeitraum::from_instants(
                start,
                start + time::Duration::minutes(15),
            )),
            ..Default::default()
        }]),
        ..Lastgang::new(Menge {
            wert: Some(Decimal::from(15)),
            einheit: Some(Mengeneinheit::Minute),
            ..Default::default()
        })
    };

    assert_eq!(ngz.intervals().count(), 1);
    assert_eq!(
        ngz.total_energy(),
        Some((Decimal::from(11), Mengeneinheit::Kwh))
    );
}

// ─── E-Mobilitätsladesäule (request 3) ───────────────────────────────────────

/// BO4E **already models** the charging point — on the technische Ressource, not
/// on `Marktlokation.verbrauchsart`. No `ZusatzAttribut` is needed.
#[test]
fn the_charging_point_is_modelled_on_the_technische_ressource() {
    // `Verbrauchsart` is the Kraft/Licht/Wärme categorisation and nothing else.
    assert_eq!(
        Verbrauchsart::VARIANTS,
        [
            Verbrauchsart::Kl,
            Verbrauchsart::Klw,
            Verbrauchsart::Klws,
            Verbrauchsart::W,
            Verbrauchsart::Ws,
        ]
    );

    // The Ladesäule is a value BO4E already has, in two places.
    let ladesaeule = TechnischeRessource {
        emobilitaetsart: Some(EMobilitaetsart::EMobilitaetsladesaeule),
        technische_ressource_verbrauchsart: Some(TechnischeRessourceVerbrauchsart::EMobilitaet),
        ..Default::default()
    };
    assert!(ladesaeule.is_emobilitaetsladesaeule());
    assert!(ladesaeule.is_emobilitaet());

    // …and it survives the wire as ordinary BO4E, with nothing in extension data.
    let json = ladesaeule.to_json_german().unwrap();
    assert!(
        json.contains(r#""emobilitaetsart":"E_MOBILITAETSLADESAEULE""#),
        "{json}"
    );
    let back = TechnischeRessource::from_json_german(&json).unwrap();
    assert!(back.extension_paths().is_empty());
    assert!(back.ensure_known_enums().is_ok());
}

// ─── The Zählpunkt (eMob) (request 4) ────────────────────────────────────────

/// AWH § 1.6.2: *"Für den Zählpunkt (eMob) wird nicht die ID der Messlokation
/// (Zählpunktbezeichnung) verwendet."* The types are what enforce it.
#[cfg(feature = "time")]
#[test]
fn a_zaehlpunkt_emob_cannot_become_a_messlokations_id() {
    use rubo4e::current::Messlokation;
    use rubo4e::identifiers::{Zaehlpunkt, Zaehlpunktart};
    use rubo4e::zusatz_attribut::{well_known, ZusatzAttributeExt};

    let zp = Zaehlpunkt::new(
        Zaehlpunktart::NetzgangzeitreiheEmob,
        Zaehlpunktbezeichnung::new("DE0000000000000000000000000000042").unwrap(),
    );
    // The grammar matches a MeLo-ID, and the conversion still refuses.
    assert_eq!(zp.as_melo_id(), None);

    // It rides on the Messlokation under the registered key instead — a
    // Lokationsbündel therefore carries it, because the MeLo is in the bundle.
    let mut melo = Messlokation {
        messlokations_id: Some(MeloId::new("DE0000000000000000000000000000001").unwrap()),
        ..Default::default()
    };
    melo.set_zusatz_attribut_key(&well_known::ZAEHLPUNKT, &zp)
        .unwrap();

    let decoded = Messlokation::from_json_german(&melo.to_json_german().unwrap()).unwrap();
    let back = decoded
        .zusatz_attribut_key(&well_known::ZAEHLPUNKT)
        .unwrap()
        .unwrap();
    assert_eq!(back, zp);
    assert!(!back.is_messlokation());
    // The MeLo's own ID is untouched and still a MeLo-ID.
    assert_ne!(
        back.bezeichnung.as_str(),
        decoded.messlokations_id.unwrap().as_ref()
    );
}

// ─── The mobile Marktlokation (request 5) ────────────────────────────────────

/// A Modell-2 Marktlokation has no Lokationsadresse and no Messlokation of its
/// own, and **validates** — because BO4E declares no `required` field on
/// `Marktlokation` and this crate's only cross-field rule is "at most one
/// Ortsangabe".
///
/// Nothing here is mandatory. The rule is a *conflict* rule, not a presence rule.
#[cfg(feature = "validate")]
#[test]
fn a_mobile_marktlokation_validates_with_almost_nothing_set() {
    let mobil = Marktlokation {
        marktlokations_id: Some(MaloId::new("51238696781").unwrap()),
        bilanzierungsgebiet: Some("11YN-0000-0001-Q".into()),
        // No lokationsadresse, no geoadresse, no katasterinformation.
        // No zaehlwerke, no verbrauchsart, no bilanzierungsmethode.
        ..Default::default()
    };
    assert!(mobil.validate().is_ok(), "{:?}", mobil.validate());

    // Even the entirely empty one conforms — BO4E has no reference type, so a
    // Marktlokation carrying only an ID is how a reference is spelled.
    assert!(Marktlokation::default().validate().is_ok());

    // The one rule that does fire is the conflict, and only the conflict.
    let conflicting = Marktlokation {
        lokationsadresse: Some(Default::default()),
        geoadresse: Some(Default::default()),
        ..Default::default()
    };
    assert!(conflicting.validate().is_err());
}

// ─── Ruhende Aggregationsverantwortung (request 6) ───────────────────────────

/// AWH § 1.6.2: *"Beim Wechsel in das Modell 2 ruht die Aggregationsverantwortung
/// für die Energiemenge der MaLo."* The wire encoding is an absent field, read
/// together with `abwicklungsmodell`.
#[test]
fn a_resting_aggregationsverantwortung_is_an_absent_field_plus_modell_2() {
    let ruhend = Bilanzierung {
        abwicklungsmodell: Some(Abwicklungsmodell::Modell2),
        ..Default::default()
    };
    assert!(ruhend.is_modell_2());
    assert!(ruhend.aggregation_ruht());
    assert_eq!(
        ruhend.aggregationszustaendigkeit(),
        Aggregationszustaendigkeit::Ruhend
    );

    // An absent field alone says nothing: BO4E declares it optional and most
    // payloads omit it.
    assert_eq!(
        Bilanzierung::default().aggregationszustaendigkeit(),
        Aggregationszustaendigkeit::Unbekannt
    );
    assert!(!Bilanzierung::default().aggregation_ruht());

    // Modell 1 with a named holder is unchanged.
    let modell1 = Bilanzierung {
        abwicklungsmodell: Some(Abwicklungsmodell::Modell1),
        aggregationsverantwortung: Some(Aggregationsverantwortung::Uenb),
        ..Default::default()
    };
    assert!(!modell1.aggregation_ruht());
    assert_eq!(
        modell1.aggregationszustaendigkeit(),
        Aggregationszustaendigkeit::Uebertragungsnetzbetreiber
    );

    // `RUHEND` is not a schema value, and a payload carrying one is rejected —
    // which is exactly why the state is not encoded that way.
    let forged = r#"{"_typ":"BILANZIERUNG","aggregationsverantwortung":"RUHEND"}"#;
    let decoded = Bilanzierung::from_json_german(forged).unwrap();
    assert!(decoded.ensure_known_enums().is_err());
    assert_eq!(
        decoded.aggregationszustaendigkeit(),
        Aggregationszustaendigkeit::Unbekannt,
        "a lenient decode of an out-of-schema value must not read as Ruhend"
    );
}
