#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(strum::EnumString, strum::EnumIter, strum::IntoStaticStr)
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Diese Rollen kann ein Marktteilnehmer einnehmen.
#[non_exhaustive]
pub enum Marktrolle {
    #[cfg_attr(feature = "serde", serde(rename = "BTR"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BTR"))]
    Btr,
    #[cfg_attr(feature = "serde", serde(rename = "BIKO"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BIKO"))]
    Biko,
    #[cfg_attr(feature = "serde", serde(rename = "BKV"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BKV"))]
    Bkv,
    #[cfg_attr(feature = "serde", serde(rename = "DP"))]
    #[cfg_attr(feature = "strum", strum(serialize = "DP"))]
    Dp,
    #[cfg_attr(feature = "serde", serde(rename = "EIV"))]
    #[cfg_attr(feature = "strum", strum(serialize = "EIV"))]
    Eiv,
    #[cfg_attr(feature = "serde", serde(rename = "ESA"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ESA"))]
    Esa,
    #[cfg_attr(feature = "serde", serde(rename = "KN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KN"))]
    Kn,
    #[cfg_attr(feature = "serde", serde(rename = "LF"))]
    #[cfg_attr(feature = "strum", strum(serialize = "LF"))]
    Lf,
    #[cfg_attr(feature = "serde", serde(rename = "MGV"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MGV"))]
    Mgv,
    #[cfg_attr(feature = "serde", serde(rename = "MSB"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MSB"))]
    Msb,
    #[cfg_attr(feature = "serde", serde(rename = "NB"))]
    #[cfg_attr(feature = "strum", strum(serialize = "NB"))]
    Nb,
    #[cfg_attr(feature = "serde", serde(rename = "RB"))]
    #[cfg_attr(feature = "strum", strum(serialize = "RB"))]
    Rb,
    #[cfg_attr(feature = "serde", serde(rename = "UENB"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UENB"))]
    Uenb,
    /// Unknown or future variant — produced when deserializing a value
    /// that is not yet known to this version of the library.
    #[cfg_attr(feature = "serde", serde(other, rename = "UNKNOWN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UNKNOWN"))]
    Unknown,
}
impl Marktrolle {
    /// All variants defined by the BO4E schema, in declaration order.
    ///
    /// Excludes the forward-compatibility [`Marktrolle::Unknown`] catch-all, so this
    /// is exactly the set of values that appear on the wire.  Available **without**
    /// the `strum` feature — use it to drift-guard SQL `CHECK` lists and mappings.
    pub const VARIANTS: &'static [Self] = &[
        Self::Btr,
        Self::Biko,
        Self::Bkv,
        Self::Dp,
        Self::Eiv,
        Self::Esa,
        Self::Kn,
        Self::Lf,
        Self::Mgv,
        Self::Msb,
        Self::Nb,
        Self::Rb,
        Self::Uenb,
    ];
    /// Number of schema-defined variants (equal to `VARIANTS.len()`), excluding the
    /// [`Marktrolle::Unknown`] catch-all.  Stable for this schema version.
    pub const COUNT: usize = Self::VARIANTS.len();
    /// Returns an iterator over all **known** variants of `Marktrolle`.
    ///
    /// Yields only variants that correspond to values defined in the BO4E schema
    /// (i.e. [`Self::VARIANTS`]), never the [`Marktrolle::Unknown`] catch-all.
    /// Available **without** the `strum` feature.
    ///
    /// # Example
    /// ```
    /// # use rubo4e::current::Marktrolle;
    /// // Never yields the `Unknown` catch-all, so the count matches `COUNT`.
    /// assert_eq!(Marktrolle::iter_known().count(), Marktrolle::COUNT);
    /// assert!(Marktrolle::iter_known().all(|v| v.is_known()));
    /// ```
    pub fn iter_known() -> impl Iterator<Item = Self> + Clone {
        Self::VARIANTS.iter().copied()
    }
    /// Returns the canonical BO4E wire string (SCREAMING_SNAKE_CASE) for this value.
    ///
    /// [`Marktrolle::Unknown`] renders as `"UNKNOWN"`, matching its serialized form.
    pub const fn as_wire(&self) -> &'static str {
        match self {
            Self::Btr => "BTR",
            Self::Biko => "BIKO",
            Self::Bkv => "BKV",
            Self::Dp => "DP",
            Self::Eiv => "EIV",
            Self::Esa => "ESA",
            Self::Kn => "KN",
            Self::Lf => "LF",
            Self::Mgv => "MGV",
            Self::Msb => "MSB",
            Self::Nb => "NB",
            Self::Rb => "RB",
            Self::Uenb => "UENB",
            Self::Unknown => "UNKNOWN",
        }
    }
    /// **Strictly** parses a BO4E wire string into a known variant.
    ///
    /// Unlike the lenient `serde` / [`FromStr`](std::str::FromStr) path — which maps
    /// any unrecognized value (a typo, a legacy code, or a value from a newer schema)
    /// to [`Marktrolle::Unknown`] — this returns
    /// [`Err`](crate::error::UnknownVariant) for values not defined in this schema
    /// version, including the literal `"UNKNOWN"`.  Use it at the ingest boundary to
    /// reject bad values instead of silently degrading them.
    ///
    /// # Example
    /// ```
    /// # use rubo4e::current::Marktrolle;
    /// /// assert_eq!(Marktrolle::from_wire("BTR"), Ok(Marktrolle::Btr));
    /// assert!(Marktrolle::from_wire("NOT_A_REAL_VALUE").is_err());
    /// // …including the `Unknown` catch-all's own wire spelling:
    /// assert!(Marktrolle::from_wire("UNKNOWN").is_err());
    /// ```
    pub fn from_wire(s: &str) -> Result<Self, crate::error::UnknownVariant> {
        match s {
            "BTR" => Ok(Self::Btr),
            "BIKO" => Ok(Self::Biko),
            "BKV" => Ok(Self::Bkv),
            "DP" => Ok(Self::Dp),
            "EIV" => Ok(Self::Eiv),
            "ESA" => Ok(Self::Esa),
            "KN" => Ok(Self::Kn),
            "LF" => Ok(Self::Lf),
            "MGV" => Ok(Self::Mgv),
            "MSB" => Ok(Self::Msb),
            "NB" => Ok(Self::Nb),
            "RB" => Ok(Self::Rb),
            "UENB" => Ok(Self::Uenb),
            other => Err(crate::error::UnknownVariant::new(other)),
        }
    }
    /// Returns `true` if this value is the forward-compatibility
    /// [`Marktrolle::Unknown`] catch-all (an out-of-schema value).
    pub const fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
    /// Returns `true` if this value is a known, schema-defined variant.
    pub const fn is_known(&self) -> bool {
        !self.is_unknown()
    }
}
impl std::fmt::Display for Marktrolle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_wire())
    }
}
impl AsRef<str> for Marktrolle {
    fn as_ref(&self) -> &str {
        self.as_wire()
    }
}
#[cfg(feature = "versioned")]
impl crate::bo4e_enum_sealed::Sealed for Marktrolle {}
#[cfg(feature = "versioned")]
impl crate::Bo4eEnum for Marktrolle {
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
impl crate::Bo4eStrict for Marktrolle {
    fn collect_unknown_enums(&self, path: &str, out: &mut Vec<String>) {
        if self.is_unknown() {
            out.push(path.to_owned());
        }
    }
}
#[cfg(feature = "sqlx")]
impl sqlx::Type<sqlx::Postgres> for Marktrolle {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
/// Encodes as the canonical BO4E wire string, borrowed from `as_wire` — no
/// intermediate `String` or `serde_json::Value` is allocated.
#[cfg(feature = "sqlx")]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Marktrolle {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        let s: &str = self.as_wire();
        <&str as sqlx::Encode<'q, sqlx::Postgres>>::encode_by_ref(&s, buf)
    }
}
/// Decodes leniently, matching the `serde` path: a value the schema does not
/// define becomes [`Marktrolle::Unknown`] rather than a decode error, so a
/// database row written by a newer schema version still reads back.
///
/// Use [`Marktrolle::from_wire`] on a `String` column, or check
/// [`Marktrolle::is_known`], where out-of-schema values must be rejected.
#[cfg(feature = "sqlx")]
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for Marktrolle {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <&str as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        Ok(Self::from_wire(s).unwrap_or(Self::Unknown))
    }
}
#[cfg(test)]
impl proptest::arbitrary::Arbitrary for Marktrolle {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        proptest::sample::select(Self::VARIANTS.to_vec()).boxed()
    }
}
