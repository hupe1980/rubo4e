use super::{ComTyp, ZusatzAttribut};
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(not(feature = "json"), derive(Hash))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "validate", derive(garde::Validate))]
#[cfg_attr(feature = "validate", garde(allow_unvalidated))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(
    all(feature = "validate", feature = "versioned"),
    garde(custom(crate::validation::v202501::validate_zeitraum))
)]
/// Diese Komponente wird zur Abbildung von Zeiträumen in Form von Dauern oder der Angabe von Start und Ende verwendet.
/// Es muss daher eine der drei Möglichkeiten angegeben sein:
/// - Einheit und Dauer oder
/// - Zeitraum: Startdatum bis Enddatum oder
/// - Zeitraum: Startzeitpunkt (Datum und Uhrzeit) bis Endzeitpunkt (Datum und Uhrzeit)
///
/// > **Note:** [Zeitraum JSON Schema](https://json-schema.app/view/%23?url=https://raw.githubusercontent.com/BO4E/BO4E-Schemas/v202501.0.0/src/bo4e_schemas/com/Zeitraum.json)
pub struct Zeitraum {
    /// Dauer in ISO 8601 Format.
    ///
    /// Example:
    /// 'P1DT30H4S'
    ///
    /// See [RFC 3339](https://datatracker.ietf.org/doc/html/rfc3339)
    #[cfg_attr(feature = "serde", serde(rename = "dauer"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub dauer: Option<String>,
    /// Enddatum des betrachteten Zeitraums ist **inklusiv**.
    ///
    /// Example:
    /// '2025-01-01'
    #[cfg_attr(feature = "serde", serde(rename = "enddatum"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(
        feature = "schemars",
        schemars(schema_with = "crate::schema_helpers::opt_date_schema")
    )]
    #[cfg_attr(
        all(feature = "serde", feature = "time"),
        serde(with = "crate::time_serde::opt_date_serde")
    )]
    #[cfg(feature = "time")]
    pub enddatum: Option<time::Date>,
    /// Requires the `time` feature for the `time::Date` representation.
    /// Without `time`, stores the ISO 8601 date string (`YYYY-MM-DD`) unchanged.
    #[cfg_attr(feature = "serde", serde(rename = "enddatum"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(
        feature = "schemars",
        schemars(schema_with = "crate::schema_helpers::opt_date_schema")
    )]
    #[cfg(not(feature = "time"))]
    pub enddatum: Option<String>,
    /// Enduhrzeit mit Zeitzone. Die angegebene Uhrzeit ist im betrachteten Zeitraum **exklusiv**.
    ///
    /// Example:
    /// '19:00:00+01:00'
    #[cfg_attr(feature = "serde", serde(rename = "enduhrzeit"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub enduhrzeit: Option<String>,
    /// Eine generische ID, die für eigene Zwecke genutzt werden kann.
    /// Z.B. könnten hier UUIDs aus einer Datenbank stehen oder URLs zu einem Backend-System.
    #[cfg_attr(feature = "serde", serde(rename = "_id"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub id: Option<String>,
    /// Startdatum des betrachteten Zeitraums ist **inklusiv**.
    ///
    /// Example:
    /// '2025-01-01'
    #[cfg_attr(feature = "serde", serde(rename = "startdatum"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(
        feature = "schemars",
        schemars(schema_with = "crate::schema_helpers::opt_date_schema")
    )]
    #[cfg_attr(
        all(feature = "serde", feature = "time"),
        serde(with = "crate::time_serde::opt_date_serde")
    )]
    #[cfg(feature = "time")]
    pub startdatum: Option<time::Date>,
    /// Requires the `time` feature for the `time::Date` representation.
    /// Without `time`, stores the ISO 8601 date string (`YYYY-MM-DD`) unchanged.
    #[cfg_attr(feature = "serde", serde(rename = "startdatum"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(
        feature = "schemars",
        schemars(schema_with = "crate::schema_helpers::opt_date_schema")
    )]
    #[cfg(not(feature = "time"))]
    pub startdatum: Option<String>,
    /// Startuhrzeit mit Zeitzone. Die angegebene Uhrzeit ist im betrachteten Zeitraum **inklusiv**.
    ///
    /// Example:
    /// '18:00:00+01:00'
    #[cfg_attr(feature = "serde", serde(rename = "startuhrzeit"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub startuhrzeit: Option<String>,
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
impl crate::json::sealed::Sealed for Zeitraum {}
#[cfg(feature = "json")]
impl crate::json::Bo4eJsonExt for Zeitraum {}
#[cfg(feature = "json")]
impl crate::json::Bo4eExtensionData for Zeitraum {
    fn extension_data(&self) -> &indexmap::IndexMap<String, serde_json::Value> {
        self._additional.as_map().unwrap_or(&crate::json::extension::EMPTY_EXTENSION_MAP)
    }
    fn has_extension_data(&self) -> bool {
        !self._additional.is_empty()
    }
}
#[cfg(feature = "json")]
impl std::fmt::Display for Zeitraum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => f.write_str(&json),
            Err(e) => write!(f, "<Zeitraum: serialization error: {e}>"),
        }
    }
}
