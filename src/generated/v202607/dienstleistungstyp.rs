#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(strum::EnumString, strum::EnumIter, strum::IntoStaticStr)
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Auflistung möglicher abzurechnender Dienstleistungen.
#[non_exhaustive]
pub enum Dienstleistungstyp {
    #[cfg_attr(feature = "serde", serde(rename = "DATENBEREITSTELLUNG_TAEGLICH"))]
    #[cfg_attr(feature = "strum", strum(serialize = "DATENBEREITSTELLUNG_TAEGLICH"))]
    DatenbereitstellungTaeglich,
    #[cfg_attr(feature = "serde", serde(rename = "DATENBEREITSTELLUNG_WOECHENTLICH"))]
    #[cfg_attr(
        feature = "strum",
        strum(serialize = "DATENBEREITSTELLUNG_WOECHENTLICH")
    )]
    DatenbereitstellungWoechentlich,
    #[cfg_attr(feature = "serde", serde(rename = "DATENBEREITSTELLUNG_MONATLICH"))]
    #[cfg_attr(feature = "strum", strum(serialize = "DATENBEREITSTELLUNG_MONATLICH"))]
    DatenbereitstellungMonatlich,
    #[cfg_attr(feature = "serde", serde(rename = "DATENBEREITSTELLUNG_JAEHRLICH"))]
    #[cfg_attr(feature = "strum", strum(serialize = "DATENBEREITSTELLUNG_JAEHRLICH"))]
    DatenbereitstellungJaehrlich,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "DATENBEREITSTELLUNG_HISTORISCHE_LG")
    )]
    #[cfg_attr(
        feature = "strum",
        strum(serialize = "DATENBEREITSTELLUNG_HISTORISCHE_LG")
    )]
    DatenbereitstellungHistorischeLg,
    #[cfg_attr(feature = "serde", serde(rename = "DATENBEREITSTELLUNG_STUENDLICH"))]
    #[cfg_attr(feature = "strum", strum(serialize = "DATENBEREITSTELLUNG_STUENDLICH"))]
    DatenbereitstellungStuendlich,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "DATENBEREITSTELLUNG_VIERTELJAEHRLICH")
    )]
    #[cfg_attr(
        feature = "strum",
        strum(serialize = "DATENBEREITSTELLUNG_VIERTELJAEHRLICH")
    )]
    DatenbereitstellungVierteljaehrlich,
    #[cfg_attr(feature = "serde", serde(rename = "DATENBEREITSTELLUNG_HALBJAEHRLICH"))]
    #[cfg_attr(
        feature = "strum",
        strum(serialize = "DATENBEREITSTELLUNG_HALBJAEHRLICH")
    )]
    DatenbereitstellungHalbjaehrlich,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "DATENBEREITSTELLUNG_MONATLICH_ZUSAETZLICH")
    )]
    #[cfg_attr(
        feature = "strum",
        strum(serialize = "DATENBEREITSTELLUNG_MONATLICH_ZUSAETZLICH")
    )]
    DatenbereitstellungMonatlichZusaetzlich,
    #[cfg_attr(feature = "serde", serde(rename = "DATENBEREITSTELLUNG_EINMALIG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "DATENBEREITSTELLUNG_EINMALIG"))]
    DatenbereitstellungEinmalig,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "AUSLESUNG_2X_TAEGLICH_FERNAUSLESUNG")
    )]
    #[cfg_attr(
        feature = "strum",
        strum(serialize = "AUSLESUNG_2X_TAEGLICH_FERNAUSLESUNG")
    )]
    Auslesung2xTaeglichFernauslesung,
    #[cfg_attr(feature = "serde", serde(rename = "AUSLESUNG_TAEGLICH_FERNAUSLESUNG"))]
    #[cfg_attr(
        feature = "strum",
        strum(serialize = "AUSLESUNG_TAEGLICH_FERNAUSLESUNG")
    )]
    AuslesungTaeglichFernauslesung,
    #[cfg_attr(feature = "serde", serde(rename = "AUSLESUNG_MANUELL_MSB"))]
    #[cfg_attr(feature = "strum", strum(serialize = "AUSLESUNG_MANUELL_MSB"))]
    AuslesungManuellMsb,
    #[cfg_attr(feature = "serde", serde(rename = "AUSLESUNG_MONATLICH_FERNAUSLESUNG"))]
    #[cfg_attr(
        feature = "strum",
        strum(serialize = "AUSLESUNG_MONATLICH_FERNAUSLESUNG")
    )]
    AuslesungMonatlichFernauslesung,
    #[cfg_attr(feature = "serde", serde(rename = "AUSLESUNG_JAEHRLICH_FERNAUSLESUNG"))]
    #[cfg_attr(
        feature = "strum",
        strum(serialize = "AUSLESUNG_JAEHRLICH_FERNAUSLESUNG")
    )]
    AuslesungJaehrlichFernauslesung,
    #[cfg_attr(feature = "serde", serde(rename = "AUSLESUNG_MDE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "AUSLESUNG_MDE"))]
    AuslesungMde,
    #[cfg_attr(feature = "serde", serde(rename = "ABLESUNG_MONATLICH"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ABLESUNG_MONATLICH"))]
    AblesungMonatlich,
    #[cfg_attr(feature = "serde", serde(rename = "ABLESUNG_VIERTELJAEHRLICH"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ABLESUNG_VIERTELJAEHRLICH"))]
    AblesungVierteljaehrlich,
    #[cfg_attr(feature = "serde", serde(rename = "ABLESUNG_HALBJAEHRLICH"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ABLESUNG_HALBJAEHRLICH"))]
    AblesungHalbjaehrlich,
    #[cfg_attr(feature = "serde", serde(rename = "ABLESUNG_JAEHRLICH"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ABLESUNG_JAEHRLICH"))]
    AblesungJaehrlich,
    #[cfg_attr(feature = "serde", serde(rename = "AUSLESUNG_FERNAUSLESUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "AUSLESUNG_FERNAUSLESUNG"))]
    AuslesungFernauslesung,
    #[cfg_attr(feature = "serde", serde(rename = "ABLESUNG_ZUSAETZLICH_MSB"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ABLESUNG_ZUSAETZLICH_MSB"))]
    AblesungZusaetzlichMsb,
    #[cfg_attr(feature = "serde", serde(rename = "ABLESUNG_ZUSAETZLICH_KUNDE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ABLESUNG_ZUSAETZLICH_KUNDE"))]
    AblesungZusaetzlichKunde,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "AUSLESUNG_FERNAUSLESUNG_ZUSAETZLICH_MSB")
    )]
    #[cfg_attr(
        feature = "strum",
        strum(serialize = "AUSLESUNG_FERNAUSLESUNG_ZUSAETZLICH_MSB")
    )]
    AuslesungFernauslesungZusaetzlichMsb,
    #[cfg_attr(feature = "serde", serde(rename = "AUSLESUNG_MOATLICH_FERNAUSLESUNG"))]
    #[cfg_attr(
        feature = "strum",
        strum(serialize = "AUSLESUNG_MOATLICH_FERNAUSLESUNG")
    )]
    AuslesungMoatlichFernauslesung,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "AUSLESUNG_STUENDLICH_FERNAUSLESUNG")
    )]
    #[cfg_attr(
        feature = "strum",
        strum(serialize = "AUSLESUNG_STUENDLICH_FERNAUSLESUNG")
    )]
    AuslesungStuendlichFernauslesung,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "AUSLESUNG_TEMPERATURMENGENUMWERTER")
    )]
    #[cfg_attr(
        feature = "strum",
        strum(serialize = "AUSLESUNG_TEMPERATURMENGENUMWERTER")
    )]
    AuslesungTemperaturmengenumwerter,
    #[cfg_attr(feature = "serde", serde(rename = "AUSLESUNG_ZUSTANDSMENGENUMWERTER"))]
    #[cfg_attr(
        feature = "strum",
        strum(serialize = "AUSLESUNG_ZUSTANDSMENGENUMWERTER")
    )]
    AuslesungZustandsmengenumwerter,
    #[cfg_attr(feature = "serde", serde(rename = "AUSLESUNG_SYSTEMMENGENUMWERTER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "AUSLESUNG_SYSTEMMENGENUMWERTER"))]
    AuslesungSystemmengenumwerter,
    #[cfg_attr(feature = "serde", serde(rename = "AUSLESUNG_VORGANG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "AUSLESUNG_VORGANG"))]
    AuslesungVorgang,
    #[cfg_attr(feature = "serde", serde(rename = "AUSLESUNG_KOMPAKTMENGENUMWERTER"))]
    #[cfg_attr(
        feature = "strum",
        strum(serialize = "AUSLESUNG_KOMPAKTMENGENUMWERTER")
    )]
    AuslesungKompaktmengenumwerter,
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
    /// Unknown or future variant — produced when deserializing a value
    /// that is not yet known to this version of the library.
    #[cfg_attr(feature = "serde", serde(other, rename = "UNKNOWN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UNKNOWN"))]
    Unknown,
}
impl Dienstleistungstyp {
    /// All variants defined by the BO4E schema, in declaration order.
    ///
    /// Excludes the forward-compatibility [`Dienstleistungstyp::Unknown`] catch-all, so this
    /// is exactly the set of values that appear on the wire.  Available **without**
    /// the `strum` feature — use it to drift-guard SQL `CHECK` lists and mappings.
    pub const VARIANTS: &'static [Self] = &[
        Self::DatenbereitstellungTaeglich,
        Self::DatenbereitstellungWoechentlich,
        Self::DatenbereitstellungMonatlich,
        Self::DatenbereitstellungJaehrlich,
        Self::DatenbereitstellungHistorischeLg,
        Self::DatenbereitstellungStuendlich,
        Self::DatenbereitstellungVierteljaehrlich,
        Self::DatenbereitstellungHalbjaehrlich,
        Self::DatenbereitstellungMonatlichZusaetzlich,
        Self::DatenbereitstellungEinmalig,
        Self::Auslesung2xTaeglichFernauslesung,
        Self::AuslesungTaeglichFernauslesung,
        Self::AuslesungManuellMsb,
        Self::AuslesungMonatlichFernauslesung,
        Self::AuslesungJaehrlichFernauslesung,
        Self::AuslesungMde,
        Self::AblesungMonatlich,
        Self::AblesungVierteljaehrlich,
        Self::AblesungHalbjaehrlich,
        Self::AblesungJaehrlich,
        Self::AuslesungFernauslesung,
        Self::AblesungZusaetzlichMsb,
        Self::AblesungZusaetzlichKunde,
        Self::AuslesungFernauslesungZusaetzlichMsb,
        Self::AuslesungMoatlichFernauslesung,
        Self::AuslesungStuendlichFernauslesung,
        Self::AuslesungTemperaturmengenumwerter,
        Self::AuslesungZustandsmengenumwerter,
        Self::AuslesungSystemmengenumwerter,
        Self::AuslesungVorgang,
        Self::AuslesungKompaktmengenumwerter,
        Self::Sperrung,
        Self::Entsperrung,
        Self::Mahnkosten,
        Self::Inkassokosten,
    ];
    /// Number of schema-defined variants (equal to `VARIANTS.len()`), excluding the
    /// [`Dienstleistungstyp::Unknown`] catch-all.  Stable for this schema version.
    pub const COUNT: usize = Self::VARIANTS.len();
    /// Returns an iterator over all **known** variants of `Dienstleistungstyp`.
    ///
    /// Yields only variants that correspond to values defined in the BO4E schema
    /// (i.e. [`Self::VARIANTS`]), never the [`Dienstleistungstyp::Unknown`] catch-all.
    /// Available **without** the `strum` feature.
    ///
    /// # Example
    /// ```
    /// # use rubo4e::current::Dienstleistungstyp;
    /// // Never yields the `Unknown` catch-all, so the count matches `COUNT`.
    /// assert_eq!(Dienstleistungstyp::iter_known().count(), Dienstleistungstyp::COUNT);
    /// assert!(Dienstleistungstyp::iter_known().all(|v| v.is_known()));
    /// ```
    pub fn iter_known() -> impl Iterator<Item = Self> + Clone {
        Self::VARIANTS.iter().copied()
    }
    /// Returns the canonical BO4E wire string (SCREAMING_SNAKE_CASE) for this value.
    ///
    /// [`Dienstleistungstyp::Unknown`] renders as `"UNKNOWN"`, matching its serialized form.
    pub const fn as_wire(&self) -> &'static str {
        match self {
            Self::DatenbereitstellungTaeglich => "DATENBEREITSTELLUNG_TAEGLICH",
            Self::DatenbereitstellungWoechentlich => "DATENBEREITSTELLUNG_WOECHENTLICH",
            Self::DatenbereitstellungMonatlich => "DATENBEREITSTELLUNG_MONATLICH",
            Self::DatenbereitstellungJaehrlich => "DATENBEREITSTELLUNG_JAEHRLICH",
            Self::DatenbereitstellungHistorischeLg => "DATENBEREITSTELLUNG_HISTORISCHE_LG",
            Self::DatenbereitstellungStuendlich => "DATENBEREITSTELLUNG_STUENDLICH",
            Self::DatenbereitstellungVierteljaehrlich => "DATENBEREITSTELLUNG_VIERTELJAEHRLICH",
            Self::DatenbereitstellungHalbjaehrlich => "DATENBEREITSTELLUNG_HALBJAEHRLICH",
            Self::DatenbereitstellungMonatlichZusaetzlich => {
                "DATENBEREITSTELLUNG_MONATLICH_ZUSAETZLICH"
            }
            Self::DatenbereitstellungEinmalig => "DATENBEREITSTELLUNG_EINMALIG",
            Self::Auslesung2xTaeglichFernauslesung => "AUSLESUNG_2X_TAEGLICH_FERNAUSLESUNG",
            Self::AuslesungTaeglichFernauslesung => "AUSLESUNG_TAEGLICH_FERNAUSLESUNG",
            Self::AuslesungManuellMsb => "AUSLESUNG_MANUELL_MSB",
            Self::AuslesungMonatlichFernauslesung => "AUSLESUNG_MONATLICH_FERNAUSLESUNG",
            Self::AuslesungJaehrlichFernauslesung => "AUSLESUNG_JAEHRLICH_FERNAUSLESUNG",
            Self::AuslesungMde => "AUSLESUNG_MDE",
            Self::AblesungMonatlich => "ABLESUNG_MONATLICH",
            Self::AblesungVierteljaehrlich => "ABLESUNG_VIERTELJAEHRLICH",
            Self::AblesungHalbjaehrlich => "ABLESUNG_HALBJAEHRLICH",
            Self::AblesungJaehrlich => "ABLESUNG_JAEHRLICH",
            Self::AuslesungFernauslesung => "AUSLESUNG_FERNAUSLESUNG",
            Self::AblesungZusaetzlichMsb => "ABLESUNG_ZUSAETZLICH_MSB",
            Self::AblesungZusaetzlichKunde => "ABLESUNG_ZUSAETZLICH_KUNDE",
            Self::AuslesungFernauslesungZusaetzlichMsb => "AUSLESUNG_FERNAUSLESUNG_ZUSAETZLICH_MSB",
            Self::AuslesungMoatlichFernauslesung => "AUSLESUNG_MOATLICH_FERNAUSLESUNG",
            Self::AuslesungStuendlichFernauslesung => "AUSLESUNG_STUENDLICH_FERNAUSLESUNG",
            Self::AuslesungTemperaturmengenumwerter => "AUSLESUNG_TEMPERATURMENGENUMWERTER",
            Self::AuslesungZustandsmengenumwerter => "AUSLESUNG_ZUSTANDSMENGENUMWERTER",
            Self::AuslesungSystemmengenumwerter => "AUSLESUNG_SYSTEMMENGENUMWERTER",
            Self::AuslesungVorgang => "AUSLESUNG_VORGANG",
            Self::AuslesungKompaktmengenumwerter => "AUSLESUNG_KOMPAKTMENGENUMWERTER",
            Self::Sperrung => "SPERRUNG",
            Self::Entsperrung => "ENTSPERRUNG",
            Self::Mahnkosten => "MAHNKOSTEN",
            Self::Inkassokosten => "INKASSOKOSTEN",
            Self::Unknown => "UNKNOWN",
        }
    }
    /// **Strictly** parses a BO4E wire string into a known variant.
    ///
    /// Unlike the lenient `serde` / [`FromStr`](std::str::FromStr) path — which maps
    /// any unrecognized value (a typo, a legacy code, or a value from a newer schema)
    /// to [`Dienstleistungstyp::Unknown`] — this returns
    /// [`Err`](crate::error::UnknownVariant) for values not defined in this schema
    /// version, including the literal `"UNKNOWN"`.  Use it at the ingest boundary to
    /// reject bad values instead of silently degrading them.
    ///
    /// # Example
    /// ```
    /// # use rubo4e::current::Dienstleistungstyp;
    /// /// assert_eq!(Dienstleistungstyp::from_wire("DATENBEREITSTELLUNG_TAEGLICH"), Ok(Dienstleistungstyp::DatenbereitstellungTaeglich));
    /// assert!(Dienstleistungstyp::from_wire("NOT_A_REAL_VALUE").is_err());
    /// // …including the `Unknown` catch-all's own wire spelling:
    /// assert!(Dienstleistungstyp::from_wire("UNKNOWN").is_err());
    /// ```
    pub fn from_wire(s: &str) -> Result<Self, crate::error::UnknownVariant> {
        match s {
            "DATENBEREITSTELLUNG_TAEGLICH" => Ok(Self::DatenbereitstellungTaeglich),
            "DATENBEREITSTELLUNG_WOECHENTLICH" => Ok(Self::DatenbereitstellungWoechentlich),
            "DATENBEREITSTELLUNG_MONATLICH" => Ok(Self::DatenbereitstellungMonatlich),
            "DATENBEREITSTELLUNG_JAEHRLICH" => Ok(Self::DatenbereitstellungJaehrlich),
            "DATENBEREITSTELLUNG_HISTORISCHE_LG" => Ok(Self::DatenbereitstellungHistorischeLg),
            "DATENBEREITSTELLUNG_STUENDLICH" => Ok(Self::DatenbereitstellungStuendlich),
            "DATENBEREITSTELLUNG_VIERTELJAEHRLICH" => Ok(Self::DatenbereitstellungVierteljaehrlich),
            "DATENBEREITSTELLUNG_HALBJAEHRLICH" => Ok(Self::DatenbereitstellungHalbjaehrlich),
            "DATENBEREITSTELLUNG_MONATLICH_ZUSAETZLICH" => {
                Ok(Self::DatenbereitstellungMonatlichZusaetzlich)
            }
            "DATENBEREITSTELLUNG_EINMALIG" => Ok(Self::DatenbereitstellungEinmalig),
            "AUSLESUNG_2X_TAEGLICH_FERNAUSLESUNG" => Ok(Self::Auslesung2xTaeglichFernauslesung),
            "AUSLESUNG_TAEGLICH_FERNAUSLESUNG" => Ok(Self::AuslesungTaeglichFernauslesung),
            "AUSLESUNG_MANUELL_MSB" => Ok(Self::AuslesungManuellMsb),
            "AUSLESUNG_MONATLICH_FERNAUSLESUNG" => Ok(Self::AuslesungMonatlichFernauslesung),
            "AUSLESUNG_JAEHRLICH_FERNAUSLESUNG" => Ok(Self::AuslesungJaehrlichFernauslesung),
            "AUSLESUNG_MDE" => Ok(Self::AuslesungMde),
            "ABLESUNG_MONATLICH" => Ok(Self::AblesungMonatlich),
            "ABLESUNG_VIERTELJAEHRLICH" => Ok(Self::AblesungVierteljaehrlich),
            "ABLESUNG_HALBJAEHRLICH" => Ok(Self::AblesungHalbjaehrlich),
            "ABLESUNG_JAEHRLICH" => Ok(Self::AblesungJaehrlich),
            "AUSLESUNG_FERNAUSLESUNG" => Ok(Self::AuslesungFernauslesung),
            "ABLESUNG_ZUSAETZLICH_MSB" => Ok(Self::AblesungZusaetzlichMsb),
            "ABLESUNG_ZUSAETZLICH_KUNDE" => Ok(Self::AblesungZusaetzlichKunde),
            "AUSLESUNG_FERNAUSLESUNG_ZUSAETZLICH_MSB" => {
                Ok(Self::AuslesungFernauslesungZusaetzlichMsb)
            }
            "AUSLESUNG_MOATLICH_FERNAUSLESUNG" => Ok(Self::AuslesungMoatlichFernauslesung),
            "AUSLESUNG_STUENDLICH_FERNAUSLESUNG" => Ok(Self::AuslesungStuendlichFernauslesung),
            "AUSLESUNG_TEMPERATURMENGENUMWERTER" => Ok(Self::AuslesungTemperaturmengenumwerter),
            "AUSLESUNG_ZUSTANDSMENGENUMWERTER" => Ok(Self::AuslesungZustandsmengenumwerter),
            "AUSLESUNG_SYSTEMMENGENUMWERTER" => Ok(Self::AuslesungSystemmengenumwerter),
            "AUSLESUNG_VORGANG" => Ok(Self::AuslesungVorgang),
            "AUSLESUNG_KOMPAKTMENGENUMWERTER" => Ok(Self::AuslesungKompaktmengenumwerter),
            "SPERRUNG" => Ok(Self::Sperrung),
            "ENTSPERRUNG" => Ok(Self::Entsperrung),
            "MAHNKOSTEN" => Ok(Self::Mahnkosten),
            "INKASSOKOSTEN" => Ok(Self::Inkassokosten),
            other => Err(crate::error::UnknownVariant::new(other)),
        }
    }
    /// Returns `true` if this value is the forward-compatibility
    /// [`Dienstleistungstyp::Unknown`] catch-all (an out-of-schema value).
    pub const fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
    /// Returns `true` if this value is a known, schema-defined variant.
    pub const fn is_known(&self) -> bool {
        !self.is_unknown()
    }
}
impl std::fmt::Display for Dienstleistungstyp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_wire())
    }
}
impl AsRef<str> for Dienstleistungstyp {
    fn as_ref(&self) -> &str {
        self.as_wire()
    }
}
#[cfg(feature = "versioned")]
impl crate::bo4e_enum_sealed::Sealed for Dienstleistungstyp {}
#[cfg(feature = "versioned")]
impl crate::Bo4eEnum for Dienstleistungstyp {
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
impl crate::Bo4eStrict for Dienstleistungstyp {
    fn collect_unknown_enums(&self, path: &str, out: &mut Vec<String>) {
        if self.is_unknown() {
            out.push(path.to_owned());
        }
    }
}
#[cfg(feature = "sqlx")]
impl sqlx::Type<sqlx::Postgres> for Dienstleistungstyp {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
/// Encodes as the canonical BO4E wire string, borrowed from `as_wire` — no
/// intermediate `String` or `serde_json::Value` is allocated.
#[cfg(feature = "sqlx")]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Dienstleistungstyp {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        let s: &str = self.as_wire();
        <&str as sqlx::Encode<'q, sqlx::Postgres>>::encode_by_ref(&s, buf)
    }
}
/// Decodes leniently, matching the `serde` path: a value the schema does not
/// define becomes [`Dienstleistungstyp::Unknown`] rather than a decode error, so a
/// database row written by a newer schema version still reads back.
///
/// Use [`Dienstleistungstyp::from_wire`] on a `String` column, or check
/// [`Dienstleistungstyp::is_known`], where out-of-schema values must be rejected.
#[cfg(feature = "sqlx")]
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for Dienstleistungstyp {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <&str as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        Ok(Self::from_wire(s).unwrap_or(Self::Unknown))
    }
}
#[cfg(test)]
impl proptest::arbitrary::Arbitrary for Dienstleistungstyp {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        proptest::sample::select(Self::VARIANTS.to_vec()).boxed()
    }
}
