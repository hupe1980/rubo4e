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
/// Auflistung der Typen von Endkunden. Daraus kann das Verbrauchsprofil abgeleitet werden.
#[non_exhaustive]
pub enum Kundentyp {
    #[cfg_attr(feature = "serde", serde(rename = "GEWERBE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GEWERBE"))]
    Gewerbe,
    #[cfg_attr(feature = "serde", serde(rename = "PRIVAT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "PRIVAT"))]
    Privat,
    #[cfg_attr(feature = "serde", serde(rename = "LANDWIRT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "LANDWIRT"))]
    Landwirt,
    #[cfg_attr(feature = "serde", serde(rename = "SONSTIGE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SONSTIGE"))]
    Sonstige,
    #[cfg_attr(feature = "serde", serde(rename = "HAUSHALT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "HAUSHALT"))]
    Haushalt,
    #[cfg_attr(feature = "serde", serde(rename = "DIREKTHEIZUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "DIREKTHEIZUNG"))]
    Direktheizung,
    #[cfg_attr(feature = "serde", serde(rename = "GEMEINSCHAFT_MFH"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GEMEINSCHAFT_MFH"))]
    GemeinschaftMfh,
    #[cfg_attr(feature = "serde", serde(rename = "KIRCHE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KIRCHE"))]
    Kirche,
    #[cfg_attr(feature = "serde", serde(rename = "KWK"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KWK"))]
    Kwk,
    #[cfg_attr(feature = "serde", serde(rename = "LADESAEULE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "LADESAEULE"))]
    Ladesaeule,
    #[cfg_attr(feature = "serde", serde(rename = "BELEUCHTUNG_OEFFENTLICH"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BELEUCHTUNG_OEFFENTLICH"))]
    BeleuchtungOeffentlich,
    #[cfg_attr(feature = "serde", serde(rename = "BELEUCHTUNG_STRASSE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BELEUCHTUNG_STRASSE"))]
    BeleuchtungStrasse,
    #[cfg_attr(feature = "serde", serde(rename = "SPEICHERHEIZUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SPEICHERHEIZUNG"))]
    Speicherheizung,
    #[cfg_attr(feature = "serde", serde(rename = "UNTERBR_EINRICHTUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UNTERBR_EINRICHTUNG"))]
    UnterbrEinrichtung,
    #[cfg_attr(feature = "serde", serde(rename = "WAERMEPUMPE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "WAERMEPUMPE"))]
    Waermepumpe,
    /// Unknown or future variant — produced when deserializing a value
    /// that is not yet known to this version of the library.
    #[cfg_attr(feature = "serde", serde(other, rename = "UNKNOWN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UNKNOWN"))]
    Unknown,
}
impl Kundentyp {
    /// Returns an iterator over all **known** variants of `Kundentyp`.
    ///
    /// Unlike [`strum::IntoEnumIterator`] which includes the [`Kundentyp::Unknown`]
    /// catch-all, this method yields only variants that correspond to values defined
    /// in the BO4E schema.  Use this when building dropdowns, lookup tables, or
    /// generating reports that should only include valid schema values.
    ///
    /// # Example
    /// ```rust,ignore
    /// for v in Kundentyp::iter_known() {
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
impl sqlx::Type<sqlx::Postgres> for Kundentyp {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
/// Strum fast path: `AsRef<str>` returns the canonical string without a
/// `serde_json::Value` intermediate, saving an allocation per encode (M-07).
#[cfg(all(feature = "sqlx", feature = "json", feature = "strum"))]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Kundentyp {
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
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Kundentyp {
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
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for Kundentyp {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <String as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        serde_json::from_value(serde_json::Value::String(s))
            .map_err(|e| Box::new(e) as sqlx::error::BoxDynError)
    }
}
#[cfg(all(test, feature = "strum"))]
impl proptest::arbitrary::Arbitrary for Kundentyp {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        use strum::IntoEnumIterator as _;
        let variants: Vec<Self> = Self::iter().collect();
        proptest::sample::select(variants).boxed()
    }
}
