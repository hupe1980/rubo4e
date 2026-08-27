use super::{
    Bo4eComponent, Bo4eTyped, ComTyp, EinheitsPreisposition, LastvariablePreisposition,
    RelativePreisposition, Zeitraum, ZeitvariablePreisposition, ZusatzAttribut,
};
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(not(feature = "json"), derive(Eq, Hash))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "validate", derive(garde::Validate))]
#[cfg_attr(feature = "validate", garde(allow_unvalidated))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Mit dieser Komponente kann ein aus verschiedenen Preispositionen zusammengesetzter Tarifpreis zeitaufgelöst
/// dargestellt werden.
///
/// > **Note:** [Tarifpreiszeitscheibe JSON Schema](https://json-schema.app/view/%23?url=https://raw.githubusercontent.com/BO4E/BO4E-Schemas/v202607.1.0/src/bo4e_schemas/com/Tarifpreiszeitscheibe.json)
pub struct Tarifpreiszeitscheibe {
    /// Eine Liste von Einheits-Preispositionen.
    #[cfg_attr(feature = "serde", serde(rename = "einheitsPreispositionen"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
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
    #[cfg_attr(feature = "validate", garde(dive))]
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
    #[cfg_attr(feature = "validate", garde(dive))]
    pub relative_preispositionen: Option<Vec<RelativePreisposition>>,
    /// BO4E type discriminant — always `ComTyp::Tarifpreiszeitscheibe` for this struct.
    #[cfg_attr(feature = "serde", serde(rename = "_typ"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(
        feature = "builder",
        builder(default = Some(ComTyp::Tarifpreiszeitscheibe), setter(skip))
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
    /// Gibt an für welchen Zeitraum dieser zusammengesetzte Tarifpreis gültig ist.
    #[cfg_attr(feature = "serde", serde(rename = "zeitscheibengueltigkeit"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub zeitscheibengueltigkeit: Option<Zeitraum>,
    /// Eine Liste von zeitvariablen Preispositionen.
    /// Dies können z.B. Preispositionen mit Zählzeitdefinitionen sein, um ein klassisches HT/NT Modell abzubilden.
    #[cfg_attr(feature = "serde", serde(rename = "zeitvariablePreispositionen"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub zeitvariable_preispositionen: Option<Vec<ZeitvariablePreisposition>>,
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
impl Default for Tarifpreiszeitscheibe {
    fn default() -> Self {
        Self {
            einheits_preispositionen: Default::default(),
            id: Default::default(),
            lastvariable_preispositionen: Default::default(),
            relative_preispositionen: Default::default(),
            typ: Some(ComTyp::Tarifpreiszeitscheibe),
            version: Some("202607.1.0".to_owned()),
            zeitscheibengueltigkeit: Default::default(),
            zeitvariable_preispositionen: Default::default(),
            zusatz_attribute: Default::default(),
            _additional: Default::default(),
        }
    }
}
impl Bo4eTyped for Tarifpreiszeitscheibe {
    type Typ = ComTyp;
    const TYP: ComTyp = ComTyp::Tarifpreiszeitscheibe;
    const TYP_WIRE: &'static str = "TARIFPREISZEITSCHEIBE";
    const SCHEMA_VERSION: &'static str = "202607.1.0";
    const SCHEMA_SERIES: &'static str = "202607";
}
impl crate::bo4e_typed_sealed::Sealed for Tarifpreiszeitscheibe {}
impl Bo4eComponent for Tarifpreiszeitscheibe {}
impl crate::bo4e_component_sealed::Sealed for Tarifpreiszeitscheibe {}
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
impl crate::Bo4eStrict for Tarifpreiszeitscheibe {
    fn collect_unknown_enums(&self, path: &str, out: &mut Vec<String>) {
        if let Some(items) = &self.einheits_preispositionen {
            let child = crate::strict::field_path(path, "einheitsPreispositionen");
            for (i, item) in items.iter().enumerate() {
                crate::Bo4eStrict::collect_unknown_enums(
                    item,
                    &crate::strict::index_path(&child, i),
                    out,
                );
            }
        }
        if let Some(items) = &self.lastvariable_preispositionen {
            let child = crate::strict::field_path(path, "lastvariablePreispositionen");
            for (i, item) in items.iter().enumerate() {
                crate::Bo4eStrict::collect_unknown_enums(
                    item,
                    &crate::strict::index_path(&child, i),
                    out,
                );
            }
        }
        if let Some(items) = &self.relative_preispositionen {
            let child = crate::strict::field_path(path, "relativePreispositionen");
            for (i, item) in items.iter().enumerate() {
                crate::Bo4eStrict::collect_unknown_enums(
                    item,
                    &crate::strict::index_path(&child, i),
                    out,
                );
            }
        }
        if let Some(v) = &self.zeitscheibengueltigkeit {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "zeitscheibengueltigkeit"),
                out,
            );
        }
        if let Some(items) = &self.zeitvariable_preispositionen {
            let child = crate::strict::field_path(path, "zeitvariablePreispositionen");
            for (i, item) in items.iter().enumerate() {
                crate::Bo4eStrict::collect_unknown_enums(
                    item,
                    &crate::strict::index_path(&child, i),
                    out,
                );
            }
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
impl crate::json::Bo4eExtensions for Tarifpreiszeitscheibe {
    fn collect_extension_paths(&self, path: &str, out: &mut Vec<String>) {
        if let Some(map) = self._additional.as_map() {
            for key in map.keys() {
                out.push(crate::strict::extension_path(path, key));
            }
        }
        if let Some(items) = &self.einheits_preispositionen {
            let child = crate::strict::field_path(path, "einheitsPreispositionen");
            for (i, item) in items.iter().enumerate() {
                crate::json::Bo4eExtensions::collect_extension_paths(
                    item,
                    &crate::strict::index_path(&child, i),
                    out,
                );
            }
        }
        if let Some(items) = &self.lastvariable_preispositionen {
            let child = crate::strict::field_path(path, "lastvariablePreispositionen");
            for (i, item) in items.iter().enumerate() {
                crate::json::Bo4eExtensions::collect_extension_paths(
                    item,
                    &crate::strict::index_path(&child, i),
                    out,
                );
            }
        }
        if let Some(items) = &self.relative_preispositionen {
            let child = crate::strict::field_path(path, "relativePreispositionen");
            for (i, item) in items.iter().enumerate() {
                crate::json::Bo4eExtensions::collect_extension_paths(
                    item,
                    &crate::strict::index_path(&child, i),
                    out,
                );
            }
        }
        if let Some(v) = &self.zeitscheibengueltigkeit {
            crate::json::Bo4eExtensions::collect_extension_paths(
                v,
                &crate::strict::field_path(path, "zeitscheibengueltigkeit"),
                out,
            );
        }
        if let Some(items) = &self.zeitvariable_preispositionen {
            let child = crate::strict::field_path(path, "zeitvariablePreispositionen");
            for (i, item) in items.iter().enumerate() {
                crate::json::Bo4eExtensions::collect_extension_paths(
                    item,
                    &crate::strict::index_path(&child, i),
                    out,
                );
            }
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
