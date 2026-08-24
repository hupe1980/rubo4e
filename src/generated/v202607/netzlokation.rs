use super::{
    Bo4eObject, BoTyp, Konfigurationsprodukt, Lokationszuordnung, Marktrolle, Menge, Sparte,
    VerwendungszweckProMarktrolle, ZusatzAttribut,
};
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(not(feature = "json"), derive(Eq, Hash))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "validate", derive(garde::Validate))]
#[cfg_attr(feature = "validate", garde(allow_unvalidated))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Object containing information about a Netzlokation
///
/// > **Note:** [Netzlokation JSON Schema](https://json-schema.app/view/%23?url=https://raw.githubusercontent.com/BO4E/BO4E-Schemas/v202607.1.0/src/bo4e_schemas/bo/Netzlokation.json)
pub struct Netzlokation {
    /// Eigenschaft des Messstellenbetreibers an der Lokation
    #[cfg_attr(feature = "serde", serde(rename = "eigenschaftMsbLokation"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub eigenschaft_msb_lokation: Option<Marktrolle>,
    /// Codenummer des grundzuständigen Messstellenbetreibers, der für diese Netzlokation zuständig ist.
    #[cfg_attr(feature = "serde", serde(rename = "grundzustaendigerMsbCodenr"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub grundzustaendiger_msb_codenr: Option<crate::identifiers::MarktpartnerId>,
    /// Eine generische ID, die für eigene Zwecke genutzt werden kann.
    /// Z.B. könnten hier UUIDs aus einer Datenbank stehen oder URLs zu einem Backend-System.
    #[cfg_attr(feature = "serde", serde(rename = "_id"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub id: Option<String>,
    /// Produkt-Daten der Netzlokation
    #[cfg_attr(feature = "serde", serde(rename = "konfigurationsprodukte"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub konfigurationsprodukte: Option<Vec<Konfigurationsprodukt>>,
    /// Lokationsbuendel Code, der die Funktion dieses BOs an der Lokationsbuendelstruktur beschreibt.
    #[cfg_attr(feature = "serde", serde(rename = "lokationsbuendelObjektcode"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub lokationsbuendel_objektcode: Option<String>,
    /// Lokationszuordnung, um bspw. die zugehörigen Messlokationen anzugeben
    #[cfg_attr(feature = "serde", serde(rename = "lokationszuordnungen"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub lokationszuordnungen: Option<Vec<Box<Lokationszuordnung>>>,
    /// Netzanschlussleistungsmenge der Netzlokation
    #[cfg_attr(feature = "serde", serde(rename = "netzanschlussleistung"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub netzanschlussleistung: Option<Menge>,
    /// Identifikationsnummer einer Netzlokation, an der Energie entweder verbraucht, oder erzeugt wird
    #[cfg_attr(feature = "serde", serde(rename = "netzlokationsId"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub netzlokations_id: Option<crate::identifiers::NeloId>,
    /// Die OBIS-Kennzahl für die Netzlokation
    #[cfg_attr(feature = "serde", serde(rename = "obiskennzahl"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub obiskennzahl: Option<crate::identifiers::ObisCode>,
    /// Sparte der Netzlokation, z.B. Gas oder Strom.
    #[cfg_attr(feature = "serde", serde(rename = "sparte"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub sparte: Option<Sparte>,
    /// Ob ein Steuerkanal der Netzlokation zugeordnet ist und somit die Netzlokation gesteuert werden kann.
    #[cfg_attr(feature = "serde", serde(rename = "steuerkanal"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub steuerkanal: Option<bool>,
    /// BO4E type discriminant — always `BoTyp::Netzlokation` for this struct.
    #[cfg_attr(feature = "serde", serde(rename = "_typ"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(
        feature = "builder",
        builder(default = Some(BoTyp::Netzlokation), setter(skip))
    )]
    pub typ: Option<BoTyp>,
    /// Version der BO-Struktur aka "fachliche Versionierung"
    #[cfg_attr(feature = "serde", serde(rename = "_version"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(
        feature = "builder",
        builder(default = Some("202607.1.0".to_owned()), setter(into))
    )]
    pub version: Option<String>,
    /// Verwendungungszweck der Werte Netzlokation
    #[cfg_attr(feature = "serde", serde(rename = "verwendungszweck"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub verwendungszweck: Option<VerwendungszweckProMarktrolle>,
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
impl Default for Netzlokation {
    fn default() -> Self {
        Self {
            eigenschaft_msb_lokation: Default::default(),
            grundzustaendiger_msb_codenr: Default::default(),
            id: Default::default(),
            konfigurationsprodukte: Default::default(),
            lokationsbuendel_objektcode: Default::default(),
            lokationszuordnungen: Default::default(),
            netzanschlussleistung: Default::default(),
            netzlokations_id: Default::default(),
            obiskennzahl: Default::default(),
            sparte: Default::default(),
            steuerkanal: Default::default(),
            typ: Some(BoTyp::Netzlokation),
            version: Some("202607.1.0".to_owned()),
            verwendungszweck: Default::default(),
            zusatz_attribute: Default::default(),
            _additional: Default::default(),
        }
    }
}
impl Bo4eObject for Netzlokation {
    type BoTyp = BoTyp;
    fn bo_type(&self) -> BoTyp {
        self.typ.unwrap_or(BoTyp::Netzlokation)
    }
    fn schema_version(&self) -> &'static str {
        "202607.1.0"
    }
    fn schema_series(&self) -> &'static str {
        "202607"
    }
}
#[cfg(feature = "json")]
impl crate::json::sealed::Sealed for Netzlokation {}
#[cfg(feature = "json")]
impl crate::json::Bo4eJsonExt for Netzlokation {}
#[cfg(feature = "json")]
impl crate::json::Bo4eExtensionData for Netzlokation {
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
impl std::fmt::Display for Netzlokation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => f.write_str(&json),
            Err(e) => write!(f, "<Netzlokation: serialization error: {e}>"),
        }
    }
}
impl crate::Bo4eStrict for Netzlokation {
    fn collect_unknown_enums(&self, path: &str, out: &mut Vec<String>) {
        if let Some(v) = &self.eigenschaft_msb_lokation {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "eigenschaftMsbLokation"),
                out,
            );
        }
        if let Some(items) = &self.konfigurationsprodukte {
            let child = crate::strict::field_path(path, "konfigurationsprodukte");
            for (i, item) in items.iter().enumerate() {
                crate::Bo4eStrict::collect_unknown_enums(
                    item,
                    &crate::strict::index_path(&child, i),
                    out,
                );
            }
        }
        if let Some(items) = &self.lokationszuordnungen {
            let child = crate::strict::field_path(path, "lokationszuordnungen");
            for (i, item) in items.iter().enumerate() {
                crate::Bo4eStrict::collect_unknown_enums(
                    &**item,
                    &crate::strict::index_path(&child, i),
                    out,
                );
            }
        }
        if let Some(v) = &self.netzanschlussleistung {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "netzanschlussleistung"),
                out,
            );
        }
        if let Some(v) = &self.sparte {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "sparte"),
                out,
            );
        }
        if let Some(v) = &self.verwendungszweck {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "verwendungszweck"),
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
