use super::{
    Bo4eComponent, Bo4eTyped, ComTyp, Preis, Preisreferenz, Zaehlzeitdefinition, ZusatzAttribut,
};
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(not(feature = "json"), derive(Eq, Hash))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "validate", derive(garde::Validate))]
#[cfg_attr(feature = "validate", garde(allow_unvalidated))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Modelliert eine zeitvariable Preisposition.
///
/// > **Note:** [ZeitvariablePreisposition JSON Schema](https://json-schema.app/view/%23?url=https://raw.githubusercontent.com/BO4E/BO4E-Schemas/v202607.1.0/src/bo4e_schemas/com/ZeitvariablePreisposition.json)
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
    #[cfg_attr(feature = "validate", garde(dive))]
    pub preis: Option<Preis>,
    /// Die Referenz worauf sich der Preis bezieht.
    /// Die explizite Einheit wird durch das Feld `bezugswert` im `COM Preis` angegeben.
    #[cfg_attr(feature = "serde", serde(rename = "preisreferenz"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub preisreferenz: Option<Preisreferenz>,
    /// BO4E type discriminant — always `ComTyp::ZeitvariablePreisposition` for this struct.
    #[cfg_attr(feature = "serde", serde(rename = "_typ"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(
        feature = "builder",
        builder(default = Some(ComTyp::ZeitvariablePreisposition), setter(skip))
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
    /// Die Zählzeitdefinition, deren Schaltschema bestimmt, wann diese Preisposition gilt.
    #[cfg_attr(feature = "serde", serde(rename = "zaehlzeitdefinition"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
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
impl Default for ZeitvariablePreisposition {
    fn default() -> Self {
        Self {
            bezeichnung: Default::default(),
            id: Default::default(),
            preis: Default::default(),
            preisreferenz: Default::default(),
            typ: Some(ComTyp::ZeitvariablePreisposition),
            version: Some("202607.1.0".to_owned()),
            zaehlzeitdefinition: Default::default(),
            zaehlzeitregister: Default::default(),
            zusatz_attribute: Default::default(),
            _additional: Default::default(),
        }
    }
}
impl Bo4eTyped for ZeitvariablePreisposition {
    type Typ = ComTyp;
    const TYP: ComTyp = ComTyp::ZeitvariablePreisposition;
    const TYP_WIRE: &'static str = "ZEITVARIABLEPREISPOSITION";
    const SCHEMA_VERSION: &'static str = "202607.1.0";
    const SCHEMA_SERIES: &'static str = "202607";
}
impl crate::bo4e_typed_sealed::Sealed for ZeitvariablePreisposition {}
impl Bo4eComponent for ZeitvariablePreisposition {}
impl crate::bo4e_component_sealed::Sealed for ZeitvariablePreisposition {}
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
impl crate::Bo4eStrict for ZeitvariablePreisposition {
    fn collect_unknown_enums(&self, path: &str, out: &mut Vec<String>) {
        if let Some(v) = &self.preis {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "preis"),
                out,
            );
        }
        if let Some(v) = &self.preisreferenz {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "preisreferenz"),
                out,
            );
        }
        if let Some(v) = &self.zaehlzeitdefinition {
            crate::Bo4eStrict::collect_unknown_enums(
                &**v,
                &crate::strict::field_path(path, "zaehlzeitdefinition"),
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
impl crate::json::Bo4eExtensions for ZeitvariablePreisposition {
    fn collect_extension_paths(&self, path: &str, out: &mut Vec<String>) {
        if let Some(map) = self._additional.as_map() {
            for key in map.keys() {
                out.push(crate::strict::extension_path(path, key));
            }
        }
        if let Some(v) = &self.preis {
            crate::json::Bo4eExtensions::collect_extension_paths(
                v,
                &crate::strict::field_path(path, "preis"),
                out,
            );
        }
        if let Some(v) = &self.zaehlzeitdefinition {
            crate::json::Bo4eExtensions::collect_extension_paths(
                &**v,
                &crate::strict::field_path(path, "zaehlzeitdefinition"),
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
