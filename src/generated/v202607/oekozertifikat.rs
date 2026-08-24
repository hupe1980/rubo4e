#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(strum::EnumString, strum::EnumIter, strum::IntoStaticStr)
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Zertifikate für Ökostrom von verschiedenen Herausgebern.
#[non_exhaustive]
pub enum Oekozertifikat {
    #[cfg_attr(feature = "serde", serde(rename = "CMS_EE01"))]
    #[cfg_attr(feature = "strum", strum(serialize = "CMS_EE01"))]
    CmsEe01,
    #[cfg_attr(feature = "serde", serde(rename = "CMS_EE02"))]
    #[cfg_attr(feature = "strum", strum(serialize = "CMS_EE02"))]
    CmsEe02,
    #[cfg_attr(feature = "serde", serde(rename = "EECS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "EECS"))]
    Eecs,
    #[cfg_attr(feature = "serde", serde(rename = "FRAUNHOFER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "FRAUNHOFER"))]
    Fraunhofer,
    #[cfg_attr(feature = "serde", serde(rename = "BET"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BET"))]
    Bet,
    #[cfg_attr(feature = "serde", serde(rename = "KLIMA_INVEST"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KLIMA_INVEST"))]
    KlimaInvest,
    #[cfg_attr(feature = "serde", serde(rename = "LGA"))]
    #[cfg_attr(feature = "strum", strum(serialize = "LGA"))]
    Lga,
    #[cfg_attr(feature = "serde", serde(rename = "FREIBERG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "FREIBERG"))]
    Freiberg,
    #[cfg_attr(feature = "serde", serde(rename = "RECS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "RECS"))]
    Recs,
    #[cfg_attr(feature = "serde", serde(rename = "REGS_EGL"))]
    #[cfg_attr(feature = "strum", strum(serialize = "REGS_EGL"))]
    RegsEgl,
    #[cfg_attr(feature = "serde", serde(rename = "TUEV"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TUEV"))]
    Tuev,
    #[cfg_attr(feature = "serde", serde(rename = "TUEV_HESSEN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TUEV_HESSEN"))]
    TuevHessen,
    #[cfg_attr(feature = "serde", serde(rename = "TUEV_NORD"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TUEV_NORD"))]
    TuevNord,
    #[cfg_attr(feature = "serde", serde(rename = "TUEV_RHEINLAND"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TUEV_RHEINLAND"))]
    TuevRheinland,
    #[cfg_attr(feature = "serde", serde(rename = "TUEV_SUED"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TUEV_SUED"))]
    TuevSued,
    #[cfg_attr(feature = "serde", serde(rename = "TUEV_SUED_EE01"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TUEV_SUED_EE01"))]
    TuevSuedEe01,
    #[cfg_attr(feature = "serde", serde(rename = "TUEV_SUED_EE02"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TUEV_SUED_EE02"))]
    TuevSuedEe02,
    /// Unknown or future variant — produced when deserializing a value
    /// that is not yet known to this version of the library.
    #[cfg_attr(feature = "serde", serde(other, rename = "UNKNOWN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UNKNOWN"))]
    Unknown,
}
impl Oekozertifikat {
    /// All variants defined by the BO4E schema, in declaration order.
    ///
    /// Excludes the forward-compatibility [`Oekozertifikat::Unknown`] catch-all, so this
    /// is exactly the set of values that appear on the wire.  Available **without**
    /// the `strum` feature — use it to drift-guard SQL `CHECK` lists and mappings.
    pub const VARIANTS: &'static [Self] = &[
        Self::CmsEe01,
        Self::CmsEe02,
        Self::Eecs,
        Self::Fraunhofer,
        Self::Bet,
        Self::KlimaInvest,
        Self::Lga,
        Self::Freiberg,
        Self::Recs,
        Self::RegsEgl,
        Self::Tuev,
        Self::TuevHessen,
        Self::TuevNord,
        Self::TuevRheinland,
        Self::TuevSued,
        Self::TuevSuedEe01,
        Self::TuevSuedEe02,
    ];
    /// Number of schema-defined variants (equal to `VARIANTS.len()`), excluding the
    /// [`Oekozertifikat::Unknown`] catch-all.  Stable for this schema version.
    pub const COUNT: usize = Self::VARIANTS.len();
    /// Returns an iterator over all **known** variants of `Oekozertifikat`.
    ///
    /// Yields only variants that correspond to values defined in the BO4E schema
    /// (i.e. [`Self::VARIANTS`]), never the [`Oekozertifikat::Unknown`] catch-all.
    /// Available **without** the `strum` feature.
    ///
    /// # Example
    /// ```
    /// # use rubo4e::current::Oekozertifikat;
    /// // Never yields the `Unknown` catch-all, so the count matches `COUNT`.
    /// assert_eq!(Oekozertifikat::iter_known().count(), Oekozertifikat::COUNT);
    /// assert!(Oekozertifikat::iter_known().all(|v| v.is_known()));
    /// ```
    pub fn iter_known() -> impl Iterator<Item = Self> + Clone {
        Self::VARIANTS.iter().copied()
    }
    /// Returns the canonical BO4E wire string (SCREAMING_SNAKE_CASE) for this value.
    ///
    /// [`Oekozertifikat::Unknown`] renders as `"UNKNOWN"`, matching its serialized form.
    pub const fn as_wire(&self) -> &'static str {
        match self {
            Self::CmsEe01 => "CMS_EE01",
            Self::CmsEe02 => "CMS_EE02",
            Self::Eecs => "EECS",
            Self::Fraunhofer => "FRAUNHOFER",
            Self::Bet => "BET",
            Self::KlimaInvest => "KLIMA_INVEST",
            Self::Lga => "LGA",
            Self::Freiberg => "FREIBERG",
            Self::Recs => "RECS",
            Self::RegsEgl => "REGS_EGL",
            Self::Tuev => "TUEV",
            Self::TuevHessen => "TUEV_HESSEN",
            Self::TuevNord => "TUEV_NORD",
            Self::TuevRheinland => "TUEV_RHEINLAND",
            Self::TuevSued => "TUEV_SUED",
            Self::TuevSuedEe01 => "TUEV_SUED_EE01",
            Self::TuevSuedEe02 => "TUEV_SUED_EE02",
            Self::Unknown => "UNKNOWN",
        }
    }
    /// **Strictly** parses a BO4E wire string into a known variant.
    ///
    /// Unlike the lenient `serde` / [`FromStr`](std::str::FromStr) path — which maps
    /// any unrecognized value (a typo, a legacy code, or a value from a newer schema)
    /// to [`Oekozertifikat::Unknown`] — this returns
    /// [`Err`](crate::error::UnknownVariant) for values not defined in this schema
    /// version, including the literal `"UNKNOWN"`.  Use it at the ingest boundary to
    /// reject bad values instead of silently degrading them.
    ///
    /// # Example
    /// ```
    /// # use rubo4e::current::Oekozertifikat;
    /// assert_eq!(Oekozertifikat::from_wire("CMS_EE01"), Ok(Oekozertifikat::CmsEe01));
    /// // Out-of-schema values are rejected rather than degraded:
    /// assert!(Oekozertifikat::from_wire("NOT_A_REAL_VALUE").is_err());
    /// // …including the `Unknown` catch-all's own wire spelling:
    /// assert!(Oekozertifikat::from_wire("UNKNOWN").is_err());
    /// ```
    pub fn from_wire(s: &str) -> Result<Self, crate::error::UnknownVariant> {
        match s {
            "CMS_EE01" => Ok(Self::CmsEe01),
            "CMS_EE02" => Ok(Self::CmsEe02),
            "EECS" => Ok(Self::Eecs),
            "FRAUNHOFER" => Ok(Self::Fraunhofer),
            "BET" => Ok(Self::Bet),
            "KLIMA_INVEST" => Ok(Self::KlimaInvest),
            "LGA" => Ok(Self::Lga),
            "FREIBERG" => Ok(Self::Freiberg),
            "RECS" => Ok(Self::Recs),
            "REGS_EGL" => Ok(Self::RegsEgl),
            "TUEV" => Ok(Self::Tuev),
            "TUEV_HESSEN" => Ok(Self::TuevHessen),
            "TUEV_NORD" => Ok(Self::TuevNord),
            "TUEV_RHEINLAND" => Ok(Self::TuevRheinland),
            "TUEV_SUED" => Ok(Self::TuevSued),
            "TUEV_SUED_EE01" => Ok(Self::TuevSuedEe01),
            "TUEV_SUED_EE02" => Ok(Self::TuevSuedEe02),
            other => Err(crate::error::UnknownVariant::new(other)),
        }
    }
    /// Returns `true` if this value is the forward-compatibility
    /// [`Oekozertifikat::Unknown`] catch-all (an out-of-schema value).
    pub const fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
    /// Returns `true` if this value is a known, schema-defined variant.
    pub const fn is_known(&self) -> bool {
        !self.is_unknown()
    }
}
impl std::fmt::Display for Oekozertifikat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_wire())
    }
}
impl AsRef<str> for Oekozertifikat {
    fn as_ref(&self) -> &str {
        self.as_wire()
    }
}
#[cfg(feature = "versioned")]
impl crate::bo4e_enum_sealed::Sealed for Oekozertifikat {}
#[cfg(feature = "versioned")]
impl crate::Bo4eEnum for Oekozertifikat {
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
impl crate::Bo4eStrict for Oekozertifikat {
    fn collect_unknown_enums(&self, path: &str, out: &mut Vec<String>) {
        if self.is_unknown() {
            out.push(path.to_owned());
        }
    }
}
#[cfg(feature = "sqlx")]
impl sqlx::Type<sqlx::Postgres> for Oekozertifikat {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
/// Encodes as the canonical BO4E wire string, borrowed from `as_wire` — no
/// intermediate `String` or `serde_json::Value` is allocated.
#[cfg(feature = "sqlx")]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Oekozertifikat {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        let s: &str = self.as_wire();
        <&str as sqlx::Encode<'q, sqlx::Postgres>>::encode_by_ref(&s, buf)
    }
}
/// Decodes leniently, matching the `serde` path: a value the schema does not
/// define becomes [`Oekozertifikat::Unknown`] rather than a decode error, so a
/// database row written by a newer schema version still reads back.
///
/// Use [`Oekozertifikat::from_wire`] on a `String` column, or check
/// [`Oekozertifikat::is_known`], where out-of-schema values must be rejected.
#[cfg(feature = "sqlx")]
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for Oekozertifikat {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <&str as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        Ok(Self::from_wire(s).unwrap_or(Self::Unknown))
    }
}
/// Lets `Vec<Oekozertifikat>` bind to a `TEXT[]` column.  Only this crate can
/// provide it: the trait and the enum are both foreign to any consumer, so the
/// orphan rule rules out a downstream impl.
#[cfg(feature = "sqlx")]
impl sqlx::postgres::PgHasArrayType for Oekozertifikat {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::postgres::PgHasArrayType>::array_type_info()
    }
}
#[cfg(test)]
impl proptest::arbitrary::Arbitrary for Oekozertifikat {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        proptest::sample::select(Self::VARIANTS.to_vec()).boxed()
    }
}
