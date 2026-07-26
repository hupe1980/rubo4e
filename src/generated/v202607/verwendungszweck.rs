#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(strum::EnumString, strum::EnumIter, strum::IntoStaticStr)
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Verwendungszweck der Werte Marktlokation
#[non_exhaustive]
pub enum Verwendungszweck {
    #[cfg_attr(feature = "serde", serde(rename = "NETZNUTZUNGSABRECHNUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "NETZNUTZUNGSABRECHNUNG"))]
    Netznutzungsabrechnung,
    #[cfg_attr(feature = "serde", serde(rename = "BILANZKREISABRECHNUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BILANZKREISABRECHNUNG"))]
    Bilanzkreisabrechnung,
    #[cfg_attr(feature = "serde", serde(rename = "MEHRMINDERMENGENABRECHNUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MEHRMINDERMENGENABRECHNUNG"))]
    Mehrmindermengenabrechnung,
    #[cfg_attr(feature = "serde", serde(rename = "ENDKUNDENABRECHNUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ENDKUNDENABRECHNUNG"))]
    Endkundenabrechnung,
    #[cfg_attr(feature = "serde", serde(rename = "UEBERMITTLUNG_AN_DAS_HKNR"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UEBERMITTLUNG_AN_DAS_HKNR"))]
    UebermittlungAnDasHknr,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "ERMITTLUNG_AUSGEGLICHENHEIT_BILANZKREIS")
    )]
    #[cfg_attr(
        feature = "strum",
        strum(serialize = "ERMITTLUNG_AUSGEGLICHENHEIT_BILANZKREIS")
    )]
    ErmittlungAusgeglichenheitBilanzkreis,
    /// Unknown or future variant — produced when deserializing a value
    /// that is not yet known to this version of the library.
    #[cfg_attr(feature = "serde", serde(other, rename = "UNKNOWN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UNKNOWN"))]
    Unknown,
}
impl Verwendungszweck {
    /// All variants defined by the BO4E schema, in declaration order.
    ///
    /// Excludes the forward-compatibility [`Verwendungszweck::Unknown`] catch-all, so this
    /// is exactly the set of values that appear on the wire.  Available **without**
    /// the `strum` feature — use it to drift-guard SQL `CHECK` lists and mappings.
    pub const VARIANTS: &'static [Self] = &[
        Self::Netznutzungsabrechnung,
        Self::Bilanzkreisabrechnung,
        Self::Mehrmindermengenabrechnung,
        Self::Endkundenabrechnung,
        Self::UebermittlungAnDasHknr,
        Self::ErmittlungAusgeglichenheitBilanzkreis,
    ];
    /// Number of schema-defined variants (equal to `VARIANTS.len()`), excluding the
    /// [`Verwendungszweck::Unknown`] catch-all.  Stable for this schema version.
    pub const COUNT: usize = Self::VARIANTS.len();
    /// Returns an iterator over all **known** variants of `Verwendungszweck`.
    ///
    /// Yields only variants that correspond to values defined in the BO4E schema
    /// (i.e. [`Self::VARIANTS`]), never the [`Verwendungszweck::Unknown`] catch-all.
    /// Available **without** the `strum` feature.
    ///
    /// # Example
    /// ```rust,ignore
    /// for v in Verwendungszweck::iter_known() {
    ///     println!("{}", v.as_wire());
    /// }
    /// ```
    pub fn iter_known() -> impl Iterator<Item = Self> + Clone {
        Self::VARIANTS.iter().copied()
    }
    /// Returns the canonical BO4E wire string (SCREAMING_SNAKE_CASE) for this value.
    ///
    /// [`Verwendungszweck::Unknown`] renders as `"UNKNOWN"`, matching its serialized form.
    pub const fn as_wire(&self) -> &'static str {
        match self {
            Self::Netznutzungsabrechnung => "NETZNUTZUNGSABRECHNUNG",
            Self::Bilanzkreisabrechnung => "BILANZKREISABRECHNUNG",
            Self::Mehrmindermengenabrechnung => "MEHRMINDERMENGENABRECHNUNG",
            Self::Endkundenabrechnung => "ENDKUNDENABRECHNUNG",
            Self::UebermittlungAnDasHknr => "UEBERMITTLUNG_AN_DAS_HKNR",
            Self::ErmittlungAusgeglichenheitBilanzkreis => {
                "ERMITTLUNG_AUSGEGLICHENHEIT_BILANZKREIS"
            }
            Self::Unknown => "UNKNOWN",
        }
    }
    /// **Strictly** parses a BO4E wire string into a known variant.
    ///
    /// Unlike the lenient `serde` / [`FromStr`](std::str::FromStr) path — which maps
    /// any unrecognized value (a typo, a legacy code, or a value from a newer schema)
    /// to [`Verwendungszweck::Unknown`] — this returns
    /// [`Err`](crate::error::UnknownVariant) for values not defined in this schema
    /// version, including the literal `"UNKNOWN"`.  Use it at the ingest boundary to
    /// reject bad values instead of silently degrading them.
    ///
    /// # Example
    /// ```rust,ignore
    /// assert!(Verwendungszweck::from_wire("NOT_A_REAL_VALUE").is_err());
    /// ```
    pub fn from_wire(s: &str) -> Result<Self, crate::error::UnknownVariant> {
        match s {
            "NETZNUTZUNGSABRECHNUNG" => Ok(Self::Netznutzungsabrechnung),
            "BILANZKREISABRECHNUNG" => Ok(Self::Bilanzkreisabrechnung),
            "MEHRMINDERMENGENABRECHNUNG" => Ok(Self::Mehrmindermengenabrechnung),
            "ENDKUNDENABRECHNUNG" => Ok(Self::Endkundenabrechnung),
            "UEBERMITTLUNG_AN_DAS_HKNR" => Ok(Self::UebermittlungAnDasHknr),
            "ERMITTLUNG_AUSGEGLICHENHEIT_BILANZKREIS" => {
                Ok(Self::ErmittlungAusgeglichenheitBilanzkreis)
            }
            other => Err(crate::error::UnknownVariant::new(other)),
        }
    }
    /// Returns `true` if this value is the forward-compatibility
    /// [`Verwendungszweck::Unknown`] catch-all (an out-of-schema value).
    pub const fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
    /// Returns `true` if this value is a known, schema-defined variant.
    pub const fn is_known(&self) -> bool {
        !self.is_unknown()
    }
}
impl std::fmt::Display for Verwendungszweck {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_wire())
    }
}
impl AsRef<str> for Verwendungszweck {
    fn as_ref(&self) -> &str {
        self.as_wire()
    }
}
#[cfg(feature = "versioned")]
impl crate::bo4e_enum_sealed::Sealed for Verwendungszweck {}
#[cfg(feature = "versioned")]
impl crate::Bo4eEnum for Verwendungszweck {
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
impl crate::Bo4eStrict for Verwendungszweck {
    fn collect_unknown_enums(&self, path: &str, out: &mut Vec<String>) {
        if self.is_unknown() {
            out.push(path.to_owned());
        }
    }
}
#[cfg(all(feature = "sqlx", feature = "json"))]
impl sqlx::Type<sqlx::Postgres> for Verwendungszweck {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
/// Encode via the canonical wire string (`as_wire`, always available) — no
/// `serde_json::Value` intermediate, saving an allocation per encode (M-07).
#[cfg(all(feature = "sqlx", feature = "json"))]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Verwendungszweck {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        let s: &str = self.as_wire();
        <&str as sqlx::Encode<'q, sqlx::Postgres>>::encode_by_ref(&s, buf)
    }
}
#[cfg(all(feature = "sqlx", feature = "json"))]
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for Verwendungszweck {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <String as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        serde_json::from_value(serde_json::Value::String(s))
            .map_err(|e| Box::new(e) as sqlx::error::BoxDynError)
    }
}
#[cfg(test)]
impl proptest::arbitrary::Arbitrary for Verwendungszweck {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        proptest::sample::select(Self::VARIANTS.to_vec()).boxed()
    }
}
