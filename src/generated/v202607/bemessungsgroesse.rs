#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(strum::EnumString, strum::EnumIter, strum::IntoStaticStr)
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Zur Abbildung von Messgrössen und zur Verwendung in energiewirtschaftlichen Berechnungen.
#[non_exhaustive]
pub enum Bemessungsgroesse {
    #[cfg_attr(feature = "serde", serde(rename = "WIRKARBEIT_EL"))]
    #[cfg_attr(feature = "strum", strum(serialize = "WIRKARBEIT_EL"))]
    WirkarbeitEl,
    #[cfg_attr(feature = "serde", serde(rename = "LEISTUNG_EL"))]
    #[cfg_attr(feature = "strum", strum(serialize = "LEISTUNG_EL"))]
    LeistungEl,
    #[cfg_attr(feature = "serde", serde(rename = "BLINDARBEIT_KAP"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BLINDARBEIT_KAP"))]
    BlindarbeitKap,
    #[cfg_attr(feature = "serde", serde(rename = "BLINDARBEIT_IND"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BLINDARBEIT_IND"))]
    BlindarbeitInd,
    #[cfg_attr(feature = "serde", serde(rename = "BLINDLEISTUNG_KAP"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BLINDLEISTUNG_KAP"))]
    BlindleistungKap,
    #[cfg_attr(feature = "serde", serde(rename = "BLINDLEISTUNG_IND"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BLINDLEISTUNG_IND"))]
    BlindleistungInd,
    #[cfg_attr(feature = "serde", serde(rename = "WIRKARBEIT_TH"))]
    #[cfg_attr(feature = "strum", strum(serialize = "WIRKARBEIT_TH"))]
    WirkarbeitTh,
    #[cfg_attr(feature = "serde", serde(rename = "LEISTUNG_TH"))]
    #[cfg_attr(feature = "strum", strum(serialize = "LEISTUNG_TH"))]
    LeistungTh,
    #[cfg_attr(feature = "serde", serde(rename = "VOLUMEN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "VOLUMEN"))]
    Volumen,
    #[cfg_attr(feature = "serde", serde(rename = "VOLUMENSTROM"))]
    #[cfg_attr(feature = "strum", strum(serialize = "VOLUMENSTROM"))]
    Volumenstrom,
    #[cfg_attr(feature = "serde", serde(rename = "BENUTZUNGSDAUER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BENUTZUNGSDAUER"))]
    Benutzungsdauer,
    #[cfg_attr(feature = "serde", serde(rename = "ANZAHL"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ANZAHL"))]
    Anzahl,
    /// Unknown or future variant — produced when deserializing a value
    /// that is not yet known to this version of the library.
    #[cfg_attr(feature = "serde", serde(other, rename = "UNKNOWN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UNKNOWN"))]
    Unknown,
}
impl Bemessungsgroesse {
    /// All variants defined by the BO4E schema, in declaration order.
    ///
    /// Excludes the forward-compatibility [`Bemessungsgroesse::Unknown`] catch-all, so this
    /// is exactly the set of values that appear on the wire.  Available **without**
    /// the `strum` feature — use it to drift-guard SQL `CHECK` lists and mappings.
    pub const VARIANTS: &'static [Self] = &[
        Self::WirkarbeitEl,
        Self::LeistungEl,
        Self::BlindarbeitKap,
        Self::BlindarbeitInd,
        Self::BlindleistungKap,
        Self::BlindleistungInd,
        Self::WirkarbeitTh,
        Self::LeistungTh,
        Self::Volumen,
        Self::Volumenstrom,
        Self::Benutzungsdauer,
        Self::Anzahl,
    ];
    /// Number of schema-defined variants (equal to `VARIANTS.len()`), excluding the
    /// [`Bemessungsgroesse::Unknown`] catch-all.  Stable for this schema version.
    pub const COUNT: usize = Self::VARIANTS.len();
    /// Returns an iterator over all **known** variants of `Bemessungsgroesse`.
    ///
    /// Yields only variants that correspond to values defined in the BO4E schema
    /// (i.e. [`Self::VARIANTS`]), never the [`Bemessungsgroesse::Unknown`] catch-all.
    /// Available **without** the `strum` feature.
    ///
    /// # Example
    /// ```rust,ignore
    /// for v in Bemessungsgroesse::iter_known() {
    ///     println!("{}", v.as_wire());
    /// }
    /// ```
    pub fn iter_known() -> impl Iterator<Item = Self> + Clone {
        Self::VARIANTS.iter().copied()
    }
    /// Returns the canonical BO4E wire string (SCREAMING_SNAKE_CASE) for this value.
    ///
    /// [`Bemessungsgroesse::Unknown`] renders as `"UNKNOWN"`, matching its serialized form.
    pub const fn as_wire(&self) -> &'static str {
        match self {
            Self::WirkarbeitEl => "WIRKARBEIT_EL",
            Self::LeistungEl => "LEISTUNG_EL",
            Self::BlindarbeitKap => "BLINDARBEIT_KAP",
            Self::BlindarbeitInd => "BLINDARBEIT_IND",
            Self::BlindleistungKap => "BLINDLEISTUNG_KAP",
            Self::BlindleistungInd => "BLINDLEISTUNG_IND",
            Self::WirkarbeitTh => "WIRKARBEIT_TH",
            Self::LeistungTh => "LEISTUNG_TH",
            Self::Volumen => "VOLUMEN",
            Self::Volumenstrom => "VOLUMENSTROM",
            Self::Benutzungsdauer => "BENUTZUNGSDAUER",
            Self::Anzahl => "ANZAHL",
            Self::Unknown => "UNKNOWN",
        }
    }
    /// **Strictly** parses a BO4E wire string into a known variant.
    ///
    /// Unlike the lenient `serde` / [`FromStr`](std::str::FromStr) path — which maps
    /// any unrecognized value (a typo, a legacy code, or a value from a newer schema)
    /// to [`Bemessungsgroesse::Unknown`] — this returns
    /// [`Err`](crate::error::UnknownVariant) for values not defined in this schema
    /// version, including the literal `"UNKNOWN"`.  Use it at the ingest boundary to
    /// reject bad values instead of silently degrading them.
    ///
    /// # Example
    /// ```rust,ignore
    /// assert!(Bemessungsgroesse::from_wire("NOT_A_REAL_VALUE").is_err());
    /// ```
    pub fn from_wire(s: &str) -> Result<Self, crate::error::UnknownVariant> {
        match s {
            "WIRKARBEIT_EL" => Ok(Self::WirkarbeitEl),
            "LEISTUNG_EL" => Ok(Self::LeistungEl),
            "BLINDARBEIT_KAP" => Ok(Self::BlindarbeitKap),
            "BLINDARBEIT_IND" => Ok(Self::BlindarbeitInd),
            "BLINDLEISTUNG_KAP" => Ok(Self::BlindleistungKap),
            "BLINDLEISTUNG_IND" => Ok(Self::BlindleistungInd),
            "WIRKARBEIT_TH" => Ok(Self::WirkarbeitTh),
            "LEISTUNG_TH" => Ok(Self::LeistungTh),
            "VOLUMEN" => Ok(Self::Volumen),
            "VOLUMENSTROM" => Ok(Self::Volumenstrom),
            "BENUTZUNGSDAUER" => Ok(Self::Benutzungsdauer),
            "ANZAHL" => Ok(Self::Anzahl),
            other => Err(crate::error::UnknownVariant::new(other)),
        }
    }
    /// Returns `true` if this value is the forward-compatibility
    /// [`Bemessungsgroesse::Unknown`] catch-all (an out-of-schema value).
    pub const fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
    /// Returns `true` if this value is a known, schema-defined variant.
    pub const fn is_known(&self) -> bool {
        !self.is_unknown()
    }
}
impl std::fmt::Display for Bemessungsgroesse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_wire())
    }
}
impl AsRef<str> for Bemessungsgroesse {
    fn as_ref(&self) -> &str {
        self.as_wire()
    }
}
#[cfg(feature = "versioned")]
impl crate::bo4e_enum_sealed::Sealed for Bemessungsgroesse {}
#[cfg(feature = "versioned")]
impl crate::Bo4eEnum for Bemessungsgroesse {
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
impl crate::Bo4eStrict for Bemessungsgroesse {
    fn collect_unknown_enums(&self, path: &str, out: &mut Vec<String>) {
        if self.is_unknown() {
            out.push(path.to_owned());
        }
    }
}
#[cfg(all(feature = "sqlx", feature = "json"))]
impl sqlx::Type<sqlx::Postgres> for Bemessungsgroesse {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
/// Encode via the canonical wire string (`as_wire`, always available) — no
/// `serde_json::Value` intermediate, saving an allocation per encode (M-07).
#[cfg(all(feature = "sqlx", feature = "json"))]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Bemessungsgroesse {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        let s: &str = self.as_wire();
        <&str as sqlx::Encode<'q, sqlx::Postgres>>::encode_by_ref(&s, buf)
    }
}
#[cfg(all(feature = "sqlx", feature = "json"))]
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for Bemessungsgroesse {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <String as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        serde_json::from_value(serde_json::Value::String(s))
            .map_err(|e| Box::new(e) as sqlx::error::BoxDynError)
    }
}
#[cfg(test)]
impl proptest::arbitrary::Arbitrary for Bemessungsgroesse {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        proptest::sample::select(Self::VARIANTS.to_vec()).boxed()
    }
}
