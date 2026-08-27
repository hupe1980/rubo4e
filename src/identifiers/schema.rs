//! What each identifier looks like on the wire, for the schema generators.
//!
//! One table, read by both the `schemars` helpers in
//! [`schema_helpers`](crate::schema_helpers) and the `utoipa` attributes on each
//! type, so a `pattern`, a `description` and an `example` are written once and
//! appear identically in the JSON Schema and the OpenAPI document.
//!
//! Neither derive does that on its own — see
//! [Ecosystem](https://hupe1980.github.io/rubo4e/docs/ecosystem/#identifier-schemas-come-from-one-table).
//! `tests/identifier_schemas.rs` is the guard.

/// The wire-format facts about one identifier, as a schema describes them.
///
/// No `minLength` / `maxLength`: every `pattern` below already pins the length.
#[derive(Debug, Clone, Copy)]
pub struct IdentifierSchema {
    /// ECMA-262 regular expression the wire form must match, anchored at both
    /// ends.
    pub pattern: &'static str,
    /// One sentence, in German — the language of the standard these identifiers
    /// come from.
    pub description: &'static str,
    /// A valid instance, checked against `pattern` **and** the type's constructor.
    pub example: &'static str,
}

// ─── §8.1 — numeric identifiers ──────────────────────────────────────────────

/// [`MaloId`](crate::identifiers::MaloId).
pub const MALO_ID: IdentifierSchema = IdentifierSchema {
    pattern: "^[1-9][0-9]{10}$",
    description: "11-stellige BDEW Marktlokations-ID: Vergabestelle (1-3 DVGW, 4-9 BDEW) + 9 Ziffern + Prüfziffer nach dem Lok- und Waggon-Kennzeichnungsverfahren (BDEW §8.1)",
    example: "41373559241",
};

/// [`MarktpartnerId`](crate::identifiers::MarktpartnerId).
pub const MARKTPARTNER_ID: IdentifierSchema = IdentifierSchema {
    pattern: "^[0-9]{13}$",
    description: "13-stellige Marktpartner-ID (BDEW-Codenummer Strom mit Prefix 99, DVGW-Codenummer Gas mit Prefix 98, oder GS1 GLN)",
    example: "9900357000003",
};

// ─── §8.2 — ASCII-Verfahren identifiers ──────────────────────────────────────

/// [`NeloId`](crate::identifiers::NeloId).
pub const NELO_ID: IdentifierSchema = IdentifierSchema {
    pattern: "^E[A-Z0-9]{9}[0-9]$",
    description: "11-stellige BDEW Netzlokations-ID (NeLo-ID): Codetyp 'E' + 9 alphanumerische Zeichen [A-Z0-9] + ASCII-Verfahren-Prüfziffer",
    example: "E0000000019",
};

/// [`NebeId`](crate::identifiers::NebeId).
pub const NEBE_ID: IdentifierSchema = IdentifierSchema {
    pattern: "^F[A-Z0-9]{9}[0-9]$",
    description: "11-stellige BDEW Netzbereich-ID (NeBe-ID): Codetyp 'F' + 9 alphanumerische Zeichen [A-Z0-9] + ASCII-Verfahren-Prüfziffer",
    example: "F0000000018",
};

/// [`CrId`](crate::identifiers::CrId).
pub const CR_ID: IdentifierSchema = IdentifierSchema {
    pattern: "^A[A-Z0-9]{9}[0-9]$",
    description: "11-stellige Cluster-Ressource-ID (CR-ID): Codetyp 'A' + 9 alphanumerische Zeichen [A-Z0-9] + ASCII-Verfahren-Prüfziffer",
    example: "A0000000013",
};

/// [`SgId`](crate::identifiers::SgId).
pub const SG_ID: IdentifierSchema = IdentifierSchema {
    pattern: "^B[A-Z0-9]{9}[0-9]$",
    description: "11-stellige Steuergruppen-ID (SG-ID): Codetyp 'B' + 9 alphanumerische Zeichen [A-Z0-9] + ASCII-Verfahren-Prüfziffer",
    example: "B0000000012",
};

/// [`SrId`](crate::identifiers::SrId).
pub const SR_ID: IdentifierSchema = IdentifierSchema {
    pattern: "^C[A-Z0-9]{9}[0-9]$",
    description: "11-stellige Steuerbare-Ressource-ID (SR-ID): Codetyp 'C' + 9 alphanumerische Zeichen [A-Z0-9] + ASCII-Verfahren-Prüfziffer",
    example: "C0000000011",
};

/// [`TrId`](crate::identifiers::TrId).
pub const TR_ID: IdentifierSchema = IdentifierSchema {
    pattern: "^D[A-Z0-9]{9}[0-9]$",
    description: "11-stellige Technische-Ressource-ID (TR-ID): Codetyp 'D' + 9 alphanumerische Zeichen [A-Z0-9] + ASCII-Verfahren-Prüfziffer",
    example: "D0000000010",
};

