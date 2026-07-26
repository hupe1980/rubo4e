#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(strum::EnumString, strum::EnumIter, strum::IntoStaticStr)
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Einheit: Messgrößen, die per Messung oder Vorgabe ermittelt werden können.
#[non_exhaustive]
pub enum Mengeneinheit {
    #[cfg_attr(feature = "serde", serde(rename = "W"))]
    #[cfg_attr(feature = "strum", strum(serialize = "W"))]
    W,
    #[cfg_attr(feature = "serde", serde(rename = "WH"))]
    #[cfg_attr(feature = "strum", strum(serialize = "WH"))]
    Wh,
    #[cfg_attr(feature = "serde", serde(rename = "KW"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KW"))]
    Kw,
    #[cfg_attr(feature = "serde", serde(rename = "KWH"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KWH"))]
    Kwh,
    #[cfg_attr(feature = "serde", serde(rename = "KVARH"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KVARH"))]
    Kvarh,
    #[cfg_attr(feature = "serde", serde(rename = "MW"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MW"))]
    Mw,
    #[cfg_attr(feature = "serde", serde(rename = "MWH"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MWH"))]
    Mwh,
    #[cfg_attr(feature = "serde", serde(rename = "STUECK"))]
    #[cfg_attr(feature = "strum", strum(serialize = "STUECK"))]
    Stueck,
    #[cfg_attr(feature = "serde", serde(rename = "KUBIKMETER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KUBIKMETER"))]
    Kubikmeter,
    #[cfg_attr(feature = "serde", serde(rename = "SEKUNDE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SEKUNDE"))]
    Sekunde,
    #[cfg_attr(feature = "serde", serde(rename = "MINUTE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MINUTE"))]
    Minute,
    #[cfg_attr(feature = "serde", serde(rename = "STUNDE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "STUNDE"))]
    Stunde,
    #[cfg_attr(feature = "serde", serde(rename = "VIERTEL_STUNDE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "VIERTEL_STUNDE"))]
    ViertelStunde,
    #[cfg_attr(feature = "serde", serde(rename = "TAG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TAG"))]
    Tag,
    #[cfg_attr(feature = "serde", serde(rename = "WOCHE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "WOCHE"))]
    Woche,
    #[cfg_attr(feature = "serde", serde(rename = "MONAT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MONAT"))]
    Monat,
    #[cfg_attr(feature = "serde", serde(rename = "QUARTAL"))]
    #[cfg_attr(feature = "strum", strum(serialize = "QUARTAL"))]
    Quartal,
    #[cfg_attr(feature = "serde", serde(rename = "HALBJAHR"))]
    #[cfg_attr(feature = "strum", strum(serialize = "HALBJAHR"))]
    Halbjahr,
    #[cfg_attr(feature = "serde", serde(rename = "JAHR"))]
    #[cfg_attr(feature = "strum", strum(serialize = "JAHR"))]
    Jahr,
    #[cfg_attr(feature = "serde", serde(rename = "PROZENT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "PROZENT"))]
    Prozent,
    #[cfg_attr(feature = "serde", serde(rename = "KVAR"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KVAR"))]
    Kvar,
    #[cfg_attr(feature = "serde", serde(rename = "KWHK"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KWHK"))]
    Kwhk,
    #[cfg_attr(feature = "serde", serde(rename = "VAR"))]
    #[cfg_attr(feature = "strum", strum(serialize = "VAR"))]
    Var,
    #[cfg_attr(feature = "serde", serde(rename = "VARH"))]
    #[cfg_attr(feature = "strum", strum(serialize = "VARH"))]
    Varh,
    /// Unknown or future variant — produced when deserializing a value
    /// that is not yet known to this version of the library.
    #[cfg_attr(feature = "serde", serde(other, rename = "UNKNOWN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UNKNOWN"))]
    Unknown,
}
impl Mengeneinheit {
    /// All variants defined by the BO4E schema, in declaration order.
    ///
    /// Excludes the forward-compatibility [`Mengeneinheit::Unknown`] catch-all, so this
    /// is exactly the set of values that appear on the wire.  Available **without**
    /// the `strum` feature — use it to drift-guard SQL `CHECK` lists and mappings.
    pub const VARIANTS: &'static [Self] = &[
        Self::W,
        Self::Wh,
        Self::Kw,
        Self::Kwh,
        Self::Kvarh,
        Self::Mw,
        Self::Mwh,
        Self::Stueck,
        Self::Kubikmeter,
        Self::Sekunde,
        Self::Minute,
        Self::Stunde,
        Self::ViertelStunde,
        Self::Tag,
        Self::Woche,
        Self::Monat,
        Self::Quartal,
        Self::Halbjahr,
        Self::Jahr,
        Self::Prozent,
        Self::Kvar,
        Self::Kwhk,
        Self::Var,
        Self::Varh,
    ];
    /// Number of schema-defined variants (equal to `VARIANTS.len()`), excluding the
    /// [`Mengeneinheit::Unknown`] catch-all.  Stable for this schema version.
    pub const COUNT: usize = Self::VARIANTS.len();
    /// Returns an iterator over all **known** variants of `Mengeneinheit`.
    ///
    /// Yields only variants that correspond to values defined in the BO4E schema
    /// (i.e. [`Self::VARIANTS`]), never the [`Mengeneinheit::Unknown`] catch-all.
    /// Available **without** the `strum` feature.
    ///
    /// # Example
    /// ```rust,ignore
    /// for v in Mengeneinheit::iter_known() {
    ///     println!("{}", v.as_wire());
    /// }
    /// ```
    pub fn iter_known() -> impl Iterator<Item = Self> + Clone {
        Self::VARIANTS.iter().copied()
    }
    /// Returns the canonical BO4E wire string (SCREAMING_SNAKE_CASE) for this value.
    ///
    /// [`Mengeneinheit::Unknown`] renders as `"UNKNOWN"`, matching its serialized form.
    pub const fn as_wire(&self) -> &'static str {
        match self {
            Self::W => "W",
            Self::Wh => "WH",
            Self::Kw => "KW",
            Self::Kwh => "KWH",
            Self::Kvarh => "KVARH",
            Self::Mw => "MW",
            Self::Mwh => "MWH",
            Self::Stueck => "STUECK",
            Self::Kubikmeter => "KUBIKMETER",
            Self::Sekunde => "SEKUNDE",
            Self::Minute => "MINUTE",
            Self::Stunde => "STUNDE",
            Self::ViertelStunde => "VIERTEL_STUNDE",
            Self::Tag => "TAG",
            Self::Woche => "WOCHE",
            Self::Monat => "MONAT",
            Self::Quartal => "QUARTAL",
            Self::Halbjahr => "HALBJAHR",
            Self::Jahr => "JAHR",
            Self::Prozent => "PROZENT",
            Self::Kvar => "KVAR",
            Self::Kwhk => "KWHK",
            Self::Var => "VAR",
            Self::Varh => "VARH",
            Self::Unknown => "UNKNOWN",
        }
    }
    /// **Strictly** parses a BO4E wire string into a known variant.
    ///
    /// Unlike the lenient `serde` / [`FromStr`](std::str::FromStr) path — which maps
    /// any unrecognized value (a typo, a legacy code, or a value from a newer schema)
    /// to [`Mengeneinheit::Unknown`] — this returns
    /// [`Err`](crate::error::UnknownVariant) for values not defined in this schema
    /// version, including the literal `"UNKNOWN"`.  Use it at the ingest boundary to
    /// reject bad values instead of silently degrading them.
    ///
    /// # Example
    /// ```rust,ignore
    /// assert!(Mengeneinheit::from_wire("NOT_A_REAL_VALUE").is_err());
    /// ```
    pub fn from_wire(s: &str) -> Result<Self, crate::error::UnknownVariant> {
        match s {
            "W" => Ok(Self::W),
            "WH" => Ok(Self::Wh),
            "KW" => Ok(Self::Kw),
            "KWH" => Ok(Self::Kwh),
            "KVARH" => Ok(Self::Kvarh),
            "MW" => Ok(Self::Mw),
            "MWH" => Ok(Self::Mwh),
            "STUECK" => Ok(Self::Stueck),
            "KUBIKMETER" => Ok(Self::Kubikmeter),
            "SEKUNDE" => Ok(Self::Sekunde),
            "MINUTE" => Ok(Self::Minute),
            "STUNDE" => Ok(Self::Stunde),
            "VIERTEL_STUNDE" => Ok(Self::ViertelStunde),
            "TAG" => Ok(Self::Tag),
            "WOCHE" => Ok(Self::Woche),
            "MONAT" => Ok(Self::Monat),
            "QUARTAL" => Ok(Self::Quartal),
            "HALBJAHR" => Ok(Self::Halbjahr),
            "JAHR" => Ok(Self::Jahr),
            "PROZENT" => Ok(Self::Prozent),
            "KVAR" => Ok(Self::Kvar),
            "KWHK" => Ok(Self::Kwhk),
            "VAR" => Ok(Self::Var),
            "VARH" => Ok(Self::Varh),
            other => Err(crate::error::UnknownVariant::new(other)),
        }
    }
    /// Returns `true` if this value is the forward-compatibility
    /// [`Mengeneinheit::Unknown`] catch-all (an out-of-schema value).
    pub const fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
    /// Returns `true` if this value is a known, schema-defined variant.
    pub const fn is_known(&self) -> bool {
        !self.is_unknown()
    }
}
impl std::fmt::Display for Mengeneinheit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_wire())
    }
}
impl AsRef<str> for Mengeneinheit {
    fn as_ref(&self) -> &str {
        self.as_wire()
    }
}
#[cfg(feature = "versioned")]
impl crate::bo4e_enum_sealed::Sealed for Mengeneinheit {}
#[cfg(feature = "versioned")]
impl crate::Bo4eEnum for Mengeneinheit {
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
impl crate::Bo4eStrict for Mengeneinheit {
    fn collect_unknown_enums(&self, path: &str, out: &mut Vec<String>) {
        if self.is_unknown() {
            out.push(path.to_owned());
        }
    }
}
#[cfg(all(feature = "sqlx", feature = "json"))]
impl sqlx::Type<sqlx::Postgres> for Mengeneinheit {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
/// Encode via the canonical wire string (`as_wire`, always available) — no
/// `serde_json::Value` intermediate, saving an allocation per encode (M-07).
#[cfg(all(feature = "sqlx", feature = "json"))]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Mengeneinheit {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        let s: &str = self.as_wire();
        <&str as sqlx::Encode<'q, sqlx::Postgres>>::encode_by_ref(&s, buf)
    }
}
#[cfg(all(feature = "sqlx", feature = "json"))]
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for Mengeneinheit {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <String as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        serde_json::from_value(serde_json::Value::String(s))
            .map_err(|e| Box::new(e) as sqlx::error::BoxDynError)
    }
}
#[cfg(test)]
impl proptest::arbitrary::Arbitrary for Mengeneinheit {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        proptest::sample::select(Self::VARIANTS.to_vec()).boxed()
    }
}
