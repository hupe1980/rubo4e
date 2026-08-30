use super::{
    Bo4eObject, Bo4eTyped, BoTyp, Medium, Mengeneinheit, Messart, Messgroesse, Messwertstatus,
    Zeitreihenwert, ZusatzAttribut,
};
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(not(feature = "json"), derive(Eq, Hash))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "validate", derive(garde::Validate))]
#[cfg_attr(feature = "validate", garde(allow_unvalidated))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Abbildung einer allgemeinen Zeitreihe mit einem Wertvektor.
/// Die Werte können mit wahlfreier zeitlicher Distanz im Vektor abgelegt sein.
///
/// > **Note:** [Zeitreihe JSON Schema](https://json-schema.app/view/%23?url=https://raw.githubusercontent.com/BO4E/BO4E-Schemas/v202607.1.0/src/bo4e_schemas/bo/Zeitreihe.json)
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
    /// BO4E type discriminant — always `BoTyp::Zeitreihe` for this struct.
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
    #[cfg_attr(feature = "validate", garde(dive))]
    pub werte: Option<Vec<Zeitreihenwert>>,
    /// Kennzeichnung, wie die Werte entstanden sind, z.B. ABGELESEN oder PROGNOSEWERT
    #[cfg_attr(feature = "serde", serde(rename = "wertherkunft"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub wertherkunft: Option<Messwertstatus>,
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
impl Default for Zeitreihe {
    fn default() -> Self {
        Self {
            beschreibung: Default::default(),
            bezeichnung: Default::default(),
            einheit: Default::default(),
            id: Default::default(),
            medium: Default::default(),
            messart: Default::default(),
            messgroesse: Default::default(),
            typ: Some(BoTyp::Zeitreihe),
            version: Default::default(),
            werte: Default::default(),
            wertherkunft: Default::default(),
            zusatz_attribute: Default::default(),
            _additional: Default::default(),
        }
    }
}
impl Bo4eTyped for Zeitreihe {
    type Typ = BoTyp;
    const TYP: BoTyp = BoTyp::Zeitreihe;
    const TYP_WIRE: &'static str = "ZEITREIHE";
    const SCHEMA_VERSION: &'static str = "202607.1.0";
    const SCHEMA_SERIES: &'static str = "202607";
}
impl crate::bo4e_typed_sealed::Sealed for Zeitreihe {}
impl Bo4eObject for Zeitreihe {}
impl crate::bo4e_object_sealed::Sealed for Zeitreihe {}
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
impl crate::Bo4eStrict for Zeitreihe {
    fn collect_unknown_enums(&self, path: &str, out: &mut Vec<String>) {
        if let Some(v) = &self.einheit {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "einheit"),
                out,
            );
        }
        if let Some(v) = &self.medium {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "medium"),
                out,
            );
        }
        if let Some(v) = &self.messart {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "messart"),
                out,
            );
        }
        if let Some(v) = &self.messgroesse {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "messgroesse"),
                out,
            );
        }
        if let Some(items) = &self.werte {
            let child = crate::strict::field_path(path, "werte");
            for (i, item) in items.iter().enumerate() {
                crate::Bo4eStrict::collect_unknown_enums(
                    item,
                    &crate::strict::index_path(&child, i),
                    out,
                );
            }
        }
        if let Some(v) = &self.wertherkunft {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "wertherkunft"),
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
impl crate::json::Bo4eExtensions for Zeitreihe {
    fn collect_extension_paths(&self, path: &str, out: &mut Vec<String>) {
        if let Some(map) = self._additional.as_map() {
            for key in map.keys() {
                out.push(crate::strict::extension_path(path, key));
            }
        }
        if let Some(items) = &self.werte {
            let child = crate::strict::field_path(path, "werte");
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
impl crate::zusatz_attribut::HasZusatzAttribute for Zeitreihe {
    fn zusatz_attribute_field(&self) -> Option<&Vec<ZusatzAttribut>> {
        self.zusatz_attribute.as_ref()
    }
    fn zusatz_attribute_field_mut(&mut self) -> &mut Option<Vec<ZusatzAttribut>> {
        &mut self.zusatz_attribute
    }
}
