//! Parse-limit counters, [`JsonParseLimits`], and low-level helpers
//! for the hardened deserialization entry points.

use std::sync::atomic::{AtomicU64, Ordering};
#[cfg(feature = "tracing")]
use std::time::Instant;

use serde::de::DeserializeOwned;
use serde::de::Error as _;
// depth is a sibling module. depth imports only `trace_limit_violation` from limits,
// and limits imports only `DepthLimitedDeserializer`/`DepthState` from depth —
// no true circular data dependency exists, Rust handles this fine within a module tree.
use super::depth::{DepthLimitedDeserializer, DepthState};

#[cfg(feature = "simd-json")]
// Threshold below which serde_json is used even when `simd-json` is enabled.
// simd-json's parser setup cost exceeds its throughput advantage on small
// payloads; 2 KiB was empirically chosen on a 2024 ARM workstation
// (see `benches/json_perf.rs`). Adjust if your payload distribution differs.
const SIMD_JSON_STR_MIN_BYTES: usize = 2048;

// For mutable-byte APIs, simd-json can still lose on small/medium payloads due
// to parser setup costs. Prefer serde_json below this threshold.
// 1.5 KiB is slightly lower than the str threshold because the byte path avoids
// the UTF-8 validation copy that the str path pays.
#[cfg(feature = "simd-json")]
const SIMD_JSON_BYTES_MIN_BYTES: usize = 1536;

#[inline]
pub(super) fn trace_deser_error<T>(result: &Result<T, serde_json::Error>, context: &'static str) {
    #[cfg(feature = "tracing")]
    if let Err(ref e) = result {
        tracing::debug!(error = %e, "{context}");
    }
    #[cfg(not(feature = "tracing"))]
    {
        let _ = (result, context);
    }
}

#[cfg(feature = "tracing")]
pub(super) fn trace_json_outcome(
    operation: &'static str,
    mode: &'static str,
    bo_type: &'static str,
    input_len: Option<usize>,
    output_len: Option<usize>,
    start: Instant,
    ok: bool,
) {
    let elapsed_us = start.elapsed().as_micros() as u64;
    tracing::debug!(
        operation,
        mode,
        bo_type,
        input_len,
        output_len,
        ok,
        elapsed_us,
        "bo4e json operation completed"
    );
}

/// Increments the process-wide atomic counter for the given limit kind.
fn increment_limit_counter(kind: &'static str) {
    match kind {
        "payload_bytes" => {
            JSON_LIMIT_HIT_PAYLOAD_BYTES.fetch_add(1, Ordering::Relaxed);
        }
        "nesting_depth" => {
            JSON_LIMIT_HIT_NESTING_DEPTH.fetch_add(1, Ordering::Relaxed);
        }
        "extension_value_bytes" => {
            JSON_LIMIT_HIT_EXTENSION_VALUE_BYTES.fetch_add(1, Ordering::Relaxed);
        }
        "extension_field_count" => {
            JSON_LIMIT_HIT_EXTENSION_FIELD_COUNT.fetch_add(1, Ordering::Relaxed);
        }
        "extension_key_len" => {
            JSON_LIMIT_HIT_EXTENSION_KEY_LEN.fetch_add(1, Ordering::Relaxed);
        }
        _ => {
            debug_assert!(false, "unknown limit kind: {kind}");
        }
    }
    #[cfg(feature = "metrics")]
    metrics::counter!("bo4e_json_limit_hit_total", "kind" => kind).increment(1);
}

pub(super) fn trace_limit_violation(kind: &'static str, actual: usize, limit: usize) {
    increment_limit_counter(kind);
    #[cfg(feature = "tracing")]
    tracing::warn!(kind, actual, limit, "bo4e json parse limit exceeded");
    #[cfg(not(feature = "tracing"))]
    let _ = (actual, limit);
}

static JSON_LIMIT_HIT_PAYLOAD_BYTES: AtomicU64 = AtomicU64::new(0);
static JSON_LIMIT_HIT_NESTING_DEPTH: AtomicU64 = AtomicU64::new(0);
static JSON_LIMIT_HIT_EXTENSION_VALUE_BYTES: AtomicU64 = AtomicU64::new(0);
static JSON_LIMIT_HIT_EXTENSION_FIELD_COUNT: AtomicU64 = AtomicU64::new(0);
static JSON_LIMIT_HIT_EXTENSION_KEY_LEN: AtomicU64 = AtomicU64::new(0);

