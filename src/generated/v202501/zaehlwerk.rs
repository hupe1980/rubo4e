use super::{
    ComTyp, Energierichtung, Konzessionsabgabe, Mengeneinheit, Verbrauchsart,
    VerwendungszweckProMarktrolle, Waermenutzung, Zaehlzeitregister, ZusatzAttribut,
};
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(not(feature = "json"), derive(Hash))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "validate", derive(garde::Validate))]
#[cfg_attr(feature = "validate", garde(allow_unvalidated))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Mit dieser Komponente werden Zählwerke modelliert.
///
/// > **Note:** [Zaehlwerk JSON Schema](https://json-schema.app/view/%23?url=https://raw.githubusercontent.com/BO4E/BO4E-Schemas/v202501.0.0/src/bo4e_schemas/com/Zaehlwerk.json)
pub struct Zaehlwerk {
    /// Anzahl Ablesungen pro Jahr
    #[cfg_attr(feature = "serde", serde(rename = "anzahlAblesungen"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub anzahl_ablesungen: Option<i64>,
    /// Zusätzliche Bezeichnung, z.B. Zählwerk_Wirkarbeit.
    #[cfg_attr(feature = "serde", serde(rename = "bezeichnung"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub bezeichnung: Option<String>,
    /// Die Einheit der gemessenen Größe, z.B. kWh
    #[cfg_attr(feature = "serde", serde(rename = "einheit"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub einheit: Option<Mengeneinheit>,
    /// Eine generische ID, die für eigene Zwecke genutzt werden kann.
    /// Z.B. könnten hier UUIDs aus einer Datenbank stehen oder URLs zu einem Backend-System.
    #[cfg_attr(feature = "serde", serde(rename = "_id"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub id: Option<String>,
    /// Abrechnungsrelevant
    #[cfg_attr(feature = "serde", serde(rename = "istAbrechnungsrelevant"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub ist_abrechnungsrelevant: Option<bool>,
    /// Schwachlastfaehigkeit
    #[cfg_attr(feature = "serde", serde(rename = "istSchwachlastfaehig"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub ist_schwachlastfaehig: Option<bool>,
    /// Steuerbefreiung
    #[cfg_attr(feature = "serde", serde(rename = "istSteuerbefreit"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub ist_steuerbefreit: Option<bool>,
    /// Unterbrechbarkeit Marktlokation
    #[cfg_attr(feature = "serde", serde(rename = "istUnterbrechbar"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub ist_unterbrechbar: Option<bool>,
    /// Konzessionsabgabe
    #[cfg_attr(feature = "serde", serde(rename = "konzessionsabgabe"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub konzessionsabgabe: Option<Konzessionsabgabe>,
    /// Anzahl der Nachkommastellen
    #[cfg_attr(feature = "serde", serde(rename = "nachkommastelle"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub nachkommastelle: Option<i64>,
    /// Die OBIS-Kennzahl für das Zählwerk, die festlegt, welche auf die gemessene Größe mit dem Stand gemeldet wird.
    /// Nur Zählwerkstände mit dieser OBIS-Kennzahl werden an diesem Zählwerk registriert.
    #[cfg_attr(feature = "serde", serde(rename = "obisKennzahl"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub obis_kennzahl: Option<crate::identifiers::ObisCode>,
    /// Die Energierichtung, Einspeisung oder Ausspeisung.
    #[cfg_attr(feature = "serde", serde(rename = "richtung"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub richtung: Option<Energierichtung>,
    /// COM type identifier for this struct.
    #[cfg_attr(feature = "serde", serde(rename = "_typ"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub typ: Option<ComTyp>,
    /// Stromverbrauchsart/Verbrauchsart Marktlokation
    #[cfg_attr(feature = "serde", serde(rename = "verbrauchsart"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub verbrauchsart: Option<Verbrauchsart>,
    /// Version der COM-Struktur aka "fachliche Versionierung"
    #[cfg_attr(feature = "serde", serde(rename = "_version"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub version: Option<String>,
    /// Verwendungungszweck der Werte Marktlokation
    #[cfg_attr(feature = "serde", serde(rename = "verwendungszwecke"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub verwendungszwecke: Option<Vec<VerwendungszweckProMarktrolle>>,
    /// Anzahl der Vorkommastellen
    #[cfg_attr(feature = "serde", serde(rename = "vorkommastelle"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub vorkommastelle: Option<i64>,
    /// Wärmenutzung Marktlokation
    #[cfg_attr(feature = "serde", serde(rename = "waermenutzung"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub waermenutzung: Option<Waermenutzung>,
    /// Mit diesem Faktor wird eine Zählerstandsdifferenz multipliziert, um zum eigentlichen Verbrauch im Zeitraum
    /// zu kommen.
    #[cfg_attr(feature = "serde", serde(rename = "wandlerfaktor"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    #[cfg(feature = "decimal")]
    pub wandlerfaktor: Option<rust_decimal::Decimal>,
    /// Requires the `decimal` feature for the `rust_decimal::Decimal` representation.
    /// Without `decimal`, stores the decimal string value unchanged.
    #[cfg_attr(feature = "serde", serde(rename = "wandlerfaktor"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    #[cfg(not(feature = "decimal"))]
    pub wandlerfaktor: Option<String>,
    /// Identifikation des Zählwerks (Registers) innerhalb des Zählers.
    /// Oftmals eine laufende Nummer hinter der Zählernummer. Z.B. 47110815_1
    #[cfg_attr(feature = "serde", serde(rename = "zaehlwerkId"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub zaehlwerk_id: Option<String>,
    /// Erweiterte Definition der Zählzeit in Bezug auf ein Register
    #[cfg_attr(feature = "serde", serde(rename = "zaehlzeitregister"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub zaehlzeitregister: Option<Zaehlzeitregister>,
    #[cfg_attr(feature = "serde", serde(rename = "zusatzAttribute"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
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
    pub(crate) _additional: crate::LimitedExtensionMap,
}
#[cfg(feature = "json")]
impl crate::json::sealed::Sealed for Zaehlwerk {}
#[cfg(feature = "json")]
impl crate::json::Bo4eJsonExt for Zaehlwerk {}
#[cfg(feature = "json")]
impl crate::json::Bo4eExtensionData for Zaehlwerk {
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
impl std::fmt::Display for Zaehlwerk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => f.write_str(&json),
            Err(e) => write!(f, "<Zaehlwerk: serialization error: {e}>"),
        }
    }
}
