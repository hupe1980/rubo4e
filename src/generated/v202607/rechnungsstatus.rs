#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(strum::EnumString, strum::EnumIter, strum::IntoStaticStr)
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Abbildung verschiedener Zustände, die im Rahmen der Rechnungsbearbeitung durchlaufen werden.
#[non_exhaustive]
pub enum Rechnungsstatus {
    #[cfg_attr(feature = "serde", serde(rename = "UNGEPRUEFT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UNGEPRUEFT"))]
    Ungeprueft,
    #[cfg_attr(feature = "serde", serde(rename = "GEPRUEFT_OK"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GEPRUEFT_OK"))]
    GeprueftOk,
    #[cfg_attr(feature = "serde", serde(rename = "GEPRUEFT_FEHLERHAFT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GEPRUEFT_FEHLERHAFT"))]
    GeprueftFehlerhaft,
    #[cfg_attr(feature = "serde", serde(rename = "GEBUCHT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GEBUCHT"))]
    Gebucht,
    #[cfg_attr(feature = "serde", serde(rename = "BEZAHLT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BEZAHLT"))]
    Bezahlt,
    /// Unknown or future variant — produced when deserializing a value
    /// that is not yet known to this version of the library.
    #[cfg_attr(feature = "serde", serde(other, rename = "UNKNOWN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UNKNOWN"))]
    Unknown,
}
impl Rechnungsstatus {
    /// All variants defined by the BO4E schema, in declaration order.
    ///
    /// Excludes the forward-compatibility [`Rechnungsstatus::Unknown`] catch-all, so this
    /// is exactly the set of values that appear on the wire.  Available **without**
    /// the `strum` feature — use it to drift-guard SQL `CHECK` lists and mappings.
    pub const VARIANTS: &'static [Self] = &[
        Self::Ungeprueft,
        Self::GeprueftOk,
        Self::GeprueftFehlerhaft,
        Self::Gebucht,
        Self::Bezahlt,
    ];
    /// Number of schema-defined variants (equal to `VARIANTS.len()`), excluding the
    /// [`Rechnungsstatus::Unknown`] catch-all.  Stable for this schema version.
    pub const COUNT: usize = Self::VARIANTS.len();
    /// Returns an iterator over all **known** variants of `Rechnungsstatus`.
    ///
    /// Yields only variants that correspond to values defined in the BO4E schema
    /// (i.e. [`Self::VARIANTS`]), never the [`Rechnungsstatus::Unknown`] catch-all.
    /// Available **without** the `strum` feature.
    ///
    /// # Example
    /// ```
    /// # use rubo4e::current::Rechnungsstatus;
    /// // Never yields the `Unknown` catch-all, so the count matches `COUNT`.
    /// assert_eq!(Rechnungsstatus::iter_known().count(), Rechnungsstatus::COUNT);
    /// assert!(Rechnungsstatus::iter_known().all(|v| v.is_known()));
    /// ```
    pub fn iter_known() -> impl Iterator<Item = Self> + Clone {
        Self::VARIANTS.iter().copied()
    }
    /// Returns the canonical BO4E wire string (SCREAMING_SNAKE_CASE) for this value.
    ///
    /// [`Rechnungsstatus::Unknown`] renders as `"UNKNOWN"`, matching its serialized form.
    pub const fn as_wire(&self) -> &'static str {
        match self {
            Self::Ungeprueft => "UNGEPRUEFT",
            Self::GeprueftOk => "GEPRUEFT_OK",
            Self::GeprueftFehlerhaft => "GEPRUEFT_FEHLERHAFT",
            Self::Gebucht => "GEBUCHT",
            Self::Bezahlt => "BEZAHLT",
            Self::Unknown => "UNKNOWN",
        }
    }
    /// **Strictly** parses a BO4E wire string into a known variant.
    ///
    /// Unlike the lenient `serde` / [`FromStr`](std::str::FromStr) path — which maps
    /// any unrecognized value (a typo, a legacy code, or a value from a newer schema)
    /// to [`Rechnungsstatus::Unknown`] — this returns
    /// [`Err`](crate::error::UnknownVariant) for values not defined in this schema
    /// version, including the literal `"UNKNOWN"`.  Use it at the ingest boundary to
    /// reject bad values instead of silently degrading them.
    ///
    /// # Example
    /// ```
    /// # use rubo4e::current::Rechnungsstatus;
    /// assert_eq!(Rechnungsstatus::from_wire("UNGEPRUEFT"), Ok(Rechnungsstatus::Ungeprueft));
    /// // Out-of-schema values are rejected rather than degraded:
    /// assert!(Rechnungsstatus::from_wire("NOT_A_REAL_VALUE").is_err());
    /// // …including the `Unknown` catch-all's own wire spelling:
    /// assert!(Rechnungsstatus::from_wire("UNKNOWN").is_err());
    /// ```
    pub fn from_wire(s: &str) -> Result<Self, crate::error::UnknownVariant> {
        match s {
            "UNGEPRUEFT" => Ok(Self::Ungeprueft),
            "GEPRUEFT_OK" => Ok(Self::GeprueftOk),
            "GEPRUEFT_FEHLERHAFT" => Ok(Self::GeprueftFehlerhaft),
            "GEBUCHT" => Ok(Self::Gebucht),
            "BEZAHLT" => Ok(Self::Bezahlt),
            other => Err(crate::error::UnknownVariant::new(other)),
        }
    }
    /// Returns `true` if this value is the forward-compatibility
    /// [`Rechnungsstatus::Unknown`] catch-all (an out-of-schema value).
    pub const fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
    /// Returns `true` if this value is a known, schema-defined variant.
    pub const fn is_known(&self) -> bool {
        !self.is_unknown()
    }
}
impl std::fmt::Display for Rechnungsstatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_wire())
    }
}
impl AsRef<str> for Rechnungsstatus {
    fn as_ref(&self) -> &str {
        self.as_wire()
    }
}
#[cfg(feature = "versioned")]
impl crate::bo4e_enum_sealed::Sealed for Rechnungsstatus {}
#[cfg(feature = "versioned")]
impl crate::Bo4eEnum for Rechnungsstatus {
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
impl crate::Bo4eStrict for Rechnungsstatus {
    fn collect_unknown_enums(&self, path: &str, out: &mut Vec<String>) {
        if self.is_unknown() {
            out.push(path.to_owned());
        }
    }
}
#[cfg(feature = "sqlx")]
impl sqlx::Type<sqlx::Postgres> for Rechnungsstatus {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
/// Encodes as the canonical BO4E wire string, borrowed from `as_wire` — no
/// intermediate `String` or `serde_json::Value` is allocated.
#[cfg(feature = "sqlx")]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Rechnungsstatus {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        let s: &str = self.as_wire();
        <&str as sqlx::Encode<'q, sqlx::Postgres>>::encode_by_ref(&s, buf)
    }
}
/// Decodes leniently, matching the `serde` path: a value the schema does not
/// define becomes [`Rechnungsstatus::Unknown`] rather than a decode error, so a
/// database row written by a newer schema version still reads back.
///
/// Use [`Rechnungsstatus::from_wire`] on a `String` column, or check
/// [`Rechnungsstatus::is_known`], where out-of-schema values must be rejected.
#[cfg(feature = "sqlx")]
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for Rechnungsstatus {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <&str as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        Ok(Self::from_wire(s).unwrap_or(Self::Unknown))
    }
}
/// Lets `Vec<Rechnungsstatus>` bind to a `TEXT[]` column.  Only this crate can
/// provide it: the trait and the enum are both foreign to any consumer, so the
/// orphan rule rules out a downstream impl.
#[cfg(feature = "sqlx")]
impl sqlx::postgres::PgHasArrayType for Rechnungsstatus {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::postgres::PgHasArrayType>::array_type_info()
    }
}
#[cfg(test)]
impl proptest::arbitrary::Arbitrary for Rechnungsstatus {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        proptest::sample::select(Self::VARIANTS.to_vec()).boxed()
    }
}