/// Snapshot of JSON hardening limit-hit counters.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct JsonLimitHitCounters {
    /// Number of payload-size limit violations.
    pub payload_bytes: u64,
    /// Number of nesting-depth limit violations.
    pub nesting_depth: u64,
    /// Number of extension-value-budget limit violations.
    pub extension_value_bytes: u64,
    /// Number of extension-field-count limit violations.
    pub extension_field_count: u64,
    /// Number of extension-field-key-length limit violations.
    pub extension_key_len: u64,
}

/// Returns a snapshot of JSON hardening limit-hit counters for this process.
#[must_use]
pub fn json_limit_hit_counters() -> JsonLimitHitCounters {
    JsonLimitHitCounters {
        payload_bytes: JSON_LIMIT_HIT_PAYLOAD_BYTES.load(Ordering::Relaxed),
        nesting_depth: JSON_LIMIT_HIT_NESTING_DEPTH.load(Ordering::Relaxed),
        extension_value_bytes: JSON_LIMIT_HIT_EXTENSION_VALUE_BYTES.load(Ordering::Relaxed),
        extension_field_count: JSON_LIMIT_HIT_EXTENSION_FIELD_COUNT.load(Ordering::Relaxed),
        extension_key_len: JSON_LIMIT_HIT_EXTENSION_KEY_LEN.load(Ordering::Relaxed),
    }
}

/// Optional hardening limits for JSON deserialization entry points.
///
/// Use with the `*_hardened` methods on `Bo4eJsonExt` to constrain resource
/// usage when parsing untrusted payloads.
#[derive(Debug, Clone, Copy, Default)]
pub struct JsonParseLimits {
    /// Maximum allowed input payload size in bytes.
    pub max_payload_bytes: Option<usize>,
    /// Maximum allowed JSON nesting depth.
    pub max_nesting_depth: Option<usize>,
    /// Maximum cumulative size budget for captured extension values.
    pub max_extension_value_bytes: Option<usize>,
    /// Maximum number of extension fields accepted per struct.
    ///
    /// This is a softer per-call limit that sits below the process-wide hard cap
    /// of [`crate::json::MAX_EXTENSION_FIELDS`], which is enforced during deserialization.
    /// Use this to apply a tighter bound for specific untrusted inputs.
    pub max_extension_field_count: Option<usize>,
}

impl JsonParseLimits {
    /// Returns a limit set with all caps disabled.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            max_payload_bytes: None,
            max_nesting_depth: None,
            max_extension_value_bytes: None,
            max_extension_field_count: None,
        }
    }

    /// Returns a conservative default profile for untrusted external inputs.
    #[must_use]
    pub const fn untrusted_defaults() -> Self {
        Self {
            max_payload_bytes: Some(1_000_000),
            max_nesting_depth: Some(64),
            max_extension_value_bytes: Some(64_000),
            max_extension_field_count: Some(32),
        }
    }
}

/// Default maximum JSON nesting depth for all non-hardened deserialization paths.
///
/// Valid BO4E structures are at most 6–8 levels deep in practice.  128 is a
/// generous allowance that eliminates the stack-overflow DoS surface while
/// accepting all legitimate payloads.
///
/// The `_hardened` variants accept an explicit [`JsonParseLimits::max_nesting_depth`]
/// which, when set, takes priority over this default.
pub(super) const DEFAULT_MAX_NESTING_DEPTH: usize = 128;

