use super::{ComTyp, Preis, Preisreferenz, Zaehlzeitdefinition, ZusatzAttribut};
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(not(feature = "json"), derive(Hash))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "validate", derive(garde::Validate))]
#[cfg_attr(feature = "validate", garde(allow_unvalidated))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Modelliert eine zeitvariable Preisposition.
///
/// > **Note:** [ZeitvariablePreisposition JSON Schema](https://json-schema.app/view/%23?url=https://raw.githubusercontent.com/BO4E/BO4E-Schemas/v202607.0.0/src/bo4e_schemas/com/ZeitvariablePreisposition.json)
pub struct ZeitvariablePreisposition {
    /// Eine (beliebige) Bezeichnung für die Preisposition.
    #[cfg_attr(feature = "serde", serde(rename = "bezeichnung"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub bezeichnung: Option<String>,
    /// Eine generische ID, die für eigene Zwecke genutzt werden kann.
    /// Z.B. könnten hier UUIDs aus einer Datenbank stehen oder URLs zu einem Backend-System.
    #[cfg_attr(feature = "serde", serde(rename = "_id"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub id: Option<String>,
    /// Der Preis für diese Position.
    #[cfg_attr(feature = "serde", serde(rename = "preis"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub preis: Option<Preis>,
    /// Die Referenz worauf sich der Preis bezieht.
    /// Die explizite Einheit wird durch das Feld `bezugswert` im `COM Preis` angegeben.
    #[cfg_attr(feature = "serde", serde(rename = "preisreferenz"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub preisreferenz: Option<Preisreferenz>,
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
    /// Die Zählzeitdefinition, deren Schaltschema bestimmt, wann diese Preisposition gilt.
    #[cfg_attr(feature = "serde", serde(rename = "zaehlzeitdefinition"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub zaehlzeitdefinition: Option<Box<Zaehlzeitdefinition>>,
    /// Der freie Register-Code (z.B. "HT", "NT", "ST", "SuperHT") innerhalb der referenzierten
    /// `zaehlzeitdefinition`, auf den sich diese Preisposition bezieht. Der Code sollte als
    /// `registercode` in mindestens einer `Umschaltzeit` der referenzierten Zählzeitdefinition
    /// vorkommen.
    #[cfg_attr(feature = "serde", serde(rename = "zaehlzeitregister"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub zaehlzeitregister: Option<String>,
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
impl crate::json::sealed::Sealed for ZeitvariablePreisposition {}
#[cfg(feature = "json")]
impl crate::json::Bo4eJsonExt for ZeitvariablePreisposition {}
#[cfg(feature = "json")]
impl crate::json::Bo4eExtensionData for ZeitvariablePreisposition {
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
impl std::fmt::Display for ZeitvariablePreisposition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => f.write_str(&json),
            Err(e) => write!(f, "<ZeitvariablePreisposition: serialization error: {e}>"),
        }
    }
}
