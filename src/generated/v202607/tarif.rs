use super::{
    Bo4eObject, BoTyp, Energiemix, Kundentyp, Marktteilnehmer, Preisgarantie, Regionspreis,
    Registeranzahl, Sparte, Tarifberechnungsparameter, Tarifeinschraenkung, Tarifmerkmal, Tariftyp,
    Vertragskonditionen, Zeitraum, ZusatzAttribut,
};
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(not(feature = "json"), derive(Hash))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "validate", derive(garde::Validate))]
#[cfg_attr(feature = "validate", garde(allow_unvalidated))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Abbildung eines Tarifs.
///
/// Der Tarifpreis kann regionsaufgelöst und unter Angabe von Zeitscheiben angegeben werden. So kann bspw. derselbe
/// Tarif je nach Region andere Preise aufweisen. Es können auch Tarifpreise abgebildet werden, die sich ab einem
/// bestimmten Zeitpunkt auf andere Regionen ausweiten, da die Regionen ebenfalls mit Zeitscheiben versehen sind.
///
/// Ein Tarifpreis setzt sich dabei aus mehreren Preispositionen zusammen. So können z.B. auch mit
/// `COM RelativePreisposition` prozentuale Auf- und Abschläge auf andere Preispositionen definiert werden.
/// Alle Preispositionen hängen unter `COM Tarifpreiszeitscheibe` mit einer Ausnahme.
///
/// Möchten Sie einen dynamischen Tarif modellieren, so gibt es das `COM DynamischePreisposition`. Da diese
/// Preisposition weder orts- noch zeitabhängig ist, hängt diese direkt unter dem `BO Tarif`. Eine zeitabhängige
/// Änderung einer dynamischen Tarifpreisposition ist unsinnig, da es sich (unserer Ansicht nach) dann um einen
/// völlig neuen Tarif handelt. Davon unabhängig können (und müssen) natürlich weiterhin zusätzlich alle anderen
/// Preispositionen orts- und zeitabhängig angegeben werden.
///
/// > Hinweis: Das Vorhandensein einer `COM DynamischePreisposition` dient gleichzeitig auch als "Flag" dafür, ob
/// > es sich bei diesem Tarif um einen dynamischen handelt.
///
/// > **Note:** [Tarif JSON Schema](https://json-schema.app/view/%23?url=https://raw.githubusercontent.com/BO4E/BO4E-Schemas/v202607.0.0/src/bo4e_schemas/bo/Tarif.json)
pub struct Tarif {
    /// Der Marktteilnehmer, der diesen Tarif anbietet, angeboten hat oder anbieten wird.
    #[cfg_attr(feature = "serde", serde(rename = "anbieter"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub anbieter: Option<Box<Marktteilnehmer>>,
    /// Der Name des Marktpartners, der den Tarif anbietet
    #[cfg_attr(feature = "serde", serde(rename = "anbietername"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub anbietername: Option<String>,
    /// Angabe des inklusiven Zeitpunkts, ab dem der Tarif bzw. der Preis angewendet und abgerechnet wird,
    /// z.B. "2021-07-20T18:31:48Z"
    #[cfg_attr(feature = "serde", serde(rename = "anwendungVon"))]
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
    pub anwendung_von: Option<time::OffsetDateTime>,
    /// Requires the `time` feature for the `time::OffsetDateTime` representation.
    /// Without `time`, stores the ISO-8601 string value unchanged.
    #[cfg_attr(feature = "serde", serde(rename = "anwendungVon"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(
        feature = "schemars",
        schemars(schema_with = "crate::schema_helpers::opt_datetime_schema")
    )]
    #[cfg(not(feature = "time"))]
    pub anwendung_von: Option<String>,
    /// Freitext
    #[cfg_attr(feature = "serde", serde(rename = "bemerkung"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub bemerkung: Option<String>,
    /// Für die Berechnung der Kosten sind die hier abgebildeten Parameter heranzuziehen
    #[cfg_attr(feature = "serde", serde(rename = "berechnungsparameter"))]
    pub berechnungsparameter: Tarifberechnungsparameter,
    /// Eine (beliebige) Beschreibung für den Tarif.
    #[cfg_attr(feature = "serde", serde(rename = "beschreibung"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub beschreibung: Option<String>,
    /// Eine (beliebige) Bezeichnung für den Tarif.
    #[cfg_attr(feature = "serde", serde(rename = "bezeichnung"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub bezeichnung: Option<String>,
    /// Gibt die Bezugsquelle (z.B. Börsenindex) für den dynamischen Tarif an.
    /// Dieses Feld muss genau dann gesetzt werden, wenn es sich bei diesem Tarif um einen dynamischen Tarif handelt.
    #[cfg_attr(feature = "serde", serde(rename = "dynamischePreispositionQuelle"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub dynamische_preisposition_quelle: Option<String>,
    /// Der Energiemix mit einem Eintrag pro Gültigkeitsjahr (siehe `Energiemix.gueltigkeitsjahr`).
    #[cfg_attr(feature = "serde", serde(rename = "energiemix"))]
    pub energiemix: Vec<Energiemix>,
    /// Eine generische ID, die für eigene Zwecke genutzt werden kann.
    /// Z.B. könnten hier UUIDs aus einer Datenbank stehen oder URLs zu einem Backend-System.
    #[cfg_attr(feature = "serde", serde(rename = "_id"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub id: Option<String>,
    /// Eine Liste an Kundentypen, für die dieser Tarif vorgesehen ist.
    #[cfg_attr(feature = "serde", serde(rename = "kundentypen"))]
    pub kundentypen: Vec<Kundentyp>,
    /// Preisgarantie für diesen Tarif
    #[cfg_attr(feature = "serde", serde(rename = "preisgarantie"))]
    pub preisgarantie: Preisgarantie,
    /// Enthält alle regions- und zeitaufgelösten Tarifpreise.
    /// Ausschließlich die `COM DynamischePreisposition` wird unter einem anderen Feld namens `dynamischePreisposition`
    /// angegeben.
    #[cfg_attr(feature = "serde", serde(rename = "regionspreise"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub regionspreise: Option<Vec<Regionspreis>>,
    /// Hinweis zu den Registern bzw. Zählwerken.
    /// Bspw. benötigt ein HT-/NT-Tarif auch eine entsprechende Registeranzahl.
    #[cfg_attr(feature = "serde", serde(rename = "registeranzahl"))]
    pub registeranzahl: Registeranzahl,
    /// Strom / Gas
    #[cfg_attr(feature = "serde", serde(rename = "sparte"))]
    pub sparte: Sparte,
    /// Die Bedingungen und Einschränkungen unter denen ein Tarif angewendet werden kann
    #[cfg_attr(feature = "serde", serde(rename = "tarifeinschraenkung"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub tarifeinschraenkung: Option<Tarifeinschraenkung>,
    /// Eine Liste von Produktmerkmalen im Zusammenhang mit diesem Tarif.
    #[cfg_attr(feature = "serde", serde(rename = "tarifmerkmale"))]
    pub tarifmerkmale: Vec<Tarifmerkmal>,
    /// Der Tariftyp. Bsp.: Grundversorgung, Ersatzversorgung, etc.
    #[cfg_attr(feature = "serde", serde(rename = "tariftyp"))]
    pub tariftyp: Tariftyp,
    /// BO type identifier — always `BoTyp::Tarif` for this struct.
    #[cfg_attr(feature = "serde", serde(rename = "_typ"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default = Some(BoTyp::Tarif), setter(skip)))]
    pub typ: Option<BoTyp>,
    /// Version der BO-Struktur aka "fachliche Versionierung"
    #[cfg_attr(feature = "serde", serde(rename = "_version"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub version: Option<String>,
    /// Vertragskonditionen für diesen Tarif.
    #[cfg_attr(feature = "serde", serde(rename = "vertragskonditionen"))]
    pub vertragskonditionen: Vertragskonditionen,
    /// Internetseite, auf der der Tarif veröffentlicht ist.
    #[cfg_attr(feature = "serde", serde(rename = "website"))]
    pub website: String,
    /// Angabe, in welchem Zeitraum der Tarif gültig ist
    #[cfg_attr(feature = "serde", serde(rename = "zeitlicheGueltigkeit"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub zeitliche_gueltigkeit: Option<Zeitraum>,
    /// Der Zeitraum, in dem eine Belieferung (für diesen Tarif) möglich ist.
    #[cfg_attr(feature = "serde", serde(rename = "zeitraumBelieferbarkeit"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub zeitraum_belieferbarkeit: Option<Zeitraum>,
    /// Der Zeitraum, in dem der Tarif beim Anbieter vertraglich abschließbar ist.
    #[cfg_attr(feature = "serde", serde(rename = "zeitraumVermarktung"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub zeitraum_vermarktung: Option<Zeitraum>,
    #[cfg_attr(feature = "serde", serde(rename = "zusatzAttribute"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
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
impl Bo4eObject for Tarif {
    type BoTyp = BoTyp;
    fn bo_type(&self) -> BoTyp {
        self.typ.unwrap_or(BoTyp::Tarif)
    }
    fn schema_version(&self) -> &'static str {
        "v202607.0.0"
    }
}
#[cfg(feature = "json")]
impl crate::json::sealed::Sealed for Tarif {}
#[cfg(feature = "json")]
impl crate::json::Bo4eJsonExt for Tarif {}
#[cfg(feature = "json")]
impl crate::json::Bo4eExtensionData for Tarif {
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
impl std::fmt::Display for Tarif {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => f.write_str(&json),
            Err(e) => write!(f, "<Tarif: serialization error: {e}>"),
        }
    }
}
