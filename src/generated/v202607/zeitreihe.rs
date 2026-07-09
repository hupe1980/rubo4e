use super::{
    Bo4eObject, BoTyp, Medium, Mengeneinheit, Messart, Messgroesse, Messwertstatus, Zeitreihenwert,
    ZusatzAttribut,
};
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(not(feature = "json"), derive(Hash))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "validate", derive(garde::Validate))]
#[cfg_attr(feature = "validate", garde(allow_unvalidated))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Abbildung einer allgemeinen Zeitreihe mit einem Wertvektor.
/// Die Werte können mit wahlfreier zeitlicher Distanz im Vektor abgelegt sein.
///
/// > **Note:** [Zeitreihe JSON Schema](https://json-schema.app/view/%23?url=https://raw.githubusercontent.com/BO4E/BO4E-Schemas/v202607.0.0/src/bo4e_schemas/bo/Zeitreihe.json)
pub struct Zeitreihe {
    /// Beschreibt die Verwendung der Zeitreihe
    #[cfg_attr(feature = "serde", serde(rename = "beschreibung"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub beschreibung: Option<String>,
    /// Bezeichnung für die Zeitreihe
    #[cfg_attr(feature = "serde", serde(rename = "bezeichnung"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub bezeichnung: Option<String>,
    /// Alle Werte in der Tabelle haben die Einheit, die hier angegeben ist
    #[cfg_attr(feature = "serde", serde(rename = "einheit"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub einheit: Option<Mengeneinheit>,
    /// Eine generische ID, die für eigene Zwecke genutzt werden kann.
    /// Z.B. könnten hier UUIDs aus einer Datenbank stehen oder URLs zu einem Backend-System.
    #[cfg_attr(feature = "serde", serde(rename = "_id"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub id: Option<String>,
    /// Medium, das gemessen wurde (z.B. Wasser, Dampf, Strom, Gas)
    #[cfg_attr(feature = "serde", serde(rename = "medium"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub medium: Option<Medium>,
    /// Beschreibt die Art der Messung (z.B. aktueller Wert, mittlerer Wert, maximaler Wert)
    #[cfg_attr(feature = "serde", serde(rename = "messart"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub messart: Option<Messart>,
    /// Beschreibt, was gemessen wurde (z.B. Strom, Spannung, Wirkleistung, Scheinleistung)
    #[cfg_attr(feature = "serde", serde(rename = "messgroesse"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub messgroesse: Option<Messgroesse>,
    /// BO type identifier — always `BoTyp::Zeitreihe` for this struct.
    #[cfg_attr(feature = "serde", serde(rename = "_typ"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(
        feature = "builder",
        builder(default = Some(BoTyp::Zeitreihe), setter(skip))
    )]
    pub typ: Option<BoTyp>,
    /// Version der Zeitreihe
    #[cfg_attr(feature = "serde", serde(rename = "version"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub version: Option<String>,
    /// Hier liegen jeweils die Werte
    #[cfg_attr(feature = "serde", serde(rename = "werte"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub werte: Option<Vec<Zeitreihenwert>>,
    /// Kennzeichnung, wie die Werte entstanden sind, z.B. ABGELESEN oder PROGNOSEWERT
    #[cfg_attr(feature = "serde", serde(rename = "wertherkunft"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub wertherkunft: Option<Messwertstatus>,
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
impl Default for Zeitreihe {
    fn default() -> Self {
        Self {
            typ: Some(BoTyp::Zeitreihe),
            beschreibung: Default::default(),
            bezeichnung: Default::default(),
            einheit: Default::default(),
            id: Default::default(),
            medium: Default::default(),
            messart: Default::default(),
            messgroesse: Default::default(),
            version: Default::default(),
            werte: Default::default(),
            wertherkunft: Default::default(),
            zusatz_attribute: Default::default(),
            _additional: Default::default(),
        }
    }
}
impl Bo4eObject for Zeitreihe {
    type BoTyp = BoTyp;
    fn bo_type(&self) -> BoTyp {
        self.typ.unwrap_or(BoTyp::Zeitreihe)
    }
    fn schema_version(&self) -> &'static str {
        "v202607.0.0"
    }
}
#[cfg(feature = "json")]
impl crate::json::sealed::Sealed for Zeitreihe {}
#[cfg(feature = "json")]
impl crate::json::Bo4eJsonExt for Zeitreihe {}
#[cfg(feature = "json")]
impl crate::json::Bo4eExtensionData for Zeitreihe {
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
impl std::fmt::Display for Zeitreihe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => f.write_str(&json),
            Err(e) => write!(f, "<Zeitreihe: serialization error: {e}>"),
        }
    }
}
