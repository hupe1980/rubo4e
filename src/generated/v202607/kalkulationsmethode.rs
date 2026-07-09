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
/// Auflistung der verschiedenen Berechnungsmethoden für ein Preisblatt.
#[non_exhaustive]
pub enum Kalkulationsmethode {
    #[cfg_attr(feature = "serde", serde(rename = "STUFEN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "STUFEN"))]
    Stufen,
    #[cfg_attr(feature = "serde", serde(rename = "ZONEN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ZONEN"))]
    Zonen,
    #[cfg_attr(feature = "serde", serde(rename = "VORZONEN_GP"))]
    #[cfg_attr(feature = "strum", strum(serialize = "VORZONEN_GP"))]
    VorzonenGp,
    #[cfg_attr(feature = "serde", serde(rename = "SIGMOID"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SIGMOID"))]
    Sigmoid,
    #[cfg_attr(feature = "serde", serde(rename = "BLINDARBEIT_GT_50_PROZENT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BLINDARBEIT_GT_50_PROZENT"))]
    BlindarbeitGt50Prozent,
    #[cfg_attr(feature = "serde", serde(rename = "BLINDARBEIT_GT_40_PROZENT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BLINDARBEIT_GT_40_PROZENT"))]
    BlindarbeitGt40Prozent,
    #[cfg_attr(feature = "serde", serde(rename = "BLINDARBEIT_MIT_FREIMENGE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BLINDARBEIT_MIT_FREIMENGE"))]
    BlindarbeitMitFreimenge,
    #[cfg_attr(feature = "serde", serde(rename = "AP_GP_ZONEN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "AP_GP_ZONEN"))]
    ApGpZonen,
    #[cfg_attr(feature = "serde", serde(rename = "LP_INSTALL_LEISTUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "LP_INSTALL_LEISTUNG"))]
    LpInstallLeistung,
    #[cfg_attr(feature = "serde", serde(rename = "AP_TRANSPORT_ODER_VERTEILNETZ"))]
    #[cfg_attr(feature = "strum", strum(serialize = "AP_TRANSPORT_ODER_VERTEILNETZ"))]
    ApTransportOderVerteilnetz,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "AP_TRANSPORT_ODER_VERTEILNETZ_ORTSVERTEILNETZ_SIGMOID")
    )]
    #[cfg_attr(
        feature = "strum",
        strum(serialize = "AP_TRANSPORT_ODER_VERTEILNETZ_ORTSVERTEILNETZ_SIGMOID")
    )]
    ApTransportOderVerteilnetzOrtsverteilnetzSigmoid,
    #[cfg_attr(feature = "serde", serde(rename = "LP_JAHRESVERBRAUCH"))]
    #[cfg_attr(feature = "strum", strum(serialize = "LP_JAHRESVERBRAUCH"))]
    LpJahresverbrauch,
    #[cfg_attr(feature = "serde", serde(rename = "LP_TRANSPORT_ODER_VERTEILNETZ"))]
    #[cfg_attr(feature = "strum", strum(serialize = "LP_TRANSPORT_ODER_VERTEILNETZ"))]
    LpTransportOderVerteilnetz,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "LP_TRANSPORT_ODER_VERTEILNETZ_ORTSVERTEILNETZ_SIGMOID")
    )]
    #[cfg_attr(
        feature = "strum",
        strum(serialize = "LP_TRANSPORT_ODER_VERTEILNETZ_ORTSVERTEILNETZ_SIGMOID")
    )]
    LpTransportOderVerteilnetzOrtsverteilnetzSigmoid,
    #[cfg_attr(feature = "serde", serde(rename = "FUNKTIONEN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "FUNKTIONEN"))]
    Funktionen,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "VERBRAUCH_UEBER_SLP_GRENZE_FUNKTIONSBEZOGEN_WEITERE_BERECHNUNG_ALS_LGK")
    )]
    #[cfg_attr(
        feature = "strum",
        strum(
            serialize = "VERBRAUCH_UEBER_SLP_GRENZE_FUNKTIONSBEZOGEN_WEITERE_BERECHNUNG_ALS_LGK"
        )
    )]
    VerbrauchUeberSlpGrenzeFunktionsbezogenWeitereBerechnungAlsLgk,
    /// Unknown or future variant — produced when deserializing a value
    /// that is not yet known to this version of the library.
    #[cfg_attr(feature = "serde", serde(other, rename = "UNKNOWN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UNKNOWN"))]
    Unknown,
}
impl Kalkulationsmethode {
    /// Returns an iterator over all **known** variants of `Kalkulationsmethode`.
    ///
    /// Unlike [`strum::IntoEnumIterator`] which includes the [`Kalkulationsmethode::Unknown`]
    /// catch-all, this method yields only variants that correspond to values defined
    /// in the BO4E schema.  Use this when building dropdowns, lookup tables, or
    /// generating reports that should only include valid schema values.
    ///
    /// # Example
    /// ```rust,ignore
    /// for v in Kalkulationsmethode::iter_known() {
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
impl sqlx::Type<sqlx::Postgres> for Kalkulationsmethode {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
/// Strum fast path: `AsRef<str>` returns the canonical string without a
/// `serde_json::Value` intermediate, saving an allocation per encode (M-07).
#[cfg(all(feature = "sqlx", feature = "json", feature = "strum"))]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Kalkulationsmethode {
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
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Kalkulationsmethode {
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
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for Kalkulationsmethode {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <String as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        serde_json::from_value(serde_json::Value::String(s))
            .map_err(|e| Box::new(e) as sqlx::error::BoxDynError)
    }
}
#[cfg(all(test, feature = "strum"))]
impl proptest::arbitrary::Arbitrary for Kalkulationsmethode {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        use strum::IntoEnumIterator as _;
        let variants: Vec<Self> = Self::iter().collect();
        proptest::sample::select(variants).boxed()
    }
}
