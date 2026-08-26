use super::{Adresse, ComTyp, Menge, Zaehlertyp, Zeitraum, ZusatzAttribut};
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(not(feature = "json"), derive(Eq, Hash))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "validate", derive(garde::Validate))]
#[cfg_attr(feature = "validate", garde(allow_unvalidated))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Die Komponente Ausschreibungsdetail wird verwendet um die Informationen zu einer Abnahmestelle innerhalb eines
/// Ausschreibungsloses abzubilden.
///
/// > **Note:** [Ausschreibungsdetail JSON Schema](https://json-schema.app/view/%23?url=https://raw.githubusercontent.com/BO4E/BO4E-Schemas/v202607.1.0/src/bo4e_schemas/com/Ausschreibungsdetail.json)
pub struct Ausschreibungsdetail {
    /// Eine generische ID, die für eigene Zwecke genutzt werden kann.
    /// Z.B. könnten hier UUIDs aus einer Datenbank stehen oder URLs zu einem Backend-System.
    #[cfg_attr(feature = "serde", serde(rename = "_id"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub id: Option<String>,
    /// Zeigt an, ob es zu der Marktlokation einen Lastgang gibt.
    /// Falls ja, kann dieser abgerufen werden und daraus die Verbrauchswerte ermittelt werden
    #[cfg_attr(feature = "serde", serde(rename = "istLastgangVorhanden"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub ist_lastgang_vorhanden: Option<bool>,
    /// Bezeichnung des Kunden, der die Marktlokation nutzt
    #[cfg_attr(feature = "serde", serde(rename = "kunde"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub kunde: Option<String>,
    /// Angefragter Zeitraum für die ausgeschriebene Belieferung
    #[cfg_attr(feature = "serde", serde(rename = "lieferzeitraum"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub lieferzeitraum: Option<Zeitraum>,
    /// Identifikation einer ausgeschriebenen Marktlokation
    #[cfg_attr(feature = "serde", serde(rename = "marktlokationsId"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub marktlokations_id: Option<crate::identifiers::MaloId>,
    /// Die Adresse an der die Marktlokation sich befindet
    #[cfg_attr(feature = "serde", serde(rename = "marktlokationsadresse"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub marktlokationsadresse: Option<Adresse>,
    /// Bezeichnung für die Lokation, z.B. 'Zentraler Einkauf, Hamburg'
    #[cfg_attr(feature = "serde", serde(rename = "marktlokationsbezeichnung"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub marktlokationsbezeichnung: Option<String>,
    /// Bezeichnung des zuständigen Netzbetreibers, z.B. 'Stromnetz Hamburg GmbH'
    #[cfg_attr(feature = "serde", serde(rename = "netzbetreiber"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub netzbetreiber: Option<String>,
    /// In der angegebenen Netzebene wird die Marktlokation versorgt, z.B. MSP für Mittelspannung
    #[cfg_attr(feature = "serde", serde(rename = "netzebeneLieferung"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub netzebene_lieferung: Option<String>,
    /// In der angegebenen Netzebene wird die Lokation gemessen, z.B. NSP für Niederspannung
    #[cfg_attr(feature = "serde", serde(rename = "netzebeneMessung"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub netzebene_messung: Option<String>,
    /// Ein Prognosewert für die Arbeit innerhalb des angefragten Lieferzeitraums der ausgeschriebenen Lokation
    #[cfg_attr(feature = "serde", serde(rename = "prognoseArbeitLieferzeitraum"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub prognose_arbeit_lieferzeitraum: Option<Menge>,
    /// Prognosewert für die Jahresarbeit der ausgeschriebenen Lokation
    #[cfg_attr(feature = "serde", serde(rename = "prognoseJahresarbeit"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub prognose_jahresarbeit: Option<Menge>,
    /// Prognosewert für die abgenommene maximale Leistung der ausgeschriebenen Lokation
    #[cfg_attr(feature = "serde", serde(rename = "prognoseLeistung"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub prognose_leistung: Option<Menge>,
    /// Die (evtl. abweichende) Rechnungsadresse
    #[cfg_attr(feature = "serde", serde(rename = "rechnungsadresse"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub rechnungsadresse: Option<Adresse>,
    /// BO4E type discriminant — always `ComTyp::Ausschreibungsdetail` for this struct.
    #[cfg_attr(feature = "serde", serde(rename = "_typ"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(
        feature = "builder",
        builder(default = Some(ComTyp::Ausschreibungsdetail), setter(skip))
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
    /// Die Bezeichnung des Zählers an der Marktlokation
    #[cfg_attr(feature = "serde", serde(rename = "zaehlernummer"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub zaehlernummer: Option<String>,
    /// Spezifikation, um welche Zählertechnik es sich im vorliegenden Fall handelt, z.B. Leistungsmessung
    #[cfg_attr(feature = "serde", serde(rename = "zaehlertechnik"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub zaehlertechnik: Option<Zaehlertyp>,
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
impl Default for Ausschreibungsdetail {
    fn default() -> Self {
        Self {
            id: Default::default(),
            ist_lastgang_vorhanden: Default::default(),
            kunde: Default::default(),
            lieferzeitraum: Default::default(),
            marktlokations_id: Default::default(),
            marktlokationsadresse: Default::default(),
            marktlokationsbezeichnung: Default::default(),
            netzbetreiber: Default::default(),
            netzebene_lieferung: Default::default(),
            netzebene_messung: Default::default(),
            prognose_arbeit_lieferzeitraum: Default::default(),
            prognose_jahresarbeit: Default::default(),
            prognose_leistung: Default::default(),
            rechnungsadresse: Default::default(),
            typ: Some(ComTyp::Ausschreibungsdetail),
            version: Some("202607.1.0".to_owned()),
            zaehlernummer: Default::default(),
            zaehlertechnik: Default::default(),
            zusatz_attribute: Default::default(),
            _additional: Default::default(),
        }
    }
}
#[cfg(feature = "json")]
impl crate::json::sealed::Sealed for Ausschreibungsdetail {}
#[cfg(feature = "json")]
impl crate::json::Bo4eJsonExt for Ausschreibungsdetail {}
#[cfg(feature = "json")]
impl crate::json::Bo4eExtensionData for Ausschreibungsdetail {
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
impl std::fmt::Display for Ausschreibungsdetail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => f.write_str(&json),
            Err(e) => write!(f, "<Ausschreibungsdetail: serialization error: {e}>"),
        }
    }
}
impl crate::Bo4eStrict for Ausschreibungsdetail {
    fn collect_unknown_enums(&self, path: &str, out: &mut Vec<String>) {
        if let Some(v) = &self.lieferzeitraum {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "lieferzeitraum"),
                out,
            );
        }
        if let Some(v) = &self.marktlokationsadresse {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "marktlokationsadresse"),
                out,
            );
        }
        if let Some(v) = &self.prognose_arbeit_lieferzeitraum {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "prognoseArbeitLieferzeitraum"),
                out,
            );
        }
        if let Some(v) = &self.prognose_jahresarbeit {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "prognoseJahresarbeit"),
                out,
            );
        }
        if let Some(v) = &self.prognose_leistung {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "prognoseLeistung"),
                out,
            );
        }
        if let Some(v) = &self.rechnungsadresse {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "rechnungsadresse"),
                out,
            );
        }
        if let Some(v) = &self.zaehlertechnik {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "zaehlertechnik"),
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
