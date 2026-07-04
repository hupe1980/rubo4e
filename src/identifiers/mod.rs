//! Domain identifier newtypes for BO4E energy-market entities.
//!
//! Every identifier:
//! - validates its input at construction time (never panics)
//! - stores the validated string as a `Box<str>` (compact, immutable)
//! - implements `Display`, `FromStr`, `TryFrom<&str>`, `TryFrom<String>`, `AsRef<str>`,
//!   `Debug`, `Clone`, `Hash`, `Eq`, `PartialEq`, `Ord`, `PartialOrd`
//! - conditionally derives `Serialize` / `Deserialize` via the `serde` feature gate

#[cfg(feature = "serde")]
use std::sync::atomic::{AtomicU64, Ordering};

mod checksum;
mod eic_code;
mod malo_id;
mod marktpartner_id;
mod melo_id;
mod nelo_id;
mod obis_code;
#[cfg(test)]
mod proptest_impls;
#[cfg(feature = "sqlx")]
mod sqlx_impls;
mod sr_id;
mod tr_id;

pub use eic_code::{EicCode, EicDomain};
pub use malo_id::MaloId;
pub use marktpartner_id::MarktpartnerId;
pub use melo_id::MeloId;
pub use nelo_id::NeloId;
pub use obis_code::{ObisCode, ObisComponents};
pub use sr_id::SrId;
pub use tr_id::TrId;

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
