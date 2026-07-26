use super::{
    Ausschreibungsdetail, ComTyp, Menge, Preismodell, Rechnungslegung, Sparte, Vertragsform,
    Zeitraum, ZusatzAttribut,
};
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(not(feature = "json"), derive(Hash))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "validate", derive(garde::Validate))]
#[cfg_attr(feature = "validate", garde(allow_unvalidated))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Eine Komponente zur Abbildung einzelner Lose einer Ausschreibung
///
/// > **Note:** [Ausschreibungslos JSON Schema](https://json-schema.app/view/%23?url=https://raw.githubusercontent.com/BO4E/BO4E-Schemas/v202607.0.0/src/bo4e_schemas/com/Ausschreibungslos.json)
pub struct Ausschreibungslos {
    /// Anzahl der Lieferstellen in dieser Ausschreibung
    #[cfg_attr(feature = "serde", serde(rename = "anzahlLieferstellen"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub anzahl_lieferstellen: Option<i64>,
    /// Bemerkung des Kunden zum Los
    #[cfg_attr(feature = "serde", serde(rename = "bemerkung"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub bemerkung: Option<String>,
    /// Name des Lizenzpartners
    #[cfg_attr(feature = "serde", serde(rename = "betreutDurch"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub betreut_durch: Option<String>,
    /// Bezeichnung der Ausschreibung
    #[cfg_attr(feature = "serde", serde(rename = "bezeichnung"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub bezeichnung: Option<String>,
    /// Unterscheidungsmöglichkeiten für die Sparte
    #[cfg_attr(feature = "serde", serde(rename = "energieart"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub energieart: Option<Sparte>,
    /// Gibt den Gesamtjahresverbrauch (z.B. in kWh) aller in diesem Los enthaltenen Lieferstellen an
    #[cfg_attr(feature = "serde", serde(rename = "gesamtMenge"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub gesamt_menge: Option<Menge>,
    /// Eine generische ID, die für eigene Zwecke genutzt werden kann.
    /// Z.B. könnten hier UUIDs aus einer Datenbank stehen oder URLs zu einem Backend-System.
    #[cfg_attr(feature = "serde", serde(rename = "_id"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub id: Option<String>,
    /// Die ausgeschriebenen Lieferstellen
    #[cfg_attr(feature = "serde", serde(rename = "lieferstellen"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub lieferstellen: Option<Vec<Ausschreibungsdetail>>,
    /// Zeitraum, für den die in diesem Los enthaltenen Lieferstellen beliefert werden sollen
    #[cfg_attr(feature = "serde", serde(rename = "lieferzeitraum"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub lieferzeitraum: Option<Zeitraum>,
    /// Laufende Nummer des Loses
    #[cfg_attr(feature = "serde", serde(rename = "losnummer"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub losnummer: Option<String>,
    /// Bezeichnung der Preismodelle in Ausschreibungen für die Energielieferung
    #[cfg_attr(feature = "serde", serde(rename = "preismodell"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub preismodell: Option<Preismodell>,
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
    /// In welchem Intervall die Angebotsabgabe wiederholt werden darf.
    /// Angabe nur gesetzt für die 2. Phase bei öffentlich-rechtlichen Ausschreibungen
    #[cfg_attr(feature = "serde", serde(rename = "wiederholungsintervall"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub wiederholungsintervall: Option<Zeitraum>,
    /// Kundenwunsch zur Kündigungsfrist in der Ausschreibung
    #[cfg_attr(feature = "serde", serde(rename = "wunschKuendingungsfrist"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub wunsch_kuendingungsfrist: Option<Zeitraum>,
    /// Maximalmenge Toleranzband (kWh, %)
    #[cfg_attr(feature = "serde", serde(rename = "wunschMaximalmenge"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub wunsch_maximalmenge: Option<Menge>,
    /// Mindesmenge Toleranzband (kWh, %)
    #[cfg_attr(feature = "serde", serde(rename = "wunschMindestmenge"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub wunsch_mindestmenge: Option<Menge>,
    /// Aufzählung der Möglichkeiten zur Rechnungslegung in Ausschreibungen
    #[cfg_attr(feature = "serde", serde(rename = "wunschRechnungslegung"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub wunsch_rechnungslegung: Option<Rechnungslegung>,
    /// Aufzählung der Möglichkeiten zu Vertragsformen in Ausschreibungen
    #[cfg_attr(feature = "serde", serde(rename = "wunschVertragsform"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub wunsch_vertragsform: Option<Vertragsform>,
    /// Kundenwunsch zum Zahlungsziel in der Ausschreibung
    #[cfg_attr(feature = "serde", serde(rename = "wunschZahlungsziel"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub wunsch_zahlungsziel: Option<Zeitraum>,
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
impl crate::json::sealed::Sealed for Ausschreibungslos {}
#[cfg(feature = "json")]
impl crate::json::Bo4eJsonExt for Ausschreibungslos {}
#[cfg(feature = "json")]
impl crate::json::Bo4eExtensionData for Ausschreibungslos {
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
impl std::fmt::Display for Ausschreibungslos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => f.write_str(&json),
            Err(e) => write!(f, "<Ausschreibungslos: serialization error: {e}>"),
        }
    }
}
impl crate::Bo4eStrict for Ausschreibungslos {
    fn collect_unknown_enums(&self, path: &str, out: &mut Vec<String>) {
        if let Some(v) = &self.energieart {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "energieart"),
                out,
            );
        }
        if let Some(v) = &self.gesamt_menge {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "gesamtMenge"),
                out,
            );
        }
        if let Some(items) = &self.lieferstellen {
            let child = crate::strict::field_path(path, "lieferstellen");
            for (i, item) in items.iter().enumerate() {
                crate::Bo4eStrict::collect_unknown_enums(
                    item,
                    &crate::strict::index_path(&child, i),
                    out,
                );
            }
        }
        if let Some(v) = &self.lieferzeitraum {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "lieferzeitraum"),
                out,
            );
        }
        if let Some(v) = &self.preismodell {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "preismodell"),
                out,
            );
        }
        if let Some(v) = &self.wiederholungsintervall {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "wiederholungsintervall"),
                out,
            );
        }
        if let Some(v) = &self.wunsch_kuendingungsfrist {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "wunschKuendingungsfrist"),
                out,
            );
        }
        if let Some(v) = &self.wunsch_maximalmenge {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "wunschMaximalmenge"),
                out,
            );
        }
        if let Some(v) = &self.wunsch_mindestmenge {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "wunschMindestmenge"),
                out,
            );
        }
        if let Some(v) = &self.wunsch_rechnungslegung {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "wunschRechnungslegung"),
                out,
            );
        }
        if let Some(v) = &self.wunsch_vertragsform {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "wunschVertragsform"),
                out,
            );
        }
        if let Some(v) = &self.wunsch_zahlungsziel {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "wunschZahlungsziel"),
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
