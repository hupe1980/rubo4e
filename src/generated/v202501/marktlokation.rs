use super::{
    Adresse, Bilanzierungsmethode, Bo4eObject, BoTyp, Energierichtung, Gasqualitaet, Gebiettyp,
    Geokoordinaten, Geschaeftspartner, Katasteradresse, Kundentyp, Lokationszuordnung, Netzebene,
    Sparte, Verbrauch, Verbrauchsart, Zaehlwerk, ZusatzAttribut,
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
    garde(custom(crate::validation::v202501::validate_marktlokation))
)]
/// Object containing information about a Marktlokation
///
/// > **Note:** [Marktlokation JSON Schema](https://json-schema.app/view/%23?url=https://raw.githubusercontent.com/BO4E/BO4E-Schemas/v202501.0.0/src/bo4e_schemas/bo/Marktlokation.json)
pub struct Marktlokation {
    /// Bilanzierungsgebiet, dem das Netzgebiet zugeordnet ist - im Falle eines Strom Netzes
    #[cfg_attr(feature = "serde", serde(rename = "bilanzierungsgebiet"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub bilanzierungsgebiet: Option<String>,
    /// Die Bilanzierungsmethode, RLM oder SLP
    #[cfg_attr(feature = "serde", serde(rename = "bilanzierungsmethode"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub bilanzierungsmethode: Option<Bilanzierungsmethode>,
    /// Geschäftspartner, dem diese Marktlokation gehört
    #[cfg_attr(feature = "serde", serde(rename = "endkunde"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub endkunde: Option<Box<Geschaeftspartner>>,
    /// Kennzeichnung, ob Energie eingespeist oder entnommen (ausgespeist) wird
    #[cfg_attr(feature = "serde", serde(rename = "energierichtung"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub energierichtung: Option<Energierichtung>,
    /// Die Gasqualität in diesem Netzgebiet. H-Gas oder L-Gas. Im Falle eines Gas-Netzes
    #[cfg_attr(feature = "serde", serde(rename = "gasqualitaet"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub gasqualitaet: Option<Gasqualitaet>,
    /// Typ des Netzgebietes, z.B. Verteilnetz
    #[cfg_attr(feature = "serde", serde(rename = "gebietstyp"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub gebietstyp: Option<Gebiettyp>,
    /// Alternativ zu einer postalischen Adresse kann hier ein Ort mittels Geokoordinaten angegeben werden
    /// (z.B. zur Identifikation von Sendemasten).
    #[cfg_attr(feature = "serde", serde(rename = "geoadresse"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub geoadresse: Option<Geokoordinaten>,
    /// Codenummer des Grundversorgers, der für diese Marktlokation zuständig ist
    #[cfg_attr(feature = "serde", serde(rename = "grundversorgercodenr"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub grundversorgercodenr: Option<crate::identifiers::MarktpartnerId>,
    /// Eine generische ID, die für eigene Zwecke genutzt werden kann.
    /// Z.B. könnten hier UUIDs aus einer Datenbank stehen oder URLs zu einem Backend-System.
    #[cfg_attr(feature = "serde", serde(rename = "_id"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub id: Option<String>,
    /// Gibt an, ob es sich um eine unterbrechbare Belieferung handelt
    #[cfg_attr(feature = "serde", serde(rename = "istUnterbrechbar"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub ist_unterbrechbar: Option<bool>,
    /// Alternativ zu einer postalischen Adresse und Geokoordinaten kann hier eine Ortsangabe mittels Gemarkung und
    /// Flurstück erfolgen.
    #[cfg_attr(feature = "serde", serde(rename = "katasterinformation"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub katasterinformation: Option<Katasteradresse>,
    /// Kundengruppen der Marktlokation
    #[cfg_attr(feature = "serde", serde(rename = "kundengruppen"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub kundengruppen: Option<Vec<Kundentyp>>,
    /// Die Adresse, an der die Energie-Lieferung oder -Einspeisung erfolgt
    #[cfg_attr(feature = "serde", serde(rename = "lokationsadresse"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub lokationsadresse: Option<Adresse>,
    /// Lokationsbuendel Code, der die Funktion dieses BOs an der Lokationsbuendelstruktur beschreibt.
    #[cfg_attr(feature = "serde", serde(rename = "lokationsbuendelObjektcode"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub lokationsbuendel_objektcode: Option<String>,
    /// Lokationszuordnung, um bspw. die zugehörigen Messlokationen anzugeben
    #[cfg_attr(feature = "serde", serde(rename = "lokationszuordnungen"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub lokationszuordnungen: Option<Vec<Box<Lokationszuordnung>>>,
    /// für Gas. Code vom EIC, <https://www.entsog.eu/data/data-portal/codes-list>
    #[cfg_attr(feature = "serde", serde(rename = "marktgebiet"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub marktgebiet: Option<crate::identifiers::EicCode>,
    /// Identifikationsnummer einer Marktlokation, an der Energie entweder verbraucht, oder erzeugt wird.
    #[cfg_attr(feature = "serde", serde(rename = "marktlokationsId"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub marktlokations_id: Option<crate::identifiers::MaloId>,
    /// Codenummer des Netzbetreibers, an dessen Netz diese Marktlokation angeschlossen ist.
    #[cfg_attr(feature = "serde", serde(rename = "netzbetreibercodenr"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub netzbetreibercodenr: Option<crate::identifiers::MarktpartnerId>,
    /// Netzebene, in der der Bezug der Energie erfolgt.
    /// Bei Strom Spannungsebene der Lieferung, bei Gas Druckstufe.
    /// Beispiel Strom: Niederspannung Beispiel Gas: Niederdruck.
    #[cfg_attr(feature = "serde", serde(rename = "netzebene"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub netzebene: Option<Netzebene>,
    /// Die ID des Gebietes in der ene't-Datenbank
    #[cfg_attr(feature = "serde", serde(rename = "netzgebietsnr"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub netzgebietsnr: Option<String>,
    /// für Strom. Code vom EIC, <https://www.entsoe.eu/data/energy-identification-codes-eic/eic-approved-codes/>
    #[cfg_attr(feature = "serde", serde(rename = "regelzone"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub regelzone: Option<crate::identifiers::EicCode>,
    /// Sparte der Marktlokation, z.B. Gas oder Strom
    #[cfg_attr(feature = "serde", serde(rename = "sparte"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub sparte: Option<Sparte>,
    /// BO type identifier — always `BoTyp::Marktlokation` for this struct.
    #[cfg_attr(feature = "serde", serde(rename = "_typ"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(
        feature = "builder",
        builder(default = Some(BoTyp::Marktlokation), setter(skip))
    )]
    pub typ: Option<BoTyp>,
    /// Verbrauchsart der Marktlokation.
    #[cfg_attr(feature = "serde", serde(rename = "verbrauchsart"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub verbrauchsart: Option<Verbrauchsart>,
    #[cfg_attr(feature = "serde", serde(rename = "verbrauchsmengen"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub verbrauchsmengen: Option<Vec<Verbrauch>>,
    /// Version der BO-Struktur aka "fachliche Versionierung"
    #[cfg_attr(feature = "serde", serde(rename = "_version"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub version: Option<String>,
    #[cfg_attr(feature = "serde", serde(rename = "zaehlwerke"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub zaehlwerke: Option<Vec<Zaehlwerk>>,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "zaehlwerkeDerBeteiligtenMarktrolle")
    )]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub zaehlwerke_der_beteiligten_marktrolle: Option<Vec<Zaehlwerk>>,
    #[cfg_attr(feature = "serde", serde(rename = "zusatzAttribute"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
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
    pub(crate) _additional: crate::LimitedExtensionMap,
}
impl Default for Marktlokation {
    fn default() -> Self {
        Self {
            typ: Some(BoTyp::Marktlokation),
            bilanzierungsgebiet: Default::default(),
            bilanzierungsmethode: Default::default(),
            endkunde: Default::default(),
            energierichtung: Default::default(),
            gasqualitaet: Default::default(),
            gebietstyp: Default::default(),
            geoadresse: Default::default(),
            grundversorgercodenr: Default::default(),
            id: Default::default(),
            ist_unterbrechbar: Default::default(),
            katasterinformation: Default::default(),
            kundengruppen: Default::default(),
            lokationsadresse: Default::default(),
            lokationsbuendel_objektcode: Default::default(),
            lokationszuordnungen: Default::default(),
            marktgebiet: Default::default(),
            marktlokations_id: Default::default(),
            netzbetreibercodenr: Default::default(),
            netzebene: Default::default(),
            netzgebietsnr: Default::default(),
            regelzone: Default::default(),
            sparte: Default::default(),
            verbrauchsart: Default::default(),
            verbrauchsmengen: Default::default(),
            version: Default::default(),
            zaehlwerke: Default::default(),
            zaehlwerke_der_beteiligten_marktrolle: Default::default(),
            zusatz_attribute: Default::default(),
            _additional: Default::default(),
        }
    }
}
impl Bo4eObject for Marktlokation {
    type BoTyp = BoTyp;
    fn bo_type(&self) -> BoTyp {
        self.typ.unwrap_or(BoTyp::Marktlokation)
    }
    fn schema_version(&self) -> &'static str {
        "v202501.0.0"
    }
}
#[cfg(feature = "json")]
impl crate::json::sealed::Sealed for Marktlokation {}
#[cfg(feature = "json")]
impl crate::json::Bo4eJsonExt for Marktlokation {}
#[cfg(feature = "json")]
impl crate::json::Bo4eExtensionData for Marktlokation {
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
impl std::fmt::Display for Marktlokation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => f.write_str(&json),
            Err(e) => write!(f, "<Marktlokation: serialization error: {e}>"),
        }
    }
}
