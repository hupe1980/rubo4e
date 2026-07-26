#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(strum::EnumString, strum::EnumIter, strum::IntoStaticStr)
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Auflistung sämtlicher existierender Komponenten.
#[non_exhaustive]
pub enum ComTyp {
    #[cfg_attr(feature = "serde", serde(rename = "ADRESSE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ADRESSE"))]
    Adresse,
    #[cfg_attr(feature = "serde", serde(rename = "ANGEBOTSPOSITION"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ANGEBOTSPOSITION"))]
    Angebotsposition,
    #[cfg_attr(feature = "serde", serde(rename = "ANGEBOTSTEIL"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ANGEBOTSTEIL"))]
    Angebotsteil,
    #[cfg_attr(feature = "serde", serde(rename = "ANGEBOTSVARIANTE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ANGEBOTSVARIANTE"))]
    Angebotsvariante,
    #[cfg_attr(feature = "serde", serde(rename = "AUFABSCHLAG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "AUFABSCHLAG"))]
    Aufabschlag,
    #[cfg_attr(feature = "serde", serde(rename = "AUSSCHREIBUNGSDETAIL"))]
    #[cfg_attr(feature = "strum", strum(serialize = "AUSSCHREIBUNGSDETAIL"))]
    Ausschreibungsdetail,
    #[cfg_attr(feature = "serde", serde(rename = "AUSSCHREIBUNGSLOS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "AUSSCHREIBUNGSLOS"))]
    Ausschreibungslos,
    #[cfg_attr(feature = "serde", serde(rename = "BETRAG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BETRAG"))]
    Betrag,
    #[cfg_attr(feature = "serde", serde(rename = "DIENSTLEISTUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "DIENSTLEISTUNG"))]
    Dienstleistung,
    #[cfg_attr(feature = "serde", serde(rename = "EINHEITSPREISPOSITION"))]
    #[cfg_attr(feature = "strum", strum(serialize = "EINHEITSPREISPOSITION"))]
    Einheitspreisposition,
    #[cfg_attr(feature = "serde", serde(rename = "ENERGIEHERKUNFT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ENERGIEHERKUNFT"))]
    Energieherkunft,
    #[cfg_attr(feature = "serde", serde(rename = "ENERGIEMIX"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ENERGIEMIX"))]
    Energiemix,
    #[cfg_attr(feature = "serde", serde(rename = "FREMDKOSTENBLOCK"))]
    #[cfg_attr(feature = "strum", strum(serialize = "FREMDKOSTENBLOCK"))]
    Fremdkostenblock,
    #[cfg_attr(feature = "serde", serde(rename = "FREMDKOSTENPOSITION"))]
    #[cfg_attr(feature = "strum", strum(serialize = "FREMDKOSTENPOSITION"))]
    Fremdkostenposition,
    #[cfg_attr(feature = "serde", serde(rename = "GEOKOORDINATEN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GEOKOORDINATEN"))]
    Geokoordinaten,
    #[cfg_attr(feature = "serde", serde(rename = "KATASTERADRESSE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KATASTERADRESSE"))]
    Katasteradresse,
    #[cfg_attr(feature = "serde", serde(rename = "KONFIGURATIONSPRODUKT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KONFIGURATIONSPRODUKT"))]
    Konfigurationsprodukt,
    #[cfg_attr(feature = "serde", serde(rename = "KONTAKTWEG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KONTAKTWEG"))]
    Kontaktweg,
    #[cfg_attr(feature = "serde", serde(rename = "KONZESSIONSABGABE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KONZESSIONSABGABE"))]
    Konzessionsabgabe,
    #[cfg_attr(feature = "serde", serde(rename = "KOSTENBLOCK"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KOSTENBLOCK"))]
    Kostenblock,
    #[cfg_attr(feature = "serde", serde(rename = "KOSTENPOSITION"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KOSTENPOSITION"))]
    Kostenposition,
    #[cfg_attr(feature = "serde", serde(rename = "LASTPROFIL"))]
    #[cfg_attr(feature = "strum", strum(serialize = "LASTPROFIL"))]
    Lastprofil,
    #[cfg_attr(feature = "serde", serde(rename = "LASTVARIABLEPREISPOSITION"))]
    #[cfg_attr(feature = "strum", strum(serialize = "LASTVARIABLEPREISPOSITION"))]
    Lastvariablepreisposition,
    #[cfg_attr(feature = "serde", serde(rename = "MARKTGEBIETINFO"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MARKTGEBIETINFO"))]
    Marktgebietinfo,
    #[cfg_attr(feature = "serde", serde(rename = "MENGE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MENGE"))]
    Menge,
    #[cfg_attr(feature = "serde", serde(rename = "MESSWERT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MESSWERT"))]
    Messwert,
    #[cfg_attr(feature = "serde", serde(rename = "PREIS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "PREIS"))]
    Preis,
    #[cfg_attr(feature = "serde", serde(rename = "PREISGARANTIE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "PREISGARANTIE"))]
    Preisgarantie,
    #[cfg_attr(feature = "serde", serde(rename = "PREISPOSITION"))]
    #[cfg_attr(feature = "strum", strum(serialize = "PREISPOSITION"))]
    Preisposition,
    #[cfg_attr(feature = "serde", serde(rename = "PREISSTAFFEL"))]
    #[cfg_attr(feature = "strum", strum(serialize = "PREISSTAFFEL"))]
    Preisstaffel,
    #[cfg_attr(feature = "serde", serde(rename = "RECHNUNGSPOSITION"))]
    #[cfg_attr(feature = "strum", strum(serialize = "RECHNUNGSPOSITION"))]
    Rechnungsposition,
    #[cfg_attr(feature = "serde", serde(rename = "REGIONSOPERATION"))]
    #[cfg_attr(feature = "strum", strum(serialize = "REGIONSOPERATION"))]
    Regionsoperation,
    #[cfg_attr(feature = "serde", serde(rename = "REGIONSPREIS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "REGIONSPREIS"))]
    Regionspreis,
    #[cfg_attr(feature = "serde", serde(rename = "REGIONSZEITSCHEIBE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "REGIONSZEITSCHEIBE"))]
    Regionszeitscheibe,
    #[cfg_attr(feature = "serde", serde(rename = "RELATIVEPREISPOSITION"))]
    #[cfg_attr(feature = "strum", strum(serialize = "RELATIVEPREISPOSITION"))]
    Relativepreisposition,
    #[cfg_attr(feature = "serde", serde(rename = "SIGMOIDPARAMETER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SIGMOIDPARAMETER"))]
    Sigmoidparameter,
    #[cfg_attr(feature = "serde", serde(rename = "STANDORTEIGENSCHAFTENGAS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "STANDORTEIGENSCHAFTENGAS"))]
    Standorteigenschaftengas,
    #[cfg_attr(feature = "serde", serde(rename = "STANDORTEIGENSCHAFTENSTROM"))]
    #[cfg_attr(feature = "strum", strum(serialize = "STANDORTEIGENSCHAFTENSTROM"))]
    Standorteigenschaftenstrom,
    #[cfg_attr(feature = "serde", serde(rename = "STEUERBETRAG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "STEUERBETRAG"))]
    Steuerbetrag,
    #[cfg_attr(feature = "serde", serde(rename = "TAGESPARAMETER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TAGESPARAMETER"))]
    Tagesparameter,
    #[cfg_attr(feature = "serde", serde(rename = "TARIFBERECHNUNGSPARAMETER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TARIFBERECHNUNGSPARAMETER"))]
    Tarifberechnungsparameter,
    #[cfg_attr(feature = "serde", serde(rename = "TARIFEINSCHRAENKUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TARIFEINSCHRAENKUNG"))]
    Tarifeinschraenkung,
    #[cfg_attr(feature = "serde", serde(rename = "TARIFPREIS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TARIFPREIS"))]
    Tarifpreis,
    #[cfg_attr(feature = "serde", serde(rename = "TARIFPREISPOSITION"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TARIFPREISPOSITION"))]
    Tarifpreisposition,
    #[cfg_attr(feature = "serde", serde(rename = "TARIFPREISZEITSCHEIBE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TARIFPREISZEITSCHEIBE"))]
    Tarifpreiszeitscheibe,
    #[cfg_attr(feature = "serde", serde(rename = "UMSCHALTZEIT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UMSCHALTZEIT"))]
    Umschaltzeit,
    #[cfg_attr(feature = "serde", serde(rename = "UNTERSCHRIFT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UNTERSCHRIFT"))]
    Unterschrift,
    #[cfg_attr(feature = "serde", serde(rename = "VERTRAGSKONDITIONEN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "VERTRAGSKONDITIONEN"))]
    Vertragskonditionen,
    #[cfg_attr(feature = "serde", serde(rename = "VERTRAGSTEIL"))]
    #[cfg_attr(feature = "strum", strum(serialize = "VERTRAGSTEIL"))]
    Vertragsteil,
    #[cfg_attr(feature = "serde", serde(rename = "VERWENDUNGSZWECKPROMARKTROLLE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "VERWENDUNGSZWECKPROMARKTROLLE"))]
    Verwendungszweckpromarktrolle,
    #[cfg_attr(feature = "serde", serde(rename = "VORAUSZAHLUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "VORAUSZAHLUNG"))]
    Vorauszahlung,
    #[cfg_attr(feature = "serde", serde(rename = "ZAEHLWERK"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ZAEHLWERK"))]
    Zaehlwerk,
    #[cfg_attr(feature = "serde", serde(rename = "ZAEHLZEITREGISTER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ZAEHLZEITREGISTER"))]
    Zaehlzeitregister,
    #[cfg_attr(feature = "serde", serde(rename = "ZAEHLZEITSAISON"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ZAEHLZEITSAISON"))]
    Zaehlzeitsaison,
    #[cfg_attr(feature = "serde", serde(rename = "ZAEHLZEITTAGTYP"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ZAEHLZEITTAGTYP"))]
    Zaehlzeittagtyp,
    #[cfg_attr(feature = "serde", serde(rename = "ZAHLUNGSINFORMATION"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ZAHLUNGSINFORMATION"))]
    Zahlungsinformation,
    #[cfg_attr(feature = "serde", serde(rename = "ZEITRAUM"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ZEITRAUM"))]
    Zeitraum,
    #[cfg_attr(feature = "serde", serde(rename = "ZEITREIHENWERT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ZEITREIHENWERT"))]
    Zeitreihenwert,
    #[cfg_attr(feature = "serde", serde(rename = "ZEITVARIABLEPREISPOSITION"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ZEITVARIABLEPREISPOSITION"))]
    Zeitvariablepreisposition,
    #[cfg_attr(feature = "serde", serde(rename = "ZUSTAENDIGKEIT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ZUSTAENDIGKEIT"))]
    Zustaendigkeit,
    /// Unknown or future variant — produced when deserializing a value
    /// that is not yet known to this version of the library.
    #[cfg_attr(feature = "serde", serde(other, rename = "UNKNOWN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UNKNOWN"))]
    Unknown,
}
impl ComTyp {
    /// All variants defined by the BO4E schema, in declaration order.
    ///
    /// Excludes the forward-compatibility [`ComTyp::Unknown`] catch-all, so this
    /// is exactly the set of values that appear on the wire.  Available **without**
    /// the `strum` feature — use it to drift-guard SQL `CHECK` lists and mappings.
    pub const VARIANTS: &'static [Self] = &[
        Self::Adresse,
        Self::Angebotsposition,
        Self::Angebotsteil,
        Self::Angebotsvariante,
        Self::Aufabschlag,
        Self::Ausschreibungsdetail,
        Self::Ausschreibungslos,
        Self::Betrag,
        Self::Dienstleistung,
        Self::Einheitspreisposition,
        Self::Energieherkunft,
        Self::Energiemix,
        Self::Fremdkostenblock,
        Self::Fremdkostenposition,
        Self::Geokoordinaten,
        Self::Katasteradresse,
        Self::Konfigurationsprodukt,
        Self::Kontaktweg,
        Self::Konzessionsabgabe,
        Self::Kostenblock,
        Self::Kostenposition,
        Self::Lastprofil,
        Self::Lastvariablepreisposition,
        Self::Marktgebietinfo,
        Self::Menge,
        Self::Messwert,
        Self::Preis,
        Self::Preisgarantie,
        Self::Preisposition,
        Self::Preisstaffel,
        Self::Rechnungsposition,
        Self::Regionsoperation,
        Self::Regionspreis,
        Self::Regionszeitscheibe,
        Self::Relativepreisposition,
        Self::Sigmoidparameter,
        Self::Standorteigenschaftengas,
        Self::Standorteigenschaftenstrom,
        Self::Steuerbetrag,
        Self::Tagesparameter,
        Self::Tarifberechnungsparameter,
        Self::Tarifeinschraenkung,
        Self::Tarifpreis,
        Self::Tarifpreisposition,
        Self::Tarifpreiszeitscheibe,
        Self::Umschaltzeit,
        Self::Unterschrift,
        Self::Vertragskonditionen,
        Self::Vertragsteil,
        Self::Verwendungszweckpromarktrolle,
        Self::Vorauszahlung,
        Self::Zaehlwerk,
        Self::Zaehlzeitregister,
        Self::Zaehlzeitsaison,
        Self::Zaehlzeittagtyp,
        Self::Zahlungsinformation,
        Self::Zeitraum,
        Self::Zeitreihenwert,
        Self::Zeitvariablepreisposition,
        Self::Zustaendigkeit,
    ];
    /// Number of schema-defined variants (equal to `VARIANTS.len()`), excluding the
    /// [`ComTyp::Unknown`] catch-all.  Stable for this schema version.
    pub const COUNT: usize = Self::VARIANTS.len();
    /// Returns an iterator over all **known** variants of `ComTyp`.
    ///
    /// Yields only variants that correspond to values defined in the BO4E schema
    /// (i.e. [`Self::VARIANTS`]), never the [`ComTyp::Unknown`] catch-all.
    /// Available **without** the `strum` feature.
    ///
    /// # Example
    /// ```rust,ignore
    /// for v in ComTyp::iter_known() {
    ///     println!("{}", v.as_wire());
    /// }
    /// ```
    pub fn iter_known() -> impl Iterator<Item = Self> + Clone {
        Self::VARIANTS.iter().copied()
    }
    /// Returns the canonical BO4E wire string (SCREAMING_SNAKE_CASE) for this value.
    ///
    /// [`ComTyp::Unknown`] renders as `"UNKNOWN"`, matching its serialized form.
    pub const fn as_wire(&self) -> &'static str {
        match self {
            Self::Adresse => "ADRESSE",
            Self::Angebotsposition => "ANGEBOTSPOSITION",
            Self::Angebotsteil => "ANGEBOTSTEIL",
            Self::Angebotsvariante => "ANGEBOTSVARIANTE",
            Self::Aufabschlag => "AUFABSCHLAG",
            Self::Ausschreibungsdetail => "AUSSCHREIBUNGSDETAIL",
            Self::Ausschreibungslos => "AUSSCHREIBUNGSLOS",
            Self::Betrag => "BETRAG",
            Self::Dienstleistung => "DIENSTLEISTUNG",
            Self::Einheitspreisposition => "EINHEITSPREISPOSITION",
            Self::Energieherkunft => "ENERGIEHERKUNFT",
            Self::Energiemix => "ENERGIEMIX",
            Self::Fremdkostenblock => "FREMDKOSTENBLOCK",
            Self::Fremdkostenposition => "FREMDKOSTENPOSITION",
            Self::Geokoordinaten => "GEOKOORDINATEN",
            Self::Katasteradresse => "KATASTERADRESSE",
            Self::Konfigurationsprodukt => "KONFIGURATIONSPRODUKT",
            Self::Kontaktweg => "KONTAKTWEG",
            Self::Konzessionsabgabe => "KONZESSIONSABGABE",
            Self::Kostenblock => "KOSTENBLOCK",
            Self::Kostenposition => "KOSTENPOSITION",
            Self::Lastprofil => "LASTPROFIL",
            Self::Lastvariablepreisposition => "LASTVARIABLEPREISPOSITION",
            Self::Marktgebietinfo => "MARKTGEBIETINFO",
            Self::Menge => "MENGE",
            Self::Messwert => "MESSWERT",
            Self::Preis => "PREIS",
            Self::Preisgarantie => "PREISGARANTIE",
            Self::Preisposition => "PREISPOSITION",
            Self::Preisstaffel => "PREISSTAFFEL",
            Self::Rechnungsposition => "RECHNUNGSPOSITION",
            Self::Regionsoperation => "REGIONSOPERATION",
            Self::Regionspreis => "REGIONSPREIS",
            Self::Regionszeitscheibe => "REGIONSZEITSCHEIBE",
            Self::Relativepreisposition => "RELATIVEPREISPOSITION",
            Self::Sigmoidparameter => "SIGMOIDPARAMETER",
            Self::Standorteigenschaftengas => "STANDORTEIGENSCHAFTENGAS",
            Self::Standorteigenschaftenstrom => "STANDORTEIGENSCHAFTENSTROM",
            Self::Steuerbetrag => "STEUERBETRAG",
            Self::Tagesparameter => "TAGESPARAMETER",
            Self::Tarifberechnungsparameter => "TARIFBERECHNUNGSPARAMETER",
            Self::Tarifeinschraenkung => "TARIFEINSCHRAENKUNG",
            Self::Tarifpreis => "TARIFPREIS",
            Self::Tarifpreisposition => "TARIFPREISPOSITION",
            Self::Tarifpreiszeitscheibe => "TARIFPREISZEITSCHEIBE",
            Self::Umschaltzeit => "UMSCHALTZEIT",
            Self::Unterschrift => "UNTERSCHRIFT",
            Self::Vertragskonditionen => "VERTRAGSKONDITIONEN",
            Self::Vertragsteil => "VERTRAGSTEIL",
            Self::Verwendungszweckpromarktrolle => "VERWENDUNGSZWECKPROMARKTROLLE",
            Self::Vorauszahlung => "VORAUSZAHLUNG",
            Self::Zaehlwerk => "ZAEHLWERK",
            Self::Zaehlzeitregister => "ZAEHLZEITREGISTER",
            Self::Zaehlzeitsaison => "ZAEHLZEITSAISON",
            Self::Zaehlzeittagtyp => "ZAEHLZEITTAGTYP",
            Self::Zahlungsinformation => "ZAHLUNGSINFORMATION",
            Self::Zeitraum => "ZEITRAUM",
            Self::Zeitreihenwert => "ZEITREIHENWERT",
            Self::Zeitvariablepreisposition => "ZEITVARIABLEPREISPOSITION",
            Self::Zustaendigkeit => "ZUSTAENDIGKEIT",
            Self::Unknown => "UNKNOWN",
        }
    }
    /// **Strictly** parses a BO4E wire string into a known variant.
    ///
    /// Unlike the lenient `serde` / [`FromStr`](std::str::FromStr) path — which maps
    /// any unrecognized value (a typo, a legacy code, or a value from a newer schema)
    /// to [`ComTyp::Unknown`] — this returns
    /// [`Err`](crate::error::UnknownVariant) for values not defined in this schema
    /// version, including the literal `"UNKNOWN"`.  Use it at the ingest boundary to
    /// reject bad values instead of silently degrading them.
    ///
    /// # Example
    /// ```rust,ignore
    /// assert!(ComTyp::from_wire("NOT_A_REAL_VALUE").is_err());
    /// ```
    pub fn from_wire(s: &str) -> Result<Self, crate::error::UnknownVariant> {
        match s {
            "ADRESSE" => Ok(Self::Adresse),
            "ANGEBOTSPOSITION" => Ok(Self::Angebotsposition),
            "ANGEBOTSTEIL" => Ok(Self::Angebotsteil),
            "ANGEBOTSVARIANTE" => Ok(Self::Angebotsvariante),
            "AUFABSCHLAG" => Ok(Self::Aufabschlag),
            "AUSSCHREIBUNGSDETAIL" => Ok(Self::Ausschreibungsdetail),
            "AUSSCHREIBUNGSLOS" => Ok(Self::Ausschreibungslos),
            "BETRAG" => Ok(Self::Betrag),
            "DIENSTLEISTUNG" => Ok(Self::Dienstleistung),
            "EINHEITSPREISPOSITION" => Ok(Self::Einheitspreisposition),
            "ENERGIEHERKUNFT" => Ok(Self::Energieherkunft),
            "ENERGIEMIX" => Ok(Self::Energiemix),
            "FREMDKOSTENBLOCK" => Ok(Self::Fremdkostenblock),
            "FREMDKOSTENPOSITION" => Ok(Self::Fremdkostenposition),
            "GEOKOORDINATEN" => Ok(Self::Geokoordinaten),
            "KATASTERADRESSE" => Ok(Self::Katasteradresse),
            "KONFIGURATIONSPRODUKT" => Ok(Self::Konfigurationsprodukt),
            "KONTAKTWEG" => Ok(Self::Kontaktweg),
            "KONZESSIONSABGABE" => Ok(Self::Konzessionsabgabe),
            "KOSTENBLOCK" => Ok(Self::Kostenblock),
            "KOSTENPOSITION" => Ok(Self::Kostenposition),
            "LASTPROFIL" => Ok(Self::Lastprofil),
            "LASTVARIABLEPREISPOSITION" => Ok(Self::Lastvariablepreisposition),
            "MARKTGEBIETINFO" => Ok(Self::Marktgebietinfo),
            "MENGE" => Ok(Self::Menge),
            "MESSWERT" => Ok(Self::Messwert),
            "PREIS" => Ok(Self::Preis),
            "PREISGARANTIE" => Ok(Self::Preisgarantie),
            "PREISPOSITION" => Ok(Self::Preisposition),
            "PREISSTAFFEL" => Ok(Self::Preisstaffel),
            "RECHNUNGSPOSITION" => Ok(Self::Rechnungsposition),
            "REGIONSOPERATION" => Ok(Self::Regionsoperation),
            "REGIONSPREIS" => Ok(Self::Regionspreis),
            "REGIONSZEITSCHEIBE" => Ok(Self::Regionszeitscheibe),
            "RELATIVEPREISPOSITION" => Ok(Self::Relativepreisposition),
            "SIGMOIDPARAMETER" => Ok(Self::Sigmoidparameter),
            "STANDORTEIGENSCHAFTENGAS" => Ok(Self::Standorteigenschaftengas),
            "STANDORTEIGENSCHAFTENSTROM" => Ok(Self::Standorteigenschaftenstrom),
            "STEUERBETRAG" => Ok(Self::Steuerbetrag),
            "TAGESPARAMETER" => Ok(Self::Tagesparameter),
            "TARIFBERECHNUNGSPARAMETER" => Ok(Self::Tarifberechnungsparameter),
            "TARIFEINSCHRAENKUNG" => Ok(Self::Tarifeinschraenkung),
            "TARIFPREIS" => Ok(Self::Tarifpreis),
            "TARIFPREISPOSITION" => Ok(Self::Tarifpreisposition),
            "TARIFPREISZEITSCHEIBE" => Ok(Self::Tarifpreiszeitscheibe),
            "UMSCHALTZEIT" => Ok(Self::Umschaltzeit),
            "UNTERSCHRIFT" => Ok(Self::Unterschrift),
            "VERTRAGSKONDITIONEN" => Ok(Self::Vertragskonditionen),
            "VERTRAGSTEIL" => Ok(Self::Vertragsteil),
            "VERWENDUNGSZWECKPROMARKTROLLE" => Ok(Self::Verwendungszweckpromarktrolle),
            "VORAUSZAHLUNG" => Ok(Self::Vorauszahlung),
            "ZAEHLWERK" => Ok(Self::Zaehlwerk),
            "ZAEHLZEITREGISTER" => Ok(Self::Zaehlzeitregister),
            "ZAEHLZEITSAISON" => Ok(Self::Zaehlzeitsaison),
            "ZAEHLZEITTAGTYP" => Ok(Self::Zaehlzeittagtyp),
            "ZAHLUNGSINFORMATION" => Ok(Self::Zahlungsinformation),
            "ZEITRAUM" => Ok(Self::Zeitraum),
            "ZEITREIHENWERT" => Ok(Self::Zeitreihenwert),
            "ZEITVARIABLEPREISPOSITION" => Ok(Self::Zeitvariablepreisposition),
            "ZUSTAENDIGKEIT" => Ok(Self::Zustaendigkeit),
            other => Err(crate::error::UnknownVariant::new(other)),
        }
    }
    /// Returns `true` if this value is the forward-compatibility
    /// [`ComTyp::Unknown`] catch-all (an out-of-schema value).
    pub const fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
    /// Returns `true` if this value is a known, schema-defined variant.
    pub const fn is_known(&self) -> bool {
        !self.is_unknown()
    }
}
impl std::fmt::Display for ComTyp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_wire())
    }
}
impl AsRef<str> for ComTyp {
    fn as_ref(&self) -> &str {
        self.as_wire()
    }
}
#[cfg(feature = "versioned")]
impl crate::bo4e_enum_sealed::Sealed for ComTyp {}
#[cfg(feature = "versioned")]
impl crate::Bo4eEnum for ComTyp {
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
impl crate::Bo4eStrict for ComTyp {
    fn collect_unknown_enums(&self, path: &str, out: &mut Vec<String>) {
        if self.is_unknown() {
            out.push(path.to_owned());
        }
    }
}
#[cfg(all(feature = "sqlx", feature = "json"))]
impl sqlx::Type<sqlx::Postgres> for ComTyp {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
/// Encode via the canonical wire string (`as_wire`, always available) — no
/// `serde_json::Value` intermediate, saving an allocation per encode (M-07).
#[cfg(all(feature = "sqlx", feature = "json"))]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for ComTyp {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        let s: &str = self.as_wire();
        <&str as sqlx::Encode<'q, sqlx::Postgres>>::encode_by_ref(&s, buf)
    }
}
#[cfg(all(feature = "sqlx", feature = "json"))]
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for ComTyp {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <String as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        serde_json::from_value(serde_json::Value::String(s))
            .map_err(|e| Box::new(e) as sqlx::error::BoxDynError)
    }
}
#[cfg(test)]
impl proptest::arbitrary::Arbitrary for ComTyp {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        proptest::sample::select(Self::VARIANTS.to_vec()).boxed()
    }
}
