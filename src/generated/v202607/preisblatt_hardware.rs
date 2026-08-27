use super::{
    Bilanzierungsmethode, Bo4eObject, Bo4eTyped, BoTyp, Dienstleistungstyp, Geraet,
    Marktteilnehmer, Netzebene, Preisposition, Preisstatus, Sparte, Zeitraum, ZusatzAttribut,
};
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(not(feature = "json"), derive(Eq, Hash))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "validate", derive(garde::Validate))]
#[cfg_attr(feature = "validate", garde(allow_unvalidated))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Variante des Preisblattmodells zur Abbildung der Preise für zusätzliche Hardware
///
/// > **Note:** [PreisblattHardware JSON Schema](https://json-schema.app/view/%23?url=https://raw.githubusercontent.com/BO4E/BO4E-Schemas/v202607.1.0/src/bo4e_schemas/bo/PreisblattHardware.json)
pub struct PreisblattHardware {
    /// Der Preis betriftt das hier angegebene Gerät, z.B. ein Tarifschaltgerät
    #[cfg_attr(feature = "serde", serde(rename = "basisgeraet"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub basisgeraet: Option<Box<Geraet>>,
    /// Eine Bezeichnung für das Preisblatt
    #[cfg_attr(feature = "serde", serde(rename = "bezeichnung"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub bezeichnung: Option<String>,
    /// Die Preise gelten für Marktlokationen der angebebenen Bilanzierungsmethode
    #[cfg_attr(feature = "serde", serde(rename = "bilanzierungsmethode"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub bilanzierungsmethode: Option<Bilanzierungsmethode>,
    /// Der Zeitraum für den der Preis festgelegt ist
    #[cfg_attr(feature = "serde", serde(rename = "gueltigkeit"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub gueltigkeit: Option<Zeitraum>,
    /// Der Netzbetreiber, der die Preise veröffentlicht hat
    #[cfg_attr(feature = "serde", serde(rename = "herausgeber"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub herausgeber: Option<Box<Marktteilnehmer>>,
    /// Eine generische ID, die für eigene Zwecke genutzt werden kann.
    /// Z.B. könnten hier UUIDs aus einer Datenbank stehen oder URLs zu einem Backend-System.
    #[cfg_attr(feature = "serde", serde(rename = "_id"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub id: Option<String>,
    /// Im Preis sind die hier angegebenen Dienstleistungen enthalten, z.B. Jährliche Ablesung
    #[cfg_attr(feature = "serde", serde(rename = "inklusiveDienstleistungen"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub inklusive_dienstleistungen: Option<Vec<Dienstleistungstyp>>,
    /// Im Preis sind die hier angegebenen Geräte mit enthalten, z.B. ein Wandler
    #[cfg_attr(feature = "serde", serde(rename = "inklusiveGeraete"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub inklusive_geraete: Option<Vec<Box<Geraet>>>,
    /// Die Preise gelten für Messlokationen in der angebebenen Netzebene
    #[cfg_attr(feature = "serde", serde(rename = "messebene"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub messebene: Option<Netzebene>,
    /// Die einzelnen Positionen, die mit dem Preisblatt abgerechnet werden können. Z.B. Arbeitspreis, Grundpreis etc
    #[cfg_attr(feature = "serde", serde(rename = "preispositionen"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
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
    /// BO4E type discriminant — always `BoTyp::PreisblattHardware` for this struct.
    #[cfg_attr(feature = "serde", serde(rename = "_typ"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(
        feature = "builder",
        builder(default = Some(BoTyp::PreisblattHardware), setter(skip))
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
impl Default for PreisblattHardware {
    fn default() -> Self {
        Self {
            basisgeraet: Default::default(),
            bezeichnung: Default::default(),
            bilanzierungsmethode: Default::default(),
            gueltigkeit: Default::default(),
            herausgeber: Default::default(),
            id: Default::default(),
            inklusive_dienstleistungen: Default::default(),
            inklusive_geraete: Default::default(),
            messebene: Default::default(),
            preispositionen: Default::default(),
            preisstatus: Default::default(),
            sparte: Default::default(),
            typ: Some(BoTyp::PreisblattHardware),
            version: Some("202607.1.0".to_owned()),
            zusatz_attribute: Default::default(),
            _additional: Default::default(),
        }
    }
}
impl Bo4eTyped for PreisblattHardware {
    type Typ = BoTyp;
    const TYP: BoTyp = BoTyp::PreisblattHardware;
    const TYP_WIRE: &'static str = "PREISBLATTHARDWARE";
    const SCHEMA_VERSION: &'static str = "202607.1.0";
    const SCHEMA_SERIES: &'static str = "202607";
}
impl crate::bo4e_typed_sealed::Sealed for PreisblattHardware {}
impl Bo4eObject for PreisblattHardware {}
impl crate::bo4e_object_sealed::Sealed for PreisblattHardware {}
#[cfg(feature = "json")]
impl crate::json::sealed::Sealed for PreisblattHardware {}
#[cfg(feature = "json")]
impl crate::json::Bo4eJsonExt for PreisblattHardware {}
#[cfg(feature = "json")]
impl crate::json::Bo4eExtensionData for PreisblattHardware {
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
impl std::fmt::Display for PreisblattHardware {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => f.write_str(&json),
            Err(e) => write!(f, "<PreisblattHardware: serialization error: {e}>"),
        }
    }
}
impl crate::Bo4eStrict for PreisblattHardware {
    fn collect_unknown_enums(&self, path: &str, out: &mut Vec<String>) {
        if let Some(v) = &self.basisgeraet {
            crate::Bo4eStrict::collect_unknown_enums(
                &**v,
                &crate::strict::field_path(path, "basisgeraet"),
                out,
            );
        }
        if let Some(v) = &self.bilanzierungsmethode {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "bilanzierungsmethode"),
                out,
            );
        }
        if let Some(v) = &self.gueltigkeit {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "gueltigkeit"),
                out,
            );
        }
        if let Some(v) = &self.herausgeber {
            crate::Bo4eStrict::collect_unknown_enums(
                &**v,
                &crate::strict::field_path(path, "herausgeber"),
                out,
            );
        }
        if let Some(items) = &self.inklusive_dienstleistungen {
            let child = crate::strict::field_path(path, "inklusiveDienstleistungen");
            for (i, item) in items.iter().enumerate() {
                crate::Bo4eStrict::collect_unknown_enums(
                    item,
                    &crate::strict::index_path(&child, i),
                    out,
                );
            }
        }
        if let Some(items) = &self.inklusive_geraete {
            let child = crate::strict::field_path(path, "inklusiveGeraete");
            for (i, item) in items.iter().enumerate() {
                crate::Bo4eStrict::collect_unknown_enums(
                    &**item,
                    &crate::strict::index_path(&child, i),
                    out,
                );
            }
        }
        if let Some(v) = &self.messebene {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "messebene"),
                out,
            );
        }
        if let Some(items) = &self.preispositionen {
            let child = crate::strict::field_path(path, "preispositionen");
            for (i, item) in items.iter().enumerate() {
                crate::Bo4eStrict::collect_unknown_enums(
                    item,
                    &crate::strict::index_path(&child, i),
                    out,
                );
            }
        }
        if let Some(v) = &self.preisstatus {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "preisstatus"),
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
