use super::{
    Ausschreibungslos, Ausschreibungsportal, Ausschreibungsstatus, Ausschreibungstyp, Bo4eObject,
    BoTyp, Geschaeftspartner, Zeitraum, ZusatzAttribut,
};
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(not(feature = "json"), derive(Hash))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "validate", derive(garde::Validate))]
#[cfg_attr(feature = "validate", garde(allow_unvalidated))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Das BO Ausschreibung dient zur detaillierten Darstellung von ausgeschriebenen Energiemengen in der Energiewirtschaft
///
/// > **Note:** [Ausschreibung JSON Schema](https://json-schema.app/view/%23?url=https://raw.githubusercontent.com/BO4E/BO4E-Schemas/v202607.0.0/src/bo4e_schemas/bo/Ausschreibung.json)
pub struct Ausschreibung {
    /// Diese Komponente wird zur Abbildung von Zeiträumen in Form von Dauern oder der Angabe von Start und Ende verwendet.
    /// Es muss daher entweder eine Dauer oder ein Zeitraum in Form von Start und Ende angegeben sein
    #[cfg_attr(feature = "serde", serde(rename = "abgabefrist"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub abgabefrist: Option<Zeitraum>,
    /// Mit diesem Objekt können Geschäftspartner übertragen werden.
    /// Sowohl Unternehmen, als auch Privatpersonen können Geschäftspartner sein
    #[cfg_attr(feature = "serde", serde(rename = "ausschreibender"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub ausschreibender: Option<Box<Geschaeftspartner>>,
    /// Aufzählung der unterstützten Ausschreibungsportale
    #[cfg_attr(feature = "serde", serde(rename = "ausschreibungportal"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub ausschreibungportal: Option<Ausschreibungsportal>,
    /// Vom Herausgeber der Ausschreibung vergebene eindeutige Nummer
    #[cfg_attr(feature = "serde", serde(rename = "ausschreibungsnummer"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub ausschreibungsnummer: Option<String>,
    /// Bezeichnungen für die Ausschreibungsphasen
    #[cfg_attr(feature = "serde", serde(rename = "ausschreibungsstatus"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub ausschreibungsstatus: Option<Ausschreibungsstatus>,
    /// Aufzählung für die Typisierung von Ausschreibungen
    #[cfg_attr(feature = "serde", serde(rename = "ausschreibungstyp"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub ausschreibungstyp: Option<Ausschreibungstyp>,
    /// Diese Komponente wird zur Abbildung von Zeiträumen in Form von Dauern oder der Angabe von Start und Ende verwendet.
    /// Es muss daher entweder eine Dauer oder ein Zeitraum in Form von Start und Ende angegeben sein
    #[cfg_attr(feature = "serde", serde(rename = "bindefrist"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub bindefrist: Option<Zeitraum>,
    /// Eine generische ID, die für eigene Zwecke genutzt werden kann.
    /// Z.B. könnten hier UUIDs aus einer Datenbank stehen oder URLs zu einem Backend-System.
    #[cfg_attr(feature = "serde", serde(rename = "_id"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub id: Option<String>,
    /// Kennzeichen, ob die Ausschreibung kostenpflichtig ist
    #[cfg_attr(feature = "serde", serde(rename = "istKostenpflichtig"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub ist_kostenpflichtig: Option<bool>,
    /// Die einzelnen Lose, aus denen sich die Ausschreibung zusammensetzt
    #[cfg_attr(feature = "serde", serde(rename = "lose"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub lose: Option<Vec<Ausschreibungslos>>,
    /// BO type identifier — always `BoTyp::Ausschreibung` for this struct.
    #[cfg_attr(feature = "serde", serde(rename = "_typ"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(
        feature = "builder",
        builder(default = Some(BoTyp::Ausschreibung), setter(skip))
    )]
    pub typ: Option<BoTyp>,
    /// Gibt den Veröffentlichungszeitpunkt der Ausschreibung an
    #[cfg_attr(feature = "serde", serde(rename = "veroeffentlichungszeitpunkt"))]
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
    pub veroeffentlichungszeitpunkt: Option<time::OffsetDateTime>,
    /// Requires the `time` feature for the `time::OffsetDateTime` representation.
    /// Without `time`, stores the ISO-8601 string value unchanged.
    #[cfg_attr(feature = "serde", serde(rename = "veroeffentlichungszeitpunkt"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(
        feature = "schemars",
        schemars(schema_with = "crate::schema_helpers::opt_datetime_schema")
    )]
    #[cfg(not(feature = "time"))]
    pub veroeffentlichungszeitpunkt: Option<String>,
    /// Version der BO-Struktur aka "fachliche Versionierung"
    #[cfg_attr(feature = "serde", serde(rename = "_version"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub version: Option<String>,
    /// Internetseite, auf der die Ausschreibung veröffentlicht wurde (falls vorhanden)
    #[cfg_attr(feature = "serde", serde(rename = "webseite"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub webseite: Option<String>,
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
impl Default for Ausschreibung {
    fn default() -> Self {
        Self {
            typ: Some(BoTyp::Ausschreibung),
            abgabefrist: Default::default(),
            ausschreibender: Default::default(),
            ausschreibungportal: Default::default(),
            ausschreibungsnummer: Default::default(),
            ausschreibungsstatus: Default::default(),
            ausschreibungstyp: Default::default(),
            bindefrist: Default::default(),
            id: Default::default(),
            ist_kostenpflichtig: Default::default(),
            lose: Default::default(),
            veroeffentlichungszeitpunkt: Default::default(),
            version: Default::default(),
            webseite: Default::default(),
            zusatz_attribute: Default::default(),
            _additional: Default::default(),
        }
    }
}
impl Bo4eObject for Ausschreibung {
    type BoTyp = BoTyp;
    fn bo_type(&self) -> BoTyp {
        self.typ.unwrap_or(BoTyp::Ausschreibung)
    }
    fn schema_version(&self) -> &'static str {
        "v202607.0.0"
    }
}
#[cfg(feature = "json")]
impl crate::json::sealed::Sealed for Ausschreibung {}
#[cfg(feature = "json")]
impl crate::json::Bo4eJsonExt for Ausschreibung {}
#[cfg(feature = "json")]
impl crate::json::Bo4eExtensionData for Ausschreibung {
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
impl std::fmt::Display for Ausschreibung {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => f.write_str(&json),
            Err(e) => write!(f, "<Ausschreibung: serialization error: {e}>"),
        }
    }
}
