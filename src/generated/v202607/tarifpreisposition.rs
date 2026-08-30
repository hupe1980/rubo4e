use super::{
    Bo4eComponent, Bo4eTyped, ComTyp, Mengeneinheit, Preisstaffel, Preistyp, Waehrungseinheit,
    ZusatzAttribut,
};
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(not(feature = "json"), derive(Eq, Hash))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "validate", derive(garde::Validate))]
#[cfg_attr(feature = "validate", garde(allow_unvalidated))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Mit dieser Komponente können Tarifpreise verschiedener Typen abgebildet werden.
///
/// > **Note:** [Tarifpreisposition JSON Schema](https://json-schema.app/view/%23?url=https://raw.githubusercontent.com/BO4E/BO4E-Schemas/v202607.1.0/src/bo4e_schemas/com/Tarifpreisposition.json)
pub struct Tarifpreisposition {
    /// Größe, auf die sich die Einheit bezieht, beispielsweise kWh, Jahr
    #[cfg_attr(feature = "serde", serde(rename = "bezugseinheit"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub bezugseinheit: Option<Mengeneinheit>,
    /// Einheit des Preises (z.B. EURO)
    #[cfg_attr(feature = "serde", serde(rename = "einheit"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub einheit: Option<Waehrungseinheit>,
    /// Eine generische ID, die für eigene Zwecke genutzt werden kann.
    /// Z.B. könnten hier UUIDs aus einer Datenbank stehen oder URLs zu einem Backend-System.
    #[cfg_attr(feature = "serde", serde(rename = "_id"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub id: Option<String>,
    /// Gibt an, nach welcher Menge die vorgenannte Einschränkung erfolgt (z.B. Jahresstromverbrauch in kWh)
    #[cfg_attr(feature = "serde", serde(rename = "mengeneinheitstaffel"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub mengeneinheitstaffel: Option<Mengeneinheit>,
    /// Hier sind die Staffeln mit ihren Preisenangaben definiert
    #[cfg_attr(feature = "serde", serde(rename = "preisstaffeln"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub preisstaffeln: Option<Vec<Preisstaffel>>,
    /// Angabe des Preistypes (z.B. Grundpreis)
    #[cfg_attr(feature = "serde", serde(rename = "preistyp"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub preistyp: Option<Preistyp>,
    /// BO4E type discriminant — always `ComTyp::Tarifpreisposition` for this struct.
    #[cfg_attr(feature = "serde", serde(rename = "_typ"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(
        feature = "builder",
        builder(default = Some(ComTyp::Tarifpreisposition), setter(skip))
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
impl Default for Tarifpreisposition {
    fn default() -> Self {
        Self {
            bezugseinheit: Default::default(),
            einheit: Default::default(),
            id: Default::default(),
            mengeneinheitstaffel: Default::default(),
            preisstaffeln: Default::default(),
            preistyp: Default::default(),
            typ: Some(ComTyp::Tarifpreisposition),
            version: Some("202607.1.0".to_owned()),
            zusatz_attribute: Default::default(),
            _additional: Default::default(),
        }
    }
}
impl Bo4eTyped for Tarifpreisposition {
    type Typ = ComTyp;
    const TYP: ComTyp = ComTyp::Tarifpreisposition;
    const TYP_WIRE: &'static str = "TARIFPREISPOSITION";
    const SCHEMA_VERSION: &'static str = "202607.1.0";
    const SCHEMA_SERIES: &'static str = "202607";
}
impl crate::bo4e_typed_sealed::Sealed for Tarifpreisposition {}
impl Bo4eComponent for Tarifpreisposition {}
impl crate::bo4e_component_sealed::Sealed for Tarifpreisposition {}
#[cfg(feature = "json")]
impl crate::json::sealed::Sealed for Tarifpreisposition {}
#[cfg(feature = "json")]
impl crate::json::Bo4eJsonExt for Tarifpreisposition {}
#[cfg(feature = "json")]
impl crate::json::Bo4eExtensionData for Tarifpreisposition {
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
impl std::fmt::Display for Tarifpreisposition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => f.write_str(&json),
            Err(e) => write!(f, "<Tarifpreisposition: serialization error: {e}>"),
        }
    }
}
impl crate::Bo4eStrict for Tarifpreisposition {
    fn collect_unknown_enums(&self, path: &str, out: &mut Vec<String>) {
        if let Some(v) = &self.bezugseinheit {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "bezugseinheit"),
                out,
            );
        }
        if let Some(v) = &self.einheit {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "einheit"),
                out,
            );
        }
        if let Some(v) = &self.mengeneinheitstaffel {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "mengeneinheitstaffel"),
                out,
            );
        }
        if let Some(items) = &self.preisstaffeln {
            let child = crate::strict::field_path(path, "preisstaffeln");
            for (i, item) in items.iter().enumerate() {
                crate::Bo4eStrict::collect_unknown_enums(
                    item,
                    &crate::strict::index_path(&child, i),
                    out,
                );
            }
        }
        if let Some(v) = &self.preistyp {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "preistyp"),
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
impl crate::json::Bo4eExtensions for Tarifpreisposition {
    fn collect_extension_paths(&self, path: &str, out: &mut Vec<String>) {
        if let Some(map) = self._additional.as_map() {
            for key in map.keys() {
                out.push(crate::strict::extension_path(path, key));
            }
        }
        if let Some(items) = &self.preisstaffeln {
            let child = crate::strict::field_path(path, "preisstaffeln");
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
impl crate::zusatz_attribut::HasZusatzAttribute for Tarifpreisposition {
    fn zusatz_attribute_field(&self) -> Option<&Vec<ZusatzAttribut>> {
        self.zusatz_attribute.as_ref()
    }
    fn zusatz_attribute_field_mut(&mut self) -> &mut Option<Vec<ZusatzAttribut>> {
        &mut self.zusatz_attribute
    }
}
