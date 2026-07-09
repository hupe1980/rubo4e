use super::{ComTyp, Messpreistyp, Preis, Tarifkalkulationsmethode, Tarifpreis, ZusatzAttribut};
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(not(feature = "json"), derive(Hash))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "validate", derive(garde::Validate))]
#[cfg_attr(feature = "validate", garde(allow_unvalidated))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// In dieser Komponente sind die Berechnungsparameter für die Ermittlung der Tarifkosten zusammengefasst.
///
/// > **Note:** [Tarifberechnungsparameter JSON Schema](https://json-schema.app/view/%23?url=https://raw.githubusercontent.com/BO4E/BO4E-Schemas/v202607.0.0/src/bo4e_schemas/com/Tarifberechnungsparameter.json)
pub struct Tarifberechnungsparameter {
    /// Gibt an, wie die Einzelpreise des Tarifes zu verarbeiten sind
    #[cfg_attr(feature = "serde", serde(rename = "berechnungsmethode"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub berechnungsmethode: Option<Tarifkalkulationsmethode>,
    /// Höchstpreis für den Durchschnitts-Arbeitspreis HT
    #[cfg_attr(feature = "serde", serde(rename = "hoechstpreisHT"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub hoechstpreis_ht: Option<Preis>,
    /// Höchstpreis für den Durchschnitts-Arbeitspreis NT
    #[cfg_attr(feature = "serde", serde(rename = "hoechstpreisNT"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub hoechstpreis_nt: Option<Preis>,
    /// Eine generische ID, die für eigene Zwecke genutzt werden kann.
    /// Z.B. könnten hier UUIDs aus einer Datenbank stehen oder URLs zu einem Backend-System.
    #[cfg_attr(feature = "serde", serde(rename = "_id"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub id: Option<String>,
    /// True, falls der Messpreis im Grundpreis (GP) enthalten ist
    #[cfg_attr(feature = "serde", serde(rename = "istMesspreisInGrundpreisEnthalten"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub ist_messpreis_in_grundpreis_enthalten: Option<bool>,
    /// True, falls bei der Bildung des Durchschnittspreises für die Höchst- und Mindestpreisbetrachtung der Messpreis mit
    /// berücksichtigt wird
    #[cfg_attr(feature = "serde", serde(rename = "istMesspreisZuBeruecksichtigen"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub ist_messpreis_zu_beruecksichtigen: Option<bool>,
    /// Im Preis bereits eingeschlossene Leistung (für Gas)
    #[cfg_attr(feature = "serde", serde(rename = "kwInklusive"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg(feature = "decimal")]
    pub kw_inklusive: Option<rust_decimal::Decimal>,
    /// Requires the `decimal` feature for the `rust_decimal::Decimal` representation.
    /// Without `decimal`, stores the decimal string value unchanged.
    #[cfg_attr(feature = "serde", serde(rename = "kwInklusive"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg(not(feature = "decimal"))]
    pub kw_inklusive: Option<String>,
    /// Intervall, indem die über "kwInklusive" hinaus abgenommene Leistung kostenpflichtig wird (z.B. je 5 kW 20 EURO)
    #[cfg_attr(feature = "serde", serde(rename = "kwWeitereMengen"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg(feature = "decimal")]
    pub kw_weitere_mengen: Option<rust_decimal::Decimal>,
    /// Requires the `decimal` feature for the `rust_decimal::Decimal` representation.
    /// Without `decimal`, stores the decimal string value unchanged.
    #[cfg_attr(feature = "serde", serde(rename = "kwWeitereMengen"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg(not(feature = "decimal"))]
    pub kw_weitere_mengen: Option<String>,
    /// Typ des Messpreises
    #[cfg_attr(feature = "serde", serde(rename = "messpreistyp"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub messpreistyp: Option<Messpreistyp>,
    /// Mindestpreis für den Durchschnitts-Arbeitspreis
    #[cfg_attr(feature = "serde", serde(rename = "mindestpreis"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub mindestpreis: Option<Preis>,
    /// COM type identifier for this struct.
    #[cfg_attr(feature = "serde", serde(rename = "_typ"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub typ: Option<ComTyp>,
    /// Version der COM-Struktur aka "fachliche Versionierung"
    #[cfg_attr(feature = "serde", serde(rename = "_version"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub version: Option<String>,
    #[cfg_attr(feature = "serde", serde(rename = "zusatzAttribute"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub zusatz_attribute: Option<Vec<ZusatzAttribut>>,
    /// Liste mit zusätzlichen Preisen, beispielsweise Messpreise und/oder Leistungspreise
    #[cfg_attr(feature = "serde", serde(rename = "zusatzpreise"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub zusatzpreise: Option<Vec<Tarifpreis>>,
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
#[cfg(feature = "json")]
impl crate::json::sealed::Sealed for Tarifberechnungsparameter {}
#[cfg(feature = "json")]
impl crate::json::Bo4eJsonExt for Tarifberechnungsparameter {}
#[cfg(feature = "json")]
impl crate::json::Bo4eExtensionData for Tarifberechnungsparameter {
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
impl std::fmt::Display for Tarifberechnungsparameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => f.write_str(&json),
            Err(e) => write!(f, "<Tarifberechnungsparameter: serialization error: {e}>"),
        }
    }
}
