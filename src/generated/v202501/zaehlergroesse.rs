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
/// Auflistung möglicher Größen von Zählern
#[non_exhaustive]
pub enum Zaehlergroesse {
    #[cfg_attr(feature = "serde", serde(rename = "G2KOMMA5"))]
    #[cfg_attr(feature = "strum", strum(serialize = "G2KOMMA5"))]
    G2komma5,
    #[cfg_attr(feature = "serde", serde(rename = "G4"))]
    #[cfg_attr(feature = "strum", strum(serialize = "G4"))]
    G4,
    #[cfg_attr(feature = "serde", serde(rename = "G6"))]
    #[cfg_attr(feature = "strum", strum(serialize = "G6"))]
    G6,
    #[cfg_attr(feature = "serde", serde(rename = "G10"))]
    #[cfg_attr(feature = "strum", strum(serialize = "G10"))]
    G10,
    #[cfg_attr(feature = "serde", serde(rename = "G16"))]
    #[cfg_attr(feature = "strum", strum(serialize = "G16"))]
    G16,
    #[cfg_attr(feature = "serde", serde(rename = "G25"))]
    #[cfg_attr(feature = "strum", strum(serialize = "G25"))]
    G25,
    #[cfg_attr(feature = "serde", serde(rename = "G40"))]
    #[cfg_attr(feature = "strum", strum(serialize = "G40"))]
    G40,
    #[cfg_attr(feature = "serde", serde(rename = "G65"))]
    #[cfg_attr(feature = "strum", strum(serialize = "G65"))]
    G65,
    #[cfg_attr(feature = "serde", serde(rename = "G100"))]
    #[cfg_attr(feature = "strum", strum(serialize = "G100"))]
    G100,
    #[cfg_attr(feature = "serde", serde(rename = "G160"))]
    #[cfg_attr(feature = "strum", strum(serialize = "G160"))]
    G160,
    #[cfg_attr(feature = "serde", serde(rename = "G250"))]
    #[cfg_attr(feature = "strum", strum(serialize = "G250"))]
    G250,
    #[cfg_attr(feature = "serde", serde(rename = "G400"))]
    #[cfg_attr(feature = "strum", strum(serialize = "G400"))]
    G400,
    #[cfg_attr(feature = "serde", serde(rename = "G650"))]
    #[cfg_attr(feature = "strum", strum(serialize = "G650"))]
    G650,
    #[cfg_attr(feature = "serde", serde(rename = "G1000"))]
    #[cfg_attr(feature = "strum", strum(serialize = "G1000"))]
    G1000,
    #[cfg_attr(feature = "serde", serde(rename = "G1600"))]
    #[cfg_attr(feature = "strum", strum(serialize = "G1600"))]
    G1600,
    #[cfg_attr(feature = "serde", serde(rename = "G2500"))]
    #[cfg_attr(feature = "strum", strum(serialize = "G2500"))]
    G2500,
    #[cfg_attr(feature = "serde", serde(rename = "G4000"))]
    #[cfg_attr(feature = "strum", strum(serialize = "G4000"))]
    G4000,
    #[cfg_attr(feature = "serde", serde(rename = "G6500"))]
    #[cfg_attr(feature = "strum", strum(serialize = "G6500"))]
    G6500,
    #[cfg_attr(feature = "serde", serde(rename = "G10000"))]
    #[cfg_attr(feature = "strum", strum(serialize = "G10000"))]
    G10000,
    #[cfg_attr(feature = "serde", serde(rename = "G12500"))]
    #[cfg_attr(feature = "strum", strum(serialize = "G12500"))]
    G12500,
    #[cfg_attr(feature = "serde", serde(rename = "G16000"))]
    #[cfg_attr(feature = "strum", strum(serialize = "G16000"))]
    G16000,
    /// Unknown or future variant — produced when deserializing a value
    /// that is not yet known to this version of the library.
    #[cfg_attr(feature = "serde", serde(other, rename = "UNKNOWN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UNKNOWN"))]
    Unknown,
}
impl Zaehlergroesse {
    /// Returns an iterator over all **known** variants of `Zaehlergroesse`.
    ///
    /// Unlike [`strum::IntoEnumIterator`] which includes the [`Zaehlergroesse::Unknown`]
    /// catch-all, this method yields only variants that correspond to values defined
    /// in the BO4E schema.  Use this when building dropdowns, lookup tables, or
    /// generating reports that should only include valid schema values.
    ///
    /// # Example
    /// ```rust,ignore
    /// for v in Zaehlergroesse::iter_known() {
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
impl sqlx::Type<sqlx::Postgres> for Zaehlergroesse {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
/// Strum fast path: `AsRef<str>` returns the canonical string without a
/// `serde_json::Value` intermediate, saving an allocation per encode (M-07).
#[cfg(all(feature = "sqlx", feature = "json", feature = "strum"))]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Zaehlergroesse {
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
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Zaehlergroesse {
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
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for Zaehlergroesse {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <String as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        serde_json::from_value(serde_json::Value::String(s))
            .map_err(|e| Box::new(e) as sqlx::error::BoxDynError)
    }
}
#[cfg(all(test, feature = "strum"))]
impl proptest::arbitrary::Arbitrary for Zaehlergroesse {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        use strum::IntoEnumIterator as _;
        let variants: Vec<Self> = Self::iter().collect();
        proptest::sample::select(variants).boxed()
    }
}
