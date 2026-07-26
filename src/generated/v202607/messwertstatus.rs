#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(strum::EnumString, strum::EnumIter, strum::IntoStaticStr)
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Der Status eines Zählerstandes
#[non_exhaustive]
pub enum Messwertstatus {
    #[cfg_attr(feature = "serde", serde(rename = "ABGELESEN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ABGELESEN"))]
    Abgelesen,
    #[cfg_attr(feature = "serde", serde(rename = "ERSATZWERT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ERSATZWERT"))]
    Ersatzwert,
    #[cfg_attr(feature = "serde", serde(rename = "ANGABE_FUER_LIEFERSCHEIN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ANGABE_FUER_LIEFERSCHEIN"))]
    AngabeFuerLieferschein,
    #[cfg_attr(feature = "serde", serde(rename = "VORSCHLAGSWERT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "VORSCHLAGSWERT"))]
    Vorschlagswert,
    #[cfg_attr(feature = "serde", serde(rename = "NICHT_VERWENDBAR"))]
    #[cfg_attr(feature = "strum", strum(serialize = "NICHT_VERWENDBAR"))]
    NichtVerwendbar,
    #[cfg_attr(feature = "serde", serde(rename = "PROGNOSEWERT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "PROGNOSEWERT"))]
    Prognosewert,
    #[cfg_attr(feature = "serde", serde(rename = "VORLAEUFIGERWERT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "VORLAEUFIGERWERT"))]
    Vorlaeufigerwert,
    #[cfg_attr(feature = "serde", serde(rename = "ENERGIEMENGESUMMIERT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ENERGIEMENGESUMMIERT"))]
    Energiemengesummiert,
    #[cfg_attr(feature = "serde", serde(rename = "FEHLT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "FEHLT"))]
    Fehlt,
    /// Unknown or future variant — produced when deserializing a value
    /// that is not yet known to this version of the library.
    #[cfg_attr(feature = "serde", serde(other, rename = "UNKNOWN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UNKNOWN"))]
    Unknown,
}
impl Messwertstatus {
    /// All variants defined by the BO4E schema, in declaration order.
    ///
    /// Excludes the forward-compatibility [`Messwertstatus::Unknown`] catch-all, so this
    /// is exactly the set of values that appear on the wire.  Available **without**
    /// the `strum` feature — use it to drift-guard SQL `CHECK` lists and mappings.
    pub const VARIANTS: &'static [Self] = &[
        Self::Abgelesen,
        Self::Ersatzwert,
        Self::AngabeFuerLieferschein,
        Self::Vorschlagswert,
        Self::NichtVerwendbar,
        Self::Prognosewert,
        Self::Vorlaeufigerwert,
        Self::Energiemengesummiert,
        Self::Fehlt,
    ];
    /// Number of schema-defined variants (equal to `VARIANTS.len()`), excluding the
    /// [`Messwertstatus::Unknown`] catch-all.  Stable for this schema version.
    pub const COUNT: usize = Self::VARIANTS.len();
    /// Returns an iterator over all **known** variants of `Messwertstatus`.
    ///
    /// Yields only variants that correspond to values defined in the BO4E schema
    /// (i.e. [`Self::VARIANTS`]), never the [`Messwertstatus::Unknown`] catch-all.
    /// Available **without** the `strum` feature.
    ///
    /// # Example
    /// ```rust,ignore
    /// for v in Messwertstatus::iter_known() {
    ///     println!("{}", v.as_wire());
    /// }
    /// ```
    pub fn iter_known() -> impl Iterator<Item = Self> + Clone {
        Self::VARIANTS.iter().copied()
    }
    /// Returns the canonical BO4E wire string (SCREAMING_SNAKE_CASE) for this value.
    ///
    /// [`Messwertstatus::Unknown`] renders as `"UNKNOWN"`, matching its serialized form.
    pub const fn as_wire(&self) -> &'static str {
        match self {
            Self::Abgelesen => "ABGELESEN",
            Self::Ersatzwert => "ERSATZWERT",
            Self::AngabeFuerLieferschein => "ANGABE_FUER_LIEFERSCHEIN",
            Self::Vorschlagswert => "VORSCHLAGSWERT",
            Self::NichtVerwendbar => "NICHT_VERWENDBAR",
            Self::Prognosewert => "PROGNOSEWERT",
            Self::Vorlaeufigerwert => "VORLAEUFIGERWERT",
            Self::Energiemengesummiert => "ENERGIEMENGESUMMIERT",
            Self::Fehlt => "FEHLT",
            Self::Unknown => "UNKNOWN",
        }
    }
    /// **Strictly** parses a BO4E wire string into a known variant.
    ///
    /// Unlike the lenient `serde` / [`FromStr`](std::str::FromStr) path — which maps
    /// any unrecognized value (a typo, a legacy code, or a value from a newer schema)
    /// to [`Messwertstatus::Unknown`] — this returns
    /// [`Err`](crate::error::UnknownVariant) for values not defined in this schema
    /// version, including the literal `"UNKNOWN"`.  Use it at the ingest boundary to
    /// reject bad values instead of silently degrading them.
    ///
    /// # Example
    /// ```rust,ignore
    /// assert!(Messwertstatus::from_wire("NOT_A_REAL_VALUE").is_err());
    /// ```
    pub fn from_wire(s: &str) -> Result<Self, crate::error::UnknownVariant> {
        match s {
            "ABGELESEN" => Ok(Self::Abgelesen),
            "ERSATZWERT" => Ok(Self::Ersatzwert),
            "ANGABE_FUER_LIEFERSCHEIN" => Ok(Self::AngabeFuerLieferschein),
            "VORSCHLAGSWERT" => Ok(Self::Vorschlagswert),
            "NICHT_VERWENDBAR" => Ok(Self::NichtVerwendbar),
            "PROGNOSEWERT" => Ok(Self::Prognosewert),
            "VORLAEUFIGERWERT" => Ok(Self::Vorlaeufigerwert),
            "ENERGIEMENGESUMMIERT" => Ok(Self::Energiemengesummiert),
            "FEHLT" => Ok(Self::Fehlt),
            other => Err(crate::error::UnknownVariant::new(other)),
        }
    }
    /// Returns `true` if this value is the forward-compatibility
    /// [`Messwertstatus::Unknown`] catch-all (an out-of-schema value).
    pub const fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
    /// Returns `true` if this value is a known, schema-defined variant.
    pub const fn is_known(&self) -> bool {
        !self.is_unknown()
    }
}
impl std::fmt::Display for Messwertstatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_wire())
    }
}
impl AsRef<str> for Messwertstatus {
    fn as_ref(&self) -> &str {
        self.as_wire()
    }
}
#[cfg(feature = "versioned")]
impl crate::bo4e_enum_sealed::Sealed for Messwertstatus {}
#[cfg(feature = "versioned")]
impl crate::Bo4eEnum for Messwertstatus {
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
impl crate::Bo4eStrict for Messwertstatus {
    fn collect_unknown_enums(&self, path: &str, out: &mut Vec<String>) {
        if self.is_unknown() {
            out.push(path.to_owned());
        }
    }
}
#[cfg(all(feature = "sqlx", feature = "json"))]
impl sqlx::Type<sqlx::Postgres> for Messwertstatus {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
/// Encode via the canonical wire string (`as_wire`, always available) — no
/// `serde_json::Value` intermediate, saving an allocation per encode (M-07).
#[cfg(all(feature = "sqlx", feature = "json"))]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Messwertstatus {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        let s: &str = self.as_wire();
        <&str as sqlx::Encode<'q, sqlx::Postgres>>::encode_by_ref(&s, buf)
    }
}
#[cfg(all(feature = "sqlx", feature = "json"))]
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for Messwertstatus {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <String as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        serde_json::from_value(serde_json::Value::String(s))
            .map_err(|e| Box::new(e) as sqlx::error::BoxDynError)
    }
}
#[cfg(test)]
impl proptest::arbitrary::Arbitrary for Messwertstatus {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        proptest::sample::select(Self::VARIANTS.to_vec()).boxed()
    }
}
