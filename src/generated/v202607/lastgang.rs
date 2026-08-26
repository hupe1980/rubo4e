use super::{
    Bo4eObject, BoTyp, Marktlokation, Menge, Mengeneinheit, Messlokation, Sparte, Zeitreihenwert,
    ZusatzAttribut,
};
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(not(feature = "json"), derive(Eq, Hash))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "validate", derive(garde::Validate))]
#[cfg_attr(feature = "validate", garde(allow_unvalidated))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Modell zur Abbildung eines Lastganges;
/// In diesem Modell werden die Messwerte mit einem vollständigen Zeitintervall (zeit_intervall_laenge) angegeben und es bietet daher eine hohe Flexibilität in der Übertragung jeglicher zeitlich veränderlicher Messgrössen.
///
/// > **Note:** [Lastgang JSON Schema](https://json-schema.app/view/%23?url=https://raw.githubusercontent.com/BO4E/BO4E-Schemas/v202607.1.0/src/bo4e_schemas/bo/Lastgang.json)
pub struct Lastgang {
    /// Eine generische ID, die für eigene Zwecke genutzt werden kann.
    /// Z.B. könnten hier UUIDs aus einer Datenbank stehen oder URLs zu einem Backend-System.
    #[cfg_attr(feature = "serde", serde(rename = "_id"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub id: Option<String>,
    #[cfg_attr(feature = "serde", serde(rename = "marktlokation"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub marktlokation: Option<Box<Marktlokation>>,
    /// Definition der gemessenen Größe anhand ihrer Einheit
    #[cfg_attr(feature = "serde", serde(rename = "messgroesse"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub messgroesse: Option<Mengeneinheit>,
    #[cfg_attr(feature = "serde", serde(rename = "messlokation"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub messlokation: Option<Box<Messlokation>>,
    /// Die OBIS-Kennzahl für den Wert, die festlegt, welche Größe mit dem Stand gemeldet wird, z.B. '1-0:1.8.1'
    #[cfg_attr(feature = "serde", serde(rename = "obisKennzahl"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub obis_kennzahl: Option<crate::identifiers::ObisCode>,
    /// Angabe, ob es sich um einen Gas- oder Stromlastgang handelt
    #[cfg_attr(feature = "serde", serde(rename = "sparte"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub sparte: Option<Sparte>,
    /// BO4E type discriminant — always `BoTyp::Lastgang` for this struct.
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
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub version: Option<String>,
    /// Die im Lastgang enthaltenen Messwerte
    #[cfg_attr(feature = "serde", serde(rename = "werte"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub werte: Option<Vec<Zeitreihenwert>>,
    #[cfg_attr(feature = "serde", serde(rename = "zeitIntervallLaenge"))]
    #[cfg_attr(feature = "validate", garde(dive))]
    pub zeit_intervall_laenge: Menge,
    #[cfg_attr(feature = "serde", serde(rename = "zusatzAttribute"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "validate", garde(dive))]
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
    #[doc(hidden)]
    pub _additional: crate::LimitedExtensionMap,
}
impl Lastgang {
    /// Creates a `Lastgang` from the one field the BO4E schema marks `required`,
    /// defaulting every other field.
    ///
    /// `Lastgang` has no [`Default`]: `zeit_intervall_laenge` is required, and its
    /// type need not implement `Default` — so this is the
    /// `..Default::default()` stand-in.
    /// `_typ` is stamped exactly as elsewhere.
    #[must_use]
    pub fn new(zeit_intervall_laenge: Menge) -> Self {
        Self {
            id: Default::default(),
            marktlokation: Default::default(),
            messgroesse: Default::default(),
            messlokation: Default::default(),
            obis_kennzahl: Default::default(),
            sparte: Default::default(),
            typ: Some(BoTyp::Lastgang),
            version: Default::default(),
            werte: Default::default(),
            zeit_intervall_laenge,
            zusatz_attribute: Default::default(),
            _additional: Default::default(),
        }
    }
}
impl Bo4eObject for Lastgang {
    type BoTyp = BoTyp;
    const BO_TYP: BoTyp = BoTyp::Lastgang;
    const TYP_WIRE: &'static str = "LASTGANG";
    const SCHEMA_VERSION: &'static str = "202607.1.0";
    const SCHEMA_SERIES: &'static str = "202607";
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
impl crate::Bo4eStrict for Lastgang {
    fn collect_unknown_enums(&self, path: &str, out: &mut Vec<String>) {
        if let Some(v) = &self.marktlokation {
            crate::Bo4eStrict::collect_unknown_enums(
                &**v,
                &crate::strict::field_path(path, "marktlokation"),
                out,
            );
        }
        if let Some(v) = &self.messgroesse {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "messgroesse"),
                out,
            );
        }
        if let Some(v) = &self.messlokation {
            crate::Bo4eStrict::collect_unknown_enums(
                &**v,
                &crate::strict::field_path(path, "messlokation"),
                out,
            );
        }
        if let Some(v) = &self.sparte {
            crate::Bo4eStrict::collect_unknown_enums(
                v,
                &crate::strict::field_path(path, "sparte"),
                out,
            );
        }
        if let Some(items) = &self.werte {
            let child = crate::strict::field_path(path, "werte");
            for (i, item) in items.iter().enumerate() {
                crate::Bo4eStrict::collect_unknown_enums(
                    item,
                    &crate::strict::index_path(&child, i),
                    out,
                );
            }
        }
        crate::Bo4eStrict::collect_unknown_enums(
            &self.zeit_intervall_laenge,
            &crate::strict::field_path(path, "zeitIntervallLaenge"),
            out,
        );
        if let Some(items) = &self.zusatz_attribute {
            let child = crate::strict::field_path(path, "zusatzAttribute");
            for (i, item) in items.iter().enumerate() {
                crate::Bo4eStrict::collect_unknown_enums(
                    item,
                    &crate::strict::index_path(&child, i),
                    out,
                );
            }
        }
    }
}