/// [`PaketId`](crate::identifiers::PaketId).
pub const PAKET_ID: IdentifierSchema = IdentifierSchema {
    pattern: "^P9[A-Z0-9]{8}[0-9]$",
    description: "11-stellige BDEW Paket-ID: Codetyp 'P' + Sparte '9' (BDEW/Strom) + 8 alphanumerische Zeichen [A-Z0-9] + ASCII-Verfahren-Prüfziffer",
    example: "P9000000010",
};

// ─── Metering and grid ───────────────────────────────────────────────────────

/// [`MeloId`](crate::identifiers::MeloId).
pub const MELO_ID: IdentifierSchema = IdentifierSchema {
    pattern: "^[A-Z]{2}[A-Za-z0-9]{31}$",
    description: "33-stellige Messlokations-ID: 2-stelliger ISO-3166-1-Ländercode + 31 alphanumerische Zeichen",
    example: "DE0000000000000000000000000000001",
};

/// [`ObisCode`](crate::identifiers::ObisCode). Its `pattern` is
/// `obis_code::OBIS_PATTERN`, so the grammar has one spelling.
pub const OBIS_CODE: IdentifierSchema = IdentifierSchema {
    pattern: crate::identifiers::obis_code::OBIS_PATTERN,
    description: "OBIS-Kennzahl nach IEC 62056-61: [A-B:]C.D[.E][*F]. Eingaben werden kanonisiert gespeichert (führende Nullen entfallen, '&' wird zu '*').",
    example: "1-0:1.8.0*255",
};

// ─── ENTSO-E EIC and its restrictions ────────────────────────────────────────

/// [`EicCode`](crate::identifiers::EicCode).
pub const EIC_CODE: IdentifierSchema = IdentifierSchema {
    pattern: "^[A-Z0-9]{2}[ATVWXYZ][A-Z0-9-]{12}[A-Z0-9]$",
    description: "16-stelliger ENTSO-E Energy Identification Code: 2 Zeichen LIO-Kennung + Objekttyp (A/T/V/W/X/Y/Z) + 12 Zeichen Körper + ENTSO-E-Prüfzeichen",
    example: "10YDE-EON------1",
};

/// [`BilanzkreisId`](crate::identifiers::BilanzkreisId).
pub const BILANZKREIS_ID: IdentifierSchema = IdentifierSchema {
    pattern: "^[A-Z0-9]{2}X[A-Z0-9-]{12}[A-Z0-9]$",
    description: "16-stelliger EIC-Code mit Objekttyp 'X' (Party) — Bilanzkreis: 2 Zeichen LIO-Kennung + 'X' + 12 Zeichen Körper + ENTSO-E-Prüfzeichen",
    example: "11XSUEDWESTSTRO8",
};

/// [`BilanzierungsgebietId`](crate::identifiers::BilanzierungsgebietId).
pub const BILANZIERUNGSGEBIET_ID: IdentifierSchema = IdentifierSchema {
    pattern: "^[A-Z0-9]{2}Y[A-Z0-9-]{12}[A-Z0-9]$",
    description: "16-stelliger EIC-Code mit Objekttyp 'Y' (Area) — Bilanzierungsgebiet: 2 Zeichen LIO-Kennung + 'Y' + 12 Zeichen Körper + ENTSO-E-Prüfzeichen",
    example: "11YN-0000-0001-Q",
};

// ─── Redispatch 2.0 and MaBiS ────────────────────────────────────────────────

/// [`AkivId`](crate::identifiers::AkivId).
pub const AKIV_ID: IdentifierSchema = IdentifierSchema {
    pattern: "^[!-~]{1,36}$",
    description: "Aktivierungsidentifikator für Redispatch 2.0 und §14a EnWG Modul 3 (BDEW WiM AHB BK6-24-174): 1–36 druckbare ASCII-Zeichen (UUID-kompatibel)",
    example: "550e8400-e29b-41d4-a716-446655440000",
};

/// [`TranchennummerId`](crate::identifiers::TranchennummerId).
pub const TRANCHENNUMMER_ID: IdentifierSchema = IdentifierSchema {
    pattern: "^(0|[1-9][0-9]{0,5})$",
    description: "Tranchennummer für MABIS Bilanzkreisabrechnung (PID 13003): 1–6-stellige Dezimalzahl ohne führende Nullen (Wertebereich 0–999 999)",
    example: "42",
};

// ─── SEPA ────────────────────────────────────────────────────────────────────

/// [`Iban`](crate::identifiers::Iban).
pub const IBAN: IdentifierSchema = IdentifierSchema {
    pattern: "^[A-Z]{2}[0-9]{2}[A-Z0-9]{11,30}$",
    description: "IBAN nach ISO 13616 mit gültigen MOD-97-10-Prüfziffern (ISO 7064); Länge und Aufbau sind länderspezifisch, für DE 22 Zeichen",
    example: "DE89370400440532013000",
};

/// [`Bic`](crate::identifiers::Bic).
pub const BIC: IdentifierSchema = IdentifierSchema {
    pattern: "^[A-Z]{4}[A-Z]{2}[A-Z0-9]{2}([A-Z0-9]{3})?$",
    description: "BIC nach ISO 9362: 8 oder 11 Zeichen (Institut, Land, Ort, optional Filiale). Kein Prüfzeichen definiert",
    example: "COBADEFFXXX",
};
