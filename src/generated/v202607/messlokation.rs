use super::{
    Adresse, Bo4eObject, Bo4eTyped, BoTyp, Dienstleistung, Geokoordinaten, Geraet, Katasteradresse,
    Lokationszuordnung, Netzebene, Sparte, Zaehler, ZusatzAttribut,
};
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(not(feature = "json"), derive(Eq, Hash))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "validate", derive(garde::Validate))]
#[cfg_attr(feature = "validate", garde(allow_unvalidated))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(
    all(feature = "validate", feature = "versioned"),
    garde(custom(crate::validation::v202607::validate_messlokation))
)]
/// Object containing information about a Messlokation
///
/// > **Note:** [Messlokation JSON Schema](https://json-schema.app/view/%23?url=https://raw.githubusercontent.com/BO4E/BO4E-Schemas/v202607.1.0/src/bo4e_schemas/bo/Messlokation.json)
pub struct Messlokation {
    /// Alternativ zu einer postalischen Adresse kann hier ein Ort mittels Geokoordinaten angegeben werden
    /// (z.B. zur Identifikation von Sendemasten).
    #[cfg_attr(feature = "serde", serde(rename = "geoadresse"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub geoadresse: Option<Geokoordinaten>,
    /// Liste der Geräte, die zu dieser Messstelle gehört
    #[cfg_attr(feature = "serde", serde(rename = "geraete"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub geraete: Option<Vec<Box<Geraet>>>,
    /// Codenummer des grundzuständigen Messstellenbetreibers, der für diese Messlokation zuständig ist.
    /// (Dieser ist immer dann Messstellenbetreiber, wenn kein anderer MSB die Einrichtungen an der Messlokation betreibt.)
    #[cfg_attr(feature = "serde", serde(rename = "grundzustaendigerMsbCodenr"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub grundzustaendiger_msb_codenr: Option<crate::identifiers::MarktpartnerId>,
    /// Codenummer des grundzuständigen Messstellenbetreibers für intelligente Messsysteme, der für diese Messlokation
    /// zuständig ist.
    /// (Dieser ist immer dann Messstellenbetreiber, wenn kein anderer MSB die Einrichtungen an der Messlokation betreibt.)
    #[cfg_attr(feature = "serde", serde(rename = "grundzustaendigerMsbimCodenr"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub grundzustaendiger_msbim_codenr: Option<crate::identifiers::MarktpartnerId>,
    /// Eine generische ID, die für eigene Zwecke genutzt werden kann.
    /// Z.B. könnten hier UUIDs aus einer Datenbank stehen oder URLs zu einem Backend-System.
    #[cfg_attr(feature = "serde", serde(rename = "_id"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub id: Option<String>,
    /// Alternativ zu einer postalischen Adresse und Geokoordinaten kann hier eine Ortsangabe mittels Gemarkung und
    /// Flurstück erfolgen.
    #[cfg_attr(feature = "serde", serde(rename = "katasterinformation"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub katasterinformation: Option<Katasteradresse>,
    /// Lokationsbuendel Code, der die Funktion dieses BOs an der Lokationsbuendelstruktur beschreibt.
    #[cfg_attr(feature = "serde", serde(rename = "lokationsbuendelObjektcode"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub lokationsbuendel_objektcode: Option<String>,
    /// Lokationszuordnung, um bspw. die zugehörigen Marktlokationen anzugeben
    #[cfg_attr(feature = "serde", serde(rename = "lokationszuordnungen"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub lokationszuordnungen: Option<Vec<Box<Lokationszuordnung>>>,
    /// Die Adresse, an der die Messeinrichtungen zu finden sind.
    /// (Nur angeben, wenn diese von der Adresse der Marktlokation abweicht.)
    #[cfg_attr(feature = "serde", serde(rename = "messadresse"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub messadresse: Option<Adresse>,
    /// Liste der Messdienstleistungen, die zu dieser Messstelle gehört
    #[cfg_attr(feature = "serde", serde(rename = "messdienstleistung"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub messdienstleistung: Option<Vec<Dienstleistung>>,
    /// Die Nummer des Messgebietes in der ene't-Datenbank
    #[cfg_attr(feature = "serde", serde(rename = "messgebietnr"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub messgebietnr: Option<String>,
    /// Die Messlokations-Identifikation; Das ist die frühere Zählpunktbezeichnung
    #[cfg_attr(feature = "serde", serde(rename = "messlokationsId"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub messlokations_id: Option<crate::identifiers::MeloId>,
    /// Zähler, die zu dieser Messlokation gehören
    #[cfg_attr(feature = "serde", serde(rename = "messlokationszaehler"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub messlokationszaehler: Option<Vec<Box<Zaehler>>>,
    /// Spannungsebene der Messung
    #[cfg_attr(feature = "serde", serde(rename = "netzebeneMessung"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub netzebene_messung: Option<Netzebene>,
    /// Sparte der Messlokation, z.B. Gas oder Strom
    #[cfg_attr(feature = "serde", serde(rename = "sparte"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub sparte: Option<Sparte>,
    /// BO4E type discriminant — always `BoTyp::Messlokation` for this struct.
    #[cfg_attr(feature = "serde", serde(rename = "_typ"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(
        feature = "builder",
        builder(default = Some(BoTyp::Messlokation), setter(skip))
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
impl Default for Messlokation {
    fn default() -> Self {
        Self {
            geoadresse: Default::default(),
            geraete: Default::default(),
            grundzustaendiger_msb_codenr: Default::default(),
            grundzustaendiger_msbim_codenr: Default::default(),
            id: Default::default(),
            katasterinformation: Default::default(),
            lokationsbuendel_objektcode: Default::default(),
            lokationszuordnungen: Default::default(),
            messadresse: Default::default(),
            messdienstleistung: Default::default(),
            messgebietnr: Default::default(),
            messlokations_id: Default::default(),
            messlokationszaehler: Default::default(),
            netzebene_messung: Default::default(),
            sparte: Default::default(),
            typ: Some(BoTyp::Messlokation),
            version: Some("202607.1.0".to_owned()),
            zusatz_attribute: Default::default(),
            _additional: Default::default(),
        }
    }
}
impl Bo4eTyped for Messlokation {
    type Typ = BoTyp;
    const TYP: BoTyp = BoTyp::Messlokation;
    const TYP_WIRE: &'static str = "MESSLOKATION";
    const SCHEMA_VERSION: &'static str = "202607.1.0";
    const SCHEMA_SERIES: &'static str = "202607";
}
impl crate::bo4e_typed_sealed::Sealed for Messlokation {}
impl Bo4eObject for Messlokation {}
impl crate::bo4e_object_sealed::Sealed for Messlokation {}
#[cfg(feature = "json")]
impl crate::json::sealed::Sealed for Messlokation {}
#[cfg(feature = "json")]
impl crate::json::Bo4eJsonExt for Messlokation {}
#[cfg(feature = "json")]
impl crate::json::Bo4eExtensionData for Messlokation {
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
impl std::fmt::Display for Messlokation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => f.write_str(&json),
            Err(e) => write!(f, "<Messlokation: serialization error: {e}>"),
        }
    }
}
impl crate::Bo4eStrict for Messlokation {
    fn collect_unknown_enums(&self, path: &str, out: &mut Vec<String>) {
        if let Some(v) = &self.geoadresse {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "geoadresse"),
                out,
            );
        }
        if let Some(items) = &self.geraete {
            let child = crate::strict::field_path(path, "geraete");
            for (i, item) in items.iter().enumerate() {
                crate::Bo4eStrict::collect_unknown_enums(
                    &**item,
                    &crate::strict::index_path(&child, i),
                    out,
                );
            }
        }
        if let Some(v) = &self.katasterinformation {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "katasterinformation"),
                out,
            );
        }
        if let Some(items) = &self.lokationszuordnungen {
            let child = crate::strict::field_path(path, "lokationszuordnungen");
            for (i, item) in items.iter().enumerate() {
                crate::Bo4eStrict::collect_unknown_enums(
                    &**item,
                    &crate::strict::index_path(&child, i),
                    out,
                );
            }
        }
        if let Some(v) = &self.messadresse {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "messadresse"),
                out,
            );
        }
        if let Some(items) = &self.messdienstleistung {
            let child = crate::strict::field_path(path, "messdienstleistung");
            for (i, item) in items.iter().enumerate() {
                crate::Bo4eStrict::collect_unknown_enums(
                    item,
                    &crate::strict::index_path(&child, i),
                    out,
                );
            }
        }
        if let Some(items) = &self.messlokationszaehler {
            let child = crate::strict::field_path(path, "messlokationszaehler");
            for (i, item) in items.iter().enumerate() {
                crate::Bo4eStrict::collect_unknown_enums(
                    &**item,
                    &crate::strict::index_path(&child, i),
                    out,
                );
            }
        }
        if let Some(v) = &self.netzebene_messung {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "netzebeneMessung"),
                out,
            );
        }
        if let Some(v) = &self.sparte {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "sparte"),
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
impl crate::json::Bo4eExtensions for Messlokation {
    fn collect_extension_paths(&self, path: &str, out: &mut Vec<String>) {
        if let Some(map) = self._additional.as_map() {
            for key in map.keys() {
                out.push(crate::strict::extension_path(path, key));
            }
        }
        if let Some(v) = &self.geoadresse {
            crate::json::Bo4eExtensions::collect_extension_paths(
                v,
                &crate::strict::field_path(path, "geoadresse"),
                out,
            );
        }
        if let Some(items) = &self.geraete {
            let child = crate::strict::field_path(path, "geraete");
            for (i, item) in items.iter().enumerate() {
                crate::json::Bo4eExtensions::collect_extension_paths(
                    &**item,
                    &crate::strict::index_path(&child, i),
                    out,
                );
            }
        }
        if let Some(v) = &self.katasterinformation {
            crate::json::Bo4eExtensions::collect_extension_paths(
                v,
                &crate::strict::field_path(path, "katasterinformation"),
                out,
            );
        }
        if let Some(items) = &self.lokationszuordnungen {
            let child = crate::strict::field_path(path, "lokationszuordnungen");
            for (i, item) in items.iter().enumerate() {
                crate::json::Bo4eExtensions::collect_extension_paths(
                    &**item,
                    &crate::strict::index_path(&child, i),
                    out,
                );
            }
        }
        if let Some(v) = &self.messadresse {
            crate::json::Bo4eExtensions::collect_extension_paths(
                v,
                &crate::strict::field_path(path, "messadresse"),
                out,
            );
        }
        if let Some(items) = &self.messdienstleistung {
            let child = crate::strict::field_path(path, "messdienstleistung");
            for (i, item) in items.iter().enumerate() {
                crate::json::Bo4eExtensions::collect_extension_paths(
                    item,
                    &crate::strict::index_path(&child, i),
                    out,
                );
            }
        }
        if let Some(items) = &self.messlokationszaehler {
            let child = crate::strict::field_path(path, "messlokationszaehler");
            for (i, item) in items.iter().enumerate() {
                crate::json::Bo4eExtensions::collect_extension_paths(
                    &**item,
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
