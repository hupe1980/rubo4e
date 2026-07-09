use super::{
    ComTyp, EinheitsPreisposition, LastvariablePreisposition, RelativePreisposition, Zeitraum,
    ZeitvariablePreisposition, ZusatzAttribut,
};
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(not(feature = "json"), derive(Hash))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "validate", derive(garde::Validate))]
#[cfg_attr(feature = "validate", garde(allow_unvalidated))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Mit dieser Komponente kann ein aus verschiedenen Preispositionen zusammengesetzter Tarifpreis zeitaufgelöst
/// dargestellt werden.
///
/// > **Note:** [Tarifpreiszeitscheibe JSON Schema](https://json-schema.app/view/%23?url=https://raw.githubusercontent.com/BO4E/BO4E-Schemas/v202607.0.0/src/bo4e_schemas/com/Tarifpreiszeitscheibe.json)
pub struct Tarifpreiszeitscheibe {
    /// Eine Liste von Einheits-Preispositionen.
    #[cfg_attr(feature = "serde", serde(rename = "einheitsPreispositionen"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub einheits_preispositionen: Option<Vec<EinheitsPreisposition>>,
    /// Eine generische ID, die für eigene Zwecke genutzt werden kann.
    /// Z.B. könnten hier UUIDs aus einer Datenbank stehen oder URLs zu einem Backend-System.
    #[cfg_attr(feature = "serde", serde(rename = "_id"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub id: Option<String>,
    /// Eine Liste von lastvariablen Preispositionen.
    /// Diese Preispositionen sind vorgesehen, um bspw. ein Staffel- oder Zonenmodell abzubilden.
    #[cfg_attr(feature = "serde", serde(rename = "lastvariablePreispositionen"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub lastvariable_preispositionen: Option<Vec<LastvariablePreisposition>>,
    /// Eine Liste von relativen Preispositionen.
    /// Diese Preispositionen modellieren prozentuale Modifikationen auf bestehende Preispositionen.
    ///
    /// Dazu wird über ein Feld in `RelativePreisposition` auf die `_id` einer anderen Preispositionen verwiesen.
    /// Die ID hat hierbei kein vorgegebenes Format und hat auch keine fachliche Bedeutung. Es handelt sich hierbei
    /// um eine rein technische Lösung, um einen Querverweis zu modellieren.
    #[cfg_attr(feature = "serde", serde(rename = "relativePreispositionen"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub relative_preispositionen: Option<Vec<RelativePreisposition>>,
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
    /// Gibt an für welchen Zeitraum dieser zusammengesetzte Tarifpreis gültig ist.
    #[cfg_attr(feature = "serde", serde(rename = "zeitscheibengueltigkeit"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub zeitscheibengueltigkeit: Option<Zeitraum>,
    /// Eine Liste von zeitvariablen Preispositionen.
    /// Dies können z.B. Preispositionen mit Zählzeitdefinitionen sein, um ein klassisches HT/NT Modell abzubilden.
    #[cfg_attr(feature = "serde", serde(rename = "zeitvariablePreispositionen"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub zeitvariable_preispositionen: Option<Vec<ZeitvariablePreisposition>>,
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
#[cfg(feature = "json")]
impl crate::json::sealed::Sealed for Tarifpreiszeitscheibe {}
#[cfg(feature = "json")]
impl crate::json::Bo4eJsonExt for Tarifpreiszeitscheibe {}
#[cfg(feature = "json")]
impl crate::json::Bo4eExtensionData for Tarifpreiszeitscheibe {
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
impl std::fmt::Display for Tarifpreiszeitscheibe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => f.write_str(&json),
            Err(e) => write!(f, "<Tarifpreiszeitscheibe: serialization error: {e}>"),
        }
    }
}
