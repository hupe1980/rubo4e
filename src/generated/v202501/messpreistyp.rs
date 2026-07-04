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
/// Festlegung, welcher Typ von Messung mit einem Preis belegt wird
#[non_exhaustive]
pub enum Messpreistyp {
    #[cfg_attr(feature = "serde", serde(rename = "MESSPREIS_G2_5"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MESSPREIS_G2_5"))]
    MesspreisG25,
    #[cfg_attr(feature = "serde", serde(rename = "MESSPREIS_G4"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MESSPREIS_G4"))]
    MesspreisG4,
    #[cfg_attr(feature = "serde", serde(rename = "MESSPREIS_G6"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MESSPREIS_G6"))]
    MesspreisG6,
    #[cfg_attr(feature = "serde", serde(rename = "MESSPREIS_G10"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MESSPREIS_G10"))]
    MesspreisG10,
    #[cfg_attr(feature = "serde", serde(rename = "MESSPREIS_G16"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MESSPREIS_G16"))]
    MesspreisG16,
    #[cfg_attr(feature = "serde", serde(rename = "MESSPREIS_G25"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MESSPREIS_G25"))]
    MESSPREISG25,
    #[cfg_attr(feature = "serde", serde(rename = "MESSPREIS_G40"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MESSPREIS_G40"))]
    MesspreisG40,
    #[cfg_attr(feature = "serde", serde(rename = "ELEKTRONISCHER_AUFSATZ"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ELEKTRONISCHER_AUFSATZ"))]
    ElektronischerAufsatz,
    #[cfg_attr(feature = "serde", serde(rename = "SMART_METER_MESSPREIS_G2_5"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SMART_METER_MESSPREIS_G2_5"))]
    SmartMeterMesspreisG25,
    #[cfg_attr(feature = "serde", serde(rename = "SMART_METER_MESSPREIS_G4"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SMART_METER_MESSPREIS_G4"))]
    SmartMeterMesspreisG4,
    #[cfg_attr(feature = "serde", serde(rename = "SMART_METER_MESSPREIS_G6"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SMART_METER_MESSPREIS_G6"))]
    SmartMeterMesspreisG6,
    #[cfg_attr(feature = "serde", serde(rename = "SMART_METER_MESSPREIS_G10"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SMART_METER_MESSPREIS_G10"))]
    SmartMeterMesspreisG10,
    #[cfg_attr(feature = "serde", serde(rename = "SMART_METER_MESSPREIS_G16"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SMART_METER_MESSPREIS_G16"))]
    SmartMeterMesspreisG16,
    #[cfg_attr(feature = "serde", serde(rename = "SMART_METER_MESSPREIS_G25"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SMART_METER_MESSPREIS_G25"))]
    SMARTMETERMESSPREISG25,
    #[cfg_attr(feature = "serde", serde(rename = "SMART_METER_MESSPREIS_G40"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SMART_METER_MESSPREIS_G40"))]
    SmartMeterMesspreisG40,
    #[cfg_attr(feature = "serde", serde(rename = "VERRECHNUNGSPREIS_ET_WECHSEL"))]
    #[cfg_attr(feature = "strum", strum(serialize = "VERRECHNUNGSPREIS_ET_WECHSEL"))]
    VerrechnungspreisEtWechsel,
    #[cfg_attr(feature = "serde", serde(rename = "VERRECHNUNGSPREIS_ET_DREH"))]
    #[cfg_attr(feature = "strum", strum(serialize = "VERRECHNUNGSPREIS_ET_DREH"))]
    VerrechnungspreisEtDreh,
    #[cfg_attr(feature = "serde", serde(rename = "VERRECHNUNGSPREIS_ZT_WECHSEL"))]
    #[cfg_attr(feature = "strum", strum(serialize = "VERRECHNUNGSPREIS_ZT_WECHSEL"))]
    VerrechnungspreisZtWechsel,
    #[cfg_attr(feature = "serde", serde(rename = "VERRECHNUNGSPREIS_ZT_DREH"))]
    #[cfg_attr(feature = "strum", strum(serialize = "VERRECHNUNGSPREIS_ZT_DREH"))]
    VerrechnungspreisZtDreh,
    #[cfg_attr(feature = "serde", serde(rename = "VERRECHNUNGSPREIS_L_ET"))]
    #[cfg_attr(feature = "strum", strum(serialize = "VERRECHNUNGSPREIS_L_ET"))]
    VerrechnungspreisLEt,
    #[cfg_attr(feature = "serde", serde(rename = "VERRECHNUNGSPREIS_L_ZT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "VERRECHNUNGSPREIS_L_ZT"))]
    VerrechnungspreisLZt,
    #[cfg_attr(feature = "serde", serde(rename = "VERRECHNUNGSPREIS_SM"))]
    #[cfg_attr(feature = "strum", strum(serialize = "VERRECHNUNGSPREIS_SM"))]
    VerrechnungspreisSm,
    #[cfg_attr(feature = "serde", serde(rename = "AUFSCHLAG_WANDLER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "AUFSCHLAG_WANDLER"))]
    AufschlagWandler,
    #[cfg_attr(feature = "serde", serde(rename = "AUFSCHLAG_TARIFSCHALTUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "AUFSCHLAG_TARIFSCHALTUNG"))]
    AufschlagTarifschaltung,
    /// Unknown or future variant — produced when deserializing a value
    /// that is not yet known to this version of the library.
    #[cfg_attr(feature = "serde", serde(other, rename = "UNKNOWN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UNKNOWN"))]
    Unknown,
}
impl Messpreistyp {
    /// Returns an iterator over all **known** variants of `Messpreistyp`.
    ///
    /// Unlike [`strum::IntoEnumIterator`] which includes the [`Messpreistyp::Unknown`]
    /// catch-all, this method yields only variants that correspond to values defined
    /// in the BO4E schema.  Use this when building dropdowns, lookup tables, or
    /// generating reports that should only include valid schema values.
    ///
    /// # Example
    /// ```rust,ignore
    /// for v in Messpreistyp::iter_known() {
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
impl sqlx::Type<sqlx::Postgres> for Messpreistyp {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
/// Strum fast path: `AsRef<str>` returns the canonical string without a
/// `serde_json::Value` intermediate, saving an allocation per encode (M-07).
#[cfg(all(feature = "sqlx", feature = "json", feature = "strum"))]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Messpreistyp {
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
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Messpreistyp {
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
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for Messpreistyp {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <String as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        serde_json::from_value(serde_json::Value::String(s))
            .map_err(|e| Box::new(e) as sqlx::error::BoxDynError)
    }
}
#[cfg(all(test, feature = "strum"))]
impl proptest::arbitrary::Arbitrary for Messpreistyp {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        use strum::IntoEnumIterator as _;
        let variants: Vec<Self> = Self::iter().collect();
        proptest::sample::select(variants).boxed()
    }
}
