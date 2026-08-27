use super::{Bo4eComponent, Bo4eTyped, ComTyp, Steuerart, Waehrungscode, ZusatzAttribut};
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(not(feature = "json"), derive(Eq, Hash))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "validate", derive(garde::Validate))]
#[cfg_attr(feature = "validate", garde(allow_unvalidated))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Abbildung eines Steuerbetrages.
///
/// > **Note:** [Steuerbetrag JSON Schema](https://json-schema.app/view/%23?url=https://raw.githubusercontent.com/BO4E/BO4E-Schemas/v202607.1.0/src/bo4e_schemas/com/Steuerbetrag.json)
pub struct Steuerbetrag {
    /// Nettobetrag für den die Steuer berechnet wurde. Z.B. 100
    #[cfg_attr(feature = "serde", serde(rename = "basiswert"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "crate::decimal_serde::deserialize_opt")
    )]
    #[cfg(feature = "decimal")]
    pub basiswert: Option<rust_decimal::Decimal>,
    /// Requires the `decimal` feature for the `rust_decimal::Decimal` representation.
    /// Without `decimal`, stores the decimal's lexical form (a JSON string or number).
    #[cfg_attr(feature = "serde", serde(rename = "basiswert"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "crate::decimal_serde::deserialize_opt")
    )]
    #[cfg(not(feature = "decimal"))]
    pub basiswert: Option<String>,
    /// Eine generische ID, die für eigene Zwecke genutzt werden kann.
    /// Z.B. könnten hier UUIDs aus einer Datenbank stehen oder URLs zu einem Backend-System.
    #[cfg_attr(feature = "serde", serde(rename = "_id"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub id: Option<String>,
    /// Kennzeichnung der Steuerart, bzw. Verfahrens.
    #[cfg_attr(feature = "serde", serde(rename = "steuerart"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub steuerart: Option<Steuerart>,
    /// Angabe des Steuersatzes in %
    #[cfg_attr(feature = "serde", serde(rename = "steuersatz"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "crate::decimal_serde::deserialize_opt")
    )]
    #[cfg(feature = "decimal")]
    pub steuersatz: Option<rust_decimal::Decimal>,
    /// Requires the `decimal` feature for the `rust_decimal::Decimal` representation.
    /// Without `decimal`, stores the decimal's lexical form (a JSON string or number).
    #[cfg_attr(feature = "serde", serde(rename = "steuersatz"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "crate::decimal_serde::deserialize_opt")
    )]
    #[cfg(not(feature = "decimal"))]
    pub steuersatz: Option<String>,
    /// Aus dem Basiswert berechnete Steuer. Z.B. 19 (bei UST_19)
    #[cfg_attr(feature = "serde", serde(rename = "steuerwert"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "crate::decimal_serde::deserialize_opt")
    )]
    #[cfg(feature = "decimal")]
    pub steuerwert: Option<rust_decimal::Decimal>,
    /// Requires the `decimal` feature for the `rust_decimal::Decimal` representation.
    /// Without `decimal`, stores the decimal's lexical form (a JSON string or number).
    #[cfg_attr(feature = "serde", serde(rename = "steuerwert"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "crate::decimal_serde::deserialize_opt")
    )]
    #[cfg(not(feature = "decimal"))]
    pub steuerwert: Option<String>,
    /// BO4E type discriminant — always `ComTyp::Steuerbetrag` for this struct.
    #[cfg_attr(feature = "serde", serde(rename = "_typ"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(
        feature = "builder",
        builder(default = Some(ComTyp::Steuerbetrag), setter(skip))
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
    /// Währung. Z.B. Euro.
    #[cfg_attr(feature = "serde", serde(rename = "waehrungscode"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub waehrungscode: Option<Waehrungscode>,
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
impl Default for Steuerbetrag {
    fn default() -> Self {
        Self {
            basiswert: Default::default(),
            id: Default::default(),
            steuerart: Default::default(),
            steuersatz: Default::default(),
            steuerwert: Default::default(),
            typ: Some(ComTyp::Steuerbetrag),
            version: Some("202607.1.0".to_owned()),
            waehrungscode: Default::default(),
            zusatz_attribute: Default::default(),
            _additional: Default::default(),
        }
    }
}
impl Bo4eTyped for Steuerbetrag {
    type Typ = ComTyp;
    const TYP: ComTyp = ComTyp::Steuerbetrag;
    const TYP_WIRE: &'static str = "STEUERBETRAG";
    const SCHEMA_VERSION: &'static str = "202607.1.0";
    const SCHEMA_SERIES: &'static str = "202607";
}
impl crate::bo4e_typed_sealed::Sealed for Steuerbetrag {}
impl Bo4eComponent for Steuerbetrag {}
impl crate::bo4e_component_sealed::Sealed for Steuerbetrag {}
#[cfg(feature = "json")]
impl crate::json::sealed::Sealed for Steuerbetrag {}
#[cfg(feature = "json")]
impl crate::json::Bo4eJsonExt for Steuerbetrag {}
#[cfg(feature = "json")]
impl crate::json::Bo4eExtensionData for Steuerbetrag {
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
impl std::fmt::Display for Steuerbetrag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => f.write_str(&json),
            Err(e) => write!(f, "<Steuerbetrag: serialization error: {e}>"),
        }
    }
}
impl crate::Bo4eStrict for Steuerbetrag {
    fn collect_unknown_enums(&self, path: &str, out: &mut Vec<String>) {
        if let Some(v) = &self.steuerart {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "steuerart"),
                out,
            );
        }
        if let Some(v) = &self.waehrungscode {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "waehrungscode"),
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
