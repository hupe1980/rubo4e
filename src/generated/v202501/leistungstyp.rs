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
#[non_exhaustive]
pub enum Leistungstyp {
    #[cfg_attr(feature = "serde", serde(rename = "ARBEITSPREIS_WIRKARBEIT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ARBEITSPREIS_WIRKARBEIT"))]
    ArbeitspreisWirkarbeit,
    #[cfg_attr(feature = "serde", serde(rename = "LEISTUNGSPREIS_WIRKLEISTUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "LEISTUNGSPREIS_WIRKLEISTUNG"))]
    LeistungspreisWirkleistung,
    #[cfg_attr(feature = "serde", serde(rename = "ARBEITSPREIS_BLINDARBEIT_IND"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ARBEITSPREIS_BLINDARBEIT_IND"))]
    ArbeitspreisBlindarbeitInd,
    #[cfg_attr(feature = "serde", serde(rename = "ARBEITSPREIS_BLINDARBEIT_KAP"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ARBEITSPREIS_BLINDARBEIT_KAP"))]
    ArbeitspreisBlindarbeitKap,
    #[cfg_attr(feature = "serde", serde(rename = "GRUNDPREIS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GRUNDPREIS"))]
    Grundpreis,
    #[cfg_attr(feature = "serde", serde(rename = "GRUNDPREIS_ARBEIT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GRUNDPREIS_ARBEIT"))]
    GrundpreisArbeit,
    #[cfg_attr(feature = "serde", serde(rename = "GRUNDPREIS_LEISTUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GRUNDPREIS_LEISTUNG"))]
    GrundpreisLeistung,
    #[cfg_attr(feature = "serde", serde(rename = "MEHRMINDERMENGE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MEHRMINDERMENGE"))]
    Mehrmindermenge,
    #[cfg_attr(feature = "serde", serde(rename = "MESSSTELLENBETRIEB"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MESSSTELLENBETRIEB"))]
    Messstellenbetrieb,
    #[cfg_attr(feature = "serde", serde(rename = "MESSDIENSTLEISTUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MESSDIENSTLEISTUNG"))]
    Messdienstleistung,
    #[cfg_attr(feature = "serde", serde(rename = "MESSDIENSTLEISTUNG_INKL_MESSUNG"))]
    #[cfg_attr(
        feature = "strum",
        strum(serialize = "MESSDIENSTLEISTUNG_INKL_MESSUNG")
    )]
    MessdienstleistungInklMessung,
    #[cfg_attr(feature = "serde", serde(rename = "ABRECHNUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ABRECHNUNG"))]
    Abrechnung,
    #[cfg_attr(feature = "serde", serde(rename = "KONZESSIONS_ABGABE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KONZESSIONS_ABGABE"))]
    KonzessionsAbgabe,
    #[cfg_attr(feature = "serde", serde(rename = "KWK_UMLAGE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KWK_UMLAGE"))]
    KwkUmlage,
    #[cfg_attr(feature = "serde", serde(rename = "OFFSHORE_UMLAGE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "OFFSHORE_UMLAGE"))]
    OffshoreUmlage,
    #[cfg_attr(feature = "serde", serde(rename = "ABLAV_UMLAGE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ABLAV_UMLAGE"))]
    AblavUmlage,
    #[cfg_attr(feature = "serde", serde(rename = "SONDERKUNDEN_UMLAGE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SONDERKUNDEN_UMLAGE"))]
    SonderkundenUmlage,
    #[cfg_attr(feature = "serde", serde(rename = "REGELENERGIE_UMLAGE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "REGELENERGIE_UMLAGE"))]
    RegelenergieUmlage,
    #[cfg_attr(feature = "serde", serde(rename = "BILANZIERUNG_UMLAGE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BILANZIERUNG_UMLAGE"))]
    BilanzierungUmlage,
    #[cfg_attr(feature = "serde", serde(rename = "AUSLESUNG_ZUSAETZLICH"))]
    #[cfg_attr(feature = "strum", strum(serialize = "AUSLESUNG_ZUSAETZLICH"))]
    AuslesungZusaetzlich,
    #[cfg_attr(feature = "serde", serde(rename = "ABLESUNG_ZUSAETZLICH"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ABLESUNG_ZUSAETZLICH"))]
    AblesungZusaetzlich,
    #[cfg_attr(feature = "serde", serde(rename = "ABRECHNUNG_ZUSAETZLICH"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ABRECHNUNG_ZUSAETZLICH"))]
    AbrechnungZusaetzlich,
    #[cfg_attr(feature = "serde", serde(rename = "SPERRUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SPERRUNG"))]
    Sperrung,
    #[cfg_attr(feature = "serde", serde(rename = "ENTSPERRUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ENTSPERRUNG"))]
    Entsperrung,
    #[cfg_attr(feature = "serde", serde(rename = "MAHNKOSTEN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MAHNKOSTEN"))]
    Mahnkosten,
    #[cfg_attr(feature = "serde", serde(rename = "INKASSOKOSTEN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "INKASSOKOSTEN"))]
    Inkassokosten,
    #[cfg_attr(feature = "serde", serde(rename = "EEG_UMLAGE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "EEG_UMLAGE"))]
    EegUmlage,
    #[cfg_attr(feature = "serde", serde(rename = "ENERGIESTEUER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ENERGIESTEUER"))]
    Energiesteuer,
    #[cfg_attr(feature = "serde", serde(rename = "NETZPREIS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "NETZPREIS"))]
    Netzpreis,
    #[cfg_attr(feature = "serde", serde(rename = "MESSPREIS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MESSPREIS"))]
    Messpreis,
    #[cfg_attr(feature = "serde", serde(rename = "SONSTIGER_PREIS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SONSTIGER_PREIS"))]
    SonstigerPreis,
    #[cfg_attr(feature = "serde", serde(rename = "DIENSTLEISTUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "DIENSTLEISTUNG"))]
    Dienstleistung,
    /// Unknown or future variant — produced when deserializing a value
    /// that is not yet known to this version of the library.
    #[cfg_attr(feature = "serde", serde(other, rename = "UNKNOWN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UNKNOWN"))]
    Unknown,
}
impl Leistungstyp {
    /// Returns an iterator over all **known** variants of `Leistungstyp`.
    ///
    /// Unlike [`strum::IntoEnumIterator`] which includes the [`Leistungstyp::Unknown`]
    /// catch-all, this method yields only variants that correspond to values defined
    /// in the BO4E schema.  Use this when building dropdowns, lookup tables, or
    /// generating reports that should only include valid schema values.
    ///
    /// # Example
    /// ```rust,ignore
    /// for v in Leistungstyp::iter_known() {
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
impl sqlx::Type<sqlx::Postgres> for Leistungstyp {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
/// Strum fast path: `AsRef<str>` returns the canonical string without a
/// `serde_json::Value` intermediate, saving an allocation per encode (M-07).
#[cfg(all(feature = "sqlx", feature = "json", feature = "strum"))]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Leistungstyp {
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
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Leistungstyp {
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
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for Leistungstyp {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <String as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        serde_json::from_value(serde_json::Value::String(s))
            .map_err(|e| Box::new(e) as sqlx::error::BoxDynError)
    }
}
#[cfg(all(test, feature = "strum"))]
impl proptest::arbitrary::Arbitrary for Leistungstyp {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        use strum::IntoEnumIterator as _;
        let variants: Vec<Self> = Self::iter().collect();
        proptest::sample::select(variants).boxed()
    }
}
