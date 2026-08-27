use super::{Bo4eComponent, Bo4eTyped, ComTyp, Zeitraum, ZusatzAttribut};
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(not(feature = "json"), derive(Eq, Hash))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "validate", derive(garde::Validate))]
#[cfg_attr(feature = "validate", garde(allow_unvalidated))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Abbildung für Vertragskonditionen. Die Komponente wird sowohl im Vertrag als auch im Tarif verwendet.
///
/// > **Note:** [Vertragskonditionen JSON Schema](https://json-schema.app/view/%23?url=https://raw.githubusercontent.com/BO4E/BO4E-Schemas/v202607.1.0/src/bo4e_schemas/com/Vertragskonditionen.json)
pub struct Vertragskonditionen {
    /// In diesen Zyklen werden Abschläge gestellt. Alternativ kann auch die Anzahl in den Konditionen angeben werden.
    #[cfg_attr(feature = "serde", serde(rename = "abschlagszyklus"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub abschlagszyklus: Option<Zeitraum>,
    /// Anzahl der vereinbarten Abschläge pro Jahr, z.B. 12
    #[cfg_attr(feature = "serde", serde(rename = "anzahlAbschlaege"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "crate::decimal_serde::deserialize_opt")
    )]
    #[cfg(feature = "decimal")]
    pub anzahl_abschlaege: Option<rust_decimal::Decimal>,
    /// Anzahl der vereinbarten Abschläge pro Jahr, z.B. 12
    #[cfg_attr(feature = "serde", serde(rename = "anzahlAbschlaege"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "crate::decimal_serde::deserialize_opt")
    )]
    #[cfg(not(feature = "decimal"))]
    pub anzahl_abschlaege: Option<String>,
    /// Freitext zur Beschreibung der Konditionen, z.B. "Standardkonditionen Gas"
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
    /// Innerhalb dieser Frist kann der Vertrag gekündigt werden
    #[cfg_attr(feature = "serde", serde(rename = "kuendigungsfrist"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub kuendigungsfrist: Option<Zeitraum>,
    /// BO4E type discriminant — always `ComTyp::Vertragskonditionen` for this struct.
    #[cfg_attr(feature = "serde", serde(rename = "_typ"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(
        feature = "builder",
        builder(default = Some(ComTyp::Vertragskonditionen), setter(skip))
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
    /// Über diesen Zeitraum läuft der Vertrag
    #[cfg_attr(feature = "serde", serde(rename = "vertragslaufzeit"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub vertragslaufzeit: Option<Zeitraum>,
    /// Falls der Vertrag nicht gekündigt wird, verlängert er sich automatisch um die hier angegebene Zeit
    #[cfg_attr(feature = "serde", serde(rename = "vertragsverlaengerung"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub vertragsverlaengerung: Option<Zeitraum>,
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
impl Default for Vertragskonditionen {
    fn default() -> Self {
        Self {
            abschlagszyklus: Default::default(),
            anzahl_abschlaege: Default::default(),
            beschreibung: Default::default(),
            id: Default::default(),
            kuendigungsfrist: Default::default(),
            typ: Some(ComTyp::Vertragskonditionen),
            version: Some("202607.1.0".to_owned()),
            vertragslaufzeit: Default::default(),
            vertragsverlaengerung: Default::default(),
            zusatz_attribute: Default::default(),
            _additional: Default::default(),
        }
    }
}
impl Bo4eTyped for Vertragskonditionen {
    type Typ = ComTyp;
    const TYP: ComTyp = ComTyp::Vertragskonditionen;
    const TYP_WIRE: &'static str = "VERTRAGSKONDITIONEN";
    const SCHEMA_VERSION: &'static str = "202607.1.0";
    const SCHEMA_SERIES: &'static str = "202607";
}
impl crate::bo4e_typed_sealed::Sealed for Vertragskonditionen {}
impl Bo4eComponent for Vertragskonditionen {}
impl crate::bo4e_component_sealed::Sealed for Vertragskonditionen {}
#[cfg(feature = "json")]
impl crate::json::sealed::Sealed for Vertragskonditionen {}
#[cfg(feature = "json")]
impl crate::json::Bo4eJsonExt for Vertragskonditionen {}
#[cfg(feature = "json")]
impl crate::json::Bo4eExtensionData for Vertragskonditionen {
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
impl std::fmt::Display for Vertragskonditionen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => f.write_str(&json),
            Err(e) => write!(f, "<Vertragskonditionen: serialization error: {e}>"),
        }
    }
}
impl crate::Bo4eStrict for Vertragskonditionen {
    fn collect_unknown_enums(&self, path: &str, out: &mut Vec<String>) {
        if let Some(v) = &self.abschlagszyklus {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "abschlagszyklus"),
                out,
            );
        }
        if let Some(v) = &self.kuendigungsfrist {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "kuendigungsfrist"),
                out,
            );
        }
        if let Some(v) = &self.vertragslaufzeit {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "vertragslaufzeit"),
                out,
            );
        }
        if let Some(v) = &self.vertragsverlaengerung {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "vertragsverlaengerung"),
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
impl crate::json::Bo4eExtensions for Vertragskonditionen {
    fn collect_extension_paths(&self, path: &str, out: &mut Vec<String>) {
        if let Some(map) = self._additional.as_map() {
            for key in map.keys() {
                out.push(crate::strict::extension_path(path, key));
            }
        }
        if let Some(v) = &self.abschlagszyklus {
            crate::json::Bo4eExtensions::collect_extension_paths(
                v,
                &crate::strict::field_path(path, "abschlagszyklus"),
                out,
            );
        }
        if let Some(v) = &self.kuendigungsfrist {
            crate::json::Bo4eExtensions::collect_extension_paths(
                v,
                &crate::strict::field_path(path, "kuendigungsfrist"),
                out,
            );
        }
        if let Some(v) = &self.vertragslaufzeit {
            crate::json::Bo4eExtensions::collect_extension_paths(
                v,
                &crate::strict::field_path(path, "vertragslaufzeit"),
                out,
            );
        }
        if let Some(v) = &self.vertragsverlaengerung {
            crate::json::Bo4eExtensions::collect_extension_paths(
                v,
                &crate::strict::field_path(path, "vertragsverlaengerung"),
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
