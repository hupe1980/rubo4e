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
/// Klassifizierung der Kriterien für eine regionale Eingrenzung.
#[non_exhaustive]
pub enum Regionskriterium {
    #[cfg_attr(feature = "serde", serde(rename = "BUNDESLANDKENNZIFFER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BUNDESLANDKENNZIFFER"))]
    Bundeslandkennziffer,
    #[cfg_attr(feature = "serde", serde(rename = "BUNDESLAND_NAME"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BUNDESLAND_NAME"))]
    BundeslandName,
    #[cfg_attr(feature = "serde", serde(rename = "MARKTGEBIET_NUMMER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MARKTGEBIET_NUMMER"))]
    MarktgebietNummer,
    #[cfg_attr(feature = "serde", serde(rename = "MARKTGEBIET_NAME"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MARKTGEBIET_NAME"))]
    MarktgebietName,
    #[cfg_attr(feature = "serde", serde(rename = "REGELGEBIET_NUMMER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "REGELGEBIET_NUMMER"))]
    RegelgebietNummer,
    #[cfg_attr(feature = "serde", serde(rename = "REGELGEBIET_NAME"))]
    #[cfg_attr(feature = "strum", strum(serialize = "REGELGEBIET_NAME"))]
    RegelgebietName,
    #[cfg_attr(feature = "serde", serde(rename = "NETZ_STROM"))]
    #[cfg_attr(feature = "strum", strum(serialize = "NETZ_STROM"))]
    NetzStrom,
    #[cfg_attr(feature = "serde", serde(rename = "NETZ_GAS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "NETZ_GAS"))]
    NetzGas,
    #[cfg_attr(feature = "serde", serde(rename = "NETZBETREIBER_NUMMER_STROM"))]
    #[cfg_attr(feature = "strum", strum(serialize = "NETZBETREIBER_NUMMER_STROM"))]
    NetzbetreiberNummerStrom,
    #[cfg_attr(feature = "serde", serde(rename = "NETZBETREIBER_NUMMER_GAS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "NETZBETREIBER_NUMMER_GAS"))]
    NetzbetreiberNummerGas,
    #[cfg_attr(feature = "serde", serde(rename = "NETZBETREIBER_NAME_STROM"))]
    #[cfg_attr(feature = "strum", strum(serialize = "NETZBETREIBER_NAME_STROM"))]
    NetzbetreiberNameStrom,
    #[cfg_attr(feature = "serde", serde(rename = "NETZBETREIBER_NAME_GAS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "NETZBETREIBER_NAME_GAS"))]
    NetzbetreiberNameGas,
    #[cfg_attr(feature = "serde", serde(rename = "BILANZIERUNGS_GEBIET_NUMMER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BILANZIERUNGS_GEBIET_NUMMER"))]
    BilanzierungsGebietNummer,
    #[cfg_attr(feature = "serde", serde(rename = "MSB_NUMMER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MSB_NUMMER"))]
    MsbNummer,
    #[cfg_attr(feature = "serde", serde(rename = "MSB_NAME"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MSB_NAME"))]
    MsbName,
    #[cfg_attr(feature = "serde", serde(rename = "VERSORGER_NUMMER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "VERSORGER_NUMMER"))]
    VersorgerNummer,
    #[cfg_attr(feature = "serde", serde(rename = "VERSORGER_NAME"))]
    #[cfg_attr(feature = "strum", strum(serialize = "VERSORGER_NAME"))]
    VersorgerName,
    #[cfg_attr(feature = "serde", serde(rename = "GRUNDVERSORGER_NUMMER_STROM"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GRUNDVERSORGER_NUMMER_STROM"))]
    GrundversorgerNummerStrom,
    #[cfg_attr(feature = "serde", serde(rename = "GRUNDVERSORGER_NAME_STROM"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GRUNDVERSORGER_NAME_STROM"))]
    GrundversorgerNameStrom,
    #[cfg_attr(feature = "serde", serde(rename = "GRUNDVERSORGER_NUMMER_GAS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GRUNDVERSORGER_NUMMER_GAS"))]
    GrundversorgerNummerGas,
    #[cfg_attr(feature = "serde", serde(rename = "GRUNDVERSORGER_NAME_GAS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GRUNDVERSORGER_NAME_GAS"))]
    GrundversorgerNameGas,
    #[cfg_attr(feature = "serde", serde(rename = "KREIS_NAME"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KREIS_NAME"))]
    KreisName,
    #[cfg_attr(feature = "serde", serde(rename = "KREISKENNZIFFER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KREISKENNZIFFER"))]
    Kreiskennziffer,
    #[cfg_attr(feature = "serde", serde(rename = "GEMEINDE_NAME"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GEMEINDE_NAME"))]
    GemeindeName,
    #[cfg_attr(feature = "serde", serde(rename = "GEMEINDEKENNZIFFER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GEMEINDEKENNZIFFER"))]
    Gemeindekennziffer,
    #[cfg_attr(feature = "serde", serde(rename = "POSTLEITZAHL"))]
    #[cfg_attr(feature = "strum", strum(serialize = "POSTLEITZAHL"))]
    Postleitzahl,
    #[cfg_attr(feature = "serde", serde(rename = "ORT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ORT"))]
    Ort,
    #[cfg_attr(feature = "serde", serde(rename = "POSTORT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "POSTORT"))]
    Postort,
    #[cfg_attr(feature = "serde", serde(rename = "EINWOHNERZAHL_GEMEINDE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "EINWOHNERZAHL_GEMEINDE"))]
    EinwohnerzahlGemeinde,
    #[cfg_attr(feature = "serde", serde(rename = "EINWOHNERZAHL_ORT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "EINWOHNERZAHL_ORT"))]
    EinwohnerzahlOrt,
    #[cfg_attr(feature = "serde", serde(rename = "PLZ_KM_UMKREIS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "PLZ_KM_UMKREIS"))]
    PlzKmUmkreis,
    #[cfg_attr(feature = "serde", serde(rename = "BUNDESWEIT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BUNDESWEIT"))]
    Bundesweit,
    #[cfg_attr(feature = "serde", serde(rename = "PLZ_BEREICH"))]
    #[cfg_attr(feature = "strum", strum(serialize = "PLZ_BEREICH"))]
    PlzBereich,
    /// Unknown or future variant — produced when deserializing a value
    /// that is not yet known to this version of the library.
    #[cfg_attr(feature = "serde", serde(other, rename = "UNKNOWN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UNKNOWN"))]
    Unknown,
}
impl Regionskriterium {
    /// Returns an iterator over all **known** variants of `Regionskriterium`.
    ///
    /// Unlike [`strum::IntoEnumIterator`] which includes the [`Regionskriterium::Unknown`]
    /// catch-all, this method yields only variants that correspond to values defined
    /// in the BO4E schema.  Use this when building dropdowns, lookup tables, or
    /// generating reports that should only include valid schema values.
    ///
    /// # Example
    /// ```rust,ignore
    /// for v in Regionskriterium::iter_known() {
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
impl sqlx::Type<sqlx::Postgres> for Regionskriterium {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
/// Strum fast path: `AsRef<str>` returns the canonical string without a
/// `serde_json::Value` intermediate, saving an allocation per encode (M-07).
#[cfg(all(feature = "sqlx", feature = "json", feature = "strum"))]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Regionskriterium {
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
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Regionskriterium {
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
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for Regionskriterium {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <String as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        serde_json::from_value(serde_json::Value::String(s))
            .map_err(|e| Box::new(e) as sqlx::error::BoxDynError)
    }
}
#[cfg(all(test, feature = "strum"))]
impl proptest::arbitrary::Arbitrary for Regionskriterium {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        use strum::IntoEnumIterator as _;
        let variants: Vec<Self> = Self::iter().collect();
        proptest::sample::select(variants).boxed()
    }
}
