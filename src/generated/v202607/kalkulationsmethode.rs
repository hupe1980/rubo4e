#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(strum::EnumString, strum::EnumIter, strum::IntoStaticStr)
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Auflistung der verschiedenen Berechnungsmethoden für ein Preisblatt.
#[non_exhaustive]
pub enum Kalkulationsmethode {
    #[cfg_attr(feature = "serde", serde(rename = "STUFEN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "STUFEN"))]
    Stufen,
    #[cfg_attr(feature = "serde", serde(rename = "ZONEN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ZONEN"))]
    Zonen,
    #[cfg_attr(feature = "serde", serde(rename = "VORZONEN_GP"))]
    #[cfg_attr(feature = "strum", strum(serialize = "VORZONEN_GP"))]
    VorzonenGp,
    #[cfg_attr(feature = "serde", serde(rename = "SIGMOID"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SIGMOID"))]
    Sigmoid,
    #[cfg_attr(feature = "serde", serde(rename = "BLINDARBEIT_GT_50_PROZENT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BLINDARBEIT_GT_50_PROZENT"))]
    BlindarbeitGt50Prozent,
    #[cfg_attr(feature = "serde", serde(rename = "BLINDARBEIT_GT_40_PROZENT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BLINDARBEIT_GT_40_PROZENT"))]
    BlindarbeitGt40Prozent,
    #[cfg_attr(feature = "serde", serde(rename = "BLINDARBEIT_MIT_FREIMENGE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BLINDARBEIT_MIT_FREIMENGE"))]
    BlindarbeitMitFreimenge,
    #[cfg_attr(feature = "serde", serde(rename = "AP_GP_ZONEN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "AP_GP_ZONEN"))]
    ApGpZonen,
    #[cfg_attr(feature = "serde", serde(rename = "LP_INSTALL_LEISTUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "LP_INSTALL_LEISTUNG"))]
    LpInstallLeistung,
    #[cfg_attr(feature = "serde", serde(rename = "AP_TRANSPORT_ODER_VERTEILNETZ"))]
    #[cfg_attr(feature = "strum", strum(serialize = "AP_TRANSPORT_ODER_VERTEILNETZ"))]
    ApTransportOderVerteilnetz,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "AP_TRANSPORT_ODER_VERTEILNETZ_ORTSVERTEILNETZ_SIGMOID")
    )]
    #[cfg_attr(
        feature = "strum",
        strum(serialize = "AP_TRANSPORT_ODER_VERTEILNETZ_ORTSVERTEILNETZ_SIGMOID")
    )]
    ApTransportOderVerteilnetzOrtsverteilnetzSigmoid,
    #[cfg_attr(feature = "serde", serde(rename = "LP_JAHRESVERBRAUCH"))]
    #[cfg_attr(feature = "strum", strum(serialize = "LP_JAHRESVERBRAUCH"))]
    LpJahresverbrauch,
    #[cfg_attr(feature = "serde", serde(rename = "LP_TRANSPORT_ODER_VERTEILNETZ"))]
    #[cfg_attr(feature = "strum", strum(serialize = "LP_TRANSPORT_ODER_VERTEILNETZ"))]
    LpTransportOderVerteilnetz,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "LP_TRANSPORT_ODER_VERTEILNETZ_ORTSVERTEILNETZ_SIGMOID")
    )]
    #[cfg_attr(
        feature = "strum",
        strum(serialize = "LP_TRANSPORT_ODER_VERTEILNETZ_ORTSVERTEILNETZ_SIGMOID")
    )]
    LpTransportOderVerteilnetzOrtsverteilnetzSigmoid,
    #[cfg_attr(feature = "serde", serde(rename = "FUNKTIONEN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "FUNKTIONEN"))]
    Funktionen,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "VERBRAUCH_UEBER_SLP_GRENZE_FUNKTIONSBEZOGEN_WEITERE_BERECHNUNG_ALS_LGK")
    )]
    #[cfg_attr(
        feature = "strum",
        strum(
            serialize = "VERBRAUCH_UEBER_SLP_GRENZE_FUNKTIONSBEZOGEN_WEITERE_BERECHNUNG_ALS_LGK"
        )
    )]
    VerbrauchUeberSlpGrenzeFunktionsbezogenWeitereBerechnungAlsLgk,
    /// Unknown or future variant — produced when deserializing a value
    /// that is not yet known to this version of the library.
    #[cfg_attr(feature = "serde", serde(other, rename = "UNKNOWN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UNKNOWN"))]
    Unknown,
}
impl Kalkulationsmethode {
    /// All variants defined by the BO4E schema, in declaration order.
    ///
    /// Excludes the forward-compatibility [`Kalkulationsmethode::Unknown`] catch-all, so this
    /// is exactly the set of values that appear on the wire.  Available **without**
    /// the `strum` feature — use it to drift-guard SQL `CHECK` lists and mappings.
    pub const VARIANTS: &'static [Self] = &[
        Self::Stufen,
        Self::Zonen,
        Self::VorzonenGp,
        Self::Sigmoid,
        Self::BlindarbeitGt50Prozent,
        Self::BlindarbeitGt40Prozent,
        Self::BlindarbeitMitFreimenge,
        Self::ApGpZonen,
        Self::LpInstallLeistung,
        Self::ApTransportOderVerteilnetz,
        Self::ApTransportOderVerteilnetzOrtsverteilnetzSigmoid,
        Self::LpJahresverbrauch,
        Self::LpTransportOderVerteilnetz,
        Self::LpTransportOderVerteilnetzOrtsverteilnetzSigmoid,
        Self::Funktionen,
        Self::VerbrauchUeberSlpGrenzeFunktionsbezogenWeitereBerechnungAlsLgk,
    ];
    /// Number of schema-defined variants (equal to `VARIANTS.len()`), excluding the
    /// [`Kalkulationsmethode::Unknown`] catch-all.  Stable for this schema version.
    pub const COUNT: usize = Self::VARIANTS.len();
    /// Returns an iterator over all **known** variants of `Kalkulationsmethode`.
    ///
    /// Yields only variants that correspond to values defined in the BO4E schema
    /// (i.e. [`Self::VARIANTS`]), never the [`Kalkulationsmethode::Unknown`] catch-all.
    /// Available **without** the `strum` feature.
    ///
    /// # Example
    /// ```rust,ignore
    /// for v in Kalkulationsmethode::iter_known() {
    ///     println!("{}", v.as_wire());
    /// }
    /// ```
    pub fn iter_known() -> impl Iterator<Item = Self> + Clone {
        Self::VARIANTS.iter().copied()
    }
    /// Returns the canonical BO4E wire string (SCREAMING_SNAKE_CASE) for this value.
    ///
    /// [`Kalkulationsmethode::Unknown`] renders as `"UNKNOWN"`, matching its serialized form.
    pub const fn as_wire(&self) -> &'static str {
        match self {
            Self::Stufen => "STUFEN",
            Self::Zonen => "ZONEN",
            Self::VorzonenGp => "VORZONEN_GP",
            Self::Sigmoid => "SIGMOID",
            Self::BlindarbeitGt50Prozent => "BLINDARBEIT_GT_50_PROZENT",
            Self::BlindarbeitGt40Prozent => "BLINDARBEIT_GT_40_PROZENT",
            Self::BlindarbeitMitFreimenge => "BLINDARBEIT_MIT_FREIMENGE",
            Self::ApGpZonen => "AP_GP_ZONEN",
            Self::LpInstallLeistung => "LP_INSTALL_LEISTUNG",
            Self::ApTransportOderVerteilnetz => "AP_TRANSPORT_ODER_VERTEILNETZ",
            Self::ApTransportOderVerteilnetzOrtsverteilnetzSigmoid => {
                "AP_TRANSPORT_ODER_VERTEILNETZ_ORTSVERTEILNETZ_SIGMOID"
            }
            Self::LpJahresverbrauch => "LP_JAHRESVERBRAUCH",
            Self::LpTransportOderVerteilnetz => "LP_TRANSPORT_ODER_VERTEILNETZ",
            Self::LpTransportOderVerteilnetzOrtsverteilnetzSigmoid => {
                "LP_TRANSPORT_ODER_VERTEILNETZ_ORTSVERTEILNETZ_SIGMOID"
            }
            Self::Funktionen => "FUNKTIONEN",
            Self::VerbrauchUeberSlpGrenzeFunktionsbezogenWeitereBerechnungAlsLgk => {
                "VERBRAUCH_UEBER_SLP_GRENZE_FUNKTIONSBEZOGEN_WEITERE_BERECHNUNG_ALS_LGK"
            }
            Self::Unknown => "UNKNOWN",
        }
    }
    /// **Strictly** parses a BO4E wire string into a known variant.
    ///
    /// Unlike the lenient `serde` / [`FromStr`](std::str::FromStr) path — which maps
    /// any unrecognized value (a typo, a legacy code, or a value from a newer schema)
    /// to [`Kalkulationsmethode::Unknown`] — this returns
    /// [`Err`](crate::error::UnknownVariant) for values not defined in this schema
    /// version, including the literal `"UNKNOWN"`.  Use it at the ingest boundary to
    /// reject bad values instead of silently degrading them.
    ///
    /// # Example
    /// ```rust,ignore
    /// assert!(Kalkulationsmethode::from_wire("NOT_A_REAL_VALUE").is_err());
    /// ```
    pub fn from_wire(s: &str) -> Result<Self, crate::error::UnknownVariant> {
        match s {
            "STUFEN" => Ok(Self::Stufen),
            "ZONEN" => Ok(Self::Zonen),
            "VORZONEN_GP" => Ok(Self::VorzonenGp),
            "SIGMOID" => Ok(Self::Sigmoid),
            "BLINDARBEIT_GT_50_PROZENT" => Ok(Self::BlindarbeitGt50Prozent),
            "BLINDARBEIT_GT_40_PROZENT" => Ok(Self::BlindarbeitGt40Prozent),
            "BLINDARBEIT_MIT_FREIMENGE" => Ok(Self::BlindarbeitMitFreimenge),
            "AP_GP_ZONEN" => Ok(Self::ApGpZonen),
            "LP_INSTALL_LEISTUNG" => Ok(Self::LpInstallLeistung),
            "AP_TRANSPORT_ODER_VERTEILNETZ" => Ok(Self::ApTransportOderVerteilnetz),
            "AP_TRANSPORT_ODER_VERTEILNETZ_ORTSVERTEILNETZ_SIGMOID" => {
                Ok(Self::ApTransportOderVerteilnetzOrtsverteilnetzSigmoid)
            }
            "LP_JAHRESVERBRAUCH" => Ok(Self::LpJahresverbrauch),
            "LP_TRANSPORT_ODER_VERTEILNETZ" => Ok(Self::LpTransportOderVerteilnetz),
            "LP_TRANSPORT_ODER_VERTEILNETZ_ORTSVERTEILNETZ_SIGMOID" => {
                Ok(Self::LpTransportOderVerteilnetzOrtsverteilnetzSigmoid)
            }
            "FUNKTIONEN" => Ok(Self::Funktionen),
            "VERBRAUCH_UEBER_SLP_GRENZE_FUNKTIONSBEZOGEN_WEITERE_BERECHNUNG_ALS_LGK" => {
                Ok(Self::VerbrauchUeberSlpGrenzeFunktionsbezogenWeitereBerechnungAlsLgk)
            }
            other => Err(crate::error::UnknownVariant::new(other)),
        }
    }
    /// Returns `true` if this value is the forward-compatibility
    /// [`Kalkulationsmethode::Unknown`] catch-all (an out-of-schema value).
    pub const fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
    /// Returns `true` if this value is a known, schema-defined variant.
    pub const fn is_known(&self) -> bool {
        !self.is_unknown()
    }
}
impl std::fmt::Display for Kalkulationsmethode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_wire())
    }
}
impl AsRef<str> for Kalkulationsmethode {
    fn as_ref(&self) -> &str {
        self.as_wire()
    }
}
#[cfg(feature = "versioned")]
impl crate::bo4e_enum_sealed::Sealed for Kalkulationsmethode {}
#[cfg(feature = "versioned")]
impl crate::Bo4eEnum for Kalkulationsmethode {
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
impl crate::Bo4eStrict for Kalkulationsmethode {
    fn collect_unknown_enums(&self, path: &str, out: &mut Vec<String>) {
        if self.is_unknown() {
            out.push(path.to_owned());
        }
    }
}
#[cfg(all(feature = "sqlx", feature = "json"))]
impl sqlx::Type<sqlx::Postgres> for Kalkulationsmethode {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
/// Encode via the canonical wire string (`as_wire`, always available) — no
/// `serde_json::Value` intermediate, saving an allocation per encode (M-07).
#[cfg(all(feature = "sqlx", feature = "json"))]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Kalkulationsmethode {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        let s: &str = self.as_wire();
        <&str as sqlx::Encode<'q, sqlx::Postgres>>::encode_by_ref(&s, buf)
    }
}
#[cfg(all(feature = "sqlx", feature = "json"))]
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for Kalkulationsmethode {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <String as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        serde_json::from_value(serde_json::Value::String(s))
            .map_err(|e| Box::new(e) as sqlx::error::BoxDynError)
    }
}
#[cfg(test)]
impl proptest::arbitrary::Arbitrary for Kalkulationsmethode {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        proptest::sample::select(Self::VARIANTS.to_vec()).boxed()
    }
}
