use super::{
    Bo4eComponent, Bo4eTyped, ComTyp, Menge, Messwertstatus, Messwertstatuszusatz, ZusatzAttribut,
};
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(not(feature = "json"), derive(Eq, Hash))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "validate", derive(garde::Validate))]
#[cfg_attr(feature = "validate", garde(allow_unvalidated))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Abbildung eines Messwertes mit Stati, Zeitpunkt und Wert.
///
/// > **Note:** [Messwert JSON Schema](https://json-schema.app/view/%23?url=https://raw.githubusercontent.com/BO4E/BO4E-Schemas/v202607.1.0/src/bo4e_schemas/com/Messwert.json)
pub struct Messwert {
    /// Eine generische ID, die für eigene Zwecke genutzt werden kann.
    /// Z.B. könnten hier UUIDs aus einer Datenbank stehen oder URLs zu einem Backend-System.
    #[cfg_attr(feature = "serde", serde(rename = "_id"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub id: Option<String>,
    /// Gibt den Status des Messwerts an.
    #[cfg_attr(feature = "serde", serde(rename = "messwertstatus"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub messwertstatus: Option<Messwertstatus>,
    /// Gibt den Status Zusatz des Messwerts an.
    #[cfg_attr(feature = "serde", serde(rename = "messwertstatuszusatz"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub messwertstatuszusatz: Option<Messwertstatuszusatz>,
    /// BO4E type discriminant — always `ComTyp::Messwert` for this struct.
    #[cfg_attr(feature = "serde", serde(rename = "_typ"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(
        feature = "builder",
        builder(default = Some(ComTyp::Messwert), setter(skip))
    )]
    pub typ: Option<ComTyp>,
    /// Version der COM-Struktur aka "fachliche Versionierung"
    #[cfg_attr(feature = "serde", serde(rename = "_version"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(
        feature = "builder",
        builder(default = Some("202607.1.0".to_owned()), setter(into))
    )]
    pub version: Option<String>,
    /// Gibt die gemessene Menge an.
    #[cfg_attr(feature = "serde", serde(rename = "wert"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub wert: Option<Menge>,
    /// Gibt den Zeitpunkt des Messwerts an.
    #[cfg_attr(feature = "serde", serde(rename = "zeitpunkt"))]
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
    pub zeitpunkt: Option<time::OffsetDateTime>,
    /// Gibt den Zeitpunkt des Messwerts an.
    #[cfg_attr(feature = "serde", serde(rename = "zeitpunkt"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(
        feature = "schemars",
        schemars(schema_with = "crate::schema_helpers::opt_datetime_schema")
    )]
    #[cfg(not(feature = "time"))]
    pub zeitpunkt: Option<String>,
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
impl Default for Messwert {
    fn default() -> Self {
        Self {
            id: Default::default(),
            messwertstatus: Default::default(),
            messwertstatuszusatz: Default::default(),
            typ: Some(ComTyp::Messwert),
            version: Some("202607.1.0".to_owned()),
            wert: Default::default(),
            zeitpunkt: Default::default(),
            zusatz_attribute: Default::default(),
            _additional: Default::default(),
        }
    }
}
impl Bo4eTyped for Messwert {
    type Typ = ComTyp;
    const TYP: ComTyp = ComTyp::Messwert;
    const TYP_WIRE: &'static str = "MESSWERT";
    const SCHEMA_VERSION: &'static str = "202607.1.0";
    const SCHEMA_SERIES: &'static str = "202607";
}
impl crate::bo4e_typed_sealed::Sealed for Messwert {}
impl Bo4eComponent for Messwert {}
impl crate::bo4e_component_sealed::Sealed for Messwert {}
#[cfg(feature = "json")]
impl crate::json::sealed::Sealed for Messwert {}
#[cfg(feature = "json")]
impl crate::json::Bo4eJsonExt for Messwert {}
#[cfg(feature = "json")]
impl crate::json::Bo4eExtensionData for Messwert {
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
impl std::fmt::Display for Messwert {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => f.write_str(&json),
            Err(e) => write!(f, "<Messwert: serialization error: {e}>"),
        }
    }
}
impl crate::Bo4eStrict for Messwert {
    fn collect_unknown_enums(&self, path: &str, out: &mut Vec<String>) {
        if let Some(v) = &self.messwertstatus {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "messwertstatus"),
                out,
            );
        }
        if let Some(v) = &self.messwertstatuszusatz {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "messwertstatuszusatz"),
                out,
            );
        }
        if let Some(v) = &self.wert {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "wert"),
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
impl crate::json::Bo4eExtensions for Messwert {
    fn collect_extension_paths(&self, path: &str, out: &mut Vec<String>) {
        if let Some(map) = self._additional.as_map() {
            for key in map.keys() {
                out.push(crate::strict::extension_path(path, key));
            }
        }
        if let Some(v) = &self.wert {
            crate::json::Bo4eExtensions::collect_extension_paths(
                v,
                &crate::strict::field_path(path, "wert"),
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
impl crate::zusatz_attribut::HasZusatzAttribute for Messwert {
    fn zusatz_attribute_field(&self) -> Option<&Vec<ZusatzAttribut>> {
        self.zusatz_attribute.as_ref()
    }
    fn zusatz_attribute_field_mut(&mut self) -> &mut Option<Vec<ZusatzAttribut>> {
        &mut self.zusatz_attribute
    }
}
