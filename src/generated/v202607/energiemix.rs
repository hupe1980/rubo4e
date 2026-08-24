use super::{ComTyp, Energieherkunft, Oekolabel, Oekozertifikat, Sparte, ZusatzAttribut};
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(not(feature = "json"), derive(Eq, Hash))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "validate", derive(garde::Validate))]
#[cfg_attr(feature = "validate", garde(allow_unvalidated))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Zusammensetzung der gelieferten Energie aus den verschiedenen Primärenergieformen.
///
/// > **Note:** [Energiemix JSON Schema](https://json-schema.app/view/%23?url=https://raw.githubusercontent.com/BO4E/BO4E-Schemas/v202607.1.0/src/bo4e_schemas/com/Energiemix.json)
pub struct Energiemix {
    /// Anteile der jeweiligen Erzeugungsart
    #[cfg_attr(feature = "serde", serde(rename = "anteil"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub anteil: Option<Vec<Energieherkunft>>,
    /// Höhe des erzeugten Atommülls in g/kWh
    #[cfg_attr(feature = "serde", serde(rename = "atommuell"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "crate::decimal_serde::deserialize_opt")
    )]
    #[cfg(feature = "decimal")]
    pub atommuell: Option<rust_decimal::Decimal>,
    /// Requires the `decimal` feature for the `rust_decimal::Decimal` representation.
    /// Without `decimal`, stores the decimal's lexical form (a JSON string or number).
    #[cfg_attr(feature = "serde", serde(rename = "atommuell"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "crate::decimal_serde::deserialize_opt")
    )]
    #[cfg(not(feature = "decimal"))]
    pub atommuell: Option<String>,
    /// Bemerkung zum Energiemix
    #[cfg_attr(feature = "serde", serde(rename = "bemerkung"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub bemerkung: Option<String>,
    /// Bezeichnung des Energiemix
    #[cfg_attr(feature = "serde", serde(rename = "bezeichnung"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub bezeichnung: Option<String>,
    /// Höhe des erzeugten CO2-Ausstosses in g/kWh
    #[cfg_attr(feature = "serde", serde(rename = "co2Emission"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "crate::decimal_serde::deserialize_opt")
    )]
    #[cfg(feature = "decimal")]
    pub co2_emission: Option<rust_decimal::Decimal>,
    /// Requires the `decimal` feature for the `rust_decimal::Decimal` representation.
    /// Without `decimal`, stores the decimal's lexical form (a JSON string or number).
    #[cfg_attr(feature = "serde", serde(rename = "co2Emission"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "crate::decimal_serde::deserialize_opt")
    )]
    #[cfg(not(feature = "decimal"))]
    pub co2_emission: Option<String>,
    /// Strom oder Gas etc.
    #[cfg_attr(feature = "serde", serde(rename = "energieart"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub energieart: Option<Sparte>,
    /// Eindeutige Nummer zur Identifizierung des Energiemixes
    #[cfg_attr(feature = "serde", serde(rename = "energiemixnummer"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub energiemixnummer: Option<i64>,
    /// Jahr, für das der Energiemix gilt
    #[cfg_attr(feature = "serde", serde(rename = "gueltigkeitsjahr"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub gueltigkeitsjahr: Option<i64>,
    /// Eine generische ID, die für eigene Zwecke genutzt werden kann.
    /// Z.B. könnten hier UUIDs aus einer Datenbank stehen oder URLs zu einem Backend-System.
    #[cfg_attr(feature = "serde", serde(rename = "_id"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub id: Option<String>,
    /// Kennzeichen, ob der Versorger zu den Öko Top Ten gehört
    #[cfg_attr(feature = "serde", serde(rename = "istInOekoTopTen"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub ist_in_oeko_top_ten: Option<bool>,
    /// Ökolabel für den Energiemix
    #[cfg_attr(feature = "serde", serde(rename = "oekolabel"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub oekolabel: Option<Vec<Oekolabel>>,
    /// Zertifikate für den Energiemix
    #[cfg_attr(feature = "serde", serde(rename = "oekozertifikate"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub oekozertifikate: Option<Vec<Oekozertifikat>>,
    /// BO4E type discriminant — always `ComTyp::Energiemix` for this struct.
    #[cfg_attr(feature = "serde", serde(rename = "_typ"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(
        feature = "builder",
        builder(default = Some(ComTyp::Energiemix), setter(skip))
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
    /// Internetseite, auf der die Strommixdaten veröffentlicht sind
    #[cfg_attr(feature = "serde", serde(rename = "website"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub website: Option<String>,
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
impl Default for Energiemix {
    fn default() -> Self {
        Self {
            anteil: Default::default(),
            atommuell: Default::default(),
            bemerkung: Default::default(),
            bezeichnung: Default::default(),
            co2_emission: Default::default(),
            energieart: Default::default(),
            energiemixnummer: Default::default(),
            gueltigkeitsjahr: Default::default(),
            id: Default::default(),
            ist_in_oeko_top_ten: Default::default(),
            oekolabel: Default::default(),
            oekozertifikate: Default::default(),
            typ: Some(ComTyp::Energiemix),
            version: Some("202607.1.0".to_owned()),
            website: Default::default(),
            zusatz_attribute: Default::default(),
            _additional: Default::default(),
        }
    }
}
#[cfg(feature = "json")]
impl crate::json::sealed::Sealed for Energiemix {}
#[cfg(feature = "json")]
impl crate::json::Bo4eJsonExt for Energiemix {}
#[cfg(feature = "json")]
impl crate::json::Bo4eExtensionData for Energiemix {
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
impl std::fmt::Display for Energiemix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => f.write_str(&json),
            Err(e) => write!(f, "<Energiemix: serialization error: {e}>"),
        }
    }
}
impl crate::Bo4eStrict for Energiemix {
    fn collect_unknown_enums(&self, path: &str, out: &mut Vec<String>) {
        if let Some(items) = &self.anteil {
            let child = crate::strict::field_path(path, "anteil");
            for (i, item) in items.iter().enumerate() {
                crate::Bo4eStrict::collect_unknown_enums(
                    item,
                    &crate::strict::index_path(&child, i),
                    out,
                );
            }
        }
        if let Some(v) = &self.energieart {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "energieart"),
                out,
            );
        }
        if let Some(items) = &self.oekolabel {
            let child = crate::strict::field_path(path, "oekolabel");
            for (i, item) in items.iter().enumerate() {
                crate::Bo4eStrict::collect_unknown_enums(
                    item,
                    &crate::strict::index_path(&child, i),
                    out,
                );
            }
        }
        if let Some(items) = &self.oekozertifikate {
            let child = crate::strict::field_path(path, "oekozertifikate");
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
