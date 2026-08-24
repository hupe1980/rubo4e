#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(strum::EnumString, strum::EnumIter, strum::IntoStaticStr)
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Klassifiziert einen Tagtyp, an dem ein wiederkehrendes Schaltschema (z.B. die Umschaltzeiten einer
/// `Zaehlzeitdefinition`) gilt. Die Werte teilen sich in drei Gruppen auf:
///
/// * `TAEGLICH` – gilt an jedem Tag des Jahres.
/// * Gruppenbezeichnungen (`WERKTAGS`, `WOCHENENDE`, `FEIERTAGS`) – fassen mehrere Wochentage zusammen.
///   Was als Feiertag zählt, wird durch den Feiertagskalender der `Zaehlzeitdefinition` bestimmt.
/// * Einzelne Wochentage (`MONTAGS` … `SONNTAGS`).
#[non_exhaustive]
pub enum Wiederholungstyp {
    #[cfg_attr(feature = "serde", serde(rename = "TAEGLICH"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TAEGLICH"))]
    Taeglich,
    #[cfg_attr(feature = "serde", serde(rename = "WERKTAGS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "WERKTAGS"))]
    Werktags,
    #[cfg_attr(feature = "serde", serde(rename = "WOCHENENDE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "WOCHENENDE"))]
    Wochenende,
    #[cfg_attr(feature = "serde", serde(rename = "FEIERTAGS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "FEIERTAGS"))]
    Feiertags,
    #[cfg_attr(feature = "serde", serde(rename = "MONTAGS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MONTAGS"))]
    Montags,
    #[cfg_attr(feature = "serde", serde(rename = "DIENSTAGS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "DIENSTAGS"))]
    Dienstags,
    #[cfg_attr(feature = "serde", serde(rename = "MITTWOCHS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MITTWOCHS"))]
    Mittwochs,
    #[cfg_attr(feature = "serde", serde(rename = "DONNERSTAGS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "DONNERSTAGS"))]
    Donnerstags,
    #[cfg_attr(feature = "serde", serde(rename = "FREITAGS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "FREITAGS"))]
    Freitags,
    #[cfg_attr(feature = "serde", serde(rename = "SAMSTAGS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SAMSTAGS"))]
    Samstags,
    #[cfg_attr(feature = "serde", serde(rename = "SONNTAGS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SONNTAGS"))]
    Sonntags,
    /// Unknown or future variant — produced when deserializing a value
    /// that is not yet known to this version of the library.
    #[cfg_attr(feature = "serde", serde(other, rename = "UNKNOWN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UNKNOWN"))]
    Unknown,
}
impl Wiederholungstyp {
    /// All variants defined by the BO4E schema, in declaration order.
    ///
    /// Excludes the forward-compatibility [`Wiederholungstyp::Unknown`] catch-all, so this
    /// is exactly the set of values that appear on the wire.  Available **without**
    /// the `strum` feature — use it to drift-guard SQL `CHECK` lists and mappings.
    pub const VARIANTS: &'static [Self] = &[
        Self::Taeglich,
        Self::Werktags,
        Self::Wochenende,
        Self::Feiertags,
        Self::Montags,
        Self::Dienstags,
        Self::Mittwochs,
        Self::Donnerstags,
        Self::Freitags,
        Self::Samstags,
        Self::Sonntags,
    ];
    /// Number of schema-defined variants (equal to `VARIANTS.len()`), excluding the
    /// [`Wiederholungstyp::Unknown`] catch-all.  Stable for this schema version.
    pub const COUNT: usize = Self::VARIANTS.len();
    /// Returns an iterator over all **known** variants of `Wiederholungstyp`.
    ///
    /// Yields only variants that correspond to values defined in the BO4E schema
    /// (i.e. [`Self::VARIANTS`]), never the [`Wiederholungstyp::Unknown`] catch-all.
    /// Available **without** the `strum` feature.
    ///
    /// # Example
    /// ```
    /// # use rubo4e::current::Wiederholungstyp;
    /// // Never yields the `Unknown` catch-all, so the count matches `COUNT`.
    /// assert_eq!(Wiederholungstyp::iter_known().count(), Wiederholungstyp::COUNT);
    /// assert!(Wiederholungstyp::iter_known().all(|v| v.is_known()));
    /// ```
    pub fn iter_known() -> impl Iterator<Item = Self> + Clone {
        Self::VARIANTS.iter().copied()
    }
    /// Returns the canonical BO4E wire string (SCREAMING_SNAKE_CASE) for this value.
    ///
    /// [`Wiederholungstyp::Unknown`] renders as `"UNKNOWN"`, matching its serialized form.
    pub const fn as_wire(&self) -> &'static str {
        match self {
            Self::Taeglich => "TAEGLICH",
            Self::Werktags => "WERKTAGS",
            Self::Wochenende => "WOCHENENDE",
            Self::Feiertags => "FEIERTAGS",
            Self::Montags => "MONTAGS",
            Self::Dienstags => "DIENSTAGS",
            Self::Mittwochs => "MITTWOCHS",
            Self::Donnerstags => "DONNERSTAGS",
            Self::Freitags => "FREITAGS",
            Self::Samstags => "SAMSTAGS",
            Self::Sonntags => "SONNTAGS",
            Self::Unknown => "UNKNOWN",
        }
    }
    /// **Strictly** parses a BO4E wire string into a known variant.
    ///
    /// Unlike the lenient `serde` / [`FromStr`](std::str::FromStr) path — which maps
    /// any unrecognized value (a typo, a legacy code, or a value from a newer schema)
    /// to [`Wiederholungstyp::Unknown`] — this returns
    /// [`Err`](crate::error::UnknownVariant) for values not defined in this schema
    /// version, including the literal `"UNKNOWN"`.  Use it at the ingest boundary to
    /// reject bad values instead of silently degrading them.
    ///
    /// # Example
    /// ```
    /// # use rubo4e::current::Wiederholungstyp;
    /// assert_eq!(Wiederholungstyp::from_wire("TAEGLICH"), Ok(Wiederholungstyp::Taeglich));
    /// // Out-of-schema values are rejected rather than degraded:
    /// assert!(Wiederholungstyp::from_wire("NOT_A_REAL_VALUE").is_err());
    /// // …including the `Unknown` catch-all's own wire spelling:
    /// assert!(Wiederholungstyp::from_wire("UNKNOWN").is_err());
    /// ```
    pub fn from_wire(s: &str) -> Result<Self, crate::error::UnknownVariant> {
        match s {
            "TAEGLICH" => Ok(Self::Taeglich),
            "WERKTAGS" => Ok(Self::Werktags),
            "WOCHENENDE" => Ok(Self::Wochenende),
            "FEIERTAGS" => Ok(Self::Feiertags),
            "MONTAGS" => Ok(Self::Montags),
            "DIENSTAGS" => Ok(Self::Dienstags),
            "MITTWOCHS" => Ok(Self::Mittwochs),
            "DONNERSTAGS" => Ok(Self::Donnerstags),
            "FREITAGS" => Ok(Self::Freitags),
            "SAMSTAGS" => Ok(Self::Samstags),
            "SONNTAGS" => Ok(Self::Sonntags),
            other => Err(crate::error::UnknownVariant::new(other)),
        }
    }
    /// Returns `true` if this value is the forward-compatibility
    /// [`Wiederholungstyp::Unknown`] catch-all (an out-of-schema value).
    pub const fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
    /// Returns `true` if this value is a known, schema-defined variant.
    pub const fn is_known(&self) -> bool {
        !self.is_unknown()
    }
}
impl std::fmt::Display for Wiederholungstyp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_wire())
    }
}
impl AsRef<str> for Wiederholungstyp {
    fn as_ref(&self) -> &str {
        self.as_wire()
    }
}
#[cfg(feature = "versioned")]
impl crate::bo4e_enum_sealed::Sealed for Wiederholungstyp {}
#[cfg(feature = "versioned")]
impl crate::Bo4eEnum for Wiederholungstyp {
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
impl crate::Bo4eStrict for Wiederholungstyp {
    fn collect_unknown_enums(&self, path: &str, out: &mut Vec<String>) {
        if self.is_unknown() {
            out.push(path.to_owned());
        }
    }
}
#[cfg(feature = "sqlx")]
impl sqlx::Type<sqlx::Postgres> for Wiederholungstyp {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
/// Encodes as the canonical BO4E wire string, borrowed from `as_wire` — no
/// intermediate `String` or `serde_json::Value` is allocated.
#[cfg(feature = "sqlx")]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Wiederholungstyp {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        let s: &str = self.as_wire();
        <&str as sqlx::Encode<'q, sqlx::Postgres>>::encode_by_ref(&s, buf)
    }
}
/// Decodes leniently, matching the `serde` path: a value the schema does not
/// define becomes [`Wiederholungstyp::Unknown`] rather than a decode error, so a
/// database row written by a newer schema version still reads back.
///
/// Use [`Wiederholungstyp::from_wire`] on a `String` column, or check
/// [`Wiederholungstyp::is_known`], where out-of-schema values must be rejected.
#[cfg(feature = "sqlx")]
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for Wiederholungstyp {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <&str as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        Ok(Self::from_wire(s).unwrap_or(Self::Unknown))
    }
}
/// Lets `Vec<Wiederholungstyp>` bind to a `TEXT[]` column.  Only this crate can
/// provide it: the trait and the enum are both foreign to any consumer, so the
/// orphan rule rules out a downstream impl.
#[cfg(feature = "sqlx")]
impl sqlx::postgres::PgHasArrayType for Wiederholungstyp {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::postgres::PgHasArrayType>::array_type_info()
    }
}
#[cfg(test)]
impl proptest::arbitrary::Arbitrary for Wiederholungstyp {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        proptest::sample::select(Self::VARIANTS.to_vec()).boxed()
    }
}
