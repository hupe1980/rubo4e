use super::{
    AufAbschlag, Bo4eObject, Bo4eTyped, BoTyp, Energiemix, Kundentyp, Marktteilnehmer,
    Preisgarantie, Registeranzahl, Sparte, Tarifberechnungsparameter, Tarifeinschraenkung,
    Tarifmerkmal, Tarifpreisposition, Tariftyp, Vertragskonditionen, Zeitraum, ZusatzAttribut,
};
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(not(feature = "json"), derive(Eq, Hash))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "validate", derive(garde::Validate))]
#[cfg_attr(feature = "validate", garde(allow_unvalidated))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Tarifinformation mit Preisen, Aufschlägen und Berechnungssystematik
///
/// > **Note:** [Tarifpreisblatt JSON Schema](https://json-schema.app/view/%23?url=https://raw.githubusercontent.com/BO4E/BO4E-Schemas/v202607.1.0/src/bo4e_schemas/bo/Tarifpreisblatt.json)
pub struct Tarifpreisblatt {
    /// Der Marktteilnehmer (Lieferant), der diesen Tarif anbietet
    #[cfg_attr(feature = "serde", serde(rename = "anbieter"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub anbieter: Option<Box<Marktteilnehmer>>,
    /// Der Name des Marktpartners, der den Tarif anbietet
    #[cfg_attr(feature = "serde", serde(rename = "anbietername"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub anbietername: Option<String>,
    /// Angabe des inklusiven Zeitpunkts, ab dem der Tarif bzw. der Preis angewendet und abgerechnet wird,
    /// z.B. "2021-07-20T18:31:48Z"
    #[cfg_attr(feature = "serde", serde(rename = "anwendungVon"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(
        feature = "schemars",
        schemars(schema_with = "crate::schema_helpers::opt_datetime_schema")
    )]
    #[cfg_attr(
        all(feature = "serde", feature = "time"),
        serde(with = "time::serde::rfc3339::option")
    )]
    #[cfg(feature = "time")]
    pub anwendung_von: Option<time::OffsetDateTime>,
    /// Angabe des inklusiven Zeitpunkts, ab dem der Tarif bzw. der Preis angewendet und abgerechnet wird,
    /// z.B. "2021-07-20T18:31:48Z"
    #[cfg_attr(feature = "serde", serde(rename = "anwendungVon"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(
        feature = "schemars",
        schemars(schema_with = "crate::schema_helpers::opt_datetime_schema")
    )]
    #[cfg(not(feature = "time"))]
    pub anwendung_von: Option<String>,
    /// Freitext
    #[cfg_attr(feature = "serde", serde(rename = "bemerkung"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub bemerkung: Option<String>,
    /// Für die Berechnung der Kosten sind die hier abgebildeten Parameter heranzuziehen
    #[cfg_attr(feature = "serde", serde(rename = "berechnungsparameter"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub berechnungsparameter: Option<Tarifberechnungsparameter>,
    /// Name des Tarifs
    #[cfg_attr(feature = "serde", serde(rename = "bezeichnung"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub bezeichnung: Option<String>,
    /// Der Energiemix, der für diesen Tarif gilt
    #[cfg_attr(feature = "serde", serde(rename = "energiemix"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub energiemix: Option<Energiemix>,
    /// Eine generische ID, die für eigene Zwecke genutzt werden kann.
    /// Z.B. könnten hier UUIDs aus einer Datenbank stehen oder URLs zu einem Backend-System.
    #[cfg_attr(feature = "serde", serde(rename = "_id"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub id: Option<String>,
    /// Kundentypen für den der Tarif gilt, z.B. Privatkunden
    #[cfg_attr(feature = "serde", serde(rename = "kundentypen"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub kundentypen: Option<Vec<Kundentyp>>,
    /// Festlegung von Garantien für bestimmte Preisanteile
    #[cfg_attr(feature = "serde", serde(rename = "preisgarantie"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub preisgarantie: Option<Preisgarantie>,
    /// Gibt an, wann der Preis zuletzt angepasst wurde
    #[cfg_attr(feature = "serde", serde(rename = "preisstand"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(
        feature = "schemars",
        schemars(schema_with = "crate::schema_helpers::opt_datetime_schema")
    )]
    #[cfg_attr(
        all(feature = "serde", feature = "time"),
        serde(with = "time::serde::rfc3339::option")
    )]
    #[cfg(feature = "time")]
    pub preisstand: Option<time::OffsetDateTime>,
    /// Gibt an, wann der Preis zuletzt angepasst wurde
    #[cfg_attr(feature = "serde", serde(rename = "preisstand"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(
        feature = "schemars",
        schemars(schema_with = "crate::schema_helpers::opt_datetime_schema")
    )]
    #[cfg(not(feature = "time"))]
    pub preisstand: Option<String>,
    /// Die Art des Tarifes, z.B. Eintarif oder Mehrtarif
    #[cfg_attr(feature = "serde", serde(rename = "registeranzahl"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub registeranzahl: Option<Registeranzahl>,
    /// Strom oder Gas, etc.
    #[cfg_attr(feature = "serde", serde(rename = "sparte"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub sparte: Option<Sparte>,
    /// Auf- und Abschläge auf die Preise oder Kosten
    #[cfg_attr(feature = "serde", serde(rename = "tarifAufAbschlaege"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub tarif_auf_abschlaege: Option<Vec<AufAbschlag>>,
    /// Die Bedingungen und Einschränkungen unter denen ein Tarif angewendet werden kann
    #[cfg_attr(feature = "serde", serde(rename = "tarifeinschraenkung"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub tarifeinschraenkung: Option<Tarifeinschraenkung>,
    /// Weitere Merkmale des Tarifs, z.B. Festpreis oder Vorkasse
    #[cfg_attr(feature = "serde", serde(rename = "tarifmerkmale"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub tarifmerkmale: Option<Vec<Tarifmerkmal>>,
    /// Die festgelegten Preise, z.B. für Arbeitspreis, Grundpreis etc.
    #[cfg_attr(feature = "serde", serde(rename = "tarifpreise"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub tarifpreise: Option<Vec<Tarifpreisposition>>,
    /// Hinweis auf den Tariftyp, z.B. Grundversorgung oder Sondertarif
    #[cfg_attr(feature = "serde", serde(rename = "tariftyp"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub tariftyp: Option<Tariftyp>,
    /// BO4E type discriminant — always `BoTyp::Tarifpreisblatt` for this struct.
    #[cfg_attr(feature = "serde", serde(rename = "_typ"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(
        feature = "builder",
        builder(default = Some(BoTyp::Tarifpreisblatt), setter(skip))
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
    /// Mindestlaufzeiten und Kündigungsfristen zusammengefasst
    #[cfg_attr(feature = "serde", serde(rename = "vertragskonditionen"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub vertragskonditionen: Option<Vertragskonditionen>,
    /// Internetseite auf dem der Tarif zu finden ist
    #[cfg_attr(feature = "serde", serde(rename = "website"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub website: Option<String>,
    /// Angabe, in welchem Zeitraum der Tarif gültig ist
    #[cfg_attr(feature = "serde", serde(rename = "zeitlicheGueltigkeit"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub zeitliche_gueltigkeit: Option<Zeitraum>,
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
impl Default for Tarifpreisblatt {
    fn default() -> Self {
        Self {
            anbieter: Default::default(),
            anbietername: Default::default(),
            anwendung_von: Default::default(),
            bemerkung: Default::default(),
            berechnungsparameter: Default::default(),
            bezeichnung: Default::default(),
            energiemix: Default::default(),
            id: Default::default(),
            kundentypen: Default::default(),
            preisgarantie: Default::default(),
            preisstand: Default::default(),
            registeranzahl: Default::default(),
            sparte: Default::default(),
            tarif_auf_abschlaege: Default::default(),
            tarifeinschraenkung: Default::default(),
            tarifmerkmale: Default::default(),
            tarifpreise: Default::default(),
            tariftyp: Default::default(),
            typ: Some(BoTyp::Tarifpreisblatt),
            version: Some("202607.1.0".to_owned()),
            vertragskonditionen: Default::default(),
            website: Default::default(),
            zeitliche_gueltigkeit: Default::default(),
            zusatz_attribute: Default::default(),
            _additional: Default::default(),
        }
    }
}
impl Bo4eTyped for Tarifpreisblatt {
    type Typ = BoTyp;
    const TYP: BoTyp = BoTyp::Tarifpreisblatt;
    const TYP_WIRE: &'static str = "TARIFPREISBLATT";
    const SCHEMA_VERSION: &'static str = "202607.1.0";
    const SCHEMA_SERIES: &'static str = "202607";
}
impl crate::bo4e_typed_sealed::Sealed for Tarifpreisblatt {}
impl Bo4eObject for Tarifpreisblatt {}
impl crate::bo4e_object_sealed::Sealed for Tarifpreisblatt {}
#[cfg(feature = "json")]
impl crate::json::sealed::Sealed for Tarifpreisblatt {}
#[cfg(feature = "json")]
impl crate::json::Bo4eJsonExt for Tarifpreisblatt {}
#[cfg(feature = "json")]
impl crate::json::Bo4eExtensionData for Tarifpreisblatt {
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
impl std::fmt::Display for Tarifpreisblatt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => f.write_str(&json),
            Err(e) => write!(f, "<Tarifpreisblatt: serialization error: {e}>"),
        }
    }
}
impl crate::Bo4eStrict for Tarifpreisblatt {
    fn collect_unknown_enums(&self, path: &str, out: &mut Vec<String>) {
        if let Some(v) = &self.anbieter {
            crate::Bo4eStrict::collect_unknown_enums(
                &**v,
                &crate::strict::field_path(path, "anbieter"),
                out,
            );
        }
        if let Some(v) = &self.berechnungsparameter {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "berechnungsparameter"),
                out,
            );
        }
        if let Some(v) = &self.energiemix {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "energiemix"),
                out,
            );
        }
        if let Some(items) = &self.kundentypen {
            let child = crate::strict::field_path(path, "kundentypen");
            for (i, item) in items.iter().enumerate() {
                crate::Bo4eStrict::collect_unknown_enums(
                    item,
                    &crate::strict::index_path(&child, i),
                    out,
                );
            }
        }
        if let Some(v) = &self.preisgarantie {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "preisgarantie"),
                out,
            );
        }
        if let Some(v) = &self.registeranzahl {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "registeranzahl"),
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
        if let Some(items) = &self.tarif_auf_abschlaege {
            let child = crate::strict::field_path(path, "tarifAufAbschlaege");
            for (i, item) in items.iter().enumerate() {
                crate::Bo4eStrict::collect_unknown_enums(
                    item,
                    &crate::strict::index_path(&child, i),
                    out,
                );
            }
        }
        if let Some(v) = &self.tarifeinschraenkung {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "tarifeinschraenkung"),
                out,
            );
        }
        if let Some(items) = &self.tarifmerkmale {
            let child = crate::strict::field_path(path, "tarifmerkmale");
            for (i, item) in items.iter().enumerate() {
                crate::Bo4eStrict::collect_unknown_enums(
                    item,
                    &crate::strict::index_path(&child, i),
                    out,
                );
            }
        }
        if let Some(items) = &self.tarifpreise {
            let child = crate::strict::field_path(path, "tarifpreise");
            for (i, item) in items.iter().enumerate() {
                crate::Bo4eStrict::collect_unknown_enums(
                    item,
                    &crate::strict::index_path(&child, i),
                    out,
                );
            }
        }
        if let Some(v) = &self.tariftyp {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "tariftyp"),
                out,
            );
        }
        if let Some(v) = &self.vertragskonditionen {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "vertragskonditionen"),
                out,
            );
        }
        if let Some(v) = &self.zeitliche_gueltigkeit {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "zeitlicheGueltigkeit"),
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
impl crate::json::Bo4eExtensions for Tarifpreisblatt {
    fn collect_extension_paths(&self, path: &str, out: &mut Vec<String>) {
        if let Some(map) = self._additional.as_map() {
            for key in map.keys() {
                out.push(crate::strict::extension_path(path, key));
            }
        }
        if let Some(v) = &self.anbieter {
            crate::json::Bo4eExtensions::collect_extension_paths(
                &**v,
                &crate::strict::field_path(path, "anbieter"),
                out,
            );
        }
        if let Some(v) = &self.berechnungsparameter {
            crate::json::Bo4eExtensions::collect_extension_paths(
                v,
                &crate::strict::field_path(path, "berechnungsparameter"),
                out,
            );
        }
        if let Some(v) = &self.energiemix {
            crate::json::Bo4eExtensions::collect_extension_paths(
                v,
                &crate::strict::field_path(path, "energiemix"),
                out,
            );
        }
        if let Some(v) = &self.preisgarantie {
            crate::json::Bo4eExtensions::collect_extension_paths(
                v,
                &crate::strict::field_path(path, "preisgarantie"),
                out,
            );
        }
        if let Some(items) = &self.tarif_auf_abschlaege {
            let child = crate::strict::field_path(path, "tarifAufAbschlaege");
            for (i, item) in items.iter().enumerate() {
                crate::json::Bo4eExtensions::collect_extension_paths(
                    item,
                    &crate::strict::index_path(&child, i),
                    out,
                );
            }
        }
        if let Some(v) = &self.tarifeinschraenkung {
            crate::json::Bo4eExtensions::collect_extension_paths(
                v,
                &crate::strict::field_path(path, "tarifeinschraenkung"),
                out,
            );
        }
        if let Some(items) = &self.tarifpreise {
            let child = crate::strict::field_path(path, "tarifpreise");
            for (i, item) in items.iter().enumerate() {
                crate::json::Bo4eExtensions::collect_extension_paths(
                    item,
                    &crate::strict::index_path(&child, i),
                    out,
                );
            }
        }
        if let Some(v) = &self.vertragskonditionen {
            crate::json::Bo4eExtensions::collect_extension_paths(
                v,
                &crate::strict::field_path(path, "vertragskonditionen"),
                out,
            );
        }
        if let Some(v) = &self.zeitliche_gueltigkeit {
            crate::json::Bo4eExtensions::collect_extension_paths(
                v,
                &crate::strict::field_path(path, "zeitlicheGueltigkeit"),
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
impl crate::zusatz_attribut::HasZusatzAttribute for Tarifpreisblatt {
    fn zusatz_attribute_field(&self) -> Option<&Vec<ZusatzAttribut>> {
        self.zusatz_attribute.as_ref()
    }
    fn zusatz_attribute_field_mut(&mut self) -> &mut Option<Vec<ZusatzAttribut>> {
        &mut self.zusatz_attribute
    }
}
