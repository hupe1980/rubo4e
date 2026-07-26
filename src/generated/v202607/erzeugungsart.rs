#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(strum::EnumString, strum::EnumIter, strum::IntoStaticStr)
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Auflistung der Erzeugungsarten von Energie.
#[non_exhaustive]
pub enum Erzeugungsart {
    #[cfg_attr(feature = "serde", serde(rename = "FOSSIL"))]
    #[cfg_attr(feature = "strum", strum(serialize = "FOSSIL"))]
    Fossil,
    #[cfg_attr(feature = "serde", serde(rename = "KWK"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KWK"))]
    Kwk,
    #[cfg_attr(feature = "serde", serde(rename = "WIND"))]
    #[cfg_attr(feature = "strum", strum(serialize = "WIND"))]
    Wind,
    #[cfg_attr(feature = "serde", serde(rename = "SOLAR"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SOLAR"))]
    Solar,
    #[cfg_attr(feature = "serde", serde(rename = "KERNKRAFT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KERNKRAFT"))]
    Kernkraft,
    #[cfg_attr(feature = "serde", serde(rename = "WASSER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "WASSER"))]
    Wasser,
    #[cfg_attr(feature = "serde", serde(rename = "GEOTHERMIE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GEOTHERMIE"))]
    Geothermie,
    #[cfg_attr(feature = "serde", serde(rename = "BIOMASSE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BIOMASSE"))]
    Biomasse,
    #[cfg_attr(feature = "serde", serde(rename = "KOHLE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KOHLE"))]
    Kohle,
    #[cfg_attr(feature = "serde", serde(rename = "GAS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GAS"))]
    Gas,
    #[cfg_attr(feature = "serde", serde(rename = "SONSTIGE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SONSTIGE"))]
    Sonstige,
    #[cfg_attr(feature = "serde", serde(rename = "SONSTIGE_EEG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SONSTIGE_EEG"))]
    SonstigeEeg,
    #[cfg_attr(feature = "serde", serde(rename = "BIOGAS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BIOGAS"))]
    Biogas,
    #[cfg_attr(feature = "serde", serde(rename = "KLIMANEUTRALES_GAS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KLIMANEUTRALES_GAS"))]
    KlimaneutralesGas,
    /// Unknown or future variant — produced when deserializing a value
    /// that is not yet known to this version of the library.
    #[cfg_attr(feature = "serde", serde(other, rename = "UNKNOWN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UNKNOWN"))]
    Unknown,
}
impl Erzeugungsart {
    /// All variants defined by the BO4E schema, in declaration order.
    ///
    /// Excludes the forward-compatibility [`Erzeugungsart::Unknown`] catch-all, so this
    /// is exactly the set of values that appear on the wire.  Available **without**
    /// the `strum` feature — use it to drift-guard SQL `CHECK` lists and mappings.
    pub const VARIANTS: &'static [Self] = &[
        Self::Fossil,
        Self::Kwk,
        Self::Wind,
        Self::Solar,
        Self::Kernkraft,
        Self::Wasser,
        Self::Geothermie,
        Self::Biomasse,
        Self::Kohle,
        Self::Gas,
        Self::Sonstige,
        Self::SonstigeEeg,
        Self::Biogas,
        Self::KlimaneutralesGas,
    ];
    /// Number of schema-defined variants (equal to `VARIANTS.len()`), excluding the
    /// [`Erzeugungsart::Unknown`] catch-all.  Stable for this schema version.
    pub const COUNT: usize = Self::VARIANTS.len();
    /// Returns an iterator over all **known** variants of `Erzeugungsart`.
    ///
    /// Yields only variants that correspond to values defined in the BO4E schema
    /// (i.e. [`Self::VARIANTS`]), never the [`Erzeugungsart::Unknown`] catch-all.
    /// Available **without** the `strum` feature.
    ///
    /// # Example
    /// ```rust,ignore
    /// for v in Erzeugungsart::iter_known() {
    ///     println!("{}", v.as_wire());
    /// }
    /// ```
    pub fn iter_known() -> impl Iterator<Item = Self> + Clone {
        Self::VARIANTS.iter().copied()
    }
    /// Returns the canonical BO4E wire string (SCREAMING_SNAKE_CASE) for this value.
    ///
    /// [`Erzeugungsart::Unknown`] renders as `"UNKNOWN"`, matching its serialized form.
    pub const fn as_wire(&self) -> &'static str {
        match self {
            Self::Fossil => "FOSSIL",
            Self::Kwk => "KWK",
            Self::Wind => "WIND",
            Self::Solar => "SOLAR",
            Self::Kernkraft => "KERNKRAFT",
            Self::Wasser => "WASSER",
            Self::Geothermie => "GEOTHERMIE",
            Self::Biomasse => "BIOMASSE",
            Self::Kohle => "KOHLE",
            Self::Gas => "GAS",
            Self::Sonstige => "SONSTIGE",
            Self::SonstigeEeg => "SONSTIGE_EEG",
            Self::Biogas => "BIOGAS",
            Self::KlimaneutralesGas => "KLIMANEUTRALES_GAS",
            Self::Unknown => "UNKNOWN",
        }
    }
    /// **Strictly** parses a BO4E wire string into a known variant.
    ///
    /// Unlike the lenient `serde` / [`FromStr`](std::str::FromStr) path — which maps
    /// any unrecognized value (a typo, a legacy code, or a value from a newer schema)
    /// to [`Erzeugungsart::Unknown`] — this returns
    /// [`Err`](crate::error::UnknownVariant) for values not defined in this schema
    /// version, including the literal `"UNKNOWN"`.  Use it at the ingest boundary to
    /// reject bad values instead of silently degrading them.
    ///
    /// # Example
    /// ```rust,ignore
    /// assert!(Erzeugungsart::from_wire("NOT_A_REAL_VALUE").is_err());
    /// ```
    pub fn from_wire(s: &str) -> Result<Self, crate::error::UnknownVariant> {
        match s {
            "FOSSIL" => Ok(Self::Fossil),
            "KWK" => Ok(Self::Kwk),
            "WIND" => Ok(Self::Wind),
            "SOLAR" => Ok(Self::Solar),
            "KERNKRAFT" => Ok(Self::Kernkraft),
            "WASSER" => Ok(Self::Wasser),
            "GEOTHERMIE" => Ok(Self::Geothermie),
            "BIOMASSE" => Ok(Self::Biomasse),
            "KOHLE" => Ok(Self::Kohle),
            "GAS" => Ok(Self::Gas),
            "SONSTIGE" => Ok(Self::Sonstige),
            "SONSTIGE_EEG" => Ok(Self::SonstigeEeg),
            "BIOGAS" => Ok(Self::Biogas),
            "KLIMANEUTRALES_GAS" => Ok(Self::KlimaneutralesGas),
            other => Err(crate::error::UnknownVariant::new(other)),
        }
    }
    /// Returns `true` if this value is the forward-compatibility
    /// [`Erzeugungsart::Unknown`] catch-all (an out-of-schema value).
    pub const fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
    /// Returns `true` if this value is a known, schema-defined variant.
    pub const fn is_known(&self) -> bool {
        !self.is_unknown()
    }
}
impl std::fmt::Display for Erzeugungsart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_wire())
    }
}
impl AsRef<str> for Erzeugungsart {
    fn as_ref(&self) -> &str {
        self.as_wire()
    }
}
#[cfg(feature = "versioned")]
impl crate::bo4e_enum_sealed::Sealed for Erzeugungsart {}
#[cfg(feature = "versioned")]
impl crate::Bo4eEnum for Erzeugungsart {
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
impl crate::Bo4eStrict for Erzeugungsart {
    fn collect_unknown_enums(&self, path: &str, out: &mut Vec<String>) {
        if self.is_unknown() {
            out.push(path.to_owned());
        }
    }
}
#[cfg(all(feature = "sqlx", feature = "json"))]
impl sqlx::Type<sqlx::Postgres> for Erzeugungsart {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
/// Encode via the canonical wire string (`as_wire`, always available) — no
/// `serde_json::Value` intermediate, saving an allocation per encode (M-07).
#[cfg(all(feature = "sqlx", feature = "json"))]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Erzeugungsart {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        let s: &str = self.as_wire();
        <&str as sqlx::Encode<'q, sqlx::Postgres>>::encode_by_ref(&s, buf)
    }
}
#[cfg(all(feature = "sqlx", feature = "json"))]
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for Erzeugungsart {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <String as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        serde_json::from_value(serde_json::Value::String(s))
            .map_err(|e| Box::new(e) as sqlx::error::BoxDynError)
    }
}
#[cfg(test)]
impl proptest::arbitrary::Arbitrary for Erzeugungsart {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        proptest::sample::select(Self::VARIANTS.to_vec()).boxed()
    }
}
