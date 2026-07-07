use super::{
    Abwicklungsmodell, Aggregationsverantwortung, Bo4eObject, BoTyp,
    Fallgruppenzuordnung, Lastprofil, Menge, Profiltyp, Prognosegrundlage,
    WahlrechtPrognosegrundlage, Zeitreihentyp, ZusatzAttribut,
};
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(not(feature = "json"), derive(Hash))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "validate", derive(garde::Validate))]
#[cfg_attr(feature = "validate", garde(allow_unvalidated))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(
    all(feature = "validate", feature = "versioned"),
    garde(custom(crate::validation::v202501::validate_bilanzierung_dates))
)]
/// Das BO Bilanzierung erfasst alle relevanten Informationen zur Bilanzierung.
///
/// > **Note:** [Lastprofil JSON Schema](https://json-schema.app/view/%23?url=https://raw.githubusercontent.com/Hochfrequenz/BO4E-Schemas/{__gh_version__}/src/bo4e_schemas/bo/Bilanzierung.json)
pub struct Bilanzierung {
    #[cfg_attr(feature = "serde", serde(rename = "abwicklungsmodell"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub abwicklungsmodell: Option<Abwicklungsmodell>,
    #[cfg_attr(feature = "serde", serde(rename = "aggregationsverantwortung"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub aggregationsverantwortung: Option<Aggregationsverantwortung>,
    #[cfg_attr(feature = "serde", serde(rename = "bilanzierungsbeginn"))]
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
    pub bilanzierungsbeginn: Option<time::OffsetDateTime>,
    /// Requires the `time` feature for the `time::OffsetDateTime` representation.
    /// Without `time`, stores the ISO-8601 string value unchanged.
    #[cfg_attr(feature = "serde", serde(rename = "bilanzierungsbeginn"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(
        feature = "schemars",
        schemars(schema_with = "crate::schema_helpers::opt_datetime_schema")
    )]
    #[cfg(not(feature = "time"))]
    pub bilanzierungsbeginn: Option<String>,
    #[cfg_attr(feature = "serde", serde(rename = "bilanzierungsende"))]
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
    pub bilanzierungsende: Option<time::OffsetDateTime>,
    /// Requires the `time` feature for the `time::OffsetDateTime` representation.
    /// Without `time`, stores the ISO-8601 string value unchanged.
    #[cfg_attr(feature = "serde", serde(rename = "bilanzierungsende"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(
        feature = "schemars",
        schemars(schema_with = "crate::schema_helpers::opt_datetime_schema")
    )]
    #[cfg(not(feature = "time"))]
    pub bilanzierungsende: Option<String>,
    #[cfg_attr(feature = "serde", serde(rename = "bilanzkreis"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub bilanzkreis: Option<crate::identifiers::EicCode>,
    /// Prognosegrundlage.
    ///
    /// Besteht der Bedarf ein tagesparameteräbhängiges Lastprofil mit gemeinsamer Messung anzugeben,
    /// so ist dies über die 2 -malige Wiederholung des CAV Segments mit der Angabe der Codes E02 und E14 möglich.
    #[cfg_attr(feature = "serde", serde(rename = "detailsPrognosegrundlage"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub details_prognosegrundlage: Option<Vec<Profiltyp>>,
    #[cfg_attr(feature = "serde", serde(rename = "fallgruppenzuordnung"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub fallgruppenzuordnung: Option<Fallgruppenzuordnung>,
    /// Grund Wahlrecht der Prognosegrundlage.
    ///
    /// true=Wahlrecht beim Lieferanten vorhanden
    #[cfg_attr(feature = "serde", serde(rename = "grundWahlrechtPrognosegrundlage"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub grund_wahlrecht_prognosegrundlage: Option<WahlrechtPrognosegrundlage>,
    /// Eine generische ID, die für eigene Zwecke genutzt werden kann.
    /// Z.B. könnten hier UUIDs aus einer Datenbank stehen oder URLs zu einem Backend-System.
    #[cfg_attr(feature = "serde", serde(rename = "_id"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub id: Option<String>,
    #[cfg_attr(feature = "serde", serde(rename = "jahresverbrauchsprognose"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub jahresverbrauchsprognose: Option<Menge>,
    #[cfg_attr(feature = "serde", serde(rename = "kundenwert"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub kundenwert: Option<Menge>,
    #[cfg_attr(feature = "serde", serde(rename = "lastprofil"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub lastprofil: Option<Vec<Lastprofil>>,
    #[cfg_attr(feature = "serde", serde(rename = "marktlokationsId"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub marktlokations_id: Option<crate::identifiers::MaloId>,
    #[cfg_attr(feature = "serde", serde(rename = "prioritaet"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub prioritaet: Option<i64>,
    #[cfg_attr(feature = "serde", serde(rename = "prognosegrundlage"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub prognosegrundlage: Option<Prognosegrundlage>,
    #[cfg_attr(feature = "serde", serde(rename = "temperaturArbeit"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub temperatur_arbeit: Option<Menge>,
    /// BO type identifier — always `BoTyp::Bilanzierung` for this struct.
    #[cfg_attr(feature = "serde", serde(rename = "_typ"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(
        feature = "builder",
        builder(default = Some(BoTyp::Bilanzierung), setter(skip))
    )]
    pub typ: Option<BoTyp>,
    /// Verbrauchsaufteilung in % zwischen SLP und TLP-Profil.
    ///
    /// 1. [Gemessene Energiemenge der OBIS "nicht Schwachlast"] * [Verbrauchsaufteilung in % / 100%]
    ///    = [zu verlagernde Energiemenge]
    /// 2. [Gemessene Energiemenge der OBIS "Schwachlast"] - [zu verlagernde Energiemenge]
    ///    = [Ermittelte Energiemenge für Schwachlast]
    /// 3. [Gemessene Energiemenge der OBIS "nicht Schwachlast"] + [zu verlagernde Energiemenge]
    ///    = [Ermittelte Energiemenge für nicht Schwachlast]
    #[cfg_attr(feature = "serde", serde(rename = "verbrauchsaufteilung"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg(feature = "decimal")]
    pub verbrauchsaufteilung: Option<rust_decimal::Decimal>,
    /// Requires the `decimal` feature for the `rust_decimal::Decimal` representation.
    /// Without `decimal`, stores the decimal string value unchanged.
    #[cfg_attr(feature = "serde", serde(rename = "verbrauchsaufteilung"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg(not(feature = "decimal"))]
    pub verbrauchsaufteilung: Option<String>,
    /// Version der BO-Struktur aka "fachliche Versionierung"
    #[cfg_attr(feature = "serde", serde(rename = "_version"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub version: Option<String>,
    /// Wahlrecht der Prognosegrundlage.
    #[cfg_attr(feature = "serde", serde(rename = "wahlrechtPrognosegrundlage"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub wahlrecht_prognosegrundlage: Option<WahlrechtPrognosegrundlage>,
    #[cfg_attr(feature = "serde", serde(rename = "zeitreihentyp"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub zeitreihentyp: Option<Zeitreihentyp>,
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
impl Default for Bilanzierung {
    fn default() -> Self {
        Self {
            typ: Some(BoTyp::Bilanzierung),
            abwicklungsmodell: Default::default(),
            aggregationsverantwortung: Default::default(),
            bilanzierungsbeginn: Default::default(),
            bilanzierungsende: Default::default(),
            bilanzkreis: Default::default(),
            details_prognosegrundlage: Default::default(),
            fallgruppenzuordnung: Default::default(),
            grund_wahlrecht_prognosegrundlage: Default::default(),
            id: Default::default(),
            jahresverbrauchsprognose: Default::default(),
            kundenwert: Default::default(),
            lastprofil: Default::default(),
            marktlokations_id: Default::default(),
            prioritaet: Default::default(),
            prognosegrundlage: Default::default(),
            temperatur_arbeit: Default::default(),
            verbrauchsaufteilung: Default::default(),
            version: Default::default(),
            wahlrecht_prognosegrundlage: Default::default(),
            zeitreihentyp: Default::default(),
            zusatz_attribute: Default::default(),
            _additional: Default::default(),
        }
    }
}
impl Bo4eObject for Bilanzierung {
    type BoTyp = BoTyp;
    fn bo_type(&self) -> BoTyp {
        self.typ.unwrap_or(BoTyp::Bilanzierung)
    }
    fn schema_version(&self) -> &'static str {
        "v202501.0.0"
    }
}
#[cfg(feature = "json")]
impl crate::json::sealed::Sealed for Bilanzierung {}
#[cfg(feature = "json")]
impl crate::json::Bo4eJsonExt for Bilanzierung {}
#[cfg(feature = "json")]
impl crate::json::Bo4eExtensionData for Bilanzierung {
    fn extension_data(&self) -> &indexmap::IndexMap<String, serde_json::Value> {
        self._additional.as_map().unwrap_or(&crate::json::extension::EMPTY_EXTENSION_MAP)
    }
    fn has_extension_data(&self) -> bool {
        !self._additional.is_empty()
    }
}
#[cfg(feature = "json")]
impl std::fmt::Display for Bilanzierung {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => f.write_str(&json),
            Err(e) => write!(f, "<Bilanzierung: serialization error: {e}>"),
        }
    }
}
