//! Domain identifier newtypes for BO4E energy-market entities.
//!
//! Every identifier:
//! - validates its input at construction time (never panics)
//! - stores the validated string as a `Box<str>` (compact, immutable)
//! - implements `Display`, `FromStr`, `TryFrom<&str>`, `TryFrom<String>`,
//!   `Into<String>`, `AsRef<str>`, `Borrow<str>`, `Deref<Target = str>`, and
//!   `Debug`, `Clone`, `Hash`, `Eq`, `Ord` — none of them behind a feature flag,
//!   because they are the minimum an EDIFACT encoder or decoder needs
//! - adds `Serialize` / `Deserialize` with the `serde` feature, routed through
//!   the same constructor
//!
//! Structural validation always runs at construction. The `validate` feature
//! additionally derives [`garde`] rules that re-run the *same* function, so a
//! `Validated<Marktlokation>` re-checks every nested identifier through garde's
//! report API.
//!
//! ## What each type validates
//!
//! Section numbers refer to the BDEW Anwendungshilfe **"Identifikatoren in der
//! Marktkommunikation"** v1.2 (7 February 2025).
//!
//! | Type | Always validated | Reference |
//! |------|-----------------|-----------|
//! | [`MaloId`] | 11 digits, first digit `1`–`9`, §8.1 check digit | BDEW §3 |
//! | [`MeloId`] | 33 chars, first 2 uppercase ASCII (country code), rest alphanumeric | No checksum defined |
//! | [`Zaehlpunktbezeichnung`] | the same 33 chars — a Zählpunkt that is *not* a Messlokation | MaBiS; BDEW AWH BK6-20-160 §1.6.2 |
//! | [`MarktpartnerId`] | 13 digits — check digit **not** enforced, see below | BDEW §2 |
//! | [`Lokationsbuendelcode`] | 13 digits, §8.1 check digit — *which* Lokationsbündelstruktur | EDI@Energy Codeliste Lokationsbündelstrukturen v1.0 |
//! | [`LokationsbuendelObjektcode`] | 13 digits, §8.1 check digit — *where in it* an object sits | EDI@Energy Codeliste Lokationsbündelstrukturen v1.0 |
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
//! ## Helper types beside the identifiers
//!
//! Not every type here is an identifier. [`EicType`], [`MaloVergabestelle`],
//! [`MpIdAuthority`] and [`ObisComponents`] are facts *read out of* an
//! identifier, returned by its accessors. [`Zaehlpunktart`] and [`Zaehlpunkt`]
//! are the exception that proves the rule: a Zählpunktart cannot be read out of
//! a [`Zaehlpunktbezeichnung`] — a Zählpunkt (eMob) and a [`MeloId`] are
//! indistinguishable as strings — so it has to be carried alongside, and
//! [`Zaehlpunkt::as_melo_id`] is the narrowing that refuses without it.
//!
//! ## The two BDEW check-digit procedures
//!
//! Chapter 8 defines two, and they are the *same* arithmetic — §8.1 for numeric
//! identifiers, §8.2 (the ASCII-Verfahren, where `A`–`Z` map to their ASCII
//! codes) for alphanumeric ones. A digit maps identically under both, so §8.1 is
//! §8.2 restricted to numeric input, and this crate implements it once.
//!
//! [`MarktpartnerId`] is the exception that enforces none: an MP-ID may carry
//! either the §8.1 digit or a GS1/EAN-13 one, and the leading digits do not
//! reliably separate them. See its own docs for the opt-in checks.
//!
//! ## Where [`Iban`] and [`Bic`] are *not* used
//!
//! `Zahlungsinformation` keeps both fields as `String`: it hangs off `Rechnung`
//! and nothing else, so a newtype refusing a **masked** IBAN — `DE89 **** ****
//! 3000`, routine on an invoice — would take the whole invoice with it.
//! `Zahlungsinformation::iban_checked()` costs the caller the field instead.

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
mod lokationsbuendel_codes;
mod malo_id;
mod marktpartner_id;
mod melo_id;
pub(crate) mod obis_code;
#[cfg(any(feature = "schemars", feature = "utoipa"))]
pub mod schema;
#[cfg(feature = "sqlx")]
mod sqlx_impls;
mod tranchennummer_id;
mod zaehlpunkt;

pub use akiv_id::{AkivId, AKIV_ID_MAX_LEN};
pub use ascii_ids::{CrId, NebeId, NeloId, PaketId, SgId, SrId, TrId};
pub use bank::{Bic, Iban, IBAN_MAX_LEN, IBAN_MIN_LEN};
pub use bilanzkreis_id::{BilanzierungsgebietId, BilanzkreisId};
pub use eic_code::{EicCode, EicType};
pub use lokationsbuendel_codes::{LokationsbuendelObjektcode, Lokationsbuendelcode};
pub use malo_id::{MaloId, MaloVergabestelle};
pub use marktpartner_id::{MarktpartnerId, MpIdAuthority};
pub use melo_id::MeloId;
pub use obis_code::{ObisCode, ObisComponents};
pub use tranchennummer_id::{TranchennummerId, TRANCHENNUMMER_MAX};
pub use zaehlpunkt::{Zaehlpunkt, Zaehlpunktart, Zaehlpunktbezeichnung};

/// Serde adapter module for encoding [`MarktpartnerId`] as a JSON integer (`i64`).
///
/// Use `#[serde(with = "rubo4e::identifiers::marktpartner_id_as_i64")]` on struct
/// fields that must round-trip through APIs which mandate integer encoding for
/// Marktpartner-IDs (BDEW-Codenummern, DVGW-Codenummern, GS1 GLNs) — e.g. BDEW
/// API-Webdienste Strom.
#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
pub use marktpartner_id::serde_as_i64 as marktpartner_id_as_i64;

/// Returns the character starting at byte index `i`, for an error report.
///
/// Every validator here walks `as_bytes()` and reports the byte offset it
/// stopped at, so the index handed back can land inside a multi-byte character —
/// `&s[i..]` would panic there. Guarded by [`str::is_char_boundary`], and
/// U+FFFD stands in for the cases it cannot name.
#[inline]
pub(crate) fn char_at(s: &str, i: usize) -> char {
    if s.is_char_boundary(i) {
        s[i..].chars().next().unwrap_or('\u{FFFD}')
    } else {
        '\u{FFFD}'
    }
}

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

/// Records one identifier that failed validation on the way in.
///
/// The rejected value is **not** logged: these types carry metering points,
/// market partners, and bank accounts, and emitting one at `warn!` copies
/// personal and payment data into whatever the log sink happens to be. The event
/// carries the type, the byte length, and the error — and
/// [`IdentifierError`](crate::error::IdentifierError) already names the
/// offending position and the expected shape. Capture the value yourself, at a
/// boundary allowed to hold it.
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
        input_len = input.len(),
        error = %error,
        "identifier validation failed during deserialization"
    );

    #[cfg(not(feature = "tracing"))]
    let _ = (identifier, input, error);
}
