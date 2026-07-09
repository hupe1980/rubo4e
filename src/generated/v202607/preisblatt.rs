use super::{
    Bo4eObject, BoTyp, Marktteilnehmer, Preisposition, Preisstatus, Sparte, Zeitraum,
    ZusatzAttribut,
};
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(not(feature = "json"), derive(Hash))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "validate", derive(garde::Validate))]
#[cfg_attr(feature = "validate", garde(allow_unvalidated))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Das allgemeine Modell zur Abbildung von Preisen;
/// Davon abgeleitet können, über die Zuordnung identifizierender Merkmale, spezielle Preisblatt-Varianten modelliert
/// werden.
///
/// Die jeweiligen Sätze von Merkmalen sind in der Grafik ergänzt worden und stellen jeweils eine Ausprägung für die
/// verschiedenen Anwendungsfälle der Preisblätter dar.
///
/// > **Note:** [Preisblatt JSON Schema](https://json-schema.app/view/%23?url=https://raw.githubusercontent.com/BO4E/BO4E-Schemas/v202607.0.0/src/bo4e_schemas/bo/Preisblatt.json)
pub struct Preisblatt {
    /// Eine Bezeichnung für das Preisblatt
    #[cfg_attr(feature = "serde", serde(rename = "bezeichnung"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub bezeichnung: Option<String>,
    /// Der Zeitraum für den der Preis festgelegt ist
    #[cfg_attr(feature = "serde", serde(rename = "gueltigkeit"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub gueltigkeit: Option<Zeitraum>,
    /// Der Netzbetreiber, der die Preise veröffentlicht hat
    #[cfg_attr(feature = "serde", serde(rename = "herausgeber"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub herausgeber: Option<Box<Marktteilnehmer>>,
    /// Eine generische ID, die für eigene Zwecke genutzt werden kann.
    /// Z.B. könnten hier UUIDs aus einer Datenbank stehen oder URLs zu einem Backend-System.
    #[cfg_attr(feature = "serde", serde(rename = "_id"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub id: Option<String>,
    /// Die einzelnen Positionen, die mit dem Preisblatt abgerechnet werden können. Z.B. Arbeitspreis, Grundpreis etc
    #[cfg_attr(feature = "serde", serde(rename = "preispositionen"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub preispositionen: Option<Vec<Preisposition>>,
    /// Merkmal, das anzeigt, ob es sich um vorläufige oder endgültige Preise handelt
    #[cfg_attr(feature = "serde", serde(rename = "preisstatus"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub preisstatus: Option<Preisstatus>,
    /// Preisblatt gilt für angegebene Sparte
    #[cfg_attr(feature = "serde", serde(rename = "sparte"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub sparte: Option<Sparte>,
    /// BO type identifier — always `BoTyp::Preisblatt` for this struct.
    #[cfg_attr(feature = "serde", serde(rename = "_typ"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(
        feature = "builder",
        builder(default = Some(BoTyp::Preisblatt), setter(skip))
    )]
    pub typ: Option<BoTyp>,
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
impl Default for Preisblatt {
    fn default() -> Self {
        Self {
            typ: Some(BoTyp::Preisblatt),
            bezeichnung: Default::default(),
            gueltigkeit: Default::default(),
            herausgeber: Default::default(),
            id: Default::default(),
            preispositionen: Default::default(),
            preisstatus: Default::default(),
            sparte: Default::default(),
            version: Default::default(),
            zusatz_attribute: Default::default(),
            _additional: Default::default(),
        }
    }
}
impl Bo4eObject for Preisblatt {
    type BoTyp = BoTyp;
    fn bo_type(&self) -> BoTyp {
        self.typ.unwrap_or(BoTyp::Preisblatt)
    }
    fn schema_version(&self) -> &'static str {
        "v202607.0.0"
    }
}
#[cfg(feature = "json")]
impl crate::json::sealed::Sealed for Preisblatt {}
#[cfg(feature = "json")]
impl crate::json::Bo4eJsonExt for Preisblatt {}
#[cfg(feature = "json")]
impl crate::json::Bo4eExtensionData for Preisblatt {
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
impl std::fmt::Display for Preisblatt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => f.write_str(&json),
            Err(e) => write!(f, "<Preisblatt: serialization error: {e}>"),
        }
    }
}
