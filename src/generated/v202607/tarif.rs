use super::{
    Bo4eObject, Bo4eTyped, BoTyp, Energiemix, Kundentyp, Marktteilnehmer, Preisgarantie,
    Regionspreis, Registeranzahl, Sparte, Tarifberechnungsparameter, Tarifeinschraenkung,
    Tarifmerkmal, Tariftyp, Vertragskonditionen, Zeitraum, ZusatzAttribut,
};
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(not(feature = "json"), derive(Eq, Hash))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "validate", derive(garde::Validate))]
#[cfg_attr(feature = "validate", garde(allow_unvalidated))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Abbildung eines Tarifs.
///
/// Der Tarifpreis kann regionsaufgelöst und unter Angabe von Zeitscheiben angegeben werden. So kann bspw. derselbe
/// Tarif je nach Region andere Preise aufweisen. Es können auch Tarifpreise abgebildet werden, die sich ab einem
/// bestimmten Zeitpunkt auf andere Regionen ausweiten, da die Regionen ebenfalls mit Zeitscheiben versehen sind.
///
/// Ein Tarifpreis setzt sich dabei aus mehreren Preispositionen zusammen. So können z.B. auch mit
/// `COM RelativePreisposition` prozentuale Auf- und Abschläge auf andere Preispositionen definiert werden.
/// Alle Preispositionen hängen unter `COM Tarifpreiszeitscheibe` mit einer Ausnahme.
///
/// Möchten Sie einen dynamischen Tarif modellieren, so gibt es das `COM DynamischePreisposition`. Da diese
/// Preisposition weder orts- noch zeitabhängig ist, hängt diese direkt unter dem `BO Tarif`. Eine zeitabhängige
/// Änderung einer dynamischen Tarifpreisposition ist unsinnig, da es sich (unserer Ansicht nach) dann um einen
/// völlig neuen Tarif handelt. Davon unabhängig können (und müssen) natürlich weiterhin zusätzlich alle anderen
/// Preispositionen orts- und zeitabhängig angegeben werden.
///
/// > Hinweis: Das Vorhandensein einer `COM DynamischePreisposition` dient gleichzeitig auch als "Flag" dafür, ob
/// > es sich bei diesem Tarif um einen dynamischen handelt.
///
/// > **Note:** [Tarif JSON Schema](https://json-schema.app/view/%23?url=https://raw.githubusercontent.com/BO4E/BO4E-Schemas/v202607.1.0/src/bo4e_schemas/bo/Tarif.json)
pub struct Tarif {
    /// Der Marktteilnehmer, der diesen Tarif anbietet, angeboten hat oder anbieten wird.
    #[cfg_attr(feature = "serde", serde(rename = "anbieter"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub anbieter: Option<Box<Marktteilnehmer>>,
    /// Der Name des Marktpartners, der den Tarif anbietet
    #[cfg_attr(feature = "serde", serde(rename = "anbietername"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub anbietername: Option<String>,
    /// Angabe des inklusiven Zeitpunkts, ab dem der Tarif bzw. der Preis angewendet und abgerechnet wird,
    /// z.B. "2021-07-20T18:31:48Z"
    #[cfg_attr(feature = "serde", serde(rename = "anwendungVon"))]
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
    pub anwendung_von: Option<time::OffsetDateTime>,
    /// Angabe des inklusiven Zeitpunkts, ab dem der Tarif bzw. der Preis angewendet und abgerechnet wird,
    /// z.B. "2021-07-20T18:31:48Z"
    #[cfg_attr(feature = "serde", serde(rename = "anwendungVon"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(
        feature = "schemars",
        schemars(schema_with = "crate::schema_helpers::opt_datetime_schema")
    )]
    #[cfg(not(feature = "time"))]
    pub anwendung_von: Option<String>,
    /// Freitext
    #[cfg_attr(feature = "serde", serde(rename = "bemerkung"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub bemerkung: Option<String>,
    /// Für die Berechnung der Kosten sind die hier abgebildeten Parameter heranzuziehen
    #[cfg_attr(feature = "serde", serde(rename = "berechnungsparameter"))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub berechnungsparameter: Tarifberechnungsparameter,
    /// Eine (beliebige) Beschreibung für den Tarif.
    #[cfg_attr(feature = "serde", serde(rename = "beschreibung"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub beschreibung: Option<String>,
    /// Eine (beliebige) Bezeichnung für den Tarif.
    #[cfg_attr(feature = "serde", serde(rename = "bezeichnung"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub bezeichnung: Option<String>,
    /// Gibt die Bezugsquelle (z.B. Börsenindex) für den dynamischen Tarif an.
    /// Dieses Feld muss genau dann gesetzt werden, wenn es sich bei diesem Tarif um einen dynamischen Tarif handelt.
    #[cfg_attr(feature = "serde", serde(rename = "dynamischePreispositionQuelle"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub dynamische_preisposition_quelle: Option<String>,
    /// Der Energiemix mit einem Eintrag pro Gültigkeitsjahr (siehe `Energiemix.gueltigkeitsjahr`).
    #[cfg_attr(feature = "serde", serde(rename = "energiemix"))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub energiemix: Vec<Energiemix>,
    /// Eine generische ID, die für eigene Zwecke genutzt werden kann.
    /// Z.B. könnten hier UUIDs aus einer Datenbank stehen oder URLs zu einem Backend-System.
    #[cfg_attr(feature = "serde", serde(rename = "_id"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub id: Option<String>,
    /// Eine Liste an Kundentypen, für die dieser Tarif vorgesehen ist.
    #[cfg_attr(feature = "serde", serde(rename = "kundentypen"))]
    pub kundentypen: Vec<Kundentyp>,
    /// Preisgarantie für diesen Tarif
    #[cfg_attr(feature = "serde", serde(rename = "preisgarantie"))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub preisgarantie: Preisgarantie,
    /// Enthält alle regions- und zeitaufgelösten Tarifpreise.
    /// Ausschließlich die `COM DynamischePreisposition` wird unter einem anderen Feld namens `dynamischePreisposition`
    /// angegeben.
    #[cfg_attr(feature = "serde", serde(rename = "regionspreise"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub regionspreise: Option<Vec<Regionspreis>>,
    /// Hinweis zu den Registern bzw. Zählwerken.
    /// Bspw. benötigt ein HT-/NT-Tarif auch eine entsprechende Registeranzahl.
    #[cfg_attr(feature = "serde", serde(rename = "registeranzahl"))]
    pub registeranzahl: Registeranzahl,
    /// Strom / Gas
    #[cfg_attr(feature = "serde", serde(rename = "sparte"))]
    pub sparte: Sparte,
    /// Die Bedingungen und Einschränkungen unter denen ein Tarif angewendet werden kann
    #[cfg_attr(feature = "serde", serde(rename = "tarifeinschraenkung"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub tarifeinschraenkung: Option<Tarifeinschraenkung>,
    /// Eine Liste von Produktmerkmalen im Zusammenhang mit diesem Tarif.
    #[cfg_attr(feature = "serde", serde(rename = "tarifmerkmale"))]
    pub tarifmerkmale: Vec<Tarifmerkmal>,
    /// Der Tariftyp. Bsp.: Grundversorgung, Ersatzversorgung, etc.
    #[cfg_attr(feature = "serde", serde(rename = "tariftyp"))]
    pub tariftyp: Tariftyp,
    /// BO4E type discriminant — always `BoTyp::Tarif` for this struct.
    #[cfg_attr(feature = "serde", serde(rename = "_typ"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default = Some(BoTyp::Tarif), setter(skip)))]
    pub typ: Option<BoTyp>,
    /// Version der BO-Struktur aka "fachliche Versionierung"
    #[cfg_attr(feature = "serde", serde(rename = "_version"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(
        feature = "builder",
        builder(default = Some("202607.1.0".to_owned()), setter(into))
    )]
    pub version: Option<String>,
    /// Vertragskonditionen für diesen Tarif.
    #[cfg_attr(feature = "serde", serde(rename = "vertragskonditionen"))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub vertragskonditionen: Vertragskonditionen,
    /// Internetseite, auf der der Tarif veröffentlicht ist.
    #[cfg_attr(feature = "serde", serde(rename = "website"))]
    pub website: String,
    /// Angabe, in welchem Zeitraum der Tarif gültig ist
    #[cfg_attr(feature = "serde", serde(rename = "zeitlicheGueltigkeit"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub zeitliche_gueltigkeit: Option<Zeitraum>,
    /// Der Zeitraum, in dem eine Belieferung (für diesen Tarif) möglich ist.
    #[cfg_attr(feature = "serde", serde(rename = "zeitraumBelieferbarkeit"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub zeitraum_belieferbarkeit: Option<Zeitraum>,
    /// Der Zeitraum, in dem der Tarif beim Anbieter vertraglich abschließbar ist.
    #[cfg_attr(feature = "serde", serde(rename = "zeitraumVermarktung"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub zeitraum_vermarktung: Option<Zeitraum>,
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
impl Tarif {
    /// Creates a `Tarif` from the 10 fields the BO4E schema marks `required`,
    /// defaulting every other field.
    ///
    /// `Tarif` has no [`Default`]: `berechnungsparameter`, `energiemix`, `kundentypen`, `preisgarantie`, `registeranzahl`, `sparte`, `tarifmerkmale`, `tariftyp`, `vertragskonditionen`, `website` are required, and their
    /// types need not implement `Default` — so this is the
    /// `..Default::default()` stand-in.
    /// `_typ` and `_version` are stamped exactly as elsewhere.
    ///
    /// With this many parameters the `builder` feature reads better at a
    /// call site; this exists so the type is constructible without it.
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        berechnungsparameter: Tarifberechnungsparameter,
        energiemix: Vec<Energiemix>,
        kundentypen: Vec<Kundentyp>,
        preisgarantie: Preisgarantie,
        registeranzahl: Registeranzahl,
        sparte: Sparte,
        tarifmerkmale: Vec<Tarifmerkmal>,
        tariftyp: Tariftyp,
        vertragskonditionen: Vertragskonditionen,
        website: String,
    ) -> Self {
        Self {
            anbieter: Default::default(),
            anbietername: Default::default(),
            anwendung_von: Default::default(),
            bemerkung: Default::default(),
            berechnungsparameter,
            beschreibung: Default::default(),
            bezeichnung: Default::default(),
            dynamische_preisposition_quelle: Default::default(),
            energiemix,
            id: Default::default(),
            kundentypen,
            preisgarantie,
            regionspreise: Default::default(),
            registeranzahl,
            sparte,
            tarifeinschraenkung: Default::default(),
            tarifmerkmale,
            tariftyp,
            typ: Some(BoTyp::Tarif),
            version: Some("202607.1.0".to_owned()),
            vertragskonditionen,
            website,
            zeitliche_gueltigkeit: Default::default(),
            zeitraum_belieferbarkeit: Default::default(),
            zeitraum_vermarktung: Default::default(),
            zusatz_attribute: Default::default(),
            _additional: Default::default(),
        }
    }
}
impl Bo4eTyped for Tarif {
    type Typ = BoTyp;
    const TYP: BoTyp = BoTyp::Tarif;
    const TYP_WIRE: &'static str = "TARIF";
    const SCHEMA_VERSION: &'static str = "202607.1.0";
    const SCHEMA_SERIES: &'static str = "202607";
}
impl crate::bo4e_typed_sealed::Sealed for Tarif {}
impl Bo4eObject for Tarif {}
impl crate::bo4e_object_sealed::Sealed for Tarif {}
#[cfg(feature = "json")]
impl crate::json::sealed::Sealed for Tarif {}
#[cfg(feature = "json")]
impl crate::json::Bo4eJsonExt for Tarif {}
#[cfg(feature = "json")]
impl crate::json::Bo4eExtensionData for Tarif {
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
impl std::fmt::Display for Tarif {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => f.write_str(&json),
            Err(e) => write!(f, "<Tarif: serialization error: {e}>"),
        }
    }
}
impl crate::Bo4eStrict for Tarif {
    fn collect_unknown_enums(&self, path: &str, out: &mut Vec<String>) {
        if let Some(v) = &self.anbieter {
            crate::Bo4eStrict::collect_unknown_enums(
                &**v,
                &crate::strict::field_path(path, "anbieter"),
                out,
            );
        }
        crate::Bo4eStrict::collect_unknown_enums(
            &self.berechnungsparameter,
            &crate::strict::field_path(path, "berechnungsparameter"),
            out,
        );
        {
            let items = &self.energiemix;
            let child = crate::strict::field_path(path, "energiemix");
            for (i, item) in items.iter().enumerate() {
                crate::Bo4eStrict::collect_unknown_enums(
                    item,
                    &crate::strict::index_path(&child, i),
                    out,
                );
            }
        }
        {
            let items = &self.kundentypen;
            let child = crate::strict::field_path(path, "kundentypen");
            for (i, item) in items.iter().enumerate() {
                crate::Bo4eStrict::collect_unknown_enums(
                    item,
                    &crate::strict::index_path(&child, i),
                    out,
                );
            }
        }
        crate::Bo4eStrict::collect_unknown_enums(
            &self.preisgarantie,
            &crate::strict::field_path(path, "preisgarantie"),
            out,
        );
        if let Some(items) = &self.regionspreise {
            let child = crate::strict::field_path(path, "regionspreise");
            for (i, item) in items.iter().enumerate() {
                crate::Bo4eStrict::collect_unknown_enums(
                    item,
                    &crate::strict::index_path(&child, i),
                    out,
                );
            }
        }
        crate::Bo4eStrict::collect_unknown_enums(
            &self.registeranzahl,
            &crate::strict::field_path(path, "registeranzahl"),
            out,
        );
        crate::Bo4eStrict::collect_unknown_enums(
            &self.sparte,
            &crate::strict::field_path(path, "sparte"),
            out,
        );
        if let Some(v) = &self.tarifeinschraenkung {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "tarifeinschraenkung"),
                out,
            );
        }
        {
            let items = &self.tarifmerkmale;
            let child = crate::strict::field_path(path, "tarifmerkmale");
            for (i, item) in items.iter().enumerate() {
                crate::Bo4eStrict::collect_unknown_enums(
                    item,
                    &crate::strict::index_path(&child, i),
                    out,
                );
            }
        }
        crate::Bo4eStrict::collect_unknown_enums(
            &self.tariftyp,
            &crate::strict::field_path(path, "tariftyp"),
            out,
        );
        crate::Bo4eStrict::collect_unknown_enums(
            &self.vertragskonditionen,
            &crate::strict::field_path(path, "vertragskonditionen"),
            out,
        );
        if let Some(v) = &self.zeitliche_gueltigkeit {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "zeitlicheGueltigkeit"),
                out,
            );
        }
        if let Some(v) = &self.zeitraum_belieferbarkeit {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "zeitraumBelieferbarkeit"),
                out,
            );
        }
        if let Some(v) = &self.zeitraum_vermarktung {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "zeitraumVermarktung"),
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
impl crate::json::Bo4eExtensions for Tarif {
    fn collect_extension_paths(&self, path: &str, out: &mut Vec<String>) {
        if let Some(map) = self._additional.as_map() {
            for key in map.keys() {
                out.push(crate::strict::extension_path(path, key));
            }
        }
        if let Some(v) = &self.anbieter {
            crate::json::Bo4eExtensions::collect_extension_paths(
                &**v,
                &crate::strict::field_path(path, "anbieter"),
                out,
            );
        }
        crate::json::Bo4eExtensions::collect_extension_paths(
            &self.berechnungsparameter,
            &crate::strict::field_path(path, "berechnungsparameter"),
            out,
        );
        {
            let items = &self.energiemix;
            let child = crate::strict::field_path(path, "energiemix");
            for (i, item) in items.iter().enumerate() {
                crate::json::Bo4eExtensions::collect_extension_paths(
                    item,
                    &crate::strict::index_path(&child, i),
                    out,
                );
            }
        }
        crate::json::Bo4eExtensions::collect_extension_paths(
            &self.preisgarantie,
            &crate::strict::field_path(path, "preisgarantie"),
            out,
        );
        if let Some(items) = &self.regionspreise {
            let child = crate::strict::field_path(path, "regionspreise");
            for (i, item) in items.iter().enumerate() {
                crate::json::Bo4eExtensions::collect_extension_paths(
                    item,
                    &crate::strict::index_path(&child, i),
                    out,
                );
            }
        }
        if let Some(v) = &self.tarifeinschraenkung {
            crate::json::Bo4eExtensions::collect_extension_paths(
                v,
                &crate::strict::field_path(path, "tarifeinschraenkung"),
                out,
            );
        }
        crate::json::Bo4eExtensions::collect_extension_paths(
            &self.vertragskonditionen,
            &crate::strict::field_path(path, "vertragskonditionen"),
            out,
        );
        if let Some(v) = &self.zeitliche_gueltigkeit {
            crate::json::Bo4eExtensions::collect_extension_paths(
                v,
                &crate::strict::field_path(path, "zeitlicheGueltigkeit"),
                out,
            );
        }
        if let Some(v) = &self.zeitraum_belieferbarkeit {
            crate::json::Bo4eExtensions::collect_extension_paths(
                v,
                &crate::strict::field_path(path, "zeitraumBelieferbarkeit"),
                out,
            );
        }
        if let Some(v) = &self.zeitraum_vermarktung {
            crate::json::Bo4eExtensions::collect_extension_paths(
                v,
                &crate::strict::field_path(path, "zeitraumVermarktung"),
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
impl crate::zusatz_attribut::HasZusatzAttribute for Tarif {
    fn zusatz_attribute_field(&self) -> Option<&Vec<ZusatzAttribut>> {
        self.zusatz_attribute.as_ref()
    }
    fn zusatz_attribute_field_mut(&mut self) -> &mut Option<Vec<ZusatzAttribut>> {
        &mut self.zusatz_attribute
    }
}
