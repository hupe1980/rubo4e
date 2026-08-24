//! Domain identifier newtypes for BO4E energy-market entities.
//!
//! Every identifier:
//! - validates its input at construction time (never panics)
//! - stores the validated string as a `Box<str>` (compact, immutable)
//! - implements `Display`, `FromStr`, `TryFrom<&str>`, `TryFrom<String>`, `AsRef<str>`,
//!   `Debug`, `Clone`, `Hash`, `Eq`, `PartialEq`, `Ord`, `PartialOrd`
//! - conditionally derives `Serialize` / `Deserialize` via the `serde` feature gate
//!
//! ## Validation at construction vs. `validate` feature
//!
//! All identifier types **always** validate the structural constraints (length,
//! character set) at construction time — this happens regardless of whether the
//! `validate` feature is enabled.
//!
//! The `validate` feature adds [`garde`]-based validation attributes so that
//! `Validated<T>` (and `#[derive(garde::Validate)]` on parent structs) can
//! re-run the same checks via the garde report API.  The actual validation logic
//! is identical in both paths.
//!
//! ### Per-type validation rules
//!
//! Section numbers refer to the BDEW Anwendungshilfe **"Identifikatoren in der
//! Marktkommunikation"** v1.2 (7 February 2025).
//!
//! | Type | Always validated | Reference |
//! |------|-----------------|-----------|
//! | [`MaloId`] | 11 digits, first digit `1`–`9`, §8.1 check digit | BDEW §3 |
//! | [`MeloId`] | 33 chars, first 2 uppercase ASCII (country code), rest alphanumeric | No checksum defined |
//! | [`MarktpartnerId`] | 13 digits — check digit **not** enforced, see below | BDEW §2 |
//! | [`NeloId`] | Codetyp `'E'` + 9 `[A-Z0-9]` + §8.2 check digit | BDEW §4 (BK6-22-128) |
//! | [`NebeId`] | Codetyp `'F'` + 9 `[A-Z0-9]` + §8.2 check digit | BDEW §5 (BK6-22-300, BK8-22/010-A) |
//! | [`CrId`] | Codetyp `'A'` + 9 `[A-Z0-9]` + §8.2 check digit | BDEW §6.5/§6.6 — Cluster Ressource |
//! | [`SgId`] | Codetyp `'B'` + 9 `[A-Z0-9]` + §8.2 check digit | BDEW §6.4/§6.6 — Steuergruppe |
//! | [`SrId`] | Codetyp `'C'` + 9 `[A-Z0-9]` + §8.2 check digit | BDEW §6.3/§6.6 — Steuerbare Ressource |
//! | [`TrId`] | Codetyp `'D'` + 9 `[A-Z0-9]` + §8.2 check digit | BDEW §6.2/§6.6 — Technische Ressource |
//! | [`PaketId`] | Codetyp `'P9'` + 8 `[A-Z0-9]` + §8.2 check digit | BDEW §7 — Netzbetreiberwechsel |
//! | [`EicCode`] | 16 chars, uppercase alphanumeric + `-`, ENTSO-E check char | ENTSO-E EIC Reference Manual |
//! | [`BilanzkreisId`] | 16-char EIC restricted to object type `'X'` (Party) | GaBi Gas BK7-14-020, MABIS BK6-06-009 |
//! | [`BilanzierungsgebietId`] | 16-char EIC restricted to object type `'Y'` (Area) | MABIS BK6-06-009 |
//! | [`ObisCode`] | `[A-B:]C.D[.E][*F]` format | IEC 62056-61 (C=0 permitted) |
//! | [`AkivId`] | 1–36 printable ASCII chars | BDEW WiM AHB BK6-24-174 |
//! | [`TranchennummerId`] | 1–6 decimal digits, no leading zeros (0–999 999) | MABIS PID 13003 (BK6-06-009) |
//! | [`Iban`] | 15–34 chars, registered per-country length, MOD-97-10 check digits | ISO 13616 / ISO 7064 |
//! | [`Bic`] | 8 or 11 chars, letters in the institution and country codes | ISO 9362 — no checksum defined |
//!
//! ### The two BDEW check-digit procedures
//!
//! BDEW chapter 8 defines two procedures, and they are the *same* arithmetic:
//! sum the mapped character values at odd positions, add twice the sum at even
//! positions, and take the difference to the next multiple of 10.
//!
//! - **§8.1 Lok- und Waggon-Kennzeichnungsverfahren** — numeric identifiers
//!   ([`MaloId`], BDEW-/DVGW-Codenummern). Each digit maps to its own value.
//! - **§8.2 ASCII-Verfahren** — alphanumeric identifiers ([`NeloId`], [`NebeId`],
//!   [`CrId`], [`SgId`], [`SrId`], [`TrId`], [`PaketId`]). Digits map to their
//!   value, uppercase letters to their ASCII code (`A` = 65 … `Z` = 90).
//!
//! Because a digit maps identically under both, §8.1 is exactly §8.2 restricted
//! to numeric input, and this crate implements the arithmetic once.
//!
//! ### Why [`MarktpartnerId`] does not enforce a check digit
//!
//! An MP-ID may be a BDEW-/DVGW-Codenummer (which uses §8.1) *or* a GS1 Global
//! Location Number (which uses the GS1/EAN-13 procedure). The two disagree, and
//! the leading digits do not reliably separate them — codes predating the
//! `98`/`99` convention are still in circulation. Enforcing either one by default
//! would reject valid production identifiers, so construction checks only the
//! unambiguous part and the check digit is available on demand via
//! [`MarktpartnerId::new_checked`], [`MarktpartnerId::has_valid_bdew_check_digit`],
//! and [`MarktpartnerId::has_valid_gln_check_digit`].
//!
//! ### The two bank identifiers, and where they are *not* used
//!
//! [`Iban`] and [`Bic`] exist for `Zahlungsinformation.iban` / `.bic`, but the
//! generated struct keeps both fields as `String`. `Zahlungsinformation` hangs
//! off `Rechnung` and nothing else, so a newtype that refuses a **masked** IBAN
//! — `DE89 **** **** 3000`, routine on an invoice — would take the whole invoice
//! down with it. `Zahlungsinformation::iban_checked()` runs the check on demand
//! and returns an error instead, which costs the caller the field rather than
//! the invoice. `Iban::new` normalises grouping spaces and case, so a value
//! copied off a statement parses.
//!
//! ### Wire-format traits without feature flags
//!
//! All identifier types unconditionally implement `Display`, `FromStr`,
//! `TryFrom<&str>`, `TryFrom<String>`, and `AsRef<str>` regardless of which
//! features are enabled.  These are the minimum needed for EDIFACT wire-format
//! encoding/decoding and are **not** gated on `serde` or any other feature.
//!
//! ### `validate` feature and `garde`
//!
//! When `validate` is enabled, each identifier derives `garde::Validate` with a
//! `custom(check_*)` validator that delegates to the same `validate()` function
//! used at construction.  This means `Validated::<Marktlokation>::new(malo)` will
//! re-validate all nested identifier fields (e.g. `marktlokations_id`) through
//! garde's recursive report API.

