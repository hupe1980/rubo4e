use super::{
    AufAbschlagstyp, AufAbschlagsziel, ComTyp, Energiemix, Preisgarantie, RegionalePreisstaffel,
    Tarifeinschraenkung, Vertragskonditionen, Waehrungseinheit, Zeitraum, ZusatzAttribut,
};
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(not(feature = "json"), derive(Hash))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "validate", derive(garde::Validate))]
#[cfg_attr(feature = "validate", garde(allow_unvalidated))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Mit dieser Komponente können Auf- und Abschläge verschiedener Typen im Zusammenhang mit regionalen Gültigkeiten
/// abgebildet werden.
/// Hier sind auch die Auswirkungen auf verschiedene Tarifparameter modelliert, die sich durch die Auswahl eines Auf-
/// oder Abschlags ergeben.
///
/// > **Note:** [RegionalerAufAbschlag JSON Schema](https://json-schema.app/view/%23?url=https://raw.githubusercontent.com/BO4E/BO4E-Schemas/v202501.0.0/src/bo4e_schemas/com/RegionalerAufAbschlag.json)
pub struct RegionalerAufAbschlag {
    /// Typ des Aufabschlages (z.B. absolut oder prozentual)
    #[cfg_attr(feature = "serde", serde(rename = "aufAbschlagstyp"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub auf_abschlagstyp: Option<AufAbschlagstyp>,
    /// Diesem Preis oder den Kosten ist der Auf/Abschlag zugeordnet. Z.B. Arbeitspreis, Gesamtpreis etc.
    #[cfg_attr(feature = "serde", serde(rename = "aufAbschlagsziel"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub auf_abschlagsziel: Option<AufAbschlagsziel>,
    /// Beschreibung des Auf-/Abschlags
    #[cfg_attr(feature = "serde", serde(rename = "beschreibung"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub beschreibung: Option<String>,
    /// Bezeichnung des Auf-/Abschlags
    #[cfg_attr(feature = "serde", serde(rename = "bezeichnung"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub bezeichnung: Option<String>,
    /// Gibt an in welcher Währungseinheit der Auf/Abschlag berechnet wird (nur im Falle absoluter Aufschlagstypen).
    #[cfg_attr(feature = "serde", serde(rename = "einheit"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub einheit: Option<Waehrungseinheit>,
    /// Änderungen in den Einschränkungen zum Tarif;
    /// Falls in dieser Komponenten angegeben, werden die Tarifparameter hiermit überschrieben.
    #[cfg_attr(feature = "serde", serde(rename = "einschraenkungsaenderung"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub einschraenkungsaenderung: Option<Tarifeinschraenkung>,
    /// Der Energiemix kann sich durch einen AufAbschlag ändern (z.B. zwei Cent Aufschlag für Ökostrom).
    /// Sollte dies der Fall sein, wird hier die neue Zusammensetzung des Energiemix angegeben.
    #[cfg_attr(feature = "serde", serde(rename = "energiemixaenderung"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub energiemixaenderung: Option<Energiemix>,
    /// Änderungen in den Garantievereinbarungen;
    /// Falls in dieser Komponenten angegeben, werden die Tarifparameter hiermit überschrieben.
    #[cfg_attr(feature = "serde", serde(rename = "garantieaenderung"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub garantieaenderung: Option<Preisgarantie>,
    /// Zeitraum, in dem der Abschlag zur Anwendung kommen kann
    #[cfg_attr(feature = "serde", serde(rename = "gueltigkeitszeitraum"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub gueltigkeitszeitraum: Option<Zeitraum>,
    /// Eine generische ID, die für eigene Zwecke genutzt werden kann.
    /// Z.B. könnten hier UUIDs aus einer Datenbank stehen oder URLs zu einem Backend-System.
    #[cfg_attr(feature = "serde", serde(rename = "_id"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub id: Option<String>,
    /// Werte für die gestaffelten Auf/Abschläge mit regionaler Eingrenzung
    #[cfg_attr(feature = "serde", serde(rename = "staffeln"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub staffeln: Option<Vec<RegionalePreisstaffel>>,
    /// Durch die Anwendung des Auf/Abschlags kann eine Änderung des Tarifnamens auftreten
    #[cfg_attr(feature = "serde", serde(rename = "tarifnamensaenderungen"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub tarifnamensaenderungen: Option<String>,
    /// COM type identifier for this struct.
    #[cfg_attr(feature = "serde", serde(rename = "_typ"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub typ: Option<ComTyp>,
    /// Version der COM-Struktur aka "fachliche Versionierung"
    #[cfg_attr(feature = "serde", serde(rename = "_version"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub version: Option<String>,
    /// Änderungen in den Vertragskonditionen;
    /// Falls in dieser Komponenten angegeben, werden die Tarifparameter hiermit überschrieben.
    #[cfg_attr(feature = "serde", serde(rename = "vertagskonditionsaenderung"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub vertagskonditionsaenderung: Option<Vertragskonditionen>,
    /// Voraussetzungen, die erfüllt sein müssen, damit dieser AufAbschlag zur Anwendung kommen kann
    #[cfg_attr(feature = "serde", serde(rename = "voraussetzungen"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub voraussetzungen: Option<Vec<String>>,
    /// Internetseite, auf der die Informationen zum Auf-/Abschlag veröffentlicht sind
    #[cfg_attr(feature = "serde", serde(rename = "website"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub website: Option<String>,
    #[cfg_attr(feature = "serde", serde(rename = "zusatzAttribute"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub zusatz_attribute: Option<Vec<ZusatzAttribut>>,
    /// Zusatzprodukte, die nur in Kombination mit diesem AufAbschlag erhältlich sind
    #[cfg_attr(feature = "serde", serde(rename = "zusatzprodukte"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub zusatzprodukte: Option<Vec<String>>,
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
#[cfg(feature = "json")]
impl crate::json::sealed::Sealed for RegionalerAufAbschlag {}
#[cfg(feature = "json")]
impl crate::json::Bo4eJsonExt for RegionalerAufAbschlag {}
#[cfg(feature = "json")]
impl crate::json::Bo4eExtensionData for RegionalerAufAbschlag {
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
impl std::fmt::Display for RegionalerAufAbschlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => f.write_str(&json),
            Err(e) => write!(f, "<RegionalerAufAbschlag: serialization error: {e}>"),
        }
    }
}
