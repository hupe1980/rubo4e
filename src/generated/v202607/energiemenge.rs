use super::{Bo4eObject, BoTyp, Menge, Zeitraum, ZusatzAttribut};
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(not(feature = "json"), derive(Hash))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "validate", derive(garde::Validate))]
#[cfg_attr(feature = "validate", garde(allow_unvalidated))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Abbildung von Mengen, die Lokationen zugeordnet sind
///
/// > **Note:** [Energiemenge JSON Schema](https://json-schema.app/view/%23?url=https://raw.githubusercontent.com/BO4E/BO4E-Schemas/v202607.0.0/src/bo4e_schemas/bo/Energiemenge.json)
pub struct Energiemenge {
    /// Ergänzende Beschreibung zur Energiemenge
    #[cfg_attr(feature = "serde", serde(rename = "beschreibung"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub beschreibung: Option<String>,
    /// Eine generische ID, die für eigene Zwecke genutzt werden kann.
    /// Z.B. könnten hier UUIDs aus einer Datenbank stehen oder URLs zu einem Backend-System.
    #[cfg_attr(feature = "serde", serde(rename = "_id"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub id: Option<String>,
    /// Die angefallene/gemessene Menge
    #[cfg_attr(feature = "serde", serde(rename = "menge"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub menge: Option<Menge>,
    /// Die OBIS-Kennzahl der Energiemenge
    #[cfg_attr(feature = "serde", serde(rename = "obisKennzahl"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub obis_kennzahl: Option<crate::identifiers::ObisCode>,
    /// BO type identifier — always `BoTyp::Energiemenge` for this struct.
    #[cfg_attr(feature = "serde", serde(rename = "_typ"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(
        feature = "builder",
        builder(default = Some(BoTyp::Energiemenge), setter(skip))
    )]
    pub typ: Option<BoTyp>,
    /// Version der BO-Struktur aka "fachliche Versionierung"
    #[cfg_attr(feature = "serde", serde(rename = "_version"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub version: Option<String>,
    /// Zeitraum, in dem die Energiemenge angefallen ist/gemessen wurde
    #[cfg_attr(feature = "serde", serde(rename = "zeitraum"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub zeitraum: Option<Zeitraum>,
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
impl Default for Energiemenge {
    fn default() -> Self {
        Self {
            typ: Some(BoTyp::Energiemenge),
            beschreibung: Default::default(),
            id: Default::default(),
            menge: Default::default(),
            obis_kennzahl: Default::default(),
            version: Default::default(),
            zeitraum: Default::default(),
            zusatz_attribute: Default::default(),
            _additional: Default::default(),
        }
    }
}
impl Bo4eObject for Energiemenge {
    type BoTyp = BoTyp;
    fn bo_type(&self) -> BoTyp {
        self.typ.unwrap_or(BoTyp::Energiemenge)
    }
    fn schema_version(&self) -> &'static str {
        "v202607.0.0"
    }
}
#[cfg(feature = "json")]
impl crate::json::sealed::Sealed for Energiemenge {}
#[cfg(feature = "json")]
impl crate::json::Bo4eJsonExt for Energiemenge {}
#[cfg(feature = "json")]
impl crate::json::Bo4eExtensionData for Energiemenge {
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
impl std::fmt::Display for Energiemenge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => f.write_str(&json),
            Err(e) => write!(f, "<Energiemenge: serialization error: {e}>"),
        }
    }
}
