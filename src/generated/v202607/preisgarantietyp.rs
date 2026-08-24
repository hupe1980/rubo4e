#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(strum::EnumString, strum::EnumIter, strum::IntoStaticStr)
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Aufzählung der Möglichkeiten für die Vergabe von Preisgarantien
#[non_exhaustive]
pub enum Preisgarantietyp {
    #[cfg_attr(feature = "serde", serde(rename = "ALLE_PREISBESTANDTEILE_BRUTTO"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ALLE_PREISBESTANDTEILE_BRUTTO"))]
    AllePreisbestandteileBrutto,
    #[cfg_attr(feature = "serde", serde(rename = "ALLE_PREISBESTANDTEILE_NETTO"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ALLE_PREISBESTANDTEILE_NETTO"))]
    AllePreisbestandteileNetto,
    #[cfg_attr(feature = "serde", serde(rename = "PREISBESTANDTEILE_OHNE_ABGABEN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "PREISBESTANDTEILE_OHNE_ABGABEN"))]
    PreisbestandteileOhneAbgaben,
    #[cfg_attr(feature = "serde", serde(rename = "NUR_ENERGIEPREIS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "NUR_ENERGIEPREIS"))]
    NurEnergiepreis,
    /// Unknown or future variant — produced when deserializing a value
    /// that is not yet known to this version of the library.
    #[cfg_attr(feature = "serde", serde(other, rename = "UNKNOWN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UNKNOWN"))]
    Unknown,
}
impl Preisgarantietyp {
    /// All variants defined by the BO4E schema, in declaration order.
    ///
    /// Excludes the forward-compatibility [`Preisgarantietyp::Unknown`] catch-all, so this
    /// is exactly the set of values that appear on the wire.  Available **without**
    /// the `strum` feature — use it to drift-guard SQL `CHECK` lists and mappings.
    pub const VARIANTS: &'static [Self] = &[
        Self::AllePreisbestandteileBrutto,
        Self::AllePreisbestandteileNetto,
        Self::PreisbestandteileOhneAbgaben,
        Self::NurEnergiepreis,
    ];
    /// Number of schema-defined variants (equal to `VARIANTS.len()`), excluding the
    /// [`Preisgarantietyp::Unknown`] catch-all.  Stable for this schema version.
    pub const COUNT: usize = Self::VARIANTS.len();
    /// Returns an iterator over all **known** variants of `Preisgarantietyp`.
    ///
    /// Yields only variants that correspond to values defined in the BO4E schema
    /// (i.e. [`Self::VARIANTS`]), never the [`Preisgarantietyp::Unknown`] catch-all.
    /// Available **without** the `strum` feature.
    ///
    /// # Example
    /// ```
    /// # use rubo4e::current::Preisgarantietyp;
    /// // Never yields the `Unknown` catch-all, so the count matches `COUNT`.
    /// assert_eq!(Preisgarantietyp::iter_known().count(), Preisgarantietyp::COUNT);
    /// assert!(Preisgarantietyp::iter_known().all(|v| v.is_known()));
    /// ```
    pub fn iter_known() -> impl Iterator<Item = Self> + Clone {
        Self::VARIANTS.iter().copied()
    }
    /// Returns the canonical BO4E wire string (SCREAMING_SNAKE_CASE) for this value.
    ///
    /// [`Preisgarantietyp::Unknown`] renders as `"UNKNOWN"`, matching its serialized form.
    pub const fn as_wire(&self) -> &'static str {
        match self {
            Self::AllePreisbestandteileBrutto => "ALLE_PREISBESTANDTEILE_BRUTTO",
            Self::AllePreisbestandteileNetto => "ALLE_PREISBESTANDTEILE_NETTO",
            Self::PreisbestandteileOhneAbgaben => "PREISBESTANDTEILE_OHNE_ABGABEN",
            Self::NurEnergiepreis => "NUR_ENERGIEPREIS",
            Self::Unknown => "UNKNOWN",
        }
    }
    /// **Strictly** parses a BO4E wire string into a known variant.
    ///
    /// Unlike the lenient `serde` / [`FromStr`](std::str::FromStr) path — which maps
    /// any unrecognized value (a typo, a legacy code, or a value from a newer schema)
    /// to [`Preisgarantietyp::Unknown`] — this returns
    /// [`Err`](crate::error::UnknownVariant) for values not defined in this schema
    /// version, including the literal `"UNKNOWN"`.  Use it at the ingest boundary to
    /// reject bad values instead of silently degrading them.
    ///
    /// # Example
    /// ```
    /// # use rubo4e::current::Preisgarantietyp;
    /// assert_eq!(Preisgarantietyp::from_wire("ALLE_PREISBESTANDTEILE_BRUTTO"), Ok(Preisgarantietyp::AllePreisbestandteileBrutto));
    /// // Out-of-schema values are rejected rather than degraded:
    /// assert!(Preisgarantietyp::from_wire("NOT_A_REAL_VALUE").is_err());
    /// // …including the `Unknown` catch-all's own wire spelling:
    /// assert!(Preisgarantietyp::from_wire("UNKNOWN").is_err());
    /// ```
    pub fn from_wire(s: &str) -> Result<Self, crate::error::UnknownVariant> {
        match s {
            "ALLE_PREISBESTANDTEILE_BRUTTO" => Ok(Self::AllePreisbestandteileBrutto),
            "ALLE_PREISBESTANDTEILE_NETTO" => Ok(Self::AllePreisbestandteileNetto),
            "PREISBESTANDTEILE_OHNE_ABGABEN" => Ok(Self::PreisbestandteileOhneAbgaben),
            "NUR_ENERGIEPREIS" => Ok(Self::NurEnergiepreis),
            other => Err(crate::error::UnknownVariant::new(other)),
        }
    }
    /// Returns `true` if this value is the forward-compatibility
    /// [`Preisgarantietyp::Unknown`] catch-all (an out-of-schema value).
    pub const fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
    /// Returns `true` if this value is a known, schema-defined variant.
    pub const fn is_known(&self) -> bool {
        !self.is_unknown()
    }
}
impl std::fmt::Display for Preisgarantietyp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_wire())
    }
}
impl AsRef<str> for Preisgarantietyp {
    fn as_ref(&self) -> &str {
        self.as_wire()
    }
}
#[cfg(feature = "versioned")]
impl crate::bo4e_enum_sealed::Sealed for Preisgarantietyp {}
#[cfg(feature = "versioned")]
impl crate::Bo4eEnum for Preisgarantietyp {
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
impl crate::Bo4eStrict for Preisgarantietyp {
    fn collect_unknown_enums(&self, path: &str, out: &mut Vec<String>) {
        if self.is_unknown() {
            out.push(path.to_owned());
        }
    }
}
#[cfg(feature = "sqlx")]
impl sqlx::Type<sqlx::Postgres> for Preisgarantietyp {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
/// Encodes as the canonical BO4E wire string, borrowed from `as_wire` — no
/// intermediate `String` or `serde_json::Value` is allocated.
#[cfg(feature = "sqlx")]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Preisgarantietyp {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        let s: &str = self.as_wire();
        <&str as sqlx::Encode<'q, sqlx::Postgres>>::encode_by_ref(&s, buf)
    }
}
/// Decodes leniently, matching the `serde` path: a value the schema does not
/// define becomes [`Preisgarantietyp::Unknown`] rather than a decode error, so a
/// database row written by a newer schema version still reads back.
///
/// Use [`Preisgarantietyp::from_wire`] on a `String` column, or check
/// [`Preisgarantietyp::is_known`], where out-of-schema values must be rejected.
#[cfg(feature = "sqlx")]
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for Preisgarantietyp {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <&str as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        Ok(Self::from_wire(s).unwrap_or(Self::Unknown))
    }
}
/// Lets `Vec<Preisgarantietyp>` bind to a `TEXT[]` column.  Only this crate can
/// provide it: the trait and the enum are both foreign to any consumer, so the
/// orphan rule rules out a downstream impl.
#[cfg(feature = "sqlx")]
impl sqlx::postgres::PgHasArrayType for Preisgarantietyp {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::postgres::PgHasArrayType>::array_type_info()
    }
}
#[cfg(test)]
impl proptest::arbitrary::Arbitrary for Preisgarantietyp {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        proptest::sample::select(Self::VARIANTS.to_vec()).boxed()
    }
}
