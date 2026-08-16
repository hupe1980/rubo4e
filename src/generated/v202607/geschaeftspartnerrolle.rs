#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(strum::EnumString, strum::EnumIter, strum::IntoStaticStr)
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Diese Rollen kann ein Geschäftspartner einnehmen.
#[non_exhaustive]
pub enum Geschaeftspartnerrolle {
    #[cfg_attr(feature = "serde", serde(rename = "LIEFERANT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "LIEFERANT"))]
    Lieferant,
    #[cfg_attr(feature = "serde", serde(rename = "DIENSTLEISTER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "DIENSTLEISTER"))]
    Dienstleister,
    #[cfg_attr(feature = "serde", serde(rename = "KUNDE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KUNDE"))]
    Kunde,
    #[cfg_attr(feature = "serde", serde(rename = "INTERESSENT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "INTERESSENT"))]
    Interessent,
    #[cfg_attr(feature = "serde", serde(rename = "MARKTPARTNER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MARKTPARTNER"))]
    Marktpartner,
    /// Unknown or future variant — produced when deserializing a value
    /// that is not yet known to this version of the library.
    #[cfg_attr(feature = "serde", serde(other, rename = "UNKNOWN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UNKNOWN"))]
    Unknown,
}
impl Geschaeftspartnerrolle {
    /// All variants defined by the BO4E schema, in declaration order.
    ///
    /// Excludes the forward-compatibility [`Geschaeftspartnerrolle::Unknown`] catch-all, so this
    /// is exactly the set of values that appear on the wire.  Available **without**
    /// the `strum` feature — use it to drift-guard SQL `CHECK` lists and mappings.
    pub const VARIANTS: &'static [Self] = &[
        Self::Lieferant,
        Self::Dienstleister,
        Self::Kunde,
        Self::Interessent,
        Self::Marktpartner,
    ];
    /// Number of schema-defined variants (equal to `VARIANTS.len()`), excluding the
    /// [`Geschaeftspartnerrolle::Unknown`] catch-all.  Stable for this schema version.
    pub const COUNT: usize = Self::VARIANTS.len();
    /// Returns an iterator over all **known** variants of `Geschaeftspartnerrolle`.
    ///
    /// Yields only variants that correspond to values defined in the BO4E schema
    /// (i.e. [`Self::VARIANTS`]), never the [`Geschaeftspartnerrolle::Unknown`] catch-all.
    /// Available **without** the `strum` feature.
    ///
    /// # Example
    /// ```
    /// # use rubo4e::current::Geschaeftspartnerrolle;
    /// // Never yields the `Unknown` catch-all, so the count matches `COUNT`.
    /// assert_eq!(Geschaeftspartnerrolle::iter_known().count(), Geschaeftspartnerrolle::COUNT);
    /// assert!(Geschaeftspartnerrolle::iter_known().all(|v| v.is_known()));
    /// ```
    pub fn iter_known() -> impl Iterator<Item = Self> + Clone {
        Self::VARIANTS.iter().copied()
    }
    /// Returns the canonical BO4E wire string (SCREAMING_SNAKE_CASE) for this value.
    ///
    /// [`Geschaeftspartnerrolle::Unknown`] renders as `"UNKNOWN"`, matching its serialized form.
    pub const fn as_wire(&self) -> &'static str {
        match self {
            Self::Lieferant => "LIEFERANT",
            Self::Dienstleister => "DIENSTLEISTER",
            Self::Kunde => "KUNDE",
            Self::Interessent => "INTERESSENT",
            Self::Marktpartner => "MARKTPARTNER",
            Self::Unknown => "UNKNOWN",
        }
    }
    /// **Strictly** parses a BO4E wire string into a known variant.
    ///
    /// Unlike the lenient `serde` / [`FromStr`](std::str::FromStr) path — which maps
    /// any unrecognized value (a typo, a legacy code, or a value from a newer schema)
    /// to [`Geschaeftspartnerrolle::Unknown`] — this returns
    /// [`Err`](crate::error::UnknownVariant) for values not defined in this schema
    /// version, including the literal `"UNKNOWN"`.  Use it at the ingest boundary to
    /// reject bad values instead of silently degrading them.
    ///
    /// # Example
    /// ```
    /// # use rubo4e::current::Geschaeftspartnerrolle;
    /// /// assert_eq!(Geschaeftspartnerrolle::from_wire("LIEFERANT"), Ok(Geschaeftspartnerrolle::Lieferant));
    /// assert!(Geschaeftspartnerrolle::from_wire("NOT_A_REAL_VALUE").is_err());
    /// // …including the `Unknown` catch-all's own wire spelling:
    /// assert!(Geschaeftspartnerrolle::from_wire("UNKNOWN").is_err());
    /// ```
    pub fn from_wire(s: &str) -> Result<Self, crate::error::UnknownVariant> {
        match s {
            "LIEFERANT" => Ok(Self::Lieferant),
            "DIENSTLEISTER" => Ok(Self::Dienstleister),
            "KUNDE" => Ok(Self::Kunde),
            "INTERESSENT" => Ok(Self::Interessent),
            "MARKTPARTNER" => Ok(Self::Marktpartner),
            other => Err(crate::error::UnknownVariant::new(other)),
        }
    }
    /// Returns `true` if this value is the forward-compatibility
    /// [`Geschaeftspartnerrolle::Unknown`] catch-all (an out-of-schema value).
    pub const fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
    /// Returns `true` if this value is a known, schema-defined variant.
    pub const fn is_known(&self) -> bool {
        !self.is_unknown()
    }
}
impl std::fmt::Display for Geschaeftspartnerrolle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_wire())
    }
}
impl AsRef<str> for Geschaeftspartnerrolle {
    fn as_ref(&self) -> &str {
        self.as_wire()
    }
}
#[cfg(feature = "versioned")]
impl crate::bo4e_enum_sealed::Sealed for Geschaeftspartnerrolle {}
#[cfg(feature = "versioned")]
impl crate::Bo4eEnum for Geschaeftspartnerrolle {
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
impl crate::Bo4eStrict for Geschaeftspartnerrolle {
    fn collect_unknown_enums(&self, path: &str, out: &mut Vec<String>) {
        if self.is_unknown() {
            out.push(path.to_owned());
        }
    }
}
#[cfg(feature = "sqlx")]
impl sqlx::Type<sqlx::Postgres> for Geschaeftspartnerrolle {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
/// Encodes as the canonical BO4E wire string, borrowed from `as_wire` — no
/// intermediate `String` or `serde_json::Value` is allocated.
#[cfg(feature = "sqlx")]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Geschaeftspartnerrolle {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        let s: &str = self.as_wire();
        <&str as sqlx::Encode<'q, sqlx::Postgres>>::encode_by_ref(&s, buf)
    }
}
/// Decodes leniently, matching the `serde` path: a value the schema does not
/// define becomes [`Geschaeftspartnerrolle::Unknown`] rather than a decode error, so a
/// database row written by a newer schema version still reads back.
///
/// Use [`Geschaeftspartnerrolle::from_wire`] on a `String` column, or check
/// [`Geschaeftspartnerrolle::is_known`], where out-of-schema values must be rejected.
#[cfg(feature = "sqlx")]
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for Geschaeftspartnerrolle {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <&str as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        Ok(Self::from_wire(s).unwrap_or(Self::Unknown))
    }
}
#[cfg(test)]
impl proptest::arbitrary::Arbitrary for Geschaeftspartnerrolle {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        proptest::sample::select(Self::VARIANTS.to_vec()).boxed()
    }
}
