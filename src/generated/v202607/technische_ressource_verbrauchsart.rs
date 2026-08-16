#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(strum::EnumString, strum::EnumIter, strum::IntoStaticStr)
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Verbrauchsart der technischen Ressource
#[non_exhaustive]
pub enum TechnischeRessourceVerbrauchsart {
    #[cfg_attr(feature = "serde", serde(rename = "KRAFT_LICHT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KRAFT_LICHT"))]
    KraftLicht,
    #[cfg_attr(feature = "serde", serde(rename = "WAERME"))]
    #[cfg_attr(feature = "strum", strum(serialize = "WAERME"))]
    Waerme,
    #[cfg_attr(feature = "serde", serde(rename = "E_MOBILITAET"))]
    #[cfg_attr(feature = "strum", strum(serialize = "E_MOBILITAET"))]
    EMobilitaet,
    #[cfg_attr(feature = "serde", serde(rename = "STRASSENBELEUCHTUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "STRASSENBELEUCHTUNG"))]
    Strassenbeleuchtung,
    /// Unknown or future variant — produced when deserializing a value
    /// that is not yet known to this version of the library.
    #[cfg_attr(feature = "serde", serde(other, rename = "UNKNOWN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UNKNOWN"))]
    Unknown,
}
impl TechnischeRessourceVerbrauchsart {
    /// All variants defined by the BO4E schema, in declaration order.
    ///
    /// Excludes the forward-compatibility [`TechnischeRessourceVerbrauchsart::Unknown`] catch-all, so this
    /// is exactly the set of values that appear on the wire.  Available **without**
    /// the `strum` feature — use it to drift-guard SQL `CHECK` lists and mappings.
    pub const VARIANTS: &'static [Self] = &[
        Self::KraftLicht,
        Self::Waerme,
        Self::EMobilitaet,
        Self::Strassenbeleuchtung,
    ];
    /// Number of schema-defined variants (equal to `VARIANTS.len()`), excluding the
    /// [`TechnischeRessourceVerbrauchsart::Unknown`] catch-all.  Stable for this schema version.
    pub const COUNT: usize = Self::VARIANTS.len();
    /// Returns an iterator over all **known** variants of `TechnischeRessourceVerbrauchsart`.
    ///
    /// Yields only variants that correspond to values defined in the BO4E schema
    /// (i.e. [`Self::VARIANTS`]), never the [`TechnischeRessourceVerbrauchsart::Unknown`] catch-all.
    /// Available **without** the `strum` feature.
    ///
    /// # Example
    /// ```
    /// # use rubo4e::current::TechnischeRessourceVerbrauchsart;
    /// // Never yields the `Unknown` catch-all, so the count matches `COUNT`.
    /// assert_eq!(TechnischeRessourceVerbrauchsart::iter_known().count(), TechnischeRessourceVerbrauchsart::COUNT);
    /// assert!(TechnischeRessourceVerbrauchsart::iter_known().all(|v| v.is_known()));
    /// ```
    pub fn iter_known() -> impl Iterator<Item = Self> + Clone {
        Self::VARIANTS.iter().copied()
    }
    /// Returns the canonical BO4E wire string (SCREAMING_SNAKE_CASE) for this value.
    ///
    /// [`TechnischeRessourceVerbrauchsart::Unknown`] renders as `"UNKNOWN"`, matching its serialized form.
    pub const fn as_wire(&self) -> &'static str {
        match self {
            Self::KraftLicht => "KRAFT_LICHT",
            Self::Waerme => "WAERME",
            Self::EMobilitaet => "E_MOBILITAET",
            Self::Strassenbeleuchtung => "STRASSENBELEUCHTUNG",
            Self::Unknown => "UNKNOWN",
        }
    }
    /// **Strictly** parses a BO4E wire string into a known variant.
    ///
    /// Unlike the lenient `serde` / [`FromStr`](std::str::FromStr) path — which maps
    /// any unrecognized value (a typo, a legacy code, or a value from a newer schema)
    /// to [`TechnischeRessourceVerbrauchsart::Unknown`] — this returns
    /// [`Err`](crate::error::UnknownVariant) for values not defined in this schema
    /// version, including the literal `"UNKNOWN"`.  Use it at the ingest boundary to
    /// reject bad values instead of silently degrading them.
    ///
    /// # Example
    /// ```
    /// # use rubo4e::current::TechnischeRessourceVerbrauchsart;
    /// /// assert_eq!(TechnischeRessourceVerbrauchsart::from_wire("KRAFT_LICHT"), Ok(TechnischeRessourceVerbrauchsart::KraftLicht));
    /// assert!(TechnischeRessourceVerbrauchsart::from_wire("NOT_A_REAL_VALUE").is_err());
    /// // …including the `Unknown` catch-all's own wire spelling:
    /// assert!(TechnischeRessourceVerbrauchsart::from_wire("UNKNOWN").is_err());
    /// ```
    pub fn from_wire(s: &str) -> Result<Self, crate::error::UnknownVariant> {
        match s {
            "KRAFT_LICHT" => Ok(Self::KraftLicht),
            "WAERME" => Ok(Self::Waerme),
            "E_MOBILITAET" => Ok(Self::EMobilitaet),
            "STRASSENBELEUCHTUNG" => Ok(Self::Strassenbeleuchtung),
            other => Err(crate::error::UnknownVariant::new(other)),
        }
    }
    /// Returns `true` if this value is the forward-compatibility
    /// [`TechnischeRessourceVerbrauchsart::Unknown`] catch-all (an out-of-schema value).
    pub const fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
    /// Returns `true` if this value is a known, schema-defined variant.
    pub const fn is_known(&self) -> bool {
        !self.is_unknown()
    }
}
impl std::fmt::Display for TechnischeRessourceVerbrauchsart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_wire())
    }
}
impl AsRef<str> for TechnischeRessourceVerbrauchsart {
    fn as_ref(&self) -> &str {
        self.as_wire()
    }
}
#[cfg(feature = "versioned")]
impl crate::bo4e_enum_sealed::Sealed for TechnischeRessourceVerbrauchsart {}
#[cfg(feature = "versioned")]
impl crate::Bo4eEnum for TechnischeRessourceVerbrauchsart {
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
impl crate::Bo4eStrict for TechnischeRessourceVerbrauchsart {
    fn collect_unknown_enums(&self, path: &str, out: &mut Vec<String>) {
        if self.is_unknown() {
            out.push(path.to_owned());
        }
    }
}
#[cfg(feature = "sqlx")]
impl sqlx::Type<sqlx::Postgres> for TechnischeRessourceVerbrauchsart {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
/// Encodes as the canonical BO4E wire string, borrowed from `as_wire` — no
/// intermediate `String` or `serde_json::Value` is allocated.
#[cfg(feature = "sqlx")]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for TechnischeRessourceVerbrauchsart {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        let s: &str = self.as_wire();
        <&str as sqlx::Encode<'q, sqlx::Postgres>>::encode_by_ref(&s, buf)
    }
}
/// Decodes leniently, matching the `serde` path: a value the schema does not
/// define becomes [`TechnischeRessourceVerbrauchsart::Unknown`] rather than a decode error, so a
/// database row written by a newer schema version still reads back.
///
/// Use [`TechnischeRessourceVerbrauchsart::from_wire`] on a `String` column, or check
/// [`TechnischeRessourceVerbrauchsart::is_known`], where out-of-schema values must be rejected.
#[cfg(feature = "sqlx")]
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for TechnischeRessourceVerbrauchsart {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <&str as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        Ok(Self::from_wire(s).unwrap_or(Self::Unknown))
    }
}
#[cfg(test)]
impl proptest::arbitrary::Arbitrary for TechnischeRessourceVerbrauchsart {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        proptest::sample::select(Self::VARIANTS.to_vec()).boxed()
    }
}
