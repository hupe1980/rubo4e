#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(strum::EnumString, strum::EnumIter, strum::IntoStaticStr)
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Codes der Summenzeitreihentypen.
///
/// Die nachfolgenden Codes sind in DE7111 zu nutzen:
/// <https://www.edi-energy.de/index.php?id=38&tx_bdew_bdew%5Buid%5D=695&tx_bdew_bdew%5Baction%5D=download>
/// &tx_bdew_bdew%5Bcontroller%5D=Dokument&cHash=67782e05d8b0f75fbe3a0e1801d07ed0
#[non_exhaustive]
pub enum Zeitreihentyp {
    #[cfg_attr(feature = "serde", serde(rename = "EGS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "EGS"))]
    Egs,
    #[cfg_attr(feature = "serde", serde(rename = "LGS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "LGS"))]
    Lgs,
    #[cfg_attr(feature = "serde", serde(rename = "NZR"))]
    #[cfg_attr(feature = "strum", strum(serialize = "NZR"))]
    Nzr,
    #[cfg_attr(feature = "serde", serde(rename = "SES"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SES"))]
    Ses,
    #[cfg_attr(feature = "serde", serde(rename = "SLS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SLS"))]
    Sls,
    #[cfg_attr(feature = "serde", serde(rename = "TES"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TES"))]
    Tes,
    #[cfg_attr(feature = "serde", serde(rename = "TLS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TLS"))]
    Tls,
    #[cfg_attr(feature = "serde", serde(rename = "SLS_TLS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SLS_TLS"))]
    SlsTls,
    #[cfg_attr(feature = "serde", serde(rename = "SES_TES"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SES_TES"))]
    SesTes,
    /// Unknown or future variant — produced when deserializing a value
    /// that is not yet known to this version of the library.
    #[cfg_attr(feature = "serde", serde(other, rename = "UNKNOWN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UNKNOWN"))]
    Unknown,
}
impl Zeitreihentyp {
    /// All variants defined by the BO4E schema, in declaration order.
    ///
    /// Excludes the forward-compatibility [`Zeitreihentyp::Unknown`] catch-all, so this
    /// is exactly the set of values that appear on the wire.  Available **without**
    /// the `strum` feature — use it to drift-guard SQL `CHECK` lists and mappings.
    pub const VARIANTS: &'static [Self] = &[
        Self::Egs,
        Self::Lgs,
        Self::Nzr,
        Self::Ses,
        Self::Sls,
        Self::Tes,
        Self::Tls,
        Self::SlsTls,
        Self::SesTes,
    ];
    /// Number of schema-defined variants (equal to `VARIANTS.len()`), excluding the
    /// [`Zeitreihentyp::Unknown`] catch-all.  Stable for this schema version.
    pub const COUNT: usize = Self::VARIANTS.len();
    /// Returns an iterator over all **known** variants of `Zeitreihentyp`.
    ///
    /// Yields only variants that correspond to values defined in the BO4E schema
    /// (i.e. [`Self::VARIANTS`]), never the [`Zeitreihentyp::Unknown`] catch-all.
    /// Available **without** the `strum` feature.
    ///
    /// # Example
    /// ```
    /// # use rubo4e::current::Zeitreihentyp;
    /// // Never yields the `Unknown` catch-all, so the count matches `COUNT`.
    /// assert_eq!(Zeitreihentyp::iter_known().count(), Zeitreihentyp::COUNT);
    /// assert!(Zeitreihentyp::iter_known().all(|v| v.is_known()));
    /// ```
    pub fn iter_known() -> impl Iterator<Item = Self> + Clone {
        Self::VARIANTS.iter().copied()
    }
    /// Returns the canonical BO4E wire string (SCREAMING_SNAKE_CASE) for this value.
    ///
    /// [`Zeitreihentyp::Unknown`] renders as `"UNKNOWN"`, matching its serialized form.
    pub const fn as_wire(&self) -> &'static str {
        match self {
            Self::Egs => "EGS",
            Self::Lgs => "LGS",
            Self::Nzr => "NZR",
            Self::Ses => "SES",
            Self::Sls => "SLS",
            Self::Tes => "TES",
            Self::Tls => "TLS",
            Self::SlsTls => "SLS_TLS",
            Self::SesTes => "SES_TES",
            Self::Unknown => "UNKNOWN",
        }
    }
    /// **Strictly** parses a BO4E wire string into a known variant.
    ///
    /// Unlike the lenient `serde` / [`FromStr`](std::str::FromStr) path — which maps
    /// any unrecognized value (a typo, a legacy code, or a value from a newer schema)
    /// to [`Zeitreihentyp::Unknown`] — this returns
    /// [`Err`](crate::error::UnknownVariant) for values not defined in this schema
    /// version, including the literal `"UNKNOWN"`.  Use it at the ingest boundary to
    /// reject bad values instead of silently degrading them.
    ///
    /// # Example
    /// ```
    /// # use rubo4e::current::Zeitreihentyp;
    /// /// assert_eq!(Zeitreihentyp::from_wire("EGS"), Ok(Zeitreihentyp::Egs));
    /// assert!(Zeitreihentyp::from_wire("NOT_A_REAL_VALUE").is_err());
    /// // …including the `Unknown` catch-all's own wire spelling:
    /// assert!(Zeitreihentyp::from_wire("UNKNOWN").is_err());
    /// ```
    pub fn from_wire(s: &str) -> Result<Self, crate::error::UnknownVariant> {
        match s {
            "EGS" => Ok(Self::Egs),
            "LGS" => Ok(Self::Lgs),
            "NZR" => Ok(Self::Nzr),
            "SES" => Ok(Self::Ses),
            "SLS" => Ok(Self::Sls),
            "TES" => Ok(Self::Tes),
            "TLS" => Ok(Self::Tls),
            "SLS_TLS" => Ok(Self::SlsTls),
            "SES_TES" => Ok(Self::SesTes),
            other => Err(crate::error::UnknownVariant::new(other)),
        }
    }
    /// Returns `true` if this value is the forward-compatibility
    /// [`Zeitreihentyp::Unknown`] catch-all (an out-of-schema value).
    pub const fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
    /// Returns `true` if this value is a known, schema-defined variant.
    pub const fn is_known(&self) -> bool {
        !self.is_unknown()
    }
}
impl std::fmt::Display for Zeitreihentyp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_wire())
    }
}
impl AsRef<str> for Zeitreihentyp {
    fn as_ref(&self) -> &str {
        self.as_wire()
    }
}
#[cfg(feature = "versioned")]
impl crate::bo4e_enum_sealed::Sealed for Zeitreihentyp {}
#[cfg(feature = "versioned")]
impl crate::Bo4eEnum for Zeitreihentyp {
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
impl crate::Bo4eStrict for Zeitreihentyp {
    fn collect_unknown_enums(&self, path: &str, out: &mut Vec<String>) {
        if self.is_unknown() {
            out.push(path.to_owned());
        }
    }
}
#[cfg(feature = "sqlx")]
impl sqlx::Type<sqlx::Postgres> for Zeitreihentyp {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
/// Encodes as the canonical BO4E wire string, borrowed from `as_wire` — no
/// intermediate `String` or `serde_json::Value` is allocated.
#[cfg(feature = "sqlx")]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Zeitreihentyp {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        let s: &str = self.as_wire();
        <&str as sqlx::Encode<'q, sqlx::Postgres>>::encode_by_ref(&s, buf)
    }
}
/// Decodes leniently, matching the `serde` path: a value the schema does not
/// define becomes [`Zeitreihentyp::Unknown`] rather than a decode error, so a
/// database row written by a newer schema version still reads back.
///
/// Use [`Zeitreihentyp::from_wire`] on a `String` column, or check
/// [`Zeitreihentyp::is_known`], where out-of-schema values must be rejected.
#[cfg(feature = "sqlx")]
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for Zeitreihentyp {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <&str as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        Ok(Self::from_wire(s).unwrap_or(Self::Unknown))
    }
}
#[cfg(test)]
impl proptest::arbitrary::Arbitrary for Zeitreihentyp {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        proptest::sample::select(Self::VARIANTS.to_vec()).boxed()
    }
}
