//! Schema helper functions for schemars integration.
//!
//! These are referenced from generated code via `#[schemars(schema_with = "...")]`
//! to provide richer JSON Schema annotations for types that schemars 1.x does
//! not natively support (notably `time::OffsetDateTime` and `time::Date`).

/// Returns a JSON Schema representing an ISO 8601 / RFC 3339 date-time string.
///
/// Equivalent to `{ "type": "string", "format": "date-time" }`.
///
/// Used as `#[schemars(schema_with = "crate::schema_helpers::datetime_schema")]`
/// on required `time::OffsetDateTime` fields in generated structs.
pub fn datetime_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
    schemars::json_schema!({
        "type": "string",
        "format": "date-time"
    })
}

/// Returns a JSON Schema representing a nullable ISO 8601 / RFC 3339 date-time string.
///
/// Equivalent to `{ "type": ["string", "null"], "format": "date-time" }`.
///
/// Used as `#[schemars(schema_with = "crate::schema_helpers::opt_datetime_schema")]`
/// on `Option<time::OffsetDateTime>` fields in generated structs.
pub fn opt_datetime_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
    schemars::json_schema!({
        "type": ["string", "null"],
        "format": "date-time"
    })
}

/// Returns a JSON Schema representing an ISO 8601 date-only string (`YYYY-MM-DD`).
///
/// Equivalent to `{ "type": "string", "format": "date" }`.
///
/// Used as `#[schemars(schema_with = "crate::schema_helpers::date_schema")]`
/// on required `time::Date` fields in generated structs.
pub fn date_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
    schemars::json_schema!({
        "type": "string",
        "format": "date"
    })
}

/// Returns a JSON Schema representing a nullable ISO 8601 date-only string (`YYYY-MM-DD`).
///
/// Equivalent to `{ "type": ["string", "null"], "format": "date" }`.
///
/// Used as `#[schemars(schema_with = "crate::schema_helpers::opt_date_schema")]`
/// on `Option<time::Date>` fields in generated structs.
pub fn opt_date_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
    schemars::json_schema!({
        "type": ["string", "null"],
        "format": "date"
    })
}

// ── Identifier schemas ────────────────────────────────────────────────────────
//
// Every one of these is the same three facts, read out of
// `crate::identifiers::schema`. They stay separate functions only because
// `#[schemars(schema_with = "…")]` names a path, not a value.
//
// The `description` each one writes is then *overwritten* by the derive, which
// merges the type's rustdoc in — which is why every identifier also carries
// `#[schemars(description = …)]` pointing at the same const. See
// `crate::identifiers::schema`.

use crate::identifiers::schema::IdentifierSchema;

/// Builds the JSON Schema for one identifier from its entry in the shared table.
fn identifier_schema(meta: &IdentifierSchema) -> schemars::Schema {
    schemars::json_schema!({
        "type": "string",
        "pattern": meta.pattern,
        "description": meta.description,
        "examples": [meta.example]
    })
}

/// JSON Schema for [`MaloId`](crate::identifiers::MaloId).
pub fn malo_id_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
    identifier_schema(&crate::identifiers::schema::MALO_ID)
}

/// JSON Schema for [`MarktpartnerId`](crate::identifiers::MarktpartnerId).
pub fn marktpartner_id_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
    identifier_schema(&crate::identifiers::schema::MARKTPARTNER_ID)
}

/// JSON Schema for [`Lokationsbuendelcode`](crate::identifiers::Lokationsbuendelcode).
pub fn lokationsbuendel_code_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
    identifier_schema(&crate::identifiers::schema::LOKATIONSBUENDEL_CODE)
}

/// JSON Schema for [`LokationsbuendelObjektcode`](crate::identifiers::LokationsbuendelObjektcode).
pub fn lokationsbuendel_objektcode_schema(
    _gen: &mut schemars::SchemaGenerator,
) -> schemars::Schema {
    identifier_schema(&crate::identifiers::schema::LOKATIONSBUENDEL_OBJEKTCODE)
}

/// JSON Schema for [`MeloId`](crate::identifiers::MeloId).
pub fn melo_id_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
    identifier_schema(&crate::identifiers::schema::MELO_ID)
}

/// JSON Schema for [`Zaehlpunktbezeichnung`](crate::identifiers::Zaehlpunktbezeichnung).
pub fn zaehlpunktbezeichnung_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
    identifier_schema(&crate::identifiers::schema::ZAEHLPUNKTBEZEICHNUNG)
}

/// JSON Schema for [`NeloId`](crate::identifiers::NeloId).
pub fn nelo_id_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
    identifier_schema(&crate::identifiers::schema::NELO_ID)
}

/// JSON Schema for [`NebeId`](crate::identifiers::NebeId).
pub fn nebe_id_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
    identifier_schema(&crate::identifiers::schema::NEBE_ID)
}

/// JSON Schema for [`CrId`](crate::identifiers::CrId).
pub fn cr_id_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
    identifier_schema(&crate::identifiers::schema::CR_ID)
}

/// JSON Schema for [`SgId`](crate::identifiers::SgId).
pub fn sg_id_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
    identifier_schema(&crate::identifiers::schema::SG_ID)
}

/// JSON Schema for [`SrId`](crate::identifiers::SrId).
pub fn sr_id_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
    identifier_schema(&crate::identifiers::schema::SR_ID)
}

/// JSON Schema for [`TrId`](crate::identifiers::TrId).
pub fn tr_id_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
    identifier_schema(&crate::identifiers::schema::TR_ID)
}

/// JSON Schema for [`PaketId`](crate::identifiers::PaketId).
pub fn paket_id_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
    identifier_schema(&crate::identifiers::schema::PAKET_ID)
}

/// JSON Schema for [`EicCode`](crate::identifiers::EicCode).
pub fn eic_code_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
    identifier_schema(&crate::identifiers::schema::EIC_CODE)
}

/// JSON Schema for [`BilanzkreisId`](crate::identifiers::BilanzkreisId).
pub fn bilanzkreis_id_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
    identifier_schema(&crate::identifiers::schema::BILANZKREIS_ID)
}

/// JSON Schema for [`BilanzierungsgebietId`](crate::identifiers::BilanzierungsgebietId).
pub fn bilanzierungsgebiet_id_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
    identifier_schema(&crate::identifiers::schema::BILANZIERUNGSGEBIET_ID)
}

/// JSON Schema for [`AkivId`](crate::identifiers::AkivId).
pub fn akiv_id_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
    identifier_schema(&crate::identifiers::schema::AKIV_ID)
}

/// JSON Schema for [`TranchennummerId`](crate::identifiers::TranchennummerId).
pub fn tranchennummer_id_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
    identifier_schema(&crate::identifiers::schema::TRANCHENNUMMER_ID)
}

/// JSON Schema for [`Iban`](crate::identifiers::Iban).
pub fn iban_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
    identifier_schema(&crate::identifiers::schema::IBAN)
}

/// JSON Schema for [`Bic`](crate::identifiers::Bic).
pub fn bic_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
    identifier_schema(&crate::identifiers::schema::BIC)
}

/// JSON Schema for [`ObisCode`](crate::identifiers::ObisCode).
pub fn obis_code_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
    identifier_schema(&crate::identifiers::schema::OBIS_CODE)
}
