use super::{
    AufAbschlagstyp, AufAbschlagsziel, Bo4eComponent, Bo4eTyped, ComTyp, Preisstaffel,
    Waehrungseinheit, Zeitraum, ZusatzAttribut,
};
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(not(feature = "json"), derive(Eq, Hash))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "validate", derive(garde::Validate))]
#[cfg_attr(feature = "validate", garde(allow_unvalidated))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Modell für die preiserhöhenden (Aufschlag) bzw. preisvermindernden (Abschlag) Zusatzvereinbarungen,
/// die individuell zu einem neuen oder bestehenden Liefervertrag abgeschlossen wurden.
///
/// > **Note:** [AufAbschlag JSON Schema](https://json-schema.app/view/%23?url=https://raw.githubusercontent.com/BO4E/BO4E-Schemas/v202607.1.0/src/bo4e_schemas/com/AufAbschlag.json)
pub struct AufAbschlag {
    /// Typ des Aufabschlages (z.B. absolut oder prozentual).
    #[cfg_attr(feature = "serde", serde(rename = "aufAbschlagstyp"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub auf_abschlagstyp: Option<AufAbschlagstyp>,
    /// Diesem Preis oder den Kosten ist der Auf/Abschlag zugeordnet. Z.B. Arbeitspreis, Gesamtpreis etc..
    #[cfg_attr(feature = "serde", serde(rename = "aufAbschlagsziel"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub auf_abschlagsziel: Option<AufAbschlagsziel>,
    /// Beschreibung zum Auf-/Abschlag
    #[cfg_attr(feature = "serde", serde(rename = "beschreibung"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub beschreibung: Option<String>,
    /// Bezeichnung des Auf-/Abschlags
    #[cfg_attr(feature = "serde", serde(rename = "bezeichnung"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub bezeichnung: Option<String>,
    /// Gibt an in welcher Währungseinheit der Auf/Abschlag berechnet wird. Euro oder Ct..
    /// (Nur im Falle absoluter Aufschlagstypen).
    #[cfg_attr(feature = "serde", serde(rename = "einheit"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub einheit: Option<Waehrungseinheit>,
    /// Internetseite, auf der die Informationen zum Auf-/Abschlag veröffentlicht sind.
    #[cfg_attr(feature = "serde", serde(rename = "gueltigkeitszeitraum"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub gueltigkeitszeitraum: Option<Zeitraum>,
    /// Eine generische ID, die für eigene Zwecke genutzt werden kann.
    /// Z.B. könnten hier UUIDs aus einer Datenbank stehen oder URLs zu einem Backend-System.
    #[cfg_attr(feature = "serde", serde(rename = "_id"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub id: Option<String>,
    /// Werte für die gestaffelten Auf/Abschläge.
    #[cfg_attr(feature = "serde", serde(rename = "staffeln"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub staffeln: Option<Vec<Preisstaffel>>,
    /// BO4E type discriminant — always `ComTyp::AufAbschlag` for this struct.
    #[cfg_attr(feature = "serde", serde(rename = "_typ"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(
        feature = "builder",
        builder(default = Some(ComTyp::AufAbschlag), setter(skip))
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
    /// Internetseite, auf der die Informationen zum Auf-/Abschlag veröffentlicht sind.
    #[cfg_attr(feature = "serde", serde(rename = "website"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub website: Option<String>,
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
impl Default for AufAbschlag {
    fn default() -> Self {
        Self {
            auf_abschlagstyp: Default::default(),
            auf_abschlagsziel: Default::default(),
            beschreibung: Default::default(),
            bezeichnung: Default::default(),
            einheit: Default::default(),
            gueltigkeitszeitraum: Default::default(),
            id: Default::default(),
            staffeln: Default::default(),
            typ: Some(ComTyp::AufAbschlag),
            version: Some("202607.1.0".to_owned()),
            website: Default::default(),
            zusatz_attribute: Default::default(),
            _additional: Default::default(),
        }
    }
}
impl Bo4eTyped for AufAbschlag {
    type Typ = ComTyp;
    const TYP: ComTyp = ComTyp::AufAbschlag;
    const TYP_WIRE: &'static str = "AUFABSCHLAG";
    const SCHEMA_VERSION: &'static str = "202607.1.0";
    const SCHEMA_SERIES: &'static str = "202607";
}
impl crate::bo4e_typed_sealed::Sealed for AufAbschlag {}
impl Bo4eComponent for AufAbschlag {}
impl crate::bo4e_component_sealed::Sealed for AufAbschlag {}
#[cfg(feature = "json")]
impl crate::json::sealed::Sealed for AufAbschlag {}
#[cfg(feature = "json")]
impl crate::json::Bo4eJsonExt for AufAbschlag {}
#[cfg(feature = "json")]
impl crate::json::Bo4eExtensionData for AufAbschlag {
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
impl std::fmt::Display for AufAbschlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => f.write_str(&json),
            Err(e) => write!(f, "<AufAbschlag: serialization error: {e}>"),
        }
    }
}
impl crate::Bo4eStrict for AufAbschlag {
    fn collect_unknown_enums(&self, path: &str, out: &mut Vec<String>) {
        if let Some(v) = &self.auf_abschlagstyp {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "aufAbschlagstyp"),
                out,
            );
        }
        if let Some(v) = &self.auf_abschlagsziel {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "aufAbschlagsziel"),
                out,
            );
        }
        if let Some(v) = &self.einheit {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "einheit"),
                out,
            );
        }
        if let Some(v) = &self.gueltigkeitszeitraum {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "gueltigkeitszeitraum"),
                out,
            );
        }
        if let Some(items) = &self.staffeln {
            let child = crate::strict::field_path(path, "staffeln");
            for (i, item) in items.iter().enumerate() {
                crate::Bo4eStrict::collect_unknown_enums(
                    item,
                    &crate::strict::index_path(&child, i),
                    out,
                );
            }
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
impl crate::json::Bo4eExtensions for AufAbschlag {
    fn collect_extension_paths(&self, path: &str, out: &mut Vec<String>) {
        if let Some(map) = self._additional.as_map() {
            for key in map.keys() {
                out.push(crate::strict::extension_path(path, key));
            }
        }
        if let Some(v) = &self.gueltigkeitszeitraum {
            crate::json::Bo4eExtensions::collect_extension_paths(
                v,
                &crate::strict::field_path(path, "gueltigkeitszeitraum"),
                out,
            );
        }
        if let Some(items) = &self.staffeln {
            let child = crate::strict::field_path(path, "staffeln");
            for (i, item) in items.iter().enumerate() {
                crate::json::Bo4eExtensions::collect_extension_paths(
                    item,
                    &crate::strict::index_path(&child, i),
                    out,
                );
            }
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
