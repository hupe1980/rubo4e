use super::{Angebotsposition, Betrag, ComTyp, Marktlokation, Menge, Zeitraum, ZusatzAttribut};
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(not(feature = "json"), derive(Eq, Hash))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "validate", derive(garde::Validate))]
#[cfg_attr(feature = "validate", garde(allow_unvalidated))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Mit dieser Komponente wird ein Teil einer Angebotsvariante abgebildet.
/// Hier werden alle Angebotspositionen aggregiert.
/// Angebotsteile werden im einfachsten Fall für eine Marktlokation oder Lieferstellenadresse erzeugt.
/// Hier werden die Mengen und Gesamtkosten aller Angebotspositionen zusammengefasst.
/// Eine Variante besteht mindestens aus einem Angebotsteil.
///
/// > **Note:** [Angebotsteil JSON Schema](https://json-schema.app/view/%23?url=https://raw.githubusercontent.com/BO4E/BO4E-Schemas/v202607.1.0/src/bo4e_schemas/com/Angebotsteil.json)
pub struct Angebotsteil {
    /// Identifizierung eines Subkapitels einer Anfrage, beispielsweise das Los einer Ausschreibung
    #[cfg_attr(feature = "serde", serde(rename = "anfrageSubreferenz"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub anfrage_subreferenz: Option<String>,
    /// Summe der Jahresenergiekosten aller in diesem Angebotsteil enthaltenen Lieferstellen
    #[cfg_attr(feature = "serde", serde(rename = "gesamtkostenangebotsteil"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub gesamtkostenangebotsteil: Option<Betrag>,
    /// Summe der Verbräuche aller in diesem Angebotsteil eingeschlossenen Lieferstellen
    #[cfg_attr(feature = "serde", serde(rename = "gesamtmengeangebotsteil"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub gesamtmengeangebotsteil: Option<Menge>,
    /// Eine generische ID, die für eigene Zwecke genutzt werden kann.
    /// Z.B. könnten hier UUIDs aus einer Datenbank stehen oder URLs zu einem Backend-System.
    #[cfg_attr(feature = "serde", serde(rename = "_id"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub id: Option<String>,
    /// Marktlokationen, für die dieses Angebotsteil gilt, falls vorhanden.
    /// Durch die Marktlokation ist auch die Lieferadresse festgelegt
    #[cfg_attr(feature = "serde", serde(rename = "lieferstellenangebotsteil"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub lieferstellenangebotsteil: Option<Vec<Box<Marktlokation>>>,
    /// Hier kann der Belieferungszeitraum angegeben werden, für den dieser Angebotsteil gilt
    #[cfg_attr(feature = "serde", serde(rename = "lieferzeitraum"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub lieferzeitraum: Option<Zeitraum>,
    /// Einzelne Positionen, die zu diesem Angebotsteil gehören
    #[cfg_attr(feature = "serde", serde(rename = "positionen"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub positionen: Option<Vec<Angebotsposition>>,
    /// BO4E type discriminant — always `ComTyp::Angebotsteil` for this struct.
    #[cfg_attr(feature = "serde", serde(rename = "_typ"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(
        feature = "builder",
        builder(default = Some(ComTyp::Angebotsteil), setter(skip))
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
impl Default for Angebotsteil {
    fn default() -> Self {
        Self {
            anfrage_subreferenz: Default::default(),
            gesamtkostenangebotsteil: Default::default(),
            gesamtmengeangebotsteil: Default::default(),
            id: Default::default(),
            lieferstellenangebotsteil: Default::default(),
            lieferzeitraum: Default::default(),
            positionen: Default::default(),
            typ: Some(ComTyp::Angebotsteil),
            version: Some("202607.1.0".to_owned()),
            zusatz_attribute: Default::default(),
            _additional: Default::default(),
        }
    }
}
#[cfg(feature = "json")]
impl crate::json::sealed::Sealed for Angebotsteil {}
#[cfg(feature = "json")]
impl crate::json::Bo4eJsonExt for Angebotsteil {}
#[cfg(feature = "json")]
impl crate::json::Bo4eExtensionData for Angebotsteil {
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
impl std::fmt::Display for Angebotsteil {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => f.write_str(&json),
            Err(e) => write!(f, "<Angebotsteil: serialization error: {e}>"),
        }
    }
}
impl crate::Bo4eStrict for Angebotsteil {
    fn collect_unknown_enums(&self, path: &str, out: &mut Vec<String>) {
        if let Some(v) = &self.gesamtkostenangebotsteil {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "gesamtkostenangebotsteil"),
                out,
            );
        }
        if let Some(v) = &self.gesamtmengeangebotsteil {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "gesamtmengeangebotsteil"),
                out,
            );
        }
        if let Some(items) = &self.lieferstellenangebotsteil {
            let child = crate::strict::field_path(path, "lieferstellenangebotsteil");
            for (i, item) in items.iter().enumerate() {
                crate::Bo4eStrict::collect_unknown_enums(
                    &**item,
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
        if let Some(items) = &self.positionen {
            let child = crate::strict::field_path(path, "positionen");
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
