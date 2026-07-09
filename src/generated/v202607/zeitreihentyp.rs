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
/// Codes der Summenzeitreihentypen.
///
/// Die nachfolgenden Codes sind in DE7111 zu nutzen:
/// <https://www.edi-energy.de/index.php?id=38&tx_bdew_bdew%5Buid%5D=695&tx_bdew_bdew%5Baction%5D=download>
/// &tx_bdew_bdew%5Bcontroller%5D=Dokument&cHash=67782e05d8b0f75fbe3a0e1801d07ed0
#[non_exhaustive]
pub enum Zeitreihentyp {
    #[cfg_attr(feature = "serde", serde(rename = "EGS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "EGS"))]
    Egs,
    #[cfg_attr(feature = "serde", serde(rename = "LGS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "LGS"))]
    Lgs,
    #[cfg_attr(feature = "serde", serde(rename = "NZR"))]
    #[cfg_attr(feature = "strum", strum(serialize = "NZR"))]
    Nzr,
    #[cfg_attr(feature = "serde", serde(rename = "SES"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SES"))]
    Ses,
    #[cfg_attr(feature = "serde", serde(rename = "SLS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SLS"))]
    Sls,
    #[cfg_attr(feature = "serde", serde(rename = "TES"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TES"))]
    Tes,
    #[cfg_attr(feature = "serde", serde(rename = "TLS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TLS"))]
    Tls,
    #[cfg_attr(feature = "serde", serde(rename = "SLS_TLS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SLS_TLS"))]
    SlsTls,
    #[cfg_attr(feature = "serde", serde(rename = "SES_TES"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SES_TES"))]
    SesTes,
    /// Unknown or future variant — produced when deserializing a value
    /// that is not yet known to this version of the library.
    #[cfg_attr(feature = "serde", serde(other, rename = "UNKNOWN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UNKNOWN"))]
    Unknown,
}
impl Zeitreihentyp {
    /// Returns an iterator over all **known** variants of `Zeitreihentyp`.
    ///
    /// Unlike [`strum::IntoEnumIterator`] which includes the [`Zeitreihentyp::Unknown`]
    /// catch-all, this method yields only variants that correspond to values defined
    /// in the BO4E schema.  Use this when building dropdowns, lookup tables, or
    /// generating reports that should only include valid schema values.
    ///
    /// # Example
    /// ```rust,ignore
    /// for v in Zeitreihentyp::iter_known() {
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
impl sqlx::Type<sqlx::Postgres> for Zeitreihentyp {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
/// Strum fast path: `AsRef<str>` returns the canonical string without a
/// `serde_json::Value` intermediate, saving an allocation per encode (M-07).
#[cfg(all(feature = "sqlx", feature = "json", feature = "strum"))]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Zeitreihentyp {
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
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Zeitreihentyp {
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
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for Zeitreihentyp {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <String as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        serde_json::from_value(serde_json::Value::String(s))
            .map_err(|e| Box::new(e) as sqlx::error::BoxDynError)
    }
}
#[cfg(all(test, feature = "strum"))]
impl proptest::arbitrary::Arbitrary for Zeitreihentyp {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        use strum::IntoEnumIterator as _;
        let variants: Vec<Self> = Self::iter().collect();
        proptest::sample::select(variants).boxed()
    }
}
