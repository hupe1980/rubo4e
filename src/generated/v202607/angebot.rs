use super::{
    Angebotsvariante, Bo4eObject, Bo4eTyped, BoTyp, Geschaeftspartner, Person, Sparte,
    ZusatzAttribut,
};
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(not(feature = "json"), derive(Eq, Hash))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "validate", derive(garde::Validate))]
#[cfg_attr(feature = "validate", garde(allow_unvalidated))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Mit diesem BO kann ein Versorgungsangebot zur Strom- oder Gasversorgung oder die Teilnahme an einer Ausschreibung
/// übertragen werden. Es können verschiedene Varianten enthalten sein (z.B. ein- und mehrjährige Laufzeit).
/// Innerhalb jeder Variante können Teile enthalten sein, die jeweils für eine oder mehrere Marktlokationen erstellt
/// werden.
///
/// > **Note:** [Angebot JSON Schema](https://json-schema.app/view/%23?url=https://raw.githubusercontent.com/BO4E/BO4E-Schemas/v202607.1.0/src/bo4e_schemas/bo/Angebot.json)
pub struct Angebot {
    /// Referenz auf eine Anfrage oder Ausschreibung;
    /// Kann dem Empfänger des Angebotes bei Zuordnung des Angebotes zur Anfrage bzw. Ausschreibung helfen.
    #[cfg_attr(feature = "serde", serde(rename = "anfragereferenz"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub anfragereferenz: Option<String>,
    /// Erstellungsdatum des Angebots
    #[cfg_attr(feature = "serde", serde(rename = "angebotsdatum"))]
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
    pub angebotsdatum: Option<time::OffsetDateTime>,
    /// Requires the `time` feature for the `time::OffsetDateTime` representation.
    /// Without `time`, stores the ISO-8601 string value unchanged.
    #[cfg_attr(feature = "serde", serde(rename = "angebotsdatum"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(
        feature = "schemars",
        schemars(schema_with = "crate::schema_helpers::opt_datetime_schema")
    )]
    #[cfg(not(feature = "time"))]
    pub angebotsdatum: Option<String>,
    /// Ersteller des Angebots
    #[cfg_attr(feature = "serde", serde(rename = "angebotsgeber"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub angebotsgeber: Option<Box<Geschaeftspartner>>,
    /// Empfänger des Angebots
    #[cfg_attr(feature = "serde", serde(rename = "angebotsnehmer"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub angebotsnehmer: Option<Box<Geschaeftspartner>>,
    /// Eindeutige Nummer des Angebotes
    #[cfg_attr(feature = "serde", serde(rename = "angebotsnummer"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub angebotsnummer: Option<String>,
    /// Bis zu diesem Zeitpunkt (Tag/Uhrzeit) inklusive gilt das Angebot
    #[cfg_attr(feature = "serde", serde(rename = "bindefrist"))]
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
    pub bindefrist: Option<time::OffsetDateTime>,
    /// Requires the `time` feature for the `time::OffsetDateTime` representation.
    /// Without `time`, stores the ISO-8601 string value unchanged.
    #[cfg_attr(feature = "serde", serde(rename = "bindefrist"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(
        feature = "schemars",
        schemars(schema_with = "crate::schema_helpers::opt_datetime_schema")
    )]
    #[cfg(not(feature = "time"))]
    pub bindefrist: Option<String>,
    /// Eine generische ID, die für eigene Zwecke genutzt werden kann.
    /// Z.B. könnten hier UUIDs aus einer Datenbank stehen oder URLs zu einem Backend-System.
    #[cfg_attr(feature = "serde", serde(rename = "_id"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub id: Option<String>,
    /// Sparte, für die das Angebot abgegeben wird (Strom/Gas)
    #[cfg_attr(feature = "serde", serde(rename = "sparte"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub sparte: Option<Sparte>,
    /// BO4E type discriminant — always `BoTyp::Angebot` for this struct.
    #[cfg_attr(feature = "serde", serde(rename = "_typ"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(
        feature = "builder",
        builder(default = Some(BoTyp::Angebot), setter(skip))
    )]
    pub typ: Option<BoTyp>,
    /// Person, die als Angebotsgeber das Angebots ausgestellt hat
    #[cfg_attr(feature = "serde", serde(rename = "unterzeichnerAngebotsgeber"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub unterzeichner_angebotsgeber: Option<Box<Person>>,
    /// Person, die als Angebotsnehmer das Angebot angenommen hat
    #[cfg_attr(feature = "serde", serde(rename = "unterzeichnerAngebotsnehmer"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub unterzeichner_angebotsnehmer: Option<Box<Person>>,
    /// Eine oder mehrere Varianten des Angebots mit den Angebotsteilen;
    /// Ein Angebot besteht mindestens aus einer Variante.
    #[cfg_attr(feature = "serde", serde(rename = "varianten"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub varianten: Option<Vec<Angebotsvariante>>,
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
impl Default for Angebot {
    fn default() -> Self {
        Self {
            anfragereferenz: Default::default(),
            angebotsdatum: Default::default(),
            angebotsgeber: Default::default(),
            angebotsnehmer: Default::default(),
            angebotsnummer: Default::default(),
            bindefrist: Default::default(),
            id: Default::default(),
            sparte: Default::default(),
            typ: Some(BoTyp::Angebot),
            unterzeichner_angebotsgeber: Default::default(),
            unterzeichner_angebotsnehmer: Default::default(),
            varianten: Default::default(),
            version: Some("202607.1.0".to_owned()),
            zusatz_attribute: Default::default(),
            _additional: Default::default(),
        }
    }
}
impl Bo4eTyped for Angebot {
    type Typ = BoTyp;
    const TYP: BoTyp = BoTyp::Angebot;
    const TYP_WIRE: &'static str = "ANGEBOT";
    const SCHEMA_VERSION: &'static str = "202607.1.0";
    const SCHEMA_SERIES: &'static str = "202607";
}
impl crate::bo4e_typed_sealed::Sealed for Angebot {}
impl Bo4eObject for Angebot {}
impl crate::bo4e_object_sealed::Sealed for Angebot {}
#[cfg(feature = "json")]
impl crate::json::sealed::Sealed for Angebot {}
#[cfg(feature = "json")]
impl crate::json::Bo4eJsonExt for Angebot {}
#[cfg(feature = "json")]
impl crate::json::Bo4eExtensionData for Angebot {
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
impl std::fmt::Display for Angebot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => f.write_str(&json),
            Err(e) => write!(f, "<Angebot: serialization error: {e}>"),
        }
    }
}
impl crate::Bo4eStrict for Angebot {
    fn collect_unknown_enums(&self, path: &str, out: &mut Vec<String>) {
        if let Some(v) = &self.angebotsgeber {
            crate::Bo4eStrict::collect_unknown_enums(
                &**v,
                &crate::strict::field_path(path, "angebotsgeber"),
                out,
            );
        }
        if let Some(v) = &self.angebotsnehmer {
            crate::Bo4eStrict::collect_unknown_enums(
                &**v,
                &crate::strict::field_path(path, "angebotsnehmer"),
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
        if let Some(v) = &self.unterzeichner_angebotsgeber {
            crate::Bo4eStrict::collect_unknown_enums(
                &**v,
                &crate::strict::field_path(path, "unterzeichnerAngebotsgeber"),
                out,
            );
        }
        if let Some(v) = &self.unterzeichner_angebotsnehmer {
            crate::Bo4eStrict::collect_unknown_enums(
                &**v,
                &crate::strict::field_path(path, "unterzeichnerAngebotsnehmer"),
                out,
            );
        }
        if let Some(items) = &self.varianten {
            let child = crate::strict::field_path(path, "varianten");
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
