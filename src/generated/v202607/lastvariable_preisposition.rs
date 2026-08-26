use super::{
    ComTyp, Mengeneinheit, Preisreferenz, Preisstaffel, Tarifkalkulationsmethode, Waehrungseinheit,
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
/// Modelliert eine lastvariable Preisposition.
///
/// > **Note:** [LastvariablePreisposition JSON Schema](https://json-schema.app/view/%23?url=https://raw.githubusercontent.com/BO4E/BO4E-Schemas/v202607.1.0/src/bo4e_schemas/com/LastvariablePreisposition.json)
pub struct LastvariablePreisposition {
    /// Eine (beliebige) Bezeichnung für die Preisposition.
    #[cfg_attr(feature = "serde", serde(rename = "bezeichnung"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub bezeichnung: Option<String>,
    /// Eine generische ID, die für eigene Zwecke genutzt werden kann.
    /// Z.B. könnten hier UUIDs aus einer Datenbank stehen oder URLs zu einem Backend-System.
    #[cfg_attr(feature = "serde", serde(rename = "_id"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub id: Option<String>,
    /// Angabe, für welche Bezugsgröße die Preise in den Preisstaffeln gelten. Z.B. kWh.
    #[cfg_attr(feature = "serde", serde(rename = "preisBezugseinheit"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub preis_bezugseinheit: Option<Mengeneinheit>,
    /// Währungseinheit für die Preise in allen Preisstaffeln, z.B. Euro oder Ct.
    #[cfg_attr(feature = "serde", serde(rename = "preisWaehrungseinheit"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub preis_waehrungseinheit: Option<Waehrungseinheit>,
    /// Die Referenz worauf sich der Preis bezieht.
    /// Die explizite Einheit wird durch das Feld `preis_bezugseinheit` angegeben.
    #[cfg_attr(feature = "serde", serde(rename = "preisreferenz"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub preisreferenz: Option<Preisreferenz>,
    /// Preisstaffeln, die zu dieser Preisposition gehören
    #[cfg_attr(feature = "serde", serde(rename = "preisstaffeln"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub preisstaffeln: Option<Vec<Preisstaffel>>,
    /// Die Einheit, in denen die Staffelgrenzen in den Preisstaffeln angegeben sind.
    #[cfg_attr(feature = "serde", serde(rename = "staffelgrenzeneinheit"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub staffelgrenzeneinheit: Option<Mengeneinheit>,
    /// Das Modell, das der Preisbildung zugrunde liegt
    #[cfg_attr(feature = "serde", serde(rename = "tarifkalkulationsmethode"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub tarifkalkulationsmethode: Option<Tarifkalkulationsmethode>,
    /// BO4E type discriminant — always `ComTyp::LastvariablePreisposition` for this struct.
    #[cfg_attr(feature = "serde", serde(rename = "_typ"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(
        feature = "builder",
        builder(default = Some(ComTyp::LastvariablePreisposition), setter(skip))
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
impl Default for LastvariablePreisposition {
    fn default() -> Self {
        Self {
            bezeichnung: Default::default(),
            id: Default::default(),
            preis_bezugseinheit: Default::default(),
            preis_waehrungseinheit: Default::default(),
            preisreferenz: Default::default(),
            preisstaffeln: Default::default(),
            staffelgrenzeneinheit: Default::default(),
            tarifkalkulationsmethode: Default::default(),
            typ: Some(ComTyp::LastvariablePreisposition),
            version: Some("202607.1.0".to_owned()),
            zusatz_attribute: Default::default(),
            _additional: Default::default(),
        }
    }
}
#[cfg(feature = "json")]
impl crate::json::sealed::Sealed for LastvariablePreisposition {}
#[cfg(feature = "json")]
impl crate::json::Bo4eJsonExt for LastvariablePreisposition {}
#[cfg(feature = "json")]
impl crate::json::Bo4eExtensionData for LastvariablePreisposition {
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
impl std::fmt::Display for LastvariablePreisposition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => f.write_str(&json),
            Err(e) => write!(f, "<LastvariablePreisposition: serialization error: {e}>"),
        }
    }
}
impl crate::Bo4eStrict for LastvariablePreisposition {
    fn collect_unknown_enums(&self, path: &str, out: &mut Vec<String>) {
        if let Some(v) = &self.preis_bezugseinheit {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "preisBezugseinheit"),
                out,
            );
        }
        if let Some(v) = &self.preis_waehrungseinheit {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "preisWaehrungseinheit"),
                out,
            );
        }
        if let Some(v) = &self.preisreferenz {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "preisreferenz"),
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
        if let Some(v) = &self.staffelgrenzeneinheit {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "staffelgrenzeneinheit"),
                out,
            );
        }
        if let Some(v) = &self.tarifkalkulationsmethode {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "tarifkalkulationsmethode"),
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
