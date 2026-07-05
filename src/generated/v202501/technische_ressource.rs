use super::{
    Bo4eObject, BoTyp, EMobilitaetsart, Erzeugungsart, Lokationszuordnung, Menge,
    Speicherart, TechnischeRessourceNutzung, TechnischeRessourceVerbrauchsart,
    Waermenutzung, ZusatzAttribut,
};
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(not(feature = "json"), derive(Hash))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "validate", derive(garde::Validate))]
#[cfg_attr(feature = "validate", garde(allow_unvalidated))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Object containing information about a technische Ressource
///
/// > **Note:** [TechnischeRessource JSON Schema](https://json-schema.app/view/%23?url=https://raw.githubusercontent.com/BO4E/BO4E-Schemas/v202501.0.0/src/bo4e_schemas/bo/TechnischeRessource.json)
pub struct TechnischeRessource {
    /// Art der E-Mobilität
    #[cfg_attr(feature = "serde", serde(rename = "emobilitaetsart"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub emobilitaetsart: Option<EMobilitaetsart>,
    /// Art der Erzeugung der Energie
    #[cfg_attr(feature = "serde", serde(rename = "erzeugungsart"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub erzeugungsart: Option<Erzeugungsart>,
    /// Eine generische ID, die für eigene Zwecke genutzt werden kann.
    /// Z.B. könnten hier UUIDs aus einer Datenbank stehen oder URLs zu einem Backend-System.
    #[cfg_attr(feature = "serde", serde(rename = "_id"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub id: Option<String>,
    /// Lokationsbuendel Code, der die Funktion dieses BOs an der Lokationsbuendelstruktur beschreibt.
    #[cfg_attr(feature = "serde", serde(rename = "lokationsbuendelObjektcode"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub lokationsbuendel_objektcode: Option<String>,
    /// Lokationszuordnung, um bspw. die zugehörigen Messlokationen anzugeben
    #[cfg_attr(feature = "serde", serde(rename = "lokationszuordnungen"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub lokationszuordnungen: Option<Vec<Box<Lokationszuordnung>>>,
    /// Nennleistung (Abgabe)
    #[cfg_attr(feature = "serde", serde(rename = "nennleistungabgabe"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub nennleistungabgabe: Option<Menge>,
    /// Nennleistung (Aufnahme)
    #[cfg_attr(feature = "serde", serde(rename = "nennleistungaufnahme"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub nennleistungaufnahme: Option<Menge>,
    /// Art des Speichers
    #[cfg_attr(feature = "serde", serde(rename = "speicherart"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub speicherart: Option<Speicherart>,
    /// Speicherkapazität
    #[cfg_attr(feature = "serde", serde(rename = "speicherkapazitaet"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub speicherkapazitaet: Option<Menge>,
    /// Identifikationsnummer einer technischen Ressource
    #[cfg_attr(feature = "serde", serde(rename = "technischeRessourceId"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub technische_ressource_id: Option<crate::identifiers::TrId>,
    /// Art und Nutzung der technischen Ressource
    #[cfg_attr(feature = "serde", serde(rename = "technischeRessourceNutzung"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub technische_ressource_nutzung: Option<TechnischeRessourceNutzung>,
    /// Verbrauchsart der technischen Ressource
    #[cfg_attr(feature = "serde", serde(rename = "technischeRessourceVerbrauchsart"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub technische_ressource_verbrauchsart: Option<TechnischeRessourceVerbrauchsart>,
    /// BO type identifier — always `BoTyp::TechnischeRessource` for this struct.
    #[cfg_attr(feature = "serde", serde(rename = "_typ"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(
        feature = "builder",
        builder(default = Some(BoTyp::Technischeressource), setter(skip))
    )]
    pub typ: Option<BoTyp>,
    /// Version der BO-Struktur aka "fachliche Versionierung"
    #[cfg_attr(feature = "serde", serde(rename = "_version"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub version: Option<String>,
    /// Vorgelagerte Messlokation ID
    #[cfg_attr(feature = "serde", serde(rename = "vorgelagerteMesslokationId"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub vorgelagerte_messlokation_id: Option<String>,
    /// Wärmenutzung
    #[cfg_attr(feature = "serde", serde(rename = "waermenutzung"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub waermenutzung: Option<Waermenutzung>,
    /// Referenz auf die der technischen Ressource zugeordneten Marktlokation
    #[cfg_attr(feature = "serde", serde(rename = "zugeordneteMarktlokationId"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub zugeordnete_marktlokation_id: Option<String>,
    /// Referenz auf die der technischen Ressource zugeordneten Steuerbaren Ressource
    #[cfg_attr(feature = "serde", serde(rename = "zugeordneteSteuerbareRessourceId"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub zugeordnete_steuerbare_ressource_id: Option<String>,
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
impl Default for TechnischeRessource {
    fn default() -> Self {
        Self {
            typ: Some(BoTyp::Technischeressource),
            emobilitaetsart: Default::default(),
            erzeugungsart: Default::default(),
            id: Default::default(),
            lokationsbuendel_objektcode: Default::default(),
            lokationszuordnungen: Default::default(),
            nennleistungabgabe: Default::default(),
            nennleistungaufnahme: Default::default(),
            speicherart: Default::default(),
            speicherkapazitaet: Default::default(),
            technische_ressource_id: Default::default(),
            technische_ressource_nutzung: Default::default(),
            technische_ressource_verbrauchsart: Default::default(),
            version: Default::default(),
            vorgelagerte_messlokation_id: Default::default(),
            waermenutzung: Default::default(),
            zugeordnete_marktlokation_id: Default::default(),
            zugeordnete_steuerbare_ressource_id: Default::default(),
            zusatz_attribute: Default::default(),
            _additional: Default::default(),
        }
    }
}
impl Bo4eObject for TechnischeRessource {
    type BoTyp = BoTyp;
    fn bo_type(&self) -> BoTyp {
        self.typ.unwrap_or(BoTyp::Technischeressource)
    }
    fn schema_version(&self) -> &'static str {
        "v202501.0.0"
    }
}
#[cfg(feature = "json")]
impl crate::json::sealed::Sealed for TechnischeRessource {}
#[cfg(feature = "json")]
impl crate::json::Bo4eJsonExt for TechnischeRessource {}
#[cfg(feature = "json")]
impl crate::json::Bo4eExtensionData for TechnischeRessource {
    fn extension_data(&self) -> &indexmap::IndexMap<String, serde_json::Value> {
        self._additional.as_map().unwrap_or(&crate::json::extension::EMPTY_EXTENSION_MAP)
    }
    fn has_extension_data(&self) -> bool {
        !self._additional.is_empty()
    }
}
#[cfg(feature = "json")]
impl std::fmt::Display for TechnischeRessource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => f.write_str(&json),
            Err(e) => write!(f, "<TechnischeRessource: serialization error: {e}>"),
        }
    }
}
