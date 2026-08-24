#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(strum::EnumString, strum::EnumIter, strum::IntoStaticStr)
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Auflistung der Typen von Endkunden. Daraus kann das Verbrauchsprofil abgeleitet werden.
#[non_exhaustive]
pub enum Kundentyp {
    #[cfg_attr(feature = "serde", serde(rename = "GEWERBE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GEWERBE"))]
    Gewerbe,
    #[cfg_attr(feature = "serde", serde(rename = "PRIVAT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "PRIVAT"))]
    Privat,
    #[cfg_attr(feature = "serde", serde(rename = "LANDWIRT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "LANDWIRT"))]
    Landwirt,
    #[cfg_attr(feature = "serde", serde(rename = "SONSTIGE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SONSTIGE"))]
    Sonstige,
    #[cfg_attr(feature = "serde", serde(rename = "HAUSHALT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "HAUSHALT"))]
    Haushalt,
    #[cfg_attr(feature = "serde", serde(rename = "DIREKTHEIZUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "DIREKTHEIZUNG"))]
    Direktheizung,
    #[cfg_attr(feature = "serde", serde(rename = "GEMEINSCHAFT_MFH"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GEMEINSCHAFT_MFH"))]
    GemeinschaftMfh,
    #[cfg_attr(feature = "serde", serde(rename = "KIRCHE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KIRCHE"))]
    Kirche,
    #[cfg_attr(feature = "serde", serde(rename = "KWK"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KWK"))]
    Kwk,
    #[cfg_attr(feature = "serde", serde(rename = "LADESAEULE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "LADESAEULE"))]
    Ladesaeule,
    #[cfg_attr(feature = "serde", serde(rename = "BELEUCHTUNG_OEFFENTLICH"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BELEUCHTUNG_OEFFENTLICH"))]
    BeleuchtungOeffentlich,
    #[cfg_attr(feature = "serde", serde(rename = "BELEUCHTUNG_STRASSE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BELEUCHTUNG_STRASSE"))]
    BeleuchtungStrasse,
    #[cfg_attr(feature = "serde", serde(rename = "SPEICHERHEIZUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SPEICHERHEIZUNG"))]
    Speicherheizung,
    #[cfg_attr(feature = "serde", serde(rename = "UNTERBR_EINRICHTUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UNTERBR_EINRICHTUNG"))]
    UnterbrEinrichtung,
    #[cfg_attr(feature = "serde", serde(rename = "WAERMEPUMPE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "WAERMEPUMPE"))]
    Waermepumpe,
    /// Unknown or future variant — produced when deserializing a value
    /// that is not yet known to this version of the library.
    #[cfg_attr(feature = "serde", serde(other, rename = "UNKNOWN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UNKNOWN"))]
    Unknown,
}
impl Kundentyp {
    /// All variants defined by the BO4E schema, in declaration order.
    ///
    /// Excludes the forward-compatibility [`Kundentyp::Unknown`] catch-all, so this
    /// is exactly the set of values that appear on the wire.  Available **without**
    /// the `strum` feature — use it to drift-guard SQL `CHECK` lists and mappings.
    pub const VARIANTS: &'static [Self] = &[
        Self::Gewerbe,
        Self::Privat,
        Self::Landwirt,
        Self::Sonstige,
        Self::Haushalt,
        Self::Direktheizung,
        Self::GemeinschaftMfh,
        Self::Kirche,
        Self::Kwk,
        Self::Ladesaeule,
        Self::BeleuchtungOeffentlich,
        Self::BeleuchtungStrasse,
        Self::Speicherheizung,
        Self::UnterbrEinrichtung,
        Self::Waermepumpe,
    ];
    /// Number of schema-defined variants (equal to `VARIANTS.len()`), excluding the
    /// [`Kundentyp::Unknown`] catch-all.  Stable for this schema version.
    pub const COUNT: usize = Self::VARIANTS.len();
    /// Returns an iterator over all **known** variants of `Kundentyp`.
    ///
    /// Yields only variants that correspond to values defined in the BO4E schema
    /// (i.e. [`Self::VARIANTS`]), never the [`Kundentyp::Unknown`] catch-all.
    /// Available **without** the `strum` feature.
    ///
    /// # Example
    /// ```
    /// # use rubo4e::current::Kundentyp;
    /// // Never yields the `Unknown` catch-all, so the count matches `COUNT`.
    /// assert_eq!(Kundentyp::iter_known().count(), Kundentyp::COUNT);
    /// assert!(Kundentyp::iter_known().all(|v| v.is_known()));
    /// ```
    pub fn iter_known() -> impl Iterator<Item = Self> + Clone {
        Self::VARIANTS.iter().copied()
    }
    /// Returns the canonical BO4E wire string (SCREAMING_SNAKE_CASE) for this value.
    ///
    /// [`Kundentyp::Unknown`] renders as `"UNKNOWN"`, matching its serialized form.
    pub const fn as_wire(&self) -> &'static str {
        match self {
            Self::Gewerbe => "GEWERBE",
            Self::Privat => "PRIVAT",
            Self::Landwirt => "LANDWIRT",
            Self::Sonstige => "SONSTIGE",
            Self::Haushalt => "HAUSHALT",
            Self::Direktheizung => "DIREKTHEIZUNG",
            Self::GemeinschaftMfh => "GEMEINSCHAFT_MFH",
            Self::Kirche => "KIRCHE",
            Self::Kwk => "KWK",
            Self::Ladesaeule => "LADESAEULE",
            Self::BeleuchtungOeffentlich => "BELEUCHTUNG_OEFFENTLICH",
            Self::BeleuchtungStrasse => "BELEUCHTUNG_STRASSE",
            Self::Speicherheizung => "SPEICHERHEIZUNG",
            Self::UnterbrEinrichtung => "UNTERBR_EINRICHTUNG",
            Self::Waermepumpe => "WAERMEPUMPE",
            Self::Unknown => "UNKNOWN",
        }
    }
    /// **Strictly** parses a BO4E wire string into a known variant.
    ///
    /// Unlike the lenient `serde` / [`FromStr`](std::str::FromStr) path — which maps
    /// any unrecognized value (a typo, a legacy code, or a value from a newer schema)
    /// to [`Kundentyp::Unknown`] — this returns
    /// [`Err`](crate::error::UnknownVariant) for values not defined in this schema
    /// version, including the literal `"UNKNOWN"`.  Use it at the ingest boundary to
    /// reject bad values instead of silently degrading them.
    ///
    /// # Example
    /// ```
    /// # use rubo4e::current::Kundentyp;
    /// assert_eq!(Kundentyp::from_wire("GEWERBE"), Ok(Kundentyp::Gewerbe));
    /// // Out-of-schema values are rejected rather than degraded:
    /// assert!(Kundentyp::from_wire("NOT_A_REAL_VALUE").is_err());
    /// // …including the `Unknown` catch-all's own wire spelling:
    /// assert!(Kundentyp::from_wire("UNKNOWN").is_err());
    /// ```
    pub fn from_wire(s: &str) -> Result<Self, crate::error::UnknownVariant> {
        match s {
            "GEWERBE" => Ok(Self::Gewerbe),
            "PRIVAT" => Ok(Self::Privat),
            "LANDWIRT" => Ok(Self::Landwirt),
            "SONSTIGE" => Ok(Self::Sonstige),
            "HAUSHALT" => Ok(Self::Haushalt),
            "DIREKTHEIZUNG" => Ok(Self::Direktheizung),
            "GEMEINSCHAFT_MFH" => Ok(Self::GemeinschaftMfh),
            "KIRCHE" => Ok(Self::Kirche),
            "KWK" => Ok(Self::Kwk),
            "LADESAEULE" => Ok(Self::Ladesaeule),
            "BELEUCHTUNG_OEFFENTLICH" => Ok(Self::BeleuchtungOeffentlich),
            "BELEUCHTUNG_STRASSE" => Ok(Self::BeleuchtungStrasse),
            "SPEICHERHEIZUNG" => Ok(Self::Speicherheizung),
            "UNTERBR_EINRICHTUNG" => Ok(Self::UnterbrEinrichtung),
            "WAERMEPUMPE" => Ok(Self::Waermepumpe),
            other => Err(crate::error::UnknownVariant::new(other)),
        }
    }
    /// Returns `true` if this value is the forward-compatibility
    /// [`Kundentyp::Unknown`] catch-all (an out-of-schema value).
    pub const fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
    /// Returns `true` if this value is a known, schema-defined variant.
    pub const fn is_known(&self) -> bool {
        !self.is_unknown()
    }
}
impl std::fmt::Display for Kundentyp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_wire())
    }
}
impl AsRef<str> for Kundentyp {
    fn as_ref(&self) -> &str {
        self.as_wire()
    }
}
#[cfg(feature = "versioned")]
impl crate::bo4e_enum_sealed::Sealed for Kundentyp {}
#[cfg(feature = "versioned")]
impl crate::Bo4eEnum for Kundentyp {
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
impl crate::Bo4eStrict for Kundentyp {
    fn collect_unknown_enums(&self, path: &str, out: &mut Vec<String>) {
        if self.is_unknown() {
            out.push(path.to_owned());
        }
    }
}
#[cfg(feature = "sqlx")]
impl sqlx::Type<sqlx::Postgres> for Kundentyp {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
/// Encodes as the canonical BO4E wire string, borrowed from `as_wire` — no
/// intermediate `String` or `serde_json::Value` is allocated.
#[cfg(feature = "sqlx")]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Kundentyp {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        let s: &str = self.as_wire();
        <&str as sqlx::Encode<'q, sqlx::Postgres>>::encode_by_ref(&s, buf)
    }
}
/// Decodes leniently, matching the `serde` path: a value the schema does not
/// define becomes [`Kundentyp::Unknown`] rather than a decode error, so a
/// database row written by a newer schema version still reads back.
///
/// Use [`Kundentyp::from_wire`] on a `String` column, or check
/// [`Kundentyp::is_known`], where out-of-schema values must be rejected.
#[cfg(feature = "sqlx")]
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for Kundentyp {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <&str as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        Ok(Self::from_wire(s).unwrap_or(Self::Unknown))
    }
}
/// Lets `Vec<Kundentyp>` bind to a `TEXT[]` column.  Only this crate can
/// provide it: the trait and the enum are both foreign to any consumer, so the
/// orphan rule rules out a downstream impl.
#[cfg(feature = "sqlx")]
impl sqlx::postgres::PgHasArrayType for Kundentyp {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::postgres::PgHasArrayType>::array_type_info()
    }
}
#[cfg(test)]
impl proptest::arbitrary::Arbitrary for Kundentyp {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        proptest::sample::select(Self::VARIANTS.to_vec()).boxed()
    }
}
