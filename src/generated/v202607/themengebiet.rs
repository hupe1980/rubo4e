#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(
        strum::Display,
        strum::EnumString,
        strum::EnumIter,
        strum::IntoStaticStr,
        strum::AsRefStr
    )
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
    /// Returns an iterator over all **known** variants of `Themengebiet`.
    ///
    /// Unlike [`strum::IntoEnumIterator`] which includes the [`Themengebiet::Unknown`]
    /// catch-all, this method yields only variants that correspond to values defined
    /// in the BO4E schema.  Use this when building dropdowns, lookup tables, or
    /// generating reports that should only include valid schema values.
    ///
    /// # Example
    /// ```rust,ignore
    /// for v in Themengebiet::iter_known() {
    ///     println!("{v}");
    /// }
    /// ```
    #[cfg(feature = "strum")]
    pub fn iter_known() -> impl Iterator<Item = Self> {
        use strum::IntoEnumIterator as _;
        Self::iter().filter(|v| !matches!(v, Self::Unknown))
    }
}
#[cfg(all(feature = "sqlx", feature = "json"))]
impl sqlx::Type<sqlx::Postgres> for Themengebiet {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
/// Strum fast path: `AsRef<str>` returns the canonical string without a
/// `serde_json::Value` intermediate, saving an allocation per encode (M-07).
#[cfg(all(feature = "sqlx", feature = "json", feature = "strum"))]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Themengebiet {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        let s: &str = self.as_ref();
        <&str as sqlx::Encode<'q, sqlx::Postgres>>::encode_by_ref(&s, buf)
    }
}
/// Fallback when `strum` is not active: serialize via `serde_json`.
#[cfg(all(feature = "sqlx", feature = "json", not(feature = "strum")))]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Themengebiet {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        let s = serde_json::to_value(self)?
            .as_str()
            .ok_or("enum variant did not serialize to a JSON string")?
            .to_owned();
        <String as sqlx::Encode<'q, sqlx::Postgres>>::encode_by_ref(&s, buf)
    }
}
#[cfg(all(feature = "sqlx", feature = "json"))]
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for Themengebiet {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <String as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        serde_json::from_value(serde_json::Value::String(s))
            .map_err(|e| Box::new(e) as sqlx::error::BoxDynError)
    }
}
#[cfg(all(test, feature = "strum"))]
impl proptest::arbitrary::Arbitrary for Themengebiet {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        use strum::IntoEnumIterator as _;
        let variants: Vec<Self> = Self::iter().collect();
        proptest::sample::select(variants).boxed()
    }
}
