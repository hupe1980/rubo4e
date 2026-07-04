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
/// Auflistung möglicher abzurechnender Gerätetypen.
#[non_exhaustive]
pub enum Geraetetyp {
    #[cfg_attr(feature = "serde", serde(rename = "MULTIPLEXANLAGE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MULTIPLEXANLAGE"))]
    Multiplexanlage,
    #[cfg_attr(feature = "serde", serde(rename = "PAUSCHALANLAGE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "PAUSCHALANLAGE"))]
    Pauschalanlage,
    #[cfg_attr(feature = "serde", serde(rename = "VERSTAERKERANLAGE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "VERSTAERKERANLAGE"))]
    Verstaerkeranlage,
    #[cfg_attr(feature = "serde", serde(rename = "SUMMATIONSGERAET"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SUMMATIONSGERAET"))]
    Summationsgeraet,
    #[cfg_attr(feature = "serde", serde(rename = "IMPULSGEBER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "IMPULSGEBER"))]
    Impulsgeber,
    #[cfg_attr(feature = "serde", serde(rename = "MENGENUMWERTER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MENGENUMWERTER"))]
    Mengenumwerter,
    #[cfg_attr(feature = "serde", serde(rename = "STROMWANDLER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "STROMWANDLER"))]
    Stromwandler,
    #[cfg_attr(feature = "serde", serde(rename = "SPANNUNGSWANDLER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SPANNUNGSWANDLER"))]
    Spannungswandler,
    #[cfg_attr(feature = "serde", serde(rename = "KOMBIMESSWANDLER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KOMBIMESSWANDLER"))]
    Kombimesswandler,
    #[cfg_attr(feature = "serde", serde(rename = "BLOCKSTROMWANDLER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BLOCKSTROMWANDLER"))]
    Blockstromwandler,
    #[cfg_attr(feature = "serde", serde(rename = "DATENLOGGER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "DATENLOGGER"))]
    Datenlogger,
    #[cfg_attr(feature = "serde", serde(rename = "KOMMUNIKATIONSANSCHLUSS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KOMMUNIKATIONSANSCHLUSS"))]
    Kommunikationsanschluss,
    #[cfg_attr(feature = "serde", serde(rename = "MODEM"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MODEM"))]
    Modem,
    #[cfg_attr(feature = "serde", serde(rename = "TELEKOMMUNIKATIONSEINRICHTUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TELEKOMMUNIKATIONSEINRICHTUNG"))]
    Telekommunikationseinrichtung,
    #[cfg_attr(feature = "serde", serde(rename = "MODERNE_MESSEINRICHTUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MODERNE_MESSEINRICHTUNG"))]
    ModerneMesseinrichtung,
    #[cfg_attr(feature = "serde", serde(rename = "INTELLIGENTES_MESSYSTEM"))]
    #[cfg_attr(feature = "strum", strum(serialize = "INTELLIGENTES_MESSYSTEM"))]
    IntelligentesMessystem,
    #[cfg_attr(feature = "serde", serde(rename = "STEUEREINRICHTUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "STEUEREINRICHTUNG"))]
    Steuereinrichtung,
    #[cfg_attr(feature = "serde", serde(rename = "TARIFSCHALTGERAET"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TARIFSCHALTGERAET"))]
    Tarifschaltgeraet,
    #[cfg_attr(feature = "serde", serde(rename = "RUNDSTEUEREMPFAENGER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "RUNDSTEUEREMPFAENGER"))]
    Rundsteuerempfaenger,
    #[cfg_attr(feature = "serde", serde(rename = "OPTIONALE_ZUS_ZAEHLEINRICHTUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "OPTIONALE_ZUS_ZAEHLEINRICHTUNG"))]
    OptionaleZusZaehleinrichtung,
    #[cfg_attr(feature = "serde", serde(rename = "MESSWANDLERSATZ_IMS_MME"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MESSWANDLERSATZ_IMS_MME"))]
    MesswandlersatzImsMme,
    #[cfg_attr(feature = "serde", serde(rename = "KOMBIMESSWANDLER_IMS_MME"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KOMBIMESSWANDLER_IMS_MME"))]
    KombimesswandlerImsMme,
    #[cfg_attr(feature = "serde", serde(rename = "TARIFSCHALTGERAET_IMS_MME"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TARIFSCHALTGERAET_IMS_MME"))]
    TarifschaltgeraetImsMme,
    #[cfg_attr(feature = "serde", serde(rename = "RUNDSTEUEREMPFAENGER_IMS_MME"))]
    #[cfg_attr(feature = "strum", strum(serialize = "RUNDSTEUEREMPFAENGER_IMS_MME"))]
    RundsteuerempfaengerImsMme,
    #[cfg_attr(feature = "serde", serde(rename = "TEMPERATUR_KOMPENSATION"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TEMPERATUR_KOMPENSATION"))]
    TemperaturKompensation,
    #[cfg_attr(feature = "serde", serde(rename = "HOECHSTBELASTUNGS_ANZEIGER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "HOECHSTBELASTUNGS_ANZEIGER"))]
    HoechstbelastungsAnzeiger,
    #[cfg_attr(feature = "serde", serde(rename = "SONSTIGES_GERAET"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SONSTIGES_GERAET"))]
    SonstigesGeraet,
    #[cfg_attr(feature = "serde", serde(rename = "EDL_21"))]
    #[cfg_attr(feature = "strum", strum(serialize = "EDL_21"))]
    Edl21,
    #[cfg_attr(feature = "serde", serde(rename = "EDL_40_ZAEHLERAUFSATZ"))]
    #[cfg_attr(feature = "strum", strum(serialize = "EDL_40_ZAEHLERAUFSATZ"))]
    Edl40Zaehleraufsatz,
    #[cfg_attr(feature = "serde", serde(rename = "EDL_40"))]
    #[cfg_attr(feature = "strum", strum(serialize = "EDL_40"))]
    Edl40,
    #[cfg_attr(feature = "serde", serde(rename = "TELEFONANSCHLUSS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TELEFONANSCHLUSS"))]
    Telefonanschluss,
    #[cfg_attr(feature = "serde", serde(rename = "MODEM_GSM"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MODEM_GSM"))]
    ModemGsm,
    #[cfg_attr(feature = "serde", serde(rename = "MODEM_GPRS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MODEM_GPRS"))]
    ModemGprs,
    #[cfg_attr(feature = "serde", serde(rename = "MODEM_FUNK"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MODEM_FUNK"))]
    ModemFunk,
    #[cfg_attr(feature = "serde", serde(rename = "MODEM_GSM_O_LG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MODEM_GSM_O_LG"))]
    ModemGsmOLg,
    #[cfg_attr(feature = "serde", serde(rename = "MODEM_GSM_M_LG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MODEM_GSM_M_LG"))]
    ModemGsmMLg,
    #[cfg_attr(feature = "serde", serde(rename = "MODEM_FESTNETZ"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MODEM_FESTNETZ"))]
    ModemFestnetz,
    #[cfg_attr(feature = "serde", serde(rename = "MODEM_GPRS_M_LG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MODEM_GPRS_M_LG"))]
    ModemGprsMLg,
    #[cfg_attr(feature = "serde", serde(rename = "PLC_KOM"))]
    #[cfg_attr(feature = "strum", strum(serialize = "PLC_KOM"))]
    PlcKom,
    #[cfg_attr(feature = "serde", serde(rename = "ETHERNET_KOM"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ETHERNET_KOM"))]
    EthernetKom,
    #[cfg_attr(feature = "serde", serde(rename = "DSL_KOM"))]
    #[cfg_attr(feature = "strum", strum(serialize = "DSL_KOM"))]
    DslKom,
    #[cfg_attr(feature = "serde", serde(rename = "LTE_KOM"))]
    #[cfg_attr(feature = "strum", strum(serialize = "LTE_KOM"))]
    LteKom,
    #[cfg_attr(feature = "serde", serde(rename = "KOMPAKT_MU"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KOMPAKT_MU"))]
    KompaktMu,
    #[cfg_attr(feature = "serde", serde(rename = "SYSTEM_MU"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SYSTEM_MU"))]
    SystemMu,
    #[cfg_attr(feature = "serde", serde(rename = "TEMPERATUR_MU"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TEMPERATUR_MU"))]
    TemperaturMu,
    #[cfg_attr(feature = "serde", serde(rename = "ZUSTANDS_MU"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ZUSTANDS_MU"))]
    ZustandsMu,
    /// Unknown or future variant — produced when deserializing a value
    /// that is not yet known to this version of the library.
    #[cfg_attr(feature = "serde", serde(other, rename = "UNKNOWN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UNKNOWN"))]
    Unknown,
}
impl Geraetetyp {
    /// Returns an iterator over all **known** variants of `Geraetetyp`.
    ///
    /// Unlike [`strum::IntoEnumIterator`] which includes the [`Geraetetyp::Unknown`]
    /// catch-all, this method yields only variants that correspond to values defined
    /// in the BO4E schema.  Use this when building dropdowns, lookup tables, or
    /// generating reports that should only include valid schema values.
    ///
    /// # Example
    /// ```rust,ignore
    /// for v in Geraetetyp::iter_known() {
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
impl sqlx::Type<sqlx::Postgres> for Geraetetyp {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
/// Strum fast path: `AsRef<str>` returns the canonical string without a
/// `serde_json::Value` intermediate, saving an allocation per encode (M-07).
#[cfg(all(feature = "sqlx", feature = "json", feature = "strum"))]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Geraetetyp {
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
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Geraetetyp {
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
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for Geraetetyp {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <String as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        serde_json::from_value(serde_json::Value::String(s))
            .map_err(|e| Box::new(e) as sqlx::error::BoxDynError)
    }
}
#[cfg(all(test, feature = "strum"))]
impl proptest::arbitrary::Arbitrary for Geraetetyp {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        use strum::IntoEnumIterator as _;
        let variants: Vec<Self> = Self::iter().collect();
        proptest::sample::select(variants).boxed()
    }
}
