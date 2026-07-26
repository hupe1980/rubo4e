#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(strum::EnumString, strum::EnumIter, strum::IntoStaticStr)
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Aufschlüsselung der Preistypen in Tarifen.
#[non_exhaustive]
pub enum Preistyp {
    #[cfg_attr(feature = "serde", serde(rename = "GRUNDPREIS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GRUNDPREIS"))]
    Grundpreis,
    #[cfg_attr(feature = "serde", serde(rename = "ARBEITSPREIS_EINTARIF"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ARBEITSPREIS_EINTARIF"))]
    ArbeitspreisEintarif,
    #[cfg_attr(feature = "serde", serde(rename = "ARBEITSPREIS_HT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ARBEITSPREIS_HT"))]
    ArbeitspreisHt,
    #[cfg_attr(feature = "serde", serde(rename = "ARBEITSPREIS_NT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ARBEITSPREIS_NT"))]
    ArbeitspreisNt,
    #[cfg_attr(feature = "serde", serde(rename = "LEISTUNGSPREIS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "LEISTUNGSPREIS"))]
    Leistungspreis,
    #[cfg_attr(feature = "serde", serde(rename = "MESSPREIS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MESSPREIS"))]
    Messpreis,
    #[cfg_attr(feature = "serde", serde(rename = "ENTGELT_ABLESUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ENTGELT_ABLESUNG"))]
    EntgeltAblesung,
    #[cfg_attr(feature = "serde", serde(rename = "ENTGELT_ABRECHNUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ENTGELT_ABRECHNUNG"))]
    EntgeltAbrechnung,
    #[cfg_attr(feature = "serde", serde(rename = "ENTGELT_MSB"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ENTGELT_MSB"))]
    EntgeltMsb,
    #[cfg_attr(feature = "serde", serde(rename = "PROVISION"))]
    #[cfg_attr(feature = "strum", strum(serialize = "PROVISION"))]
    Provision,
    /// Unknown or future variant — produced when deserializing a value
    /// that is not yet known to this version of the library.
    #[cfg_attr(feature = "serde", serde(other, rename = "UNKNOWN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UNKNOWN"))]
    Unknown,
}
impl Preistyp {
    /// All variants defined by the BO4E schema, in declaration order.
    ///
    /// Excludes the forward-compatibility [`Preistyp::Unknown`] catch-all, so this
    /// is exactly the set of values that appear on the wire.  Available **without**
    /// the `strum` feature — use it to drift-guard SQL `CHECK` lists and mappings.
    pub const VARIANTS: &'static [Self] = &[
        Self::Grundpreis,
        Self::ArbeitspreisEintarif,
        Self::ArbeitspreisHt,
        Self::ArbeitspreisNt,
        Self::Leistungspreis,
        Self::Messpreis,
        Self::EntgeltAblesung,
        Self::EntgeltAbrechnung,
        Self::EntgeltMsb,
        Self::Provision,
    ];
    /// Number of schema-defined variants (equal to `VARIANTS.len()`), excluding the
    /// [`Preistyp::Unknown`] catch-all.  Stable for this schema version.
    pub const COUNT: usize = Self::VARIANTS.len();
    /// Returns an iterator over all **known** variants of `Preistyp`.
    ///
    /// Yields only variants that correspond to values defined in the BO4E schema
    /// (i.e. [`Self::VARIANTS`]), never the [`Preistyp::Unknown`] catch-all.
    /// Available **without** the `strum` feature.
    ///
    /// # Example
    /// ```rust,ignore
    /// for v in Preistyp::iter_known() {
    ///     println!("{}", v.as_wire());
    /// }
    /// ```
    pub fn iter_known() -> impl Iterator<Item = Self> + Clone {
        Self::VARIANTS.iter().copied()
    }
    /// Returns the canonical BO4E wire string (SCREAMING_SNAKE_CASE) for this value.
    ///
    /// [`Preistyp::Unknown`] renders as `"UNKNOWN"`, matching its serialized form.
    pub const fn as_wire(&self) -> &'static str {
        match self {
            Self::Grundpreis => "GRUNDPREIS",
            Self::ArbeitspreisEintarif => "ARBEITSPREIS_EINTARIF",
            Self::ArbeitspreisHt => "ARBEITSPREIS_HT",
            Self::ArbeitspreisNt => "ARBEITSPREIS_NT",
            Self::Leistungspreis => "LEISTUNGSPREIS",
            Self::Messpreis => "MESSPREIS",
            Self::EntgeltAblesung => "ENTGELT_ABLESUNG",
            Self::EntgeltAbrechnung => "ENTGELT_ABRECHNUNG",
            Self::EntgeltMsb => "ENTGELT_MSB",
            Self::Provision => "PROVISION",
            Self::Unknown => "UNKNOWN",
        }
    }
    /// **Strictly** parses a BO4E wire string into a known variant.
    ///
    /// Unlike the lenient `serde` / [`FromStr`](std::str::FromStr) path — which maps
    /// any unrecognized value (a typo, a legacy code, or a value from a newer schema)
    /// to [`Preistyp::Unknown`] — this returns
    /// [`Err`](crate::error::UnknownVariant) for values not defined in this schema
    /// version, including the literal `"UNKNOWN"`.  Use it at the ingest boundary to
    /// reject bad values instead of silently degrading them.
    ///
    /// # Example
    /// ```rust,ignore
    /// assert!(Preistyp::from_wire("NOT_A_REAL_VALUE").is_err());
    /// ```
    pub fn from_wire(s: &str) -> Result<Self, crate::error::UnknownVariant> {
        match s {
            "GRUNDPREIS" => Ok(Self::Grundpreis),
            "ARBEITSPREIS_EINTARIF" => Ok(Self::ArbeitspreisEintarif),
            "ARBEITSPREIS_HT" => Ok(Self::ArbeitspreisHt),
            "ARBEITSPREIS_NT" => Ok(Self::ArbeitspreisNt),
            "LEISTUNGSPREIS" => Ok(Self::Leistungspreis),
            "MESSPREIS" => Ok(Self::Messpreis),
            "ENTGELT_ABLESUNG" => Ok(Self::EntgeltAblesung),
            "ENTGELT_ABRECHNUNG" => Ok(Self::EntgeltAbrechnung),
            "ENTGELT_MSB" => Ok(Self::EntgeltMsb),
            "PROVISION" => Ok(Self::Provision),
            other => Err(crate::error::UnknownVariant::new(other)),
        }
    }
    /// Returns `true` if this value is the forward-compatibility
    /// [`Preistyp::Unknown`] catch-all (an out-of-schema value).
    pub const fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
    /// Returns `true` if this value is a known, schema-defined variant.
    pub const fn is_known(&self) -> bool {
        !self.is_unknown()
    }
}
impl std::fmt::Display for Preistyp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_wire())
    }
}
impl AsRef<str> for Preistyp {
    fn as_ref(&self) -> &str {
        self.as_wire()
    }
}
#[cfg(feature = "versioned")]
impl crate::bo4e_enum_sealed::Sealed for Preistyp {}
#[cfg(feature = "versioned")]
impl crate::Bo4eEnum for Preistyp {
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
impl crate::Bo4eStrict for Preistyp {
    fn collect_unknown_enums(&self, path: &str, out: &mut Vec<String>) {
        if self.is_unknown() {
            out.push(path.to_owned());
        }
    }
}
#[cfg(all(feature = "sqlx", feature = "json"))]
impl sqlx::Type<sqlx::Postgres> for Preistyp {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
/// Encode via the canonical wire string (`as_wire`, always available) — no
/// `serde_json::Value` intermediate, saving an allocation per encode (M-07).
#[cfg(all(feature = "sqlx", feature = "json"))]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Preistyp {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        let s: &str = self.as_wire();
        <&str as sqlx::Encode<'q, sqlx::Postgres>>::encode_by_ref(&s, buf)
    }
}
#[cfg(all(feature = "sqlx", feature = "json"))]
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for Preistyp {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <String as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        serde_json::from_value(serde_json::Value::String(s))
            .map_err(|e| Box::new(e) as sqlx::error::BoxDynError)
    }
}
#[cfg(test)]
impl proptest::arbitrary::Arbitrary for Preistyp {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        proptest::sample::select(Self::VARIANTS.to_vec()).boxed()
    }
}
