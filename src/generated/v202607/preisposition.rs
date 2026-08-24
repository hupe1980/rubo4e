use super::{
    BdewArtikelnummer, Bemessungsgroesse, ComTyp, Kalkulationsmethode, Leistungstyp, Mengeneinheit,
    Preisstaffel, Tarifzeit, Waehrungseinheit, ZusatzAttribut,
};
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(not(feature = "json"), derive(Eq, Hash))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "validate", derive(garde::Validate))]
#[cfg_attr(feature = "validate", garde(allow_unvalidated))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Preis für eine definierte Lieferung oder Leistung innerhalb eines Preisblattes
///
/// > **Note:** [Preisposition JSON Schema](https://json-schema.app/view/%23?url=https://raw.githubusercontent.com/BO4E/BO4E-Schemas/v202607.1.0/src/bo4e_schemas/com/Preisposition.json)
pub struct Preisposition {
    /// Eine vom BDEW standardisierte Bezeichnug für die abgerechnete Leistungserbringung;
    /// Diese Artikelnummer wird auch im Rechnungsteil der INVOIC verwendet.
    #[cfg_attr(feature = "serde", serde(rename = "bdewArtikelnummer"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub bdew_artikelnummer: Option<BdewArtikelnummer>,
    /// Das Modell, das der Preisbildung zugrunde liegt
    #[cfg_attr(feature = "serde", serde(rename = "berechnungsmethode"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub berechnungsmethode: Option<Kalkulationsmethode>,
    /// Hier wird festgelegt, auf welche Bezugsgrösse sich der Preis bezieht, z.B. kWh oder Stück
    #[cfg_attr(feature = "serde", serde(rename = "bezugsgroesse"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub bezugsgroesse: Option<Mengeneinheit>,
    /// Der Anteil der Menge der Blindarbeit in Prozent von der Wirkarbeit, für die keine Abrechnung erfolgt
    #[cfg_attr(feature = "serde", serde(rename = "freimengeBlindarbeit"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "crate::decimal_serde::deserialize_opt")
    )]
    #[cfg(feature = "decimal")]
    pub freimenge_blindarbeit: Option<rust_decimal::Decimal>,
    /// Requires the `decimal` feature for the `rust_decimal::Decimal` representation.
    /// Without `decimal`, stores the decimal's lexical form (a JSON string or number).
    #[cfg_attr(feature = "serde", serde(rename = "freimengeBlindarbeit"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "crate::decimal_serde::deserialize_opt")
    )]
    #[cfg(not(feature = "decimal"))]
    pub freimenge_blindarbeit: Option<String>,
    /// Der cos phi (Verhältnis Wirkleistung/Scheinleistung) aus dem die Freimenge für die Blindarbeit berechnet wird als
    /// tan phi (Verhältnis Blindleistung/Wirkleistung)
    #[cfg_attr(feature = "serde", serde(rename = "freimengeLeistungsfaktor"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "crate::decimal_serde::deserialize_opt")
    )]
    #[cfg(feature = "decimal")]
    pub freimenge_leistungsfaktor: Option<rust_decimal::Decimal>,
    /// Requires the `decimal` feature for the `rust_decimal::Decimal` representation.
    /// Without `decimal`, stores the decimal's lexical form (a JSON string or number).
    #[cfg_attr(feature = "serde", serde(rename = "freimengeLeistungsfaktor"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "crate::decimal_serde::deserialize_opt")
    )]
    #[cfg(not(feature = "decimal"))]
    pub freimenge_leistungsfaktor: Option<String>,
    /// Übergeordnete Gruppen-ID, die sich ggf. auf die Artikel-ID in der Preisstaffel bezieht
    #[cfg_attr(feature = "serde", serde(rename = "gruppenartikelId"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub gruppenartikel_id: Option<String>,
    /// Eine generische ID, die für eigene Zwecke genutzt werden kann.
    /// Z.B. könnten hier UUIDs aus einer Datenbank stehen oder URLs zu einem Backend-System.
    #[cfg_attr(feature = "serde", serde(rename = "_id"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub id: Option<String>,
    /// Bezeichnung für die in der Position abgebildete Leistungserbringung
    #[cfg_attr(feature = "serde", serde(rename = "leistungsbezeichnung"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub leistungsbezeichnung: Option<String>,
    /// Standardisierte Bezeichnung für die abgerechnete Leistungserbringung
    #[cfg_attr(feature = "serde", serde(rename = "leistungstyp"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub leistungstyp: Option<Leistungstyp>,
    /// Festlegung, mit welcher Preiseinheit abgerechnet wird, z.B. Ct. oder €
    #[cfg_attr(feature = "serde", serde(rename = "preiseinheit"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub preiseinheit: Option<Waehrungseinheit>,
    /// Preisstaffeln, die zu dieser Preisposition gehören
    #[cfg_attr(feature = "serde", serde(rename = "preisstaffeln"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub preisstaffeln: Option<Vec<Preisstaffel>>,
    /// Festlegung, für welche Tarifzeit der Preis hier festgelegt ist
    #[cfg_attr(feature = "serde", serde(rename = "tarifzeit"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub tarifzeit: Option<Tarifzeit>,
    /// BO4E type discriminant — always `ComTyp::Preisposition` for this struct.
    #[cfg_attr(feature = "serde", serde(rename = "_typ"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(
        feature = "builder",
        builder(default = Some(ComTyp::Preisposition), setter(skip))
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
    /// Die Zeit(dauer) auf die sich der Preis bezieht.
    /// Z.B. ein Jahr für einen Leistungspreis der in €/kW/Jahr ausgegeben wird
    #[cfg_attr(feature = "serde", serde(rename = "zeitbasis"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub zeitbasis: Option<Mengeneinheit>,
    /// Mit der Menge der hier angegebenen Größe wird die Staffelung/Zonung durchgeführt. Z.B. Vollbenutzungsstunden
    #[cfg_attr(feature = "serde", serde(rename = "zonungsgroesse"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub zonungsgroesse: Option<Bemessungsgroesse>,
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
impl Default for Preisposition {
    fn default() -> Self {
        Self {
            bdew_artikelnummer: Default::default(),
            berechnungsmethode: Default::default(),
            bezugsgroesse: Default::default(),
            freimenge_blindarbeit: Default::default(),
            freimenge_leistungsfaktor: Default::default(),
            gruppenartikel_id: Default::default(),
            id: Default::default(),
            leistungsbezeichnung: Default::default(),
            leistungstyp: Default::default(),
            preiseinheit: Default::default(),
            preisstaffeln: Default::default(),
            tarifzeit: Default::default(),
            typ: Some(ComTyp::Preisposition),
            version: Some("202607.1.0".to_owned()),
            zeitbasis: Default::default(),
            zonungsgroesse: Default::default(),
            zusatz_attribute: Default::default(),
            _additional: Default::default(),
        }
    }
}
#[cfg(feature = "json")]
impl crate::json::sealed::Sealed for Preisposition {}
#[cfg(feature = "json")]
impl crate::json::Bo4eJsonExt for Preisposition {}
#[cfg(feature = "json")]
impl crate::json::Bo4eExtensionData for Preisposition {
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
impl std::fmt::Display for Preisposition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => f.write_str(&json),
            Err(e) => write!(f, "<Preisposition: serialization error: {e}>"),
        }
    }
}
impl crate::Bo4eStrict for Preisposition {
    fn collect_unknown_enums(&self, path: &str, out: &mut Vec<String>) {
        if let Some(v) = &self.bdew_artikelnummer {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "bdewArtikelnummer"),
                out,
            );
        }
        if let Some(v) = &self.berechnungsmethode {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "berechnungsmethode"),
                out,
            );
        }
        if let Some(v) = &self.bezugsgroesse {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "bezugsgroesse"),
                out,
            );
        }
        if let Some(v) = &self.leistungstyp {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "leistungstyp"),
                out,
            );
        }
        if let Some(v) = &self.preiseinheit {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "preiseinheit"),
                out,
            );
        }
        if let Some(items) = &self.preisstaffeln {
            let child = crate::strict::field_path(path, "preisstaffeln");
            for (i, item) in items.iter().enumerate() {
                crate::Bo4eStrict::collect_unknown_enums(
                    item,
                    &crate::strict::index_path(&child, i),
                    out,
                );
            }
        }
        if let Some(v) = &self.tarifzeit {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "tarifzeit"),
                out,
            );
        }
        if let Some(v) = &self.zeitbasis {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "zeitbasis"),
                out,
            );
        }
        if let Some(v) = &self.zonungsgroesse {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "zonungsgroesse"),
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
