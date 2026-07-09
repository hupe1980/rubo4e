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
/// Eine Aufzählung zur Einordnung für die Höhe der Konzessionsabgabe.
#[non_exhaustive]
pub enum KundengruppeKa {
    #[cfg_attr(feature = "serde", serde(rename = "S_SCHWACHLAST"))]
    #[cfg_attr(feature = "strum", strum(serialize = "S_SCHWACHLAST"))]
    SSchwachlast,
    #[cfg_attr(feature = "serde", serde(rename = "S_TARIF_25000"))]
    #[cfg_attr(feature = "strum", strum(serialize = "S_TARIF_25000"))]
    STarif25000,
    #[cfg_attr(feature = "serde", serde(rename = "S_TARIF_100000"))]
    #[cfg_attr(feature = "strum", strum(serialize = "S_TARIF_100000"))]
    STarif100000,
    #[cfg_attr(feature = "serde", serde(rename = "S_TARIF_500000"))]
    #[cfg_attr(feature = "strum", strum(serialize = "S_TARIF_500000"))]
    STarif500000,
    #[cfg_attr(feature = "serde", serde(rename = "S_TARIF_G_500000"))]
    #[cfg_attr(feature = "strum", strum(serialize = "S_TARIF_G_500000"))]
    STarifG500000,
    #[cfg_attr(feature = "serde", serde(rename = "S_SONDERKUNDE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "S_SONDERKUNDE"))]
    SSonderkunde,
    #[cfg_attr(feature = "serde", serde(rename = "G_KOWA_25000"))]
    #[cfg_attr(feature = "strum", strum(serialize = "G_KOWA_25000"))]
    GKowa25000,
    #[cfg_attr(feature = "serde", serde(rename = "G_KOWA_100000"))]
    #[cfg_attr(feature = "strum", strum(serialize = "G_KOWA_100000"))]
    GKowa100000,
    #[cfg_attr(feature = "serde", serde(rename = "G_KOWA_500000"))]
    #[cfg_attr(feature = "strum", strum(serialize = "G_KOWA_500000"))]
    GKowa500000,
    #[cfg_attr(feature = "serde", serde(rename = "G_KOWA_G_500000"))]
    #[cfg_attr(feature = "strum", strum(serialize = "G_KOWA_G_500000"))]
    GKowaG500000,
    #[cfg_attr(feature = "serde", serde(rename = "G_TARIF_25000"))]
    #[cfg_attr(feature = "strum", strum(serialize = "G_TARIF_25000"))]
    GTarif25000,
    #[cfg_attr(feature = "serde", serde(rename = "G_TARIF_100000"))]
    #[cfg_attr(feature = "strum", strum(serialize = "G_TARIF_100000"))]
    GTarif100000,
    #[cfg_attr(feature = "serde", serde(rename = "G_TARIF_500000"))]
    #[cfg_attr(feature = "strum", strum(serialize = "G_TARIF_500000"))]
    GTarif500000,
    #[cfg_attr(feature = "serde", serde(rename = "G_TARIF_G_500000"))]
    #[cfg_attr(feature = "strum", strum(serialize = "G_TARIF_G_500000"))]
    GTarifG500000,
    #[cfg_attr(feature = "serde", serde(rename = "G_SONDERKUNDE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "G_SONDERKUNDE"))]
    GSonderkunde,
    #[cfg_attr(feature = "serde", serde(rename = "SONDER_KAS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SONDER_KAS"))]
    SonderKas,
    #[cfg_attr(feature = "serde", serde(rename = "SONDER_SAS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SONDER_SAS"))]
    SonderSas,
    #[cfg_attr(feature = "serde", serde(rename = "SONDER_TAS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SONDER_TAS"))]
    SonderTas,
    #[cfg_attr(feature = "serde", serde(rename = "SONDER_TKS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SONDER_TKS"))]
    SonderTks,
    #[cfg_attr(feature = "serde", serde(rename = "SONDER_TSS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SONDER_TSS"))]
    SonderTss,
    /// Unknown or future variant — produced when deserializing a value
    /// that is not yet known to this version of the library.
    #[cfg_attr(feature = "serde", serde(other, rename = "UNKNOWN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UNKNOWN"))]
    Unknown,
}
impl KundengruppeKa {
    /// Returns an iterator over all **known** variants of `KundengruppeKa`.
    ///
    /// Unlike [`strum::IntoEnumIterator`] which includes the [`KundengruppeKa::Unknown`]
    /// catch-all, this method yields only variants that correspond to values defined
    /// in the BO4E schema.  Use this when building dropdowns, lookup tables, or
    /// generating reports that should only include valid schema values.
    ///
    /// # Example
    /// ```rust,ignore
    /// for v in KundengruppeKa::iter_known() {
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
impl sqlx::Type<sqlx::Postgres> for KundengruppeKa {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
/// Strum fast path: `AsRef<str>` returns the canonical string without a
/// `serde_json::Value` intermediate, saving an allocation per encode (M-07).
#[cfg(all(feature = "sqlx", feature = "json", feature = "strum"))]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for KundengruppeKa {
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
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for KundengruppeKa {
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
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for KundengruppeKa {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <String as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        serde_json::from_value(serde_json::Value::String(s))
            .map_err(|e| Box::new(e) as sqlx::error::BoxDynError)
    }
}
#[cfg(all(test, feature = "strum"))]
impl proptest::arbitrary::Arbitrary for KundengruppeKa {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        use strum::IntoEnumIterator as _;
        let variants: Vec<Self> = Self::iter().collect();
        proptest::sample::select(variants).boxed()
    }
}
