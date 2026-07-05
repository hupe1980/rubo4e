use super::{
    Befestigungsart, Bo4eObject, BoTyp, Geraet, Geschaeftspartner, Messwerterfassung,
    Registeranzahl, Sparte, Zaehlerauspraegung, Zaehlergroesse, Zaehlertyp,
    ZaehlertypSpezifikation, Zaehlwerk, ZusatzAttribut,
};
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(not(feature = "json"), derive(Hash))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "validate", derive(garde::Validate))]
#[cfg_attr(feature = "validate", garde(allow_unvalidated))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Object containing information about a meter/"Zaehler".
///
/// > **Note:** [Zaehler JSON Schema](https://json-schema.app/view/%23?url=https://raw.githubusercontent.com/BO4E/BO4E-Schemas/v202501.0.0/src/bo4e_schemas/bo/Zaehler.json)
pub struct Zaehler {
    /// Befestigungsart
    #[cfg_attr(feature = "serde", serde(rename = "befestigungsart"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub befestigungsart: Option<Befestigungsart>,
    /// Bis zu diesem Datum (exklusiv) ist der Zähler geeicht.
    #[cfg_attr(feature = "serde", serde(rename = "eichungBis"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub eichung_bis: Option<String>,
    /// Liste der Geräte, die zu diesem Zähler gehören, bspw. Smartmeter-Gateway
    #[cfg_attr(feature = "serde", serde(rename = "geraete"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub geraete: Option<Vec<Box<Geraet>>>,
    /// Eine generische ID, die für eigene Zwecke genutzt werden kann.
    /// Z.B. könnten hier UUIDs aus einer Datenbank stehen oder URLs zu einem Backend-System.
    #[cfg_attr(feature = "serde", serde(rename = "_id"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub id: Option<String>,
    /// Fernschaltung
    #[cfg_attr(feature = "serde", serde(rename = "istFernschaltbar"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub ist_fernschaltbar: Option<bool>,
    /// Zu diesem Datum fand die letzte Eichprüfung des Zählers statt.
    #[cfg_attr(feature = "serde", serde(rename = "letzteEichung"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub letzte_eichung: Option<String>,
    /// Messwerterfassung des Zählers
    #[cfg_attr(feature = "serde", serde(rename = "messwerterfassung"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub messwerterfassung: Option<Messwerterfassung>,
    /// Spezifikation bezüglich unterstützter Tarif
    #[cfg_attr(feature = "serde", serde(rename = "registeranzahl"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub registeranzahl: Option<Registeranzahl>,
    /// Strom oder Gas
    #[cfg_attr(feature = "serde", serde(rename = "sparte"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub sparte: Option<Sparte>,
    /// BO type identifier — always `BoTyp::Zaehler` for this struct.
    #[cfg_attr(feature = "serde", serde(rename = "_typ"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(
        feature = "builder",
        builder(default = Some(BoTyp::Zaehler), setter(skip))
    )]
    pub typ: Option<BoTyp>,
    /// Version der BO-Struktur aka "fachliche Versionierung"
    #[cfg_attr(feature = "serde", serde(rename = "_version"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub version: Option<String>,
    /// Spezifikation die Richtung des Zählers betreffend
    #[cfg_attr(feature = "serde", serde(rename = "zaehlerauspraegung"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub zaehlerauspraegung: Option<Zaehlerauspraegung>,
    /// Größe des Zählers
    #[cfg_attr(feature = "serde", serde(rename = "zaehlergroesse"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub zaehlergroesse: Option<Zaehlergroesse>,
    /// Der Hersteller des Zählers
    #[cfg_attr(feature = "serde", serde(rename = "zaehlerhersteller"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub zaehlerhersteller: Option<Box<Geschaeftspartner>>,
    /// Zählerkonstante auf dem Zähler
    #[cfg_attr(feature = "serde", serde(rename = "zaehlerkonstante"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg(feature = "decimal")]
    pub zaehlerkonstante: Option<rust_decimal::Decimal>,
    /// Requires the `decimal` feature for the `rust_decimal::Decimal` representation.
    /// Without `decimal`, stores the decimal string value unchanged.
    #[cfg_attr(feature = "serde", serde(rename = "zaehlerkonstante"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg(not(feature = "decimal"))]
    pub zaehlerkonstante: Option<String>,
    /// Nummerierung des Zählers,vergeben durch den Messstellenbetreiber
    #[cfg_attr(feature = "serde", serde(rename = "zaehlernummer"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub zaehlernummer: Option<String>,
    /// Typisierung des Zählers
    #[cfg_attr(feature = "serde", serde(rename = "zaehlertyp"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub zaehlertyp: Option<Zaehlertyp>,
    /// Besondere Spezifikation des Zählers
    #[cfg_attr(feature = "serde", serde(rename = "zaehlertypSpezifikation"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub zaehlertyp_spezifikation: Option<ZaehlertypSpezifikation>,
    #[cfg_attr(feature = "serde", serde(rename = "zaehlwerke"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub zaehlwerke: Option<Vec<Zaehlwerk>>,
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
impl Default for Zaehler {
    fn default() -> Self {
        Self {
            typ: Some(BoTyp::Zaehler),
            befestigungsart: Default::default(),
            eichung_bis: Default::default(),
            geraete: Default::default(),
            id: Default::default(),
            ist_fernschaltbar: Default::default(),
            letzte_eichung: Default::default(),
            messwerterfassung: Default::default(),
            registeranzahl: Default::default(),
            sparte: Default::default(),
            version: Default::default(),
            zaehlerauspraegung: Default::default(),
            zaehlergroesse: Default::default(),
            zaehlerhersteller: Default::default(),
            zaehlerkonstante: Default::default(),
            zaehlernummer: Default::default(),
            zaehlertyp: Default::default(),
            zaehlertyp_spezifikation: Default::default(),
            zaehlwerke: Default::default(),
            zusatz_attribute: Default::default(),
            _additional: Default::default(),
        }
    }
}
impl Bo4eObject for Zaehler {
    type BoTyp = BoTyp;
    fn bo_type(&self) -> BoTyp {
        self.typ.unwrap_or(BoTyp::Zaehler)
    }
    fn schema_version(&self) -> &'static str {
        "v202501.0.0"
    }
}
#[cfg(feature = "json")]
impl crate::json::sealed::Sealed for Zaehler {}
#[cfg(feature = "json")]
impl crate::json::Bo4eJsonExt for Zaehler {}
#[cfg(feature = "json")]
impl crate::json::Bo4eExtensionData for Zaehler {
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
impl std::fmt::Display for Zaehler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => f.write_str(&json),
            Err(e) => write!(f, "<Zaehler: serialization error: {e}>"),
        }
    }
}
