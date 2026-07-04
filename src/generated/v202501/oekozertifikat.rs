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
/// Zertifikate für Ökostrom von verschiedenen Herausgebern.
#[non_exhaustive]
pub enum Oekozertifikat {
    #[cfg_attr(feature = "serde", serde(rename = "CMS_EE01"))]
    #[cfg_attr(feature = "strum", strum(serialize = "CMS_EE01"))]
    CmsEe01,
    #[cfg_attr(feature = "serde", serde(rename = "CMS_EE02"))]
    #[cfg_attr(feature = "strum", strum(serialize = "CMS_EE02"))]
    CmsEe02,
    #[cfg_attr(feature = "serde", serde(rename = "EECS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "EECS"))]
    Eecs,
    #[cfg_attr(feature = "serde", serde(rename = "FRAUNHOFER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "FRAUNHOFER"))]
    Fraunhofer,
    #[cfg_attr(feature = "serde", serde(rename = "BET"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BET"))]
    Bet,
    #[cfg_attr(feature = "serde", serde(rename = "KLIMA_INVEST"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KLIMA_INVEST"))]
    KlimaInvest,
    #[cfg_attr(feature = "serde", serde(rename = "LGA"))]
    #[cfg_attr(feature = "strum", strum(serialize = "LGA"))]
    Lga,
    #[cfg_attr(feature = "serde", serde(rename = "FREIBERG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "FREIBERG"))]
    Freiberg,
    #[cfg_attr(feature = "serde", serde(rename = "RECS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "RECS"))]
    Recs,
    #[cfg_attr(feature = "serde", serde(rename = "REGS_EGL"))]
    #[cfg_attr(feature = "strum", strum(serialize = "REGS_EGL"))]
    RegsEgl,
    #[cfg_attr(feature = "serde", serde(rename = "TUEV"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TUEV"))]
    Tuev,
    #[cfg_attr(feature = "serde", serde(rename = "TUEV_HESSEN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TUEV_HESSEN"))]
    TuevHessen,
    #[cfg_attr(feature = "serde", serde(rename = "TUEV_NORD"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TUEV_NORD"))]
    TuevNord,
    #[cfg_attr(feature = "serde", serde(rename = "TUEV_RHEINLAND"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TUEV_RHEINLAND"))]
    TuevRheinland,
    #[cfg_attr(feature = "serde", serde(rename = "TUEV_SUED"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TUEV_SUED"))]
    TuevSued,
    #[cfg_attr(feature = "serde", serde(rename = "TUEV_SUED_EE01"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TUEV_SUED_EE01"))]
    TuevSuedEe01,
    #[cfg_attr(feature = "serde", serde(rename = "TUEV_SUED_EE02"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TUEV_SUED_EE02"))]
    TuevSuedEe02,
    /// Unknown or future variant — produced when deserializing a value
    /// that is not yet known to this version of the library.
    #[cfg_attr(feature = "serde", serde(other, rename = "UNKNOWN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UNKNOWN"))]
    Unknown,
}
impl Oekozertifikat {
    /// Returns an iterator over all **known** variants of `Oekozertifikat`.
    ///
    /// Unlike [`strum::IntoEnumIterator`] which includes the [`Oekozertifikat::Unknown`]
    /// catch-all, this method yields only variants that correspond to values defined
    /// in the BO4E schema.  Use this when building dropdowns, lookup tables, or
    /// generating reports that should only include valid schema values.
    ///
    /// # Example
    /// ```rust,ignore
    /// for v in Oekozertifikat::iter_known() {
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
impl sqlx::Type<sqlx::Postgres> for Oekozertifikat {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
/// Strum fast path: `AsRef<str>` returns the canonical string without a
/// `serde_json::Value` intermediate, saving an allocation per encode (M-07).
#[cfg(all(feature = "sqlx", feature = "json", feature = "strum"))]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Oekozertifikat {
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
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Oekozertifikat {
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
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for Oekozertifikat {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <String as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        serde_json::from_value(serde_json::Value::String(s))
            .map_err(|e| Box::new(e) as sqlx::error::BoxDynError)
    }
}
#[cfg(all(test, feature = "strum"))]
impl proptest::arbitrary::Arbitrary for Oekozertifikat {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        use strum::IntoEnumIterator as _;
        let variants: Vec<Self> = Self::iter().collect();
        proptest::sample::select(variants).boxed()
    }
}
