pub mod abgabe_art;
pub mod abwicklungsmodell;
pub mod adresse;
pub mod aggregationsverantwortung;
pub mod angebot;
pub mod angebotsposition;
pub mod angebotsstatus;
pub mod angebotsteil;
pub mod angebotsvariante;
pub mod anrede;
pub mod arithmetische_operation;
pub mod artikel_id;
pub mod auf_abschlag;
pub mod auf_abschlag_pro_ort;
pub mod auf_abschlag_regional;
pub mod auf_abschlagstaffel_pro_ort;
pub mod auf_abschlagstyp;
pub mod auf_abschlagsziel;
pub mod ausschreibung;
pub mod ausschreibungsdetail;
pub mod ausschreibungslos;
pub mod ausschreibungsportal;
pub mod ausschreibungsstatus;
pub mod ausschreibungstyp;
pub mod bdew_artikelnummer;
pub mod befestigungsart;
pub mod bemessungsgroesse;
pub mod betrag;
pub mod bilanzierung;
pub mod bilanzierungsmethode;
pub mod bo_typ;
pub mod buendelvertrag;
pub mod com_typ;
pub mod dienstleistung;
pub mod dienstleistungstyp;
pub mod e_mobilitaetsart;
pub mod energieherkunft;
pub mod energiemenge;
pub mod energiemix;
pub mod energierichtung;
pub mod erzeugungsart;
pub mod fallgruppenzuordnung;
pub mod fremdkosten;
pub mod fremdkostenblock;
pub mod fremdkostenposition;
pub mod gasqualitaet;
pub mod gebiettyp;
pub mod geokoordinaten;
pub mod geraet;
pub mod geraeteklasse;
pub mod geraetetyp;
pub mod geschaeftspartner;
pub mod geschaeftspartnerrolle;
pub mod gueltigkeitstyp;
pub mod kalkulationsmethode;
pub mod katasteradresse;
pub mod konfigurationsprodukt;
pub mod kontaktart;
pub mod kontaktweg;
pub mod konzessionsabgabe;
pub mod kosten;
pub mod kostenblock;
pub mod kostenklasse;
pub mod kostenposition;
pub mod kriterium_wert;
pub mod kundengruppe;
pub mod kundengruppe_ka;
pub mod kundentyp;
pub mod landescode;
pub mod lastgang;
pub mod lastprofil;
pub mod leistungstyp;
pub mod lokationstyp;
pub mod lokationszuordnung;
pub mod marktgebiet_info;
pub mod marktlokation;
pub mod marktrolle;
pub mod marktteilnehmer;
pub mod medium;
pub mod menge;
pub mod mengeneinheit;
pub mod mengenoperator;
pub mod messart;
pub mod messgroesse;
pub mod messlokation;
pub mod messpreistyp;
pub mod messwerterfassung;
pub mod messwertstatus;
pub mod messwertstatuszusatz;
pub mod netzebene;
pub mod netzlokation;
pub mod netznutzung_rechnungsart;
pub mod netznutzung_rechnungstyp;
pub mod oekolabel;
pub mod oekozertifikat;
pub mod organisationstyp;
pub mod person;
pub mod positions_auf_abschlag;
pub mod preis;
pub mod preisblatt;
pub mod preisblatt_dienstleistung;
pub mod preisblatt_hardware;
pub mod preisblatt_konzessionsabgabe;
pub mod preisblatt_messung;
pub mod preisblatt_netznutzung;
pub mod preisgarantie;
pub mod preisgarantietyp;
pub mod preismodell;
pub mod preisposition;
pub mod preisstaffel;
pub mod preisstatus;
pub mod preistyp;
pub mod profilart;
pub mod profiltyp;
pub mod profilverfahren;
pub mod prognosegrundlage;
pub mod rechnung;
pub mod rechnungslegung;
pub mod rechnungsposition;
pub mod rechnungsstatus;
pub mod rechnungstyp;
pub mod region;
pub mod regionale_gueltigkeit;
pub mod regionale_preisgarantie;
pub mod regionale_preisstaffel;
pub mod regionale_tarifpreisposition;
pub mod regionaler_auf_abschlag;
pub mod regionaltarif;
pub mod regionskriterium;
pub mod regionskriteriumtyp;
pub mod registeranzahl;
pub mod rollencodetyp;
pub mod sigmoidparameter;
pub mod sparte;
pub mod speicherart;
pub mod standorteigenschaften;
pub mod standorteigenschaften_gas;
pub mod standorteigenschaften_strom;
pub mod steuerbare_ressource;
pub mod steuerbetrag;
pub mod steuerkanal_leistungsbeschreibung;
pub mod steuerkennzeichen;
pub mod tagesparameter;
pub mod tarif;
pub mod tarifberechnungsparameter;
pub mod tarifeinschraenkung;
pub mod tarifinfo;
pub mod tarifkalkulationsmethode;
pub mod tarifkosten;
pub mod tarifmerkmal;
pub mod tarifpreis;
pub mod tarifpreisblatt;
pub mod tarifpreisposition;
pub mod tarifpreisposition_pro_ort;
pub mod tarifpreisstaffel_pro_ort;
pub mod tarifregionskriterium;
pub mod tariftyp;
pub mod tarifzeit;
pub mod technische_ressource;
pub mod technische_ressource_nutzung;
pub mod technische_ressource_verbrauchsart;
pub mod themengebiet;
pub mod titel;
pub mod unterschrift;
pub mod verbrauch;
pub mod verbrauchsart;
pub mod vertrag;
pub mod vertragsart;
pub mod vertragsform;
pub mod vertragskonditionen;
pub mod vertragsstatus;
pub mod vertragsteil;
pub mod verwendungszweck;
pub mod verwendungszweck_pro_marktrolle;
pub mod voraussetzungen;
pub mod waehrungscode;
pub mod waehrungseinheit;
pub mod waermenutzung;
pub mod wahlrecht_prognosegrundlage;
pub mod zaehler;
pub mod zaehlerauspraegung;
pub mod zaehlergroesse;
pub mod zaehlertyp;
pub mod zaehlertyp_spezifikation;
pub mod zaehlwerk;
pub mod zaehlzeitregister;
pub mod zeitraum;
pub mod zeitreihe;
pub mod zeitreihentyp;
pub mod zeitreihenwert;
pub mod zusatz_attribut;
pub mod zustaendigkeit;
pub use crate::Bo4eObject;
pub use abgabe_art::AbgabeArt;
pub use abwicklungsmodell::Abwicklungsmodell;
pub use adresse::Adresse;
pub use aggregationsverantwortung::Aggregationsverantwortung;
pub use angebot::Angebot;
pub use angebotsposition::Angebotsposition;
pub use angebotsstatus::Angebotsstatus;
pub use angebotsteil::Angebotsteil;
pub use angebotsvariante::Angebotsvariante;
pub use anrede::Anrede;
pub use arithmetische_operation::ArithmetischeOperation;
pub use artikel_id::ArtikelId;
pub use auf_abschlag::AufAbschlag;
pub use auf_abschlag_pro_ort::AufAbschlagProOrt;
pub use auf_abschlag_regional::AufAbschlagRegional;
pub use auf_abschlagstaffel_pro_ort::AufAbschlagstaffelProOrt;
pub use auf_abschlagstyp::AufAbschlagstyp;
pub use auf_abschlagsziel::AufAbschlagsziel;
pub use ausschreibung::Ausschreibung;
pub use ausschreibungsdetail::Ausschreibungsdetail;
pub use ausschreibungslos::Ausschreibungslos;
pub use ausschreibungsportal::Ausschreibungsportal;
pub use ausschreibungsstatus::Ausschreibungsstatus;
pub use ausschreibungstyp::Ausschreibungstyp;
pub use bdew_artikelnummer::BdewArtikelnummer;
pub use befestigungsart::Befestigungsart;
pub use bemessungsgroesse::Bemessungsgroesse;
pub use betrag::Betrag;
pub use bilanzierung::Bilanzierung;
pub use bilanzierungsmethode::Bilanzierungsmethode;
pub use bo_typ::BoTyp;
pub use buendelvertrag::Buendelvertrag;
pub use com_typ::ComTyp;
pub use dienstleistung::Dienstleistung;
pub use dienstleistungstyp::Dienstleistungstyp;
pub use e_mobilitaetsart::EMobilitaetsart;
pub use energieherkunft::Energieherkunft;
pub use energiemenge::Energiemenge;
pub use energiemix::Energiemix;
pub use energierichtung::Energierichtung;
pub use erzeugungsart::Erzeugungsart;
pub use fallgruppenzuordnung::Fallgruppenzuordnung;
pub use fremdkosten::Fremdkosten;
pub use fremdkostenblock::Fremdkostenblock;
pub use fremdkostenposition::Fremdkostenposition;
pub use gasqualitaet::Gasqualitaet;
pub use gebiettyp::Gebiettyp;
pub use geokoordinaten::Geokoordinaten;
pub use geraet::Geraet;
pub use geraeteklasse::Geraeteklasse;
pub use geraetetyp::Geraetetyp;
pub use geschaeftspartner::Geschaeftspartner;
pub use geschaeftspartnerrolle::Geschaeftspartnerrolle;
pub use gueltigkeitstyp::Gueltigkeitstyp;
pub use kalkulationsmethode::Kalkulationsmethode;
pub use katasteradresse::Katasteradresse;
pub use konfigurationsprodukt::Konfigurationsprodukt;
pub use kontaktart::Kontaktart;
pub use kontaktweg::Kontaktweg;
pub use konzessionsabgabe::Konzessionsabgabe;
pub use kosten::Kosten;
pub use kostenblock::Kostenblock;
pub use kostenklasse::Kostenklasse;
pub use kostenposition::Kostenposition;
pub use kriterium_wert::KriteriumWert;
pub use kundengruppe::Kundengruppe;
pub use kundengruppe_ka::KundengruppeKa;
pub use kundentyp::Kundentyp;
pub use landescode::Landescode;
pub use lastgang::Lastgang;
pub use lastprofil::Lastprofil;
pub use leistungstyp::Leistungstyp;
pub use lokationstyp::Lokationstyp;
pub use lokationszuordnung::Lokationszuordnung;
pub use marktgebiet_info::MarktgebietInfo;
pub use marktlokation::Marktlokation;
pub use marktrolle::Marktrolle;
pub use marktteilnehmer::Marktteilnehmer;
pub use medium::Medium;
pub use menge::Menge;
pub use mengeneinheit::Mengeneinheit;
pub use mengenoperator::Mengenoperator;
pub use messart::Messart;
pub use messgroesse::Messgroesse;
pub use messlokation::Messlokation;
pub use messpreistyp::Messpreistyp;
pub use messwerterfassung::Messwerterfassung;
pub use messwertstatus::Messwertstatus;
pub use messwertstatuszusatz::Messwertstatuszusatz;
pub use netzebene::Netzebene;
pub use netzlokation::Netzlokation;
pub use netznutzung_rechnungsart::NetznutzungRechnungsart;
pub use netznutzung_rechnungstyp::NetznutzungRechnungstyp;
pub use oekolabel::Oekolabel;
pub use oekozertifikat::Oekozertifikat;
pub use organisationstyp::Organisationstyp;
pub use person::Person;
pub use positions_auf_abschlag::PositionsAufAbschlag;
pub use preis::Preis;
pub use preisblatt::Preisblatt;
pub use preisblatt_dienstleistung::PreisblattDienstleistung;
pub use preisblatt_hardware::PreisblattHardware;
pub use preisblatt_konzessionsabgabe::PreisblattKonzessionsabgabe;
pub use preisblatt_messung::PreisblattMessung;
pub use preisblatt_netznutzung::PreisblattNetznutzung;
pub use preisgarantie::Preisgarantie;
pub use preisgarantietyp::Preisgarantietyp;
pub use preismodell::Preismodell;
pub use preisposition::Preisposition;
pub use preisstaffel::Preisstaffel;
pub use preisstatus::Preisstatus;
pub use preistyp::Preistyp;
pub use profilart::Profilart;
pub use profiltyp::Profiltyp;
pub use profilverfahren::Profilverfahren;
pub use prognosegrundlage::Prognosegrundlage;
pub use rechnung::Rechnung;
pub use rechnungslegung::Rechnungslegung;
pub use rechnungsposition::Rechnungsposition;
pub use rechnungsstatus::Rechnungsstatus;
pub use rechnungstyp::Rechnungstyp;
pub use region::Region;
pub use regionale_gueltigkeit::RegionaleGueltigkeit;
pub use regionale_preisgarantie::RegionalePreisgarantie;
pub use regionale_preisstaffel::RegionalePreisstaffel;
pub use regionale_tarifpreisposition::RegionaleTarifpreisposition;
pub use regionaler_auf_abschlag::RegionalerAufAbschlag;
pub use regionaltarif::Regionaltarif;
pub use regionskriterium::Regionskriterium;
pub use regionskriteriumtyp::Regionskriteriumtyp;
pub use registeranzahl::Registeranzahl;
pub use rollencodetyp::Rollencodetyp;
pub use sigmoidparameter::Sigmoidparameter;
pub use sparte::Sparte;
pub use speicherart::Speicherart;
pub use standorteigenschaften::Standorteigenschaften;
pub use standorteigenschaften_gas::StandorteigenschaftenGas;
pub use standorteigenschaften_strom::StandorteigenschaftenStrom;
pub use steuerbare_ressource::SteuerbareRessource;
pub use steuerbetrag::Steuerbetrag;
pub use steuerkanal_leistungsbeschreibung::SteuerkanalLeistungsbeschreibung;
pub use steuerkennzeichen::Steuerkennzeichen;
pub use tagesparameter::Tagesparameter;
pub use tarif::Tarif;
pub use tarifberechnungsparameter::Tarifberechnungsparameter;
pub use tarifeinschraenkung::Tarifeinschraenkung;
pub use tarifinfo::Tarifinfo;
pub use tarifkalkulationsmethode::Tarifkalkulationsmethode;
pub use tarifkosten::Tarifkosten;
pub use tarifmerkmal::Tarifmerkmal;
pub use tarifpreis::Tarifpreis;
pub use tarifpreisblatt::Tarifpreisblatt;
pub use tarifpreisposition::Tarifpreisposition;
pub use tarifpreisposition_pro_ort::TarifpreispositionProOrt;
pub use tarifpreisstaffel_pro_ort::TarifpreisstaffelProOrt;
pub use tarifregionskriterium::Tarifregionskriterium;
pub use tariftyp::Tariftyp;
pub use tarifzeit::Tarifzeit;
pub use technische_ressource::TechnischeRessource;
pub use technische_ressource_nutzung::TechnischeRessourceNutzung;
pub use technische_ressource_verbrauchsart::TechnischeRessourceVerbrauchsart;
pub use themengebiet::Themengebiet;
pub use titel::Titel;
pub use unterschrift::Unterschrift;
pub use verbrauch::Verbrauch;
pub use verbrauchsart::Verbrauchsart;
pub use vertrag::Vertrag;
pub use vertragsart::Vertragsart;
pub use vertragsform::Vertragsform;
pub use vertragskonditionen::Vertragskonditionen;
pub use vertragsstatus::Vertragsstatus;
pub use vertragsteil::Vertragsteil;
pub use verwendungszweck::Verwendungszweck;
pub use verwendungszweck_pro_marktrolle::VerwendungszweckProMarktrolle;
pub use voraussetzungen::Voraussetzungen;
pub use waehrungscode::Waehrungscode;
pub use waehrungseinheit::Waehrungseinheit;
pub use waermenutzung::Waermenutzung;
pub use wahlrecht_prognosegrundlage::WahlrechtPrognosegrundlage;
pub use zaehler::Zaehler;
pub use zaehlerauspraegung::Zaehlerauspraegung;
pub use zaehlergroesse::Zaehlergroesse;
pub use zaehlertyp::Zaehlertyp;
pub use zaehlertyp_spezifikation::ZaehlertypSpezifikation;
pub use zaehlwerk::Zaehlwerk;
pub use zaehlzeitregister::Zaehlzeitregister;
pub use zeitraum::Zeitraum;
pub use zeitreihe::Zeitreihe;
pub use zeitreihentyp::Zeitreihentyp;
pub use zeitreihenwert::Zeitreihenwert;
pub use zusatz_attribut::ZusatzAttribut;
pub use zustaendigkeit::Zustaendigkeit;
/// Sum type over **all** BO4E Geschäftsobjekte for dynamic type dispatch.
///
/// Use this when you receive a JSON message where the concrete BO type is
/// determined at runtime by the `"_typ"` discriminant field.
///
/// Deserialization requires the `json` feature (uses `serde_json::Value` for
/// the two-pass `_typ` peeking strategy).  Serialization requires only `serde`.
///
/// # Example
/// ```rust,ignore
/// use rubo4e::v202501::AnyBo;
/// let json = r#"{"_typ":"MARKTLOKATION","marktlokationsId":"51238696780"}"#;
/// let bo: AnyBo = serde_json::from_str(json)?;
/// if let AnyBo::Marktlokation(malo) = bo {
///     println!("ID: {:?}", malo.marktlokations_id);
/// }
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(not(feature = "json"), derive(Hash))]
#[non_exhaustive]
pub enum AnyBo {
    /// A [`Angebot`] Geschäftsobjekt.
    Angebot(Box<Angebot>),
    /// A [`Ausschreibung`] Geschäftsobjekt.
    Ausschreibung(Box<Ausschreibung>),
    /// A [`Bilanzierung`] Geschäftsobjekt.
    Bilanzierung(Box<Bilanzierung>),
    /// A [`Buendelvertrag`] Geschäftsobjekt.
    Buendelvertrag(Box<Buendelvertrag>),
    /// A [`Energiemenge`] Geschäftsobjekt.
    Energiemenge(Box<Energiemenge>),
    /// A [`Fremdkosten`] Geschäftsobjekt.
    Fremdkosten(Box<Fremdkosten>),
    /// A [`Geraet`] Geschäftsobjekt.
    Geraet(Box<Geraet>),
    /// A [`Geschaeftspartner`] Geschäftsobjekt.
    Geschaeftspartner(Box<Geschaeftspartner>),
    /// A [`Kosten`] Geschäftsobjekt.
    Kosten(Box<Kosten>),
    /// A [`Lastgang`] Geschäftsobjekt.
    Lastgang(Box<Lastgang>),
    /// A [`Lokationszuordnung`] Geschäftsobjekt.
    Lokationszuordnung(Box<Lokationszuordnung>),
    /// A [`Marktlokation`] Geschäftsobjekt.
    Marktlokation(Box<Marktlokation>),
    /// A [`Marktteilnehmer`] Geschäftsobjekt.
    Marktteilnehmer(Box<Marktteilnehmer>),
    /// A [`Messlokation`] Geschäftsobjekt.
    Messlokation(Box<Messlokation>),
    /// A [`Netzlokation`] Geschäftsobjekt.
    Netzlokation(Box<Netzlokation>),
    /// A [`Person`] Geschäftsobjekt.
    Person(Box<Person>),
    /// A [`Preisblatt`] Geschäftsobjekt.
    Preisblatt(Box<Preisblatt>),
    /// A [`PreisblattDienstleistung`] Geschäftsobjekt.
    PreisblattDienstleistung(Box<PreisblattDienstleistung>),
    /// A [`PreisblattHardware`] Geschäftsobjekt.
    PreisblattHardware(Box<PreisblattHardware>),
    /// A [`PreisblattKonzessionsabgabe`] Geschäftsobjekt.
    PreisblattKonzessionsabgabe(Box<PreisblattKonzessionsabgabe>),
    /// A [`PreisblattMessung`] Geschäftsobjekt.
    PreisblattMessung(Box<PreisblattMessung>),
    /// A [`PreisblattNetznutzung`] Geschäftsobjekt.
    PreisblattNetznutzung(Box<PreisblattNetznutzung>),
    /// A [`Rechnung`] Geschäftsobjekt.
    Rechnung(Box<Rechnung>),
    /// A [`Region`] Geschäftsobjekt.
    Region(Box<Region>),
    /// A [`Regionaltarif`] Geschäftsobjekt.
    Regionaltarif(Box<Regionaltarif>),
    /// A [`Standorteigenschaften`] Geschäftsobjekt.
    Standorteigenschaften(Box<Standorteigenschaften>),
    /// A [`SteuerbareRessource`] Geschäftsobjekt.
    SteuerbareRessource(Box<SteuerbareRessource>),
    /// A [`Tarif`] Geschäftsobjekt.
    Tarif(Box<Tarif>),
    /// A [`Tarifinfo`] Geschäftsobjekt.
    Tarifinfo(Box<Tarifinfo>),
    /// A [`Tarifkosten`] Geschäftsobjekt.
    Tarifkosten(Box<Tarifkosten>),
    /// A [`Tarifpreisblatt`] Geschäftsobjekt.
    Tarifpreisblatt(Box<Tarifpreisblatt>),
    /// A [`TechnischeRessource`] Geschäftsobjekt.
    TechnischeRessource(Box<TechnischeRessource>),
    /// A [`Vertrag`] Geschäftsobjekt.
    Vertrag(Box<Vertrag>),
    /// A [`Zaehler`] Geschäftsobjekt.
    Zaehler(Box<Zaehler>),
    /// A [`Zeitreihe`] Geschäftsobjekt.
    Zeitreihe(Box<Zeitreihe>),
    /// Unrecognized `_typ` value — raw JSON preserved for forward-compatibility.
    ///
    /// Produced when the `_typ` string is not matched by any known variant.
    /// Allows graceful handling of new BO types without a library upgrade.
    #[cfg(feature = "json")]
    Unknown {
        /// The raw value of the `_typ` field.
        typ: String,
        /// The full JSON object for inspection or re-serialization.
        data: serde_json::Value,
    },
}
impl AnyBo {
    /// Returns the [`BoTyp`] discriminant for this BO object.
    ///
    /// Delegates to the inner type's [`Bo4eObject::bo_type`] for all known
    /// variants; returns [`BoTyp::Unknown`] for the `Unknown` catch-all.
    pub fn bo_type(&self) -> BoTyp {
        match self {
            AnyBo::Angebot(v) => v.bo_type(),
            AnyBo::Ausschreibung(v) => v.bo_type(),
            AnyBo::Bilanzierung(v) => v.bo_type(),
            AnyBo::Buendelvertrag(v) => v.bo_type(),
            AnyBo::Energiemenge(v) => v.bo_type(),
            AnyBo::Fremdkosten(v) => v.bo_type(),
            AnyBo::Geraet(v) => v.bo_type(),
            AnyBo::Geschaeftspartner(v) => v.bo_type(),
            AnyBo::Kosten(v) => v.bo_type(),
            AnyBo::Lastgang(v) => v.bo_type(),
            AnyBo::Lokationszuordnung(v) => v.bo_type(),
            AnyBo::Marktlokation(v) => v.bo_type(),
            AnyBo::Marktteilnehmer(v) => v.bo_type(),
            AnyBo::Messlokation(v) => v.bo_type(),
            AnyBo::Netzlokation(v) => v.bo_type(),
            AnyBo::Person(v) => v.bo_type(),
            AnyBo::Preisblatt(v) => v.bo_type(),
            AnyBo::PreisblattDienstleistung(v) => v.bo_type(),
            AnyBo::PreisblattHardware(v) => v.bo_type(),
            AnyBo::PreisblattKonzessionsabgabe(v) => v.bo_type(),
            AnyBo::PreisblattMessung(v) => v.bo_type(),
            AnyBo::PreisblattNetznutzung(v) => v.bo_type(),
            AnyBo::Rechnung(v) => v.bo_type(),
            AnyBo::Region(v) => v.bo_type(),
            AnyBo::Regionaltarif(v) => v.bo_type(),
            AnyBo::Standorteigenschaften(v) => v.bo_type(),
            AnyBo::SteuerbareRessource(v) => v.bo_type(),
            AnyBo::Tarif(v) => v.bo_type(),
            AnyBo::Tarifinfo(v) => v.bo_type(),
            AnyBo::Tarifkosten(v) => v.bo_type(),
            AnyBo::Tarifpreisblatt(v) => v.bo_type(),
            AnyBo::TechnischeRessource(v) => v.bo_type(),
            AnyBo::Vertrag(v) => v.bo_type(),
            AnyBo::Zaehler(v) => v.bo_type(),
            AnyBo::Zeitreihe(v) => v.bo_type(),
            #[cfg(feature = "json")]
            AnyBo::Unknown { .. } => BoTyp::Unknown,
        }
    }
}
impl From<Angebot> for AnyBo {
    fn from(v: Angebot) -> Self {
        AnyBo::Angebot(Box::new(v))
    }
}
impl From<Box<Angebot>> for AnyBo {
    fn from(v: Box<Angebot>) -> Self {
        AnyBo::Angebot(v)
    }
}
impl From<Ausschreibung> for AnyBo {
    fn from(v: Ausschreibung) -> Self {
        AnyBo::Ausschreibung(Box::new(v))
    }
}
impl From<Box<Ausschreibung>> for AnyBo {
    fn from(v: Box<Ausschreibung>) -> Self {
        AnyBo::Ausschreibung(v)
    }
}
impl From<Bilanzierung> for AnyBo {
    fn from(v: Bilanzierung) -> Self {
        AnyBo::Bilanzierung(Box::new(v))
    }
}
impl From<Box<Bilanzierung>> for AnyBo {
    fn from(v: Box<Bilanzierung>) -> Self {
        AnyBo::Bilanzierung(v)
    }
}
impl From<Buendelvertrag> for AnyBo {
    fn from(v: Buendelvertrag) -> Self {
        AnyBo::Buendelvertrag(Box::new(v))
    }
}
impl From<Box<Buendelvertrag>> for AnyBo {
    fn from(v: Box<Buendelvertrag>) -> Self {
        AnyBo::Buendelvertrag(v)
    }
}
impl From<Energiemenge> for AnyBo {
    fn from(v: Energiemenge) -> Self {
        AnyBo::Energiemenge(Box::new(v))
    }
}
impl From<Box<Energiemenge>> for AnyBo {
    fn from(v: Box<Energiemenge>) -> Self {
        AnyBo::Energiemenge(v)
    }
}
impl From<Fremdkosten> for AnyBo {
    fn from(v: Fremdkosten) -> Self {
        AnyBo::Fremdkosten(Box::new(v))
    }
}
impl From<Box<Fremdkosten>> for AnyBo {
    fn from(v: Box<Fremdkosten>) -> Self {
        AnyBo::Fremdkosten(v)
    }
}
impl From<Geraet> for AnyBo {
    fn from(v: Geraet) -> Self {
        AnyBo::Geraet(Box::new(v))
    }
}
impl From<Box<Geraet>> for AnyBo {
    fn from(v: Box<Geraet>) -> Self {
        AnyBo::Geraet(v)
    }
}
impl From<Geschaeftspartner> for AnyBo {
    fn from(v: Geschaeftspartner) -> Self {
        AnyBo::Geschaeftspartner(Box::new(v))
    }
}
impl From<Box<Geschaeftspartner>> for AnyBo {
    fn from(v: Box<Geschaeftspartner>) -> Self {
        AnyBo::Geschaeftspartner(v)
    }
}
impl From<Kosten> for AnyBo {
    fn from(v: Kosten) -> Self {
        AnyBo::Kosten(Box::new(v))
    }
}
impl From<Box<Kosten>> for AnyBo {
    fn from(v: Box<Kosten>) -> Self {
        AnyBo::Kosten(v)
    }
}
impl From<Lastgang> for AnyBo {
    fn from(v: Lastgang) -> Self {
        AnyBo::Lastgang(Box::new(v))
    }
}
impl From<Box<Lastgang>> for AnyBo {
    fn from(v: Box<Lastgang>) -> Self {
        AnyBo::Lastgang(v)
    }
}
impl From<Lokationszuordnung> for AnyBo {
    fn from(v: Lokationszuordnung) -> Self {
        AnyBo::Lokationszuordnung(Box::new(v))
    }
}
impl From<Box<Lokationszuordnung>> for AnyBo {
    fn from(v: Box<Lokationszuordnung>) -> Self {
        AnyBo::Lokationszuordnung(v)
    }
}
impl From<Marktlokation> for AnyBo {
    fn from(v: Marktlokation) -> Self {
        AnyBo::Marktlokation(Box::new(v))
    }
}
impl From<Box<Marktlokation>> for AnyBo {
    fn from(v: Box<Marktlokation>) -> Self {
        AnyBo::Marktlokation(v)
    }
}
impl From<Marktteilnehmer> for AnyBo {
    fn from(v: Marktteilnehmer) -> Self {
        AnyBo::Marktteilnehmer(Box::new(v))
    }
}
impl From<Box<Marktteilnehmer>> for AnyBo {
    fn from(v: Box<Marktteilnehmer>) -> Self {
        AnyBo::Marktteilnehmer(v)
    }
}
impl From<Messlokation> for AnyBo {
    fn from(v: Messlokation) -> Self {
        AnyBo::Messlokation(Box::new(v))
    }
}
impl From<Box<Messlokation>> for AnyBo {
    fn from(v: Box<Messlokation>) -> Self {
        AnyBo::Messlokation(v)
    }
}
impl From<Netzlokation> for AnyBo {
    fn from(v: Netzlokation) -> Self {
        AnyBo::Netzlokation(Box::new(v))
    }
}
impl From<Box<Netzlokation>> for AnyBo {
    fn from(v: Box<Netzlokation>) -> Self {
        AnyBo::Netzlokation(v)
    }
}
impl From<Person> for AnyBo {
    fn from(v: Person) -> Self {
        AnyBo::Person(Box::new(v))
    }
}
impl From<Box<Person>> for AnyBo {
    fn from(v: Box<Person>) -> Self {
        AnyBo::Person(v)
    }
}
impl From<Preisblatt> for AnyBo {
    fn from(v: Preisblatt) -> Self {
        AnyBo::Preisblatt(Box::new(v))
    }
}
impl From<Box<Preisblatt>> for AnyBo {
    fn from(v: Box<Preisblatt>) -> Self {
        AnyBo::Preisblatt(v)
    }
}
impl From<PreisblattDienstleistung> for AnyBo {
    fn from(v: PreisblattDienstleistung) -> Self {
        AnyBo::PreisblattDienstleistung(Box::new(v))
    }
}
impl From<Box<PreisblattDienstleistung>> for AnyBo {
    fn from(v: Box<PreisblattDienstleistung>) -> Self {
        AnyBo::PreisblattDienstleistung(v)
    }
}
impl From<PreisblattHardware> for AnyBo {
    fn from(v: PreisblattHardware) -> Self {
        AnyBo::PreisblattHardware(Box::new(v))
    }
}
impl From<Box<PreisblattHardware>> for AnyBo {
    fn from(v: Box<PreisblattHardware>) -> Self {
        AnyBo::PreisblattHardware(v)
    }
}
impl From<PreisblattKonzessionsabgabe> for AnyBo {
    fn from(v: PreisblattKonzessionsabgabe) -> Self {
        AnyBo::PreisblattKonzessionsabgabe(Box::new(v))
    }
}
impl From<Box<PreisblattKonzessionsabgabe>> for AnyBo {
    fn from(v: Box<PreisblattKonzessionsabgabe>) -> Self {
        AnyBo::PreisblattKonzessionsabgabe(v)
    }
}
impl From<PreisblattMessung> for AnyBo {
    fn from(v: PreisblattMessung) -> Self {
        AnyBo::PreisblattMessung(Box::new(v))
    }
}
impl From<Box<PreisblattMessung>> for AnyBo {
    fn from(v: Box<PreisblattMessung>) -> Self {
        AnyBo::PreisblattMessung(v)
    }
}
impl From<PreisblattNetznutzung> for AnyBo {
    fn from(v: PreisblattNetznutzung) -> Self {
        AnyBo::PreisblattNetznutzung(Box::new(v))
    }
}
impl From<Box<PreisblattNetznutzung>> for AnyBo {
    fn from(v: Box<PreisblattNetznutzung>) -> Self {
        AnyBo::PreisblattNetznutzung(v)
    }
}
impl From<Rechnung> for AnyBo {
    fn from(v: Rechnung) -> Self {
        AnyBo::Rechnung(Box::new(v))
    }
}
impl From<Box<Rechnung>> for AnyBo {
    fn from(v: Box<Rechnung>) -> Self {
        AnyBo::Rechnung(v)
    }
}
impl From<Region> for AnyBo {
    fn from(v: Region) -> Self {
        AnyBo::Region(Box::new(v))
    }
}
impl From<Box<Region>> for AnyBo {
    fn from(v: Box<Region>) -> Self {
        AnyBo::Region(v)
    }
}
impl From<Regionaltarif> for AnyBo {
    fn from(v: Regionaltarif) -> Self {
        AnyBo::Regionaltarif(Box::new(v))
    }
}
impl From<Box<Regionaltarif>> for AnyBo {
    fn from(v: Box<Regionaltarif>) -> Self {
        AnyBo::Regionaltarif(v)
    }
}
impl From<Standorteigenschaften> for AnyBo {
    fn from(v: Standorteigenschaften) -> Self {
        AnyBo::Standorteigenschaften(Box::new(v))
    }
}
impl From<Box<Standorteigenschaften>> for AnyBo {
    fn from(v: Box<Standorteigenschaften>) -> Self {
        AnyBo::Standorteigenschaften(v)
    }
}
impl From<SteuerbareRessource> for AnyBo {
    fn from(v: SteuerbareRessource) -> Self {
        AnyBo::SteuerbareRessource(Box::new(v))
    }
}
impl From<Box<SteuerbareRessource>> for AnyBo {
    fn from(v: Box<SteuerbareRessource>) -> Self {
        AnyBo::SteuerbareRessource(v)
    }
}
impl From<Tarif> for AnyBo {
    fn from(v: Tarif) -> Self {
        AnyBo::Tarif(Box::new(v))
    }
}
impl From<Box<Tarif>> for AnyBo {
    fn from(v: Box<Tarif>) -> Self {
        AnyBo::Tarif(v)
    }
}
impl From<Tarifinfo> for AnyBo {
    fn from(v: Tarifinfo) -> Self {
        AnyBo::Tarifinfo(Box::new(v))
    }
}
impl From<Box<Tarifinfo>> for AnyBo {
    fn from(v: Box<Tarifinfo>) -> Self {
        AnyBo::Tarifinfo(v)
    }
}
impl From<Tarifkosten> for AnyBo {
    fn from(v: Tarifkosten) -> Self {
        AnyBo::Tarifkosten(Box::new(v))
    }
}
impl From<Box<Tarifkosten>> for AnyBo {
    fn from(v: Box<Tarifkosten>) -> Self {
        AnyBo::Tarifkosten(v)
    }
}
impl From<Tarifpreisblatt> for AnyBo {
    fn from(v: Tarifpreisblatt) -> Self {
        AnyBo::Tarifpreisblatt(Box::new(v))
    }
}
impl From<Box<Tarifpreisblatt>> for AnyBo {
    fn from(v: Box<Tarifpreisblatt>) -> Self {
        AnyBo::Tarifpreisblatt(v)
    }
}
impl From<TechnischeRessource> for AnyBo {
    fn from(v: TechnischeRessource) -> Self {
        AnyBo::TechnischeRessource(Box::new(v))
    }
}
impl From<Box<TechnischeRessource>> for AnyBo {
    fn from(v: Box<TechnischeRessource>) -> Self {
        AnyBo::TechnischeRessource(v)
    }
}
impl From<Vertrag> for AnyBo {
    fn from(v: Vertrag) -> Self {
        AnyBo::Vertrag(Box::new(v))
    }
}
impl From<Box<Vertrag>> for AnyBo {
    fn from(v: Box<Vertrag>) -> Self {
        AnyBo::Vertrag(v)
    }
}
impl From<Zaehler> for AnyBo {
    fn from(v: Zaehler) -> Self {
        AnyBo::Zaehler(Box::new(v))
    }
}
impl From<Box<Zaehler>> for AnyBo {
    fn from(v: Box<Zaehler>) -> Self {
        AnyBo::Zaehler(v)
    }
}
impl From<Zeitreihe> for AnyBo {
    fn from(v: Zeitreihe) -> Self {
        AnyBo::Zeitreihe(Box::new(v))
    }
}
impl From<Box<Zeitreihe>> for AnyBo {
    fn from(v: Box<Zeitreihe>) -> Self {
        AnyBo::Zeitreihe(v)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for AnyBo {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self {
            AnyBo::Angebot(inner) => inner.serialize(s),
            AnyBo::Ausschreibung(inner) => inner.serialize(s),
            AnyBo::Bilanzierung(inner) => inner.serialize(s),
            AnyBo::Buendelvertrag(inner) => inner.serialize(s),
            AnyBo::Energiemenge(inner) => inner.serialize(s),
            AnyBo::Fremdkosten(inner) => inner.serialize(s),
            AnyBo::Geraet(inner) => inner.serialize(s),
            AnyBo::Geschaeftspartner(inner) => inner.serialize(s),
            AnyBo::Kosten(inner) => inner.serialize(s),
            AnyBo::Lastgang(inner) => inner.serialize(s),
            AnyBo::Lokationszuordnung(inner) => inner.serialize(s),
            AnyBo::Marktlokation(inner) => inner.serialize(s),
            AnyBo::Marktteilnehmer(inner) => inner.serialize(s),
            AnyBo::Messlokation(inner) => inner.serialize(s),
            AnyBo::Netzlokation(inner) => inner.serialize(s),
            AnyBo::Person(inner) => inner.serialize(s),
            AnyBo::Preisblatt(inner) => inner.serialize(s),
            AnyBo::PreisblattDienstleistung(inner) => inner.serialize(s),
            AnyBo::PreisblattHardware(inner) => inner.serialize(s),
            AnyBo::PreisblattKonzessionsabgabe(inner) => inner.serialize(s),
            AnyBo::PreisblattMessung(inner) => inner.serialize(s),
            AnyBo::PreisblattNetznutzung(inner) => inner.serialize(s),
            AnyBo::Rechnung(inner) => inner.serialize(s),
            AnyBo::Region(inner) => inner.serialize(s),
            AnyBo::Regionaltarif(inner) => inner.serialize(s),
            AnyBo::Standorteigenschaften(inner) => inner.serialize(s),
            AnyBo::SteuerbareRessource(inner) => inner.serialize(s),
            AnyBo::Tarif(inner) => inner.serialize(s),
            AnyBo::Tarifinfo(inner) => inner.serialize(s),
            AnyBo::Tarifkosten(inner) => inner.serialize(s),
            AnyBo::Tarifpreisblatt(inner) => inner.serialize(s),
            AnyBo::TechnischeRessource(inner) => inner.serialize(s),
            AnyBo::Vertrag(inner) => inner.serialize(s),
            AnyBo::Zaehler(inner) => inner.serialize(s),
            AnyBo::Zeitreihe(inner) => inner.serialize(s),
            #[cfg(feature = "json")]
            AnyBo::Unknown { data, .. } => data.serialize(s),
        }
    }
}

/// Extracts the `_typ` field value from a raw JSON object string without
/// building a full [`serde_json::Value`] tree.
///
/// Uses a simple linear scan that honours JSON string escaping.  Designed for
/// BO4E payloads where `_typ` appears early in the object (typically first or
/// second field).
///
/// Returns `""` if `_typ` is absent, null, or not a string.
///
/// Scans at depth-1 only: stops tracking once a `{` or `[` is seen inside a
/// value (and resumes only when the matching closer is found).  This prevents
/// a `"_typ"` key embedded inside a nested object from being returned instead
/// of the top-level one.
#[cfg(all(feature = "serde", feature = "json"))]
fn peek_typ_field(raw: &str) -> &str {
    const NEEDLE: &str = "\"_typ\"";
    let bytes = raw.as_bytes();
    let mut i = 0;
    let mut depth: usize = 0;
    let mut in_string = false;

    while i < bytes.len() {
        let b = bytes[i];
        if in_string {
            if b == b'\\' {
                i += 2;
                continue;
            } else if b == b'"' {
                in_string = false;
            }
        } else {
            match b {
                b'"' => {
                    // Only scan for `"_typ"` key at depth 1 (top-level object).
                    if depth == 1 {
                        let rest = &raw[i..];
                        if rest.starts_with(NEEDLE) {
                            // Skip past the key and any whitespace/colon.
                            let after_key = rest.strip_prefix(NEEDLE).unwrap_or_default();
                            let after_colon = after_key
                                .trim_start_matches([' ', '\t', '\n', '\r'])
                                .trim_start_matches(':')
                                .trim_start_matches([' ', '\t', '\n', '\r']);
                            if !after_colon.starts_with('"') {
                                return "";
                            }
                            let value_start = &after_colon[1..];
                            let vbytes = value_start.as_bytes();
                            let mut j = 0;
                            while j < vbytes.len() {
                                match vbytes[j] {
                                    b'\\' => j += 2,
                                    b'"' => return &value_start[..j],
                                    _ => j += 1,
                                }
                            }
                            return "";
                        }
                    }
                    in_string = true;
                }
                b'{' | b'[' => depth += 1,
                b'}' | b']' => depth = depth.saturating_sub(1),
                _ => {}
            }
        }
        i += 1;
    }
    ""
}

#[cfg(all(feature = "serde", feature = "json"))]
impl<'de> serde::Deserialize<'de> for AnyBo {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        // C-02 fix: single-pass deserialization via RawValue.
        //
        // The old two-pass approach materialized the full serde_json::Value tree,
        // extracted `_typ`, then called from_value() — double-allocating every payload.
        // This approach instead:
        //   1. Captures the raw JSON bytes as a RawValue (zero-parse, just a string copy).
        //   2. Peeks at `_typ` via a fast linear scan of the raw string.
        //   3. Parses directly from raw.get() into the concrete type in one pass.
        // For the Unknown variant the full Value tree is still needed for data preservation.
        let raw = Box::<serde_json::value::RawValue>::deserialize(d)?;
        let typ_str = peek_typ_field(raw.get());
        match typ_str {
            "ANGEBOT" => serde_json::from_str::<Angebot>(raw.get())
                .map(|v| AnyBo::Angebot(Box::new(v)))
                .map_err(serde::de::Error::custom),
            "AUSSCHREIBUNG" => serde_json::from_str::<Ausschreibung>(raw.get())
                .map(|v| AnyBo::Ausschreibung(Box::new(v)))
                .map_err(serde::de::Error::custom),
            "BILANZIERUNG" => serde_json::from_str::<Bilanzierung>(raw.get())
                .map(|v| AnyBo::Bilanzierung(Box::new(v)))
                .map_err(serde::de::Error::custom),
            "BUENDELVERTRAG" => serde_json::from_str::<Buendelvertrag>(raw.get())
                .map(|v| AnyBo::Buendelvertrag(Box::new(v)))
                .map_err(serde::de::Error::custom),
            "ENERGIEMENGE" => serde_json::from_str::<Energiemenge>(raw.get())
                .map(|v| AnyBo::Energiemenge(Box::new(v)))
                .map_err(serde::de::Error::custom),
            "FREMDKOSTEN" => serde_json::from_str::<Fremdkosten>(raw.get())
                .map(|v| AnyBo::Fremdkosten(Box::new(v)))
                .map_err(serde::de::Error::custom),
            "GERAET" => serde_json::from_str::<Geraet>(raw.get())
                .map(|v| AnyBo::Geraet(Box::new(v)))
                .map_err(serde::de::Error::custom),
            "GESCHAEFTSPARTNER" => serde_json::from_str::<Geschaeftspartner>(raw.get())
                .map(|v| AnyBo::Geschaeftspartner(Box::new(v)))
                .map_err(serde::de::Error::custom),
            "KOSTEN" => serde_json::from_str::<Kosten>(raw.get())
                .map(|v| AnyBo::Kosten(Box::new(v)))
                .map_err(serde::de::Error::custom),
            "LASTGANG" => serde_json::from_str::<Lastgang>(raw.get())
                .map(|v| AnyBo::Lastgang(Box::new(v)))
                .map_err(serde::de::Error::custom),
            "LOKATIONSZUORDNUNG" => serde_json::from_str::<Lokationszuordnung>(raw.get())
                .map(|v| AnyBo::Lokationszuordnung(Box::new(v)))
                .map_err(serde::de::Error::custom),
            "MARKTLOKATION" => serde_json::from_str::<Marktlokation>(raw.get())
                .map(|v| AnyBo::Marktlokation(Box::new(v)))
                .map_err(serde::de::Error::custom),
            "MARKTTEILNEHMER" => serde_json::from_str::<Marktteilnehmer>(raw.get())
                .map(|v| AnyBo::Marktteilnehmer(Box::new(v)))
                .map_err(serde::de::Error::custom),
            "MESSLOKATION" => serde_json::from_str::<Messlokation>(raw.get())
                .map(|v| AnyBo::Messlokation(Box::new(v)))
                .map_err(serde::de::Error::custom),
            "NETZLOKATION" => serde_json::from_str::<Netzlokation>(raw.get())
                .map(|v| AnyBo::Netzlokation(Box::new(v)))
                .map_err(serde::de::Error::custom),
            "PERSON" => serde_json::from_str::<Person>(raw.get())
                .map(|v| AnyBo::Person(Box::new(v)))
                .map_err(serde::de::Error::custom),
            "PREISBLATT" => serde_json::from_str::<Preisblatt>(raw.get())
                .map(|v| AnyBo::Preisblatt(Box::new(v)))
                .map_err(serde::de::Error::custom),
            "PREISBLATTDIENSTLEISTUNG" => {
                serde_json::from_str::<PreisblattDienstleistung>(raw.get())
                    .map(|v| AnyBo::PreisblattDienstleistung(Box::new(v)))
                    .map_err(serde::de::Error::custom)
            }
            "PREISBLATTHARDWARE" => serde_json::from_str::<PreisblattHardware>(raw.get())
                .map(|v| AnyBo::PreisblattHardware(Box::new(v)))
                .map_err(serde::de::Error::custom),
            "PREISBLATTKONZESSIONSABGABE" => {
                serde_json::from_str::<PreisblattKonzessionsabgabe>(raw.get())
                    .map(|v| AnyBo::PreisblattKonzessionsabgabe(Box::new(v)))
                    .map_err(serde::de::Error::custom)
            }
            "PREISBLATTMESSUNG" => serde_json::from_str::<PreisblattMessung>(raw.get())
                .map(|v| AnyBo::PreisblattMessung(Box::new(v)))
                .map_err(serde::de::Error::custom),
            "PREISBLATTNETZNUTZUNG" => serde_json::from_str::<PreisblattNetznutzung>(raw.get())
                .map(|v| AnyBo::PreisblattNetznutzung(Box::new(v)))
                .map_err(serde::de::Error::custom),
            "RECHNUNG" => serde_json::from_str::<Rechnung>(raw.get())
                .map(|v| AnyBo::Rechnung(Box::new(v)))
                .map_err(serde::de::Error::custom),
            "REGION" => serde_json::from_str::<Region>(raw.get())
                .map(|v| AnyBo::Region(Box::new(v)))
                .map_err(serde::de::Error::custom),
            "REGIONALTARIF" => serde_json::from_str::<Regionaltarif>(raw.get())
                .map(|v| AnyBo::Regionaltarif(Box::new(v)))
                .map_err(serde::de::Error::custom),
            "STANDORTEIGENSCHAFTEN" => serde_json::from_str::<Standorteigenschaften>(raw.get())
                .map(|v| AnyBo::Standorteigenschaften(Box::new(v)))
                .map_err(serde::de::Error::custom),
            "STEUERBARERESSOURCE" => serde_json::from_str::<SteuerbareRessource>(raw.get())
                .map(|v| AnyBo::SteuerbareRessource(Box::new(v)))
                .map_err(serde::de::Error::custom),
            "TARIF" => serde_json::from_str::<Tarif>(raw.get())
                .map(|v| AnyBo::Tarif(Box::new(v)))
                .map_err(serde::de::Error::custom),
            "TARIFINFO" => serde_json::from_str::<Tarifinfo>(raw.get())
                .map(|v| AnyBo::Tarifinfo(Box::new(v)))
                .map_err(serde::de::Error::custom),
            "TARIFKOSTEN" => serde_json::from_str::<Tarifkosten>(raw.get())
                .map(|v| AnyBo::Tarifkosten(Box::new(v)))
                .map_err(serde::de::Error::custom),
            "TARIFPREISBLATT" => serde_json::from_str::<Tarifpreisblatt>(raw.get())
                .map(|v| AnyBo::Tarifpreisblatt(Box::new(v)))
                .map_err(serde::de::Error::custom),
            "TECHNISCHERESSOURCE" => serde_json::from_str::<TechnischeRessource>(raw.get())
                .map(|v| AnyBo::TechnischeRessource(Box::new(v)))
                .map_err(serde::de::Error::custom),
            "VERTRAG" => serde_json::from_str::<Vertrag>(raw.get())
                .map(|v| AnyBo::Vertrag(Box::new(v)))
                .map_err(serde::de::Error::custom),
            "ZAEHLER" => serde_json::from_str::<Zaehler>(raw.get())
                .map(|v| AnyBo::Zaehler(Box::new(v)))
                .map_err(serde::de::Error::custom),
            "ZEITREIHE" => serde_json::from_str::<Zeitreihe>(raw.get())
                .map(|v| AnyBo::Zeitreihe(Box::new(v)))
                .map_err(serde::de::Error::custom),
            _ => {
                // Unknown _typ: preserve raw data as a full Value for forward-compat.
                let data = serde_json::from_str::<serde_json::Value>(raw.get())
                    .map_err(serde::de::Error::custom)?;
                Ok(AnyBo::Unknown {
                    typ: typ_str.to_owned(),
                    data,
                })
            }
        }
    }
}
#[cfg(feature = "json")]
impl crate::json::sealed::Sealed for AnyBo {}
#[cfg(feature = "json")]
impl crate::json::Bo4eJsonExt for AnyBo {}

