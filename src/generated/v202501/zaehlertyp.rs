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
/// Bei diesem Enum handelt es sich um die Abbildung von Zählertypen der Sparten Strom und Gas.
#[non_exhaustive]
pub enum Zaehlertyp {
    #[cfg_attr(feature = "serde", serde(rename = "DREHSTROMZAEHLER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "DREHSTROMZAEHLER"))]
    Drehstromzaehler,
    #[cfg_attr(feature = "serde", serde(rename = "BALGENGASZAEHLER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BALGENGASZAEHLER"))]
    Balgengaszaehler,
    #[cfg_attr(feature = "serde", serde(rename = "DREHKOLBENZAEHLER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "DREHKOLBENZAEHLER"))]
    Drehkolbenzaehler,
    #[cfg_attr(feature = "serde", serde(rename = "LEISTUNGSZAEHLER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "LEISTUNGSZAEHLER"))]
    Leistungszaehler,
    #[cfg_attr(feature = "serde", serde(rename = "MAXIMUMZAEHLER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MAXIMUMZAEHLER"))]
    Maximumzaehler,
    #[cfg_attr(feature = "serde", serde(rename = "TURBINENRADGASZAEHLER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TURBINENRADGASZAEHLER"))]
    Turbinenradgaszaehler,
    #[cfg_attr(feature = "serde", serde(rename = "ULTRASCHALLGASZAEHLER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ULTRASCHALLGASZAEHLER"))]
    Ultraschallgaszaehler,
    #[cfg_attr(feature = "serde", serde(rename = "WECHSELSTROMZAEHLER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "WECHSELSTROMZAEHLER"))]
    Wechselstromzaehler,
    #[cfg_attr(feature = "serde", serde(rename = "MODERNE_MESSEINRICHTUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MODERNE_MESSEINRICHTUNG"))]
    ModerneMesseinrichtung,
    #[cfg_attr(feature = "serde", serde(rename = "INTELLIGENTES_MESSSYSTEM"))]
    #[cfg_attr(feature = "strum", strum(serialize = "INTELLIGENTES_MESSSYSTEM"))]
    IntelligentesMesssystem,
    #[cfg_attr(feature = "serde", serde(rename = "ELEKTRONISCHER_ZAEHLER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ELEKTRONISCHER_ZAEHLER"))]
    ElektronischerZaehler,
    #[cfg_attr(feature = "serde", serde(rename = "WIRBELGASZAEHLER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "WIRBELGASZAEHLER"))]
    Wirbelgaszaehler,
    #[cfg_attr(feature = "serde", serde(rename = "WASSERZAEHLER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "WASSERZAEHLER"))]
    Wasserzaehler,
    /// Unknown or future variant — produced when deserializing a value
    /// that is not yet known to this version of the library.
    #[cfg_attr(feature = "serde", serde(other, rename = "UNKNOWN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UNKNOWN"))]
    Unknown,
}
impl Zaehlertyp {
    /// Returns an iterator over all **known** variants of `Zaehlertyp`.
    ///
    /// Unlike [`strum::IntoEnumIterator`] which includes the [`Zaehlertyp::Unknown`]
    /// catch-all, this method yields only variants that correspond to values defined
    /// in the BO4E schema.  Use this when building dropdowns, lookup tables, or
    /// generating reports that should only include valid schema values.
    ///
    /// # Example
    /// ```rust,ignore
    /// for v in Zaehlertyp::iter_known() {
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
impl sqlx::Type<sqlx::Postgres> for Zaehlertyp {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
/// Strum fast path: `AsRef<str>` returns the canonical string without a
/// `serde_json::Value` intermediate, saving an allocation per encode (M-07).
#[cfg(all(feature = "sqlx", feature = "json", feature = "strum"))]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Zaehlertyp {
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
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Zaehlertyp {
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
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for Zaehlertyp {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <String as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        serde_json::from_value(serde_json::Value::String(s))
            .map_err(|e| Box::new(e) as sqlx::error::BoxDynError)
    }
}
#[cfg(all(test, feature = "strum"))]
impl proptest::arbitrary::Arbitrary for Zaehlertyp {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        use strum::IntoEnumIterator as _;
        let variants: Vec<Self> = Self::iter().collect();
        proptest::sample::select(variants).boxed()
    }
}
