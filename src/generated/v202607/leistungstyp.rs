#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(strum::EnumString, strum::EnumIter, strum::IntoStaticStr)
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[non_exhaustive]
pub enum Leistungstyp {
    #[cfg_attr(feature = "serde", serde(rename = "ARBEITSPREIS_WIRKARBEIT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ARBEITSPREIS_WIRKARBEIT"))]
    ArbeitspreisWirkarbeit,
    #[cfg_attr(feature = "serde", serde(rename = "LEISTUNGSPREIS_WIRKLEISTUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "LEISTUNGSPREIS_WIRKLEISTUNG"))]
    LeistungspreisWirkleistung,
    #[cfg_attr(feature = "serde", serde(rename = "ARBEITSPREIS_BLINDARBEIT_IND"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ARBEITSPREIS_BLINDARBEIT_IND"))]
    ArbeitspreisBlindarbeitInd,
    #[cfg_attr(feature = "serde", serde(rename = "ARBEITSPREIS_BLINDARBEIT_KAP"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ARBEITSPREIS_BLINDARBEIT_KAP"))]
    ArbeitspreisBlindarbeitKap,
    #[cfg_attr(feature = "serde", serde(rename = "GRUNDPREIS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GRUNDPREIS"))]
    Grundpreis,
    #[cfg_attr(feature = "serde", serde(rename = "GRUNDPREIS_ARBEIT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GRUNDPREIS_ARBEIT"))]
    GrundpreisArbeit,
    #[cfg_attr(feature = "serde", serde(rename = "GRUNDPREIS_LEISTUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GRUNDPREIS_LEISTUNG"))]
    GrundpreisLeistung,
    #[cfg_attr(feature = "serde", serde(rename = "MEHRMINDERMENGE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MEHRMINDERMENGE"))]
    Mehrmindermenge,
    #[cfg_attr(feature = "serde", serde(rename = "MESSSTELLENBETRIEB"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MESSSTELLENBETRIEB"))]
    Messstellenbetrieb,
    #[cfg_attr(feature = "serde", serde(rename = "MESSDIENSTLEISTUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MESSDIENSTLEISTUNG"))]
    Messdienstleistung,
    #[cfg_attr(feature = "serde", serde(rename = "MESSDIENSTLEISTUNG_INKL_MESSUNG"))]
    #[cfg_attr(
        feature = "strum",
        strum(serialize = "MESSDIENSTLEISTUNG_INKL_MESSUNG")
    )]
    MessdienstleistungInklMessung,
    #[cfg_attr(feature = "serde", serde(rename = "ABRECHNUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ABRECHNUNG"))]
    Abrechnung,
    #[cfg_attr(feature = "serde", serde(rename = "KONZESSIONS_ABGABE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KONZESSIONS_ABGABE"))]
    KonzessionsAbgabe,
    #[cfg_attr(feature = "serde", serde(rename = "KWK_UMLAGE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KWK_UMLAGE"))]
    KwkUmlage,
    #[cfg_attr(feature = "serde", serde(rename = "OFFSHORE_UMLAGE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "OFFSHORE_UMLAGE"))]
    OffshoreUmlage,
    #[cfg_attr(feature = "serde", serde(rename = "ABLAV_UMLAGE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ABLAV_UMLAGE"))]
    AblavUmlage,
    #[cfg_attr(feature = "serde", serde(rename = "SONDERKUNDEN_UMLAGE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SONDERKUNDEN_UMLAGE"))]
    SonderkundenUmlage,
    #[cfg_attr(feature = "serde", serde(rename = "REGELENERGIE_UMLAGE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "REGELENERGIE_UMLAGE"))]
    RegelenergieUmlage,
    #[cfg_attr(feature = "serde", serde(rename = "BILANZIERUNG_UMLAGE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BILANZIERUNG_UMLAGE"))]
    BilanzierungUmlage,
    #[cfg_attr(feature = "serde", serde(rename = "AUSLESUNG_ZUSAETZLICH"))]
    #[cfg_attr(feature = "strum", strum(serialize = "AUSLESUNG_ZUSAETZLICH"))]
    AuslesungZusaetzlich,
    #[cfg_attr(feature = "serde", serde(rename = "ABLESUNG_ZUSAETZLICH"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ABLESUNG_ZUSAETZLICH"))]
    AblesungZusaetzlich,
    #[cfg_attr(feature = "serde", serde(rename = "ABRECHNUNG_ZUSAETZLICH"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ABRECHNUNG_ZUSAETZLICH"))]
    AbrechnungZusaetzlich,
    #[cfg_attr(feature = "serde", serde(rename = "SPERRUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SPERRUNG"))]
    Sperrung,
    #[cfg_attr(feature = "serde", serde(rename = "ENTSPERRUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ENTSPERRUNG"))]
    Entsperrung,
    #[cfg_attr(feature = "serde", serde(rename = "MAHNKOSTEN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MAHNKOSTEN"))]
    Mahnkosten,
    #[cfg_attr(feature = "serde", serde(rename = "INKASSOKOSTEN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "INKASSOKOSTEN"))]
    Inkassokosten,
    #[cfg_attr(feature = "serde", serde(rename = "EEG_UMLAGE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "EEG_UMLAGE"))]
    EegUmlage,
    #[cfg_attr(feature = "serde", serde(rename = "ENERGIESTEUER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ENERGIESTEUER"))]
    Energiesteuer,
    #[cfg_attr(feature = "serde", serde(rename = "NETZPREIS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "NETZPREIS"))]
    Netzpreis,
    #[cfg_attr(feature = "serde", serde(rename = "MESSPREIS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MESSPREIS"))]
    Messpreis,
    #[cfg_attr(feature = "serde", serde(rename = "SONSTIGER_PREIS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SONSTIGER_PREIS"))]
    SonstigerPreis,
    #[cfg_attr(feature = "serde", serde(rename = "DIENSTLEISTUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "DIENSTLEISTUNG"))]
    Dienstleistung,
    /// Unknown or future variant — produced when deserializing a value
    /// that is not yet known to this version of the library.
    #[cfg_attr(feature = "serde", serde(other, rename = "UNKNOWN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UNKNOWN"))]
    Unknown,
}
impl Leistungstyp {
    /// All variants defined by the BO4E schema, in declaration order.
    ///
    /// Excludes the forward-compatibility [`Leistungstyp::Unknown`] catch-all, so this
    /// is exactly the set of values that appear on the wire.  Available **without**
    /// the `strum` feature — use it to drift-guard SQL `CHECK` lists and mappings.
    pub const VARIANTS: &'static [Self] = &[
        Self::ArbeitspreisWirkarbeit,
        Self::LeistungspreisWirkleistung,
        Self::ArbeitspreisBlindarbeitInd,
        Self::ArbeitspreisBlindarbeitKap,
        Self::Grundpreis,
        Self::GrundpreisArbeit,
        Self::GrundpreisLeistung,
        Self::Mehrmindermenge,
        Self::Messstellenbetrieb,
        Self::Messdienstleistung,
        Self::MessdienstleistungInklMessung,
        Self::Abrechnung,
        Self::KonzessionsAbgabe,
        Self::KwkUmlage,
        Self::OffshoreUmlage,
        Self::AblavUmlage,
        Self::SonderkundenUmlage,
        Self::RegelenergieUmlage,
        Self::BilanzierungUmlage,
        Self::AuslesungZusaetzlich,
        Self::AblesungZusaetzlich,
        Self::AbrechnungZusaetzlich,
        Self::Sperrung,
        Self::Entsperrung,
        Self::Mahnkosten,
        Self::Inkassokosten,
        Self::EegUmlage,
        Self::Energiesteuer,
        Self::Netzpreis,
        Self::Messpreis,
        Self::SonstigerPreis,
        Self::Dienstleistung,
    ];
    /// Number of schema-defined variants (equal to `VARIANTS.len()`), excluding the
    /// [`Leistungstyp::Unknown`] catch-all.  Stable for this schema version.
    pub const COUNT: usize = Self::VARIANTS.len();
    /// Returns an iterator over all **known** variants of `Leistungstyp`.
    ///
    /// Yields only variants that correspond to values defined in the BO4E schema
    /// (i.e. [`Self::VARIANTS`]), never the [`Leistungstyp::Unknown`] catch-all.
    /// Available **without** the `strum` feature.
    ///
    /// # Example
    /// ```
    /// # use rubo4e::current::Leistungstyp;
    /// // Never yields the `Unknown` catch-all, so the count matches `COUNT`.
    /// assert_eq!(Leistungstyp::iter_known().count(), Leistungstyp::COUNT);
    /// assert!(Leistungstyp::iter_known().all(|v| v.is_known()));
    /// ```
    pub fn iter_known() -> impl Iterator<Item = Self> + Clone {
        Self::VARIANTS.iter().copied()
    }
    /// Returns the canonical BO4E wire string (SCREAMING_SNAKE_CASE) for this value.
    ///
    /// [`Leistungstyp::Unknown`] renders as `"UNKNOWN"`, matching its serialized form.
    pub const fn as_wire(&self) -> &'static str {
        match self {
            Self::ArbeitspreisWirkarbeit => "ARBEITSPREIS_WIRKARBEIT",
            Self::LeistungspreisWirkleistung => "LEISTUNGSPREIS_WIRKLEISTUNG",
            Self::ArbeitspreisBlindarbeitInd => "ARBEITSPREIS_BLINDARBEIT_IND",
            Self::ArbeitspreisBlindarbeitKap => "ARBEITSPREIS_BLINDARBEIT_KAP",
            Self::Grundpreis => "GRUNDPREIS",
            Self::GrundpreisArbeit => "GRUNDPREIS_ARBEIT",
            Self::GrundpreisLeistung => "GRUNDPREIS_LEISTUNG",
            Self::Mehrmindermenge => "MEHRMINDERMENGE",
            Self::Messstellenbetrieb => "MESSSTELLENBETRIEB",
            Self::Messdienstleistung => "MESSDIENSTLEISTUNG",
            Self::MessdienstleistungInklMessung => "MESSDIENSTLEISTUNG_INKL_MESSUNG",
            Self::Abrechnung => "ABRECHNUNG",
            Self::KonzessionsAbgabe => "KONZESSIONS_ABGABE",
            Self::KwkUmlage => "KWK_UMLAGE",
            Self::OffshoreUmlage => "OFFSHORE_UMLAGE",
            Self::AblavUmlage => "ABLAV_UMLAGE",
            Self::SonderkundenUmlage => "SONDERKUNDEN_UMLAGE",
            Self::RegelenergieUmlage => "REGELENERGIE_UMLAGE",
            Self::BilanzierungUmlage => "BILANZIERUNG_UMLAGE",
            Self::AuslesungZusaetzlich => "AUSLESUNG_ZUSAETZLICH",
            Self::AblesungZusaetzlich => "ABLESUNG_ZUSAETZLICH",
            Self::AbrechnungZusaetzlich => "ABRECHNUNG_ZUSAETZLICH",
            Self::Sperrung => "SPERRUNG",
            Self::Entsperrung => "ENTSPERRUNG",
            Self::Mahnkosten => "MAHNKOSTEN",
            Self::Inkassokosten => "INKASSOKOSTEN",
            Self::EegUmlage => "EEG_UMLAGE",
            Self::Energiesteuer => "ENERGIESTEUER",
            Self::Netzpreis => "NETZPREIS",
            Self::Messpreis => "MESSPREIS",
            Self::SonstigerPreis => "SONSTIGER_PREIS",
            Self::Dienstleistung => "DIENSTLEISTUNG",
            Self::Unknown => "UNKNOWN",
        }
    }
    /// **Strictly** parses a BO4E wire string into a known variant.
    ///
    /// Unlike the lenient `serde` / [`FromStr`](std::str::FromStr) path — which maps
    /// any unrecognized value (a typo, a legacy code, or a value from a newer schema)
    /// to [`Leistungstyp::Unknown`] — this returns
    /// [`Err`](crate::error::UnknownVariant) for values not defined in this schema
    /// version, including the literal `"UNKNOWN"`.  Use it at the ingest boundary to
    /// reject bad values instead of silently degrading them.
    ///
    /// # Example
    /// ```
    /// # use rubo4e::current::Leistungstyp;
    /// assert_eq!(Leistungstyp::from_wire("ARBEITSPREIS_WIRKARBEIT"), Ok(Leistungstyp::ArbeitspreisWirkarbeit));
    /// // Out-of-schema values are rejected rather than degraded:
    /// assert!(Leistungstyp::from_wire("NOT_A_REAL_VALUE").is_err());
    /// // …including the `Unknown` catch-all's own wire spelling:
    /// assert!(Leistungstyp::from_wire("UNKNOWN").is_err());
    /// ```
    pub fn from_wire(s: &str) -> Result<Self, crate::error::UnknownVariant> {
        match s {
            "ARBEITSPREIS_WIRKARBEIT" => Ok(Self::ArbeitspreisWirkarbeit),
            "LEISTUNGSPREIS_WIRKLEISTUNG" => Ok(Self::LeistungspreisWirkleistung),
            "ARBEITSPREIS_BLINDARBEIT_IND" => Ok(Self::ArbeitspreisBlindarbeitInd),
            "ARBEITSPREIS_BLINDARBEIT_KAP" => Ok(Self::ArbeitspreisBlindarbeitKap),
            "GRUNDPREIS" => Ok(Self::Grundpreis),
            "GRUNDPREIS_ARBEIT" => Ok(Self::GrundpreisArbeit),
            "GRUNDPREIS_LEISTUNG" => Ok(Self::GrundpreisLeistung),
            "MEHRMINDERMENGE" => Ok(Self::Mehrmindermenge),
            "MESSSTELLENBETRIEB" => Ok(Self::Messstellenbetrieb),
            "MESSDIENSTLEISTUNG" => Ok(Self::Messdienstleistung),
            "MESSDIENSTLEISTUNG_INKL_MESSUNG" => Ok(Self::MessdienstleistungInklMessung),
            "ABRECHNUNG" => Ok(Self::Abrechnung),
            "KONZESSIONS_ABGABE" => Ok(Self::KonzessionsAbgabe),
            "KWK_UMLAGE" => Ok(Self::KwkUmlage),
            "OFFSHORE_UMLAGE" => Ok(Self::OffshoreUmlage),
            "ABLAV_UMLAGE" => Ok(Self::AblavUmlage),
            "SONDERKUNDEN_UMLAGE" => Ok(Self::SonderkundenUmlage),
            "REGELENERGIE_UMLAGE" => Ok(Self::RegelenergieUmlage),
            "BILANZIERUNG_UMLAGE" => Ok(Self::BilanzierungUmlage),
            "AUSLESUNG_ZUSAETZLICH" => Ok(Self::AuslesungZusaetzlich),
            "ABLESUNG_ZUSAETZLICH" => Ok(Self::AblesungZusaetzlich),
            "ABRECHNUNG_ZUSAETZLICH" => Ok(Self::AbrechnungZusaetzlich),
            "SPERRUNG" => Ok(Self::Sperrung),
            "ENTSPERRUNG" => Ok(Self::Entsperrung),
            "MAHNKOSTEN" => Ok(Self::Mahnkosten),
            "INKASSOKOSTEN" => Ok(Self::Inkassokosten),
            "EEG_UMLAGE" => Ok(Self::EegUmlage),
            "ENERGIESTEUER" => Ok(Self::Energiesteuer),
            "NETZPREIS" => Ok(Self::Netzpreis),
            "MESSPREIS" => Ok(Self::Messpreis),
            "SONSTIGER_PREIS" => Ok(Self::SonstigerPreis),
            "DIENSTLEISTUNG" => Ok(Self::Dienstleistung),
            other => Err(crate::error::UnknownVariant::new(other)),
        }
    }
    /// Returns `true` if this value is the forward-compatibility
    /// [`Leistungstyp::Unknown`] catch-all (an out-of-schema value).
    pub const fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
    /// Returns `true` if this value is a known, schema-defined variant.
    pub const fn is_known(&self) -> bool {
        !self.is_unknown()
    }
}
impl std::fmt::Display for Leistungstyp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_wire())
    }
}
impl AsRef<str> for Leistungstyp {
    fn as_ref(&self) -> &str {
        self.as_wire()
    }
}
#[cfg(feature = "versioned")]
impl crate::bo4e_enum_sealed::Sealed for Leistungstyp {}
#[cfg(feature = "versioned")]
impl crate::Bo4eEnum for Leistungstyp {
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
impl crate::Bo4eStrict for Leistungstyp {
    fn collect_unknown_enums(&self, path: &str, out: &mut Vec<String>) {
        if self.is_unknown() {
            out.push(path.to_owned());
        }
    }
}
#[cfg(feature = "sqlx")]
impl sqlx::Type<sqlx::Postgres> for Leistungstyp {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
/// Encodes as the canonical BO4E wire string, borrowed from `as_wire` — no
/// intermediate `String` or `serde_json::Value` is allocated.
#[cfg(feature = "sqlx")]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Leistungstyp {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        let s: &str = self.as_wire();
        <&str as sqlx::Encode<'q, sqlx::Postgres>>::encode_by_ref(&s, buf)
    }
}
/// Decodes leniently, matching the `serde` path: a value the schema does not
/// define becomes [`Leistungstyp::Unknown`] rather than a decode error, so a
/// database row written by a newer schema version still reads back.
///
/// Use [`Leistungstyp::from_wire`] on a `String` column, or check
/// [`Leistungstyp::is_known`], where out-of-schema values must be rejected.
#[cfg(feature = "sqlx")]
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for Leistungstyp {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <&str as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        Ok(Self::from_wire(s).unwrap_or(Self::Unknown))
    }
}
/// Lets `Vec<Leistungstyp>` bind to a `TEXT[]` column.  Only this crate can
/// provide it: the trait and the enum are both foreign to any consumer, so the
/// orphan rule rules out a downstream impl.
#[cfg(feature = "sqlx")]
impl sqlx::postgres::PgHasArrayType for Leistungstyp {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::postgres::PgHasArrayType>::array_type_info()
    }
}
#[cfg(test)]
impl proptest::arbitrary::Arbitrary for Leistungstyp {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        proptest::sample::select(Self::VARIANTS.to_vec()).boxed()
    }
}
