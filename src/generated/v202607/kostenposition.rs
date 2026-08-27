use super::{Betrag, Bo4eComponent, Bo4eTyped, ComTyp, Menge, Preis, ZusatzAttribut};
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(not(feature = "json"), derive(Eq, Hash))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "validate", derive(garde::Validate))]
#[cfg_attr(feature = "validate", garde(allow_unvalidated))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(
    all(feature = "validate", feature = "versioned"),
    garde(custom(crate::validation::v202607::validate_kostenposition_arithmetic))
)]
/// Diese Komponente wird zur Übertagung der Details zu einer Kostenposition verwendet.
///
/// > **Note:** [Kostenposition JSON Schema](https://json-schema.app/view/%23?url=https://raw.githubusercontent.com/BO4E/BO4E-Schemas/v202607.1.0/src/bo4e_schemas/com/Kostenposition.json)
pub struct Kostenposition {
    /// Bezeichnung für den Artikel für den die Kosten ermittelt wurden. Beispiel: Arbeitspreis HT
    #[cfg_attr(feature = "serde", serde(rename = "artikelbezeichnung"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub artikelbezeichnung: Option<String>,
    /// Detaillierung des Artikels (optional). Beispiel: 'Drehstromzähler'
    #[cfg_attr(feature = "serde", serde(rename = "artikeldetail"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub artikeldetail: Option<String>,
    /// Der errechnete Gesamtbetrag der Position als Ergebnis der Berechnung <Menge * Einzelpreis> oder
    /// <Einzelpreis / (Anzahl Tage Jahr) * zeitmenge>
    #[cfg_attr(feature = "serde", serde(rename = "betragKostenposition"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub betrag_kostenposition: Option<Betrag>,
    /// exklusiver bis-Zeitpunkt der Kostenzeitscheibe
    #[cfg_attr(feature = "serde", serde(rename = "bis"))]
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
    pub bis: Option<time::OffsetDateTime>,
    /// Requires the `time` feature for the `time::OffsetDateTime` representation.
    /// Without `time`, stores the ISO-8601 string value unchanged.
    #[cfg_attr(feature = "serde", serde(rename = "bis"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(
        feature = "schemars",
        schemars(schema_with = "crate::schema_helpers::opt_datetime_schema")
    )]
    #[cfg(not(feature = "time"))]
    pub bis: Option<String>,
    /// Der Preis für eine Einheit. Beispiele: 5,8200 ct/kWh oder 55 €/Jahr.
    #[cfg_attr(feature = "serde", serde(rename = "einzelpreis"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub einzelpreis: Option<Preis>,
    /// Eine generische ID, die für eigene Zwecke genutzt werden kann.
    /// Z.B. könnten hier UUIDs aus einer Datenbank stehen oder URLs zu einem Backend-System.
    #[cfg_attr(feature = "serde", serde(rename = "_id"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub id: Option<String>,
    /// Die Menge, die in die Kostenberechnung eingeflossen ist. Beispiel: 3.660 kWh
    #[cfg_attr(feature = "serde", serde(rename = "menge"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub menge: Option<Menge>,
    /// Ein Titel für die Zeile. Hier kann z.B. der Netzbetreiber eingetragen werden, wenn es sich um Netzkosten handelt.
    #[cfg_attr(feature = "serde", serde(rename = "positionstitel"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub positionstitel: Option<String>,
    /// BO4E type discriminant — always `ComTyp::Kostenposition` for this struct.
    #[cfg_attr(feature = "serde", serde(rename = "_typ"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(
        feature = "builder",
        builder(default = Some(ComTyp::Kostenposition), setter(skip))
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
    /// inklusiver von-Zeitpunkt der Kostenzeitscheibe
    #[cfg_attr(feature = "serde", serde(rename = "von"))]
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
    pub von: Option<time::OffsetDateTime>,
    /// Requires the `time` feature for the `time::OffsetDateTime` representation.
    /// Without `time`, stores the ISO-8601 string value unchanged.
    #[cfg_attr(feature = "serde", serde(rename = "von"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(
        feature = "schemars",
        schemars(schema_with = "crate::schema_helpers::opt_datetime_schema")
    )]
    #[cfg(not(feature = "time"))]
    pub von: Option<String>,
    /// Wenn es einen zeitbasierten Preis gibt (z.B. €/Jahr), dann ist hier die Menge angegeben mit der die Kosten berechnet
    /// wurden. Z.B. 138 Tage.
    #[cfg_attr(feature = "serde", serde(rename = "zeitmenge"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub zeitmenge: Option<Menge>,
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
impl Default for Kostenposition {
    fn default() -> Self {
        Self {
            artikelbezeichnung: Default::default(),
            artikeldetail: Default::default(),
            betrag_kostenposition: Default::default(),
            bis: Default::default(),
            einzelpreis: Default::default(),
            id: Default::default(),
            menge: Default::default(),
            positionstitel: Default::default(),
            typ: Some(ComTyp::Kostenposition),
            version: Some("202607.1.0".to_owned()),
            von: Default::default(),
            zeitmenge: Default::default(),
            zusatz_attribute: Default::default(),
            _additional: Default::default(),
        }
    }
}
impl Bo4eTyped for Kostenposition {
    type Typ = ComTyp;
    const TYP: ComTyp = ComTyp::Kostenposition;
    const TYP_WIRE: &'static str = "KOSTENPOSITION";
    const SCHEMA_VERSION: &'static str = "202607.1.0";
    const SCHEMA_SERIES: &'static str = "202607";
}
impl crate::bo4e_typed_sealed::Sealed for Kostenposition {}
impl Bo4eComponent for Kostenposition {}
impl crate::bo4e_component_sealed::Sealed for Kostenposition {}
#[cfg(feature = "json")]
impl crate::json::sealed::Sealed for Kostenposition {}
#[cfg(feature = "json")]
impl crate::json::Bo4eJsonExt for Kostenposition {}
#[cfg(feature = "json")]
impl crate::json::Bo4eExtensionData for Kostenposition {
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
impl std::fmt::Display for Kostenposition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => f.write_str(&json),
            Err(e) => write!(f, "<Kostenposition: serialization error: {e}>"),
        }
    }
}
impl crate::Bo4eStrict for Kostenposition {
    fn collect_unknown_enums(&self, path: &str, out: &mut Vec<String>) {
        if let Some(v) = &self.betrag_kostenposition {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "betragKostenposition"),
                out,
            );
        }
        if let Some(v) = &self.einzelpreis {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "einzelpreis"),
                out,
            );
        }
        if let Some(v) = &self.menge {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "menge"),
                out,
            );
        }
        if let Some(v) = &self.zeitmenge {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "zeitmenge"),
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
impl crate::json::Bo4eExtensions for Kostenposition {
    fn collect_extension_paths(&self, path: &str, out: &mut Vec<String>) {
        if let Some(map) = self._additional.as_map() {
            for key in map.keys() {
                out.push(crate::strict::extension_path(path, key));
            }
        }
        if let Some(v) = &self.betrag_kostenposition {
            crate::json::Bo4eExtensions::collect_extension_paths(
                v,
                &crate::strict::field_path(path, "betragKostenposition"),
                out,
            );
        }
        if let Some(v) = &self.einzelpreis {
            crate::json::Bo4eExtensions::collect_extension_paths(
                v,
                &crate::strict::field_path(path, "einzelpreis"),
                out,
            );
        }
        if let Some(v) = &self.menge {
            crate::json::Bo4eExtensions::collect_extension_paths(
                v,
                &crate::strict::field_path(path, "menge"),
                out,
            );
        }
        if let Some(v) = &self.zeitmenge {
            crate::json::Bo4eExtensions::collect_extension_paths(
                v,
                &crate::strict::field_path(path, "zeitmenge"),
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
