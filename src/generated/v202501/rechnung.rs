use super::{
    Betrag, Bo4eObject, BoTyp, Geschaeftspartner, Marktlokation, Messlokation,
    NetznutzungRechnungsart, NetznutzungRechnungstyp, Rechnungsposition, Rechnungsstatus,
    Rechnungstyp, Sparte, Steuerbetrag, Zeitraum, ZusatzAttribut,
};
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(not(feature = "json"), derive(Hash))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "validate", derive(garde::Validate))]
#[cfg_attr(feature = "validate", garde(allow_unvalidated))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(
    all(feature = "validate", feature = "versioned"),
    garde(custom(crate::validation::v202501::validate_rechnung_arithmetic))
)]
/// Modell für die Abbildung von Rechnungen und Netznutzungsrechnungen im Kontext der Energiewirtschaft;
///
/// > **Note:** [Rechnung JSON Schema](https://json-schema.app/view/%23?url=https://raw.githubusercontent.com/BO4E/BO4E-Schemas/v202501.0.0/src/bo4e_schemas/bo/Rechnung.json)
pub struct Rechnung {
    /// Zu diesem Datum ist die Zahlung fällig
    #[cfg_attr(feature = "serde", serde(rename = "faelligkeitsdatum"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
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
    /// Requires the `time` feature for the `time::OffsetDateTime` representation.
    /// Without `time`, stores the ISO-8601 string value unchanged.
    #[cfg_attr(feature = "serde", serde(rename = "faelligkeitsdatum"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    #[cfg_attr(
        feature = "schemars",
        schemars(schema_with = "crate::schema_helpers::opt_datetime_schema")
    )]
    #[cfg(not(feature = "time"))]
    pub faelligkeitsdatum: Option<String>,
    /// Die Summe aus Netto- und Steuerbetrag
    #[cfg_attr(feature = "serde", serde(rename = "gesamtbrutto"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub gesamtbrutto: Option<Betrag>,
    /// Die Summe der Nettobeträge der Rechnungsteile
    #[cfg_attr(feature = "serde", serde(rename = "gesamtnetto"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub gesamtnetto: Option<Betrag>,
    /// Die Summe der Steuerbeträge der Rechnungsteile
    #[cfg_attr(feature = "serde", serde(rename = "gesamtsteuer"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub gesamtsteuer: Option<Betrag>,
    /// Eine generische ID, die für eigene Zwecke genutzt werden kann.
    /// Z.B. könnten hier UUIDs aus einer Datenbank stehen oder URLs zu einem Backend-System.
    #[cfg_attr(feature = "serde", serde(rename = "_id"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub id: Option<String>,
    /// Kennzeichen, ob es sich um ein Original (true) oder eine Kopie handelt (false)
    #[cfg_attr(feature = "serde", serde(rename = "istOriginal"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub ist_original: Option<bool>,
    /// Kennzeichen, ob es sich um eine simulierte Rechnung, z.B. zur Rechnungsprüfung handelt
    #[cfg_attr(feature = "serde", serde(rename = "istSimuliert"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub ist_simuliert: Option<bool>,
    /// Kennzeichnung, ob es sich um eine Stornorechnung handelt;
    /// im Falle "true" findet sich im Attribut "originalrechnungsnummer" die Nummer der Originalrechnung.
    #[cfg_attr(feature = "serde", serde(rename = "istStorno"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub ist_storno: Option<bool>,
    /// Marktlokation, auf die sich die Rechnung bezieht
    #[cfg_attr(feature = "serde", serde(rename = "marktlokation"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub marktlokation: Option<Box<Marktlokation>>,
    /// Messlokation, auf die sich die Rechnung bezieht
    #[cfg_attr(feature = "serde", serde(rename = "messlokation"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub messlokation: Option<Box<Messlokation>>,
    /// Aus der INVOIC entnommen, befüllt wenn es sich um eine Netznutzungsrechnung handelt
    #[cfg_attr(feature = "serde", serde(rename = "netznutzungrechnungsart"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub netznutzungrechnungsart: Option<NetznutzungRechnungsart>,
    /// Aus der INVOIC entnommen, befüllt wenn es sich um eine Netznutzungsrechnung handelt
    #[cfg_attr(feature = "serde", serde(rename = "netznutzungrechnungstyp"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub netznutzungrechnungstyp: Option<NetznutzungRechnungstyp>,
    /// Im Falle einer Stornorechnung (storno = true) steht hier die Rechnungsnummer der stornierten Rechnung
    #[cfg_attr(feature = "serde", serde(rename = "originalRechnungsnummer"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub original_rechnungsnummer: Option<String>,
    /// Gesamtrabatt auf den Bruttobetrag
    #[cfg_attr(feature = "serde", serde(rename = "rabattBrutto"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub rabatt_brutto: Option<Betrag>,
    /// Ausstellungsdatum der Rechnung
    #[cfg_attr(feature = "serde", serde(rename = "rechnungsdatum"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
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
    /// Requires the `time` feature for the `time::OffsetDateTime` representation.
    /// Without `time`, stores the ISO-8601 string value unchanged.
    #[cfg_attr(feature = "serde", serde(rename = "rechnungsdatum"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    #[cfg_attr(
        feature = "schemars",
        schemars(schema_with = "crate::schema_helpers::opt_datetime_schema")
    )]
    #[cfg(not(feature = "time"))]
    pub rechnungsdatum: Option<String>,
    /// Der Aussteller der Rechnung, die Rollencodenummer kennt man über den im Geschäftspartner verlinkten Marktteilnehmer
    #[cfg_attr(feature = "serde", serde(rename = "rechnungsempfaenger"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub rechnungsempfaenger: Option<Box<Geschaeftspartner>>,
    /// Der Aussteller der Rechnung, die Rollencodenummer kennt man über den im Geschäftspartner verlinkten Marktteilnehmer
    #[cfg_attr(feature = "serde", serde(rename = "rechnungsersteller"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub rechnungsersteller: Option<Box<Geschaeftspartner>>,
    /// Eine im Verwendungskontext eindeutige Nummer für die Rechnung
    #[cfg_attr(feature = "serde", serde(rename = "rechnungsnummer"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub rechnungsnummer: Option<String>,
    /// Der Zeitraum der zugrunde liegenden Lieferung zur Rechnung
    #[cfg_attr(feature = "serde", serde(rename = "rechnungsperiode"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub rechnungsperiode: Option<Zeitraum>,
    /// Die Rechnungspositionen
    #[cfg_attr(feature = "serde", serde(rename = "rechnungspositionen"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub rechnungspositionen: Option<Vec<Rechnungsposition>>,
    /// Status der Rechnung zur Kennzeichnung des Bearbeitungsstandes
    #[cfg_attr(feature = "serde", serde(rename = "rechnungsstatus"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub rechnungsstatus: Option<Rechnungsstatus>,
    /// Bezeichnung für die vorliegende Rechnung
    #[cfg_attr(feature = "serde", serde(rename = "rechnungstitel"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub rechnungstitel: Option<String>,
    /// Ein kontextbezogender Rechnungstyp, z.B. Netznutzungsrechnung
    #[cfg_attr(feature = "serde", serde(rename = "rechnungstyp"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub rechnungstyp: Option<Rechnungstyp>,
    /// Sparte (Strom, Gas ...) für die die Rechnung ausgestellt ist
    #[cfg_attr(feature = "serde", serde(rename = "sparte"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub sparte: Option<Sparte>,
    /// Eine Liste mit Steuerbeträgen pro Steuerkennzeichen/Steuersatz;
    /// die Summe dieser Beträge ergibt den Wert für gesamtsteuer.
    #[cfg_attr(feature = "serde", serde(rename = "steuerbetraege"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub steuerbetraege: Option<Vec<Steuerbetrag>>,
    /// BO type identifier — always `BoTyp::Rechnung` for this struct.
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
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub version: Option<String>,
    /// Die Summe evtl. vorausgezahlter Beträge, z.B. Abschläge. Angabe als Bruttowert
    #[cfg_attr(feature = "serde", serde(rename = "vorausgezahlt"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub vorausgezahlt: Option<Betrag>,
    /// Der zu zahlende Betrag, der sich aus (gesamtbrutto - vorausbezahlt - rabattBrutto) ergibt
    #[cfg_attr(feature = "serde", serde(rename = "zuZahlen"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub zu_zahlen: Option<Betrag>,
    #[cfg_attr(feature = "serde", serde(rename = "zusatzAttribute"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
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
    pub(crate) _additional: crate::LimitedExtensionMap,
}
impl Default for Rechnung {
    fn default() -> Self {
        Self {
            typ: Some(BoTyp::Rechnung),
            faelligkeitsdatum: Default::default(),
            gesamtbrutto: Default::default(),
            gesamtnetto: Default::default(),
            gesamtsteuer: Default::default(),
            id: Default::default(),
            ist_original: Default::default(),
            ist_simuliert: Default::default(),
            ist_storno: Default::default(),
            marktlokation: Default::default(),
            messlokation: Default::default(),
            netznutzungrechnungsart: Default::default(),
            netznutzungrechnungstyp: Default::default(),
            original_rechnungsnummer: Default::default(),
            rabatt_brutto: Default::default(),
            rechnungsdatum: Default::default(),
            rechnungsempfaenger: Default::default(),
            rechnungsersteller: Default::default(),
            rechnungsnummer: Default::default(),
            rechnungsperiode: Default::default(),
            rechnungspositionen: Default::default(),
            rechnungsstatus: Default::default(),
            rechnungstitel: Default::default(),
            rechnungstyp: Default::default(),
            sparte: Default::default(),
            steuerbetraege: Default::default(),
            version: Default::default(),
            vorausgezahlt: Default::default(),
            zu_zahlen: Default::default(),
            zusatz_attribute: Default::default(),
            _additional: Default::default(),
        }
    }
}
impl Bo4eObject for Rechnung {
    type BoTyp = BoTyp;
    fn bo_type(&self) -> BoTyp {
        self.typ.unwrap_or(BoTyp::Rechnung)
    }
    fn schema_version(&self) -> &'static str {
        "v202501.0.0"
    }
}
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
