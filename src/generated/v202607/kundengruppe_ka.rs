#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(strum::EnumString, strum::EnumIter, strum::IntoStaticStr)
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Eine Aufzählung zur Einordnung für die Höhe der Konzessionsabgabe.
#[non_exhaustive]
pub enum KundengruppeKa {
    #[cfg_attr(feature = "serde", serde(rename = "S_SCHWACHLAST"))]
    #[cfg_attr(feature = "strum", strum(serialize = "S_SCHWACHLAST"))]
    SSchwachlast,
    #[cfg_attr(feature = "serde", serde(rename = "S_TARIF_25000"))]
    #[cfg_attr(feature = "strum", strum(serialize = "S_TARIF_25000"))]
    STarif25000,
    #[cfg_attr(feature = "serde", serde(rename = "S_TARIF_100000"))]
    #[cfg_attr(feature = "strum", strum(serialize = "S_TARIF_100000"))]
    STarif100000,
    #[cfg_attr(feature = "serde", serde(rename = "S_TARIF_500000"))]
    #[cfg_attr(feature = "strum", strum(serialize = "S_TARIF_500000"))]
    STarif500000,
    #[cfg_attr(feature = "serde", serde(rename = "S_TARIF_G_500000"))]
    #[cfg_attr(feature = "strum", strum(serialize = "S_TARIF_G_500000"))]
    STarifG500000,
    #[cfg_attr(feature = "serde", serde(rename = "S_SONDERKUNDE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "S_SONDERKUNDE"))]
    SSonderkunde,
    #[cfg_attr(feature = "serde", serde(rename = "G_KOWA_25000"))]
    #[cfg_attr(feature = "strum", strum(serialize = "G_KOWA_25000"))]
    GKowa25000,
    #[cfg_attr(feature = "serde", serde(rename = "G_KOWA_100000"))]
    #[cfg_attr(feature = "strum", strum(serialize = "G_KOWA_100000"))]
    GKowa100000,
    #[cfg_attr(feature = "serde", serde(rename = "G_KOWA_500000"))]
    #[cfg_attr(feature = "strum", strum(serialize = "G_KOWA_500000"))]
    GKowa500000,
    #[cfg_attr(feature = "serde", serde(rename = "G_KOWA_G_500000"))]
    #[cfg_attr(feature = "strum", strum(serialize = "G_KOWA_G_500000"))]
    GKowaG500000,
    #[cfg_attr(feature = "serde", serde(rename = "G_TARIF_25000"))]
    #[cfg_attr(feature = "strum", strum(serialize = "G_TARIF_25000"))]
    GTarif25000,
    #[cfg_attr(feature = "serde", serde(rename = "G_TARIF_100000"))]
    #[cfg_attr(feature = "strum", strum(serialize = "G_TARIF_100000"))]
    GTarif100000,
    #[cfg_attr(feature = "serde", serde(rename = "G_TARIF_500000"))]
    #[cfg_attr(feature = "strum", strum(serialize = "G_TARIF_500000"))]
    GTarif500000,
    #[cfg_attr(feature = "serde", serde(rename = "G_TARIF_G_500000"))]
    #[cfg_attr(feature = "strum", strum(serialize = "G_TARIF_G_500000"))]
    GTarifG500000,
    #[cfg_attr(feature = "serde", serde(rename = "G_SONDERKUNDE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "G_SONDERKUNDE"))]
    GSonderkunde,
    #[cfg_attr(feature = "serde", serde(rename = "SONDER_KAS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SONDER_KAS"))]
    SonderKas,
    #[cfg_attr(feature = "serde", serde(rename = "SONDER_SAS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SONDER_SAS"))]
    SonderSas,
    #[cfg_attr(feature = "serde", serde(rename = "SONDER_TAS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SONDER_TAS"))]
    SonderTas,
    #[cfg_attr(feature = "serde", serde(rename = "SONDER_TKS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SONDER_TKS"))]
    SonderTks,
    #[cfg_attr(feature = "serde", serde(rename = "SONDER_TSS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SONDER_TSS"))]
    SonderTss,
    /// Unknown or future variant — produced when deserializing a value
    /// that is not yet known to this version of the library.
    #[cfg_attr(feature = "serde", serde(other, rename = "UNKNOWN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UNKNOWN"))]
    Unknown,
}
impl KundengruppeKa {
    /// All variants defined by the BO4E schema, in declaration order.
    ///
    /// Excludes the forward-compatibility [`KundengruppeKa::Unknown`] catch-all, so this
    /// is exactly the set of values that appear on the wire.  Available **without**
    /// the `strum` feature — use it to drift-guard SQL `CHECK` lists and mappings.
    pub const VARIANTS: &'static [Self] = &[
        Self::SSchwachlast,
        Self::STarif25000,
        Self::STarif100000,
        Self::STarif500000,
        Self::STarifG500000,
        Self::SSonderkunde,
        Self::GKowa25000,
        Self::GKowa100000,
        Self::GKowa500000,
        Self::GKowaG500000,
        Self::GTarif25000,
        Self::GTarif100000,
        Self::GTarif500000,
        Self::GTarifG500000,
        Self::GSonderkunde,
        Self::SonderKas,
        Self::SonderSas,
        Self::SonderTas,
        Self::SonderTks,
        Self::SonderTss,
    ];
    /// Number of schema-defined variants (equal to `VARIANTS.len()`), excluding the
    /// [`KundengruppeKa::Unknown`] catch-all.  Stable for this schema version.
    pub const COUNT: usize = Self::VARIANTS.len();
    /// Returns an iterator over all **known** variants of `KundengruppeKa`.
    ///
    /// Yields only variants that correspond to values defined in the BO4E schema
    /// (i.e. [`Self::VARIANTS`]), never the [`KundengruppeKa::Unknown`] catch-all.
    /// Available **without** the `strum` feature.
    ///
    /// # Example
    /// ```
    /// # use rubo4e::current::KundengruppeKa;
    /// // Never yields the `Unknown` catch-all, so the count matches `COUNT`.
    /// assert_eq!(KundengruppeKa::iter_known().count(), KundengruppeKa::COUNT);
    /// assert!(KundengruppeKa::iter_known().all(|v| v.is_known()));
    /// ```
    pub fn iter_known() -> impl Iterator<Item = Self> + Clone {
        Self::VARIANTS.iter().copied()
    }
    /// Returns the canonical BO4E wire string (SCREAMING_SNAKE_CASE) for this value.
    ///
    /// [`KundengruppeKa::Unknown`] renders as `"UNKNOWN"`, matching its serialized form.
    pub const fn as_wire(&self) -> &'static str {
        match self {
            Self::SSchwachlast => "S_SCHWACHLAST",
            Self::STarif25000 => "S_TARIF_25000",
            Self::STarif100000 => "S_TARIF_100000",
            Self::STarif500000 => "S_TARIF_500000",
            Self::STarifG500000 => "S_TARIF_G_500000",
            Self::SSonderkunde => "S_SONDERKUNDE",
            Self::GKowa25000 => "G_KOWA_25000",
            Self::GKowa100000 => "G_KOWA_100000",
            Self::GKowa500000 => "G_KOWA_500000",
            Self::GKowaG500000 => "G_KOWA_G_500000",
            Self::GTarif25000 => "G_TARIF_25000",
            Self::GTarif100000 => "G_TARIF_100000",
            Self::GTarif500000 => "G_TARIF_500000",
            Self::GTarifG500000 => "G_TARIF_G_500000",
            Self::GSonderkunde => "G_SONDERKUNDE",
            Self::SonderKas => "SONDER_KAS",
            Self::SonderSas => "SONDER_SAS",
            Self::SonderTas => "SONDER_TAS",
            Self::SonderTks => "SONDER_TKS",
            Self::SonderTss => "SONDER_TSS",
            Self::Unknown => "UNKNOWN",
        }
    }
    /// **Strictly** parses a BO4E wire string into a known variant.
    ///
    /// Unlike the lenient `serde` / [`FromStr`](std::str::FromStr) path — which maps
    /// any unrecognized value (a typo, a legacy code, or a value from a newer schema)
    /// to [`KundengruppeKa::Unknown`] — this returns
    /// [`Err`](crate::error::UnknownVariant) for values not defined in this schema
    /// version, including the literal `"UNKNOWN"`.  Use it at the ingest boundary to
    /// reject bad values instead of silently degrading them.
    ///
    /// # Example
    /// ```
    /// # use rubo4e::current::KundengruppeKa;
    /// /// assert_eq!(KundengruppeKa::from_wire("S_SCHWACHLAST"), Ok(KundengruppeKa::SSchwachlast));
    /// assert!(KundengruppeKa::from_wire("NOT_A_REAL_VALUE").is_err());
    /// // …including the `Unknown` catch-all's own wire spelling:
    /// assert!(KundengruppeKa::from_wire("UNKNOWN").is_err());
    /// ```
    pub fn from_wire(s: &str) -> Result<Self, crate::error::UnknownVariant> {
        match s {
            "S_SCHWACHLAST" => Ok(Self::SSchwachlast),
            "S_TARIF_25000" => Ok(Self::STarif25000),
            "S_TARIF_100000" => Ok(Self::STarif100000),
            "S_TARIF_500000" => Ok(Self::STarif500000),
            "S_TARIF_G_500000" => Ok(Self::STarifG500000),
            "S_SONDERKUNDE" => Ok(Self::SSonderkunde),
            "G_KOWA_25000" => Ok(Self::GKowa25000),
            "G_KOWA_100000" => Ok(Self::GKowa100000),
            "G_KOWA_500000" => Ok(Self::GKowa500000),
            "G_KOWA_G_500000" => Ok(Self::GKowaG500000),
            "G_TARIF_25000" => Ok(Self::GTarif25000),
            "G_TARIF_100000" => Ok(Self::GTarif100000),
            "G_TARIF_500000" => Ok(Self::GTarif500000),
            "G_TARIF_G_500000" => Ok(Self::GTarifG500000),
            "G_SONDERKUNDE" => Ok(Self::GSonderkunde),
            "SONDER_KAS" => Ok(Self::SonderKas),
            "SONDER_SAS" => Ok(Self::SonderSas),
            "SONDER_TAS" => Ok(Self::SonderTas),
            "SONDER_TKS" => Ok(Self::SonderTks),
            "SONDER_TSS" => Ok(Self::SonderTss),
            other => Err(crate::error::UnknownVariant::new(other)),
        }
    }
    /// Returns `true` if this value is the forward-compatibility
    /// [`KundengruppeKa::Unknown`] catch-all (an out-of-schema value).
    pub const fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
    /// Returns `true` if this value is a known, schema-defined variant.
    pub const fn is_known(&self) -> bool {
        !self.is_unknown()
    }
}
impl std::fmt::Display for KundengruppeKa {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_wire())
    }
}
impl AsRef<str> for KundengruppeKa {
    fn as_ref(&self) -> &str {
        self.as_wire()
    }
}
#[cfg(feature = "versioned")]
impl crate::bo4e_enum_sealed::Sealed for KundengruppeKa {}
#[cfg(feature = "versioned")]
impl crate::Bo4eEnum for KundengruppeKa {
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
impl crate::Bo4eStrict for KundengruppeKa {
    fn collect_unknown_enums(&self, path: &str, out: &mut Vec<String>) {
        if self.is_unknown() {
            out.push(path.to_owned());
        }
    }
}
#[cfg(feature = "sqlx")]
impl sqlx::Type<sqlx::Postgres> for KundengruppeKa {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
/// Encodes as the canonical BO4E wire string, borrowed from `as_wire` — no
/// intermediate `String` or `serde_json::Value` is allocated.
#[cfg(feature = "sqlx")]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for KundengruppeKa {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        let s: &str = self.as_wire();
        <&str as sqlx::Encode<'q, sqlx::Postgres>>::encode_by_ref(&s, buf)
    }
}
/// Decodes leniently, matching the `serde` path: a value the schema does not
/// define becomes [`KundengruppeKa::Unknown`] rather than a decode error, so a
/// database row written by a newer schema version still reads back.
///
/// Use [`KundengruppeKa::from_wire`] on a `String` column, or check
/// [`KundengruppeKa::is_known`], where out-of-schema values must be rejected.
#[cfg(feature = "sqlx")]
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for KundengruppeKa {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <&str as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        Ok(Self::from_wire(s).unwrap_or(Self::Unknown))
    }
}
#[cfg(test)]
impl proptest::arbitrary::Arbitrary for KundengruppeKa {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        proptest::sample::select(Self::VARIANTS.to_vec()).boxed()
    }
}
