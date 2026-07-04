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
    /// Returns an iterator over all **known** variants of `Messwertstatuszusatz`.
    ///
    /// Unlike [`strum::IntoEnumIterator`] which includes the [`Messwertstatuszusatz::Unknown`]
    /// catch-all, this method yields only variants that correspond to values defined
    /// in the BO4E schema.  Use this when building dropdowns, lookup tables, or
    /// generating reports that should only include valid schema values.
    ///
    /// # Example
    /// ```rust,ignore
    /// for v in Messwertstatuszusatz::iter_known() {
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
impl sqlx::Type<sqlx::Postgres> for Messwertstatuszusatz {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
/// Strum fast path: `AsRef<str>` returns the canonical string without a
/// `serde_json::Value` intermediate, saving an allocation per encode (M-07).
#[cfg(all(feature = "sqlx", feature = "json", feature = "strum"))]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Messwertstatuszusatz {
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
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Messwertstatuszusatz {
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
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for Messwertstatuszusatz {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <String as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        serde_json::from_value(serde_json::Value::String(s))
            .map_err(|e| Box::new(e) as sqlx::error::BoxDynError)
    }
}
#[cfg(all(test, feature = "strum"))]
impl proptest::arbitrary::Arbitrary for Messwertstatuszusatz {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        use strum::IntoEnumIterator as _;
        let variants: Vec<Self> = Self::iter().collect();
        proptest::sample::select(variants).boxed()
    }
}
