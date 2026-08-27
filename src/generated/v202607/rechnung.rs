use super::{
    Betrag, Bo4eObject, Bo4eTyped, BoTyp, Energiemenge, Fremdkosten, Geschaeftspartner,
    Marktlokation, Marktteilnehmer, Messlokation, NetznutzungRechnungsart, NetznutzungRechnungstyp,
    Rechnungsposition, Rechnungsstatus, Rechnungstyp, Sparte, Steuerbetrag, Vertrag, Vorauszahlung,
    Zaehler, Zahlungsinformation, Zeitraum, ZusatzAttribut,
};
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(not(feature = "json"), derive(Eq, Hash))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "validate", derive(garde::Validate))]
#[cfg_attr(feature = "validate", garde(allow_unvalidated))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(
    all(feature = "validate", feature = "versioned"),
    garde(custom(crate::validation::v202607::validate_rechnung_arithmetic))
)]
/// Modell für die Abbildung von Rechnungen und Netznutzungsrechnungen im Kontext der Energiewirtschaft;
///
/// > **Note:** [Rechnung JSON Schema](https://json-schema.app/view/%23?url=https://raw.githubusercontent.com/BO4E/BO4E-Schemas/v202607.1.0/src/bo4e_schemas/bo/Rechnung.json)
pub struct Rechnung {
    /// Verbrauch des abgerechneten Zeitraums, Pflicht für Rechnungen gemäß EnWG § 40
    #[cfg_attr(feature = "serde", serde(rename = "aktuellerVerbrauch"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub aktueller_verbrauch: Option<Box<Energiemenge>>,
    /// Für Verbrauchsbasierte Rechnungen der Zählerstand zur Beginn des abgerechneten Zeitraums, Pflicht für Rechnungen gemäß EnWG § 40
    #[cfg_attr(feature = "serde", serde(rename = "anfangszaehlerstand"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub anfangszaehlerstand: Option<Box<Energiemenge>>,
    /// Für Verbrauchsbasierte Rechnungen der Zählerstand zum Ende des abgerechneten Zeitraums, Pflicht für Rechnungen gemäß EnWG § 40
    #[cfg_attr(feature = "serde", serde(rename = "endzaehlerstand"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub endzaehlerstand: Option<Box<Energiemenge>>,
    /// Zu diesem Datum ist die Zahlung fällig
    #[cfg_attr(feature = "serde", serde(rename = "faelligkeitsdatum"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(
        feature = "schemars",
        schemars(schema_with = "crate::schema_helpers::opt_datetime_schema")
    )]
    #[cfg_attr(
        all(feature = "serde", feature = "time"),
        serde(with = "time::serde::rfc3339::option")
    )]
    #[cfg(feature = "time")]
    pub faelligkeitsdatum: Option<time::OffsetDateTime>,
    /// Zu diesem Datum ist die Zahlung fällig
    #[cfg_attr(feature = "serde", serde(rename = "faelligkeitsdatum"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(
        feature = "schemars",
        schemars(schema_with = "crate::schema_helpers::opt_datetime_schema")
    )]
    #[cfg(not(feature = "time"))]
    pub faelligkeitsdatum: Option<String>,
    /// Zur Ausweisung der in die Kalkulation eingeflossenen Preise gemäß EnWG § 40
    #[cfg_attr(feature = "serde", serde(rename = "fremdkosten"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub fremdkosten: Option<Box<Fremdkosten>>,
    /// Die Summe aus Netto- und Steuerbetrag
    #[cfg_attr(feature = "serde", serde(rename = "gesamtbrutto"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub gesamtbrutto: Option<Betrag>,
    /// Die Summe der Nettobeträge der Rechnungsteile
    #[cfg_attr(feature = "serde", serde(rename = "gesamtnetto"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub gesamtnetto: Option<Betrag>,
    /// Die Summe der Steuerbeträge der Rechnungsteile
    #[cfg_attr(feature = "serde", serde(rename = "gesamtsteuer"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub gesamtsteuer: Option<Betrag>,
    /// Eine generische ID, die für eigene Zwecke genutzt werden kann.
    /// Z.B. könnten hier UUIDs aus einer Datenbank stehen oder URLs zu einem Backend-System.
    #[cfg_attr(feature = "serde", serde(rename = "_id"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub id: Option<String>,
    /// Kennzeichen, ob es sich um ein Original (true) oder eine Kopie handelt (false)
    #[cfg_attr(feature = "serde", serde(rename = "istOriginal"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub ist_original: Option<bool>,
    /// Kennzeichen, ob es sich um eine simulierte Rechnung, z.B. zur Rechnungsprüfung handelt
    #[cfg_attr(feature = "serde", serde(rename = "istSimuliert"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub ist_simuliert: Option<bool>,
    /// Kennzeichnung, ob es sich um eine Stornorechnung handelt;
    /// im Falle "true" findet sich im Attribut "originalrechnungsnummer" die Nummer der Originalrechnung.
    #[cfg_attr(feature = "serde", serde(rename = "istStorno"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub ist_storno: Option<bool>,
    /// ggf. auf einen Vergleichszeitraum hochgerechneter Verbrauch des abgerechneten Zeitraums zu Vergleichszwecken mit dem Vorjahr, gemäß EnWG § 40
    #[cfg_attr(feature = "serde", serde(rename = "jahresverbrauch"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub jahresverbrauch: Option<Box<Energiemenge>>,
    #[cfg_attr(feature = "serde", serde(rename = "kaeuferreferenz"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub kaeuferreferenz: Option<String>,
    /// Marktlokation, auf die sich die Rechnung bezieht
    #[cfg_attr(feature = "serde", serde(rename = "marktlokation"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub marktlokation: Option<Box<Marktlokation>>,
    /// Messlokation, auf die sich die Rechnung bezieht
    #[cfg_attr(feature = "serde", serde(rename = "messlokation"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub messlokation: Option<Box<Messlokation>>,
    /// der Messtellenbetreiber an der Lieferstelle, relevant für Rechnungen gemäß EnWG § 40
    #[cfg_attr(feature = "serde", serde(rename = "messstellenbetreiber"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub messstellenbetreiber: Option<Box<Marktteilnehmer>>,
    /// der Netzbetreiber an der Lieferstelle, relevant für Rechnungen gemäß EnWG § 40
    #[cfg_attr(feature = "serde", serde(rename = "netzbetreiber"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub netzbetreiber: Option<Box<Marktteilnehmer>>,
    /// Aus der INVOIC entnommen, befüllt wenn es sich um eine Netznutzungsrechnung handelt
    #[cfg_attr(feature = "serde", serde(rename = "netznutzungrechnungsart"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub netznutzungrechnungsart: Option<NetznutzungRechnungsart>,
    /// Aus der INVOIC entnommen, befüllt wenn es sich um eine Netznutzungsrechnung handelt
    #[cfg_attr(feature = "serde", serde(rename = "netznutzungrechnungstyp"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub netznutzungrechnungstyp: Option<NetznutzungRechnungstyp>,
    /// Im Falle einer Stornorechnung (storno = true) steht hier die Rechnungsnummer der stornierten Rechnung
    #[cfg_attr(feature = "serde", serde(rename = "originalRechnungsnummer"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub original_rechnungsnummer: Option<String>,
    /// Gesamtrabatt auf den Nettobetrag
    #[cfg_attr(feature = "serde", serde(rename = "rabattNetto"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub rabatt_netto: Option<Betrag>,
    /// Ausstellungsdatum der Rechnung
    #[cfg_attr(feature = "serde", serde(rename = "rechnungsdatum"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(
        feature = "schemars",
        schemars(schema_with = "crate::schema_helpers::opt_datetime_schema")
    )]
    #[cfg_attr(
        all(feature = "serde", feature = "time"),
        serde(with = "time::serde::rfc3339::option")
    )]
    #[cfg(feature = "time")]
    pub rechnungsdatum: Option<time::OffsetDateTime>,
    /// Ausstellungsdatum der Rechnung
    #[cfg_attr(feature = "serde", serde(rename = "rechnungsdatum"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(
        feature = "schemars",
        schemars(schema_with = "crate::schema_helpers::opt_datetime_schema")
    )]
    #[cfg(not(feature = "time"))]
    pub rechnungsdatum: Option<String>,
    /// Der Aussteller der Rechnung. Die Rollencodenummer kennt man über den im Geschäftspartner verlinkten Marktteilnehmer.
    #[cfg_attr(feature = "serde", serde(rename = "rechnungsempfaenger"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub rechnungsempfaenger: Option<Box<Geschaeftspartner>>,
    /// Der Aussteller der Rechnung. Die Rollencodenummer kennt man über den im Geschäftspartner verlinkten Marktteilnehmer.
    #[cfg_attr(feature = "serde", serde(rename = "rechnungsersteller"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub rechnungsersteller: Option<Box<Geschaeftspartner>>,
    /// Eine im Verwendungskontext eindeutige Nummer für die Rechnung
    #[cfg_attr(feature = "serde", serde(rename = "rechnungsnummer"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub rechnungsnummer: Option<String>,
    /// Der Zeitraum der zugrunde liegenden Lieferung zur Rechnung
    #[cfg_attr(feature = "serde", serde(rename = "rechnungsperiode"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub rechnungsperiode: Option<Zeitraum>,
    /// Die Rechnungspositionen
    #[cfg_attr(feature = "serde", serde(rename = "rechnungspositionen"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub rechnungspositionen: Option<Vec<Rechnungsposition>>,
    /// Status der Rechnung zur Kennzeichnung des Bearbeitungsstandes
    #[cfg_attr(feature = "serde", serde(rename = "rechnungsstatus"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub rechnungsstatus: Option<Rechnungsstatus>,
    /// Bezeichnung für die vorliegende Rechnung
    #[cfg_attr(feature = "serde", serde(rename = "rechnungstitel"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub rechnungstitel: Option<String>,
    /// Ein kontextbezogender Rechnungstyp, z.B. Netznutzungsrechnung
    #[cfg_attr(feature = "serde", serde(rename = "rechnungstyp"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub rechnungstyp: Option<Rechnungstyp>,
    /// Verbräuche von Referenzkundengruppen gemäß EnWG § 40
    #[cfg_attr(feature = "serde", serde(rename = "referenzverbraeuche"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub referenzverbraeuche: Option<Vec<Box<Energiemenge>>>,
    /// Sparte (Strom, Gas ...) für die die Rechnung ausgestellt ist
    #[cfg_attr(feature = "serde", serde(rename = "sparte"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub sparte: Option<Sparte>,
    /// Eine Liste mit Steuerbeträgen pro Steuerkennzeichen/Steuersatz;
    /// die Summe dieser Beträge ergibt den Wert für gesamtsteuer.
    #[cfg_attr(feature = "serde", serde(rename = "steuerbetraege"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub steuerbetraege: Option<Vec<Steuerbetrag>>,
    /// Rechnungen, die durch diese Rechnung zusammengefasst werden
    #[cfg_attr(feature = "serde", serde(rename = "teilrechnungen"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub teilrechnungen: Option<Vec<Box<Rechnung>>>,
    /// BO4E type discriminant — always `BoTyp::Rechnung` for this struct.
    #[cfg_attr(feature = "serde", serde(rename = "_typ"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(
        feature = "builder",
        builder(default = Some(BoTyp::Rechnung), setter(skip))
    )]
    pub typ: Option<BoTyp>,
    /// Version der BO-Struktur aka "fachliche Versionierung"
    #[cfg_attr(feature = "serde", serde(rename = "_version"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(
        feature = "builder",
        builder(default = Some("202607.1.0".to_owned()), setter(into))
    )]
    pub version: Option<String>,
    /// enthält Informationen über den der Rechnung zugrundeliegenden Vertrag für Rechnungen nach EnWG § 40
    #[cfg_attr(feature = "serde", serde(rename = "vertrag"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub vertrag: Option<Box<Vertrag>>,
    /// Die Summe evtl. vorausgezahlter Beträge, z.B. Abschläge. Angabe als Bruttowert
    #[cfg_attr(feature = "serde", serde(rename = "vorauszahlungen"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub vorauszahlungen: Option<Vec<Vorauszahlung>>,
    /// ggf. auf einen Vergleichszeitraum hochgerechneter Verbrauch des vorherigen Jahres zu Vergleichszwecken mit dem aktuellen Jahr, gemäß EnWG § 40
    #[cfg_attr(feature = "serde", serde(rename = "vorjahresverbrauch"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub vorjahresverbrauch: Option<Box<Energiemenge>>,
    #[cfg_attr(feature = "serde", serde(rename = "zaehler"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub zaehler: Option<Vec<Box<Zaehler>>>,
    /// Informationen wie eine Rechnung bezahlt werden soll
    #[cfg_attr(feature = "serde", serde(rename = "zahlungsinformationen"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub zahlungsinformationen: Option<Vec<Zahlungsinformation>>,
    /// Der zu zahlende Betrag, der sich aus (gesamtbrutto - vorausbezahlt - rabattBrutto) ergibt
    #[cfg_attr(feature = "serde", serde(rename = "zuZahlen"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub zu_zahlen: Option<Betrag>,
    #[cfg_attr(feature = "serde", serde(rename = "zukuenftigerAbschlag"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub zukuenftiger_abschlag: Option<Betrag>,
    #[cfg_attr(feature = "serde", serde(rename = "zusatzAttribute"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub zusatz_attribute: Option<Vec<ZusatzAttribut>>,
    /// Unknown JSON fields captured during deserialization for round-trip preservation.
    /// `None` when no unknown fields were present (zero heap allocation).
    #[cfg_attr(feature = "json", serde(flatten))]
    #[cfg_attr(
        feature = "json",
        serde(skip_serializing_if = "crate::json::ext_map_is_empty")
    )]
    #[cfg_attr(not(feature = "json"), serde(skip))]
    #[cfg_attr(feature = "builder", builder(default, setter(skip)))]
    #[doc(hidden)]
    pub _additional: crate::LimitedExtensionMap,
}
impl Default for Rechnung {
    fn default() -> Self {
        Self {
            aktueller_verbrauch: Default::default(),
            anfangszaehlerstand: Default::default(),
            endzaehlerstand: Default::default(),
            faelligkeitsdatum: Default::default(),
            fremdkosten: Default::default(),
            gesamtbrutto: Default::default(),
            gesamtnetto: Default::default(),
            gesamtsteuer: Default::default(),
            id: Default::default(),
            ist_original: Default::default(),
            ist_simuliert: Default::default(),
            ist_storno: Default::default(),
            jahresverbrauch: Default::default(),
            kaeuferreferenz: Default::default(),
            marktlokation: Default::default(),
            messlokation: Default::default(),
            messstellenbetreiber: Default::default(),
            netzbetreiber: Default::default(),
            netznutzungrechnungsart: Default::default(),
            netznutzungrechnungstyp: Default::default(),
            original_rechnungsnummer: Default::default(),
            rabatt_netto: Default::default(),
            rechnungsdatum: Default::default(),
            rechnungsempfaenger: Default::default(),
            rechnungsersteller: Default::default(),
            rechnungsnummer: Default::default(),
            rechnungsperiode: Default::default(),
            rechnungspositionen: Default::default(),
            rechnungsstatus: Default::default(),
            rechnungstitel: Default::default(),
            rechnungstyp: Default::default(),
            referenzverbraeuche: Default::default(),
            sparte: Default::default(),
            steuerbetraege: Default::default(),
            teilrechnungen: Default::default(),
            typ: Some(BoTyp::Rechnung),
            version: Some("202607.1.0".to_owned()),
            vertrag: Default::default(),
            vorauszahlungen: Default::default(),
            vorjahresverbrauch: Default::default(),
            zaehler: Default::default(),
            zahlungsinformationen: Default::default(),
            zu_zahlen: Default::default(),
            zukuenftiger_abschlag: Default::default(),
            zusatz_attribute: Default::default(),
            _additional: Default::default(),
        }
    }
}
impl Bo4eTyped for Rechnung {
    type Typ = BoTyp;
    const TYP: BoTyp = BoTyp::Rechnung;
    const TYP_WIRE: &'static str = "RECHNUNG";
    const SCHEMA_VERSION: &'static str = "202607.1.0";
    const SCHEMA_SERIES: &'static str = "202607";
}
impl crate::bo4e_typed_sealed::Sealed for Rechnung {}
impl Bo4eObject for Rechnung {}
impl crate::bo4e_object_sealed::Sealed for Rechnung {}
#[cfg(feature = "json")]
impl crate::json::sealed::Sealed for Rechnung {}
#[cfg(feature = "json")]
impl crate::json::Bo4eJsonExt for Rechnung {}
#[cfg(feature = "json")]
impl crate::json::Bo4eExtensionData for Rechnung {
    fn extension_data(&self) -> &indexmap::IndexMap<String, serde_json::Value> {
        self._additional
            .as_map()
            .unwrap_or(&crate::json::extension::EMPTY_EXTENSION_MAP)
    }
    fn has_extension_data(&self) -> bool {
        !self._additional.is_empty()
    }
}
#[cfg(feature = "json")]
impl std::fmt::Display for Rechnung {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => f.write_str(&json),
            Err(e) => write!(f, "<Rechnung: serialization error: {e}>"),
        }
    }
}
impl crate::Bo4eStrict for Rechnung {
    fn collect_unknown_enums(&self, path: &str, out: &mut Vec<String>) {
        if let Some(v) = &self.aktueller_verbrauch {
            crate::Bo4eStrict::collect_unknown_enums(
                &**v,
                &crate::strict::field_path(path, "aktuellerVerbrauch"),
                out,
            );
        }
        if let Some(v) = &self.anfangszaehlerstand {
            crate::Bo4eStrict::collect_unknown_enums(
                &**v,
                &crate::strict::field_path(path, "anfangszaehlerstand"),
                out,
            );
        }
        if let Some(v) = &self.endzaehlerstand {
            crate::Bo4eStrict::collect_unknown_enums(
                &**v,
                &crate::strict::field_path(path, "endzaehlerstand"),
                out,
            );
        }
        if let Some(v) = &self.fremdkosten {
            crate::Bo4eStrict::collect_unknown_enums(
                &**v,
                &crate::strict::field_path(path, "fremdkosten"),
                out,
            );
        }
        if let Some(v) = &self.gesamtbrutto {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "gesamtbrutto"),
                out,
            );
        }
        if let Some(v) = &self.gesamtnetto {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "gesamtnetto"),
                out,
            );
        }
        if let Some(v) = &self.gesamtsteuer {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "gesamtsteuer"),
                out,
            );
        }
        if let Some(v) = &self.jahresverbrauch {
            crate::Bo4eStrict::collect_unknown_enums(
                &**v,
                &crate::strict::field_path(path, "jahresverbrauch"),
                out,
            );
        }
        if let Some(v) = &self.marktlokation {
            crate::Bo4eStrict::collect_unknown_enums(
                &**v,
                &crate::strict::field_path(path, "marktlokation"),
                out,
            );
        }
        if let Some(v) = &self.messlokation {
            crate::Bo4eStrict::collect_unknown_enums(
                &**v,
                &crate::strict::field_path(path, "messlokation"),
                out,
            );
        }
        if let Some(v) = &self.messstellenbetreiber {
            crate::Bo4eStrict::collect_unknown_enums(
                &**v,
                &crate::strict::field_path(path, "messstellenbetreiber"),
                out,
            );
        }
        if let Some(v) = &self.netzbetreiber {
            crate::Bo4eStrict::collect_unknown_enums(
                &**v,
                &crate::strict::field_path(path, "netzbetreiber"),
                out,
            );
        }
        if let Some(v) = &self.netznutzungrechnungsart {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "netznutzungrechnungsart"),
                out,
            );
        }
        if let Some(v) = &self.netznutzungrechnungstyp {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "netznutzungrechnungstyp"),
                out,
            );
        }
        if let Some(v) = &self.rabatt_netto {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "rabattNetto"),
                out,
            );
        }
        if let Some(v) = &self.rechnungsempfaenger {
            crate::Bo4eStrict::collect_unknown_enums(
                &**v,
                &crate::strict::field_path(path, "rechnungsempfaenger"),
                out,
            );
        }
        if let Some(v) = &self.rechnungsersteller {
            crate::Bo4eStrict::collect_unknown_enums(
                &**v,
                &crate::strict::field_path(path, "rechnungsersteller"),
                out,
            );
        }
        if let Some(v) = &self.rechnungsperiode {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "rechnungsperiode"),
                out,
            );
        }
        if let Some(items) = &self.rechnungspositionen {
            let child = crate::strict::field_path(path, "rechnungspositionen");
            for (i, item) in items.iter().enumerate() {
                crate::Bo4eStrict::collect_unknown_enums(
                    item,
                    &crate::strict::index_path(&child, i),
                    out,
                );
            }
        }
        if let Some(v) = &self.rechnungsstatus {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "rechnungsstatus"),
                out,
            );
        }
        if let Some(v) = &self.rechnungstyp {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "rechnungstyp"),
                out,
            );
        }
        if let Some(items) = &self.referenzverbraeuche {
            let child = crate::strict::field_path(path, "referenzverbraeuche");
            for (i, item) in items.iter().enumerate() {
                crate::Bo4eStrict::collect_unknown_enums(
                    &**item,
                    &crate::strict::index_path(&child, i),
                    out,
                );
            }
        }
        if let Some(v) = &self.sparte {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "sparte"),
                out,
            );
        }
        if let Some(items) = &self.steuerbetraege {
            let child = crate::strict::field_path(path, "steuerbetraege");
            for (i, item) in items.iter().enumerate() {
                crate::Bo4eStrict::collect_unknown_enums(
                    item,
                    &crate::strict::index_path(&child, i),
                    out,
                );
            }
        }
        if let Some(items) = &self.teilrechnungen {
            let child = crate::strict::field_path(path, "teilrechnungen");
            for (i, item) in items.iter().enumerate() {
                crate::Bo4eStrict::collect_unknown_enums(
                    &**item,
                    &crate::strict::index_path(&child, i),
                    out,
                );
            }
        }
        if let Some(v) = &self.vertrag {
            crate::Bo4eStrict::collect_unknown_enums(
                &**v,
                &crate::strict::field_path(path, "vertrag"),
                out,
            );
        }
        if let Some(items) = &self.vorauszahlungen {
            let child = crate::strict::field_path(path, "vorauszahlungen");
            for (i, item) in items.iter().enumerate() {
                crate::Bo4eStrict::collect_unknown_enums(
                    item,
                    &crate::strict::index_path(&child, i),
                    out,
                );
            }
        }
        if let Some(v) = &self.vorjahresverbrauch {
            crate::Bo4eStrict::collect_unknown_enums(
                &**v,
                &crate::strict::field_path(path, "vorjahresverbrauch"),
                out,
            );
        }
        if let Some(items) = &self.zaehler {
            let child = crate::strict::field_path(path, "zaehler");
            for (i, item) in items.iter().enumerate() {
                crate::Bo4eStrict::collect_unknown_enums(
                    &**item,
                    &crate::strict::index_path(&child, i),
                    out,
                );
            }
        }
        if let Some(items) = &self.zahlungsinformationen {
            let child = crate::strict::field_path(path, "zahlungsinformationen");
            for (i, item) in items.iter().enumerate() {
                crate::Bo4eStrict::collect_unknown_enums(
                    item,
                    &crate::strict::index_path(&child, i),
                    out,
                );
            }
        }
        if let Some(v) = &self.zu_zahlen {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "zuZahlen"),
                out,
            );
        }
        if let Some(v) = &self.zukuenftiger_abschlag {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "zukuenftigerAbschlag"),
                out,
            );
        }
        if let Some(items) = &self.zusatz_attribute {
            let child = crate::strict::field_path(path, "zusatzAttribute");
            for (i, item) in items.iter().enumerate() {
                crate::Bo4eStrict::collect_unknown_enums(
                    item,
                    &crate::strict::index_path(&child, i),
                    out,
                );
            }
        }
    }
}
#[cfg(feature = "json")]
impl crate::json::Bo4eExtensions for Rechnung {
    fn collect_extension_paths(&self, path: &str, out: &mut Vec<String>) {
        if let Some(map) = self._additional.as_map() {
            for key in map.keys() {
                out.push(crate::strict::extension_path(path, key));
            }
        }
        if let Some(v) = &self.aktueller_verbrauch {
            crate::json::Bo4eExtensions::collect_extension_paths(
                &**v,
                &crate::strict::field_path(path, "aktuellerVerbrauch"),
                out,
            );
        }
        if let Some(v) = &self.anfangszaehlerstand {
            crate::json::Bo4eExtensions::collect_extension_paths(
                &**v,
                &crate::strict::field_path(path, "anfangszaehlerstand"),
                out,
            );
        }
        if let Some(v) = &self.endzaehlerstand {
            crate::json::Bo4eExtensions::collect_extension_paths(
                &**v,
                &crate::strict::field_path(path, "endzaehlerstand"),
                out,
            );
        }
        if let Some(v) = &self.fremdkosten {
            crate::json::Bo4eExtensions::collect_extension_paths(
                &**v,
                &crate::strict::field_path(path, "fremdkosten"),
                out,
            );
        }
        if let Some(v) = &self.gesamtbrutto {
            crate::json::Bo4eExtensions::collect_extension_paths(
                v,
                &crate::strict::field_path(path, "gesamtbrutto"),
                out,
            );
        }
        if let Some(v) = &self.gesamtnetto {
            crate::json::Bo4eExtensions::collect_extension_paths(
                v,
                &crate::strict::field_path(path, "gesamtnetto"),
                out,
            );
        }
        if let Some(v) = &self.gesamtsteuer {
            crate::json::Bo4eExtensions::collect_extension_paths(
                v,
                &crate::strict::field_path(path, "gesamtsteuer"),
                out,
            );
        }
        if let Some(v) = &self.jahresverbrauch {
            crate::json::Bo4eExtensions::collect_extension_paths(
                &**v,
                &crate::strict::field_path(path, "jahresverbrauch"),
                out,
            );
        }
        if let Some(v) = &self.marktlokation {
            crate::json::Bo4eExtensions::collect_extension_paths(
                &**v,
                &crate::strict::field_path(path, "marktlokation"),
                out,
            );
        }
        if let Some(v) = &self.messlokation {
            crate::json::Bo4eExtensions::collect_extension_paths(
                &**v,
                &crate::strict::field_path(path, "messlokation"),
                out,
            );
        }
        if let Some(v) = &self.messstellenbetreiber {
            crate::json::Bo4eExtensions::collect_extension_paths(
                &**v,
                &crate::strict::field_path(path, "messstellenbetreiber"),
                out,
            );
        }
        if let Some(v) = &self.netzbetreiber {
            crate::json::Bo4eExtensions::collect_extension_paths(
                &**v,
                &crate::strict::field_path(path, "netzbetreiber"),
                out,
            );
        }
        if let Some(v) = &self.rabatt_netto {
            crate::json::Bo4eExtensions::collect_extension_paths(
                v,
                &crate::strict::field_path(path, "rabattNetto"),
                out,
            );
        }
        if let Some(v) = &self.rechnungsempfaenger {
            crate::json::Bo4eExtensions::collect_extension_paths(
                &**v,
                &crate::strict::field_path(path, "rechnungsempfaenger"),
                out,
            );
        }
        if let Some(v) = &self.rechnungsersteller {
            crate::json::Bo4eExtensions::collect_extension_paths(
                &**v,
                &crate::strict::field_path(path, "rechnungsersteller"),
                out,
            );
        }
        if let Some(v) = &self.rechnungsperiode {
            crate::json::Bo4eExtensions::collect_extension_paths(
                v,
                &crate::strict::field_path(path, "rechnungsperiode"),
                out,
            );
        }
        if let Some(items) = &self.rechnungspositionen {
            let child = crate::strict::field_path(path, "rechnungspositionen");
            for (i, item) in items.iter().enumerate() {
                crate::json::Bo4eExtensions::collect_extension_paths(
                    item,
                    &crate::strict::index_path(&child, i),
                    out,
                );
            }
        }
        if let Some(items) = &self.referenzverbraeuche {
            let child = crate::strict::field_path(path, "referenzverbraeuche");
            for (i, item) in items.iter().enumerate() {
                crate::json::Bo4eExtensions::collect_extension_paths(
                    &**item,
                    &crate::strict::index_path(&child, i),
                    out,
                );
            }
        }
        if let Some(items) = &self.steuerbetraege {
            let child = crate::strict::field_path(path, "steuerbetraege");
            for (i, item) in items.iter().enumerate() {
                crate::json::Bo4eExtensions::collect_extension_paths(
                    item,
                    &crate::strict::index_path(&child, i),
                    out,
                );
            }
        }
        if let Some(items) = &self.teilrechnungen {
            let child = crate::strict::field_path(path, "teilrechnungen");
            for (i, item) in items.iter().enumerate() {
                crate::json::Bo4eExtensions::collect_extension_paths(
                    &**item,
                    &crate::strict::index_path(&child, i),
                    out,
                );
            }
        }
        if let Some(v) = &self.vertrag {
            crate::json::Bo4eExtensions::collect_extension_paths(
                &**v,
                &crate::strict::field_path(path, "vertrag"),
                out,
            );
        }
        if let Some(items) = &self.vorauszahlungen {
            let child = crate::strict::field_path(path, "vorauszahlungen");
            for (i, item) in items.iter().enumerate() {
                crate::json::Bo4eExtensions::collect_extension_paths(
                    item,
                    &crate::strict::index_path(&child, i),
                    out,
                );
            }
        }
        if let Some(v) = &self.vorjahresverbrauch {
            crate::json::Bo4eExtensions::collect_extension_paths(
                &**v,
                &crate::strict::field_path(path, "vorjahresverbrauch"),
                out,
            );
        }
        if let Some(items) = &self.zaehler {
            let child = crate::strict::field_path(path, "zaehler");
            for (i, item) in items.iter().enumerate() {
                crate::json::Bo4eExtensions::collect_extension_paths(
                    &**item,
                    &crate::strict::index_path(&child, i),
                    out,
                );
            }
        }
        if let Some(items) = &self.zahlungsinformationen {
            let child = crate::strict::field_path(path, "zahlungsinformationen");
            for (i, item) in items.iter().enumerate() {
                crate::json::Bo4eExtensions::collect_extension_paths(
                    item,
                    &crate::strict::index_path(&child, i),
                    out,
                );
            }
        }
        if let Some(v) = &self.zu_zahlen {
            crate::json::Bo4eExtensions::collect_extension_paths(
                v,
                &crate::strict::field_path(path, "zuZahlen"),
                out,
            );
        }
        if let Some(v) = &self.zukuenftiger_abschlag {
            crate::json::Bo4eExtensions::collect_extension_paths(
                v,
                &crate::strict::field_path(path, "zukuenftigerAbschlag"),
                out,
            );
        }
        if let Some(items) = &self.zusatz_attribute {
            let child = crate::strict::field_path(path, "zusatzAttribute");
            for (i, item) in items.iter().enumerate() {
                crate::json::Bo4eExtensions::collect_extension_paths(
                    item,
                    &crate::strict::index_path(&child, i),
                    out,
                );
            }
        }
    }
}