/// Scans `bytes` for the maximum JSON nesting depth without parsing.
///
/// This is a single-pass linear scan that correctly skips `{` / `[` / `}` / `]`
/// characters inside JSON string values (honouring `\"` escape sequences).  It
/// does **not** do full JSON validation — it is only used to guard against
/// deeply-nested payloads before handing off to the real parser.
///
/// Used by [`deserialize_german_from_str`] / [`deserialize_german_from_slice`]
/// on code paths where `simd-json` is active, because the SIMD parser does not
/// support visitor wrapping and therefore cannot use `DepthLimitedDeserializer`.
/// The `_hardened` variants use the true single-pass visitor approach instead.
pub(super) fn scan_max_nesting_depth(bytes: &[u8]) -> usize {
    let mut depth: usize = 0;
    let mut max: usize = 0;
    let mut in_string = false;
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        if in_string {
            if b == b'\\' {
                i += 1; // skip the next escaped byte (could be '"' or another escape)
            } else if b == b'"' {
                in_string = false;
            }
        } else {
            match b {
                b'"' => in_string = true,
                b'{' | b'[' => {
                    depth += 1;
                    if depth > max {
                        max = depth;
                    }
                }
                b'}' | b']' => {
                    depth = depth.saturating_sub(1);
                }
                _ => {}
            }
        }
        i += 1;
    }
    max
}

pub(super) fn check_payload_limit(
    payload_len: usize,
    limits: JsonParseLimits,
) -> Result<(), serde_json::Error> {
    if let Some(max) = limits.max_payload_bytes {
        if payload_len > max {
            trace_limit_violation("payload_bytes", payload_len, max);
            return Err(serde_json::Error::custom(format!(
                "payload too large: {payload_len} bytes exceeds limit {max}"
            )));
        }
    }
    Ok(())
}

/// Checks `bytes` against [`DEFAULT_MAX_NESTING_DEPTH`] using a pre-scan.
///
/// Returns a serde error if the depth is exceeded.  Called on paths where
/// `DepthLimitedDeserializer` cannot be used (simd-json).
pub(super) fn check_default_depth(bytes: &[u8]) -> Result<(), serde_json::Error> {
    let actual = scan_max_nesting_depth(bytes);
    if actual > DEFAULT_MAX_NESTING_DEPTH {
        trace_limit_violation("nesting_depth", actual, DEFAULT_MAX_NESTING_DEPTH);
        Err(serde_json::Error::custom(format!(
            "JSON nesting depth {actual} exceeds default limit {DEFAULT_MAX_NESTING_DEPTH}; \
             use from_json_german_hardened with a JsonParseLimits to adjust"
        )))
    } else {
        Ok(())
    }
}

pub(super) fn deserialize_german_from_str<T: DeserializeOwned>(
    s: &str,
) -> Result<T, serde_json::Error> {
    #[cfg(feature = "simd-json")]
    {
        if s.len() < SIMD_JSON_STR_MIN_BYTES {
            // Small payload: fall through to the serde_json single-pass path below.
        } else {
            // Large payload: simd-json does not support visitor wrapping, so we
            // pre-scan the raw bytes for nesting depth before dispatching.
            check_default_depth(s.as_bytes())?;
            let mut buf = s.as_bytes().to_vec();
            return simd_json::from_slice::<T>(&mut buf).map_err(serde_json::Error::custom);
        }
    }
    // serde_json path: single-pass depth enforcement via DepthLimitedDeserializer.
    let state = DepthState::new(DEFAULT_MAX_NESTING_DEPTH);
    let mut de = serde_json::Deserializer::from_str(s);
    T::deserialize(DepthLimitedDeserializer::new(&mut de, &state))
}

pub(super) fn deserialize_german_from_slice<T: DeserializeOwned>(
    bytes: &[u8],
) -> Result<T, serde_json::Error> {
    #[cfg(feature = "simd-json")]
    {
        if bytes.len() < SIMD_JSON_BYTES_MIN_BYTES {
            // Small payload: fall through to the serde_json single-pass path below.
        } else {
            // Large payload: pre-scan depth before simd-json (no visitor wrapping available).
            check_default_depth(bytes)?;
            let mut buf = bytes.to_vec();
            return simd_json::from_slice::<T>(&mut buf).map_err(serde_json::Error::custom);
        }
    }
    // serde_json path: single-pass depth enforcement via DepthLimitedDeserializer.
    let state = DepthState::new(DEFAULT_MAX_NESTING_DEPTH);
    let mut de = serde_json::Deserializer::from_slice(bytes);
    T::deserialize(DepthLimitedDeserializer::new(&mut de, &state))
}
