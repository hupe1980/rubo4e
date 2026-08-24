#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(strum::EnumString, strum::EnumIter, strum::IntoStaticStr)
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Auflistung möglicher Größen von Zählern
#[non_exhaustive]
pub enum Zaehlergroesse {
    #[cfg_attr(feature = "serde", serde(rename = "G2KOMMA5"))]
    #[cfg_attr(feature = "strum", strum(serialize = "G2KOMMA5"))]
    G2Komma5,
    #[cfg_attr(feature = "serde", serde(rename = "G4"))]
    #[cfg_attr(feature = "strum", strum(serialize = "G4"))]
    G4,
    #[cfg_attr(feature = "serde", serde(rename = "G6"))]
    #[cfg_attr(feature = "strum", strum(serialize = "G6"))]
    G6,
    #[cfg_attr(feature = "serde", serde(rename = "G10"))]
    #[cfg_attr(feature = "strum", strum(serialize = "G10"))]
    G10,
    #[cfg_attr(feature = "serde", serde(rename = "G16"))]
    #[cfg_attr(feature = "strum", strum(serialize = "G16"))]
    G16,
    #[cfg_attr(feature = "serde", serde(rename = "G25"))]
    #[cfg_attr(feature = "strum", strum(serialize = "G25"))]
    G25,
    #[cfg_attr(feature = "serde", serde(rename = "G40"))]
    #[cfg_attr(feature = "strum", strum(serialize = "G40"))]
    G40,
    #[cfg_attr(feature = "serde", serde(rename = "G65"))]
    #[cfg_attr(feature = "strum", strum(serialize = "G65"))]
    G65,
    #[cfg_attr(feature = "serde", serde(rename = "G100"))]
    #[cfg_attr(feature = "strum", strum(serialize = "G100"))]
    G100,
    #[cfg_attr(feature = "serde", serde(rename = "G160"))]
    #[cfg_attr(feature = "strum", strum(serialize = "G160"))]
    G160,
    #[cfg_attr(feature = "serde", serde(rename = "G250"))]
    #[cfg_attr(feature = "strum", strum(serialize = "G250"))]
    G250,
    #[cfg_attr(feature = "serde", serde(rename = "G400"))]
    #[cfg_attr(feature = "strum", strum(serialize = "G400"))]
    G400,
    #[cfg_attr(feature = "serde", serde(rename = "G650"))]
    #[cfg_attr(feature = "strum", strum(serialize = "G650"))]
    G650,
    #[cfg_attr(feature = "serde", serde(rename = "G1000"))]
    #[cfg_attr(feature = "strum", strum(serialize = "G1000"))]
    G1000,
    #[cfg_attr(feature = "serde", serde(rename = "G1600"))]
    #[cfg_attr(feature = "strum", strum(serialize = "G1600"))]
    G1600,
    #[cfg_attr(feature = "serde", serde(rename = "G2500"))]
    #[cfg_attr(feature = "strum", strum(serialize = "G2500"))]
    G2500,
    #[cfg_attr(feature = "serde", serde(rename = "G4000"))]
    #[cfg_attr(feature = "strum", strum(serialize = "G4000"))]
    G4000,
    #[cfg_attr(feature = "serde", serde(rename = "G6500"))]
    #[cfg_attr(feature = "strum", strum(serialize = "G6500"))]
    G6500,
    #[cfg_attr(feature = "serde", serde(rename = "G10000"))]
    #[cfg_attr(feature = "strum", strum(serialize = "G10000"))]
    G10000,
    #[cfg_attr(feature = "serde", serde(rename = "G12500"))]
    #[cfg_attr(feature = "strum", strum(serialize = "G12500"))]
    G12500,
    #[cfg_attr(feature = "serde", serde(rename = "G16000"))]
    #[cfg_attr(feature = "strum", strum(serialize = "G16000"))]
    G16000,
    /// Unknown or future variant — produced when deserializing a value
    /// that is not yet known to this version of the library.
    #[cfg_attr(feature = "serde", serde(other, rename = "UNKNOWN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UNKNOWN"))]
    Unknown,
}
impl Zaehlergroesse {
    /// All variants defined by the BO4E schema, in declaration order.
    ///
    /// Excludes the forward-compatibility [`Zaehlergroesse::Unknown`] catch-all, so this
    /// is exactly the set of values that appear on the wire.  Available **without**
    /// the `strum` feature — use it to drift-guard SQL `CHECK` lists and mappings.
    pub const VARIANTS: &'static [Self] = &[
        Self::G2Komma5,
        Self::G4,
        Self::G6,
        Self::G10,
        Self::G16,
        Self::G25,
        Self::G40,
        Self::G65,
        Self::G100,
        Self::G160,
        Self::G250,
        Self::G400,
        Self::G650,
        Self::G1000,
        Self::G1600,
        Self::G2500,
        Self::G4000,
        Self::G6500,
        Self::G10000,
        Self::G12500,
        Self::G16000,
    ];
    /// Number of schema-defined variants (equal to `VARIANTS.len()`), excluding the
    /// [`Zaehlergroesse::Unknown`] catch-all.  Stable for this schema version.
    pub const COUNT: usize = Self::VARIANTS.len();
    /// Returns an iterator over all **known** variants of `Zaehlergroesse`.
    ///
    /// Yields only variants that correspond to values defined in the BO4E schema
    /// (i.e. [`Self::VARIANTS`]), never the [`Zaehlergroesse::Unknown`] catch-all.
    /// Available **without** the `strum` feature.
    ///
    /// # Example
    /// ```
    /// # use rubo4e::current::Zaehlergroesse;
    /// // Never yields the `Unknown` catch-all, so the count matches `COUNT`.
    /// assert_eq!(Zaehlergroesse::iter_known().count(), Zaehlergroesse::COUNT);
    /// assert!(Zaehlergroesse::iter_known().all(|v| v.is_known()));
    /// ```
    pub fn iter_known() -> impl Iterator<Item = Self> + Clone {
        Self::VARIANTS.iter().copied()
    }
    /// Returns the canonical BO4E wire string (SCREAMING_SNAKE_CASE) for this value.
    ///
    /// [`Zaehlergroesse::Unknown`] renders as `"UNKNOWN"`, matching its serialized form.
    pub const fn as_wire(&self) -> &'static str {
        match self {
            Self::G2Komma5 => "G2KOMMA5",
            Self::G4 => "G4",
            Self::G6 => "G6",
            Self::G10 => "G10",
            Self::G16 => "G16",
            Self::G25 => "G25",
            Self::G40 => "G40",
            Self::G65 => "G65",
            Self::G100 => "G100",
            Self::G160 => "G160",
            Self::G250 => "G250",
            Self::G400 => "G400",
            Self::G650 => "G650",
            Self::G1000 => "G1000",
            Self::G1600 => "G1600",
            Self::G2500 => "G2500",
            Self::G4000 => "G4000",
            Self::G6500 => "G6500",
            Self::G10000 => "G10000",
            Self::G12500 => "G12500",
            Self::G16000 => "G16000",
            Self::Unknown => "UNKNOWN",
        }
    }
    /// **Strictly** parses a BO4E wire string into a known variant.
    ///
    /// Unlike the lenient `serde` / [`FromStr`](std::str::FromStr) path — which maps
    /// any unrecognized value (a typo, a legacy code, or a value from a newer schema)
    /// to [`Zaehlergroesse::Unknown`] — this returns
    /// [`Err`](crate::error::UnknownVariant) for values not defined in this schema
    /// version, including the literal `"UNKNOWN"`.  Use it at the ingest boundary to
    /// reject bad values instead of silently degrading them.
    ///
    /// # Example
    /// ```
    /// # use rubo4e::current::Zaehlergroesse;
    /// assert_eq!(Zaehlergroesse::from_wire("G2KOMMA5"), Ok(Zaehlergroesse::G2Komma5));
    /// // Out-of-schema values are rejected rather than degraded:
    /// assert!(Zaehlergroesse::from_wire("NOT_A_REAL_VALUE").is_err());
    /// // …including the `Unknown` catch-all's own wire spelling:
    /// assert!(Zaehlergroesse::from_wire("UNKNOWN").is_err());
    /// ```
    pub fn from_wire(s: &str) -> Result<Self, crate::error::UnknownVariant> {
        match s {
            "G2KOMMA5" => Ok(Self::G2Komma5),
            "G4" => Ok(Self::G4),
            "G6" => Ok(Self::G6),
            "G10" => Ok(Self::G10),
            "G16" => Ok(Self::G16),
            "G25" => Ok(Self::G25),
            "G40" => Ok(Self::G40),
            "G65" => Ok(Self::G65),
            "G100" => Ok(Self::G100),
            "G160" => Ok(Self::G160),
            "G250" => Ok(Self::G250),
            "G400" => Ok(Self::G400),
            "G650" => Ok(Self::G650),
            "G1000" => Ok(Self::G1000),
            "G1600" => Ok(Self::G1600),
            "G2500" => Ok(Self::G2500),
            "G4000" => Ok(Self::G4000),
            "G6500" => Ok(Self::G6500),
            "G10000" => Ok(Self::G10000),
            "G12500" => Ok(Self::G12500),
            "G16000" => Ok(Self::G16000),
            other => Err(crate::error::UnknownVariant::new(other)),
        }
    }
    /// Returns `true` if this value is the forward-compatibility
    /// [`Zaehlergroesse::Unknown`] catch-all (an out-of-schema value).
    pub const fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
    /// Returns `true` if this value is a known, schema-defined variant.
    pub const fn is_known(&self) -> bool {
        !self.is_unknown()
    }
}
impl std::fmt::Display for Zaehlergroesse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_wire())
    }
}
impl AsRef<str> for Zaehlergroesse {
    fn as_ref(&self) -> &str {
        self.as_wire()
    }
}
#[cfg(feature = "versioned")]
impl crate::bo4e_enum_sealed::Sealed for Zaehlergroesse {}
#[cfg(feature = "versioned")]
impl crate::Bo4eEnum for Zaehlergroesse {
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
impl crate::Bo4eStrict for Zaehlergroesse {
    fn collect_unknown_enums(&self, path: &str, out: &mut Vec<String>) {
        if self.is_unknown() {
            out.push(path.to_owned());
        }
    }
}
#[cfg(feature = "sqlx")]
impl sqlx::Type<sqlx::Postgres> for Zaehlergroesse {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
/// Encodes as the canonical BO4E wire string, borrowed from `as_wire` — no
/// intermediate `String` or `serde_json::Value` is allocated.
#[cfg(feature = "sqlx")]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Zaehlergroesse {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        let s: &str = self.as_wire();
        <&str as sqlx::Encode<'q, sqlx::Postgres>>::encode_by_ref(&s, buf)
    }
}
/// Decodes leniently, matching the `serde` path: a value the schema does not
/// define becomes [`Zaehlergroesse::Unknown`] rather than a decode error, so a
/// database row written by a newer schema version still reads back.
///
/// Use [`Zaehlergroesse::from_wire`] on a `String` column, or check
/// [`Zaehlergroesse::is_known`], where out-of-schema values must be rejected.
#[cfg(feature = "sqlx")]
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for Zaehlergroesse {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <&str as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        Ok(Self::from_wire(s).unwrap_or(Self::Unknown))
    }
}
/// Lets `Vec<Zaehlergroesse>` bind to a `TEXT[]` column.  Only this crate can
/// provide it: the trait and the enum are both foreign to any consumer, so the
/// orphan rule rules out a downstream impl.
#[cfg(feature = "sqlx")]
impl sqlx::postgres::PgHasArrayType for Zaehlergroesse {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::postgres::PgHasArrayType>::array_type_info()
    }
}
#[cfg(test)]
impl proptest::arbitrary::Arbitrary for Zaehlergroesse {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        proptest::sample::select(Self::VARIANTS.to_vec()).boxed()
    }
}
