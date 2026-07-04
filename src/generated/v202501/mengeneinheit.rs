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
/// Einheit: Messgrößen, die per Messung oder Vorgabe ermittelt werden können.
#[non_exhaustive]
pub enum Mengeneinheit {
    #[cfg_attr(feature = "serde", serde(rename = "W"))]
    #[cfg_attr(feature = "strum", strum(serialize = "W"))]
    W,
    #[cfg_attr(feature = "serde", serde(rename = "WH"))]
    #[cfg_attr(feature = "strum", strum(serialize = "WH"))]
    Wh,
    #[cfg_attr(feature = "serde", serde(rename = "KW"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KW"))]
    Kw,
    #[cfg_attr(feature = "serde", serde(rename = "KWH"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KWH"))]
    Kwh,
    #[cfg_attr(feature = "serde", serde(rename = "KVARH"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KVARH"))]
    Kvarh,
    #[cfg_attr(feature = "serde", serde(rename = "MW"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MW"))]
    Mw,
    #[cfg_attr(feature = "serde", serde(rename = "MWH"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MWH"))]
    Mwh,
    #[cfg_attr(feature = "serde", serde(rename = "STUECK"))]
    #[cfg_attr(feature = "strum", strum(serialize = "STUECK"))]
    Stueck,
    #[cfg_attr(feature = "serde", serde(rename = "KUBIKMETER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KUBIKMETER"))]
    Kubikmeter,
    #[cfg_attr(feature = "serde", serde(rename = "SEKUNDE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SEKUNDE"))]
    Sekunde,
    #[cfg_attr(feature = "serde", serde(rename = "MINUTE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MINUTE"))]
    Minute,
    #[cfg_attr(feature = "serde", serde(rename = "STUNDE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "STUNDE"))]
    Stunde,
    #[cfg_attr(feature = "serde", serde(rename = "VIERTEL_STUNDE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "VIERTEL_STUNDE"))]
    ViertelStunde,
    #[cfg_attr(feature = "serde", serde(rename = "TAG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TAG"))]
    Tag,
    #[cfg_attr(feature = "serde", serde(rename = "WOCHE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "WOCHE"))]
    Woche,
    #[cfg_attr(feature = "serde", serde(rename = "MONAT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MONAT"))]
    Monat,
    #[cfg_attr(feature = "serde", serde(rename = "QUARTAL"))]
    #[cfg_attr(feature = "strum", strum(serialize = "QUARTAL"))]
    Quartal,
    #[cfg_attr(feature = "serde", serde(rename = "HALBJAHR"))]
    #[cfg_attr(feature = "strum", strum(serialize = "HALBJAHR"))]
    Halbjahr,
    #[cfg_attr(feature = "serde", serde(rename = "JAHR"))]
    #[cfg_attr(feature = "strum", strum(serialize = "JAHR"))]
    Jahr,
    #[cfg_attr(feature = "serde", serde(rename = "PROZENT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "PROZENT"))]
    Prozent,
    #[cfg_attr(feature = "serde", serde(rename = "KVAR"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KVAR"))]
    Kvar,
    #[cfg_attr(feature = "serde", serde(rename = "KWHK"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KWHK"))]
    Kwhk,
    #[cfg_attr(feature = "serde", serde(rename = "VAR"))]
    #[cfg_attr(feature = "strum", strum(serialize = "VAR"))]
    Var,
    #[cfg_attr(feature = "serde", serde(rename = "VARH"))]
    #[cfg_attr(feature = "strum", strum(serialize = "VARH"))]
    Varh,
    /// Unknown or future variant — produced when deserializing a value
    /// that is not yet known to this version of the library.
    #[cfg_attr(feature = "serde", serde(other, rename = "UNKNOWN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UNKNOWN"))]
    Unknown,
}
impl Mengeneinheit {
    /// Returns an iterator over all **known** variants of `Mengeneinheit`.
    ///
    /// Unlike [`strum::IntoEnumIterator`] which includes the [`Mengeneinheit::Unknown`]
    /// catch-all, this method yields only variants that correspond to values defined
    /// in the BO4E schema.  Use this when building dropdowns, lookup tables, or
    /// generating reports that should only include valid schema values.
    ///
    /// # Example
    /// ```rust,ignore
    /// for v in Mengeneinheit::iter_known() {
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
impl sqlx::Type<sqlx::Postgres> for Mengeneinheit {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
/// Strum fast path: `AsRef<str>` returns the canonical string without a
/// `serde_json::Value` intermediate, saving an allocation per encode (M-07).
#[cfg(all(feature = "sqlx", feature = "json", feature = "strum"))]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Mengeneinheit {
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
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Mengeneinheit {
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
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for Mengeneinheit {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <String as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        serde_json::from_value(serde_json::Value::String(s))
            .map_err(|e| Box::new(e) as sqlx::error::BoxDynError)
    }
}
#[cfg(all(test, feature = "strum"))]
impl proptest::arbitrary::Arbitrary for Mengeneinheit {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        use strum::IntoEnumIterator as _;
        let variants: Vec<Self> = Self::iter().collect();
        proptest::sample::select(variants).boxed()
    }
}
