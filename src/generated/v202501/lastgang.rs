use super::{
    Bo4eObject, BoTyp, Marktlokation, Menge, Mengeneinheit, Messlokation, Sparte, Zeitreihenwert,
    ZusatzAttribut,
};
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(not(feature = "json"), derive(Hash))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "validate", derive(garde::Validate))]
#[cfg_attr(feature = "validate", garde(allow_unvalidated))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Modell zur Abbildung eines Lastganges;
/// In diesem Modell werden die Messwerte mit einem vollständigen Zeitintervall (zeit_intervall_laenge) angegeben und es bietet daher eine hohe Flexibilität in der Übertragung jeglicher zeitlich veränderlicher Messgrössen.
///
/// > **Note:** [Lastgang JSON Schema](https://json-schema.app/view/%23?url=https://raw.githubusercontent.com/BO4E/BO4E-Schemas/v202501.0.0/src/bo4e_schemas/bo/Lastgang.json)
pub struct Lastgang {
    /// Eine generische ID, die für eigene Zwecke genutzt werden kann.
    /// Z.B. könnten hier UUIDs aus einer Datenbank stehen oder URLs zu einem Backend-System.
    #[cfg_attr(feature = "serde", serde(rename = "_id"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub id: Option<String>,
    #[cfg_attr(feature = "serde", serde(rename = "marktlokation"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub marktlokation: Option<Box<Marktlokation>>,
    /// Definition der gemessenen Größe anhand ihrer Einheit
    #[cfg_attr(feature = "serde", serde(rename = "messgroesse"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub messgroesse: Option<Mengeneinheit>,
    #[cfg_attr(feature = "serde", serde(rename = "messlokation"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub messlokation: Option<Box<Messlokation>>,
    /// Die OBIS-Kennzahl für den Wert, die festlegt, welche Größe mit dem Stand gemeldet wird, z.B. '1-0:1.8.1'
    #[cfg_attr(feature = "serde", serde(rename = "obisKennzahl"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub obis_kennzahl: Option<crate::identifiers::ObisCode>,
    /// Angabe, ob es sich um einen Gas- oder Stromlastgang handelt
    #[cfg_attr(feature = "serde", serde(rename = "sparte"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub sparte: Option<Sparte>,
    /// BO type identifier — always `BoTyp::Lastgang` for this struct.
    #[cfg_attr(feature = "serde", serde(rename = "_typ"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(
        feature = "builder",
        builder(default = Some(BoTyp::Lastgang), setter(skip))
    )]
    pub typ: Option<BoTyp>,
    /// Versionsnummer des Lastgangs
    #[cfg_attr(feature = "serde", serde(rename = "version"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub version: Option<String>,
    /// Die im Lastgang enthaltenen Messwerte
    #[cfg_attr(feature = "serde", serde(rename = "werte"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub werte: Option<Vec<Zeitreihenwert>>,
    #[cfg_attr(feature = "serde", serde(rename = "zeitIntervallLaenge"))]
    pub zeit_intervall_laenge: Menge,
    #[cfg_attr(feature = "serde", serde(rename = "zusatzAttribute"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub zusatz_attribute: Option<Vec<ZusatzAttribut>>,
    /// Unknown JSON fields captured during deserialization for round-trip preservation.
    /// `None` when no unknown fields were present (zero heap allocation).
    #[cfg_attr(feature = "json", serde(flatten))]
    #[cfg_attr(
        feature = "json",
        serde(skip_serializing_if = "crate::json::ext_map_is_empty")
    )]
    #[cfg_attr(not(feature = "json"), serde(skip))]
    #[cfg_attr(feature = "builder", builder(default, setter(skip)))]
    pub(crate) _additional: crate::LimitedExtensionMap,
}
impl Default for Lastgang {
    fn default() -> Self {
        Self {
            typ: Some(BoTyp::Lastgang),
            id: Default::default(),
            marktlokation: Default::default(),
            messgroesse: Default::default(),
            messlokation: Default::default(),
            obis_kennzahl: Default::default(),
            sparte: Default::default(),
            version: Default::default(),
            werte: Default::default(),
            zeit_intervall_laenge: Default::default(),
            zusatz_attribute: Default::default(),
            _additional: Default::default(),
        }
    }
}
impl Bo4eObject for Lastgang {
    type BoTyp = BoTyp;
    fn bo_type(&self) -> BoTyp {
        self.typ.unwrap_or(BoTyp::Lastgang)
    }
    fn schema_version(&self) -> &'static str {
        "v202501.0.0"
    }
}
#[cfg(feature = "json")]
impl crate::json::sealed::Sealed for Lastgang {}
#[cfg(feature = "json")]
impl crate::json::Bo4eJsonExt for Lastgang {}
#[cfg(feature = "json")]
impl crate::json::Bo4eExtensionData for Lastgang {
    fn extension_data(&self) -> &indexmap::IndexMap<String, serde_json::Value> {
        self._additional
            .as_map()
            .unwrap_or(&crate::json::extension::EMPTY_EXTENSION_MAP)
    }
    fn has_extension_data(&self) -> bool {
        !self._additional.is_empty()
    }
}
#[cfg(feature = "json")]
impl std::fmt::Display for Lastgang {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => f.write_str(&json),
            Err(e) => write!(f, "<Lastgang: serialization error: {e}>"),
        }
    }
}
