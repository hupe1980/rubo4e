use super::{
    Ausschreibungslos, Ausschreibungsportal, Ausschreibungsstatus, Ausschreibungstyp, Bo4eObject,
    Bo4eTyped, BoTyp, Geschaeftspartner, Zeitraum, ZusatzAttribut,
};
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(not(feature = "json"), derive(Eq, Hash))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "validate", derive(garde::Validate))]
#[cfg_attr(feature = "validate", garde(allow_unvalidated))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Das BO Ausschreibung dient zur detaillierten Darstellung von ausgeschriebenen Energiemengen in der Energiewirtschaft
///
/// > **Note:** [Ausschreibung JSON Schema](https://json-schema.app/view/%23?url=https://raw.githubusercontent.com/BO4E/BO4E-Schemas/v202607.1.0/src/bo4e_schemas/bo/Ausschreibung.json)
pub struct Ausschreibung {
    /// Diese Komponente wird zur Abbildung von Zeiträumen in Form von Dauern oder der Angabe von Start und Ende verwendet.
    /// Es muss daher entweder eine Dauer oder ein Zeitraum in Form von Start und Ende angegeben sein
    #[cfg_attr(feature = "serde", serde(rename = "abgabefrist"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub abgabefrist: Option<Zeitraum>,
    /// Mit diesem Objekt können Geschäftspartner übertragen werden.
    /// Sowohl Unternehmen, als auch Privatpersonen können Geschäftspartner sein
    #[cfg_attr(feature = "serde", serde(rename = "ausschreibender"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub ausschreibender: Option<Box<Geschaeftspartner>>,
    /// Aufzählung der unterstützten Ausschreibungsportale
    #[cfg_attr(feature = "serde", serde(rename = "ausschreibungportal"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub ausschreibungportal: Option<Ausschreibungsportal>,
    /// Vom Herausgeber der Ausschreibung vergebene eindeutige Nummer
    #[cfg_attr(feature = "serde", serde(rename = "ausschreibungsnummer"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub ausschreibungsnummer: Option<String>,
    /// Bezeichnungen für die Ausschreibungsphasen
    #[cfg_attr(feature = "serde", serde(rename = "ausschreibungsstatus"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub ausschreibungsstatus: Option<Ausschreibungsstatus>,
    /// Aufzählung für die Typisierung von Ausschreibungen
    #[cfg_attr(feature = "serde", serde(rename = "ausschreibungstyp"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub ausschreibungstyp: Option<Ausschreibungstyp>,
    /// Diese Komponente wird zur Abbildung von Zeiträumen in Form von Dauern oder der Angabe von Start und Ende verwendet.
    /// Es muss daher entweder eine Dauer oder ein Zeitraum in Form von Start und Ende angegeben sein
    #[cfg_attr(feature = "serde", serde(rename = "bindefrist"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub bindefrist: Option<Zeitraum>,
    /// Eine generische ID, die für eigene Zwecke genutzt werden kann.
    /// Z.B. könnten hier UUIDs aus einer Datenbank stehen oder URLs zu einem Backend-System.
    #[cfg_attr(feature = "serde", serde(rename = "_id"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub id: Option<String>,
    /// Kennzeichen, ob die Ausschreibung kostenpflichtig ist
    #[cfg_attr(feature = "serde", serde(rename = "istKostenpflichtig"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub ist_kostenpflichtig: Option<bool>,
    /// Die einzelnen Lose, aus denen sich die Ausschreibung zusammensetzt
    #[cfg_attr(feature = "serde", serde(rename = "lose"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub lose: Option<Vec<Ausschreibungslos>>,
    /// BO4E type discriminant — always `BoTyp::Ausschreibung` for this struct.
    #[cfg_attr(feature = "serde", serde(rename = "_typ"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(
        feature = "builder",
        builder(default = Some(BoTyp::Ausschreibung), setter(skip))
    )]
    pub typ: Option<BoTyp>,
    /// Gibt den Veröffentlichungszeitpunkt der Ausschreibung an
    #[cfg_attr(feature = "serde", serde(rename = "veroeffentlichungszeitpunkt"))]
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
    pub veroeffentlichungszeitpunkt: Option<time::OffsetDateTime>,
    /// Gibt den Veröffentlichungszeitpunkt der Ausschreibung an
    #[cfg_attr(feature = "serde", serde(rename = "veroeffentlichungszeitpunkt"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(
        feature = "schemars",
        schemars(schema_with = "crate::schema_helpers::opt_datetime_schema")
    )]
    #[cfg(not(feature = "time"))]
    pub veroeffentlichungszeitpunkt: Option<String>,
    /// Version der BO-Struktur aka "fachliche Versionierung"
    #[cfg_attr(feature = "serde", serde(rename = "_version"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(
        feature = "builder",
        builder(default = Some("202607.1.0".to_owned()), setter(into))
    )]
    pub version: Option<String>,
    /// Internetseite, auf der die Ausschreibung veröffentlicht wurde (falls vorhanden)
    #[cfg_attr(feature = "serde", serde(rename = "webseite"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub webseite: Option<String>,
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
impl Default for Ausschreibung {
    fn default() -> Self {
        Self {
            abgabefrist: Default::default(),
            ausschreibender: Default::default(),
            ausschreibungportal: Default::default(),
            ausschreibungsnummer: Default::default(),
            ausschreibungsstatus: Default::default(),
            ausschreibungstyp: Default::default(),
            bindefrist: Default::default(),
            id: Default::default(),
            ist_kostenpflichtig: Default::default(),
            lose: Default::default(),
            typ: Some(BoTyp::Ausschreibung),
            veroeffentlichungszeitpunkt: Default::default(),
            version: Some("202607.1.0".to_owned()),
            webseite: Default::default(),
            zusatz_attribute: Default::default(),
            _additional: Default::default(),
        }
    }
}
impl Bo4eTyped for Ausschreibung {
    type Typ = BoTyp;
    const TYP: BoTyp = BoTyp::Ausschreibung;
    const TYP_WIRE: &'static str = "AUSSCHREIBUNG";
    const SCHEMA_VERSION: &'static str = "202607.1.0";
    const SCHEMA_SERIES: &'static str = "202607";
}
impl crate::bo4e_typed_sealed::Sealed for Ausschreibung {}
impl Bo4eObject for Ausschreibung {}
impl crate::bo4e_object_sealed::Sealed for Ausschreibung {}
#[cfg(feature = "json")]
impl crate::json::sealed::Sealed for Ausschreibung {}
#[cfg(feature = "json")]
impl crate::json::Bo4eJsonExt for Ausschreibung {}
#[cfg(feature = "json")]
impl crate::json::Bo4eExtensionData for Ausschreibung {
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
impl std::fmt::Display for Ausschreibung {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => f.write_str(&json),
            Err(e) => write!(f, "<Ausschreibung: serialization error: {e}>"),
        }
    }
}
impl crate::Bo4eStrict for Ausschreibung {
    fn collect_unknown_enums(&self, path: &str, out: &mut Vec<String>) {
        if let Some(v) = &self.abgabefrist {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "abgabefrist"),
                out,
            );
        }
        if let Some(v) = &self.ausschreibender {
            crate::Bo4eStrict::collect_unknown_enums(
                &**v,
                &crate::strict::field_path(path, "ausschreibender"),
                out,
            );
        }
        if let Some(v) = &self.ausschreibungportal {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "ausschreibungportal"),
                out,
            );
        }
        if let Some(v) = &self.ausschreibungsstatus {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "ausschreibungsstatus"),
                out,
            );
        }
        if let Some(v) = &self.ausschreibungstyp {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "ausschreibungstyp"),
                out,
            );
        }
        if let Some(v) = &self.bindefrist {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "bindefrist"),
                out,
            );
        }
        if let Some(items) = &self.lose {
            let child = crate::strict::field_path(path, "lose");
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
impl crate::json::Bo4eExtensions for Ausschreibung {
    fn collect_extension_paths(&self, path: &str, out: &mut Vec<String>) {
        if let Some(map) = self._additional.as_map() {
            for key in map.keys() {
                out.push(crate::strict::extension_path(path, key));
            }
        }
        if let Some(v) = &self.abgabefrist {
            crate::json::Bo4eExtensions::collect_extension_paths(
                v,
                &crate::strict::field_path(path, "abgabefrist"),
                out,
            );
        }
        if let Some(v) = &self.ausschreibender {
            crate::json::Bo4eExtensions::collect_extension_paths(
                &**v,
                &crate::strict::field_path(path, "ausschreibender"),
                out,
            );
        }
        if let Some(v) = &self.bindefrist {
            crate::json::Bo4eExtensions::collect_extension_paths(
                v,
                &crate::strict::field_path(path, "bindefrist"),
                out,
            );
        }
        if let Some(items) = &self.lose {
            let child = crate::strict::field_path(path, "lose");
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
impl crate::zusatz_attribut::HasZusatzAttribute for Ausschreibung {
    fn zusatz_attribute_field(&self) -> Option<&Vec<ZusatzAttribut>> {
        self.zusatz_attribute.as_ref()
    }
    fn zusatz_attribute_field_mut(&mut self) -> &mut Option<Vec<ZusatzAttribut>> {
        &mut self.zusatz_attribute
    }
}
