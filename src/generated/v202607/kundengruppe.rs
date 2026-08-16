#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(strum::EnumString, strum::EnumIter, strum::IntoStaticStr)
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Kundengruppe für eine Marktlokation (orientiert sich an den Standard-Lastprofilen).
#[non_exhaustive]
pub enum Kundengruppe {
    #[cfg_attr(feature = "serde", serde(rename = "RLM"))]
    #[cfg_attr(feature = "strum", strum(serialize = "RLM"))]
    Rlm,
    #[cfg_attr(feature = "serde", serde(rename = "RLM_KOMMUNAL"))]
    #[cfg_attr(feature = "strum", strum(serialize = "RLM_KOMMUNAL"))]
    RlmKommunal,
    #[cfg_attr(feature = "serde", serde(rename = "SLP_KOMMUNAL"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SLP_KOMMUNAL"))]
    SlpKommunal,
    #[cfg_attr(feature = "serde", serde(rename = "SLP_S_G0"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SLP_S_G0"))]
    SlpSG0,
    #[cfg_attr(feature = "serde", serde(rename = "SLP_S_G1"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SLP_S_G1"))]
    SlpSG1,
    #[cfg_attr(feature = "serde", serde(rename = "SLP_S_G2"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SLP_S_G2"))]
    SlpSG2,
    #[cfg_attr(feature = "serde", serde(rename = "SLP_S_G3"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SLP_S_G3"))]
    SlpSG3,
    #[cfg_attr(feature = "serde", serde(rename = "SLP_S_G4"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SLP_S_G4"))]
    SlpSG4,
    #[cfg_attr(feature = "serde", serde(rename = "SLP_S_G5"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SLP_S_G5"))]
    SlpSG5,
    #[cfg_attr(feature = "serde", serde(rename = "SLP_S_G6"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SLP_S_G6"))]
    SlpSG6,
    #[cfg_attr(feature = "serde", serde(rename = "SLP_S_G7"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SLP_S_G7"))]
    SlpSG7,
    #[cfg_attr(feature = "serde", serde(rename = "SLP_S_L0"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SLP_S_L0"))]
    SlpSL0,
    #[cfg_attr(feature = "serde", serde(rename = "SLP_S_L1"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SLP_S_L1"))]
    SlpSL1,
    #[cfg_attr(feature = "serde", serde(rename = "SLP_S_L2"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SLP_S_L2"))]
    SlpSL2,
    #[cfg_attr(feature = "serde", serde(rename = "SLP_S_H0"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SLP_S_H0"))]
    SlpSH0,
    #[cfg_attr(feature = "serde", serde(rename = "SLP_S_SB"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SLP_S_SB"))]
    SlpSSb,
    #[cfg_attr(feature = "serde", serde(rename = "SLP_S_HZ"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SLP_S_HZ"))]
    SlpSHz,
    #[cfg_attr(feature = "serde", serde(rename = "SLP_S_WP"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SLP_S_WP"))]
    SlpSWp,
    #[cfg_attr(feature = "serde", serde(rename = "SLP_S_EM"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SLP_S_EM"))]
    SlpSEm,
    #[cfg_attr(feature = "serde", serde(rename = "SLP_S_HZ_GEM"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SLP_S_HZ_GEM"))]
    SlpSHzGem,
    #[cfg_attr(feature = "serde", serde(rename = "SLP_G_GKO"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SLP_G_GKO"))]
    SlpGGko,
    #[cfg_attr(feature = "serde", serde(rename = "SLP_G_STANDARD"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SLP_G_STANDARD"))]
    SlpGStandard,
    #[cfg_attr(feature = "serde", serde(rename = "SLP_G_GHA"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SLP_G_GHA"))]
    SlpGGha,
    #[cfg_attr(feature = "serde", serde(rename = "SLP_G_GMK"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SLP_G_GMK"))]
    SlpGGmk,
    #[cfg_attr(feature = "serde", serde(rename = "SLP_G_GBD"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SLP_G_GBD"))]
    SlpGGbd,
    #[cfg_attr(feature = "serde", serde(rename = "SLP_G_GGA"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SLP_G_GGA"))]
    SlpGGga,
    #[cfg_attr(feature = "serde", serde(rename = "SLP_G_GBH"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SLP_G_GBH"))]
    SlpGGbh,
    #[cfg_attr(feature = "serde", serde(rename = "SLP_G_GBA"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SLP_G_GBA"))]
    SlpGGba,
    #[cfg_attr(feature = "serde", serde(rename = "SLP_G_GWA"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SLP_G_GWA"))]
    SlpGGwa,
    #[cfg_attr(feature = "serde", serde(rename = "SLP_G_GGB"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SLP_G_GGB"))]
    SlpGGgb,
    #[cfg_attr(feature = "serde", serde(rename = "SLP_G_GPD"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SLP_G_GPD"))]
    SlpGGpd,
    #[cfg_attr(feature = "serde", serde(rename = "SLP_G_GMF"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SLP_G_GMF"))]
    SlpGGmf,
    #[cfg_attr(feature = "serde", serde(rename = "SLP_G_HEF"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SLP_G_HEF"))]
    SlpGHef,
    #[cfg_attr(feature = "serde", serde(rename = "SLP_G_HMF"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SLP_G_HMF"))]
    SlpGHmf,
    #[cfg_attr(feature = "serde", serde(rename = "SLP_G_HKO"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SLP_G_HKO"))]
    SlpGHko,
    /// Unknown or future variant — produced when deserializing a value
    /// that is not yet known to this version of the library.
    #[cfg_attr(feature = "serde", serde(other, rename = "UNKNOWN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UNKNOWN"))]
    Unknown,
}
impl Kundengruppe {
    /// All variants defined by the BO4E schema, in declaration order.
    ///
    /// Excludes the forward-compatibility [`Kundengruppe::Unknown`] catch-all, so this
    /// is exactly the set of values that appear on the wire.  Available **without**
    /// the `strum` feature — use it to drift-guard SQL `CHECK` lists and mappings.
    pub const VARIANTS: &'static [Self] = &[
        Self::Rlm,
        Self::RlmKommunal,
        Self::SlpKommunal,
        Self::SlpSG0,
        Self::SlpSG1,
        Self::SlpSG2,
        Self::SlpSG3,
        Self::SlpSG4,
        Self::SlpSG5,
        Self::SlpSG6,
        Self::SlpSG7,
        Self::SlpSL0,
        Self::SlpSL1,
        Self::SlpSL2,
        Self::SlpSH0,
        Self::SlpSSb,
        Self::SlpSHz,
        Self::SlpSWp,
        Self::SlpSEm,
        Self::SlpSHzGem,
        Self::SlpGGko,
        Self::SlpGStandard,
        Self::SlpGGha,
        Self::SlpGGmk,
        Self::SlpGGbd,
        Self::SlpGGga,
        Self::SlpGGbh,
        Self::SlpGGba,
        Self::SlpGGwa,
        Self::SlpGGgb,
        Self::SlpGGpd,
        Self::SlpGGmf,
        Self::SlpGHef,
        Self::SlpGHmf,
        Self::SlpGHko,
    ];
    /// Number of schema-defined variants (equal to `VARIANTS.len()`), excluding the
    /// [`Kundengruppe::Unknown`] catch-all.  Stable for this schema version.
    pub const COUNT: usize = Self::VARIANTS.len();
    /// Returns an iterator over all **known** variants of `Kundengruppe`.
    ///
    /// Yields only variants that correspond to values defined in the BO4E schema
    /// (i.e. [`Self::VARIANTS`]), never the [`Kundengruppe::Unknown`] catch-all.
    /// Available **without** the `strum` feature.
    ///
    /// # Example
    /// ```
    /// # use rubo4e::current::Kundengruppe;
    /// // Never yields the `Unknown` catch-all, so the count matches `COUNT`.
    /// assert_eq!(Kundengruppe::iter_known().count(), Kundengruppe::COUNT);
    /// assert!(Kundengruppe::iter_known().all(|v| v.is_known()));
    /// ```
    pub fn iter_known() -> impl Iterator<Item = Self> + Clone {
        Self::VARIANTS.iter().copied()
    }
    /// Returns the canonical BO4E wire string (SCREAMING_SNAKE_CASE) for this value.
    ///
    /// [`Kundengruppe::Unknown`] renders as `"UNKNOWN"`, matching its serialized form.
    pub const fn as_wire(&self) -> &'static str {
        match self {
            Self::Rlm => "RLM",
            Self::RlmKommunal => "RLM_KOMMUNAL",
            Self::SlpKommunal => "SLP_KOMMUNAL",
            Self::SlpSG0 => "SLP_S_G0",
            Self::SlpSG1 => "SLP_S_G1",
            Self::SlpSG2 => "SLP_S_G2",
            Self::SlpSG3 => "SLP_S_G3",
            Self::SlpSG4 => "SLP_S_G4",
            Self::SlpSG5 => "SLP_S_G5",
            Self::SlpSG6 => "SLP_S_G6",
            Self::SlpSG7 => "SLP_S_G7",
            Self::SlpSL0 => "SLP_S_L0",
            Self::SlpSL1 => "SLP_S_L1",
            Self::SlpSL2 => "SLP_S_L2",
            Self::SlpSH0 => "SLP_S_H0",
            Self::SlpSSb => "SLP_S_SB",
            Self::SlpSHz => "SLP_S_HZ",
            Self::SlpSWp => "SLP_S_WP",
            Self::SlpSEm => "SLP_S_EM",
            Self::SlpSHzGem => "SLP_S_HZ_GEM",
            Self::SlpGGko => "SLP_G_GKO",
            Self::SlpGStandard => "SLP_G_STANDARD",
            Self::SlpGGha => "SLP_G_GHA",
            Self::SlpGGmk => "SLP_G_GMK",
            Self::SlpGGbd => "SLP_G_GBD",
            Self::SlpGGga => "SLP_G_GGA",
            Self::SlpGGbh => "SLP_G_GBH",
            Self::SlpGGba => "SLP_G_GBA",
            Self::SlpGGwa => "SLP_G_GWA",
            Self::SlpGGgb => "SLP_G_GGB",
            Self::SlpGGpd => "SLP_G_GPD",
            Self::SlpGGmf => "SLP_G_GMF",
            Self::SlpGHef => "SLP_G_HEF",
            Self::SlpGHmf => "SLP_G_HMF",
            Self::SlpGHko => "SLP_G_HKO",
            Self::Unknown => "UNKNOWN",
        }
    }
    /// **Strictly** parses a BO4E wire string into a known variant.
    ///
    /// Unlike the lenient `serde` / [`FromStr`](std::str::FromStr) path — which maps
    /// any unrecognized value (a typo, a legacy code, or a value from a newer schema)
    /// to [`Kundengruppe::Unknown`] — this returns
    /// [`Err`](crate::error::UnknownVariant) for values not defined in this schema
    /// version, including the literal `"UNKNOWN"`.  Use it at the ingest boundary to
    /// reject bad values instead of silently degrading them.
    ///
    /// # Example
    /// ```
    /// # use rubo4e::current::Kundengruppe;
    /// /// assert_eq!(Kundengruppe::from_wire("RLM"), Ok(Kundengruppe::Rlm));
    /// assert!(Kundengruppe::from_wire("NOT_A_REAL_VALUE").is_err());
    /// // …including the `Unknown` catch-all's own wire spelling:
    /// assert!(Kundengruppe::from_wire("UNKNOWN").is_err());
    /// ```
    pub fn from_wire(s: &str) -> Result<Self, crate::error::UnknownVariant> {
        match s {
            "RLM" => Ok(Self::Rlm),
            "RLM_KOMMUNAL" => Ok(Self::RlmKommunal),
            "SLP_KOMMUNAL" => Ok(Self::SlpKommunal),
            "SLP_S_G0" => Ok(Self::SlpSG0),
            "SLP_S_G1" => Ok(Self::SlpSG1),
            "SLP_S_G2" => Ok(Self::SlpSG2),
            "SLP_S_G3" => Ok(Self::SlpSG3),
            "SLP_S_G4" => Ok(Self::SlpSG4),
            "SLP_S_G5" => Ok(Self::SlpSG5),
            "SLP_S_G6" => Ok(Self::SlpSG6),
            "SLP_S_G7" => Ok(Self::SlpSG7),
            "SLP_S_L0" => Ok(Self::SlpSL0),
            "SLP_S_L1" => Ok(Self::SlpSL1),
            "SLP_S_L2" => Ok(Self::SlpSL2),
            "SLP_S_H0" => Ok(Self::SlpSH0),
            "SLP_S_SB" => Ok(Self::SlpSSb),
            "SLP_S_HZ" => Ok(Self::SlpSHz),
            "SLP_S_WP" => Ok(Self::SlpSWp),
            "SLP_S_EM" => Ok(Self::SlpSEm),
            "SLP_S_HZ_GEM" => Ok(Self::SlpSHzGem),
            "SLP_G_GKO" => Ok(Self::SlpGGko),
            "SLP_G_STANDARD" => Ok(Self::SlpGStandard),
            "SLP_G_GHA" => Ok(Self::SlpGGha),
            "SLP_G_GMK" => Ok(Self::SlpGGmk),
            "SLP_G_GBD" => Ok(Self::SlpGGbd),
            "SLP_G_GGA" => Ok(Self::SlpGGga),
            "SLP_G_GBH" => Ok(Self::SlpGGbh),
            "SLP_G_GBA" => Ok(Self::SlpGGba),
            "SLP_G_GWA" => Ok(Self::SlpGGwa),
            "SLP_G_GGB" => Ok(Self::SlpGGgb),
            "SLP_G_GPD" => Ok(Self::SlpGGpd),
            "SLP_G_GMF" => Ok(Self::SlpGGmf),
            "SLP_G_HEF" => Ok(Self::SlpGHef),
            "SLP_G_HMF" => Ok(Self::SlpGHmf),
            "SLP_G_HKO" => Ok(Self::SlpGHko),
            other => Err(crate::error::UnknownVariant::new(other)),
        }
    }
    /// Returns `true` if this value is the forward-compatibility
    /// [`Kundengruppe::Unknown`] catch-all (an out-of-schema value).
    pub const fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
    /// Returns `true` if this value is a known, schema-defined variant.
    pub const fn is_known(&self) -> bool {
        !self.is_unknown()
    }
}
impl std::fmt::Display for Kundengruppe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_wire())
    }
}
impl AsRef<str> for Kundengruppe {
    fn as_ref(&self) -> &str {
        self.as_wire()
    }
}
#[cfg(feature = "versioned")]
impl crate::bo4e_enum_sealed::Sealed for Kundengruppe {}
#[cfg(feature = "versioned")]
impl crate::Bo4eEnum for Kundengruppe {
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
impl crate::Bo4eStrict for Kundengruppe {
    fn collect_unknown_enums(&self, path: &str, out: &mut Vec<String>) {
        if self.is_unknown() {
            out.push(path.to_owned());
        }
    }
}
#[cfg(feature = "sqlx")]
impl sqlx::Type<sqlx::Postgres> for Kundengruppe {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
/// Encodes as the canonical BO4E wire string, borrowed from `as_wire` — no
/// intermediate `String` or `serde_json::Value` is allocated.
#[cfg(feature = "sqlx")]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Kundengruppe {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        let s: &str = self.as_wire();
        <&str as sqlx::Encode<'q, sqlx::Postgres>>::encode_by_ref(&s, buf)
    }
}
/// Decodes leniently, matching the `serde` path: a value the schema does not
/// define becomes [`Kundengruppe::Unknown`] rather than a decode error, so a
/// database row written by a newer schema version still reads back.
///
/// Use [`Kundengruppe::from_wire`] on a `String` column, or check
/// [`Kundengruppe::is_known`], where out-of-schema values must be rejected.
#[cfg(feature = "sqlx")]
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for Kundengruppe {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <&str as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        Ok(Self::from_wire(s).unwrap_or(Self::Unknown))
    }
}
#[cfg(test)]
impl proptest::arbitrary::Arbitrary for Kundengruppe {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        proptest::sample::select(Self::VARIANTS.to_vec()).boxed()
    }
}
