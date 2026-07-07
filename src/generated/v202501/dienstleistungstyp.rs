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
/// Auflistung möglicher abzurechnender Dienstleistungen.
#[non_exhaustive]
pub enum Dienstleistungstyp {
    #[cfg_attr(feature = "serde", serde(rename = "DATENBEREITSTELLUNG_TAEGLICH"))]
    #[cfg_attr(feature = "strum", strum(serialize = "DATENBEREITSTELLUNG_TAEGLICH"))]
    DatenbereitstellungTaeglich,
    #[cfg_attr(feature = "serde", serde(rename = "DATENBEREITSTELLUNG_WOECHENTLICH"))]
    #[cfg_attr(
        feature = "strum",
        strum(serialize = "DATENBEREITSTELLUNG_WOECHENTLICH")
    )]
    DatenbereitstellungWoechentlich,
    #[cfg_attr(feature = "serde", serde(rename = "DATENBEREITSTELLUNG_MONATLICH"))]
    #[cfg_attr(feature = "strum", strum(serialize = "DATENBEREITSTELLUNG_MONATLICH"))]
    DatenbereitstellungMonatlich,
    #[cfg_attr(feature = "serde", serde(rename = "DATENBEREITSTELLUNG_JAEHRLICH"))]
    #[cfg_attr(feature = "strum", strum(serialize = "DATENBEREITSTELLUNG_JAEHRLICH"))]
    DatenbereitstellungJaehrlich,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "DATENBEREITSTELLUNG_HISTORISCHE_LG")
    )]
    #[cfg_attr(
        feature = "strum",
        strum(serialize = "DATENBEREITSTELLUNG_HISTORISCHE_LG")
    )]
    DatenbereitstellungHistorischeLg,
    #[cfg_attr(feature = "serde", serde(rename = "DATENBEREITSTELLUNG_STUENDLICH"))]
    #[cfg_attr(feature = "strum", strum(serialize = "DATENBEREITSTELLUNG_STUENDLICH"))]
    DatenbereitstellungStuendlich,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "DATENBEREITSTELLUNG_VIERTELJAEHRLICH")
    )]
    #[cfg_attr(
        feature = "strum",
        strum(serialize = "DATENBEREITSTELLUNG_VIERTELJAEHRLICH")
    )]
    DatenbereitstellungVierteljaehrlich,
    #[cfg_attr(feature = "serde", serde(rename = "DATENBEREITSTELLUNG_HALBJAEHRLICH"))]
    #[cfg_attr(
        feature = "strum",
        strum(serialize = "DATENBEREITSTELLUNG_HALBJAEHRLICH")
    )]
    DatenbereitstellungHalbjaehrlich,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "DATENBEREITSTELLUNG_MONATLICH_ZUSAETZLICH")
    )]
    #[cfg_attr(
        feature = "strum",
        strum(serialize = "DATENBEREITSTELLUNG_MONATLICH_ZUSAETZLICH")
    )]
    DatenbereitstellungMonatlichZusaetzlich,
    #[cfg_attr(feature = "serde", serde(rename = "DATENBEREITSTELLUNG_EINMALIG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "DATENBEREITSTELLUNG_EINMALIG"))]
    DatenbereitstellungEinmalig,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "AUSLESUNG_2X_TAEGLICH_FERNAUSLESUNG")
    )]
    #[cfg_attr(
        feature = "strum",
        strum(serialize = "AUSLESUNG_2X_TAEGLICH_FERNAUSLESUNG")
    )]
    Auslesung2xTaeglichFernauslesung,
    #[cfg_attr(feature = "serde", serde(rename = "AUSLESUNG_TAEGLICH_FERNAUSLESUNG"))]
    #[cfg_attr(
        feature = "strum",
        strum(serialize = "AUSLESUNG_TAEGLICH_FERNAUSLESUNG")
    )]
    AuslesungTaeglichFernauslesung,
    #[cfg_attr(feature = "serde", serde(rename = "AUSLESUNG_MANUELL_MSB"))]
    #[cfg_attr(feature = "strum", strum(serialize = "AUSLESUNG_MANUELL_MSB"))]
    AuslesungManuellMsb,
    #[cfg_attr(feature = "serde", serde(rename = "AUSLESUNG_MONATLICH_FERNAUSLESUNG"))]
    #[cfg_attr(
        feature = "strum",
        strum(serialize = "AUSLESUNG_MONATLICH_FERNAUSLESUNG")
    )]
    AuslesungMonatlichFernauslesung,
    #[cfg_attr(feature = "serde", serde(rename = "AUSLESUNG_JAEHRLICH_FERNAUSLESUNG"))]
    #[cfg_attr(
        feature = "strum",
        strum(serialize = "AUSLESUNG_JAEHRLICH_FERNAUSLESUNG")
    )]
    AuslesungJaehrlichFernauslesung,
    #[cfg_attr(feature = "serde", serde(rename = "AUSLESUNG_MDE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "AUSLESUNG_MDE"))]
    AuslesungMde,
    #[cfg_attr(feature = "serde", serde(rename = "ABLESUNG_MONATLICH"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ABLESUNG_MONATLICH"))]
    AblesungMonatlich,
    #[cfg_attr(feature = "serde", serde(rename = "ABLESUNG_VIERTELJAEHRLICH"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ABLESUNG_VIERTELJAEHRLICH"))]
    AblesungVierteljaehrlich,
    #[cfg_attr(feature = "serde", serde(rename = "ABLESUNG_HALBJAEHRLICH"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ABLESUNG_HALBJAEHRLICH"))]
    AblesungHalbjaehrlich,
    #[cfg_attr(feature = "serde", serde(rename = "ABLESUNG_JAEHRLICH"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ABLESUNG_JAEHRLICH"))]
    AblesungJaehrlich,
    #[cfg_attr(feature = "serde", serde(rename = "AUSLESUNG_FERNAUSLESUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "AUSLESUNG_FERNAUSLESUNG"))]
    AuslesungFernauslesung,
    #[cfg_attr(feature = "serde", serde(rename = "ABLESUNG_ZUSAETZLICH_MSB"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ABLESUNG_ZUSAETZLICH_MSB"))]
    AblesungZusaetzlichMsb,
    #[cfg_attr(feature = "serde", serde(rename = "ABLESUNG_ZUSAETZLICH_KUNDE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ABLESUNG_ZUSAETZLICH_KUNDE"))]
    AblesungZusaetzlichKunde,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "AUSLESUNG_FERNAUSLESUNG_ZUSAETZLICH_MSB")
    )]
    #[cfg_attr(
        feature = "strum",
        strum(serialize = "AUSLESUNG_FERNAUSLESUNG_ZUSAETZLICH_MSB")
    )]
    AuslesungFernauslesungZusaetzlichMsb,
    #[cfg_attr(feature = "serde", serde(rename = "AUSLESUNG_MOATLICH_FERNAUSLESUNG"))]
    #[cfg_attr(
        feature = "strum",
        strum(serialize = "AUSLESUNG_MOATLICH_FERNAUSLESUNG")
    )]
    AuslesungMoatlichFernauslesung,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "AUSLESUNG_STUENDLICH_FERNAUSLESUNG")
    )]
    #[cfg_attr(
        feature = "strum",
        strum(serialize = "AUSLESUNG_STUENDLICH_FERNAUSLESUNG")
    )]
    AuslesungStuendlichFernauslesung,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "AUSLESUNG_TEMPERATURMENGENUMWERTER")
    )]
    #[cfg_attr(
        feature = "strum",
        strum(serialize = "AUSLESUNG_TEMPERATURMENGENUMWERTER")
    )]
    AuslesungTemperaturmengenumwerter,
    #[cfg_attr(feature = "serde", serde(rename = "AUSLESUNG_ZUSTANDSMENGENUMWERTER"))]
    #[cfg_attr(
        feature = "strum",
        strum(serialize = "AUSLESUNG_ZUSTANDSMENGENUMWERTER")
    )]
    AuslesungZustandsmengenumwerter,
    #[cfg_attr(feature = "serde", serde(rename = "AUSLESUNG_SYSTEMMENGENUMWERTER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "AUSLESUNG_SYSTEMMENGENUMWERTER"))]
    AuslesungSystemmengenumwerter,
    #[cfg_attr(feature = "serde", serde(rename = "AUSLESUNG_VORGANG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "AUSLESUNG_VORGANG"))]
    AuslesungVorgang,
    #[cfg_attr(feature = "serde", serde(rename = "AUSLESUNG_KOMPAKTMENGENUMWERTER"))]
    #[cfg_attr(
        feature = "strum",
        strum(serialize = "AUSLESUNG_KOMPAKTMENGENUMWERTER")
    )]
    AuslesungKompaktmengenumwerter,
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
    /// Unknown or future variant — produced when deserializing a value
    /// that is not yet known to this version of the library.
    #[cfg_attr(feature = "serde", serde(other, rename = "UNKNOWN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UNKNOWN"))]
    Unknown,
}
impl Dienstleistungstyp {
    /// Returns an iterator over all **known** variants of `Dienstleistungstyp`.
    ///
    /// Unlike [`strum::IntoEnumIterator`] which includes the [`Dienstleistungstyp::Unknown`]
    /// catch-all, this method yields only variants that correspond to values defined
    /// in the BO4E schema.  Use this when building dropdowns, lookup tables, or
    /// generating reports that should only include valid schema values.
    ///
    /// # Example
    /// ```rust,ignore
    /// for v in Dienstleistungstyp::iter_known() {
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
impl sqlx::Type<sqlx::Postgres> for Dienstleistungstyp {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
/// Strum fast path: `AsRef<str>` returns the canonical string without a
/// `serde_json::Value` intermediate, saving an allocation per encode (M-07).
#[cfg(all(feature = "sqlx", feature = "json", feature = "strum"))]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Dienstleistungstyp {
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
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Dienstleistungstyp {
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
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for Dienstleistungstyp {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <String as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        serde_json::from_value(serde_json::Value::String(s))
            .map_err(|e| Box::new(e) as sqlx::error::BoxDynError)
    }
}
#[cfg(all(test, feature = "strum"))]
impl proptest::arbitrary::Arbitrary for Dienstleistungstyp {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        use strum::IntoEnumIterator as _;
        let variants: Vec<Self> = Self::iter().collect();
        proptest::sample::select(variants).boxed()
    }
}
