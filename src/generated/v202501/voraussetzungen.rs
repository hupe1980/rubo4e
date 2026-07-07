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
/// Voraussetzungen, die erfüllt sein müssen, damit dieser Tarif zur Anwendung kommen kann.
#[non_exhaustive]
pub enum Voraussetzungen {
    #[cfg_attr(feature = "serde", serde(rename = "EINZUGSERMAECHTIGUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "EINZUGSERMAECHTIGUNG"))]
    Einzugsermaechtigung,
    #[cfg_attr(feature = "serde", serde(rename = "ZEITPUNKT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ZEITPUNKT"))]
    Zeitpunkt,
    #[cfg_attr(feature = "serde", serde(rename = "LIEFERANBINDUNG_EINE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "LIEFERANBINDUNG_EINE"))]
    LieferanbindungEine,
    #[cfg_attr(feature = "serde", serde(rename = "LIEFERANBINDUNG_ALLE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "LIEFERANBINDUNG_ALLE"))]
    LieferanbindungAlle,
    #[cfg_attr(feature = "serde", serde(rename = "GEWERBE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GEWERBE"))]
    Gewerbe,
    #[cfg_attr(feature = "serde", serde(rename = "LASTPROFIL"))]
    #[cfg_attr(feature = "strum", strum(serialize = "LASTPROFIL"))]
    Lastprofil,
    #[cfg_attr(feature = "serde", serde(rename = "ZAEHLERTYP_GROESSE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ZAEHLERTYP_GROESSE"))]
    ZaehlertypGroesse,
    #[cfg_attr(feature = "serde", serde(rename = "AUSSCHLUSS_GROSSVERBRAUCHER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "AUSSCHLUSS_GROSSVERBRAUCHER"))]
    AusschlussGrossverbraucher,
    #[cfg_attr(feature = "serde", serde(rename = "NEUKUNDE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "NEUKUNDE"))]
    Neukunde,
    #[cfg_attr(feature = "serde", serde(rename = "BESTIMMTE_VERTRAGSFORMALITAETEN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BESTIMMTE_VERTRAGSFORMALITAETEN"))]
    BestimmteVertragsformalitaeten,
    #[cfg_attr(feature = "serde", serde(rename = "SELBSTABLESUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SELBSTABLESUNG"))]
    Selbstablesung,
    #[cfg_attr(feature = "serde", serde(rename = "ONLINEVORAUSSETZUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ONLINEVORAUSSETZUNG"))]
    Onlinevoraussetzung,
    #[cfg_attr(feature = "serde", serde(rename = "MINDESTUMSATZ"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MINDESTUMSATZ"))]
    Mindestumsatz,
    #[cfg_attr(feature = "serde", serde(rename = "ZUSATZPRODUKT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ZUSATZPRODUKT"))]
    Zusatzprodukt,
    #[cfg_attr(feature = "serde", serde(rename = "NEUKUNDE_MIT_VORAUSSETZUNGEN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "NEUKUNDE_MIT_VORAUSSETZUNGEN"))]
    NeukundeMitVoraussetzungen,
    #[cfg_attr(feature = "serde", serde(rename = "DIREKTVERTRIEB"))]
    #[cfg_attr(feature = "strum", strum(serialize = "DIREKTVERTRIEB"))]
    Direktvertrieb,
    #[cfg_attr(feature = "serde", serde(rename = "ANSCHLUSSART"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ANSCHLUSSART"))]
    Anschlussart,
    #[cfg_attr(feature = "serde", serde(rename = "ANSCHLUSSWERT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ANSCHLUSSWERT"))]
    Anschlusswert,
    #[cfg_attr(feature = "serde", serde(rename = "ALTER_KUNDENANLAGE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ALTER_KUNDENANLAGE"))]
    AlterKundenanlage,
    #[cfg_attr(feature = "serde", serde(rename = "ANLAGEBESCHAFFENHEIT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ANLAGEBESCHAFFENHEIT"))]
    Anlagebeschaffenheit,
    #[cfg_attr(feature = "serde", serde(rename = "BETRIEBSSTUNDENBEGRENZUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BETRIEBSSTUNDENBEGRENZUNG"))]
    Betriebsstundenbegrenzung,
    #[cfg_attr(feature = "serde", serde(rename = "FREIGABEZEITEN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "FREIGABEZEITEN"))]
    Freigabezeiten,
    #[cfg_attr(feature = "serde", serde(rename = "FAMILIENSTRUKTUR"))]
    #[cfg_attr(feature = "strum", strum(serialize = "FAMILIENSTRUKTUR"))]
    Familienstruktur,
    #[cfg_attr(feature = "serde", serde(rename = "MITGLIEDSCHAFT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MITGLIEDSCHAFT"))]
    Mitgliedschaft,
    #[cfg_attr(feature = "serde", serde(rename = "STAATLICHE_FOERDERUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "STAATLICHE_FOERDERUNG"))]
    StaatlicheFoerderung,
    #[cfg_attr(feature = "serde", serde(rename = "BESONDERE_VERBRAUCHSSTELLE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BESONDERE_VERBRAUCHSSTELLE"))]
    BesondereVerbrauchsstelle,
    #[cfg_attr(feature = "serde", serde(rename = "NIEDRIGENERGIE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "NIEDRIGENERGIE"))]
    Niedrigenergie,
    #[cfg_attr(feature = "serde", serde(rename = "ORTSTEILE_LIEFERGEBIET"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ORTSTEILE_LIEFERGEBIET"))]
    OrtsteileLiefergebiet,
    #[cfg_attr(feature = "serde", serde(rename = "WAERMEBEDARF_ERDGAS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "WAERMEBEDARF_ERDGAS"))]
    WaermebedarfErdgas,
    #[cfg_attr(feature = "serde", serde(rename = "MAX_ZAEHLER_LIEFERSTELLEN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MAX_ZAEHLER_LIEFERSTELLEN"))]
    MaxZaehlerLieferstellen,
    #[cfg_attr(feature = "serde", serde(rename = "LIEFERUNGSBESCHRAENKUNG_GASART"))]
    #[cfg_attr(feature = "strum", strum(serialize = "LIEFERUNGSBESCHRAENKUNG_GASART"))]
    LieferungsbeschraenkungGasart,
    #[cfg_attr(feature = "serde", serde(rename = "KOMBI_BONI"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KOMBI_BONI"))]
    KombiBoni,
    #[cfg_attr(feature = "serde", serde(rename = "ALTVERTRAG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ALTVERTRAG"))]
    Altvertrag,
    #[cfg_attr(feature = "serde", serde(rename = "VORGESCHRIEBENE_ZUSATZANLAGE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "VORGESCHRIEBENE_ZUSATZANLAGE"))]
    VorgeschriebeneZusatzanlage,
    #[cfg_attr(feature = "serde", serde(rename = "MEHRERE_ZAEHLER_ABNAHMESTELLEN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MEHRERE_ZAEHLER_ABNAHMESTELLEN"))]
    MehrereZaehlerAbnahmestellen,
    #[cfg_attr(feature = "serde", serde(rename = "BESTIMMTER_ABNAHMEFALL"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BESTIMMTER_ABNAHMEFALL"))]
    BestimmterAbnahmefall,
    #[cfg_attr(feature = "serde", serde(rename = "ZUSATZMODALITAET"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ZUSATZMODALITAET"))]
    Zusatzmodalitaet,
    #[cfg_attr(feature = "serde", serde(rename = "NACHWEIS_ZAHLUNGSFAEHIGKEIT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "NACHWEIS_ZAHLUNGSFAEHIGKEIT"))]
    NachweisZahlungsfaehigkeit,
    #[cfg_attr(feature = "serde", serde(rename = "UMSTELLUNG_ENERGIEART"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UMSTELLUNG_ENERGIEART"))]
    UmstellungEnergieart,
    /// Unknown or future variant — produced when deserializing a value
    /// that is not yet known to this version of the library.
    #[cfg_attr(feature = "serde", serde(other, rename = "UNKNOWN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UNKNOWN"))]
    Unknown,
}
impl Voraussetzungen {
    /// Returns an iterator over all **known** variants of `Voraussetzungen`.
    ///
    /// Unlike [`strum::IntoEnumIterator`] which includes the [`Voraussetzungen::Unknown`]
    /// catch-all, this method yields only variants that correspond to values defined
    /// in the BO4E schema.  Use this when building dropdowns, lookup tables, or
    /// generating reports that should only include valid schema values.
    ///
    /// # Example
    /// ```rust,ignore
    /// for v in Voraussetzungen::iter_known() {
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
impl sqlx::Type<sqlx::Postgres> for Voraussetzungen {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
/// Strum fast path: `AsRef<str>` returns the canonical string without a
/// `serde_json::Value` intermediate, saving an allocation per encode (M-07).
#[cfg(all(feature = "sqlx", feature = "json", feature = "strum"))]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Voraussetzungen {
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
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Voraussetzungen {
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
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for Voraussetzungen {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <String as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        serde_json::from_value(serde_json::Value::String(s))
            .map_err(|e| Box::new(e) as sqlx::error::BoxDynError)
    }
}
#[cfg(all(test, feature = "strum"))]
impl proptest::arbitrary::Arbitrary for Voraussetzungen {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        use strum::IntoEnumIterator as _;
        let variants: Vec<Self> = Self::iter().collect();
        proptest::sample::select(variants).boxed()
    }
}
