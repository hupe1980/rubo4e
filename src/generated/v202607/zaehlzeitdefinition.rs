use super::{Bo4eObject, BoTyp, Marktteilnehmer, Zaehlzeitsaison, ZusatzAttribut};
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(not(feature = "json"), derive(Hash))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "validate", derive(garde::Validate))]
#[cfg_attr(feature = "validate", garde(allow_unvalidated))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Beschreibt, zu welchen Zeiten welche Register aktiv sind.
///
/// Die `Zaehlzeitdefinition` ist unabhängig von konkreter Messhardware: sie weiß nichts über
/// `Zaehlwerk`e oder `Zaehler` an einer Messlokation und definiert lediglich abstrakt, welche
/// Register-Codes (z.B. "HT", "NT", aber prinzipiell beliebige freie Zeichenketten) zu welchen
/// Tageszeiten gelten.
///
/// Die zeitliche Modellierung ist dreistufig geschachtelt:
///
/// 1. ``saisons`` – ein Jahr kann in unterschiedliche Saisons aufgeteilt sein (z.B. Sommer/Winter).
///    Welche Tage zu welcher Saison gehören, gibt das `saisonprofil` an.
/// 2. Pro Saison: ``tagtypen`` – verschiedene Tagtypen (z.B. Werktag, Wochenende, Feiertag)
///    können unterschiedliche Schaltschemata haben. Welche Tage als Feiertag gelten, gibt der
///    `feiertagskalender` an.
/// 3. Pro Tagtyp: ``umschaltzeiten`` – die eigentlichen Uhrzeiten, zu denen auf welches Register
///    umgeschaltet wird.
///
/// > **Note:** [Zaehlzeitdefinition JSON Schema](https://json-schema.app/view/%23?url=https://raw.githubusercontent.com/BO4E/BO4E-Schemas/v202607.0.0/src/bo4e_schemas/bo/Zaehlzeitdefinition.json)
pub struct Zaehlzeitdefinition {
    /// Bezeichnung des Feiertagskalenders, nach dem `FEIERTAGS`-Tagtypen aufgelöst werden (z.B. "BDEW",
    /// landes- oder gemeindespezifische Kalender). Frei wählbare Zeichenkette, deren Bedeutung zwischen
    /// den Marktpartnern abgestimmt sein muss.
    #[cfg_attr(feature = "serde", serde(rename = "feiertagskalender"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub feiertagskalender: Option<String>,
    /// Eine generische ID, die für eigene Zwecke genutzt werden kann.
    /// Z.B. könnten hier UUIDs aus einer Datenbank stehen oder URLs zu einem Backend-System.
    #[cfg_attr(feature = "serde", serde(rename = "_id"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub id: Option<String>,
    /// Bezeichnung des Saisonprofils, das die Datumsgrenzen der in `saisons` referenzierten Saisons
    /// festlegt (z.B. "Sommer/Winter").
    #[cfg_attr(feature = "serde", serde(rename = "saisonprofil"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub saisonprofil: Option<String>,
    /// Die Schaltschemata, gruppiert nach Saison.
    #[cfg_attr(feature = "serde", serde(rename = "saisons"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub saisons: Option<Vec<Zaehlzeitsaison>>,
    /// BO type identifier — always `BoTyp::Zaehlzeitdefinition` for this struct.
    #[cfg_attr(feature = "serde", serde(rename = "_typ"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(
        feature = "builder",
        builder(default = Some(BoTyp::Zaehlzeitdefinition), setter(skip))
    )]
    pub typ: Option<BoTyp>,
    /// Der Marktteilnehmer, der diese Zählzeitdefinition herausgegeben hat (typischerweise Netzbetreiber
    /// oder Lieferant). Identifiziert zusammen mit einer fachlichen Gültigkeit, welche Definition zu welchem
    /// Zeitpunkt gilt.
    #[cfg_attr(feature = "serde", serde(rename = "urheber"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub urheber: Option<Box<Marktteilnehmer>>,
    /// Version der BO-Struktur aka "fachliche Versionierung"
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
impl Default for Zaehlzeitdefinition {
    fn default() -> Self {
        Self {
            typ: Some(BoTyp::Zaehlzeitdefinition),
            feiertagskalender: Default::default(),
            id: Default::default(),
            saisonprofil: Default::default(),
            saisons: Default::default(),
            urheber: Default::default(),
            version: Default::default(),
            zusatz_attribute: Default::default(),
            _additional: Default::default(),
        }
    }
}
impl Bo4eObject for Zaehlzeitdefinition {
    type BoTyp = BoTyp;
    fn bo_type(&self) -> BoTyp {
        self.typ.unwrap_or(BoTyp::Zaehlzeitdefinition)
    }
    fn schema_version(&self) -> &'static str {
        "v202607.0.0"
    }
}
#[cfg(feature = "json")]
impl crate::json::sealed::Sealed for Zaehlzeitdefinition {}
#[cfg(feature = "json")]
impl crate::json::Bo4eJsonExt for Zaehlzeitdefinition {}
#[cfg(feature = "json")]
impl crate::json::Bo4eExtensionData for Zaehlzeitdefinition {
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
impl std::fmt::Display for Zaehlzeitdefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => f.write_str(&json),
            Err(e) => write!(f, "<Zaehlzeitdefinition: serialization error: {e}>"),
        }
    }
}
