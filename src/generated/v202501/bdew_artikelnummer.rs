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
/// BDEW Artikelnummern
#[non_exhaustive]
pub enum BdewArtikelnummer {
    #[cfg_attr(feature = "serde", serde(rename = "LEISTUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "LEISTUNG"))]
    Leistung,
    #[cfg_attr(feature = "serde", serde(rename = "LEISTUNG_PAUSCHAL"))]
    #[cfg_attr(feature = "strum", strum(serialize = "LEISTUNG_PAUSCHAL"))]
    LeistungPauschal,
    #[cfg_attr(feature = "serde", serde(rename = "GRUNDPREIS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GRUNDPREIS"))]
    Grundpreis,
    #[cfg_attr(feature = "serde", serde(rename = "REGELENERGIE_ARBEIT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "REGELENERGIE_ARBEIT"))]
    RegelenergieArbeit,
    #[cfg_attr(feature = "serde", serde(rename = "REGELENERGIE_LEISTUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "REGELENERGIE_LEISTUNG"))]
    RegelenergieLeistung,
    #[cfg_attr(feature = "serde", serde(rename = "NOTSTROMLIEFERUNG_ARBEIT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "NOTSTROMLIEFERUNG_ARBEIT"))]
    NotstromlieferungArbeit,
    #[cfg_attr(feature = "serde", serde(rename = "NOTSTROMLIEFERUNG_LEISTUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "NOTSTROMLIEFERUNG_LEISTUNG"))]
    NotstromlieferungLeistung,
    #[cfg_attr(feature = "serde", serde(rename = "RESERVENETZKAPAZITAET"))]
    #[cfg_attr(feature = "strum", strum(serialize = "RESERVENETZKAPAZITAET"))]
    Reservenetzkapazitaet,
    #[cfg_attr(feature = "serde", serde(rename = "RESERVELEISTUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "RESERVELEISTUNG"))]
    Reserveleistung,
    #[cfg_attr(feature = "serde", serde(rename = "ZUSAETZLICHE_ABLESUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ZUSAETZLICHE_ABLESUNG"))]
    ZusaetzlicheAblesung,
    #[cfg_attr(feature = "serde", serde(rename = "PRUEFGEBUEHREN_AUSSERPLANMAESSIG"))]
    #[cfg_attr(
        feature = "strum",
        strum(serialize = "PRUEFGEBUEHREN_AUSSERPLANMAESSIG")
    )]
    PruefgebuehrenAusserplanmaessig,
    #[cfg_attr(feature = "serde", serde(rename = "WIRKARBEIT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "WIRKARBEIT"))]
    Wirkarbeit,
    #[cfg_attr(feature = "serde", serde(rename = "SINGULAER_GENUTZTE_BETRIEBSMITTEL"))]
    #[cfg_attr(
        feature = "strum",
        strum(serialize = "SINGULAER_GENUTZTE_BETRIEBSMITTEL")
    )]
    SingulaerGenutzteBetriebsmittel,
    #[cfg_attr(feature = "serde", serde(rename = "ABGABE_KWKG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ABGABE_KWKG"))]
    AbgabeKwkg,
    #[cfg_attr(feature = "serde", serde(rename = "ABSCHLAG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ABSCHLAG"))]
    Abschlag,
    #[cfg_attr(feature = "serde", serde(rename = "KONZESSIONSABGABE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KONZESSIONSABGABE"))]
    Konzessionsabgabe,
    #[cfg_attr(feature = "serde", serde(rename = "ENTGELT_FERNAUSLESUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ENTGELT_FERNAUSLESUNG"))]
    EntgeltFernauslesung,
    #[cfg_attr(feature = "serde", serde(rename = "UNTERMESSUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UNTERMESSUNG"))]
    Untermessung,
    #[cfg_attr(feature = "serde", serde(rename = "BLINDMEHRARBEIT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BLINDMEHRARBEIT"))]
    Blindmehrarbeit,
    #[cfg_attr(feature = "serde", serde(rename = "ENTGELT_ABRECHNUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ENTGELT_ABRECHNUNG"))]
    EntgeltAbrechnung,
    #[cfg_attr(feature = "serde", serde(rename = "SPERRKOSTEN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SPERRKOSTEN"))]
    Sperrkosten,
    #[cfg_attr(feature = "serde", serde(rename = "ENTSPERRKOSTEN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ENTSPERRKOSTEN"))]
    Entsperrkosten,
    #[cfg_attr(feature = "serde", serde(rename = "MAHNKOSTEN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MAHNKOSTEN"))]
    Mahnkosten,
    #[cfg_attr(feature = "serde", serde(rename = "MEHR_MINDERMENGEN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MEHR_MINDERMENGEN"))]
    MehrMindermengen,
    #[cfg_attr(feature = "serde", serde(rename = "INKASSOKOSTEN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "INKASSOKOSTEN"))]
    Inkassokosten,
    #[cfg_attr(feature = "serde", serde(rename = "BLINDMEHRLEISTUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BLINDMEHRLEISTUNG"))]
    Blindmehrleistung,
    #[cfg_attr(feature = "serde", serde(rename = "ENTGELT_MESSUNG_ABLESUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ENTGELT_MESSUNG_ABLESUNG"))]
    EntgeltMessungAblesung,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "ENTGELT_EINBAU_BETRIEB_WARTUNG_MESSTECHNIK")
    )]
    #[cfg_attr(
        feature = "strum",
        strum(serialize = "ENTGELT_EINBAU_BETRIEB_WARTUNG_MESSTECHNIK")
    )]
    EntgeltEinbauBetriebWartungMesstechnik,
    #[cfg_attr(feature = "serde", serde(rename = "AUSGLEICHSENERGIE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "AUSGLEICHSENERGIE"))]
    Ausgleichsenergie,
    #[cfg_attr(feature = "serde", serde(rename = "ZAEHLEINRICHTUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ZAEHLEINRICHTUNG"))]
    Zaehleinrichtung,
    #[cfg_attr(feature = "serde", serde(rename = "WANDLER_MENGENUMWERTER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "WANDLER_MENGENUMWERTER"))]
    WandlerMengenumwerter,
    #[cfg_attr(feature = "serde", serde(rename = "KOMMUNIKATIONSEINRICHTUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KOMMUNIKATIONSEINRICHTUNG"))]
    Kommunikationseinrichtung,
    #[cfg_attr(feature = "serde", serde(rename = "TECHNISCHE_STEUEREINRICHTUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TECHNISCHE_STEUEREINRICHTUNG"))]
    TechnischeSteuereinrichtung,
    #[cfg_attr(feature = "serde", serde(rename = "PARAGRAF_19_STROM_NEV_UMLAGE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "PARAGRAF_19_STROM_NEV_UMLAGE"))]
    Paragraf19StromNevUmlage,
    #[cfg_attr(feature = "serde", serde(rename = "BEFESTIGUNGSEINRICHTUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BEFESTIGUNGSEINRICHTUNG"))]
    Befestigungseinrichtung,
    #[cfg_attr(feature = "serde", serde(rename = "OFFSHORE_HAFTUNGSUMLAGE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "OFFSHORE_HAFTUNGSUMLAGE"))]
    OffshoreHaftungsumlage,
    #[cfg_attr(feature = "serde", serde(rename = "FIXE_ARBEITSENTGELTKOMPONENTE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "FIXE_ARBEITSENTGELTKOMPONENTE"))]
    FixeArbeitsentgeltkomponente,
    #[cfg_attr(feature = "serde", serde(rename = "FIXE_LEISTUNGSENTGELTKOMPONENTE"))]
    #[cfg_attr(
        feature = "strum",
        strum(serialize = "FIXE_LEISTUNGSENTGELTKOMPONENTE")
    )]
    FixeLeistungsentgeltkomponente,
    #[cfg_attr(feature = "serde", serde(rename = "UMLAGE_ABSCHALTBARE_LASTEN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UMLAGE_ABSCHALTBARE_LASTEN"))]
    UmlageAbschaltbareLasten,
    #[cfg_attr(feature = "serde", serde(rename = "MEHRMENGE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MEHRMENGE"))]
    Mehrmenge,
    #[cfg_attr(feature = "serde", serde(rename = "MINDERMENGE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MINDERMENGE"))]
    Mindermenge,
    #[cfg_attr(feature = "serde", serde(rename = "ENERGIESTEUER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ENERGIESTEUER"))]
    Energiesteuer,
    #[cfg_attr(feature = "serde", serde(rename = "SMARTMETER_GATEWAY"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SMARTMETER_GATEWAY"))]
    SmartmeterGateway,
    #[cfg_attr(feature = "serde", serde(rename = "STEUERBOX"))]
    #[cfg_attr(feature = "strum", strum(serialize = "STEUERBOX"))]
    Steuerbox,
    #[cfg_attr(feature = "serde", serde(rename = "MSB_INKL_MESSUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MSB_INKL_MESSUNG"))]
    MsbInklMessung,
    #[cfg_attr(feature = "serde", serde(rename = "AUSGLEICHSENERGIE_UNTERDECKUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "AUSGLEICHSENERGIE_UNTERDECKUNG"))]
    AusgleichsenergieUnterdeckung,
    /// Unknown or future variant — produced when deserializing a value
    /// that is not yet known to this version of the library.
    #[cfg_attr(feature = "serde", serde(other, rename = "UNKNOWN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UNKNOWN"))]
    Unknown,
}
impl BdewArtikelnummer {
    /// Returns an iterator over all **known** variants of `BdewArtikelnummer`.
    ///
    /// Unlike [`strum::IntoEnumIterator`] which includes the [`BdewArtikelnummer::Unknown`]
    /// catch-all, this method yields only variants that correspond to values defined
    /// in the BO4E schema.  Use this when building dropdowns, lookup tables, or
    /// generating reports that should only include valid schema values.
    ///
    /// # Example
    /// ```rust,ignore
    /// for v in BdewArtikelnummer::iter_known() {
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
impl sqlx::Type<sqlx::Postgres> for BdewArtikelnummer {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
/// Strum fast path: `AsRef<str>` returns the canonical string without a
/// `serde_json::Value` intermediate, saving an allocation per encode (M-07).
#[cfg(all(feature = "sqlx", feature = "json", feature = "strum"))]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for BdewArtikelnummer {
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
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for BdewArtikelnummer {
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
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for BdewArtikelnummer {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <String as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        serde_json::from_value(serde_json::Value::String(s))
            .map_err(|e| Box::new(e) as sqlx::error::BoxDynError)
    }
}
#[cfg(all(test, feature = "strum"))]
impl proptest::arbitrary::Arbitrary for BdewArtikelnummer {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        use strum::IntoEnumIterator as _;
        let variants: Vec<Self> = Self::iter().collect();
        proptest::sample::select(variants).boxed()
    }
}
