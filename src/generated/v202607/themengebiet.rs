#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(strum::EnumString, strum::EnumIter, strum::IntoStaticStr)
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Über dieses ENUM kann eine thematische Zuordnung, beispielsweise eines Ansprechpartners, vorgenommen werden.
#[non_exhaustive]
pub enum Themengebiet {
    #[cfg_attr(feature = "serde", serde(rename = "ALLGEMEINER_INFORMATIONSAUSTAUSCH"))]
    #[cfg_attr(
        feature = "strum",
        strum(serialize = "ALLGEMEINER_INFORMATIONSAUSTAUSCH")
    )]
    AllgemeinerInformationsaustausch,
    #[cfg_attr(feature = "serde", serde(rename = "AN_UND_ABMELDUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "AN_UND_ABMELDUNG"))]
    AnUndAbmeldung,
    #[cfg_attr(feature = "serde", serde(rename = "ANSPRECHPARTNER_ALLGEMEIN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ANSPRECHPARTNER_ALLGEMEIN"))]
    AnsprechpartnerAllgemein,
    #[cfg_attr(feature = "serde", serde(rename = "ANSPRECHPARTNER_BDEW_DVGW"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ANSPRECHPARTNER_BDEW_DVGW"))]
    AnsprechpartnerBdewDvgw,
    #[cfg_attr(feature = "serde", serde(rename = "ANSPRECHPARTNER_IT_TECHNIK"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ANSPRECHPARTNER_IT_TECHNIK"))]
    AnsprechpartnerItTechnik,
    #[cfg_attr(feature = "serde", serde(rename = "BILANZIERUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BILANZIERUNG"))]
    Bilanzierung,
    #[cfg_attr(feature = "serde", serde(rename = "BILANZKREISKOORDINATOR"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BILANZKREISKOORDINATOR"))]
    Bilanzkreiskoordinator,
    #[cfg_attr(feature = "serde", serde(rename = "BILANZKREISVERANTWORTLICHER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BILANZKREISVERANTWORTLICHER"))]
    Bilanzkreisverantwortlicher,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "DATENFORMATE_ZERTIFIKATE_VERSCHLUESSELUNGEN")
    )]
    #[cfg_attr(
        feature = "strum",
        strum(serialize = "DATENFORMATE_ZERTIFIKATE_VERSCHLUESSELUNGEN")
    )]
    DatenformateZertifikateVerschluesselungen,
    #[cfg_attr(feature = "serde", serde(rename = "DEBITORENMANAGEMENT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "DEBITORENMANAGEMENT"))]
    Debitorenmanagement,
    #[cfg_attr(feature = "serde", serde(rename = "DEMAND_SIDE_MANAGEMENT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "DEMAND_SIDE_MANAGEMENT"))]
    DemandSideManagement,
    #[cfg_attr(feature = "serde", serde(rename = "EDI_VEREINBARUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "EDI_VEREINBARUNG"))]
    EdiVereinbarung,
    #[cfg_attr(feature = "serde", serde(rename = "EDIFACT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "EDIFACT"))]
    Edifact,
    #[cfg_attr(feature = "serde", serde(rename = "ENERGIEDATENMANAGEMENT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ENERGIEDATENMANAGEMENT"))]
    Energiedatenmanagement,
    #[cfg_attr(feature = "serde", serde(rename = "FAHRPLANMANAGEMENT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "FAHRPLANMANAGEMENT"))]
    Fahrplanmanagement,
    #[cfg_attr(feature = "serde", serde(rename = "ALOCAT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ALOCAT"))]
    Alocat,
    #[cfg_attr(feature = "serde", serde(rename = "APERAK"))]
    #[cfg_attr(feature = "strum", strum(serialize = "APERAK"))]
    Aperak,
    #[cfg_attr(feature = "serde", serde(rename = "CONTRL"))]
    #[cfg_attr(feature = "strum", strum(serialize = "CONTRL"))]
    Contrl,
    #[cfg_attr(feature = "serde", serde(rename = "INVOIC"))]
    #[cfg_attr(feature = "strum", strum(serialize = "INVOIC"))]
    Invoic,
    #[cfg_attr(feature = "serde", serde(rename = "MSCONS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MSCONS"))]
    Mscons,
    #[cfg_attr(feature = "serde", serde(rename = "ORDERS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ORDERS"))]
    Orders,
    #[cfg_attr(feature = "serde", serde(rename = "ORDERSP"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ORDERSP"))]
    Ordersp,
    #[cfg_attr(feature = "serde", serde(rename = "REMADV"))]
    #[cfg_attr(feature = "strum", strum(serialize = "REMADV"))]
    Remadv,
    #[cfg_attr(feature = "serde", serde(rename = "UTILMD"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UTILMD"))]
    Utilmd,
    #[cfg_attr(feature = "serde", serde(rename = "GABI"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GABI"))]
    Gabi,
    #[cfg_attr(feature = "serde", serde(rename = "GELI"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GELI"))]
    Geli,
    #[cfg_attr(feature = "serde", serde(rename = "GERAETERUECKGABE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GERAETERUECKGABE"))]
    Geraeterueckgabe,
    #[cfg_attr(feature = "serde", serde(rename = "GERAETEWECHSEL"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GERAETEWECHSEL"))]
    Geraetewechsel,
    #[cfg_attr(feature = "serde", serde(rename = "GPKE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GPKE"))]
    Gpke,
    #[cfg_attr(feature = "serde", serde(rename = "INBETRIEBNAHME"))]
    #[cfg_attr(feature = "strum", strum(serialize = "INBETRIEBNAHME"))]
    Inbetriebnahme,
    #[cfg_attr(feature = "serde", serde(rename = "KAPAZITAETSMANAGEMENT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KAPAZITAETSMANAGEMENT"))]
    Kapazitaetsmanagement,
    #[cfg_attr(feature = "serde", serde(rename = "KLAERFAELLE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KLAERFAELLE"))]
    Klaerfaelle,
    #[cfg_attr(feature = "serde", serde(rename = "LASTGAENGE_RLM"))]
    #[cfg_attr(feature = "strum", strum(serialize = "LASTGAENGE_RLM"))]
    LastgaengeRlm,
    #[cfg_attr(feature = "serde", serde(rename = "LIEFERANTENRAHMENVERTRAG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "LIEFERANTENRAHMENVERTRAG"))]
    Lieferantenrahmenvertrag,
    #[cfg_attr(feature = "serde", serde(rename = "LIEFERANTENWECHSEL"))]
    #[cfg_attr(feature = "strum", strum(serialize = "LIEFERANTENWECHSEL"))]
    Lieferantenwechsel,
    #[cfg_attr(feature = "serde", serde(rename = "MABIS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MABIS"))]
    Mabis,
    #[cfg_attr(feature = "serde", serde(rename = "MAHNWESEN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MAHNWESEN"))]
    Mahnwesen,
    #[cfg_attr(feature = "serde", serde(rename = "MARKTGEBIETSVERANTWORTLICHER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MARKTGEBIETSVERANTWORTLICHER"))]
    Marktgebietsverantwortlicher,
    #[cfg_attr(feature = "serde", serde(rename = "MARKTKOMMUNIKATION"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MARKTKOMMUNIKATION"))]
    Marktkommunikation,
    #[cfg_attr(feature = "serde", serde(rename = "MEHR_MINDERMENGEN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MEHR_MINDERMENGEN"))]
    MehrMindermengen,
    #[cfg_attr(feature = "serde", serde(rename = "MSB_MDL"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MSB_MDL"))]
    MsbMdl,
    #[cfg_attr(feature = "serde", serde(rename = "NETZABRECHNUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "NETZABRECHNUNG"))]
    Netzabrechnung,
    #[cfg_attr(feature = "serde", serde(rename = "NETZENTGELTE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "NETZENTGELTE"))]
    Netzentgelte,
    #[cfg_attr(feature = "serde", serde(rename = "NETZMANAGEMENT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "NETZMANAGEMENT"))]
    Netzmanagement,
    #[cfg_attr(feature = "serde", serde(rename = "RECHT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "RECHT"))]
    Recht,
    #[cfg_attr(feature = "serde", serde(rename = "REGULIERUNGSMANAGEMENT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "REGULIERUNGSMANAGEMENT"))]
    Regulierungsmanagement,
    #[cfg_attr(feature = "serde", serde(rename = "REKLAMATIONEN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "REKLAMATIONEN"))]
    Reklamationen,
    #[cfg_attr(feature = "serde", serde(rename = "SPERREN_ENTSPERREN_INKASSO"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SPERREN_ENTSPERREN_INKASSO"))]
    SperrenEntsperrenInkasso,
    #[cfg_attr(feature = "serde", serde(rename = "STAMMDATEN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "STAMMDATEN"))]
    Stammdaten,
    #[cfg_attr(feature = "serde", serde(rename = "STOERUNGSFAELLE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "STOERUNGSFAELLE"))]
    Stoerungsfaelle,
    #[cfg_attr(feature = "serde", serde(rename = "TECHNISCHE_FRAGEN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TECHNISCHE_FRAGEN"))]
    TechnischeFragen,
    #[cfg_attr(feature = "serde", serde(rename = "UMSTELLUNG_INVOIC"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UMSTELLUNG_INVOIC"))]
    UmstellungInvoic,
    #[cfg_attr(feature = "serde", serde(rename = "VERSCHLUESSELUNG_SIGNATUR"))]
    #[cfg_attr(feature = "strum", strum(serialize = "VERSCHLUESSELUNG_SIGNATUR"))]
    VerschluesselungSignatur,
    #[cfg_attr(feature = "serde", serde(rename = "VERTRAGSMANAGEMENT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "VERTRAGSMANAGEMENT"))]
    Vertragsmanagement,
    #[cfg_attr(feature = "serde", serde(rename = "VERTRIEB"))]
    #[cfg_attr(feature = "strum", strum(serialize = "VERTRIEB"))]
    Vertrieb,
    #[cfg_attr(feature = "serde", serde(rename = "WIM"))]
    #[cfg_attr(feature = "strum", strum(serialize = "WIM"))]
    Wim,
    #[cfg_attr(feature = "serde", serde(rename = "ZAEHLERSTAENDE_SLP"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ZAEHLERSTAENDE_SLP"))]
    ZaehlerstaendeSlp,
    #[cfg_attr(feature = "serde", serde(rename = "ZAHLUNGSVERKEHR"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ZAHLUNGSVERKEHR"))]
    Zahlungsverkehr,
    #[cfg_attr(feature = "serde", serde(rename = "ZUORDNUNGSVEREINBARUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ZUORDNUNGSVEREINBARUNG"))]
    Zuordnungsvereinbarung,
    #[cfg_attr(feature = "serde", serde(rename = "EINSPEISUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "EINSPEISUNG"))]
    Einspeisung,
    #[cfg_attr(feature = "serde", serde(rename = "BEWEGUNGSDATEN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BEWEGUNGSDATEN"))]
    Bewegungsdaten,
    /// Unknown or future variant — produced when deserializing a value
    /// that is not yet known to this version of the library.
    #[cfg_attr(feature = "serde", serde(other, rename = "UNKNOWN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UNKNOWN"))]
    Unknown,
}
impl Themengebiet {
    /// All variants defined by the BO4E schema, in declaration order.
    ///
    /// Excludes the forward-compatibility [`Themengebiet::Unknown`] catch-all, so this
    /// is exactly the set of values that appear on the wire.  Available **without**
    /// the `strum` feature — use it to drift-guard SQL `CHECK` lists and mappings.
    pub const VARIANTS: &'static [Self] = &[
        Self::AllgemeinerInformationsaustausch,
        Self::AnUndAbmeldung,
        Self::AnsprechpartnerAllgemein,
        Self::AnsprechpartnerBdewDvgw,
        Self::AnsprechpartnerItTechnik,
        Self::Bilanzierung,
        Self::Bilanzkreiskoordinator,
        Self::Bilanzkreisverantwortlicher,
        Self::DatenformateZertifikateVerschluesselungen,
        Self::Debitorenmanagement,
        Self::DemandSideManagement,
        Self::EdiVereinbarung,
        Self::Edifact,
        Self::Energiedatenmanagement,
        Self::Fahrplanmanagement,
        Self::Alocat,
        Self::Aperak,
        Self::Contrl,
        Self::Invoic,
        Self::Mscons,
        Self::Orders,
        Self::Ordersp,
        Self::Remadv,
        Self::Utilmd,
        Self::Gabi,
        Self::Geli,
        Self::Geraeterueckgabe,
        Self::Geraetewechsel,
        Self::Gpke,
        Self::Inbetriebnahme,
        Self::Kapazitaetsmanagement,
        Self::Klaerfaelle,
        Self::LastgaengeRlm,
        Self::Lieferantenrahmenvertrag,
        Self::Lieferantenwechsel,
        Self::Mabis,
        Self::Mahnwesen,
        Self::Marktgebietsverantwortlicher,
        Self::Marktkommunikation,
        Self::MehrMindermengen,
        Self::MsbMdl,
        Self::Netzabrechnung,
        Self::Netzentgelte,
        Self::Netzmanagement,
        Self::Recht,
        Self::Regulierungsmanagement,
        Self::Reklamationen,
        Self::SperrenEntsperrenInkasso,
        Self::Stammdaten,
        Self::Stoerungsfaelle,
        Self::TechnischeFragen,
        Self::UmstellungInvoic,
        Self::VerschluesselungSignatur,
        Self::Vertragsmanagement,
        Self::Vertrieb,
        Self::Wim,
        Self::ZaehlerstaendeSlp,
        Self::Zahlungsverkehr,
        Self::Zuordnungsvereinbarung,
        Self::Einspeisung,
        Self::Bewegungsdaten,
    ];
    /// Number of schema-defined variants (equal to `VARIANTS.len()`), excluding the
    /// [`Themengebiet::Unknown`] catch-all.  Stable for this schema version.
    pub const COUNT: usize = Self::VARIANTS.len();
    /// Returns an iterator over all **known** variants of `Themengebiet`.
    ///
    /// Yields only variants that correspond to values defined in the BO4E schema
    /// (i.e. [`Self::VARIANTS`]), never the [`Themengebiet::Unknown`] catch-all.
    /// Available **without** the `strum` feature.
    ///
    /// # Example
    /// ```
    /// # use rubo4e::current::Themengebiet;
    /// // Never yields the `Unknown` catch-all, so the count matches `COUNT`.
    /// assert_eq!(Themengebiet::iter_known().count(), Themengebiet::COUNT);
    /// assert!(Themengebiet::iter_known().all(|v| v.is_known()));
    /// ```
    pub fn iter_known() -> impl Iterator<Item = Self> + Clone {
        Self::VARIANTS.iter().copied()
    }
    /// Returns the canonical BO4E wire string (SCREAMING_SNAKE_CASE) for this value.
    ///
    /// [`Themengebiet::Unknown`] renders as `"UNKNOWN"`, matching its serialized form.
    pub const fn as_wire(&self) -> &'static str {
        match self {
            Self::AllgemeinerInformationsaustausch => "ALLGEMEINER_INFORMATIONSAUSTAUSCH",
            Self::AnUndAbmeldung => "AN_UND_ABMELDUNG",
            Self::AnsprechpartnerAllgemein => "ANSPRECHPARTNER_ALLGEMEIN",
            Self::AnsprechpartnerBdewDvgw => "ANSPRECHPARTNER_BDEW_DVGW",
            Self::AnsprechpartnerItTechnik => "ANSPRECHPARTNER_IT_TECHNIK",
            Self::Bilanzierung => "BILANZIERUNG",
            Self::Bilanzkreiskoordinator => "BILANZKREISKOORDINATOR",
            Self::Bilanzkreisverantwortlicher => "BILANZKREISVERANTWORTLICHER",
            Self::DatenformateZertifikateVerschluesselungen => {
                "DATENFORMATE_ZERTIFIKATE_VERSCHLUESSELUNGEN"
            }
            Self::Debitorenmanagement => "DEBITORENMANAGEMENT",
            Self::DemandSideManagement => "DEMAND_SIDE_MANAGEMENT",
            Self::EdiVereinbarung => "EDI_VEREINBARUNG",
            Self::Edifact => "EDIFACT",
            Self::Energiedatenmanagement => "ENERGIEDATENMANAGEMENT",
            Self::Fahrplanmanagement => "FAHRPLANMANAGEMENT",
            Self::Alocat => "ALOCAT",
            Self::Aperak => "APERAK",
            Self::Contrl => "CONTRL",
            Self::Invoic => "INVOIC",
            Self::Mscons => "MSCONS",
            Self::Orders => "ORDERS",
            Self::Ordersp => "ORDERSP",
            Self::Remadv => "REMADV",
            Self::Utilmd => "UTILMD",
            Self::Gabi => "GABI",
            Self::Geli => "GELI",
            Self::Geraeterueckgabe => "GERAETERUECKGABE",
            Self::Geraetewechsel => "GERAETEWECHSEL",
            Self::Gpke => "GPKE",
            Self::Inbetriebnahme => "INBETRIEBNAHME",
            Self::Kapazitaetsmanagement => "KAPAZITAETSMANAGEMENT",
            Self::Klaerfaelle => "KLAERFAELLE",
            Self::LastgaengeRlm => "LASTGAENGE_RLM",
            Self::Lieferantenrahmenvertrag => "LIEFERANTENRAHMENVERTRAG",
            Self::Lieferantenwechsel => "LIEFERANTENWECHSEL",
            Self::Mabis => "MABIS",
            Self::Mahnwesen => "MAHNWESEN",
            Self::Marktgebietsverantwortlicher => "MARKTGEBIETSVERANTWORTLICHER",
            Self::Marktkommunikation => "MARKTKOMMUNIKATION",
            Self::MehrMindermengen => "MEHR_MINDERMENGEN",
            Self::MsbMdl => "MSB_MDL",
            Self::Netzabrechnung => "NETZABRECHNUNG",
            Self::Netzentgelte => "NETZENTGELTE",
            Self::Netzmanagement => "NETZMANAGEMENT",
            Self::Recht => "RECHT",
            Self::Regulierungsmanagement => "REGULIERUNGSMANAGEMENT",
            Self::Reklamationen => "REKLAMATIONEN",
            Self::SperrenEntsperrenInkasso => "SPERREN_ENTSPERREN_INKASSO",
            Self::Stammdaten => "STAMMDATEN",
            Self::Stoerungsfaelle => "STOERUNGSFAELLE",
            Self::TechnischeFragen => "TECHNISCHE_FRAGEN",
            Self::UmstellungInvoic => "UMSTELLUNG_INVOIC",
            Self::VerschluesselungSignatur => "VERSCHLUESSELUNG_SIGNATUR",
            Self::Vertragsmanagement => "VERTRAGSMANAGEMENT",
            Self::Vertrieb => "VERTRIEB",
            Self::Wim => "WIM",
            Self::ZaehlerstaendeSlp => "ZAEHLERSTAENDE_SLP",
            Self::Zahlungsverkehr => "ZAHLUNGSVERKEHR",
            Self::Zuordnungsvereinbarung => "ZUORDNUNGSVEREINBARUNG",
            Self::Einspeisung => "EINSPEISUNG",
            Self::Bewegungsdaten => "BEWEGUNGSDATEN",
            Self::Unknown => "UNKNOWN",
        }
    }
    /// **Strictly** parses a BO4E wire string into a known variant.
    ///
    /// Unlike the lenient `serde` / [`FromStr`](std::str::FromStr) path — which maps
    /// any unrecognized value (a typo, a legacy code, or a value from a newer schema)
    /// to [`Themengebiet::Unknown`] — this returns
    /// [`Err`](crate::error::UnknownVariant) for values not defined in this schema
    /// version, including the literal `"UNKNOWN"`.  Use it at the ingest boundary to
    /// reject bad values instead of silently degrading them.
    ///
    /// # Example
    /// ```
    /// # use rubo4e::current::Themengebiet;
    /// /// assert_eq!(Themengebiet::from_wire("ALLGEMEINER_INFORMATIONSAUSTAUSCH"), Ok(Themengebiet::AllgemeinerInformationsaustausch));
    /// assert!(Themengebiet::from_wire("NOT_A_REAL_VALUE").is_err());
    /// // …including the `Unknown` catch-all's own wire spelling:
    /// assert!(Themengebiet::from_wire("UNKNOWN").is_err());
    /// ```
    pub fn from_wire(s: &str) -> Result<Self, crate::error::UnknownVariant> {
        match s {
            "ALLGEMEINER_INFORMATIONSAUSTAUSCH" => Ok(Self::AllgemeinerInformationsaustausch),
            "AN_UND_ABMELDUNG" => Ok(Self::AnUndAbmeldung),
            "ANSPRECHPARTNER_ALLGEMEIN" => Ok(Self::AnsprechpartnerAllgemein),
            "ANSPRECHPARTNER_BDEW_DVGW" => Ok(Self::AnsprechpartnerBdewDvgw),
            "ANSPRECHPARTNER_IT_TECHNIK" => Ok(Self::AnsprechpartnerItTechnik),
            "BILANZIERUNG" => Ok(Self::Bilanzierung),
            "BILANZKREISKOORDINATOR" => Ok(Self::Bilanzkreiskoordinator),
            "BILANZKREISVERANTWORTLICHER" => Ok(Self::Bilanzkreisverantwortlicher),
            "DATENFORMATE_ZERTIFIKATE_VERSCHLUESSELUNGEN" => {
                Ok(Self::DatenformateZertifikateVerschluesselungen)
            }
            "DEBITORENMANAGEMENT" => Ok(Self::Debitorenmanagement),
            "DEMAND_SIDE_MANAGEMENT" => Ok(Self::DemandSideManagement),
            "EDI_VEREINBARUNG" => Ok(Self::EdiVereinbarung),
            "EDIFACT" => Ok(Self::Edifact),
            "ENERGIEDATENMANAGEMENT" => Ok(Self::Energiedatenmanagement),
            "FAHRPLANMANAGEMENT" => Ok(Self::Fahrplanmanagement),
            "ALOCAT" => Ok(Self::Alocat),
            "APERAK" => Ok(Self::Aperak),
            "CONTRL" => Ok(Self::Contrl),
            "INVOIC" => Ok(Self::Invoic),
            "MSCONS" => Ok(Self::Mscons),
            "ORDERS" => Ok(Self::Orders),
            "ORDERSP" => Ok(Self::Ordersp),
            "REMADV" => Ok(Self::Remadv),
            "UTILMD" => Ok(Self::Utilmd),
            "GABI" => Ok(Self::Gabi),
            "GELI" => Ok(Self::Geli),
            "GERAETERUECKGABE" => Ok(Self::Geraeterueckgabe),
            "GERAETEWECHSEL" => Ok(Self::Geraetewechsel),
            "GPKE" => Ok(Self::Gpke),
            "INBETRIEBNAHME" => Ok(Self::Inbetriebnahme),
            "KAPAZITAETSMANAGEMENT" => Ok(Self::Kapazitaetsmanagement),
            "KLAERFAELLE" => Ok(Self::Klaerfaelle),
            "LASTGAENGE_RLM" => Ok(Self::LastgaengeRlm),
            "LIEFERANTENRAHMENVERTRAG" => Ok(Self::Lieferantenrahmenvertrag),
            "LIEFERANTENWECHSEL" => Ok(Self::Lieferantenwechsel),
            "MABIS" => Ok(Self::Mabis),
            "MAHNWESEN" => Ok(Self::Mahnwesen),
            "MARKTGEBIETSVERANTWORTLICHER" => Ok(Self::Marktgebietsverantwortlicher),
            "MARKTKOMMUNIKATION" => Ok(Self::Marktkommunikation),
            "MEHR_MINDERMENGEN" => Ok(Self::MehrMindermengen),
            "MSB_MDL" => Ok(Self::MsbMdl),
            "NETZABRECHNUNG" => Ok(Self::Netzabrechnung),
            "NETZENTGELTE" => Ok(Self::Netzentgelte),
            "NETZMANAGEMENT" => Ok(Self::Netzmanagement),
            "RECHT" => Ok(Self::Recht),
            "REGULIERUNGSMANAGEMENT" => Ok(Self::Regulierungsmanagement),
            "REKLAMATIONEN" => Ok(Self::Reklamationen),
            "SPERREN_ENTSPERREN_INKASSO" => Ok(Self::SperrenEntsperrenInkasso),
            "STAMMDATEN" => Ok(Self::Stammdaten),
            "STOERUNGSFAELLE" => Ok(Self::Stoerungsfaelle),
            "TECHNISCHE_FRAGEN" => Ok(Self::TechnischeFragen),
            "UMSTELLUNG_INVOIC" => Ok(Self::UmstellungInvoic),
            "VERSCHLUESSELUNG_SIGNATUR" => Ok(Self::VerschluesselungSignatur),
            "VERTRAGSMANAGEMENT" => Ok(Self::Vertragsmanagement),
            "VERTRIEB" => Ok(Self::Vertrieb),
            "WIM" => Ok(Self::Wim),
            "ZAEHLERSTAENDE_SLP" => Ok(Self::ZaehlerstaendeSlp),
            "ZAHLUNGSVERKEHR" => Ok(Self::Zahlungsverkehr),
            "ZUORDNUNGSVEREINBARUNG" => Ok(Self::Zuordnungsvereinbarung),
            "EINSPEISUNG" => Ok(Self::Einspeisung),
            "BEWEGUNGSDATEN" => Ok(Self::Bewegungsdaten),
            other => Err(crate::error::UnknownVariant::new(other)),
        }
    }
    /// Returns `true` if this value is the forward-compatibility
    /// [`Themengebiet::Unknown`] catch-all (an out-of-schema value).
    pub const fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
    /// Returns `true` if this value is a known, schema-defined variant.
    pub const fn is_known(&self) -> bool {
        !self.is_unknown()
    }
}
impl std::fmt::Display for Themengebiet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_wire())
    }
}
impl AsRef<str> for Themengebiet {
    fn as_ref(&self) -> &str {
        self.as_wire()
    }
}
#[cfg(feature = "versioned")]
impl crate::bo4e_enum_sealed::Sealed for Themengebiet {}
#[cfg(feature = "versioned")]
impl crate::Bo4eEnum for Themengebiet {
    const VARIANTS: &'static [Self] = Self::VARIANTS;
    const COUNT: usize = Self::COUNT;
    fn as_wire(&self) -> &'static str {
        Self::as_wire(self)
    }
    fn from_wire(s: &str) -> Result<Self, crate::error::UnknownVariant> {
        Self::from_wire(s)
    }
    fn is_unknown(&self) -> bool {
        Self::is_unknown(self)
    }
}
#[cfg(feature = "versioned")]
impl crate::Bo4eStrict for Themengebiet {
    fn collect_unknown_enums(&self, path: &str, out: &mut Vec<String>) {
        if self.is_unknown() {
            out.push(path.to_owned());
        }
    }
}
#[cfg(feature = "sqlx")]
impl sqlx::Type<sqlx::Postgres> for Themengebiet {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
/// Encodes as the canonical BO4E wire string, borrowed from `as_wire` — no
/// intermediate `String` or `serde_json::Value` is allocated.
#[cfg(feature = "sqlx")]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Themengebiet {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        let s: &str = self.as_wire();
        <&str as sqlx::Encode<'q, sqlx::Postgres>>::encode_by_ref(&s, buf)
    }
}
/// Decodes leniently, matching the `serde` path: a value the schema does not
/// define becomes [`Themengebiet::Unknown`] rather than a decode error, so a
/// database row written by a newer schema version still reads back.
///
/// Use [`Themengebiet::from_wire`] on a `String` column, or check
/// [`Themengebiet::is_known`], where out-of-schema values must be rejected.
#[cfg(feature = "sqlx")]
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for Themengebiet {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <&str as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        Ok(Self::from_wire(s).unwrap_or(Self::Unknown))
    }
}
#[cfg(test)]
impl proptest::arbitrary::Arbitrary for Themengebiet {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        proptest::sample::select(Self::VARIANTS.to_vec()).boxed()
    }
}
