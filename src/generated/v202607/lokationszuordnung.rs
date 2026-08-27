use super::{
    Bo4eObject, Bo4eTyped, BoTyp, Marktlokation, Messlokation, Netzlokation, SteuerbareRessource,
    TechnischeRessource, Zeitraum, ZusatzAttribut,
};
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(not(feature = "json"), derive(Eq, Hash))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "validate", derive(garde::Validate))]
#[cfg_attr(feature = "validate", garde(allow_unvalidated))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Modell für die Abbildung der Referenz auf die Lokationsbündelstruktur. Diese gibt an welche Marktlokationen,
/// Messlokationen, Netzlokationen, technische/steuerbaren Ressourcen an einer Lokation vorhanden sind.
///
/// > **Note:** [Lokationszuordnung JSON Schema](https://json-schema.app/view/%23?url=https://raw.githubusercontent.com/BO4E/BO4E-Schemas/v202607.1.0/src/bo4e_schemas/bo/Lokationszuordnung.json)
pub struct Lokationszuordnung {
    /// Zeitraum der Gültigkeit
    #[cfg_attr(feature = "serde", serde(rename = "gueltigkeit"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub gueltigkeit: Option<Zeitraum>,
    /// Eine generische ID, die für eigene Zwecke genutzt werden kann.
    /// Z.B. könnten hier UUIDs aus einer Datenbank stehen oder URLs zu einem Backend-System.
    #[cfg_attr(feature = "serde", serde(rename = "_id"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub id: Option<String>,
    /// Code, der angibt wie die Lokationsbündelstruktur zusammengesetzt ist (zu finden unter "Codeliste der Lokationsbündelstrukturen" auf <https://www.edi-energy.de/index.php?id=38>)
    #[cfg_attr(feature = "serde", serde(rename = "lokationsbuendelcode"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub lokationsbuendelcode: Option<String>,
    /// Liste mit referenzierten Marktlokationen
    #[cfg_attr(feature = "serde", serde(rename = "marktlokationen"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub marktlokationen: Option<Vec<Box<Marktlokation>>>,
    /// Liste mit referenzierten Messlokationen
    #[cfg_attr(feature = "serde", serde(rename = "messlokationen"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub messlokationen: Option<Vec<Box<Messlokation>>>,
    /// Liste mit referenzierten Netzlokationen
    #[cfg_attr(feature = "serde", serde(rename = "netzlokationen"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub netzlokationen: Option<Vec<Box<Netzlokation>>>,
    /// Liste mit referenzierten steuerbaren Ressourcen
    #[cfg_attr(feature = "serde", serde(rename = "steuerbareRessourcen"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub steuerbare_ressourcen: Option<Vec<Box<SteuerbareRessource>>>,
    /// Liste mit referenzierten technischen Ressourcen
    #[cfg_attr(feature = "serde", serde(rename = "technischeRessourcen"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub technische_ressourcen: Option<Vec<Box<TechnischeRessource>>>,
    /// BO4E type discriminant — always `BoTyp::Lokationszuordnung` for this struct.
    #[cfg_attr(feature = "serde", serde(rename = "_typ"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(
        feature = "builder",
        builder(default = Some(BoTyp::Lokationszuordnung), setter(skip))
    )]
    pub typ: Option<BoTyp>,
    /// Version der BO-Struktur aka "fachliche Versionierung"
    #[cfg_attr(feature = "serde", serde(rename = "_version"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(
        feature = "builder",
        builder(default = Some("202607.1.0".to_owned()), setter(into))
    )]
    pub version: Option<String>,
    /// Verknüpfungsrichtung z.B. Malo-Melo [TODO: Eventuell anderer Datentyp]
    #[cfg_attr(feature = "serde", serde(rename = "zuordnungstyp"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub zuordnungstyp: Option<String>,
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
impl Default for Lokationszuordnung {
    fn default() -> Self {
        Self {
            gueltigkeit: Default::default(),
            id: Default::default(),
            lokationsbuendelcode: Default::default(),
            marktlokationen: Default::default(),
            messlokationen: Default::default(),
            netzlokationen: Default::default(),
            steuerbare_ressourcen: Default::default(),
            technische_ressourcen: Default::default(),
            typ: Some(BoTyp::Lokationszuordnung),
            version: Some("202607.1.0".to_owned()),
            zuordnungstyp: Default::default(),
            zusatz_attribute: Default::default(),
            _additional: Default::default(),
        }
    }
}
impl Bo4eTyped for Lokationszuordnung {
    type Typ = BoTyp;
    const TYP: BoTyp = BoTyp::Lokationszuordnung;
    const TYP_WIRE: &'static str = "LOKATIONSZUORDNUNG";
    const SCHEMA_VERSION: &'static str = "202607.1.0";
    const SCHEMA_SERIES: &'static str = "202607";
}
impl crate::bo4e_typed_sealed::Sealed for Lokationszuordnung {}
impl Bo4eObject for Lokationszuordnung {}
impl crate::bo4e_object_sealed::Sealed for Lokationszuordnung {}
#[cfg(feature = "json")]
impl crate::json::sealed::Sealed for Lokationszuordnung {}
#[cfg(feature = "json")]
impl crate::json::Bo4eJsonExt for Lokationszuordnung {}
#[cfg(feature = "json")]
impl crate::json::Bo4eExtensionData for Lokationszuordnung {
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
impl std::fmt::Display for Lokationszuordnung {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => f.write_str(&json),
            Err(e) => write!(f, "<Lokationszuordnung: serialization error: {e}>"),
        }
    }
}
impl crate::Bo4eStrict for Lokationszuordnung {
    fn collect_unknown_enums(&self, path: &str, out: &mut Vec<String>) {
        if let Some(v) = &self.gueltigkeit {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "gueltigkeit"),
                out,
            );
        }
        if let Some(items) = &self.marktlokationen {
            let child = crate::strict::field_path(path, "marktlokationen");
            for (i, item) in items.iter().enumerate() {
                crate::Bo4eStrict::collect_unknown_enums(
                    &**item,
                    &crate::strict::index_path(&child, i),
                    out,
                );
            }
        }
        if let Some(items) = &self.messlokationen {
            let child = crate::strict::field_path(path, "messlokationen");
            for (i, item) in items.iter().enumerate() {
                crate::Bo4eStrict::collect_unknown_enums(
                    &**item,
                    &crate::strict::index_path(&child, i),
                    out,
                );
            }
        }
        if let Some(items) = &self.netzlokationen {
            let child = crate::strict::field_path(path, "netzlokationen");
            for (i, item) in items.iter().enumerate() {
                crate::Bo4eStrict::collect_unknown_enums(
                    &**item,
                    &crate::strict::index_path(&child, i),
                    out,
                );
            }
        }
        if let Some(items) = &self.steuerbare_ressourcen {
            let child = crate::strict::field_path(path, "steuerbareRessourcen");
            for (i, item) in items.iter().enumerate() {
                crate::Bo4eStrict::collect_unknown_enums(
                    &**item,
                    &crate::strict::index_path(&child, i),
                    out,
                );
            }
        }
        if let Some(items) = &self.technische_ressourcen {
            let child = crate::strict::field_path(path, "technischeRessourcen");
            for (i, item) in items.iter().enumerate() {
                crate::Bo4eStrict::collect_unknown_enums(
                    &**item,
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
