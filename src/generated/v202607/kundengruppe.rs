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
/// Kundengruppe für eine Marktlokation (orientiert sich an den Standard-Lastprofilen).
#[non_exhaustive]
pub enum Kundengruppe {
    #[cfg_attr(feature = "serde", serde(rename = "RLM"))]
    #[cfg_attr(feature = "strum", strum(serialize = "RLM"))]
    Rlm,
    #[cfg_attr(feature = "serde", serde(rename = "RLM_KOMMUNAL"))]
    #[cfg_attr(feature = "strum", strum(serialize = "RLM_KOMMUNAL"))]
    RlmKommunal,
    #[cfg_attr(feature = "serde", serde(rename = "SLP_KOMMUNAL"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SLP_KOMMUNAL"))]
    SlpKommunal,
    #[cfg_attr(feature = "serde", serde(rename = "SLP_S_G0"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SLP_S_G0"))]
    SlpSG0,
    #[cfg_attr(feature = "serde", serde(rename = "SLP_S_G1"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SLP_S_G1"))]
    SlpSG1,
    #[cfg_attr(feature = "serde", serde(rename = "SLP_S_G2"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SLP_S_G2"))]
    SlpSG2,
    #[cfg_attr(feature = "serde", serde(rename = "SLP_S_G3"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SLP_S_G3"))]
    SlpSG3,
    #[cfg_attr(feature = "serde", serde(rename = "SLP_S_G4"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SLP_S_G4"))]
    SlpSG4,
    #[cfg_attr(feature = "serde", serde(rename = "SLP_S_G5"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SLP_S_G5"))]
    SlpSG5,
    #[cfg_attr(feature = "serde", serde(rename = "SLP_S_G6"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SLP_S_G6"))]
    SlpSG6,
    #[cfg_attr(feature = "serde", serde(rename = "SLP_S_G7"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SLP_S_G7"))]
    SlpSG7,
    #[cfg_attr(feature = "serde", serde(rename = "SLP_S_L0"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SLP_S_L0"))]
    SlpSL0,
    #[cfg_attr(feature = "serde", serde(rename = "SLP_S_L1"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SLP_S_L1"))]
    SlpSL1,
    #[cfg_attr(feature = "serde", serde(rename = "SLP_S_L2"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SLP_S_L2"))]
    SlpSL2,
    #[cfg_attr(feature = "serde", serde(rename = "SLP_S_H0"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SLP_S_H0"))]
    SlpSH0,
    #[cfg_attr(feature = "serde", serde(rename = "SLP_S_SB"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SLP_S_SB"))]
    SlpSSb,
    #[cfg_attr(feature = "serde", serde(rename = "SLP_S_HZ"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SLP_S_HZ"))]
    SlpSHz,
    #[cfg_attr(feature = "serde", serde(rename = "SLP_S_WP"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SLP_S_WP"))]
    SlpSWp,
    #[cfg_attr(feature = "serde", serde(rename = "SLP_S_EM"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SLP_S_EM"))]
    SlpSEm,
    #[cfg_attr(feature = "serde", serde(rename = "SLP_S_HZ_GEM"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SLP_S_HZ_GEM"))]
    SlpSHzGem,
    #[cfg_attr(feature = "serde", serde(rename = "SLP_G_GKO"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SLP_G_GKO"))]
    SlpGGko,
    #[cfg_attr(feature = "serde", serde(rename = "SLP_G_STANDARD"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SLP_G_STANDARD"))]
    SlpGStandard,
    #[cfg_attr(feature = "serde", serde(rename = "SLP_G_GHA"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SLP_G_GHA"))]
    SlpGGha,
    #[cfg_attr(feature = "serde", serde(rename = "SLP_G_GMK"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SLP_G_GMK"))]
    SlpGGmk,
    #[cfg_attr(feature = "serde", serde(rename = "SLP_G_GBD"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SLP_G_GBD"))]
    SlpGGbd,
    #[cfg_attr(feature = "serde", serde(rename = "SLP_G_GGA"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SLP_G_GGA"))]
    SlpGGga,
    #[cfg_attr(feature = "serde", serde(rename = "SLP_G_GBH"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SLP_G_GBH"))]
    SlpGGbh,
    #[cfg_attr(feature = "serde", serde(rename = "SLP_G_GBA"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SLP_G_GBA"))]
    SlpGGba,
    #[cfg_attr(feature = "serde", serde(rename = "SLP_G_GWA"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SLP_G_GWA"))]
    SlpGGwa,
    #[cfg_attr(feature = "serde", serde(rename = "SLP_G_GGB"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SLP_G_GGB"))]
    SlpGGgb,
    #[cfg_attr(feature = "serde", serde(rename = "SLP_G_GPD"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SLP_G_GPD"))]
    SlpGGpd,
    #[cfg_attr(feature = "serde", serde(rename = "SLP_G_GMF"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SLP_G_GMF"))]
    SlpGGmf,
    #[cfg_attr(feature = "serde", serde(rename = "SLP_G_HEF"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SLP_G_HEF"))]
    SlpGHef,
    #[cfg_attr(feature = "serde", serde(rename = "SLP_G_HMF"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SLP_G_HMF"))]
    SlpGHmf,
    #[cfg_attr(feature = "serde", serde(rename = "SLP_G_HKO"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SLP_G_HKO"))]
    SlpGHko,
    /// Unknown or future variant — produced when deserializing a value
    /// that is not yet known to this version of the library.
    #[cfg_attr(feature = "serde", serde(other, rename = "UNKNOWN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UNKNOWN"))]
    Unknown,
}
impl Kundengruppe {
    /// Returns an iterator over all **known** variants of `Kundengruppe`.
    ///
    /// Unlike [`strum::IntoEnumIterator`] which includes the [`Kundengruppe::Unknown`]
    /// catch-all, this method yields only variants that correspond to values defined
    /// in the BO4E schema.  Use this when building dropdowns, lookup tables, or
    /// generating reports that should only include valid schema values.
    ///
    /// # Example
    /// ```rust,ignore
    /// for v in Kundengruppe::iter_known() {
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
impl sqlx::Type<sqlx::Postgres> for Kundengruppe {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
/// Strum fast path: `AsRef<str>` returns the canonical string without a
/// `serde_json::Value` intermediate, saving an allocation per encode (M-07).
#[cfg(all(feature = "sqlx", feature = "json", feature = "strum"))]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Kundengruppe {
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
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Kundengruppe {
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
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for Kundengruppe {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <String as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        serde_json::from_value(serde_json::Value::String(s))
            .map_err(|e| Box::new(e) as sqlx::error::BoxDynError)
    }
}
#[cfg(all(test, feature = "strum"))]
impl proptest::arbitrary::Arbitrary for Kundengruppe {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        use strum::IntoEnumIterator as _;
        let variants: Vec<Self> = Self::iter().collect();
        proptest::sample::select(variants).boxed()
    }
}
