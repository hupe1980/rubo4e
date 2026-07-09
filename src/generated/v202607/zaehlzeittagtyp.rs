use super::{ComTyp, Umschaltzeit, Wiederholungstyp, ZusatzAttribut};
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(not(feature = "json"), derive(Hash))]
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
/// > **Note:** [Zaehlzeittagtyp JSON Schema](https://json-schema.app/view/%23?url=https://raw.githubusercontent.com/BO4E/BO4E-Schemas/v202607.0.0/src/bo4e_schemas/com/Zaehlzeittagtyp.json)
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
    /// COM type identifier for this struct.
    #[cfg_attr(feature = "serde", serde(rename = "_typ"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub typ: Option<ComTyp>,
    /// Die Umschaltzeiten dieses Tagtyps. Sortiert ergibt sich daraus das Schaltschema für einen Tag.
    #[cfg_attr(feature = "serde", serde(rename = "umschaltzeiten"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub umschaltzeiten: Option<Vec<Umschaltzeit>>,
    /// Version der COM-Struktur aka "fachliche Versionierung"
    #[cfg_attr(feature = "serde", serde(rename = "_version"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub version: Option<String>,
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
