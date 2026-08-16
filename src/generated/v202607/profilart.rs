#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(strum::EnumString, strum::EnumIter, strum::IntoStaticStr)
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Profilart: temperaturabh./Standardlastprofil.
#[non_exhaustive]
pub enum Profilart {
    #[cfg_attr(feature = "serde", serde(rename = "ART_STANDARDLASTPROFIL"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ART_STANDARDLASTPROFIL"))]
    ArtStandardlastprofil,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "ART_TAGESPARAMETERABHAENGIGES_LASTPROFIL")
    )]
    #[cfg_attr(
        feature = "strum",
        strum(serialize = "ART_TAGESPARAMETERABHAENGIGES_LASTPROFIL")
    )]
    ArtTagesparameterabhaengigesLastprofil,
    #[cfg_attr(feature = "serde", serde(rename = "ART_LASTPROFIL"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ART_LASTPROFIL"))]
    ArtLastprofil,
    #[cfg_attr(feature = "serde", serde(rename = "ART_STANDARDEINSPEISEPROFIL"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ART_STANDARDEINSPEISEPROFIL"))]
    ArtStandardeinspeiseprofil,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "ART_TAGESPARAMETERABHAENGIGES_EINSPEISEPROFIL")
    )]
    #[cfg_attr(
        feature = "strum",
        strum(serialize = "ART_TAGESPARAMETERABHAENGIGES_EINSPEISEPROFIL")
    )]
    ArtTagesparameterabhaengigesEinspeiseprofil,
    /// Unknown or future variant — produced when deserializing a value
    /// that is not yet known to this version of the library.
    #[cfg_attr(feature = "serde", serde(other, rename = "UNKNOWN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UNKNOWN"))]
    Unknown,
}
impl Profilart {
    /// All variants defined by the BO4E schema, in declaration order.
    ///
    /// Excludes the forward-compatibility [`Profilart::Unknown`] catch-all, so this
    /// is exactly the set of values that appear on the wire.  Available **without**
    /// the `strum` feature — use it to drift-guard SQL `CHECK` lists and mappings.
    pub const VARIANTS: &'static [Self] = &[
        Self::ArtStandardlastprofil,
        Self::ArtTagesparameterabhaengigesLastprofil,
        Self::ArtLastprofil,
        Self::ArtStandardeinspeiseprofil,
        Self::ArtTagesparameterabhaengigesEinspeiseprofil,
    ];
    /// Number of schema-defined variants (equal to `VARIANTS.len()`), excluding the
    /// [`Profilart::Unknown`] catch-all.  Stable for this schema version.
    pub const COUNT: usize = Self::VARIANTS.len();
    /// Returns an iterator over all **known** variants of `Profilart`.
    ///
    /// Yields only variants that correspond to values defined in the BO4E schema
    /// (i.e. [`Self::VARIANTS`]), never the [`Profilart::Unknown`] catch-all.
    /// Available **without** the `strum` feature.
    ///
    /// # Example
    /// ```
    /// # use rubo4e::current::Profilart;
    /// // Never yields the `Unknown` catch-all, so the count matches `COUNT`.
    /// assert_eq!(Profilart::iter_known().count(), Profilart::COUNT);
    /// assert!(Profilart::iter_known().all(|v| v.is_known()));
    /// ```
    pub fn iter_known() -> impl Iterator<Item = Self> + Clone {
        Self::VARIANTS.iter().copied()
    }
    /// Returns the canonical BO4E wire string (SCREAMING_SNAKE_CASE) for this value.
    ///
    /// [`Profilart::Unknown`] renders as `"UNKNOWN"`, matching its serialized form.
    pub const fn as_wire(&self) -> &'static str {
        match self {
            Self::ArtStandardlastprofil => "ART_STANDARDLASTPROFIL",
            Self::ArtTagesparameterabhaengigesLastprofil => {
                "ART_TAGESPARAMETERABHAENGIGES_LASTPROFIL"
            }
            Self::ArtLastprofil => "ART_LASTPROFIL",
            Self::ArtStandardeinspeiseprofil => "ART_STANDARDEINSPEISEPROFIL",
            Self::ArtTagesparameterabhaengigesEinspeiseprofil => {
                "ART_TAGESPARAMETERABHAENGIGES_EINSPEISEPROFIL"
            }
            Self::Unknown => "UNKNOWN",
        }
    }
    /// **Strictly** parses a BO4E wire string into a known variant.
    ///
    /// Unlike the lenient `serde` / [`FromStr`](std::str::FromStr) path — which maps
    /// any unrecognized value (a typo, a legacy code, or a value from a newer schema)
    /// to [`Profilart::Unknown`] — this returns
    /// [`Err`](crate::error::UnknownVariant) for values not defined in this schema
    /// version, including the literal `"UNKNOWN"`.  Use it at the ingest boundary to
    /// reject bad values instead of silently degrading them.
    ///
    /// # Example
    /// ```
    /// # use rubo4e::current::Profilart;
    /// /// assert_eq!(Profilart::from_wire("ART_STANDARDLASTPROFIL"), Ok(Profilart::ArtStandardlastprofil));
    /// assert!(Profilart::from_wire("NOT_A_REAL_VALUE").is_err());
    /// // …including the `Unknown` catch-all's own wire spelling:
    /// assert!(Profilart::from_wire("UNKNOWN").is_err());
    /// ```
    pub fn from_wire(s: &str) -> Result<Self, crate::error::UnknownVariant> {
        match s {
            "ART_STANDARDLASTPROFIL" => Ok(Self::ArtStandardlastprofil),
            "ART_TAGESPARAMETERABHAENGIGES_LASTPROFIL" => {
                Ok(Self::ArtTagesparameterabhaengigesLastprofil)
            }
            "ART_LASTPROFIL" => Ok(Self::ArtLastprofil),
            "ART_STANDARDEINSPEISEPROFIL" => Ok(Self::ArtStandardeinspeiseprofil),
            "ART_TAGESPARAMETERABHAENGIGES_EINSPEISEPROFIL" => {
                Ok(Self::ArtTagesparameterabhaengigesEinspeiseprofil)
            }
            other => Err(crate::error::UnknownVariant::new(other)),
        }
    }
    /// Returns `true` if this value is the forward-compatibility
    /// [`Profilart::Unknown`] catch-all (an out-of-schema value).
    pub const fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
    /// Returns `true` if this value is a known, schema-defined variant.
    pub const fn is_known(&self) -> bool {
        !self.is_unknown()
    }
}
impl std::fmt::Display for Profilart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_wire())
    }
}
impl AsRef<str> for Profilart {
    fn as_ref(&self) -> &str {
        self.as_wire()
    }
}
#[cfg(feature = "versioned")]
impl crate::bo4e_enum_sealed::Sealed for Profilart {}
#[cfg(feature = "versioned")]
impl crate::Bo4eEnum for Profilart {
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
impl crate::Bo4eStrict for Profilart {
    fn collect_unknown_enums(&self, path: &str, out: &mut Vec<String>) {
        if self.is_unknown() {
            out.push(path.to_owned());
        }
    }
}
#[cfg(feature = "sqlx")]
impl sqlx::Type<sqlx::Postgres> for Profilart {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
/// Encodes as the canonical BO4E wire string, borrowed from `as_wire` — no
/// intermediate `String` or `serde_json::Value` is allocated.
#[cfg(feature = "sqlx")]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Profilart {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        let s: &str = self.as_wire();
        <&str as sqlx::Encode<'q, sqlx::Postgres>>::encode_by_ref(&s, buf)
    }
}
/// Decodes leniently, matching the `serde` path: a value the schema does not
/// define becomes [`Profilart::Unknown`] rather than a decode error, so a
/// database row written by a newer schema version still reads back.
///
/// Use [`Profilart::from_wire`] on a `String` column, or check
/// [`Profilart::is_known`], where out-of-schema values must be rejected.
#[cfg(feature = "sqlx")]
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for Profilart {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <&str as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        Ok(Self::from_wire(s).unwrap_or(Self::Unknown))
    }
}
#[cfg(test)]
impl proptest::arbitrary::Arbitrary for Profilart {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        proptest::sample::select(Self::VARIANTS.to_vec()).boxed()
    }
}
