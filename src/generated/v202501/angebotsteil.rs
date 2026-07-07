use super::{Angebotsposition, Betrag, ComTyp, Marktlokation, Menge, Zeitraum, ZusatzAttribut};
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(not(feature = "json"), derive(Hash))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "validate", derive(garde::Validate))]
#[cfg_attr(feature = "validate", garde(allow_unvalidated))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Mit dieser Komponente wird ein Teil einer Angebotsvariante abgebildet.
/// Hier werden alle Angebotspositionen aggregiert.
/// Angebotsteile werden im einfachsten Fall für eine Marktlokation oder Lieferstellenadresse erzeugt.
/// Hier werden die Mengen und Gesamtkosten aller Angebotspositionen zusammengefasst.
/// Eine Variante besteht mindestens aus einem Angebotsteil.
///
/// > **Note:** [Angebotsteil JSON Schema](https://json-schema.app/view/%23?url=https://raw.githubusercontent.com/BO4E/BO4E-Schemas/v202501.0.0/src/bo4e_schemas/com/Angebotsteil.json)
pub struct Angebotsteil {
    /// Identifizierung eines Subkapitels einer Anfrage, beispielsweise das Los einer Ausschreibung
    #[cfg_attr(feature = "serde", serde(rename = "anfrageSubreferenz"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub anfrage_subreferenz: Option<String>,
    /// Summe der Jahresenergiekosten aller in diesem Angebotsteil enthaltenen Lieferstellen
    #[cfg_attr(feature = "serde", serde(rename = "gesamtkostenangebotsteil"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub gesamtkostenangebotsteil: Option<Betrag>,
    /// Summe der Verbräuche aller in diesem Angebotsteil eingeschlossenen Lieferstellen
    #[cfg_attr(feature = "serde", serde(rename = "gesamtmengeangebotsteil"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub gesamtmengeangebotsteil: Option<Menge>,
    /// Eine generische ID, die für eigene Zwecke genutzt werden kann.
    /// Z.B. könnten hier UUIDs aus einer Datenbank stehen oder URLs zu einem Backend-System.
    #[cfg_attr(feature = "serde", serde(rename = "_id"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub id: Option<String>,
    /// Marktlokationen, für die dieses Angebotsteil gilt, falls vorhanden.
    /// Durch die Marktlokation ist auch die Lieferadresse festgelegt
    #[cfg_attr(feature = "serde", serde(rename = "lieferstellenangebotsteil"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub lieferstellenangebotsteil: Option<Vec<Box<Marktlokation>>>,
    /// Hier kann der Belieferungszeitraum angegeben werden, für den dieser Angebotsteil gilt
    #[cfg_attr(feature = "serde", serde(rename = "lieferzeitraum"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub lieferzeitraum: Option<Zeitraum>,
    /// Einzelne Positionen, die zu diesem Angebotsteil gehören
    #[cfg_attr(feature = "serde", serde(rename = "positionen"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub positionen: Option<Vec<Angebotsposition>>,
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
impl crate::json::sealed::Sealed for Angebotsteil {}
#[cfg(feature = "json")]
impl crate::json::Bo4eJsonExt for Angebotsteil {}
#[cfg(feature = "json")]
impl crate::json::Bo4eExtensionData for Angebotsteil {
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
impl std::fmt::Display for Angebotsteil {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => f.write_str(&json),
            Err(e) => write!(f, "<Angebotsteil: serialization error: {e}>"),
        }
    }
}