// ── Bo4eObject sealed-trait impls ────────────────────────────────────────────
// These implement the sealing supertrait for all BO types that carry
// `impl Bo4eObject for Type`.  External crates cannot implement this trait.
#[cfg(feature = "versioned")]
impl crate::bo4e_object_sealed::Sealed for Angebot {}
impl crate::bo4e_object_sealed::Sealed for Ausschreibung {}
impl crate::bo4e_object_sealed::Sealed for Bilanzierung {}
impl crate::bo4e_object_sealed::Sealed for Buendelvertrag {}
impl crate::bo4e_object_sealed::Sealed for Energiemenge {}
impl crate::bo4e_object_sealed::Sealed for Fremdkosten {}
impl crate::bo4e_object_sealed::Sealed for Geraet {}
impl crate::bo4e_object_sealed::Sealed for Geschaeftspartner {}
impl crate::bo4e_object_sealed::Sealed for Kosten {}
impl crate::bo4e_object_sealed::Sealed for Lastgang {}
impl crate::bo4e_object_sealed::Sealed for Lokationszuordnung {}
impl crate::bo4e_object_sealed::Sealed for Marktlokation {}
impl crate::bo4e_object_sealed::Sealed for Marktteilnehmer {}
impl crate::bo4e_object_sealed::Sealed for Messlokation {}
impl crate::bo4e_object_sealed::Sealed for Netzlokation {}
impl crate::bo4e_object_sealed::Sealed for Person {}
impl crate::bo4e_object_sealed::Sealed for Preisblatt {}
impl crate::bo4e_object_sealed::Sealed for PreisblattDienstleistung {}
impl crate::bo4e_object_sealed::Sealed for PreisblattHardware {}
impl crate::bo4e_object_sealed::Sealed for PreisblattKonzessionsabgabe {}
impl crate::bo4e_object_sealed::Sealed for PreisblattMessung {}
impl crate::bo4e_object_sealed::Sealed for PreisblattNetznutzung {}
impl crate::bo4e_object_sealed::Sealed for Rechnung {}
impl crate::bo4e_object_sealed::Sealed for Region {}
impl crate::bo4e_object_sealed::Sealed for Regionaltarif {}
impl crate::bo4e_object_sealed::Sealed for Standorteigenschaften {}
impl crate::bo4e_object_sealed::Sealed for SteuerbareRessource {}
impl crate::bo4e_object_sealed::Sealed for Tarif {}
impl crate::bo4e_object_sealed::Sealed for Tarifinfo {}
impl crate::bo4e_object_sealed::Sealed for Tarifkosten {}
impl crate::bo4e_object_sealed::Sealed for Tarifpreisblatt {}
impl crate::bo4e_object_sealed::Sealed for TechnischeRessource {}
impl crate::bo4e_object_sealed::Sealed for Vertrag {}
impl crate::bo4e_object_sealed::Sealed for Zaehler {}
impl crate::bo4e_object_sealed::Sealed for Zeitreihe {}
