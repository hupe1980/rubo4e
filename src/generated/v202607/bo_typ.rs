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
/// Auflistung sämtlicher existierender Geschäftsobjekte.
#[non_exhaustive]
pub enum BoTyp {
    #[cfg_attr(feature = "serde", serde(rename = "ANGEBOT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ANGEBOT"))]
    Angebot,
    #[cfg_attr(feature = "serde", serde(rename = "AUSSCHREIBUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "AUSSCHREIBUNG"))]
    Ausschreibung,
    #[cfg_attr(feature = "serde", serde(rename = "BILANZIERUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BILANZIERUNG"))]
    Bilanzierung,
    #[cfg_attr(feature = "serde", serde(rename = "BUENDELVERTRAG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BUENDELVERTRAG"))]
    Buendelvertrag,
    #[cfg_attr(feature = "serde", serde(rename = "ENERGIEMENGE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ENERGIEMENGE"))]
    Energiemenge,
    #[cfg_attr(feature = "serde", serde(rename = "FREMDKOSTEN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "FREMDKOSTEN"))]
    Fremdkosten,
    #[cfg_attr(feature = "serde", serde(rename = "GERAET"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GERAET"))]
    Geraet,
    #[cfg_attr(feature = "serde", serde(rename = "GESCHAEFTSOBJEKT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GESCHAEFTSOBJEKT"))]
    Geschaeftsobjekt,
    #[cfg_attr(feature = "serde", serde(rename = "GESCHAEFTSPARTNER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GESCHAEFTSPARTNER"))]
    Geschaeftspartner,
    #[cfg_attr(feature = "serde", serde(rename = "KOSTEN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KOSTEN"))]
    Kosten,
    #[cfg_attr(feature = "serde", serde(rename = "LASTGANG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "LASTGANG"))]
    Lastgang,
    #[cfg_attr(feature = "serde", serde(rename = "MARKTLOKATION"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MARKTLOKATION"))]
    Marktlokation,
    #[cfg_attr(feature = "serde", serde(rename = "MESSLOKATION"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MESSLOKATION"))]
    Messlokation,
    #[cfg_attr(feature = "serde", serde(rename = "NETZLOKATION"))]
    #[cfg_attr(feature = "strum", strum(serialize = "NETZLOKATION"))]
    Netzlokation,
    #[cfg_attr(feature = "serde", serde(rename = "MARKTTEILNEHMER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MARKTTEILNEHMER"))]
    Marktteilnehmer,
    #[cfg_attr(feature = "serde", serde(rename = "NETZNUTZUNGSRECHNUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "NETZNUTZUNGSRECHNUNG"))]
    Netznutzungsrechnung,
    #[cfg_attr(feature = "serde", serde(rename = "TECHNISCHERESSOURCE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TECHNISCHERESSOURCE"))]
    Technischeressource,
    #[cfg_attr(feature = "serde", serde(rename = "STEUERBARERESSOURCE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "STEUERBARERESSOURCE"))]
    Steuerbareressource,
    #[cfg_attr(feature = "serde", serde(rename = "PERSON"))]
    #[cfg_attr(feature = "strum", strum(serialize = "PERSON"))]
    Person,
    #[cfg_attr(feature = "serde", serde(rename = "PREISBLATT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "PREISBLATT"))]
    Preisblatt,
    #[cfg_attr(feature = "serde", serde(rename = "PREISBLATTDIENSTLEISTUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "PREISBLATTDIENSTLEISTUNG"))]
    Preisblattdienstleistung,
    #[cfg_attr(feature = "serde", serde(rename = "PREISBLATTHARDWARE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "PREISBLATTHARDWARE"))]
    Preisblatthardware,
    #[cfg_attr(feature = "serde", serde(rename = "PREISBLATTKONZESSIONSABGABE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "PREISBLATTKONZESSIONSABGABE"))]
    Preisblattkonzessionsabgabe,
    #[cfg_attr(feature = "serde", serde(rename = "PREISBLATTMESSUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "PREISBLATTMESSUNG"))]
    Preisblattmessung,
    #[cfg_attr(feature = "serde", serde(rename = "PREISBLATTNETZNUTZUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "PREISBLATTNETZNUTZUNG"))]
    Preisblattnetznutzung,
    #[cfg_attr(feature = "serde", serde(rename = "PREISBLATTUMLAGEN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "PREISBLATTUMLAGEN"))]
    Preisblattumlagen,
    #[cfg_attr(feature = "serde", serde(rename = "RECHNUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "RECHNUNG"))]
    Rechnung,
    #[cfg_attr(feature = "serde", serde(rename = "REGION"))]
    #[cfg_attr(feature = "strum", strum(serialize = "REGION"))]
    Region,
    #[cfg_attr(feature = "serde", serde(rename = "STANDORTEIGENSCHAFTEN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "STANDORTEIGENSCHAFTEN"))]
    Standorteigenschaften,
    #[cfg_attr(feature = "serde", serde(rename = "TARIF"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TARIF"))]
    Tarif,
    #[cfg_attr(feature = "serde", serde(rename = "TARIFINFO"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TARIFINFO"))]
    Tarifinfo,
    #[cfg_attr(feature = "serde", serde(rename = "TARIFKOSTEN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TARIFKOSTEN"))]
    Tarifkosten,
    #[cfg_attr(feature = "serde", serde(rename = "TARIFPREISBLATT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TARIFPREISBLATT"))]
    Tarifpreisblatt,
    #[cfg_attr(feature = "serde", serde(rename = "VERTRAG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "VERTRAG"))]
    Vertrag,
    #[cfg_attr(feature = "serde", serde(rename = "ZAEHLER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ZAEHLER"))]
    Zaehler,
    #[cfg_attr(feature = "serde", serde(rename = "ZAEHLZEITDEFINITION"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ZAEHLZEITDEFINITION"))]
    Zaehlzeitdefinition,
    #[cfg_attr(feature = "serde", serde(rename = "ZEITREIHE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ZEITREIHE"))]
    Zeitreihe,
    #[cfg_attr(feature = "serde", serde(rename = "LOKATIONSZUORDNUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "LOKATIONSZUORDNUNG"))]
    Lokationszuordnung,
    /// Unknown or future variant — produced when deserializing a value
    /// that is not yet known to this version of the library.
    #[cfg_attr(feature = "serde", serde(other, rename = "UNKNOWN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UNKNOWN"))]
    Unknown,
}
impl BoTyp {
    /// Returns an iterator over all **known** variants of `BoTyp`.
    ///
    /// Unlike [`strum::IntoEnumIterator`] which includes the [`BoTyp::Unknown`]
    /// catch-all, this method yields only variants that correspond to values defined
    /// in the BO4E schema.  Use this when building dropdowns, lookup tables, or
    /// generating reports that should only include valid schema values.
    ///
    /// # Example
    /// ```rust,ignore
    /// for v in BoTyp::iter_known() {
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
impl sqlx::Type<sqlx::Postgres> for BoTyp {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
/// Strum fast path: `AsRef<str>` returns the canonical string without a
/// `serde_json::Value` intermediate, saving an allocation per encode (M-07).
#[cfg(all(feature = "sqlx", feature = "json", feature = "strum"))]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for BoTyp {
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
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for BoTyp {
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
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for BoTyp {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <String as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        serde_json::from_value(serde_json::Value::String(s))
            .map_err(|e| Box::new(e) as sqlx::error::BoxDynError)
    }
}
#[cfg(all(test, feature = "strum"))]
impl proptest::arbitrary::Arbitrary for BoTyp {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        use strum::IntoEnumIterator as _;
        let variants: Vec<Self> = Self::iter().collect();
        proptest::sample::select(variants).boxed()
    }
}
