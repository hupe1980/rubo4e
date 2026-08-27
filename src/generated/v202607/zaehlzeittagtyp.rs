use super::{Bo4eComponent, Bo4eTyped, ComTyp, Umschaltzeit, Wiederholungstyp, ZusatzAttribut};
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(not(feature = "json"), derive(Eq, Hash))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "validate", derive(garde::Validate))]
#[cfg_attr(feature = "validate", garde(allow_unvalidated))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Beschreibt das Schaltschema eines Tagtyps innerhalb einer `Zaehlzeitsaison`: welcher Tagtyp
/// gemeint ist und zu welchen Uhrzeiten welches Register an diesem Tagtyp aktiv ist.
///
/// Der Tagtyp wird über einen `Wiederholungstyp` ausgedrückt (z.B. `WERKTAGS`, `MONTAGS`,
/// `FEIERTAGS`).
///
/// Die `umschaltzeiten` füllen einen ganzen Tag vollständig und überlappungsfrei aus.
/// Die jeweilige Umschaltzeit definiert (wenn der Größe nach sortiert) die untere Grenze (inklusiv);
/// der Beginn des Tages (00:00 Uhr) und das Ende des Tages (24:00 Uhr) bilden die äußeren Grenzen.
///
/// > **Note:** [Zaehlzeittagtyp JSON Schema](https://json-schema.app/view/%23?url=https://raw.githubusercontent.com/BO4E/BO4E-Schemas/v202607.1.0/src/bo4e_schemas/com/Zaehlzeittagtyp.json)
pub struct Zaehlzeittagtyp {
    /// Eine generische ID, die für eigene Zwecke genutzt werden kann.
    /// Z.B. könnten hier UUIDs aus einer Datenbank stehen oder URLs zu einem Backend-System.
    #[cfg_attr(feature = "serde", serde(rename = "_id"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub id: Option<String>,
    /// An welchen Tagen das Schaltschema dieses Tagtyps gilt (z.B. `WERKTAGS`, `MONTAGS`, `FEIERTAGS`).
    #[cfg_attr(feature = "serde", serde(rename = "tagtyp"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub tagtyp: Option<Wiederholungstyp>,
    /// BO4E type discriminant — always `ComTyp::Zaehlzeittagtyp` for this struct.
    #[cfg_attr(feature = "serde", serde(rename = "_typ"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(
        feature = "builder",
        builder(default = Some(ComTyp::Zaehlzeittagtyp), setter(skip))
    )]
    pub typ: Option<ComTyp>,
    /// Die Umschaltzeiten dieses Tagtyps. Sortiert ergibt sich daraus das Schaltschema für einen Tag.
    #[cfg_attr(feature = "serde", serde(rename = "umschaltzeiten"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub umschaltzeiten: Option<Vec<Umschaltzeit>>,
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
impl Default for Zaehlzeittagtyp {
    fn default() -> Self {
        Self {
            id: Default::default(),
            tagtyp: Default::default(),
            typ: Some(ComTyp::Zaehlzeittagtyp),
            umschaltzeiten: Default::default(),
            version: Some("202607.1.0".to_owned()),
            zusatz_attribute: Default::default(),
            _additional: Default::default(),
        }
    }
}
impl Bo4eTyped for Zaehlzeittagtyp {
    type Typ = ComTyp;
    const TYP: ComTyp = ComTyp::Zaehlzeittagtyp;
    const TYP_WIRE: &'static str = "ZAEHLZEITTAGTYP";
    const SCHEMA_VERSION: &'static str = "202607.1.0";
    const SCHEMA_SERIES: &'static str = "202607";
}
impl crate::bo4e_typed_sealed::Sealed for Zaehlzeittagtyp {}
impl Bo4eComponent for Zaehlzeittagtyp {}
impl crate::bo4e_component_sealed::Sealed for Zaehlzeittagtyp {}
#[cfg(feature = "json")]
impl crate::json::sealed::Sealed for Zaehlzeittagtyp {}
#[cfg(feature = "json")]
impl crate::json::Bo4eJsonExt for Zaehlzeittagtyp {}
#[cfg(feature = "json")]
impl crate::json::Bo4eExtensionData for Zaehlzeittagtyp {
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
impl std::fmt::Display for Zaehlzeittagtyp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => f.write_str(&json),
            Err(e) => write!(f, "<Zaehlzeittagtyp: serialization error: {e}>"),
        }
    }
}
impl crate::Bo4eStrict for Zaehlzeittagtyp {
    fn collect_unknown_enums(&self, path: &str, out: &mut Vec<String>) {
        if let Some(v) = &self.tagtyp {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "tagtyp"),
                out,
            );
        }
        if let Some(items) = &self.umschaltzeiten {
            let child = crate::strict::field_path(path, "umschaltzeiten");
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
impl crate::json::Bo4eExtensions for Zaehlzeittagtyp {
    fn collect_extension_paths(&self, path: &str, out: &mut Vec<String>) {
        if let Some(map) = self._additional.as_map() {
            for key in map.keys() {
                out.push(crate::strict::extension_path(path, key));
            }
        }
        if let Some(items) = &self.umschaltzeiten {
            let child = crate::strict::field_path(path, "umschaltzeiten");
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
