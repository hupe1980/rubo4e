#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(strum::EnumString, strum::EnumIter, strum::IntoStaticStr)
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Abbildung verschiedener Rechnungstypen zur Kennzeichnung von Rechnungen
///
/// # Correction / reversal invoices
///
/// BO4E does not model a Korrektur/Storno value in this enum.  The sanctioned
/// representation is a process label carried as a `ZusatzAttribut` on the
/// `Rechnung` (e.g. `rechnungsart = "KORREKTURRECHNUNG"`) rather than a
/// dedicated `Rechnungstyp` variant.  This is an upstream BO4E modelling gap;
/// if a future schema introduces a correction value it will surface here.
#[non_exhaustive]
pub enum Rechnungstyp {
    #[cfg_attr(feature = "serde", serde(rename = "ENDKUNDENRECHNUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ENDKUNDENRECHNUNG"))]
    Endkundenrechnung,
    #[cfg_attr(feature = "serde", serde(rename = "NETZNUTZUNGSRECHNUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "NETZNUTZUNGSRECHNUNG"))]
    Netznutzungsrechnung,
    #[cfg_attr(feature = "serde", serde(rename = "MEHRMINDERMENGENRECHNUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MEHRMINDERMENGENRECHNUNG"))]
    Mehrmindermengenrechnung,
    #[cfg_attr(feature = "serde", serde(rename = "MESSSTELLENBETRIEBSRECHNUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MESSSTELLENBETRIEBSRECHNUNG"))]
    Messstellenbetriebsrechnung,
    #[cfg_attr(feature = "serde", serde(rename = "BESCHAFFUNGSRECHNUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BESCHAFFUNGSRECHNUNG"))]
    Beschaffungsrechnung,
    #[cfg_attr(feature = "serde", serde(rename = "AUSGLEICHSENERGIERECHNUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "AUSGLEICHSENERGIERECHNUNG"))]
    Ausgleichsenergierechnung,
    #[cfg_attr(feature = "serde", serde(rename = "ABSCHLUSSRECHNUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ABSCHLUSSRECHNUNG"))]
    Abschlussrechnung,
    #[cfg_attr(feature = "serde", serde(rename = "ABSCHLAGSRECHNUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ABSCHLAGSRECHNUNG"))]
    Abschlagsrechnung,
    #[cfg_attr(feature = "serde", serde(rename = "TURNUSRECHNUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TURNUSRECHNUNG"))]
    Turnusrechnung,
    #[cfg_attr(feature = "serde", serde(rename = "MONATSRECHNUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MONATSRECHNUNG"))]
    Monatsrechnung,
    #[cfg_attr(feature = "serde", serde(rename = "ZWISCHENRECHNUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ZWISCHENRECHNUNG"))]
    Zwischenrechnung,
    #[cfg_attr(feature = "serde", serde(rename = "INTEGRIERTE_13TE_RECHNUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "INTEGRIERTE_13TE_RECHNUNG"))]
    Integrierte13TeRechnung,
    #[cfg_attr(feature = "serde", serde(rename = "ZUSAETZLICHE_13TE_RECHNUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ZUSAETZLICHE_13TE_RECHNUNG"))]
    Zusaetzliche13TeRechnung,
    /// Unknown or future variant — produced when deserializing a value
    /// that is not yet known to this version of the library.
    #[cfg_attr(feature = "serde", serde(other, rename = "UNKNOWN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UNKNOWN"))]
    Unknown,
}
impl Rechnungstyp {
    /// All variants defined by the BO4E schema, in declaration order.
    ///
    /// Excludes the forward-compatibility [`Rechnungstyp::Unknown`] catch-all, so this
    /// is exactly the set of values that appear on the wire.  Available **without**
    /// the `strum` feature — use it to drift-guard SQL `CHECK` lists and mappings.
    pub const VARIANTS: &'static [Self] = &[
        Self::Endkundenrechnung,
        Self::Netznutzungsrechnung,
        Self::Mehrmindermengenrechnung,
        Self::Messstellenbetriebsrechnung,
        Self::Beschaffungsrechnung,
        Self::Ausgleichsenergierechnung,
        Self::Abschlussrechnung,
        Self::Abschlagsrechnung,
        Self::Turnusrechnung,
        Self::Monatsrechnung,
        Self::Zwischenrechnung,
        Self::Integrierte13TeRechnung,
        Self::Zusaetzliche13TeRechnung,
    ];
    /// Number of schema-defined variants (equal to `VARIANTS.len()`), excluding the
    /// [`Rechnungstyp::Unknown`] catch-all.  Stable for this schema version.
    pub const COUNT: usize = Self::VARIANTS.len();
    /// Returns an iterator over all **known** variants of `Rechnungstyp`.
    ///
    /// Yields only variants that correspond to values defined in the BO4E schema
    /// (i.e. [`Self::VARIANTS`]), never the [`Rechnungstyp::Unknown`] catch-all.
    /// Available **without** the `strum` feature.
    ///
    /// # Example
    /// ```
    /// # use rubo4e::current::Rechnungstyp;
    /// // Never yields the `Unknown` catch-all, so the count matches `COUNT`.
    /// assert_eq!(Rechnungstyp::iter_known().count(), Rechnungstyp::COUNT);
    /// assert!(Rechnungstyp::iter_known().all(|v| v.is_known()));
    /// ```
    pub fn iter_known() -> impl Iterator<Item = Self> + Clone {
        Self::VARIANTS.iter().copied()
    }
    /// Returns the canonical BO4E wire string (SCREAMING_SNAKE_CASE) for this value.
    ///
    /// [`Rechnungstyp::Unknown`] renders as `"UNKNOWN"`, matching its serialized form.
    pub const fn as_wire(&self) -> &'static str {
        match self {
            Self::Endkundenrechnung => "ENDKUNDENRECHNUNG",
            Self::Netznutzungsrechnung => "NETZNUTZUNGSRECHNUNG",
            Self::Mehrmindermengenrechnung => "MEHRMINDERMENGENRECHNUNG",
            Self::Messstellenbetriebsrechnung => "MESSSTELLENBETRIEBSRECHNUNG",
            Self::Beschaffungsrechnung => "BESCHAFFUNGSRECHNUNG",
            Self::Ausgleichsenergierechnung => "AUSGLEICHSENERGIERECHNUNG",
            Self::Abschlussrechnung => "ABSCHLUSSRECHNUNG",
            Self::Abschlagsrechnung => "ABSCHLAGSRECHNUNG",
            Self::Turnusrechnung => "TURNUSRECHNUNG",
            Self::Monatsrechnung => "MONATSRECHNUNG",
            Self::Zwischenrechnung => "ZWISCHENRECHNUNG",
            Self::Integrierte13TeRechnung => "INTEGRIERTE_13TE_RECHNUNG",
            Self::Zusaetzliche13TeRechnung => "ZUSAETZLICHE_13TE_RECHNUNG",
            Self::Unknown => "UNKNOWN",
        }
    }
    /// **Strictly** parses a BO4E wire string into a known variant.
    ///
    /// Unlike the lenient `serde` / [`FromStr`](std::str::FromStr) path — which maps
    /// any unrecognized value (a typo, a legacy code, or a value from a newer schema)
    /// to [`Rechnungstyp::Unknown`] — this returns
    /// [`Err`](crate::error::UnknownVariant) for values not defined in this schema
    /// version, including the literal `"UNKNOWN"`.  Use it at the ingest boundary to
    /// reject bad values instead of silently degrading them.
    ///
    /// # Example
    /// ```
    /// # use rubo4e::current::Rechnungstyp;
    /// assert_eq!(Rechnungstyp::from_wire("ENDKUNDENRECHNUNG"), Ok(Rechnungstyp::Endkundenrechnung));
    /// // Out-of-schema values are rejected rather than degraded:
    /// assert!(Rechnungstyp::from_wire("NOT_A_REAL_VALUE").is_err());
    /// // …including the `Unknown` catch-all's own wire spelling:
    /// assert!(Rechnungstyp::from_wire("UNKNOWN").is_err());
    /// ```
    pub fn from_wire(s: &str) -> Result<Self, crate::error::UnknownVariant> {
        match s {
            "ENDKUNDENRECHNUNG" => Ok(Self::Endkundenrechnung),
            "NETZNUTZUNGSRECHNUNG" => Ok(Self::Netznutzungsrechnung),
            "MEHRMINDERMENGENRECHNUNG" => Ok(Self::Mehrmindermengenrechnung),
            "MESSSTELLENBETRIEBSRECHNUNG" => Ok(Self::Messstellenbetriebsrechnung),
            "BESCHAFFUNGSRECHNUNG" => Ok(Self::Beschaffungsrechnung),
            "AUSGLEICHSENERGIERECHNUNG" => Ok(Self::Ausgleichsenergierechnung),
            "ABSCHLUSSRECHNUNG" => Ok(Self::Abschlussrechnung),
            "ABSCHLAGSRECHNUNG" => Ok(Self::Abschlagsrechnung),
            "TURNUSRECHNUNG" => Ok(Self::Turnusrechnung),
            "MONATSRECHNUNG" => Ok(Self::Monatsrechnung),
            "ZWISCHENRECHNUNG" => Ok(Self::Zwischenrechnung),
            "INTEGRIERTE_13TE_RECHNUNG" => Ok(Self::Integrierte13TeRechnung),
            "ZUSAETZLICHE_13TE_RECHNUNG" => Ok(Self::Zusaetzliche13TeRechnung),
            other => Err(crate::error::UnknownVariant::new(other)),
        }
    }
    /// Returns `true` if this value is the forward-compatibility
    /// [`Rechnungstyp::Unknown`] catch-all (an out-of-schema value).
    pub const fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
    /// Returns `true` if this value is a known, schema-defined variant.
    pub const fn is_known(&self) -> bool {
        !self.is_unknown()
    }
}
impl std::fmt::Display for Rechnungstyp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_wire())
    }
}
impl AsRef<str> for Rechnungstyp {
    fn as_ref(&self) -> &str {
        self.as_wire()
    }
}
#[cfg(feature = "versioned")]
impl crate::bo4e_enum_sealed::Sealed for Rechnungstyp {}
#[cfg(feature = "versioned")]
impl crate::Bo4eEnum for Rechnungstyp {
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
impl crate::Bo4eStrict for Rechnungstyp {
    fn collect_unknown_enums(&self, path: &str, out: &mut Vec<String>) {
        if self.is_unknown() {
            out.push(path.to_owned());
        }
    }
}
#[cfg(feature = "sqlx")]
impl sqlx::Type<sqlx::Postgres> for Rechnungstyp {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
/// Encodes as the canonical BO4E wire string, borrowed from `as_wire` — no
/// intermediate `String` or `serde_json::Value` is allocated.
#[cfg(feature = "sqlx")]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Rechnungstyp {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        let s: &str = self.as_wire();
        <&str as sqlx::Encode<'q, sqlx::Postgres>>::encode_by_ref(&s, buf)
    }
}
/// Decodes leniently, matching the `serde` path: a value the schema does not
/// define becomes [`Rechnungstyp::Unknown`] rather than a decode error, so a
/// database row written by a newer schema version still reads back.
///
/// Use [`Rechnungstyp::from_wire`] on a `String` column, or check
/// [`Rechnungstyp::is_known`], where out-of-schema values must be rejected.
#[cfg(feature = "sqlx")]
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for Rechnungstyp {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <&str as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        Ok(Self::from_wire(s).unwrap_or(Self::Unknown))
    }
}
/// Lets `Vec<Rechnungstyp>` bind to a `TEXT[]` column.  Only this crate can
/// provide it: the trait and the enum are both foreign to any consumer, so the
/// orphan rule rules out a downstream impl.
#[cfg(feature = "sqlx")]
impl sqlx::postgres::PgHasArrayType for Rechnungstyp {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::postgres::PgHasArrayType>::array_type_info()
    }
}
#[cfg(test)]
impl proptest::arbitrary::Arbitrary for Rechnungstyp {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        proptest::sample::select(Self::VARIANTS.to_vec()).boxed()
    }
}
