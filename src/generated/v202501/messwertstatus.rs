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
/// Der Status eines Zählerstandes
#[non_exhaustive]
pub enum Messwertstatus {
    #[cfg_attr(feature = "serde", serde(rename = "ABGELESEN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ABGELESEN"))]
    Abgelesen,
    #[cfg_attr(feature = "serde", serde(rename = "ERSATZWERT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ERSATZWERT"))]
    Ersatzwert,
    #[cfg_attr(feature = "serde", serde(rename = "ANGABE_FUER_LIEFERSCHEIN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ANGABE_FUER_LIEFERSCHEIN"))]
    AngabeFuerLieferschein,
    #[cfg_attr(feature = "serde", serde(rename = "VORSCHLAGSWERT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "VORSCHLAGSWERT"))]
    Vorschlagswert,
    #[cfg_attr(feature = "serde", serde(rename = "NICHT_VERWENDBAR"))]
    #[cfg_attr(feature = "strum", strum(serialize = "NICHT_VERWENDBAR"))]
    NichtVerwendbar,
    #[cfg_attr(feature = "serde", serde(rename = "PROGNOSEWERT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "PROGNOSEWERT"))]
    Prognosewert,
    #[cfg_attr(feature = "serde", serde(rename = "VORLAEUFIGERWERT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "VORLAEUFIGERWERT"))]
    Vorlaeufigerwert,
    #[cfg_attr(feature = "serde", serde(rename = "ENERGIEMENGESUMMIERT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ENERGIEMENGESUMMIERT"))]
    Energiemengesummiert,
    #[cfg_attr(feature = "serde", serde(rename = "FEHLT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "FEHLT"))]
    Fehlt,
    /// Unknown or future variant — produced when deserializing a value
    /// that is not yet known to this version of the library.
    #[cfg_attr(feature = "serde", serde(other, rename = "UNKNOWN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UNKNOWN"))]
    Unknown,
}
impl Messwertstatus {
    /// Returns an iterator over all **known** variants of `Messwertstatus`.
    ///
    /// Unlike [`strum::IntoEnumIterator`] which includes the [`Messwertstatus::Unknown`]
    /// catch-all, this method yields only variants that correspond to values defined
    /// in the BO4E schema.  Use this when building dropdowns, lookup tables, or
    /// generating reports that should only include valid schema values.
    ///
    /// # Example
    /// ```rust,ignore
    /// for v in Messwertstatus::iter_known() {
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
impl sqlx::Type<sqlx::Postgres> for Messwertstatus {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
/// Strum fast path: `AsRef<str>` returns the canonical string without a
/// `serde_json::Value` intermediate, saving an allocation per encode (M-07).
#[cfg(all(feature = "sqlx", feature = "json", feature = "strum"))]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Messwertstatus {
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
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Messwertstatus {
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
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for Messwertstatus {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <String as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        serde_json::from_value(serde_json::Value::String(s))
            .map_err(|e| Box::new(e) as sqlx::error::BoxDynError)
    }
}
#[cfg(all(test, feature = "strum"))]
impl proptest::arbitrary::Arbitrary for Messwertstatus {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        use strum::IntoEnumIterator as _;
        let variants: Vec<Self> = Self::iter().collect();
        proptest::sample::select(variants).boxed()
    }
}
