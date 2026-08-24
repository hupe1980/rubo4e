#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(strum::EnumString, strum::EnumIter, strum::IntoStaticStr)
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
    ///
    /// **Wire spelling:** `INTELLIGENTES_MESSYSTEM` (two `s`).  ⚠ BO4E spells the
    /// *same* iMSys concept differently across BOs: `Zaehlertyp::IntelligentesMesssystem`
    /// uses `INTELLIGENTES_MESSSYSTEM` (three `s`).  This divergence is upstream, not a
    /// `rubo4e` transcription error; each BO keeps its own canonical spelling.
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
    /// All variants defined by the BO4E schema, in declaration order.
    ///
    /// Excludes the forward-compatibility [`Geraetetyp::Unknown`] catch-all, so this
    /// is exactly the set of values that appear on the wire.  Available **without**
    /// the `strum` feature — use it to drift-guard SQL `CHECK` lists and mappings.
    pub const VARIANTS: &'static [Self] = &[
        Self::Multiplexanlage,
        Self::Pauschalanlage,
        Self::Verstaerkeranlage,
        Self::Summationsgeraet,
        Self::Impulsgeber,
        Self::Mengenumwerter,
        Self::Stromwandler,
        Self::Spannungswandler,
        Self::Kombimesswandler,
        Self::Blockstromwandler,
        Self::Datenlogger,
        Self::Kommunikationsanschluss,
        Self::Modem,
        Self::Telekommunikationseinrichtung,
        Self::ModerneMesseinrichtung,
        Self::IntelligentesMessystem,
        Self::Steuereinrichtung,
        Self::Tarifschaltgeraet,
        Self::Rundsteuerempfaenger,
        Self::OptionaleZusZaehleinrichtung,
        Self::MesswandlersatzImsMme,
        Self::KombimesswandlerImsMme,
        Self::TarifschaltgeraetImsMme,
        Self::RundsteuerempfaengerImsMme,
        Self::TemperaturKompensation,
        Self::HoechstbelastungsAnzeiger,
        Self::SonstigesGeraet,
        Self::Edl21,
        Self::Edl40Zaehleraufsatz,
        Self::Edl40,
        Self::Telefonanschluss,
        Self::ModemGsm,
        Self::ModemGprs,
        Self::ModemFunk,
        Self::ModemGsmOLg,
        Self::ModemGsmMLg,
        Self::ModemFestnetz,
        Self::ModemGprsMLg,
        Self::PlcKom,
        Self::EthernetKom,
        Self::DslKom,
        Self::LteKom,
        Self::KompaktMu,
        Self::SystemMu,
        Self::TemperaturMu,
        Self::ZustandsMu,
    ];
    /// Number of schema-defined variants (equal to `VARIANTS.len()`), excluding the
    /// [`Geraetetyp::Unknown`] catch-all.  Stable for this schema version.
    pub const COUNT: usize = Self::VARIANTS.len();
    /// Returns an iterator over all **known** variants of `Geraetetyp`.
    ///
    /// Yields only variants that correspond to values defined in the BO4E schema
    /// (i.e. [`Self::VARIANTS`]), never the [`Geraetetyp::Unknown`] catch-all.
    /// Available **without** the `strum` feature.
    ///
    /// # Example
    /// ```
    /// # use rubo4e::current::Geraetetyp;
    /// // Never yields the `Unknown` catch-all, so the count matches `COUNT`.
    /// assert_eq!(Geraetetyp::iter_known().count(), Geraetetyp::COUNT);
    /// assert!(Geraetetyp::iter_known().all(|v| v.is_known()));
    /// ```
    pub fn iter_known() -> impl Iterator<Item = Self> + Clone {
        Self::VARIANTS.iter().copied()
    }
    /// Returns the canonical BO4E wire string (SCREAMING_SNAKE_CASE) for this value.
    ///
    /// [`Geraetetyp::Unknown`] renders as `"UNKNOWN"`, matching its serialized form.
    pub const fn as_wire(&self) -> &'static str {
        match self {
            Self::Multiplexanlage => "MULTIPLEXANLAGE",
            Self::Pauschalanlage => "PAUSCHALANLAGE",
            Self::Verstaerkeranlage => "VERSTAERKERANLAGE",
            Self::Summationsgeraet => "SUMMATIONSGERAET",
            Self::Impulsgeber => "IMPULSGEBER",
            Self::Mengenumwerter => "MENGENUMWERTER",
            Self::Stromwandler => "STROMWANDLER",
            Self::Spannungswandler => "SPANNUNGSWANDLER",
            Self::Kombimesswandler => "KOMBIMESSWANDLER",
            Self::Blockstromwandler => "BLOCKSTROMWANDLER",
            Self::Datenlogger => "DATENLOGGER",
            Self::Kommunikationsanschluss => "KOMMUNIKATIONSANSCHLUSS",
            Self::Modem => "MODEM",
            Self::Telekommunikationseinrichtung => "TELEKOMMUNIKATIONSEINRICHTUNG",
            Self::ModerneMesseinrichtung => "MODERNE_MESSEINRICHTUNG",
            Self::IntelligentesMessystem => "INTELLIGENTES_MESSYSTEM",
            Self::Steuereinrichtung => "STEUEREINRICHTUNG",
            Self::Tarifschaltgeraet => "TARIFSCHALTGERAET",
            Self::Rundsteuerempfaenger => "RUNDSTEUEREMPFAENGER",
            Self::OptionaleZusZaehleinrichtung => "OPTIONALE_ZUS_ZAEHLEINRICHTUNG",
            Self::MesswandlersatzImsMme => "MESSWANDLERSATZ_IMS_MME",
            Self::KombimesswandlerImsMme => "KOMBIMESSWANDLER_IMS_MME",
            Self::TarifschaltgeraetImsMme => "TARIFSCHALTGERAET_IMS_MME",
            Self::RundsteuerempfaengerImsMme => "RUNDSTEUEREMPFAENGER_IMS_MME",
            Self::TemperaturKompensation => "TEMPERATUR_KOMPENSATION",
            Self::HoechstbelastungsAnzeiger => "HOECHSTBELASTUNGS_ANZEIGER",
            Self::SonstigesGeraet => "SONSTIGES_GERAET",
            Self::Edl21 => "EDL_21",
            Self::Edl40Zaehleraufsatz => "EDL_40_ZAEHLERAUFSATZ",
            Self::Edl40 => "EDL_40",
            Self::Telefonanschluss => "TELEFONANSCHLUSS",
            Self::ModemGsm => "MODEM_GSM",
            Self::ModemGprs => "MODEM_GPRS",
            Self::ModemFunk => "MODEM_FUNK",
            Self::ModemGsmOLg => "MODEM_GSM_O_LG",
            Self::ModemGsmMLg => "MODEM_GSM_M_LG",
            Self::ModemFestnetz => "MODEM_FESTNETZ",
            Self::ModemGprsMLg => "MODEM_GPRS_M_LG",
            Self::PlcKom => "PLC_KOM",
            Self::EthernetKom => "ETHERNET_KOM",
            Self::DslKom => "DSL_KOM",
            Self::LteKom => "LTE_KOM",
            Self::KompaktMu => "KOMPAKT_MU",
            Self::SystemMu => "SYSTEM_MU",
            Self::TemperaturMu => "TEMPERATUR_MU",
            Self::ZustandsMu => "ZUSTANDS_MU",
            Self::Unknown => "UNKNOWN",
        }
    }
    /// **Strictly** parses a BO4E wire string into a known variant.
    ///
    /// Unlike the lenient `serde` / [`FromStr`](std::str::FromStr) path — which maps
    /// any unrecognized value (a typo, a legacy code, or a value from a newer schema)
    /// to [`Geraetetyp::Unknown`] — this returns
    /// [`Err`](crate::error::UnknownVariant) for values not defined in this schema
    /// version, including the literal `"UNKNOWN"`.  Use it at the ingest boundary to
    /// reject bad values instead of silently degrading them.
    ///
    /// # Example
    /// ```
    /// # use rubo4e::current::Geraetetyp;
    /// assert_eq!(Geraetetyp::from_wire("MULTIPLEXANLAGE"), Ok(Geraetetyp::Multiplexanlage));
    /// // Out-of-schema values are rejected rather than degraded:
    /// assert!(Geraetetyp::from_wire("NOT_A_REAL_VALUE").is_err());
    /// // …including the `Unknown` catch-all's own wire spelling:
    /// assert!(Geraetetyp::from_wire("UNKNOWN").is_err());
    /// ```
    pub fn from_wire(s: &str) -> Result<Self, crate::error::UnknownVariant> {
        match s {
            "MULTIPLEXANLAGE" => Ok(Self::Multiplexanlage),
            "PAUSCHALANLAGE" => Ok(Self::Pauschalanlage),
            "VERSTAERKERANLAGE" => Ok(Self::Verstaerkeranlage),
            "SUMMATIONSGERAET" => Ok(Self::Summationsgeraet),
            "IMPULSGEBER" => Ok(Self::Impulsgeber),
            "MENGENUMWERTER" => Ok(Self::Mengenumwerter),
            "STROMWANDLER" => Ok(Self::Stromwandler),
            "SPANNUNGSWANDLER" => Ok(Self::Spannungswandler),
            "KOMBIMESSWANDLER" => Ok(Self::Kombimesswandler),
            "BLOCKSTROMWANDLER" => Ok(Self::Blockstromwandler),
            "DATENLOGGER" => Ok(Self::Datenlogger),
            "KOMMUNIKATIONSANSCHLUSS" => Ok(Self::Kommunikationsanschluss),
            "MODEM" => Ok(Self::Modem),
            "TELEKOMMUNIKATIONSEINRICHTUNG" => Ok(Self::Telekommunikationseinrichtung),
            "MODERNE_MESSEINRICHTUNG" => Ok(Self::ModerneMesseinrichtung),
            "INTELLIGENTES_MESSYSTEM" => Ok(Self::IntelligentesMessystem),
            "STEUEREINRICHTUNG" => Ok(Self::Steuereinrichtung),
            "TARIFSCHALTGERAET" => Ok(Self::Tarifschaltgeraet),
            "RUNDSTEUEREMPFAENGER" => Ok(Self::Rundsteuerempfaenger),
            "OPTIONALE_ZUS_ZAEHLEINRICHTUNG" => Ok(Self::OptionaleZusZaehleinrichtung),
            "MESSWANDLERSATZ_IMS_MME" => Ok(Self::MesswandlersatzImsMme),
            "KOMBIMESSWANDLER_IMS_MME" => Ok(Self::KombimesswandlerImsMme),
            "TARIFSCHALTGERAET_IMS_MME" => Ok(Self::TarifschaltgeraetImsMme),
            "RUNDSTEUEREMPFAENGER_IMS_MME" => Ok(Self::RundsteuerempfaengerImsMme),
            "TEMPERATUR_KOMPENSATION" => Ok(Self::TemperaturKompensation),
            "HOECHSTBELASTUNGS_ANZEIGER" => Ok(Self::HoechstbelastungsAnzeiger),
            "SONSTIGES_GERAET" => Ok(Self::SonstigesGeraet),
            "EDL_21" => Ok(Self::Edl21),
            "EDL_40_ZAEHLERAUFSATZ" => Ok(Self::Edl40Zaehleraufsatz),
            "EDL_40" => Ok(Self::Edl40),
            "TELEFONANSCHLUSS" => Ok(Self::Telefonanschluss),
            "MODEM_GSM" => Ok(Self::ModemGsm),
            "MODEM_GPRS" => Ok(Self::ModemGprs),
            "MODEM_FUNK" => Ok(Self::ModemFunk),
            "MODEM_GSM_O_LG" => Ok(Self::ModemGsmOLg),
            "MODEM_GSM_M_LG" => Ok(Self::ModemGsmMLg),
            "MODEM_FESTNETZ" => Ok(Self::ModemFestnetz),
            "MODEM_GPRS_M_LG" => Ok(Self::ModemGprsMLg),
            "PLC_KOM" => Ok(Self::PlcKom),
            "ETHERNET_KOM" => Ok(Self::EthernetKom),
            "DSL_KOM" => Ok(Self::DslKom),
            "LTE_KOM" => Ok(Self::LteKom),
            "KOMPAKT_MU" => Ok(Self::KompaktMu),
            "SYSTEM_MU" => Ok(Self::SystemMu),
            "TEMPERATUR_MU" => Ok(Self::TemperaturMu),
            "ZUSTANDS_MU" => Ok(Self::ZustandsMu),
            other => Err(crate::error::UnknownVariant::new(other)),
        }
    }
    /// Returns `true` if this value is the forward-compatibility
    /// [`Geraetetyp::Unknown`] catch-all (an out-of-schema value).
    pub const fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
    /// Returns `true` if this value is a known, schema-defined variant.
    pub const fn is_known(&self) -> bool {
        !self.is_unknown()
    }
}
impl std::fmt::Display for Geraetetyp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_wire())
    }
}
impl AsRef<str> for Geraetetyp {
    fn as_ref(&self) -> &str {
        self.as_wire()
    }
}
#[cfg(feature = "versioned")]
impl crate::bo4e_enum_sealed::Sealed for Geraetetyp {}
#[cfg(feature = "versioned")]
impl crate::Bo4eEnum for Geraetetyp {
    const VARIANTS: &'static [Self] = Self::VARIANTS;
    const COUNT: usize = Self::COUNT;
    fn as_wire(&self) -> &'static str {
        Self::as_wire(self)
    }
    fn from_wire(s: &str) -> Result<Self, crate::error::UnknownVariant> {
        Self::from_wire(s)
    }
    fn is_unknown(&self) -> bool {
        Self::is_unknown(self)
    }
}
#[cfg(feature = "versioned")]
impl crate::Bo4eStrict for Geraetetyp {
    fn collect_unknown_enums(&self, path: &str, out: &mut Vec<String>) {
        if self.is_unknown() {
            out.push(path.to_owned());
        }
    }
}
#[cfg(feature = "sqlx")]
impl sqlx::Type<sqlx::Postgres> for Geraetetyp {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
/// Encodes as the canonical BO4E wire string, borrowed from `as_wire` — no
/// intermediate `String` or `serde_json::Value` is allocated.
#[cfg(feature = "sqlx")]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Geraetetyp {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        let s: &str = self.as_wire();
        <&str as sqlx::Encode<'q, sqlx::Postgres>>::encode_by_ref(&s, buf)
    }
}
/// Decodes leniently, matching the `serde` path: a value the schema does not
/// define becomes [`Geraetetyp::Unknown`] rather than a decode error, so a
/// database row written by a newer schema version still reads back.
///
/// Use [`Geraetetyp::from_wire`] on a `String` column, or check
/// [`Geraetetyp::is_known`], where out-of-schema values must be rejected.
#[cfg(feature = "sqlx")]
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for Geraetetyp {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <&str as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        Ok(Self::from_wire(s).unwrap_or(Self::Unknown))
    }
}
/// Lets `Vec<Geraetetyp>` bind to a `TEXT[]` column.  Only this crate can
/// provide it: the trait and the enum are both foreign to any consumer, so the
/// orphan rule rules out a downstream impl.
#[cfg(feature = "sqlx")]
impl sqlx::postgres::PgHasArrayType for Geraetetyp {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::postgres::PgHasArrayType>::array_type_info()
    }
}
#[cfg(test)]
impl proptest::arbitrary::Arbitrary for Geraetetyp {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        proptest::sample::select(Self::VARIANTS.to_vec()).boxed()
    }
}
