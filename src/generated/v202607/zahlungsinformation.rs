use super::{Bo4eComponent, Bo4eTyped, ComTyp, Zahlungsart, ZusatzAttribut};
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(not(feature = "json"), derive(Eq, Hash))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "validate", derive(garde::Validate))]
#[cfg_attr(feature = "validate", garde(allow_unvalidated))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Mit dieser Komponente kann eine einzelne Zahlungsinformation dargestellt werden.
///
/// > **Note:** [Zahlungsinformation JSON Schema](https://json-schema.app/view/%23?url=https://raw.githubusercontent.com/BO4E/BO4E-Schemas/v202607.1.0/src/bo4e_schemas/com/Zahlungsinformation.json)
pub struct Zahlungsinformation {
    /// Eine konstante Betreffzeile für Überweisungen
    #[cfg_attr(feature = "serde", serde(rename = "betreff"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub betreff: Option<String>,
    /// Eine BIC-Nummer
    #[cfg_attr(feature = "serde", serde(rename = "bic"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub bic: Option<String>,
    /// Eine IBAN-Nummer
    #[cfg_attr(feature = "serde", serde(rename = "iban"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub iban: Option<String>,
    /// Eine generische ID, die für eigene Zwecke genutzt werden kann.
    /// Z.B. könnten hier UUIDs aus einer Datenbank stehen oder URLs zu einem Backend-System.
    #[cfg_attr(feature = "serde", serde(rename = "_id"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub id: Option<String>,
    /// Der Name des Kontoinhabers
    #[cfg_attr(feature = "serde", serde(rename = "kontoinhaber"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub kontoinhaber: Option<String>,
    /// Eine SEPA-Referenz
    #[cfg_attr(feature = "serde", serde(rename = "sepaReferenz"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub sepa_referenz: Option<String>,
    /// BO4E type discriminant — always `ComTyp::Zahlungsinformation` for this struct.
    #[cfg_attr(feature = "serde", serde(rename = "_typ"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(
        feature = "builder",
        builder(default = Some(ComTyp::Zahlungsinformation), setter(skip))
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
    /// Die Zahlungsart dieser Zahlungsinformation
    #[cfg_attr(feature = "serde", serde(rename = "zahlungsart"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub zahlungsart: Option<Zahlungsart>,
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
impl Default for Zahlungsinformation {
    fn default() -> Self {
        Self {
            betreff: Default::default(),
            bic: Default::default(),
            iban: Default::default(),
            id: Default::default(),
            kontoinhaber: Default::default(),
            sepa_referenz: Default::default(),
            typ: Some(ComTyp::Zahlungsinformation),
            version: Some("202607.1.0".to_owned()),
            zahlungsart: Default::default(),
            zusatz_attribute: Default::default(),
            _additional: Default::default(),
        }
    }
}
impl Bo4eTyped for Zahlungsinformation {
    type Typ = ComTyp;
    const TYP: ComTyp = ComTyp::Zahlungsinformation;
    const TYP_WIRE: &'static str = "ZAHLUNGSINFORMATION";
    const SCHEMA_VERSION: &'static str = "202607.1.0";
    const SCHEMA_SERIES: &'static str = "202607";
}
impl crate::bo4e_typed_sealed::Sealed for Zahlungsinformation {}
impl Bo4eComponent for Zahlungsinformation {}
impl crate::bo4e_component_sealed::Sealed for Zahlungsinformation {}
#[cfg(feature = "json")]
impl crate::json::sealed::Sealed for Zahlungsinformation {}
#[cfg(feature = "json")]
impl crate::json::Bo4eJsonExt for Zahlungsinformation {}
#[cfg(feature = "json")]
impl crate::json::Bo4eExtensionData for Zahlungsinformation {
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
impl std::fmt::Display for Zahlungsinformation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => f.write_str(&json),
            Err(e) => write!(f, "<Zahlungsinformation: serialization error: {e}>"),
        }
    }
}
impl crate::Bo4eStrict for Zahlungsinformation {
    fn collect_unknown_enums(&self, path: &str, out: &mut Vec<String>) {
        if let Some(v) = &self.zahlungsart {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "zahlungsart"),
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
