use super::{
    Adresse, Anrede, Bo4eObject, BoTyp, Kontaktweg, Titel, ZusatzAttribut, Zustaendigkeit,
};
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(not(feature = "json"), derive(Hash))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "validate", derive(garde::Validate))]
#[cfg_attr(feature = "validate", garde(allow_unvalidated))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Object containing information about a Person
///
/// > **Note:** [Person JSON Schema](https://json-schema.app/view/%23?url=https://raw.githubusercontent.com/BO4E/BO4E-Schemas/v202501.0.0/src/bo4e_schemas/bo/Person.json)
pub struct Person {
    /// Adresse der Person, falls diese von der Adresse des Geschäftspartners abweicht
    #[cfg_attr(feature = "serde", serde(rename = "adresse"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub adresse: Option<Adresse>,
    /// Mögliche Anrede der Person
    #[cfg_attr(feature = "serde", serde(rename = "anrede"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub anrede: Option<Anrede>,
    /// Geburtsdatum der Person
    #[cfg_attr(feature = "serde", serde(rename = "geburtsdatum"))]
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
    pub geburtsdatum: Option<time::Date>,
    /// Requires the `time` feature for the `time::Date` representation.
    /// Without `time`, stores the ISO 8601 date string (`YYYY-MM-DD`) unchanged.
    #[cfg_attr(feature = "serde", serde(rename = "geburtsdatum"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(
        feature = "schemars",
        schemars(schema_with = "crate::schema_helpers::opt_date_schema")
    )]
    #[cfg(not(feature = "time"))]
    pub geburtsdatum: Option<String>,
    /// Eine generische ID, die für eigene Zwecke genutzt werden kann.
    /// Z.B. könnten hier UUIDs aus einer Datenbank stehen oder URLs zu einem Backend-System.
    #[cfg_attr(feature = "serde", serde(rename = "_id"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub id: Option<String>,
    /// Im Falle einer nicht standardisierten Anrede kann hier eine frei definierbare Anrede vorgegeben werden.
    /// Beispiel: "Vereinsgemeinschaft", "Pfarrer", "Hochwürdigster Herr Abt".
    #[cfg_attr(feature = "serde", serde(rename = "individuelleAnrede"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub individuelle_anrede: Option<String>,
    /// Weitere Informationen zur Person
    #[cfg_attr(feature = "serde", serde(rename = "kommentar"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub kommentar: Option<String>,
    /// Kontaktwege der Person
    #[cfg_attr(feature = "serde", serde(rename = "kontaktwege"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub kontaktwege: Option<Vec<Kontaktweg>>,
    /// Nachname (Familienname) der Person
    #[cfg_attr(feature = "serde", serde(rename = "nachname"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub nachname: Option<String>,
    /// Möglicher Titel der Person
    #[cfg_attr(feature = "serde", serde(rename = "titel"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub titel: Option<Titel>,
    /// BO type identifier — always `BoTyp::Person` for this struct.
    #[cfg_attr(feature = "serde", serde(rename = "_typ"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(
        feature = "builder",
        builder(default = Some(BoTyp::Person), setter(skip))
    )]
    pub typ: Option<BoTyp>,
    /// Version der BO-Struktur aka "fachliche Versionierung"
    #[cfg_attr(feature = "serde", serde(rename = "_version"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub version: Option<String>,
    /// Vorname der Person
    #[cfg_attr(feature = "serde", serde(rename = "vorname"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub vorname: Option<String>,
    #[cfg_attr(feature = "serde", serde(rename = "zusatzAttribute"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub zusatz_attribute: Option<Vec<ZusatzAttribut>>,
    /// Liste der Abteilungen und Zuständigkeiten der Person
    #[cfg_attr(feature = "serde", serde(rename = "zustaendigkeiten"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub zustaendigkeiten: Option<Vec<Zustaendigkeit>>,
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
impl Default for Person {
    fn default() -> Self {
        Self {
            typ: Some(BoTyp::Person),
            adresse: Default::default(),
            anrede: Default::default(),
            geburtsdatum: Default::default(),
            id: Default::default(),
            individuelle_anrede: Default::default(),
            kommentar: Default::default(),
            kontaktwege: Default::default(),
            nachname: Default::default(),
            titel: Default::default(),
            version: Default::default(),
            vorname: Default::default(),
            zusatz_attribute: Default::default(),
            zustaendigkeiten: Default::default(),
            _additional: Default::default(),
        }
    }
}
impl Bo4eObject for Person {
    type BoTyp = BoTyp;
    fn bo_type(&self) -> BoTyp {
        self.typ.unwrap_or(BoTyp::Person)
    }
    fn schema_version(&self) -> &'static str {
        "v202501.0.0"
    }
}
#[cfg(feature = "json")]
impl crate::json::sealed::Sealed for Person {}
#[cfg(feature = "json")]
impl crate::json::Bo4eJsonExt for Person {}
#[cfg(feature = "json")]
impl crate::json::Bo4eExtensionData for Person {
    fn extension_data(&self) -> &indexmap::IndexMap<String, serde_json::Value> {
        self._additional.as_map().unwrap_or(&crate::json::extension::EMPTY_EXTENSION_MAP)
    }
    fn has_extension_data(&self) -> bool {
        !self._additional.is_empty()
    }
}
#[cfg(feature = "json")]
impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => f.write_str(&json),
            Err(e) => write!(f, "<Person: serialization error: {e}>"),
        }
    }
}
