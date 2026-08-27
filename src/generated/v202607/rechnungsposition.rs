use super::{
    BdewArtikelnummer, Betrag, Bo4eComponent, Bo4eTyped, ComTyp, Menge, Mengeneinheit, Preis,
    Steuerbetrag, Zeitraum, ZusatzAttribut,
};
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(not(feature = "json"), derive(Eq, Hash))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "validate", derive(garde::Validate))]
#[cfg_attr(feature = "validate", garde(allow_unvalidated))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Über Rechnungspositionen werden Rechnungen strukturiert.
/// In einem Rechnungsteil wird jeweils eine in sich geschlossene Leistung abgerechnet.
///
/// > **Note:** [Rechnungsposition JSON Schema](https://json-schema.app/view/%23?url=https://raw.githubusercontent.com/BO4E/BO4E-Schemas/v202607.1.0/src/bo4e_schemas/com/Rechnungsposition.json)
pub struct Rechnungsposition {
    /// Standardisierte vom BDEW herausgegebene Liste, welche im Strommarkt die BDEW-Artikelnummer ablöst
    #[cfg_attr(feature = "serde", serde(rename = "artikelId"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub artikel_id: Option<String>,
    /// Kennzeichnung der Rechnungsposition mit der Standard-Artikelnummer des BDEW
    #[cfg_attr(feature = "serde", serde(rename = "artikelnummer"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub artikelnummer: Option<BdewArtikelnummer>,
    /// Der Preis für eine Einheit der energetischen Menge
    #[cfg_attr(feature = "serde", serde(rename = "einzelpreis"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub einzelpreis: Option<Preis>,
    /// Das Ergebnis der Multiplikation aus einzelpreis * positionsMenge * (Faktor aus zeitbezogeneMenge).
    /// Z.B. 12,60€ * 120 kW * 3/12 (für 3 Monate).
    #[cfg_attr(feature = "serde", serde(rename = "gesamtpreis"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub gesamtpreis: Option<Betrag>,
    /// Eine generische ID, die für eigene Zwecke genutzt werden kann.
    /// Z.B. könnten hier UUIDs aus einer Datenbank stehen oder URLs zu einem Backend-System.
    #[cfg_attr(feature = "serde", serde(rename = "_id"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub id: Option<String>,
    /// Zeitraum der Lieferung für die abgerechnete Leistung
    #[cfg_attr(feature = "serde", serde(rename = "lieferungszeitraum"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub lieferungszeitraum: Option<Zeitraum>,
    /// Die abgerechnete Menge mit Einheit
    #[cfg_attr(feature = "serde", serde(rename = "positionsMenge"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub positions_menge: Option<Menge>,
    /// Fortlaufende Nummer für die Rechnungsposition
    #[cfg_attr(feature = "serde", serde(rename = "positionsnummer"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub positionsnummer: Option<i64>,
    /// Bezeichung für die abgerechnete Position
    #[cfg_attr(feature = "serde", serde(rename = "positionstext"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub positionstext: Option<String>,
    /// Auf die Position entfallende Steuer, bestehend aus Steuersatz und Betrag
    #[cfg_attr(feature = "serde", serde(rename = "steuerbetrag"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub steuerbetrag: Option<Steuerbetrag>,
    /// BO4E type discriminant — always `ComTyp::Rechnungsposition` for this struct.
    #[cfg_attr(feature = "serde", serde(rename = "_typ"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(
        feature = "builder",
        builder(default = Some(ComTyp::Rechnungsposition), setter(skip))
    )]
    pub typ: Option<ComTyp>,
    /// Version der COM-Struktur aka "fachliche Versionierung"
    #[cfg_attr(feature = "serde", serde(rename = "_version"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(
        feature = "builder",
        builder(default = Some("202607.1.0".to_owned()), setter(into))
    )]
    pub version: Option<String>,
    /// Eine auf die Zeiteinheit bezogene Untermenge.
    /// Z.B. bei einem Jahrespreis, 3 Monate oder 146 Tage.
    /// Basierend darauf wird der Preis aufgeteilt.
    #[cfg_attr(feature = "serde", serde(rename = "zeitbezogeneMenge"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub zeitbezogene_menge: Option<Menge>,
    /// Falls sich der Preis auf eine Zeit bezieht, steht hier die Einheit
    #[cfg_attr(feature = "serde", serde(rename = "zeiteinheit"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub zeiteinheit: Option<Mengeneinheit>,
    #[cfg_attr(feature = "serde", serde(rename = "zusatzAttribute"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
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
impl Default for Rechnungsposition {
    fn default() -> Self {
        Self {
            artikel_id: Default::default(),
            artikelnummer: Default::default(),
            einzelpreis: Default::default(),
            gesamtpreis: Default::default(),
            id: Default::default(),
            lieferungszeitraum: Default::default(),
            positions_menge: Default::default(),
            positionsnummer: Default::default(),
            positionstext: Default::default(),
            steuerbetrag: Default::default(),
            typ: Some(ComTyp::Rechnungsposition),
            version: Some("202607.1.0".to_owned()),
            zeitbezogene_menge: Default::default(),
            zeiteinheit: Default::default(),
            zusatz_attribute: Default::default(),
            _additional: Default::default(),
        }
    }
}
impl Bo4eTyped for Rechnungsposition {
    type Typ = ComTyp;
    const TYP: ComTyp = ComTyp::Rechnungsposition;
    const TYP_WIRE: &'static str = "RECHNUNGSPOSITION";
    const SCHEMA_VERSION: &'static str = "202607.1.0";
    const SCHEMA_SERIES: &'static str = "202607";
}
impl crate::bo4e_typed_sealed::Sealed for Rechnungsposition {}
impl Bo4eComponent for Rechnungsposition {}
impl crate::bo4e_component_sealed::Sealed for Rechnungsposition {}
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
impl crate::Bo4eStrict for Rechnungsposition {
    fn collect_unknown_enums(&self, path: &str, out: &mut Vec<String>) {
        if let Some(v) = &self.artikelnummer {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "artikelnummer"),
                out,
            );
        }
        if let Some(v) = &self.einzelpreis {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "einzelpreis"),
                out,
            );
        }
        if let Some(v) = &self.gesamtpreis {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "gesamtpreis"),
                out,
            );
        }
        if let Some(v) = &self.lieferungszeitraum {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "lieferungszeitraum"),
                out,
            );
        }
        if let Some(v) = &self.positions_menge {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "positionsMenge"),
                out,
            );
        }
        if let Some(v) = &self.steuerbetrag {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "steuerbetrag"),
                out,
            );
        }
        if let Some(v) = &self.zeitbezogene_menge {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "zeitbezogeneMenge"),
                out,
            );
        }
        if let Some(v) = &self.zeiteinheit {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "zeiteinheit"),
                out,
            );
        }
        if let Some(items) = &self.zusatz_attribute {
            let child = crate::strict::field_path(path, "zusatzAttribute");
            for (i, item) in items.iter().enumerate() {
                crate::Bo4eStrict::collect_unknown_enums(
                    item,
                    &crate::strict::index_path(&child, i),
                    out,
                );
            }
        }
    }
}
#[cfg(feature = "json")]
impl crate::json::Bo4eExtensions for Rechnungsposition {
    fn collect_extension_paths(&self, path: &str, out: &mut Vec<String>) {
        if let Some(map) = self._additional.as_map() {
            for key in map.keys() {
                out.push(crate::strict::extension_path(path, key));
            }
        }
        if let Some(v) = &self.einzelpreis {
            crate::json::Bo4eExtensions::collect_extension_paths(
                v,
                &crate::strict::field_path(path, "einzelpreis"),
                out,
            );
        }
        if let Some(v) = &self.gesamtpreis {
            crate::json::Bo4eExtensions::collect_extension_paths(
                v,
                &crate::strict::field_path(path, "gesamtpreis"),
                out,
            );
        }
        if let Some(v) = &self.lieferungszeitraum {
            crate::json::Bo4eExtensions::collect_extension_paths(
                v,
                &crate::strict::field_path(path, "lieferungszeitraum"),
                out,
            );
        }
        if let Some(v) = &self.positions_menge {
            crate::json::Bo4eExtensions::collect_extension_paths(
                v,
                &crate::strict::field_path(path, "positionsMenge"),
                out,
            );
        }
        if let Some(v) = &self.steuerbetrag {
            crate::json::Bo4eExtensions::collect_extension_paths(
                v,
                &crate::strict::field_path(path, "steuerbetrag"),
                out,
            );
        }
        if let Some(v) = &self.zeitbezogene_menge {
            crate::json::Bo4eExtensions::collect_extension_paths(
                v,
                &crate::strict::field_path(path, "zeitbezogeneMenge"),
                out,
            );
        }
        if let Some(items) = &self.zusatz_attribute {
            let child = crate::strict::field_path(path, "zusatzAttribute");
            for (i, item) in items.iter().enumerate() {
                crate::json::Bo4eExtensions::collect_extension_paths(
                    item,
                    &crate::strict::index_path(&child, i),
                    out,
                );
            }
        }
    }
}
