use super::{
    BdewArtikelnummer, Betrag, ComTyp, Menge, Mengeneinheit, Steuerbetrag, ZusatzAttribut,
};
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(not(feature = "json"), derive(Hash))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "validate", derive(garde::Validate))]
#[cfg_attr(feature = "validate", garde(allow_unvalidated))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Über Rechnungspositionen werden Rechnungen strukturiert.
/// In einem Rechnungsteil wird jeweils eine in sich geschlossene Leistung abgerechnet.
///
/// > **Note:** [Rechnungsposition JSON Schema](https://json-schema.app/view/%23?url=https://raw.githubusercontent.com/BO4E/BO4E-Schemas/v202501.0.0/src/bo4e_schemas/com/Rechnungsposition.json)
pub struct Rechnungsposition {
    /// Standardisierte vom BDEW herausgegebene Liste, welche im Strommarkt die BDEW-Artikelnummer ablöst
    #[cfg_attr(feature = "serde", serde(rename = "artikelId"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub artikel_id: Option<String>,
    /// Kennzeichnung der Rechnungsposition mit der Standard-Artikelnummer des BDEW
    #[cfg_attr(feature = "serde", serde(rename = "artikelnummer"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub artikelnummer: Option<BdewArtikelnummer>,
    /// Der Preis für eine Einheit der energetischen Menge
    #[cfg_attr(feature = "serde", serde(rename = "einzelpreis"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    #[cfg(feature = "decimal")]
    pub einzelpreis: Option<rust_decimal::Decimal>,
    /// Requires the `decimal` feature for the `rust_decimal::Decimal` representation.
    /// Without `decimal`, stores the decimal string value unchanged.
    #[cfg_attr(feature = "serde", serde(rename = "einzelpreis"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    #[cfg(not(feature = "decimal"))]
    pub einzelpreis: Option<String>,
    /// Eine generische ID, die für eigene Zwecke genutzt werden kann.
    /// Z.B. könnten hier UUIDs aus einer Datenbank stehen oder URLs zu einem Backend-System.
    #[cfg_attr(feature = "serde", serde(rename = "_id"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub id: Option<String>,
    /// Ende der Lieferung für die abgerechnete Leistung (exklusiv)
    #[cfg_attr(feature = "serde", serde(rename = "lieferungBis"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub lieferung_bis: Option<String>,
    /// Start der Lieferung für die abgerechnete Leistung (inklusiv)
    #[cfg_attr(feature = "serde", serde(rename = "lieferungVon"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub lieferung_von: Option<String>,
    /// Marktlokation, die zu dieser Position gehört
    #[cfg_attr(feature = "serde", serde(rename = "lokationsId"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub lokations_id: Option<String>,
    /// Die abgerechnete Menge mit Einheit
    #[cfg_attr(feature = "serde", serde(rename = "positionsMenge"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub positions_menge: Option<Menge>,
    /// Fortlaufende Nummer für die Rechnungsposition
    #[cfg_attr(feature = "serde", serde(rename = "positionsnummer"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub positionsnummer: Option<i64>,
    /// Bezeichung für die abgerechnete Position
    #[cfg_attr(feature = "serde", serde(rename = "positionstext"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub positionstext: Option<String>,
    /// Nettobetrag für den Rabatt dieser Position
    #[cfg_attr(feature = "serde", serde(rename = "teilrabattNetto"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub teilrabatt_netto: Option<Betrag>,
    /// Das Ergebnis der Multiplikation aus einzelpreis * positionsMenge * (Faktor aus zeitbezogeneMenge).
    /// Z.B. 12,60€ * 120 kW * 3/12 (für 3 Monate).
    #[cfg_attr(feature = "serde", serde(rename = "teilsummeNetto"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub teilsumme_netto: Option<Betrag>,
    /// Auf die Position entfallende Steuer, bestehend aus Steuersatz und Betrag
    #[cfg_attr(feature = "serde", serde(rename = "teilsummeSteuer"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub teilsumme_steuer: Option<Steuerbetrag>,
    /// COM type identifier for this struct.
    #[cfg_attr(feature = "serde", serde(rename = "_typ"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub typ: Option<ComTyp>,
    /// Version der COM-Struktur aka "fachliche Versionierung"
    #[cfg_attr(feature = "serde", serde(rename = "_version"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub version: Option<String>,
    /// Eine auf die Zeiteinheit bezogene Untermenge.
    /// Z.B. bei einem Jahrespreis, 3 Monate oder 146 Tage.
    /// Basierend darauf wird der Preis aufgeteilt.
    #[cfg_attr(feature = "serde", serde(rename = "zeitbezogeneMenge"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub zeitbezogene_menge: Option<Menge>,
    /// Falls sich der Preis auf eine Zeit bezieht, steht hier die Einheit
    #[cfg_attr(feature = "serde", serde(rename = "zeiteinheit"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub zeiteinheit: Option<Mengeneinheit>,
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
impl crate::json::sealed::Sealed for Rechnungsposition {}
#[cfg(feature = "json")]
impl crate::json::Bo4eJsonExt for Rechnungsposition {}
#[cfg(feature = "json")]
impl crate::json::Bo4eExtensionData for Rechnungsposition {
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
impl std::fmt::Display for Rechnungsposition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => f.write_str(&json),
            Err(e) => write!(f, "<Rechnungsposition: serialization error: {e}>"),
        }
    }
}