#[cfg(feature = "serde")]
use std::sync::atomic::{AtomicU64, Ordering};

// Declared first and with `#[macro_use]` so the identifier macros are in textual
// scope for every module below.
#[macro_use]
mod macros;

mod akiv_id;
mod ascii_ids;
mod bank;
mod bilanzkreis_id;
mod checksum;
mod eic_code;
mod malo_id;
mod marktpartner_id;
mod melo_id;
pub(crate) mod obis_code;
#[cfg(feature = "sqlx")]
mod sqlx_impls;
mod tranchennummer_id;

pub use akiv_id::{AkivId, AKIV_ID_MAX_LEN};
pub use ascii_ids::{CrId, NebeId, NeloId, PaketId, SgId, SrId, TrId};
pub use bank::{Bic, Iban, IBAN_MAX_LEN, IBAN_MIN_LEN};
pub use bilanzkreis_id::{BilanzierungsgebietId, BilanzkreisId};
pub use eic_code::{EicCode, EicType};
pub use malo_id::{MaloId, MaloVergabestelle};
pub use marktpartner_id::{MarktpartnerId, MpIdAuthority};
pub use melo_id::MeloId;
pub use obis_code::{ObisCode, ObisComponents};
pub use tranchennummer_id::{TranchennummerId, TRANCHENNUMMER_MAX};

/// Serde adapter module for encoding [`MarktpartnerId`] as a JSON integer (`i64`).
///
/// Use `#[serde(with = "rubo4e::identifiers::marktpartner_id_as_i64")]` on struct
/// fields that must round-trip through APIs which mandate integer encoding for
/// Marktpartner-IDs (BDEW-Codenummern, DVGW-Codenummern, GS1 GLNs) — e.g. BDEW
/// API-Webdienste Strom.
#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
pub use marktpartner_id::serde_as_i64 as marktpartner_id_as_i64;

#[cfg(feature = "serde")]
static IDENTIFIER_DESER_FAILURES: AtomicU64 = AtomicU64::new(0);

/// Returns the total number of identifier deserialization validation failures
/// observed in this process (across all identifier types).
///
/// This counter is incremented each time a JSON string fails to deserialize into
/// a typed identifier (e.g. a malformed `MaloId` in a JSON payload).  The count
/// is monotonically non-decreasing and uses `Ordering::Relaxed` — it is suitable
/// for monitoring but not for synchronization.
///
/// Use this in observability endpoints or health-check endpoints to detect data
/// quality regressions in upstream JSON producers.  Pair with the `tracing` and
/// `metrics` features for structured logging and metric export.
///
/// # Semver stability
///
/// This function is part of the public API and subject to semver guarantees.
/// The counter resets to zero at process start.
#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
#[must_use]
pub fn identifier_deser_failure_count() -> u64 {
    IDENTIFIER_DESER_FAILURES.load(Ordering::Relaxed)
}

#[cfg(feature = "serde")]
pub(crate) fn trace_identifier_deser_error(
    identifier: &'static str,
    input: &str,
    error: &crate::error::IdentifierError,
) {
    IDENTIFIER_DESER_FAILURES.fetch_add(1, Ordering::Relaxed);

    #[cfg(feature = "metrics")]
    metrics::counter!(
        "bo4e_identifier_deser_failure_total",
        "identifier" => identifier,
    )
    .increment(1);

    #[cfg(feature = "tracing")]
    tracing::warn!(
        identifier,
        input,
        error = %error,
        "identifier validation failed during deserialization"
    );

    #[cfg(not(feature = "tracing"))]
    {
        let _ = (identifier, input, error);
    }
}
