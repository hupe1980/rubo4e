#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(strum::EnumString, strum::EnumIter, strum::IntoStaticStr)
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Aufzählung der Labels für Öko-Strom von verschiedenen Herausgebern.
#[non_exhaustive]
pub enum Oekolabel {
    #[cfg_attr(feature = "serde", serde(rename = "ENERGREEN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ENERGREEN"))]
    Energreen,
    #[cfg_attr(feature = "serde", serde(rename = "GASGREEN_GRUENER_STROM"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GASGREEN_GRUENER_STROM"))]
    GasgreenGruenerStrom,
    #[cfg_attr(feature = "serde", serde(rename = "GASGREEN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GASGREEN"))]
    Gasgreen,
    #[cfg_attr(feature = "serde", serde(rename = "GRUENER_STROM_GOLD"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GRUENER_STROM_GOLD"))]
    GruenerStromGold,
    #[cfg_attr(feature = "serde", serde(rename = "GRUENER_STROM_SILBER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GRUENER_STROM_SILBER"))]
    GruenerStromSilber,
    #[cfg_attr(feature = "serde", serde(rename = "GRUENER_STROM"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GRUENER_STROM"))]
    GruenerStrom,
    #[cfg_attr(feature = "serde", serde(rename = "GRUENES_GAS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GRUENES_GAS"))]
    GruenesGas,
    #[cfg_attr(feature = "serde", serde(rename = "NATURWATT_STROM"))]
    #[cfg_attr(feature = "strum", strum(serialize = "NATURWATT_STROM"))]
    NaturwattStrom,
    #[cfg_attr(feature = "serde", serde(rename = "OK_POWER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "OK_POWER"))]
    OkPower,
    #[cfg_attr(feature = "serde", serde(rename = "RENEWABLE_PLUS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "RENEWABLE_PLUS"))]
    RenewablePlus,
    #[cfg_attr(feature = "serde", serde(rename = "WATERGREEN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "WATERGREEN"))]
    Watergreen,
    #[cfg_attr(feature = "serde", serde(rename = "WATERGREEN_PLUS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "WATERGREEN_PLUS"))]
    WatergreenPlus,
    /// Unknown or future variant — produced when deserializing a value
    /// that is not yet known to this version of the library.
    #[cfg_attr(feature = "serde", serde(other, rename = "UNKNOWN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UNKNOWN"))]
    Unknown,
}
impl Oekolabel {
    /// All variants defined by the BO4E schema, in declaration order.
    ///
    /// Excludes the forward-compatibility [`Oekolabel::Unknown`] catch-all, so this
    /// is exactly the set of values that appear on the wire.  Available **without**
    /// the `strum` feature — use it to drift-guard SQL `CHECK` lists and mappings.
    pub const VARIANTS: &'static [Self] = &[
        Self::Energreen,
        Self::GasgreenGruenerStrom,
        Self::Gasgreen,
        Self::GruenerStromGold,
        Self::GruenerStromSilber,
        Self::GruenerStrom,
        Self::GruenesGas,
        Self::NaturwattStrom,
        Self::OkPower,
        Self::RenewablePlus,
        Self::Watergreen,
        Self::WatergreenPlus,
    ];
    /// Number of schema-defined variants (equal to `VARIANTS.len()`), excluding the
    /// [`Oekolabel::Unknown`] catch-all.  Stable for this schema version.
    pub const COUNT: usize = Self::VARIANTS.len();
    /// Returns an iterator over all **known** variants of `Oekolabel`.
    ///
    /// Yields only variants that correspond to values defined in the BO4E schema
    /// (i.e. [`Self::VARIANTS`]), never the [`Oekolabel::Unknown`] catch-all.
    /// Available **without** the `strum` feature.
    ///
    /// # Example
    /// ```rust,ignore
    /// for v in Oekolabel::iter_known() {
    ///     println!("{}", v.as_wire());
    /// }
    /// ```
    pub fn iter_known() -> impl Iterator<Item = Self> + Clone {
        Self::VARIANTS.iter().copied()
    }
    /// Returns the canonical BO4E wire string (SCREAMING_SNAKE_CASE) for this value.
    ///
    /// [`Oekolabel::Unknown`] renders as `"UNKNOWN"`, matching its serialized form.
    pub const fn as_wire(&self) -> &'static str {
        match self {
            Self::Energreen => "ENERGREEN",
            Self::GasgreenGruenerStrom => "GASGREEN_GRUENER_STROM",
            Self::Gasgreen => "GASGREEN",
            Self::GruenerStromGold => "GRUENER_STROM_GOLD",
            Self::GruenerStromSilber => "GRUENER_STROM_SILBER",
            Self::GruenerStrom => "GRUENER_STROM",
            Self::GruenesGas => "GRUENES_GAS",
            Self::NaturwattStrom => "NATURWATT_STROM",
            Self::OkPower => "OK_POWER",
            Self::RenewablePlus => "RENEWABLE_PLUS",
            Self::Watergreen => "WATERGREEN",
            Self::WatergreenPlus => "WATERGREEN_PLUS",
            Self::Unknown => "UNKNOWN",
        }
    }
    /// **Strictly** parses a BO4E wire string into a known variant.
    ///
    /// Unlike the lenient `serde` / [`FromStr`](std::str::FromStr) path — which maps
    /// any unrecognized value (a typo, a legacy code, or a value from a newer schema)
    /// to [`Oekolabel::Unknown`] — this returns
    /// [`Err`](crate::error::UnknownVariant) for values not defined in this schema
    /// version, including the literal `"UNKNOWN"`.  Use it at the ingest boundary to
    /// reject bad values instead of silently degrading them.
    ///
    /// # Example
    /// ```rust,ignore
    /// assert!(Oekolabel::from_wire("NOT_A_REAL_VALUE").is_err());
    /// ```
    pub fn from_wire(s: &str) -> Result<Self, crate::error::UnknownVariant> {
        match s {
            "ENERGREEN" => Ok(Self::Energreen),
            "GASGREEN_GRUENER_STROM" => Ok(Self::GasgreenGruenerStrom),
            "GASGREEN" => Ok(Self::Gasgreen),
            "GRUENER_STROM_GOLD" => Ok(Self::GruenerStromGold),
            "GRUENER_STROM_SILBER" => Ok(Self::GruenerStromSilber),
            "GRUENER_STROM" => Ok(Self::GruenerStrom),
            "GRUENES_GAS" => Ok(Self::GruenesGas),
            "NATURWATT_STROM" => Ok(Self::NaturwattStrom),
            "OK_POWER" => Ok(Self::OkPower),
            "RENEWABLE_PLUS" => Ok(Self::RenewablePlus),
            "WATERGREEN" => Ok(Self::Watergreen),
            "WATERGREEN_PLUS" => Ok(Self::WatergreenPlus),
            other => Err(crate::error::UnknownVariant::new(other)),
        }
    }
    /// Returns `true` if this value is the forward-compatibility
    /// [`Oekolabel::Unknown`] catch-all (an out-of-schema value).
    pub const fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
    /// Returns `true` if this value is a known, schema-defined variant.
    pub const fn is_known(&self) -> bool {
        !self.is_unknown()
    }
}
impl std::fmt::Display for Oekolabel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_wire())
    }
}
impl AsRef<str> for Oekolabel {
    fn as_ref(&self) -> &str {
        self.as_wire()
    }
}
#[cfg(feature = "versioned")]
impl crate::bo4e_enum_sealed::Sealed for Oekolabel {}
#[cfg(feature = "versioned")]
impl crate::Bo4eEnum for Oekolabel {
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
impl crate::Bo4eStrict for Oekolabel {
    fn collect_unknown_enums(&self, path: &str, out: &mut Vec<String>) {
        if self.is_unknown() {
            out.push(path.to_owned());
        }
    }
}
#[cfg(all(feature = "sqlx", feature = "json"))]
impl sqlx::Type<sqlx::Postgres> for Oekolabel {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
/// Encode via the canonical wire string (`as_wire`, always available) — no
/// `serde_json::Value` intermediate, saving an allocation per encode (M-07).
#[cfg(all(feature = "sqlx", feature = "json"))]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Oekolabel {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        let s: &str = self.as_wire();
        <&str as sqlx::Encode<'q, sqlx::Postgres>>::encode_by_ref(&s, buf)
    }
}
#[cfg(all(feature = "sqlx", feature = "json"))]
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for Oekolabel {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <String as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        serde_json::from_value(serde_json::Value::String(s))
            .map_err(|e| Box::new(e) as sqlx::error::BoxDynError)
    }
}
#[cfg(test)]
impl proptest::arbitrary::Arbitrary for Oekolabel {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        proptest::sample::select(Self::VARIANTS.to_vec()).boxed()
    }
}
