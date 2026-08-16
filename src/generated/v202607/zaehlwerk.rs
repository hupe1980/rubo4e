use super::{
    ComTyp, Energierichtung, Konzessionsabgabe, Mengeneinheit, Messwert, Verbrauchsart,
    VerwendungszweckProMarktrolle, Waermenutzung, Zaehlzeitregister, ZusatzAttribut,
};
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(not(feature = "json"), derive(Hash))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "validate", derive(garde::Validate))]
#[cfg_attr(feature = "validate", garde(allow_unvalidated))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Mit dieser Komponente werden Zählwerke modelliert.
///
/// > **Note:** [Zaehlwerk JSON Schema](https://json-schema.app/view/%23?url=https://raw.githubusercontent.com/BO4E/BO4E-Schemas/v202607.0.0/src/bo4e_schemas/com/Zaehlwerk.json)
pub struct Zaehlwerk {
    /// Anzahl Ablesungen pro Jahr
    #[cfg_attr(feature = "serde", serde(rename = "anzahlAblesungen"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub anzahl_ablesungen: Option<i64>,
    /// Zusätzliche Bezeichnung, z.B. Zählwerk_Wirkarbeit.
    #[cfg_attr(feature = "serde", serde(rename = "bezeichnung"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub bezeichnung: Option<String>,
    /// Die Einheit der gemessenen Größe, z.B. kWh
    #[cfg_attr(feature = "serde", serde(rename = "einheit"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub einheit: Option<Mengeneinheit>,
    /// Eine generische ID, die für eigene Zwecke genutzt werden kann.
    /// Z.B. könnten hier UUIDs aus einer Datenbank stehen oder URLs zu einem Backend-System.
    #[cfg_attr(feature = "serde", serde(rename = "_id"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub id: Option<String>,
    /// Abrechnungsrelevant
    #[cfg_attr(feature = "serde", serde(rename = "istAbrechnungsrelevant"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub ist_abrechnungsrelevant: Option<bool>,
    /// Schwachlastfaehigkeit
    #[cfg_attr(feature = "serde", serde(rename = "istSchwachlastfaehig"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub ist_schwachlastfaehig: Option<bool>,
    /// Steuerbefreiung
    #[cfg_attr(feature = "serde", serde(rename = "istSteuerbefreit"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub ist_steuerbefreit: Option<bool>,
    /// Konzessionsabgabe
    #[cfg_attr(feature = "serde", serde(rename = "konzessionsabgabe"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub konzessionsabgabe: Option<Konzessionsabgabe>,
    /// Gemessene Werte des Zählwerks
    #[cfg_attr(feature = "serde", serde(rename = "messwerte"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub messwerte: Option<Vec<Messwert>>,
    /// Anzahl der Nachkommastellen
    #[cfg_attr(feature = "serde", serde(rename = "nachkommastelle"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub nachkommastelle: Option<i64>,
    /// Die OBIS-Kennzahl für das Zählwerk, die festlegt, welche auf die gemessene Größe mit dem Stand gemeldet wird.
    /// Nur Zählwerkstände mit dieser OBIS-Kennzahl werden an diesem Zählwerk registriert.
    #[cfg_attr(feature = "serde", serde(rename = "obisKennzahl"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub obis_kennzahl: Option<crate::identifiers::ObisCode>,
    /// Die Energierichtung, Einspeisung oder Ausspeisung.
    #[cfg_attr(feature = "serde", serde(rename = "richtung"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub richtung: Option<Energierichtung>,
    /// COM type identifier for this struct.
    #[cfg_attr(feature = "serde", serde(rename = "_typ"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub typ: Option<ComTyp>,
    /// Stromverbrauchsart/Verbrauchsart Marktlokation
    #[cfg_attr(feature = "serde", serde(rename = "verbrauchsart"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub verbrauchsart: Option<Verbrauchsart>,
    /// Version der COM-Struktur aka "fachliche Versionierung"
    #[cfg_attr(feature = "serde", serde(rename = "_version"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(
        feature = "builder",
        builder(default = Some("v202607.0.0".to_owned()), setter(into))
    )]
    pub version: Option<String>,
    /// Verwendungungszweck der Werte Marktlokation
    #[cfg_attr(feature = "serde", serde(rename = "verwendungszwecke"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub verwendungszwecke: Option<Vec<VerwendungszweckProMarktrolle>>,
    /// Anzahl der Vorkommastellen
    #[cfg_attr(feature = "serde", serde(rename = "vorkommastelle"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub vorkommastelle: Option<i64>,
    /// Wärmenutzung Marktlokation
    #[cfg_attr(feature = "serde", serde(rename = "waermenutzung"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub waermenutzung: Option<Waermenutzung>,
    /// Mit diesem Faktor wird eine Zählerstandsdifferenz multipliziert, um zum eigentlichen Verbrauch im Zeitraum
    /// zu kommen.
    #[cfg_attr(feature = "serde", serde(rename = "wandlerfaktor"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg(feature = "decimal")]
    pub wandlerfaktor: Option<rust_decimal::Decimal>,
    /// Requires the `decimal` feature for the `rust_decimal::Decimal` representation.
    /// Without `decimal`, stores the decimal string value unchanged.
    #[cfg_attr(feature = "serde", serde(rename = "wandlerfaktor"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg(not(feature = "decimal"))]
    pub wandlerfaktor: Option<String>,
    /// Identifikation des Zählwerks (Registers) innerhalb des Zählers.
    /// Oftmals eine laufende Nummer hinter der Zählernummer. Z.B. 47110815_1
    #[cfg_attr(feature = "serde", serde(rename = "zaehlwerkId"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub zaehlwerk_id: Option<String>,
    /// Erweiterte Definition der Zählzeit in Bezug auf ein Register
    #[cfg_attr(feature = "serde", serde(rename = "zaehlzeitregister"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub zaehlzeitregister: Option<Zaehlzeitregister>,
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
impl Default for Zaehlwerk {
    fn default() -> Self {
        Self {
            anzahl_ablesungen: Default::default(),
            bezeichnung: Default::default(),
            einheit: Default::default(),
            id: Default::default(),
            ist_abrechnungsrelevant: Default::default(),
            ist_schwachlastfaehig: Default::default(),
            ist_steuerbefreit: Default::default(),
            konzessionsabgabe: Default::default(),
            messwerte: Default::default(),
            nachkommastelle: Default::default(),
            obis_kennzahl: Default::default(),
            richtung: Default::default(),
            typ: Default::default(),
            verbrauchsart: Default::default(),
            version: Some("v202607.0.0".to_owned()),
            verwendungszwecke: Default::default(),
            vorkommastelle: Default::default(),
            waermenutzung: Default::default(),
            wandlerfaktor: Default::default(),
            zaehlwerk_id: Default::default(),
            zaehlzeitregister: Default::default(),
            zusatz_attribute: Default::default(),
            _additional: Default::default(),
        }
    }
}
#[cfg(feature = "json")]
impl crate::json::sealed::Sealed for Zaehlwerk {}
#[cfg(feature = "json")]
impl crate::json::Bo4eJsonExt for Zaehlwerk {}
#[cfg(feature = "json")]
impl crate::json::Bo4eExtensionData for Zaehlwerk {
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
impl std::fmt::Display for Zaehlwerk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => f.write_str(&json),
            Err(e) => write!(f, "<Zaehlwerk: serialization error: {e}>"),
        }
    }
}
impl crate::Bo4eStrict for Zaehlwerk {
    fn collect_unknown_enums(&self, path: &str, out: &mut Vec<String>) {
        if let Some(v) = &self.einheit {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "einheit"),
                out,
            );
        }
        if let Some(v) = &self.konzessionsabgabe {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "konzessionsabgabe"),
                out,
            );
        }
        if let Some(items) = &self.messwerte {
            let child = crate::strict::field_path(path, "messwerte");
            for (i, item) in items.iter().enumerate() {
                crate::Bo4eStrict::collect_unknown_enums(
                    item,
                    &crate::strict::index_path(&child, i),
                    out,
                );
            }
        }
        if let Some(v) = &self.richtung {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "richtung"),
                out,
            );
        }
        if let Some(v) = &self.verbrauchsart {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "verbrauchsart"),
                out,
            );
        }
        if let Some(items) = &self.verwendungszwecke {
            let child = crate::strict::field_path(path, "verwendungszwecke");
            for (i, item) in items.iter().enumerate() {
                crate::Bo4eStrict::collect_unknown_enums(
                    item,
                    &crate::strict::index_path(&child, i),
                    out,
                );
            }
        }
        if let Some(v) = &self.waermenutzung {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "waermenutzung"),
                out,
            );
        }
        if let Some(v) = &self.zaehlzeitregister {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "zaehlzeitregister"),
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
