use super::{
    Adresse, Anrede, Bo4eObject, BoTyp, Geschaeftspartnerrolle, Kontaktweg,
    Organisationstyp, Person, Titel, ZusatzAttribut,
};
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(not(feature = "json"), derive(Hash))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "validate", derive(garde::Validate))]
#[cfg_attr(feature = "validate", garde(allow_unvalidated))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Mit diesem Objekt können Geschäftspartner übertragen werden.
/// Sowohl Unternehmen, als auch Privatpersonen können Geschäftspartner sein.
/// Hinweis: "Marktteilnehmer" haben ein eigenes BO, welches sich von diesem BO ableitet.
/// Hier sollte daher keine Zuordnung zu Marktrollen erfolgen.
///
/// > **Note:** [Geschaeftspartner JSON Schema](https://json-schema.app/view/%23?url=https://raw.githubusercontent.com/BO4E/BO4E-Schemas/v202501.0.0/src/bo4e_schemas/bo/Geschaeftspartner.json)
pub struct Geschaeftspartner {
    /// Adresse des Geschäftspartners
    #[cfg_attr(feature = "serde", serde(rename = "adresse"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub adresse: Option<Adresse>,
    /// Amtsgericht bzw Handelsregistergericht, das die Handelsregisternummer herausgegeben hat
    #[cfg_attr(feature = "serde", serde(rename = "amtsgericht"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub amtsgericht: Option<String>,
    /// Mögliche Anrede der Person
    #[cfg_attr(feature = "serde", serde(rename = "anrede"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub anrede: Option<Anrede>,
    #[cfg_attr(feature = "serde", serde(rename = "ansprechpartner"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub ansprechpartner: Option<Vec<Box<Person>>>,
    /// Rollen, die die Geschäftspartner inne haben (z.B. Interessent, Kunde)
    #[cfg_attr(feature = "serde", serde(rename = "geschaeftspartnerrollen"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub geschaeftspartnerrollen: Option<Vec<Geschaeftspartnerrolle>>,
    /// Die Gläubiger-ID welche im Zahlungsverkehr verwendet wird; Z.B. "DE 47116789"
    #[cfg_attr(feature = "serde", serde(rename = "glaeubigerId"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub glaeubiger_id: Option<String>,
    /// Handelsregisternummer des Geschäftspartners
    #[cfg_attr(feature = "serde", serde(rename = "handelsregisternummer"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub handelsregisternummer: Option<String>,
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
    /// Kontaktwege des Geschäftspartners
    #[cfg_attr(feature = "serde", serde(rename = "kontaktwege"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub kontaktwege: Option<Vec<Kontaktweg>>,
    /// Nachname (Familienname) der Person
    #[cfg_attr(feature = "serde", serde(rename = "nachname"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub nachname: Option<String>,
    /// Name der Firma, wenn Gewerbe oder andere Organisation.
    #[cfg_attr(feature = "serde", serde(rename = "organisationsname"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub organisationsname: Option<String>,
    /// Kennzeichnung ob es sich um ein Gewerbe/Unternehmen, eine Privatperson oder eine andere Art von Organisation handelt.
    #[cfg_attr(feature = "serde", serde(rename = "organisationstyp"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub organisationstyp: Option<Organisationstyp>,
    /// Möglicher Titel der Person
    #[cfg_attr(feature = "serde", serde(rename = "titel"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub titel: Option<Titel>,
    /// BO type identifier — always `BoTyp::Geschaeftspartner` for this struct.
    #[cfg_attr(feature = "serde", serde(rename = "_typ"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(
        feature = "builder",
        builder(default = Some(BoTyp::Geschaeftspartner), setter(skip))
    )]
    pub typ: Option<BoTyp>,
    /// Die Steuer-ID des Geschäftspartners; Beispiel: "DE 813281825"
    #[cfg_attr(feature = "serde", serde(rename = "umsatzsteuerId"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub umsatzsteuer_id: Option<String>,
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
    /// Internetseite des Marktpartners
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
impl Default for Geschaeftspartner {
    fn default() -> Self {
        Self {
            typ: Some(BoTyp::Geschaeftspartner),
            adresse: Default::default(),
            amtsgericht: Default::default(),
            anrede: Default::default(),
            ansprechpartner: Default::default(),
            geschaeftspartnerrollen: Default::default(),
            glaeubiger_id: Default::default(),
            handelsregisternummer: Default::default(),
            id: Default::default(),
            individuelle_anrede: Default::default(),
            kontaktwege: Default::default(),
            nachname: Default::default(),
            organisationsname: Default::default(),
            organisationstyp: Default::default(),
            titel: Default::default(),
            umsatzsteuer_id: Default::default(),
            version: Default::default(),
            vorname: Default::default(),
            website: Default::default(),
            zusatz_attribute: Default::default(),
            _additional: Default::default(),
        }
    }
}
impl Bo4eObject for Geschaeftspartner {
    type BoTyp = BoTyp;
    fn bo_type(&self) -> BoTyp {
        self.typ.unwrap_or(BoTyp::Geschaeftspartner)
    }
    fn schema_version(&self) -> &'static str {
        "v202501.0.0"
    }
}
#[cfg(feature = "json")]
impl crate::json::sealed::Sealed for Geschaeftspartner {}
#[cfg(feature = "json")]
impl crate::json::Bo4eJsonExt for Geschaeftspartner {}
#[cfg(feature = "json")]
impl crate::json::Bo4eExtensionData for Geschaeftspartner {
    fn extension_data(&self) -> &indexmap::IndexMap<String, serde_json::Value> {
        self._additional.as_map().unwrap_or(&crate::json::extension::EMPTY_EXTENSION_MAP)
    }
    fn has_extension_data(&self) -> bool {
        !self._additional.is_empty()
    }
}
#[cfg(feature = "json")]
impl std::fmt::Display for Geschaeftspartner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => f.write_str(&json),
            Err(e) => write!(f, "<Geschaeftspartner: serialization error: {e}>"),
        }
    }
}
