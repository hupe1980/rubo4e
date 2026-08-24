#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(strum::EnumString, strum::EnumIter, strum::IntoStaticStr)
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// BDEW Artikelnummern
///
/// # Provenance
///
/// The variants are transcribed 1:1 from the `BdewArtikelnummer` enum of the
/// pinned BO4E schema release (see the module's schema-version tag, e.g.
/// `v202607.0.0`).  BO4E does not annotate this enum with the corresponding
/// *BDEW Codeliste der Artikelnummern und Artikel-IDs* release, so treat the
/// BO4E schema tag — not a BDEW Codeliste version — as the authoritative
/// coverage signal.  New codes arrive only via a schema bump; the per-release
/// CHANGELOG records enum additions.  Values absent from this version decode
/// to [`BdewArtikelnummer::Unknown`]; use `from_wire` to reject them strictly.
#[non_exhaustive]
pub enum BdewArtikelnummer {
    #[cfg_attr(feature = "serde", serde(rename = "LEISTUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "LEISTUNG"))]
    Leistung,
    #[cfg_attr(feature = "serde", serde(rename = "LEISTUNG_PAUSCHAL"))]
    #[cfg_attr(feature = "strum", strum(serialize = "LEISTUNG_PAUSCHAL"))]
    LeistungPauschal,
    #[cfg_attr(feature = "serde", serde(rename = "GRUNDPREIS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GRUNDPREIS"))]
    Grundpreis,
    #[cfg_attr(feature = "serde", serde(rename = "REGELENERGIE_ARBEIT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "REGELENERGIE_ARBEIT"))]
    RegelenergieArbeit,
    #[cfg_attr(feature = "serde", serde(rename = "REGELENERGIE_LEISTUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "REGELENERGIE_LEISTUNG"))]
    RegelenergieLeistung,
    #[cfg_attr(feature = "serde", serde(rename = "NOTSTROMLIEFERUNG_ARBEIT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "NOTSTROMLIEFERUNG_ARBEIT"))]
    NotstromlieferungArbeit,
    #[cfg_attr(feature = "serde", serde(rename = "NOTSTROMLIEFERUNG_LEISTUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "NOTSTROMLIEFERUNG_LEISTUNG"))]
    NotstromlieferungLeistung,
    #[cfg_attr(feature = "serde", serde(rename = "RESERVENETZKAPAZITAET"))]
    #[cfg_attr(feature = "strum", strum(serialize = "RESERVENETZKAPAZITAET"))]
    Reservenetzkapazitaet,
    #[cfg_attr(feature = "serde", serde(rename = "RESERVELEISTUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "RESERVELEISTUNG"))]
    Reserveleistung,
    #[cfg_attr(feature = "serde", serde(rename = "ZUSAETZLICHE_ABLESUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ZUSAETZLICHE_ABLESUNG"))]
    ZusaetzlicheAblesung,
    #[cfg_attr(feature = "serde", serde(rename = "PRUEFGEBUEHREN_AUSSERPLANMAESSIG"))]
    #[cfg_attr(
        feature = "strum",
        strum(serialize = "PRUEFGEBUEHREN_AUSSERPLANMAESSIG")
    )]
    PruefgebuehrenAusserplanmaessig,
    #[cfg_attr(feature = "serde", serde(rename = "WIRKARBEIT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "WIRKARBEIT"))]
    Wirkarbeit,
    #[cfg_attr(feature = "serde", serde(rename = "SINGULAER_GENUTZTE_BETRIEBSMITTEL"))]
    #[cfg_attr(
        feature = "strum",
        strum(serialize = "SINGULAER_GENUTZTE_BETRIEBSMITTEL")
    )]
    SingulaerGenutzteBetriebsmittel,
    #[cfg_attr(feature = "serde", serde(rename = "ABGABE_KWKG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ABGABE_KWKG"))]
    AbgabeKwkg,
    #[cfg_attr(feature = "serde", serde(rename = "ABSCHLAG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ABSCHLAG"))]
    Abschlag,
    #[cfg_attr(feature = "serde", serde(rename = "KONZESSIONSABGABE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KONZESSIONSABGABE"))]
    Konzessionsabgabe,
    #[cfg_attr(feature = "serde", serde(rename = "ENTGELT_FERNAUSLESUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ENTGELT_FERNAUSLESUNG"))]
    EntgeltFernauslesung,
    #[cfg_attr(feature = "serde", serde(rename = "UNTERMESSUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UNTERMESSUNG"))]
    Untermessung,
    #[cfg_attr(feature = "serde", serde(rename = "BLINDMEHRARBEIT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BLINDMEHRARBEIT"))]
    Blindmehrarbeit,
    #[cfg_attr(feature = "serde", serde(rename = "ENTGELT_ABRECHNUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ENTGELT_ABRECHNUNG"))]
    EntgeltAbrechnung,
    #[cfg_attr(feature = "serde", serde(rename = "SPERRKOSTEN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SPERRKOSTEN"))]
    Sperrkosten,
    #[cfg_attr(feature = "serde", serde(rename = "ENTSPERRKOSTEN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ENTSPERRKOSTEN"))]
    Entsperrkosten,
    #[cfg_attr(feature = "serde", serde(rename = "MAHNKOSTEN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MAHNKOSTEN"))]
    Mahnkosten,
    #[cfg_attr(feature = "serde", serde(rename = "MEHR_MINDERMENGEN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MEHR_MINDERMENGEN"))]
    MehrMindermengen,
    #[cfg_attr(feature = "serde", serde(rename = "INKASSOKOSTEN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "INKASSOKOSTEN"))]
    Inkassokosten,
    #[cfg_attr(feature = "serde", serde(rename = "BLINDMEHRLEISTUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BLINDMEHRLEISTUNG"))]
    Blindmehrleistung,
    #[cfg_attr(feature = "serde", serde(rename = "ENTGELT_MESSUNG_ABLESUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ENTGELT_MESSUNG_ABLESUNG"))]
    EntgeltMessungAblesung,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "ENTGELT_EINBAU_BETRIEB_WARTUNG_MESSTECHNIK")
    )]
    #[cfg_attr(
        feature = "strum",
        strum(serialize = "ENTGELT_EINBAU_BETRIEB_WARTUNG_MESSTECHNIK")
    )]
    EntgeltEinbauBetriebWartungMesstechnik,
    #[cfg_attr(feature = "serde", serde(rename = "AUSGLEICHSENERGIE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "AUSGLEICHSENERGIE"))]
    Ausgleichsenergie,
    #[cfg_attr(feature = "serde", serde(rename = "ZAEHLEINRICHTUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ZAEHLEINRICHTUNG"))]
    Zaehleinrichtung,
    #[cfg_attr(feature = "serde", serde(rename = "WANDLER_MENGENUMWERTER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "WANDLER_MENGENUMWERTER"))]
    WandlerMengenumwerter,
    #[cfg_attr(feature = "serde", serde(rename = "KOMMUNIKATIONSEINRICHTUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KOMMUNIKATIONSEINRICHTUNG"))]
    Kommunikationseinrichtung,
    #[cfg_attr(feature = "serde", serde(rename = "TECHNISCHE_STEUEREINRICHTUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TECHNISCHE_STEUEREINRICHTUNG"))]
    TechnischeSteuereinrichtung,
    #[cfg_attr(feature = "serde", serde(rename = "PARAGRAF_19_STROM_NEV_UMLAGE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "PARAGRAF_19_STROM_NEV_UMLAGE"))]
    Paragraf19StromNevUmlage,
    #[cfg_attr(feature = "serde", serde(rename = "BEFESTIGUNGSEINRICHTUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BEFESTIGUNGSEINRICHTUNG"))]
    Befestigungseinrichtung,
    #[cfg_attr(feature = "serde", serde(rename = "OFFSHORE_HAFTUNGSUMLAGE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "OFFSHORE_HAFTUNGSUMLAGE"))]
    OffshoreHaftungsumlage,
    #[cfg_attr(feature = "serde", serde(rename = "FIXE_ARBEITSENTGELTKOMPONENTE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "FIXE_ARBEITSENTGELTKOMPONENTE"))]
    FixeArbeitsentgeltkomponente,
    #[cfg_attr(feature = "serde", serde(rename = "FIXE_LEISTUNGSENTGELTKOMPONENTE"))]
    #[cfg_attr(
        feature = "strum",
        strum(serialize = "FIXE_LEISTUNGSENTGELTKOMPONENTE")
    )]
    FixeLeistungsentgeltkomponente,
    #[cfg_attr(feature = "serde", serde(rename = "UMLAGE_ABSCHALTBARE_LASTEN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UMLAGE_ABSCHALTBARE_LASTEN"))]
    UmlageAbschaltbareLasten,
    #[cfg_attr(feature = "serde", serde(rename = "MEHRMENGE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MEHRMENGE"))]
    Mehrmenge,
    #[cfg_attr(feature = "serde", serde(rename = "MINDERMENGE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MINDERMENGE"))]
    Mindermenge,
    #[cfg_attr(feature = "serde", serde(rename = "ENERGIESTEUER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ENERGIESTEUER"))]
    Energiesteuer,
    #[cfg_attr(feature = "serde", serde(rename = "SMARTMETER_GATEWAY"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SMARTMETER_GATEWAY"))]
    SmartmeterGateway,
    #[cfg_attr(feature = "serde", serde(rename = "STEUERBOX"))]
    #[cfg_attr(feature = "strum", strum(serialize = "STEUERBOX"))]
    Steuerbox,
    #[cfg_attr(feature = "serde", serde(rename = "MSB_INKL_MESSUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MSB_INKL_MESSUNG"))]
    MsbInklMessung,
    #[cfg_attr(feature = "serde", serde(rename = "AUSGLEICHSENERGIE_UNTERDECKUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "AUSGLEICHSENERGIE_UNTERDECKUNG"))]
    AusgleichsenergieUnterdeckung,
    /// Unknown or future variant — produced when deserializing a value
    /// that is not yet known to this version of the library.
    #[cfg_attr(feature = "serde", serde(other, rename = "UNKNOWN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UNKNOWN"))]
    Unknown,
}
impl BdewArtikelnummer {
    /// All variants defined by the BO4E schema, in declaration order.
    ///
    /// Excludes the forward-compatibility [`BdewArtikelnummer::Unknown`] catch-all, so this
    /// is exactly the set of values that appear on the wire.  Available **without**
    /// the `strum` feature — use it to drift-guard SQL `CHECK` lists and mappings.
    pub const VARIANTS: &'static [Self] = &[
        Self::Leistung,
        Self::LeistungPauschal,
        Self::Grundpreis,
        Self::RegelenergieArbeit,
        Self::RegelenergieLeistung,
        Self::NotstromlieferungArbeit,
        Self::NotstromlieferungLeistung,
        Self::Reservenetzkapazitaet,
        Self::Reserveleistung,
        Self::ZusaetzlicheAblesung,
        Self::PruefgebuehrenAusserplanmaessig,
        Self::Wirkarbeit,
        Self::SingulaerGenutzteBetriebsmittel,
        Self::AbgabeKwkg,
        Self::Abschlag,
        Self::Konzessionsabgabe,
        Self::EntgeltFernauslesung,
        Self::Untermessung,
        Self::Blindmehrarbeit,
        Self::EntgeltAbrechnung,
        Self::Sperrkosten,
        Self::Entsperrkosten,
        Self::Mahnkosten,
        Self::MehrMindermengen,
        Self::Inkassokosten,
        Self::Blindmehrleistung,
        Self::EntgeltMessungAblesung,
        Self::EntgeltEinbauBetriebWartungMesstechnik,
        Self::Ausgleichsenergie,
        Self::Zaehleinrichtung,
        Self::WandlerMengenumwerter,
        Self::Kommunikationseinrichtung,
        Self::TechnischeSteuereinrichtung,
        Self::Paragraf19StromNevUmlage,
        Self::Befestigungseinrichtung,
        Self::OffshoreHaftungsumlage,
        Self::FixeArbeitsentgeltkomponente,
        Self::FixeLeistungsentgeltkomponente,
        Self::UmlageAbschaltbareLasten,
        Self::Mehrmenge,
        Self::Mindermenge,
        Self::Energiesteuer,
        Self::SmartmeterGateway,
        Self::Steuerbox,
        Self::MsbInklMessung,
        Self::AusgleichsenergieUnterdeckung,
    ];
    /// Number of schema-defined variants (equal to `VARIANTS.len()`), excluding the
    /// [`BdewArtikelnummer::Unknown`] catch-all.  Stable for this schema version.
    pub const COUNT: usize = Self::VARIANTS.len();
    /// Returns an iterator over all **known** variants of `BdewArtikelnummer`.
    ///
    /// Yields only variants that correspond to values defined in the BO4E schema
    /// (i.e. [`Self::VARIANTS`]), never the [`BdewArtikelnummer::Unknown`] catch-all.
    /// Available **without** the `strum` feature.
    ///
    /// # Example
    /// ```
    /// # use rubo4e::current::BdewArtikelnummer;
    /// // Never yields the `Unknown` catch-all, so the count matches `COUNT`.
    /// assert_eq!(BdewArtikelnummer::iter_known().count(), BdewArtikelnummer::COUNT);
    /// assert!(BdewArtikelnummer::iter_known().all(|v| v.is_known()));
    /// ```
    pub fn iter_known() -> impl Iterator<Item = Self> + Clone {
        Self::VARIANTS.iter().copied()
    }
    /// Returns the canonical BO4E wire string (SCREAMING_SNAKE_CASE) for this value.
    ///
    /// [`BdewArtikelnummer::Unknown`] renders as `"UNKNOWN"`, matching its serialized form.
    pub const fn as_wire(&self) -> &'static str {
        match self {
            Self::Leistung => "LEISTUNG",
            Self::LeistungPauschal => "LEISTUNG_PAUSCHAL",
            Self::Grundpreis => "GRUNDPREIS",
            Self::RegelenergieArbeit => "REGELENERGIE_ARBEIT",
            Self::RegelenergieLeistung => "REGELENERGIE_LEISTUNG",
            Self::NotstromlieferungArbeit => "NOTSTROMLIEFERUNG_ARBEIT",
            Self::NotstromlieferungLeistung => "NOTSTROMLIEFERUNG_LEISTUNG",
            Self::Reservenetzkapazitaet => "RESERVENETZKAPAZITAET",
            Self::Reserveleistung => "RESERVELEISTUNG",
            Self::ZusaetzlicheAblesung => "ZUSAETZLICHE_ABLESUNG",
            Self::PruefgebuehrenAusserplanmaessig => "PRUEFGEBUEHREN_AUSSERPLANMAESSIG",
            Self::Wirkarbeit => "WIRKARBEIT",
            Self::SingulaerGenutzteBetriebsmittel => "SINGULAER_GENUTZTE_BETRIEBSMITTEL",
            Self::AbgabeKwkg => "ABGABE_KWKG",
            Self::Abschlag => "ABSCHLAG",
            Self::Konzessionsabgabe => "KONZESSIONSABGABE",
            Self::EntgeltFernauslesung => "ENTGELT_FERNAUSLESUNG",
            Self::Untermessung => "UNTERMESSUNG",
            Self::Blindmehrarbeit => "BLINDMEHRARBEIT",
            Self::EntgeltAbrechnung => "ENTGELT_ABRECHNUNG",
            Self::Sperrkosten => "SPERRKOSTEN",
            Self::Entsperrkosten => "ENTSPERRKOSTEN",
            Self::Mahnkosten => "MAHNKOSTEN",
            Self::MehrMindermengen => "MEHR_MINDERMENGEN",
            Self::Inkassokosten => "INKASSOKOSTEN",
            Self::Blindmehrleistung => "BLINDMEHRLEISTUNG",
            Self::EntgeltMessungAblesung => "ENTGELT_MESSUNG_ABLESUNG",
            Self::EntgeltEinbauBetriebWartungMesstechnik => {
                "ENTGELT_EINBAU_BETRIEB_WARTUNG_MESSTECHNIK"
            }
            Self::Ausgleichsenergie => "AUSGLEICHSENERGIE",
            Self::Zaehleinrichtung => "ZAEHLEINRICHTUNG",
            Self::WandlerMengenumwerter => "WANDLER_MENGENUMWERTER",
            Self::Kommunikationseinrichtung => "KOMMUNIKATIONSEINRICHTUNG",
            Self::TechnischeSteuereinrichtung => "TECHNISCHE_STEUEREINRICHTUNG",
            Self::Paragraf19StromNevUmlage => "PARAGRAF_19_STROM_NEV_UMLAGE",
            Self::Befestigungseinrichtung => "BEFESTIGUNGSEINRICHTUNG",
            Self::OffshoreHaftungsumlage => "OFFSHORE_HAFTUNGSUMLAGE",
            Self::FixeArbeitsentgeltkomponente => "FIXE_ARBEITSENTGELTKOMPONENTE",
            Self::FixeLeistungsentgeltkomponente => "FIXE_LEISTUNGSENTGELTKOMPONENTE",
            Self::UmlageAbschaltbareLasten => "UMLAGE_ABSCHALTBARE_LASTEN",
            Self::Mehrmenge => "MEHRMENGE",
            Self::Mindermenge => "MINDERMENGE",
            Self::Energiesteuer => "ENERGIESTEUER",
            Self::SmartmeterGateway => "SMARTMETER_GATEWAY",
            Self::Steuerbox => "STEUERBOX",
            Self::MsbInklMessung => "MSB_INKL_MESSUNG",
            Self::AusgleichsenergieUnterdeckung => "AUSGLEICHSENERGIE_UNTERDECKUNG",
            Self::Unknown => "UNKNOWN",
        }
    }
    /// **Strictly** parses a BO4E wire string into a known variant.
    ///
    /// Unlike the lenient `serde` / [`FromStr`](std::str::FromStr) path — which maps
    /// any unrecognized value (a typo, a legacy code, or a value from a newer schema)
    /// to [`BdewArtikelnummer::Unknown`] — this returns
    /// [`Err`](crate::error::UnknownVariant) for values not defined in this schema
    /// version, including the literal `"UNKNOWN"`.  Use it at the ingest boundary to
    /// reject bad values instead of silently degrading them.
    ///
    /// # Example
    /// ```
    /// # use rubo4e::current::BdewArtikelnummer;
    /// assert_eq!(BdewArtikelnummer::from_wire("LEISTUNG"), Ok(BdewArtikelnummer::Leistung));
    /// // Out-of-schema values are rejected rather than degraded:
    /// assert!(BdewArtikelnummer::from_wire("NOT_A_REAL_VALUE").is_err());
    /// // …including the `Unknown` catch-all's own wire spelling:
    /// assert!(BdewArtikelnummer::from_wire("UNKNOWN").is_err());
    /// ```
    pub fn from_wire(s: &str) -> Result<Self, crate::error::UnknownVariant> {
        match s {
            "LEISTUNG" => Ok(Self::Leistung),
            "LEISTUNG_PAUSCHAL" => Ok(Self::LeistungPauschal),
            "GRUNDPREIS" => Ok(Self::Grundpreis),
            "REGELENERGIE_ARBEIT" => Ok(Self::RegelenergieArbeit),
            "REGELENERGIE_LEISTUNG" => Ok(Self::RegelenergieLeistung),
            "NOTSTROMLIEFERUNG_ARBEIT" => Ok(Self::NotstromlieferungArbeit),
            "NOTSTROMLIEFERUNG_LEISTUNG" => Ok(Self::NotstromlieferungLeistung),
            "RESERVENETZKAPAZITAET" => Ok(Self::Reservenetzkapazitaet),
            "RESERVELEISTUNG" => Ok(Self::Reserveleistung),
            "ZUSAETZLICHE_ABLESUNG" => Ok(Self::ZusaetzlicheAblesung),
            "PRUEFGEBUEHREN_AUSSERPLANMAESSIG" => Ok(Self::PruefgebuehrenAusserplanmaessig),
            "WIRKARBEIT" => Ok(Self::Wirkarbeit),
            "SINGULAER_GENUTZTE_BETRIEBSMITTEL" => Ok(Self::SingulaerGenutzteBetriebsmittel),
            "ABGABE_KWKG" => Ok(Self::AbgabeKwkg),
            "ABSCHLAG" => Ok(Self::Abschlag),
            "KONZESSIONSABGABE" => Ok(Self::Konzessionsabgabe),
            "ENTGELT_FERNAUSLESUNG" => Ok(Self::EntgeltFernauslesung),
            "UNTERMESSUNG" => Ok(Self::Untermessung),
            "BLINDMEHRARBEIT" => Ok(Self::Blindmehrarbeit),
            "ENTGELT_ABRECHNUNG" => Ok(Self::EntgeltAbrechnung),
            "SPERRKOSTEN" => Ok(Self::Sperrkosten),
            "ENTSPERRKOSTEN" => Ok(Self::Entsperrkosten),
            "MAHNKOSTEN" => Ok(Self::Mahnkosten),
            "MEHR_MINDERMENGEN" => Ok(Self::MehrMindermengen),
            "INKASSOKOSTEN" => Ok(Self::Inkassokosten),
            "BLINDMEHRLEISTUNG" => Ok(Self::Blindmehrleistung),
            "ENTGELT_MESSUNG_ABLESUNG" => Ok(Self::EntgeltMessungAblesung),
            "ENTGELT_EINBAU_BETRIEB_WARTUNG_MESSTECHNIK" => {
                Ok(Self::EntgeltEinbauBetriebWartungMesstechnik)
            }
            "AUSGLEICHSENERGIE" => Ok(Self::Ausgleichsenergie),
            "ZAEHLEINRICHTUNG" => Ok(Self::Zaehleinrichtung),
            "WANDLER_MENGENUMWERTER" => Ok(Self::WandlerMengenumwerter),
            "KOMMUNIKATIONSEINRICHTUNG" => Ok(Self::Kommunikationseinrichtung),
            "TECHNISCHE_STEUEREINRICHTUNG" => Ok(Self::TechnischeSteuereinrichtung),
            "PARAGRAF_19_STROM_NEV_UMLAGE" => Ok(Self::Paragraf19StromNevUmlage),
            "BEFESTIGUNGSEINRICHTUNG" => Ok(Self::Befestigungseinrichtung),
            "OFFSHORE_HAFTUNGSUMLAGE" => Ok(Self::OffshoreHaftungsumlage),
            "FIXE_ARBEITSENTGELTKOMPONENTE" => Ok(Self::FixeArbeitsentgeltkomponente),
            "FIXE_LEISTUNGSENTGELTKOMPONENTE" => Ok(Self::FixeLeistungsentgeltkomponente),
            "UMLAGE_ABSCHALTBARE_LASTEN" => Ok(Self::UmlageAbschaltbareLasten),
            "MEHRMENGE" => Ok(Self::Mehrmenge),
            "MINDERMENGE" => Ok(Self::Mindermenge),
            "ENERGIESTEUER" => Ok(Self::Energiesteuer),
            "SMARTMETER_GATEWAY" => Ok(Self::SmartmeterGateway),
            "STEUERBOX" => Ok(Self::Steuerbox),
            "MSB_INKL_MESSUNG" => Ok(Self::MsbInklMessung),
            "AUSGLEICHSENERGIE_UNTERDECKUNG" => Ok(Self::AusgleichsenergieUnterdeckung),
            other => Err(crate::error::UnknownVariant::new(other)),
        }
    }
    /// Returns `true` if this value is the forward-compatibility
    /// [`BdewArtikelnummer::Unknown`] catch-all (an out-of-schema value).
    pub const fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
    /// Returns `true` if this value is a known, schema-defined variant.
    pub const fn is_known(&self) -> bool {
        !self.is_unknown()
    }
}
impl std::fmt::Display for BdewArtikelnummer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_wire())
    }
}
impl AsRef<str> for BdewArtikelnummer {
    fn as_ref(&self) -> &str {
        self.as_wire()
    }
}
#[cfg(feature = "versioned")]
impl crate::bo4e_enum_sealed::Sealed for BdewArtikelnummer {}
#[cfg(feature = "versioned")]
impl crate::Bo4eEnum for BdewArtikelnummer {
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
impl crate::Bo4eStrict for BdewArtikelnummer {
    fn collect_unknown_enums(&self, path: &str, out: &mut Vec<String>) {
        if self.is_unknown() {
            out.push(path.to_owned());
        }
    }
}
#[cfg(feature = "sqlx")]
impl sqlx::Type<sqlx::Postgres> for BdewArtikelnummer {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
/// Encodes as the canonical BO4E wire string, borrowed from `as_wire` — no
/// intermediate `String` or `serde_json::Value` is allocated.
#[cfg(feature = "sqlx")]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for BdewArtikelnummer {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        let s: &str = self.as_wire();
        <&str as sqlx::Encode<'q, sqlx::Postgres>>::encode_by_ref(&s, buf)
    }
}
/// Decodes leniently, matching the `serde` path: a value the schema does not
/// define becomes [`BdewArtikelnummer::Unknown`] rather than a decode error, so a
/// database row written by a newer schema version still reads back.
///
/// Use [`BdewArtikelnummer::from_wire`] on a `String` column, or check
/// [`BdewArtikelnummer::is_known`], where out-of-schema values must be rejected.
#[cfg(feature = "sqlx")]
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for BdewArtikelnummer {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <&str as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        Ok(Self::from_wire(s).unwrap_or(Self::Unknown))
    }
}
/// Lets `Vec<BdewArtikelnummer>` bind to a `TEXT[]` column.  Only this crate can
/// provide it: the trait and the enum are both foreign to any consumer, so the
/// orphan rule rules out a downstream impl.
#[cfg(feature = "sqlx")]
impl sqlx::postgres::PgHasArrayType for BdewArtikelnummer {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::postgres::PgHasArrayType>::array_type_info()
    }
}
#[cfg(test)]
impl proptest::arbitrary::Arbitrary for BdewArtikelnummer {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        proptest::sample::select(Self::VARIANTS.to_vec()).boxed()
    }
}
