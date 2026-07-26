#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(strum::EnumString, strum::EnumIter, strum::IntoStaticStr)
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
    #[cfg_attr(
        feature = "strum",
        strum(serialize = "BESTIMMTE_VERTRAGSFORMALITAETEN")
    )]
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
    /// All variants defined by the BO4E schema, in declaration order.
    ///
    /// Excludes the forward-compatibility [`Voraussetzungen::Unknown`] catch-all, so this
    /// is exactly the set of values that appear on the wire.  Available **without**
    /// the `strum` feature — use it to drift-guard SQL `CHECK` lists and mappings.
    pub const VARIANTS: &'static [Self] = &[
        Self::Einzugsermaechtigung,
        Self::Zeitpunkt,
        Self::LieferanbindungEine,
        Self::LieferanbindungAlle,
        Self::Gewerbe,
        Self::Lastprofil,
        Self::ZaehlertypGroesse,
        Self::AusschlussGrossverbraucher,
        Self::Neukunde,
        Self::BestimmteVertragsformalitaeten,
        Self::Selbstablesung,
        Self::Onlinevoraussetzung,
        Self::Mindestumsatz,
        Self::Zusatzprodukt,
        Self::NeukundeMitVoraussetzungen,
        Self::Direktvertrieb,
        Self::Anschlussart,
        Self::Anschlusswert,
        Self::AlterKundenanlage,
        Self::Anlagebeschaffenheit,
        Self::Betriebsstundenbegrenzung,
        Self::Freigabezeiten,
        Self::Familienstruktur,
        Self::Mitgliedschaft,
        Self::StaatlicheFoerderung,
        Self::BesondereVerbrauchsstelle,
        Self::Niedrigenergie,
        Self::OrtsteileLiefergebiet,
        Self::WaermebedarfErdgas,
        Self::MaxZaehlerLieferstellen,
        Self::LieferungsbeschraenkungGasart,
        Self::KombiBoni,
        Self::Altvertrag,
        Self::VorgeschriebeneZusatzanlage,
        Self::MehrereZaehlerAbnahmestellen,
        Self::BestimmterAbnahmefall,
        Self::Zusatzmodalitaet,
        Self::NachweisZahlungsfaehigkeit,
        Self::UmstellungEnergieart,
    ];
    /// Number of schema-defined variants (equal to `VARIANTS.len()`), excluding the
    /// [`Voraussetzungen::Unknown`] catch-all.  Stable for this schema version.
    pub const COUNT: usize = Self::VARIANTS.len();
    /// Returns an iterator over all **known** variants of `Voraussetzungen`.
    ///
    /// Yields only variants that correspond to values defined in the BO4E schema
    /// (i.e. [`Self::VARIANTS`]), never the [`Voraussetzungen::Unknown`] catch-all.
    /// Available **without** the `strum` feature.
    ///
    /// # Example
    /// ```rust,ignore
    /// for v in Voraussetzungen::iter_known() {
    ///     println!("{}", v.as_wire());
    /// }
    /// ```
    pub fn iter_known() -> impl Iterator<Item = Self> + Clone {
        Self::VARIANTS.iter().copied()
    }
    /// Returns the canonical BO4E wire string (SCREAMING_SNAKE_CASE) for this value.
    ///
    /// [`Voraussetzungen::Unknown`] renders as `"UNKNOWN"`, matching its serialized form.
    pub const fn as_wire(&self) -> &'static str {
        match self {
            Self::Einzugsermaechtigung => "EINZUGSERMAECHTIGUNG",
            Self::Zeitpunkt => "ZEITPUNKT",
            Self::LieferanbindungEine => "LIEFERANBINDUNG_EINE",
            Self::LieferanbindungAlle => "LIEFERANBINDUNG_ALLE",
            Self::Gewerbe => "GEWERBE",
            Self::Lastprofil => "LASTPROFIL",
            Self::ZaehlertypGroesse => "ZAEHLERTYP_GROESSE",
            Self::AusschlussGrossverbraucher => "AUSSCHLUSS_GROSSVERBRAUCHER",
            Self::Neukunde => "NEUKUNDE",
            Self::BestimmteVertragsformalitaeten => "BESTIMMTE_VERTRAGSFORMALITAETEN",
            Self::Selbstablesung => "SELBSTABLESUNG",
            Self::Onlinevoraussetzung => "ONLINEVORAUSSETZUNG",
            Self::Mindestumsatz => "MINDESTUMSATZ",
            Self::Zusatzprodukt => "ZUSATZPRODUKT",
            Self::NeukundeMitVoraussetzungen => "NEUKUNDE_MIT_VORAUSSETZUNGEN",
            Self::Direktvertrieb => "DIREKTVERTRIEB",
            Self::Anschlussart => "ANSCHLUSSART",
            Self::Anschlusswert => "ANSCHLUSSWERT",
            Self::AlterKundenanlage => "ALTER_KUNDENANLAGE",
            Self::Anlagebeschaffenheit => "ANLAGEBESCHAFFENHEIT",
            Self::Betriebsstundenbegrenzung => "BETRIEBSSTUNDENBEGRENZUNG",
            Self::Freigabezeiten => "FREIGABEZEITEN",
            Self::Familienstruktur => "FAMILIENSTRUKTUR",
            Self::Mitgliedschaft => "MITGLIEDSCHAFT",
            Self::StaatlicheFoerderung => "STAATLICHE_FOERDERUNG",
            Self::BesondereVerbrauchsstelle => "BESONDERE_VERBRAUCHSSTELLE",
            Self::Niedrigenergie => "NIEDRIGENERGIE",
            Self::OrtsteileLiefergebiet => "ORTSTEILE_LIEFERGEBIET",
            Self::WaermebedarfErdgas => "WAERMEBEDARF_ERDGAS",
            Self::MaxZaehlerLieferstellen => "MAX_ZAEHLER_LIEFERSTELLEN",
            Self::LieferungsbeschraenkungGasart => "LIEFERUNGSBESCHRAENKUNG_GASART",
            Self::KombiBoni => "KOMBI_BONI",
            Self::Altvertrag => "ALTVERTRAG",
            Self::VorgeschriebeneZusatzanlage => "VORGESCHRIEBENE_ZUSATZANLAGE",
            Self::MehrereZaehlerAbnahmestellen => "MEHRERE_ZAEHLER_ABNAHMESTELLEN",
            Self::BestimmterAbnahmefall => "BESTIMMTER_ABNAHMEFALL",
            Self::Zusatzmodalitaet => "ZUSATZMODALITAET",
            Self::NachweisZahlungsfaehigkeit => "NACHWEIS_ZAHLUNGSFAEHIGKEIT",
            Self::UmstellungEnergieart => "UMSTELLUNG_ENERGIEART",
            Self::Unknown => "UNKNOWN",
        }
    }
    /// **Strictly** parses a BO4E wire string into a known variant.
    ///
    /// Unlike the lenient `serde` / [`FromStr`](std::str::FromStr) path — which maps
    /// any unrecognized value (a typo, a legacy code, or a value from a newer schema)
    /// to [`Voraussetzungen::Unknown`] — this returns
    /// [`Err`](crate::error::UnknownVariant) for values not defined in this schema
    /// version, including the literal `"UNKNOWN"`.  Use it at the ingest boundary to
    /// reject bad values instead of silently degrading them.
    ///
    /// # Example
    /// ```rust,ignore
    /// assert!(Voraussetzungen::from_wire("NOT_A_REAL_VALUE").is_err());
    /// ```
    pub fn from_wire(s: &str) -> Result<Self, crate::error::UnknownVariant> {
        match s {
            "EINZUGSERMAECHTIGUNG" => Ok(Self::Einzugsermaechtigung),
            "ZEITPUNKT" => Ok(Self::Zeitpunkt),
            "LIEFERANBINDUNG_EINE" => Ok(Self::LieferanbindungEine),
            "LIEFERANBINDUNG_ALLE" => Ok(Self::LieferanbindungAlle),
            "GEWERBE" => Ok(Self::Gewerbe),
            "LASTPROFIL" => Ok(Self::Lastprofil),
            "ZAEHLERTYP_GROESSE" => Ok(Self::ZaehlertypGroesse),
            "AUSSCHLUSS_GROSSVERBRAUCHER" => Ok(Self::AusschlussGrossverbraucher),
            "NEUKUNDE" => Ok(Self::Neukunde),
            "BESTIMMTE_VERTRAGSFORMALITAETEN" => Ok(Self::BestimmteVertragsformalitaeten),
            "SELBSTABLESUNG" => Ok(Self::Selbstablesung),
            "ONLINEVORAUSSETZUNG" => Ok(Self::Onlinevoraussetzung),
            "MINDESTUMSATZ" => Ok(Self::Mindestumsatz),
            "ZUSATZPRODUKT" => Ok(Self::Zusatzprodukt),
            "NEUKUNDE_MIT_VORAUSSETZUNGEN" => Ok(Self::NeukundeMitVoraussetzungen),
            "DIREKTVERTRIEB" => Ok(Self::Direktvertrieb),
            "ANSCHLUSSART" => Ok(Self::Anschlussart),
            "ANSCHLUSSWERT" => Ok(Self::Anschlusswert),
            "ALTER_KUNDENANLAGE" => Ok(Self::AlterKundenanlage),
            "ANLAGEBESCHAFFENHEIT" => Ok(Self::Anlagebeschaffenheit),
            "BETRIEBSSTUNDENBEGRENZUNG" => Ok(Self::Betriebsstundenbegrenzung),
            "FREIGABEZEITEN" => Ok(Self::Freigabezeiten),
            "FAMILIENSTRUKTUR" => Ok(Self::Familienstruktur),
            "MITGLIEDSCHAFT" => Ok(Self::Mitgliedschaft),
            "STAATLICHE_FOERDERUNG" => Ok(Self::StaatlicheFoerderung),
            "BESONDERE_VERBRAUCHSSTELLE" => Ok(Self::BesondereVerbrauchsstelle),
            "NIEDRIGENERGIE" => Ok(Self::Niedrigenergie),
            "ORTSTEILE_LIEFERGEBIET" => Ok(Self::OrtsteileLiefergebiet),
            "WAERMEBEDARF_ERDGAS" => Ok(Self::WaermebedarfErdgas),
            "MAX_ZAEHLER_LIEFERSTELLEN" => Ok(Self::MaxZaehlerLieferstellen),
            "LIEFERUNGSBESCHRAENKUNG_GASART" => Ok(Self::LieferungsbeschraenkungGasart),
            "KOMBI_BONI" => Ok(Self::KombiBoni),
            "ALTVERTRAG" => Ok(Self::Altvertrag),
            "VORGESCHRIEBENE_ZUSATZANLAGE" => Ok(Self::VorgeschriebeneZusatzanlage),
            "MEHRERE_ZAEHLER_ABNAHMESTELLEN" => Ok(Self::MehrereZaehlerAbnahmestellen),
            "BESTIMMTER_ABNAHMEFALL" => Ok(Self::BestimmterAbnahmefall),
            "ZUSATZMODALITAET" => Ok(Self::Zusatzmodalitaet),
            "NACHWEIS_ZAHLUNGSFAEHIGKEIT" => Ok(Self::NachweisZahlungsfaehigkeit),
            "UMSTELLUNG_ENERGIEART" => Ok(Self::UmstellungEnergieart),
            other => Err(crate::error::UnknownVariant::new(other)),
        }
    }
    /// Returns `true` if this value is the forward-compatibility
    /// [`Voraussetzungen::Unknown`] catch-all (an out-of-schema value).
    pub const fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
    /// Returns `true` if this value is a known, schema-defined variant.
    pub const fn is_known(&self) -> bool {
        !self.is_unknown()
    }
}
impl std::fmt::Display for Voraussetzungen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_wire())
    }
}
impl AsRef<str> for Voraussetzungen {
    fn as_ref(&self) -> &str {
        self.as_wire()
    }
}
#[cfg(feature = "versioned")]
impl crate::bo4e_enum_sealed::Sealed for Voraussetzungen {}
#[cfg(feature = "versioned")]
impl crate::Bo4eEnum for Voraussetzungen {
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
impl crate::Bo4eStrict for Voraussetzungen {
    fn collect_unknown_enums(&self, path: &str, out: &mut Vec<String>) {
        if self.is_unknown() {
            out.push(path.to_owned());
        }
    }
}
#[cfg(all(feature = "sqlx", feature = "json"))]
impl sqlx::Type<sqlx::Postgres> for Voraussetzungen {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
/// Encode via the canonical wire string (`as_wire`, always available) — no
/// `serde_json::Value` intermediate, saving an allocation per encode (M-07).
#[cfg(all(feature = "sqlx", feature = "json"))]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Voraussetzungen {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        let s: &str = self.as_wire();
        <&str as sqlx::Encode<'q, sqlx::Postgres>>::encode_by_ref(&s, buf)
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
#[cfg(test)]
impl proptest::arbitrary::Arbitrary for Voraussetzungen {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        proptest::sample::select(Self::VARIANTS.to_vec()).boxed()
    }
}
