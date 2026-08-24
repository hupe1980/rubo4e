#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(strum::EnumString, strum::EnumIter, strum::IntoStaticStr)
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Abbildung einer Statusinformation für Verträge.
#[non_exhaustive]
pub enum Vertragsstatus {
    #[cfg_attr(feature = "serde", serde(rename = "IN_ARBEIT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "IN_ARBEIT"))]
    InArbeit,
    #[cfg_attr(feature = "serde", serde(rename = "UEBERMITTELT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UEBERMITTELT"))]
    Uebermittelt,
    #[cfg_attr(feature = "serde", serde(rename = "ANGENOMMEN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ANGENOMMEN"))]
    Angenommen,
    #[cfg_attr(feature = "serde", serde(rename = "AKTIV"))]
    #[cfg_attr(feature = "strum", strum(serialize = "AKTIV"))]
    Aktiv,
    #[cfg_attr(feature = "serde", serde(rename = "ABGELEHNT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ABGELEHNT"))]
    Abgelehnt,
    #[cfg_attr(feature = "serde", serde(rename = "WIDERRUFEN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "WIDERRUFEN"))]
    Widerrufen,
    #[cfg_attr(feature = "serde", serde(rename = "STORNIERT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "STORNIERT"))]
    Storniert,
    #[cfg_attr(feature = "serde", serde(rename = "GEKUENDIGT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GEKUENDIGT"))]
    Gekuendigt,
    #[cfg_attr(feature = "serde", serde(rename = "BEENDET"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BEENDET"))]
    Beendet,
    /// Unknown or future variant — produced when deserializing a value
    /// that is not yet known to this version of the library.
    #[cfg_attr(feature = "serde", serde(other, rename = "UNKNOWN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UNKNOWN"))]
    Unknown,
}
impl Vertragsstatus {
    /// All variants defined by the BO4E schema, in declaration order.
    ///
    /// Excludes the forward-compatibility [`Vertragsstatus::Unknown`] catch-all, so this
    /// is exactly the set of values that appear on the wire.  Available **without**
    /// the `strum` feature — use it to drift-guard SQL `CHECK` lists and mappings.
    pub const VARIANTS: &'static [Self] = &[
        Self::InArbeit,
        Self::Uebermittelt,
        Self::Angenommen,
        Self::Aktiv,
        Self::Abgelehnt,
        Self::Widerrufen,
        Self::Storniert,
        Self::Gekuendigt,
        Self::Beendet,
    ];
    /// Number of schema-defined variants (equal to `VARIANTS.len()`), excluding the
    /// [`Vertragsstatus::Unknown`] catch-all.  Stable for this schema version.
    pub const COUNT: usize = Self::VARIANTS.len();
    /// Returns an iterator over all **known** variants of `Vertragsstatus`.
    ///
    /// Yields only variants that correspond to values defined in the BO4E schema
    /// (i.e. [`Self::VARIANTS`]), never the [`Vertragsstatus::Unknown`] catch-all.
    /// Available **without** the `strum` feature.
    ///
    /// # Example
    /// ```
    /// # use rubo4e::current::Vertragsstatus;
    /// // Never yields the `Unknown` catch-all, so the count matches `COUNT`.
    /// assert_eq!(Vertragsstatus::iter_known().count(), Vertragsstatus::COUNT);
    /// assert!(Vertragsstatus::iter_known().all(|v| v.is_known()));
    /// ```
    pub fn iter_known() -> impl Iterator<Item = Self> + Clone {
        Self::VARIANTS.iter().copied()
    }
    /// Returns the canonical BO4E wire string (SCREAMING_SNAKE_CASE) for this value.
    ///
    /// [`Vertragsstatus::Unknown`] renders as `"UNKNOWN"`, matching its serialized form.
    pub const fn as_wire(&self) -> &'static str {
        match self {
            Self::InArbeit => "IN_ARBEIT",
            Self::Uebermittelt => "UEBERMITTELT",
            Self::Angenommen => "ANGENOMMEN",
            Self::Aktiv => "AKTIV",
            Self::Abgelehnt => "ABGELEHNT",
            Self::Widerrufen => "WIDERRUFEN",
            Self::Storniert => "STORNIERT",
            Self::Gekuendigt => "GEKUENDIGT",
            Self::Beendet => "BEENDET",
            Self::Unknown => "UNKNOWN",
        }
    }
    /// **Strictly** parses a BO4E wire string into a known variant.
    ///
    /// Unlike the lenient `serde` / [`FromStr`](std::str::FromStr) path — which maps
    /// any unrecognized value (a typo, a legacy code, or a value from a newer schema)
    /// to [`Vertragsstatus::Unknown`] — this returns
    /// [`Err`](crate::error::UnknownVariant) for values not defined in this schema
    /// version, including the literal `"UNKNOWN"`.  Use it at the ingest boundary to
    /// reject bad values instead of silently degrading them.
    ///
    /// # Example
    /// ```
    /// # use rubo4e::current::Vertragsstatus;
    /// assert_eq!(Vertragsstatus::from_wire("IN_ARBEIT"), Ok(Vertragsstatus::InArbeit));
    /// // Out-of-schema values are rejected rather than degraded:
    /// assert!(Vertragsstatus::from_wire("NOT_A_REAL_VALUE").is_err());
    /// // …including the `Unknown` catch-all's own wire spelling:
    /// assert!(Vertragsstatus::from_wire("UNKNOWN").is_err());
    /// ```
    pub fn from_wire(s: &str) -> Result<Self, crate::error::UnknownVariant> {
        match s {
            "IN_ARBEIT" => Ok(Self::InArbeit),
            "UEBERMITTELT" => Ok(Self::Uebermittelt),
            "ANGENOMMEN" => Ok(Self::Angenommen),
            "AKTIV" => Ok(Self::Aktiv),
            "ABGELEHNT" => Ok(Self::Abgelehnt),
            "WIDERRUFEN" => Ok(Self::Widerrufen),
            "STORNIERT" => Ok(Self::Storniert),
            "GEKUENDIGT" => Ok(Self::Gekuendigt),
            "BEENDET" => Ok(Self::Beendet),
            other => Err(crate::error::UnknownVariant::new(other)),
        }
    }
    /// Returns `true` if this value is the forward-compatibility
    /// [`Vertragsstatus::Unknown`] catch-all (an out-of-schema value).
    pub const fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
    /// Returns `true` if this value is a known, schema-defined variant.
    pub const fn is_known(&self) -> bool {
        !self.is_unknown()
    }
}
impl std::fmt::Display for Vertragsstatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_wire())
    }
}
impl AsRef<str> for Vertragsstatus {
    fn as_ref(&self) -> &str {
        self.as_wire()
    }
}
#[cfg(feature = "versioned")]
impl crate::bo4e_enum_sealed::Sealed for Vertragsstatus {}
#[cfg(feature = "versioned")]
impl crate::Bo4eEnum for Vertragsstatus {
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
impl crate::Bo4eStrict for Vertragsstatus {
    fn collect_unknown_enums(&self, path: &str, out: &mut Vec<String>) {
        if self.is_unknown() {
            out.push(path.to_owned());
        }
    }
}
#[cfg(feature = "sqlx")]
impl sqlx::Type<sqlx::Postgres> for Vertragsstatus {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
/// Encodes as the canonical BO4E wire string, borrowed from `as_wire` — no
/// intermediate `String` or `serde_json::Value` is allocated.
#[cfg(feature = "sqlx")]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Vertragsstatus {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        let s: &str = self.as_wire();
        <&str as sqlx::Encode<'q, sqlx::Postgres>>::encode_by_ref(&s, buf)
    }
}
/// Decodes leniently, matching the `serde` path: a value the schema does not
/// define becomes [`Vertragsstatus::Unknown`] rather than a decode error, so a
/// database row written by a newer schema version still reads back.
///
/// Use [`Vertragsstatus::from_wire`] on a `String` column, or check
/// [`Vertragsstatus::is_known`], where out-of-schema values must be rejected.
#[cfg(feature = "sqlx")]
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for Vertragsstatus {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <&str as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        Ok(Self::from_wire(s).unwrap_or(Self::Unknown))
    }
}
/// Lets `Vec<Vertragsstatus>` bind to a `TEXT[]` column.  Only this crate can
/// provide it: the trait and the enum are both foreign to any consumer, so the
/// orphan rule rules out a downstream impl.
#[cfg(feature = "sqlx")]
impl sqlx::postgres::PgHasArrayType for Vertragsstatus {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::postgres::PgHasArrayType>::array_type_info()
    }
}
#[cfg(test)]
impl proptest::arbitrary::Arbitrary for Vertragsstatus {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        proptest::sample::select(Self::VARIANTS.to_vec()).boxed()
    }
}
