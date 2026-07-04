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
/// Diese Rollen kann ein Marktteilnehmer einnehmen.
#[non_exhaustive]
pub enum Marktrolle {
    #[cfg_attr(feature = "serde", serde(rename = "BTR"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BTR"))]
    Btr,
    #[cfg_attr(feature = "serde", serde(rename = "BIKO"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BIKO"))]
    Biko,
    #[cfg_attr(feature = "serde", serde(rename = "BKV"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BKV"))]
    Bkv,
    #[cfg_attr(feature = "serde", serde(rename = "DP"))]
    #[cfg_attr(feature = "strum", strum(serialize = "DP"))]
    Dp,
    #[cfg_attr(feature = "serde", serde(rename = "EIV"))]
    #[cfg_attr(feature = "strum", strum(serialize = "EIV"))]
    Eiv,
    #[cfg_attr(feature = "serde", serde(rename = "ESA"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ESA"))]
    Esa,
    #[cfg_attr(feature = "serde", serde(rename = "KN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KN"))]
    Kn,
    #[cfg_attr(feature = "serde", serde(rename = "LF"))]
    #[cfg_attr(feature = "strum", strum(serialize = "LF"))]
    Lf,
    #[cfg_attr(feature = "serde", serde(rename = "MGV"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MGV"))]
    Mgv,
    #[cfg_attr(feature = "serde", serde(rename = "MSB"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MSB"))]
    Msb,
    #[cfg_attr(feature = "serde", serde(rename = "NB"))]
    #[cfg_attr(feature = "strum", strum(serialize = "NB"))]
    Nb,
    #[cfg_attr(feature = "serde", serde(rename = "RB"))]
    #[cfg_attr(feature = "strum", strum(serialize = "RB"))]
    Rb,
    #[cfg_attr(feature = "serde", serde(rename = "UENB"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UENB"))]
    Uenb,
    /// Unknown or future variant — produced when deserializing a value
    /// that is not yet known to this version of the library.
    #[cfg_attr(feature = "serde", serde(other, rename = "UNKNOWN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UNKNOWN"))]
    Unknown,
}
impl Marktrolle {
    /// Returns an iterator over all **known** variants of `Marktrolle`.
    ///
    /// Unlike [`strum::IntoEnumIterator`] which includes the [`Marktrolle::Unknown`]
    /// catch-all, this method yields only variants that correspond to values defined
    /// in the BO4E schema.  Use this when building dropdowns, lookup tables, or
    /// generating reports that should only include valid schema values.
    ///
    /// # Example
    /// ```rust,ignore
    /// for v in Marktrolle::iter_known() {
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
impl sqlx::Type<sqlx::Postgres> for Marktrolle {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
/// Strum fast path: `AsRef<str>` returns the canonical string without a
/// `serde_json::Value` intermediate, saving an allocation per encode (M-07).
#[cfg(all(feature = "sqlx", feature = "json", feature = "strum"))]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Marktrolle {
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
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Marktrolle {
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
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for Marktrolle {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <String as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        serde_json::from_value(serde_json::Value::String(s))
            .map_err(|e| Box::new(e) as sqlx::error::BoxDynError)
    }
}
#[cfg(all(test, feature = "strum"))]
impl proptest::arbitrary::Arbitrary for Marktrolle {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        use strum::IntoEnumIterator as _;
        let variants: Vec<Self> = Self::iter().collect();
        proptest::sample::select(variants).boxed()
    }
}
