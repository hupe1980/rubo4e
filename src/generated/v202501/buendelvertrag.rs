use super::{
    Bo4eObject, BoTyp, Geschaeftspartner, Sparte, Unterschrift, Vertrag, Vertragsart,
    Vertragskonditionen, Vertragsstatus, ZusatzAttribut,
};
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(not(feature = "json"), derive(Hash))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "validate", derive(garde::Validate))]
#[cfg_attr(feature = "validate", garde(allow_unvalidated))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Abbildung eines Bündelvertrags.
/// Es handelt sich hierbei um eine Liste von Einzelverträgen, die in einem Vertragsobjekt gebündelt sind.
///
/// > **Note:** [Buendelvertrag JSON Schema](https://json-schema.app/view/%23?url=https://raw.githubusercontent.com/BO4E/BO4E-Schemas/v202501.0.0/src/bo4e_schemas/bo/Buendelvertrag.json)
pub struct Buendelvertrag {
    /// Beschreibung zum Vertrag
    #[cfg_attr(feature = "serde", serde(rename = "beschreibung"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub beschreibung: Option<String>,
    /// Die Liste mit den Einzelverträgen zu den Abnahmestellen
    #[cfg_attr(feature = "serde", serde(rename = "einzelvertraege"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub einzelvertraege: Option<Vec<Box<Vertrag>>>,
    /// Eine generische ID, die für eigene Zwecke genutzt werden kann.
    /// Z.B. könnten hier UUIDs aus einer Datenbank stehen oder URLs zu einem Backend-System.
    #[cfg_attr(feature = "serde", serde(rename = "_id"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub id: Option<String>,
    /// Unterscheidungsmöglichkeiten für die Sparte
    #[cfg_attr(feature = "serde", serde(rename = "sparte"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub sparte: Option<Sparte>,
    /// BO type identifier — always `BoTyp::Buendelvertrag` for this struct.
    #[cfg_attr(feature = "serde", serde(rename = "_typ"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(
        feature = "builder",
        builder(default = Some(BoTyp::Buendelvertrag), setter(skip))
    )]
    pub typ: Option<BoTyp>,
    /// Unterzeichner des Vertragspartners1
    #[cfg_attr(feature = "serde", serde(rename = "unterzeichnervp1"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub unterzeichnervp1: Option<Vec<Unterschrift>>,
    /// Unterzeichner des Vertragspartners2
    #[cfg_attr(feature = "serde", serde(rename = "unterzeichnervp2"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub unterzeichnervp2: Option<Vec<Unterschrift>>,
    /// Version der BO-Struktur aka "fachliche Versionierung"
    #[cfg_attr(feature = "serde", serde(rename = "_version"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub version: Option<String>,
    /// Hier ist festgelegt, um welche Art von Vertrag es sich handelt. Z.B. Netznutzungvertrag
    #[cfg_attr(feature = "serde", serde(rename = "vertragsart"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub vertragsart: Option<Vertragsart>,
    /// Gibt an, wann der Vertrag beginnt (inklusiv)
    #[cfg_attr(feature = "serde", serde(rename = "vertragsbeginn"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(
        feature = "schemars",
        schemars(schema_with = "crate::schema_helpers::opt_datetime_schema")
    )]
    #[cfg_attr(
        all(feature = "serde", feature = "time"),
        serde(with = "time::serde::rfc3339::option")
    )]
    #[cfg(feature = "time")]
    pub vertragsbeginn: Option<time::OffsetDateTime>,
    /// Requires the `time` feature for the `time::OffsetDateTime` representation.
    /// Without `time`, stores the ISO-8601 string value unchanged.
    #[cfg_attr(feature = "serde", serde(rename = "vertragsbeginn"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(
        feature = "schemars",
        schemars(schema_with = "crate::schema_helpers::opt_datetime_schema")
    )]
    #[cfg(not(feature = "time"))]
    pub vertragsbeginn: Option<String>,
    /// Gibt an, wann der Vertrag (voraussichtlich) endet oder beendet wurde (exklusiv)
    #[cfg_attr(feature = "serde", serde(rename = "vertragsende"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(
        feature = "schemars",
        schemars(schema_with = "crate::schema_helpers::opt_datetime_schema")
    )]
    #[cfg_attr(
        all(feature = "serde", feature = "time"),
        serde(with = "time::serde::rfc3339::option")
    )]
    #[cfg(feature = "time")]
    pub vertragsende: Option<time::OffsetDateTime>,
    /// Requires the `time` feature for the `time::OffsetDateTime` representation.
    /// Without `time`, stores the ISO-8601 string value unchanged.
    #[cfg_attr(feature = "serde", serde(rename = "vertragsende"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(
        feature = "schemars",
        schemars(schema_with = "crate::schema_helpers::opt_datetime_schema")
    )]
    #[cfg(not(feature = "time"))]
    pub vertragsende: Option<String>,
    /// Festlegungen zu Laufzeiten und Kündigungsfristen
    #[cfg_attr(feature = "serde", serde(rename = "vertragskonditionen"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub vertragskonditionen: Option<Vec<Vertragskonditionen>>,
    /// Eine im Verwendungskontext eindeutige Nummer für den Vertrag
    #[cfg_attr(feature = "serde", serde(rename = "vertragsnummer"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub vertragsnummer: Option<String>,
    /// Der "erstgenannte" Vertragspartner. In der Regel der Aussteller des Vertrags.
    /// Beispiel: "Vertrag zwischen Vertagspartner 1 ..."
    #[cfg_attr(feature = "serde", serde(rename = "vertragspartner1"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub vertragspartner1: Option<Box<Geschaeftspartner>>,
    /// Der "zweitgenannte" Vertragspartner. In der Regel der Empfänger des Vertrags.
    /// Beispiel "Vertrag zwischen Vertagspartner 1 und Vertragspartner 2"
    #[cfg_attr(feature = "serde", serde(rename = "vertragspartner2"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub vertragspartner2: Option<Box<Geschaeftspartner>>,
    /// Gibt den Status des Vertrages an
    #[cfg_attr(feature = "serde", serde(rename = "vertragsstatus"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub vertragsstatus: Option<Vertragsstatus>,
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
impl Default for Buendelvertrag {
    fn default() -> Self {
        Self {
            typ: Some(BoTyp::Buendelvertrag),
            beschreibung: Default::default(),
            einzelvertraege: Default::default(),
            id: Default::default(),
            sparte: Default::default(),
            unterzeichnervp1: Default::default(),
            unterzeichnervp2: Default::default(),
            version: Default::default(),
            vertragsart: Default::default(),
            vertragsbeginn: Default::default(),
            vertragsende: Default::default(),
            vertragskonditionen: Default::default(),
            vertragsnummer: Default::default(),
            vertragspartner1: Default::default(),
            vertragspartner2: Default::default(),
            vertragsstatus: Default::default(),
            zusatz_attribute: Default::default(),
            _additional: Default::default(),
        }
    }
}
impl Bo4eObject for Buendelvertrag {
    type BoTyp = BoTyp;
    fn bo_type(&self) -> BoTyp {
        self.typ.unwrap_or(BoTyp::Buendelvertrag)
    }
    fn schema_version(&self) -> &'static str {
        "v202501.0.0"
    }
}
#[cfg(feature = "json")]
impl crate::json::sealed::Sealed for Buendelvertrag {}
#[cfg(feature = "json")]
impl crate::json::Bo4eJsonExt for Buendelvertrag {}
#[cfg(feature = "json")]
impl crate::json::Bo4eExtensionData for Buendelvertrag {
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
impl std::fmt::Display for Buendelvertrag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => f.write_str(&json),
            Err(e) => write!(f, "<Buendelvertrag: serialization error: {e}>"),
        }
    }
}
