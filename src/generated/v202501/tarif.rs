use super::{
    AufAbschlagRegional, Bo4eObject, BoTyp, Energiemix, Kundentyp, Marktteilnehmer, Preisgarantie,
    Registeranzahl, Sparte, Tarifberechnungsparameter, Tarifeinschraenkung, Tarifmerkmal,
    TarifpreispositionProOrt, Tariftyp, Vertragskonditionen, Zeitraum, ZusatzAttribut,
};
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(not(feature = "json"), derive(Hash))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "validate", derive(garde::Validate))]
#[cfg_attr(feature = "validate", garde(allow_unvalidated))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Abbildung eines Tarifs mit regionaler Zuordnung von Preisen und Auf- und Abschlägen
///
/// > **Note:** [Tarif JSON Schema](https://json-schema.app/view/%23?url=https://raw.githubusercontent.com/BO4E/BO4E-Schemas/v202501.0.0/src/bo4e_schemas/bo/Tarif.json)
pub struct Tarif {
    /// Der Marktteilnehmer (Lieferant), der diesen Tarif anbietet
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
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub berechnungsparameter: Option<Tarifberechnungsparameter>,
    /// Name des Tarifs
    #[cfg_attr(feature = "serde", serde(rename = "bezeichnung"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub bezeichnung: Option<String>,
    /// Der Energiemix, der für diesen Tarif gilt
    #[cfg_attr(feature = "serde", serde(rename = "energiemix"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub energiemix: Option<Energiemix>,
    /// Eine generische ID, die für eigene Zwecke genutzt werden kann.
    /// Z.B. könnten hier UUIDs aus einer Datenbank stehen oder URLs zu einem Backend-System.
    #[cfg_attr(feature = "serde", serde(rename = "_id"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub id: Option<String>,
    /// Kundentypen für den der Tarif gilt, z.B. Privatkunden
    #[cfg_attr(feature = "serde", serde(rename = "kundentypen"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub kundentypen: Option<Vec<Kundentyp>>,
    /// Preisgarantie für diesen Tarif
    #[cfg_attr(feature = "serde", serde(rename = "preisgarantie"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub preisgarantie: Option<Preisgarantie>,
    /// Gibt an, wann der Preis zuletzt angepasst wurde
    #[cfg_attr(feature = "serde", serde(rename = "preisstand"))]
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
    pub preisstand: Option<time::OffsetDateTime>,
    /// Requires the `time` feature for the `time::OffsetDateTime` representation.
    /// Without `time`, stores the ISO-8601 string value unchanged.
    #[cfg_attr(feature = "serde", serde(rename = "preisstand"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(
        feature = "schemars",
        schemars(schema_with = "crate::schema_helpers::opt_datetime_schema")
    )]
    #[cfg(not(feature = "time"))]
    pub preisstand: Option<String>,
    /// Die Art des Tarifes, z.B. Eintarif oder Mehrtarif
    #[cfg_attr(feature = "serde", serde(rename = "registeranzahl"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub registeranzahl: Option<Registeranzahl>,
    /// Strom oder Gas, etc.
    #[cfg_attr(feature = "serde", serde(rename = "sparte"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub sparte: Option<Sparte>,
    /// Auf- und Abschläge auf die Preise oder Kosten mit regionaler Eingrenzung
    #[cfg_attr(feature = "serde", serde(rename = "tarifAufAbschlaege"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub tarif_auf_abschlaege: Option<Vec<AufAbschlagRegional>>,
    /// Die Bedingungen und Einschränkungen unter denen ein Tarif angewendet werden kann
    #[cfg_attr(feature = "serde", serde(rename = "tarifeinschraenkung"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub tarifeinschraenkung: Option<Tarifeinschraenkung>,
    /// Weitere Merkmale des Tarifs, z.B. Festpreis oder Vorkasse
    #[cfg_attr(feature = "serde", serde(rename = "tarifmerkmale"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub tarifmerkmale: Option<Vec<Tarifmerkmal>>,
    /// Die festgelegten Preise mit regionaler Eingrenzung z.B. für Arbeitspreis, Grundpreis etc.
    #[cfg_attr(feature = "serde", serde(rename = "tarifpreise"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub tarifpreise: Option<Vec<TarifpreispositionProOrt>>,
    /// Hinweis auf den Tariftyp, z.B. Grundversorgung oder Sondertarif
    #[cfg_attr(feature = "serde", serde(rename = "tariftyp"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub tariftyp: Option<Tariftyp>,
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
    /// Mindestlaufzeiten und Kündigungsfristen zusammengefasst
    #[cfg_attr(feature = "serde", serde(rename = "vertragskonditionen"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub vertragskonditionen: Option<Vertragskonditionen>,
    /// Internetseite auf dem der Tarif zu finden ist
    #[cfg_attr(feature = "serde", serde(rename = "website"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub website: Option<String>,
    /// Angabe, in welchem Zeitraum der Tarif gültig ist
    #[cfg_attr(feature = "serde", serde(rename = "zeitlicheGueltigkeit"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub zeitliche_gueltigkeit: Option<Zeitraum>,
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
impl Default for Tarif {
    fn default() -> Self {
        Self {
            typ: Some(BoTyp::Tarif),
            anbieter: Default::default(),
            anbietername: Default::default(),
            anwendung_von: Default::default(),
            bemerkung: Default::default(),
            berechnungsparameter: Default::default(),
            bezeichnung: Default::default(),
            energiemix: Default::default(),
            id: Default::default(),
            kundentypen: Default::default(),
            preisgarantie: Default::default(),
            preisstand: Default::default(),
            registeranzahl: Default::default(),
            sparte: Default::default(),
            tarif_auf_abschlaege: Default::default(),
            tarifeinschraenkung: Default::default(),
            tarifmerkmale: Default::default(),
            tarifpreise: Default::default(),
            tariftyp: Default::default(),
            version: Default::default(),
            vertragskonditionen: Default::default(),
            website: Default::default(),
            zeitliche_gueltigkeit: Default::default(),
            zusatz_attribute: Default::default(),
            _additional: Default::default(),
        }
    }
}
impl Bo4eObject for Tarif {
    type BoTyp = BoTyp;
    fn bo_type(&self) -> BoTyp {
        self.typ.unwrap_or(BoTyp::Tarif)
    }
    fn schema_version(&self) -> &'static str {
        "v202501.0.0"
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
