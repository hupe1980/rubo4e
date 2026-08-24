#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(strum::EnumString, strum::EnumIter, strum::IntoStaticStr)
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Aufzählung von zusätzlichen Informationen zum Status, beispielsweise in Lastgängen oder Zählwerkständen.
#[non_exhaustive]
pub enum Messwertstatuszusatz {
    #[cfg_attr(feature = "serde", serde(rename = "Z84_LEERSTAND"))]
    #[cfg_attr(feature = "strum", strum(serialize = "Z84_LEERSTAND"))]
    Z84Leerstand,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "Z85_REALERZAEHLERUEBERLAUFGEPRUEFT")
    )]
    #[cfg_attr(
        feature = "strum",
        strum(serialize = "Z85_REALERZAEHLERUEBERLAUFGEPRUEFT")
    )]
    Z85Realerzaehlerueberlaufgeprueft,
    #[cfg_attr(feature = "serde", serde(rename = "Z86_PLAUSIBELWGKONTROLLABLESUNG"))]
    #[cfg_attr(
        feature = "strum",
        strum(serialize = "Z86_PLAUSIBELWGKONTROLLABLESUNG")
    )]
    Z86Plausibelwgkontrollablesung,
    #[cfg_attr(feature = "serde", serde(rename = "Z87_PLAUSIBELWGKUNDENHINWEIS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "Z87_PLAUSIBELWGKUNDENHINWEIS"))]
    Z87Plausibelwgkundenhinweis,
    #[cfg_attr(feature = "serde", serde(rename = "ZC3_AUSTAUSCHDESERSATZWERTES"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ZC3_AUSTAUSCHDESERSATZWERTES"))]
    Zc3Austauschdesersatzwertes,
    #[cfg_attr(feature = "serde", serde(rename = "Z88_VERGLEICHSMESSUNG(GEEICHT)"))]
    #[cfg_attr(feature = "strum", strum(serialize = "Z88_VERGLEICHSMESSUNG(GEEICHT)"))]
    Z88VergleichsmessungGeeicht,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "Z89_VERGLEICHSMESSUNG(NICHTGEEICHT)")
    )]
    #[cfg_attr(
        feature = "strum",
        strum(serialize = "Z89_VERGLEICHSMESSUNG(NICHTGEEICHT)")
    )]
    Z89VergleichsmessungNichtgeeicht,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "Z90_MESSWERTNACHBILDUNGAUSGEEICHTENWERTEN")
    )]
    #[cfg_attr(
        feature = "strum",
        strum(serialize = "Z90_MESSWERTNACHBILDUNGAUSGEEICHTENWERTEN")
    )]
    Z90Messwertnachbildungausgeeichtenwerten,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "Z91_MESSWERTNACHBILDUNGAUSNICHTGEEICHTENWERTEN")
    )]
    #[cfg_attr(
        feature = "strum",
        strum(serialize = "Z91_MESSWERTNACHBILDUNGAUSNICHTGEEICHTENWERTEN")
    )]
    Z91Messwertnachbildungausnichtgeeichtenwerten,
    #[cfg_attr(feature = "serde", serde(rename = "Z92_INTERPOLATION"))]
    #[cfg_attr(feature = "strum", strum(serialize = "Z92_INTERPOLATION"))]
    Z92Interpolation,
    #[cfg_attr(feature = "serde", serde(rename = "Z93_HALTEWERT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "Z93_HALTEWERT"))]
    Z93Haltewert,
    #[cfg_attr(feature = "serde", serde(rename = "Z94_BILANZIERUNGNETZABSCHNITT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "Z94_BILANZIERUNGNETZABSCHNITT"))]
    Z94Bilanzierungnetzabschnitt,
    #[cfg_attr(feature = "serde", serde(rename = "Z95_HISTORISCHEMESSWERTE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "Z95_HISTORISCHEMESSWERTE"))]
    Z95Historischemesswerte,
    #[cfg_attr(feature = "serde", serde(rename = "ZJ2_STATISTISCHEMETHODE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ZJ2_STATISTISCHEMETHODE"))]
    Zj2Statistischemethode,
    #[cfg_attr(feature = "serde", serde(rename = "Z74_KEINZUGANG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "Z74_KEINZUGANG"))]
    Z74Keinzugang,
    #[cfg_attr(feature = "serde", serde(rename = "Z75_KOMMUNIKATIONSSTOERUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "Z75_KOMMUNIKATIONSSTOERUNG"))]
    Z75Kommunikationsstoerung,
    #[cfg_attr(feature = "serde", serde(rename = "Z76_NETZAUSFALL"))]
    #[cfg_attr(feature = "strum", strum(serialize = "Z76_NETZAUSFALL"))]
    Z76Netzausfall,
    #[cfg_attr(feature = "serde", serde(rename = "Z77_SPANNUNGSAUSFALL"))]
    #[cfg_attr(feature = "strum", strum(serialize = "Z77_SPANNUNGSAUSFALL"))]
    Z77Spannungsausfall,
    #[cfg_attr(feature = "serde", serde(rename = "Z78_GERAETEWECHSEL"))]
    #[cfg_attr(feature = "strum", strum(serialize = "Z78_GERAETEWECHSEL"))]
    Z78Geraetewechsel,
    #[cfg_attr(feature = "serde", serde(rename = "Z79_KALIBRIERUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "Z79_KALIBRIERUNG"))]
    Z79Kalibrierung,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "Z80_GERAETARBEITETAUSSERHALBDERBETRIEBSBEDINGUNGEN")
    )]
    #[cfg_attr(
        feature = "strum",
        strum(serialize = "Z80_GERAETARBEITETAUSSERHALBDERBETRIEBSBEDINGUNGEN")
    )]
    Z80Geraetarbeitetausserhalbderbetriebsbedingungen,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "Z81_MESSEINRICHTUNGGESTOERT_DEFEKT")
    )]
    #[cfg_attr(
        feature = "strum",
        strum(serialize = "Z81_MESSEINRICHTUNGGESTOERT_DEFEKT")
    )]
    Z81MesseinrichtunggestoertDefekt,
    #[cfg_attr(feature = "serde", serde(rename = "Z82_UNSICHERHEITMESSUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "Z82_UNSICHERHEITMESSUNG"))]
    Z82Unsicherheitmessung,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "Z98_BERUECKSICHTIGUNGSTOERMENGENZAEHLWERK")
    )]
    #[cfg_attr(
        feature = "strum",
        strum(serialize = "Z98_BERUECKSICHTIGUNGSTOERMENGENZAEHLWERK")
    )]
    Z98Beruecksichtigungstoermengenzaehlwerk,
    #[cfg_attr(feature = "serde", serde(rename = "Z99_MENGENUMWERTUNGUNVOLLSTAENDIG"))]
    #[cfg_attr(
        feature = "strum",
        strum(serialize = "Z99_MENGENUMWERTUNGUNVOLLSTAENDIG")
    )]
    Z99Mengenumwertungunvollstaendig,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "ZA0_UHRZEITGESTELLT_SYNCHRONISATION")
    )]
    #[cfg_attr(
        feature = "strum",
        strum(serialize = "ZA0_UHRZEITGESTELLT_SYNCHRONISATION")
    )]
    Za0UhrzeitgestelltSynchronisation,
    #[cfg_attr(feature = "serde", serde(rename = "ZA1_MESSWERTUNPLAUSIBEL"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ZA1_MESSWERTUNPLAUSIBEL"))]
    Za1Messwertunplausibel,
    #[cfg_attr(feature = "serde", serde(rename = "ZC2_TARIFSCHALTGERAETDEFEKT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ZC2_TARIFSCHALTGERAETDEFEKT"))]
    Zc2Tarifschaltgeraetdefekt,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "ZC4_IMPULSWERTIGKEITNICHTAUSREICHEND")
    )]
    #[cfg_attr(
        feature = "strum",
        strum(serialize = "ZC4_IMPULSWERTIGKEITNICHTAUSREICHEND")
    )]
    Zc4Impulswertigkeitnichtausreichend,
    #[cfg_attr(feature = "serde", serde(rename = "ZA3_FALSCHERWANDLERFAKTOR"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ZA3_FALSCHERWANDLERFAKTOR"))]
    Za3Falscherwandlerfaktor,
    #[cfg_attr(feature = "serde", serde(rename = "ZA4_FEHLERHAFTEABLESUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ZA4_FEHLERHAFTEABLESUNG"))]
    Za4Fehlerhafteablesung,
    #[cfg_attr(feature = "serde", serde(rename = "ZA5_AENDERUNGDERBERECHNUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ZA5_AENDERUNGDERBERECHNUNG"))]
    Za5Aenderungderberechnung,
    #[cfg_attr(feature = "serde", serde(rename = "ZA6_UMBAUDERMESSLOKATION"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ZA6_UMBAUDERMESSLOKATION"))]
    Za6Umbaudermesslokation,
    #[cfg_attr(feature = "serde", serde(rename = "ZA7_DATENBEARBEITUNGSFEHLER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ZA7_DATENBEARBEITUNGSFEHLER"))]
    Za7Datenbearbeitungsfehler,
    #[cfg_attr(feature = "serde", serde(rename = "ZA8_BRENNWERTKORREKTUR"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ZA8_BRENNWERTKORREKTUR"))]
    Za8Brennwertkorrektur,
    #[cfg_attr(feature = "serde", serde(rename = "ZA9_Z-ZAHL-KORREKTUR"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ZA9_Z-ZAHL-KORREKTUR"))]
    Za9ZZahlKorrektur,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "ZB0_STOERUNG_DEFEKTMESSEINRICHTUNG")
    )]
    #[cfg_attr(
        feature = "strum",
        strum(serialize = "ZB0_STOERUNG_DEFEKTMESSEINRICHTUNG")
    )]
    Zb0StoerungDefektmesseinrichtung,
    #[cfg_attr(feature = "serde", serde(rename = "ZB9_AENDERUNGTARIFSCHALTZEITEN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ZB9_AENDERUNGTARIFSCHALTZEITEN"))]
    Zb9Aenderungtarifschaltzeiten,
    #[cfg_attr(feature = "serde", serde(rename = "ZG3_UMSTELLUNGGASQUALITAET"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ZG3_UMSTELLUNGGASQUALITAET"))]
    Zg3Umstellunggasqualitaet,
    /// Unknown or future variant — produced when deserializing a value
    /// that is not yet known to this version of the library.
    #[cfg_attr(feature = "serde", serde(other, rename = "UNKNOWN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UNKNOWN"))]
    Unknown,
}
impl Messwertstatuszusatz {
    /// All variants defined by the BO4E schema, in declaration order.
    ///
    /// Excludes the forward-compatibility [`Messwertstatuszusatz::Unknown`] catch-all, so this
    /// is exactly the set of values that appear on the wire.  Available **without**
    /// the `strum` feature — use it to drift-guard SQL `CHECK` lists and mappings.
    pub const VARIANTS: &'static [Self] = &[
        Self::Z84Leerstand,
        Self::Z85Realerzaehlerueberlaufgeprueft,
        Self::Z86Plausibelwgkontrollablesung,
        Self::Z87Plausibelwgkundenhinweis,
        Self::Zc3Austauschdesersatzwertes,
        Self::Z88VergleichsmessungGeeicht,
        Self::Z89VergleichsmessungNichtgeeicht,
        Self::Z90Messwertnachbildungausgeeichtenwerten,
        Self::Z91Messwertnachbildungausnichtgeeichtenwerten,
        Self::Z92Interpolation,
        Self::Z93Haltewert,
        Self::Z94Bilanzierungnetzabschnitt,
        Self::Z95Historischemesswerte,
        Self::Zj2Statistischemethode,
        Self::Z74Keinzugang,
        Self::Z75Kommunikationsstoerung,
        Self::Z76Netzausfall,
        Self::Z77Spannungsausfall,
        Self::Z78Geraetewechsel,
        Self::Z79Kalibrierung,
        Self::Z80Geraetarbeitetausserhalbderbetriebsbedingungen,
        Self::Z81MesseinrichtunggestoertDefekt,
        Self::Z82Unsicherheitmessung,
        Self::Z98Beruecksichtigungstoermengenzaehlwerk,
        Self::Z99Mengenumwertungunvollstaendig,
        Self::Za0UhrzeitgestelltSynchronisation,
        Self::Za1Messwertunplausibel,
        Self::Zc2Tarifschaltgeraetdefekt,
        Self::Zc4Impulswertigkeitnichtausreichend,
        Self::Za3Falscherwandlerfaktor,
        Self::Za4Fehlerhafteablesung,
        Self::Za5Aenderungderberechnung,
        Self::Za6Umbaudermesslokation,
        Self::Za7Datenbearbeitungsfehler,
        Self::Za8Brennwertkorrektur,
        Self::Za9ZZahlKorrektur,
        Self::Zb0StoerungDefektmesseinrichtung,
        Self::Zb9Aenderungtarifschaltzeiten,
        Self::Zg3Umstellunggasqualitaet,
    ];
    /// Number of schema-defined variants (equal to `VARIANTS.len()`), excluding the
    /// [`Messwertstatuszusatz::Unknown`] catch-all.  Stable for this schema version.
    pub const COUNT: usize = Self::VARIANTS.len();
    /// Returns an iterator over all **known** variants of `Messwertstatuszusatz`.
    ///
    /// Yields only variants that correspond to values defined in the BO4E schema
    /// (i.e. [`Self::VARIANTS`]), never the [`Messwertstatuszusatz::Unknown`] catch-all.
    /// Available **without** the `strum` feature.
    ///
    /// # Example
    /// ```
    /// # use rubo4e::current::Messwertstatuszusatz;
    /// // Never yields the `Unknown` catch-all, so the count matches `COUNT`.
    /// assert_eq!(Messwertstatuszusatz::iter_known().count(), Messwertstatuszusatz::COUNT);
    /// assert!(Messwertstatuszusatz::iter_known().all(|v| v.is_known()));
    /// ```
    pub fn iter_known() -> impl Iterator<Item = Self> + Clone {
        Self::VARIANTS.iter().copied()
    }
    /// Returns the canonical BO4E wire string (SCREAMING_SNAKE_CASE) for this value.
    ///
    /// [`Messwertstatuszusatz::Unknown`] renders as `"UNKNOWN"`, matching its serialized form.
    pub const fn as_wire(&self) -> &'static str {
        match self {
            Self::Z84Leerstand => "Z84_LEERSTAND",
            Self::Z85Realerzaehlerueberlaufgeprueft => "Z85_REALERZAEHLERUEBERLAUFGEPRUEFT",
            Self::Z86Plausibelwgkontrollablesung => "Z86_PLAUSIBELWGKONTROLLABLESUNG",
            Self::Z87Plausibelwgkundenhinweis => "Z87_PLAUSIBELWGKUNDENHINWEIS",
            Self::Zc3Austauschdesersatzwertes => "ZC3_AUSTAUSCHDESERSATZWERTES",
            Self::Z88VergleichsmessungGeeicht => "Z88_VERGLEICHSMESSUNG(GEEICHT)",
            Self::Z89VergleichsmessungNichtgeeicht => "Z89_VERGLEICHSMESSUNG(NICHTGEEICHT)",
            Self::Z90Messwertnachbildungausgeeichtenwerten => {
                "Z90_MESSWERTNACHBILDUNGAUSGEEICHTENWERTEN"
            }
            Self::Z91Messwertnachbildungausnichtgeeichtenwerten => {
                "Z91_MESSWERTNACHBILDUNGAUSNICHTGEEICHTENWERTEN"
            }
            Self::Z92Interpolation => "Z92_INTERPOLATION",
            Self::Z93Haltewert => "Z93_HALTEWERT",
            Self::Z94Bilanzierungnetzabschnitt => "Z94_BILANZIERUNGNETZABSCHNITT",
            Self::Z95Historischemesswerte => "Z95_HISTORISCHEMESSWERTE",
            Self::Zj2Statistischemethode => "ZJ2_STATISTISCHEMETHODE",
            Self::Z74Keinzugang => "Z74_KEINZUGANG",
            Self::Z75Kommunikationsstoerung => "Z75_KOMMUNIKATIONSSTOERUNG",
            Self::Z76Netzausfall => "Z76_NETZAUSFALL",
            Self::Z77Spannungsausfall => "Z77_SPANNUNGSAUSFALL",
            Self::Z78Geraetewechsel => "Z78_GERAETEWECHSEL",
            Self::Z79Kalibrierung => "Z79_KALIBRIERUNG",
            Self::Z80Geraetarbeitetausserhalbderbetriebsbedingungen => {
                "Z80_GERAETARBEITETAUSSERHALBDERBETRIEBSBEDINGUNGEN"
            }
            Self::Z81MesseinrichtunggestoertDefekt => "Z81_MESSEINRICHTUNGGESTOERT_DEFEKT",
            Self::Z82Unsicherheitmessung => "Z82_UNSICHERHEITMESSUNG",
            Self::Z98Beruecksichtigungstoermengenzaehlwerk => {
                "Z98_BERUECKSICHTIGUNGSTOERMENGENZAEHLWERK"
            }
            Self::Z99Mengenumwertungunvollstaendig => "Z99_MENGENUMWERTUNGUNVOLLSTAENDIG",
            Self::Za0UhrzeitgestelltSynchronisation => "ZA0_UHRZEITGESTELLT_SYNCHRONISATION",
            Self::Za1Messwertunplausibel => "ZA1_MESSWERTUNPLAUSIBEL",
            Self::Zc2Tarifschaltgeraetdefekt => "ZC2_TARIFSCHALTGERAETDEFEKT",
            Self::Zc4Impulswertigkeitnichtausreichend => "ZC4_IMPULSWERTIGKEITNICHTAUSREICHEND",
            Self::Za3Falscherwandlerfaktor => "ZA3_FALSCHERWANDLERFAKTOR",
            Self::Za4Fehlerhafteablesung => "ZA4_FEHLERHAFTEABLESUNG",
            Self::Za5Aenderungderberechnung => "ZA5_AENDERUNGDERBERECHNUNG",
            Self::Za6Umbaudermesslokation => "ZA6_UMBAUDERMESSLOKATION",
            Self::Za7Datenbearbeitungsfehler => "ZA7_DATENBEARBEITUNGSFEHLER",
            Self::Za8Brennwertkorrektur => "ZA8_BRENNWERTKORREKTUR",
            Self::Za9ZZahlKorrektur => "ZA9_Z-ZAHL-KORREKTUR",
            Self::Zb0StoerungDefektmesseinrichtung => "ZB0_STOERUNG_DEFEKTMESSEINRICHTUNG",
            Self::Zb9Aenderungtarifschaltzeiten => "ZB9_AENDERUNGTARIFSCHALTZEITEN",
            Self::Zg3Umstellunggasqualitaet => "ZG3_UMSTELLUNGGASQUALITAET",
            Self::Unknown => "UNKNOWN",
        }
    }
    /// **Strictly** parses a BO4E wire string into a known variant.
    ///
    /// Unlike the lenient `serde` / [`FromStr`](std::str::FromStr) path — which maps
    /// any unrecognized value (a typo, a legacy code, or a value from a newer schema)
    /// to [`Messwertstatuszusatz::Unknown`] — this returns
    /// [`Err`](crate::error::UnknownVariant) for values not defined in this schema
    /// version, including the literal `"UNKNOWN"`.  Use it at the ingest boundary to
    /// reject bad values instead of silently degrading them.
    ///
    /// # Example
    /// ```
    /// # use rubo4e::current::Messwertstatuszusatz;
    /// assert_eq!(Messwertstatuszusatz::from_wire("Z84_LEERSTAND"), Ok(Messwertstatuszusatz::Z84Leerstand));
    /// // Out-of-schema values are rejected rather than degraded:
    /// assert!(Messwertstatuszusatz::from_wire("NOT_A_REAL_VALUE").is_err());
    /// // …including the `Unknown` catch-all's own wire spelling:
    /// assert!(Messwertstatuszusatz::from_wire("UNKNOWN").is_err());
    /// ```
    pub fn from_wire(s: &str) -> Result<Self, crate::error::UnknownVariant> {
        match s {
            "Z84_LEERSTAND" => Ok(Self::Z84Leerstand),
            "Z85_REALERZAEHLERUEBERLAUFGEPRUEFT" => Ok(Self::Z85Realerzaehlerueberlaufgeprueft),
            "Z86_PLAUSIBELWGKONTROLLABLESUNG" => Ok(Self::Z86Plausibelwgkontrollablesung),
            "Z87_PLAUSIBELWGKUNDENHINWEIS" => Ok(Self::Z87Plausibelwgkundenhinweis),
            "ZC3_AUSTAUSCHDESERSATZWERTES" => Ok(Self::Zc3Austauschdesersatzwertes),
            "Z88_VERGLEICHSMESSUNG(GEEICHT)" => Ok(Self::Z88VergleichsmessungGeeicht),
            "Z89_VERGLEICHSMESSUNG(NICHTGEEICHT)" => Ok(Self::Z89VergleichsmessungNichtgeeicht),
            "Z90_MESSWERTNACHBILDUNGAUSGEEICHTENWERTEN" => {
                Ok(Self::Z90Messwertnachbildungausgeeichtenwerten)
            }
            "Z91_MESSWERTNACHBILDUNGAUSNICHTGEEICHTENWERTEN" => {
                Ok(Self::Z91Messwertnachbildungausnichtgeeichtenwerten)
            }
            "Z92_INTERPOLATION" => Ok(Self::Z92Interpolation),
            "Z93_HALTEWERT" => Ok(Self::Z93Haltewert),
            "Z94_BILANZIERUNGNETZABSCHNITT" => Ok(Self::Z94Bilanzierungnetzabschnitt),
            "Z95_HISTORISCHEMESSWERTE" => Ok(Self::Z95Historischemesswerte),
            "ZJ2_STATISTISCHEMETHODE" => Ok(Self::Zj2Statistischemethode),
            "Z74_KEINZUGANG" => Ok(Self::Z74Keinzugang),
            "Z75_KOMMUNIKATIONSSTOERUNG" => Ok(Self::Z75Kommunikationsstoerung),
            "Z76_NETZAUSFALL" => Ok(Self::Z76Netzausfall),
            "Z77_SPANNUNGSAUSFALL" => Ok(Self::Z77Spannungsausfall),
            "Z78_GERAETEWECHSEL" => Ok(Self::Z78Geraetewechsel),
            "Z79_KALIBRIERUNG" => Ok(Self::Z79Kalibrierung),
            "Z80_GERAETARBEITETAUSSERHALBDERBETRIEBSBEDINGUNGEN" => {
                Ok(Self::Z80Geraetarbeitetausserhalbderbetriebsbedingungen)
            }
            "Z81_MESSEINRICHTUNGGESTOERT_DEFEKT" => Ok(Self::Z81MesseinrichtunggestoertDefekt),
            "Z82_UNSICHERHEITMESSUNG" => Ok(Self::Z82Unsicherheitmessung),
            "Z98_BERUECKSICHTIGUNGSTOERMENGENZAEHLWERK" => {
                Ok(Self::Z98Beruecksichtigungstoermengenzaehlwerk)
            }
            "Z99_MENGENUMWERTUNGUNVOLLSTAENDIG" => Ok(Self::Z99Mengenumwertungunvollstaendig),
            "ZA0_UHRZEITGESTELLT_SYNCHRONISATION" => Ok(Self::Za0UhrzeitgestelltSynchronisation),
            "ZA1_MESSWERTUNPLAUSIBEL" => Ok(Self::Za1Messwertunplausibel),
            "ZC2_TARIFSCHALTGERAETDEFEKT" => Ok(Self::Zc2Tarifschaltgeraetdefekt),
            "ZC4_IMPULSWERTIGKEITNICHTAUSREICHEND" => Ok(Self::Zc4Impulswertigkeitnichtausreichend),
            "ZA3_FALSCHERWANDLERFAKTOR" => Ok(Self::Za3Falscherwandlerfaktor),
            "ZA4_FEHLERHAFTEABLESUNG" => Ok(Self::Za4Fehlerhafteablesung),
            "ZA5_AENDERUNGDERBERECHNUNG" => Ok(Self::Za5Aenderungderberechnung),
            "ZA6_UMBAUDERMESSLOKATION" => Ok(Self::Za6Umbaudermesslokation),
            "ZA7_DATENBEARBEITUNGSFEHLER" => Ok(Self::Za7Datenbearbeitungsfehler),
            "ZA8_BRENNWERTKORREKTUR" => Ok(Self::Za8Brennwertkorrektur),
            "ZA9_Z-ZAHL-KORREKTUR" => Ok(Self::Za9ZZahlKorrektur),
            "ZB0_STOERUNG_DEFEKTMESSEINRICHTUNG" => Ok(Self::Zb0StoerungDefektmesseinrichtung),
            "ZB9_AENDERUNGTARIFSCHALTZEITEN" => Ok(Self::Zb9Aenderungtarifschaltzeiten),
            "ZG3_UMSTELLUNGGASQUALITAET" => Ok(Self::Zg3Umstellunggasqualitaet),
            other => Err(crate::error::UnknownVariant::new(other)),
        }
    }
    /// Returns `true` if this value is the forward-compatibility
    /// [`Messwertstatuszusatz::Unknown`] catch-all (an out-of-schema value).
    pub const fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
    /// Returns `true` if this value is a known, schema-defined variant.
    pub const fn is_known(&self) -> bool {
        !self.is_unknown()
    }
}
impl std::fmt::Display for Messwertstatuszusatz {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_wire())
    }
}
impl AsRef<str> for Messwertstatuszusatz {
    fn as_ref(&self) -> &str {
        self.as_wire()
    }
}
#[cfg(feature = "versioned")]
impl crate::bo4e_enum_sealed::Sealed for Messwertstatuszusatz {}
#[cfg(feature = "versioned")]
impl crate::Bo4eEnum for Messwertstatuszusatz {
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
impl crate::Bo4eStrict for Messwertstatuszusatz {
    fn collect_unknown_enums(&self, path: &str, out: &mut Vec<String>) {
        if self.is_unknown() {
            out.push(path.to_owned());
        }
    }
}
#[cfg(feature = "sqlx")]
impl sqlx::Type<sqlx::Postgres> for Messwertstatuszusatz {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
/// Encodes as the canonical BO4E wire string, borrowed from `as_wire` — no
/// intermediate `String` or `serde_json::Value` is allocated.
#[cfg(feature = "sqlx")]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Messwertstatuszusatz {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        let s: &str = self.as_wire();
        <&str as sqlx::Encode<'q, sqlx::Postgres>>::encode_by_ref(&s, buf)
    }
}
/// Decodes leniently, matching the `serde` path: a value the schema does not
/// define becomes [`Messwertstatuszusatz::Unknown`] rather than a decode error, so a
/// database row written by a newer schema version still reads back.
///
/// Use [`Messwertstatuszusatz::from_wire`] on a `String` column, or check
/// [`Messwertstatuszusatz::is_known`], where out-of-schema values must be rejected.
#[cfg(feature = "sqlx")]
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for Messwertstatuszusatz {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <&str as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        Ok(Self::from_wire(s).unwrap_or(Self::Unknown))
    }
}
/// Lets `Vec<Messwertstatuszusatz>` bind to a `TEXT[]` column.  Only this crate can
/// provide it: the trait and the enum are both foreign to any consumer, so the
/// orphan rule rules out a downstream impl.
#[cfg(feature = "sqlx")]
impl sqlx::postgres::PgHasArrayType for Messwertstatuszusatz {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::postgres::PgHasArrayType>::array_type_info()
    }
}
#[cfg(test)]
impl proptest::arbitrary::Arbitrary for Messwertstatuszusatz {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        proptest::sample::select(Self::VARIANTS.to_vec()).boxed()
    }
}
