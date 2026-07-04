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
/// Auflistung sämtlicher existierender Komponenten.
#[non_exhaustive]
pub enum ComTyp {
    #[cfg_attr(feature = "serde", serde(rename = "ADRESSE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ADRESSE"))]
    Adresse,
    #[cfg_attr(feature = "serde", serde(rename = "ANGEBOTSPOSITION"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ANGEBOTSPOSITION"))]
    Angebotsposition,
    #[cfg_attr(feature = "serde", serde(rename = "ANGEBOTSTEIL"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ANGEBOTSTEIL"))]
    Angebotsteil,
    #[cfg_attr(feature = "serde", serde(rename = "ANGEBOTSVARIANTE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ANGEBOTSVARIANTE"))]
    Angebotsvariante,
    #[cfg_attr(feature = "serde", serde(rename = "AUFABSCHLAG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "AUFABSCHLAG"))]
    Aufabschlag,
    #[cfg_attr(feature = "serde", serde(rename = "AUFABSCHLAGPROORT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "AUFABSCHLAGPROORT"))]
    Aufabschlagproort,
    #[cfg_attr(feature = "serde", serde(rename = "AUFABSCHLAGREGIONAL"))]
    #[cfg_attr(feature = "strum", strum(serialize = "AUFABSCHLAGREGIONAL"))]
    Aufabschlagregional,
    #[cfg_attr(feature = "serde", serde(rename = "AUFABSCHLAGSTAFFELPROORT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "AUFABSCHLAGSTAFFELPROORT"))]
    Aufabschlagstaffelproort,
    #[cfg_attr(feature = "serde", serde(rename = "AUSSCHREIBUNGSDETAIL"))]
    #[cfg_attr(feature = "strum", strum(serialize = "AUSSCHREIBUNGSDETAIL"))]
    Ausschreibungsdetail,
    #[cfg_attr(feature = "serde", serde(rename = "AUSSCHREIBUNGSLOS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "AUSSCHREIBUNGSLOS"))]
    Ausschreibungslos,
    #[cfg_attr(feature = "serde", serde(rename = "BETRAG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BETRAG"))]
    Betrag,
    #[cfg_attr(feature = "serde", serde(rename = "DIENSTLEISTUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "DIENSTLEISTUNG"))]
    Dienstleistung,
    #[cfg_attr(feature = "serde", serde(rename = "ENERGIEHERKUNFT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ENERGIEHERKUNFT"))]
    Energieherkunft,
    #[cfg_attr(feature = "serde", serde(rename = "ENERGIEMIX"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ENERGIEMIX"))]
    Energiemix,
    #[cfg_attr(feature = "serde", serde(rename = "FREMDKOSTENBLOCK"))]
    #[cfg_attr(feature = "strum", strum(serialize = "FREMDKOSTENBLOCK"))]
    Fremdkostenblock,
    #[cfg_attr(feature = "serde", serde(rename = "FREMDKOSTENPOSITION"))]
    #[cfg_attr(feature = "strum", strum(serialize = "FREMDKOSTENPOSITION"))]
    Fremdkostenposition,
    #[cfg_attr(feature = "serde", serde(rename = "GEOKOORDINATEN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GEOKOORDINATEN"))]
    Geokoordinaten,
    #[cfg_attr(feature = "serde", serde(rename = "KATASTERADRESSE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KATASTERADRESSE"))]
    Katasteradresse,
    #[cfg_attr(feature = "serde", serde(rename = "KONFIGURATIONSPRODUKT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KONFIGURATIONSPRODUKT"))]
    Konfigurationsprodukt,
    #[cfg_attr(feature = "serde", serde(rename = "KONTAKTWEG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KONTAKTWEG"))]
    Kontaktweg,
    #[cfg_attr(feature = "serde", serde(rename = "KONZESSIONSABGABE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KONZESSIONSABGABE"))]
    Konzessionsabgabe,
    #[cfg_attr(feature = "serde", serde(rename = "KOSTENBLOCK"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KOSTENBLOCK"))]
    Kostenblock,
    #[cfg_attr(feature = "serde", serde(rename = "KOSTENPOSITION"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KOSTENPOSITION"))]
    Kostenposition,
    #[cfg_attr(feature = "serde", serde(rename = "KRITERIUMWERT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KRITERIUMWERT"))]
    Kriteriumwert,
    #[cfg_attr(feature = "serde", serde(rename = "LASTPROFIL"))]
    #[cfg_attr(feature = "strum", strum(serialize = "LASTPROFIL"))]
    Lastprofil,
    #[cfg_attr(feature = "serde", serde(rename = "MARKTGEBIETINFO"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MARKTGEBIETINFO"))]
    Marktgebietinfo,
    #[cfg_attr(feature = "serde", serde(rename = "MENGE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MENGE"))]
    Menge,
    #[cfg_attr(feature = "serde", serde(rename = "POSITIONSAUFABSCHLAG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "POSITIONSAUFABSCHLAG"))]
    Positionsaufabschlag,
    #[cfg_attr(feature = "serde", serde(rename = "PREIS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "PREIS"))]
    Preis,
    #[cfg_attr(feature = "serde", serde(rename = "PREISGARANTIE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "PREISGARANTIE"))]
    Preisgarantie,
    #[cfg_attr(feature = "serde", serde(rename = "PREISPOSITION"))]
    #[cfg_attr(feature = "strum", strum(serialize = "PREISPOSITION"))]
    Preisposition,
    #[cfg_attr(feature = "serde", serde(rename = "PREISSTAFFEL"))]
    #[cfg_attr(feature = "strum", strum(serialize = "PREISSTAFFEL"))]
    Preisstaffel,
    #[cfg_attr(feature = "serde", serde(rename = "RECHNUNGSPOSITION"))]
    #[cfg_attr(feature = "strum", strum(serialize = "RECHNUNGSPOSITION"))]
    Rechnungsposition,
    #[cfg_attr(feature = "serde", serde(rename = "REGIONALEGUELTIGKEIT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "REGIONALEGUELTIGKEIT"))]
    Regionalegueltigkeit,
    #[cfg_attr(feature = "serde", serde(rename = "REGIONALEPREISGARANTIE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "REGIONALEPREISGARANTIE"))]
    Regionalepreisgarantie,
    #[cfg_attr(feature = "serde", serde(rename = "REGIONALEPREISSTAFFEL"))]
    #[cfg_attr(feature = "strum", strum(serialize = "REGIONALEPREISSTAFFEL"))]
    Regionalepreisstaffel,
    #[cfg_attr(feature = "serde", serde(rename = "REGIONALERAUFABSCHLAG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "REGIONALERAUFABSCHLAG"))]
    Regionaleraufabschlag,
    #[cfg_attr(feature = "serde", serde(rename = "REGIONALETARIFPREISPOSITION"))]
    #[cfg_attr(feature = "strum", strum(serialize = "REGIONALETARIFPREISPOSITION"))]
    Regionaletarifpreisposition,
    #[cfg_attr(feature = "serde", serde(rename = "REGIONSKRITERIUM"))]
    #[cfg_attr(feature = "strum", strum(serialize = "REGIONSKRITERIUM"))]
    Regionskriterium,
    #[cfg_attr(feature = "serde", serde(rename = "SIGMOIDPARAMETER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SIGMOIDPARAMETER"))]
    Sigmoidparameter,
    #[cfg_attr(feature = "serde", serde(rename = "STANDORTEIGENSCHAFTENGAS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "STANDORTEIGENSCHAFTENGAS"))]
    Standorteigenschaftengas,
    #[cfg_attr(feature = "serde", serde(rename = "STANDORTEIGENSCHAFTENSTROM"))]
    #[cfg_attr(feature = "strum", strum(serialize = "STANDORTEIGENSCHAFTENSTROM"))]
    Standorteigenschaftenstrom,
    #[cfg_attr(feature = "serde", serde(rename = "STEUERBETRAG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "STEUERBETRAG"))]
    Steuerbetrag,
    #[cfg_attr(feature = "serde", serde(rename = "TAGESPARAMETER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TAGESPARAMETER"))]
    Tagesparameter,
    #[cfg_attr(feature = "serde", serde(rename = "TARIFBERECHNUNGSPARAMETER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TARIFBERECHNUNGSPARAMETER"))]
    Tarifberechnungsparameter,
    #[cfg_attr(feature = "serde", serde(rename = "TARIFEINSCHRAENKUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TARIFEINSCHRAENKUNG"))]
    Tarifeinschraenkung,
    #[cfg_attr(feature = "serde", serde(rename = "TARIFPREIS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TARIFPREIS"))]
    Tarifpreis,
    #[cfg_attr(feature = "serde", serde(rename = "TARIFPREISPOSITION"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TARIFPREISPOSITION"))]
    Tarifpreisposition,
    #[cfg_attr(feature = "serde", serde(rename = "TARIFPREISPOSITIONPROORT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TARIFPREISPOSITIONPROORT"))]
    Tarifpreispositionproort,
    #[cfg_attr(feature = "serde", serde(rename = "TARIFPREISSTAFFELPROORT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TARIFPREISSTAFFELPROORT"))]
    Tarifpreisstaffelproort,
    #[cfg_attr(feature = "serde", serde(rename = "UNTERSCHRIFT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UNTERSCHRIFT"))]
    Unterschrift,
    #[cfg_attr(feature = "serde", serde(rename = "VERBRAUCH"))]
    #[cfg_attr(feature = "strum", strum(serialize = "VERBRAUCH"))]
    Verbrauch,
    #[cfg_attr(feature = "serde", serde(rename = "VERTRAGSKONDITIONEN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "VERTRAGSKONDITIONEN"))]
    Vertragskonditionen,
    #[cfg_attr(feature = "serde", serde(rename = "VERTRAGSTEIL"))]
    #[cfg_attr(feature = "strum", strum(serialize = "VERTRAGSTEIL"))]
    Vertragsteil,
    #[cfg_attr(feature = "serde", serde(rename = "VERWENDUNGSZWECKPROMARKTROLLE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "VERWENDUNGSZWECKPROMARKTROLLE"))]
    Verwendungszweckpromarktrolle,
    #[cfg_attr(feature = "serde", serde(rename = "ZAEHLWERK"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ZAEHLWERK"))]
    Zaehlwerk,
    #[cfg_attr(feature = "serde", serde(rename = "ZAEHLZEITREGISTER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ZAEHLZEITREGISTER"))]
    Zaehlzeitregister,
    #[cfg_attr(feature = "serde", serde(rename = "ZEITRAUM"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ZEITRAUM"))]
    Zeitraum,
    #[cfg_attr(feature = "serde", serde(rename = "ZEITREIHENWERT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ZEITREIHENWERT"))]
    Zeitreihenwert,
    #[cfg_attr(feature = "serde", serde(rename = "ZUSTAENDIGKEIT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ZUSTAENDIGKEIT"))]
    Zustaendigkeit,
    /// Unknown or future variant — produced when deserializing a value
    /// that is not yet known to this version of the library.
    #[cfg_attr(feature = "serde", serde(other, rename = "UNKNOWN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UNKNOWN"))]
    Unknown,
}
impl ComTyp {
    /// Returns an iterator over all **known** variants of `ComTyp`.
    ///
    /// Unlike [`strum::IntoEnumIterator`] which includes the [`ComTyp::Unknown`]
    /// catch-all, this method yields only variants that correspond to values defined
    /// in the BO4E schema.  Use this when building dropdowns, lookup tables, or
    /// generating reports that should only include valid schema values.
    ///
    /// # Example
    /// ```rust,ignore
    /// for v in ComTyp::iter_known() {
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
impl sqlx::Type<sqlx::Postgres> for ComTyp {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
/// Strum fast path: `AsRef<str>` returns the canonical string without a
/// `serde_json::Value` intermediate, saving an allocation per encode (M-07).
#[cfg(all(feature = "sqlx", feature = "json", feature = "strum"))]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for ComTyp {
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
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for ComTyp {
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
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for ComTyp {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <String as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        serde_json::from_value(serde_json::Value::String(s))
            .map_err(|e| Box::new(e) as sqlx::error::BoxDynError)
    }
}
#[cfg(all(test, feature = "strum"))]
impl proptest::arbitrary::Arbitrary for ComTyp {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        use strum::IntoEnumIterator as _;
        let variants: Vec<Self> = Self::iter().collect();
        proptest::sample::select(variants).boxed()
    }
}
