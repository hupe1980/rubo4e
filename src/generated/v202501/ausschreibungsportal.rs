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
/// Aufzählung der unterstützten Ausschreibungsportale.
#[non_exhaustive]
pub enum Ausschreibungsportal {
    #[cfg_attr(feature = "serde", serde(rename = "ENPORTAL"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ENPORTAL"))]
    Enportal,
    #[cfg_attr(feature = "serde", serde(rename = "ENERGIE_AGENTUR"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ENERGIE_AGENTUR"))]
    EnergieAgentur,
    #[cfg_attr(feature = "serde", serde(rename = "BMWI"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BMWI"))]
    Bmwi,
    #[cfg_attr(feature = "serde", serde(rename = "ENERGIE_HANDELSPLATZ"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ENERGIE_HANDELSPLATZ"))]
    EnergieHandelsplatz,
    #[cfg_attr(feature = "serde", serde(rename = "BUND"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BUND"))]
    Bund,
    #[cfg_attr(feature = "serde", serde(rename = "VERA_ONLINE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "VERA_ONLINE"))]
    VeraOnline,
    #[cfg_attr(feature = "serde", serde(rename = "ISPEX"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ISPEX"))]
    Ispex,
    #[cfg_attr(feature = "serde", serde(rename = "ENERGIEMARKTPLATZ"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ENERGIEMARKTPLATZ"))]
    Energiemarktplatz,
    #[cfg_attr(feature = "serde", serde(rename = "EVERGABE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "EVERGABE"))]
    Evergabe,
    #[cfg_attr(feature = "serde", serde(rename = "DTAD"))]
    #[cfg_attr(feature = "strum", strum(serialize = "DTAD"))]
    Dtad,
    /// Unknown or future variant — produced when deserializing a value
    /// that is not yet known to this version of the library.
    #[cfg_attr(feature = "serde", serde(other, rename = "UNKNOWN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UNKNOWN"))]
    Unknown,
}
impl Ausschreibungsportal {
    /// Returns an iterator over all **known** variants of `Ausschreibungsportal`.
    ///
    /// Unlike [`strum::IntoEnumIterator`] which includes the [`Ausschreibungsportal::Unknown`]
    /// catch-all, this method yields only variants that correspond to values defined
    /// in the BO4E schema.  Use this when building dropdowns, lookup tables, or
    /// generating reports that should only include valid schema values.
    ///
    /// # Example
    /// ```rust,ignore
    /// for v in Ausschreibungsportal::iter_known() {
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
impl sqlx::Type<sqlx::Postgres> for Ausschreibungsportal {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
/// Strum fast path: `AsRef<str>` returns the canonical string without a
/// `serde_json::Value` intermediate, saving an allocation per encode (M-07).
#[cfg(all(feature = "sqlx", feature = "json", feature = "strum"))]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Ausschreibungsportal {
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
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Ausschreibungsportal {
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
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for Ausschreibungsportal {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <String as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        serde_json::from_value(serde_json::Value::String(s))
            .map_err(|e| Box::new(e) as sqlx::error::BoxDynError)
    }
}
#[cfg(all(test, feature = "strum"))]
impl proptest::arbitrary::Arbitrary for Ausschreibungsportal {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        use strum::IntoEnumIterator as _;
        let variants: Vec<Self> = Self::iter().collect();
        proptest::sample::select(variants).boxed()
    }
}
