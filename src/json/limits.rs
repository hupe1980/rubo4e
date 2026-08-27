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

/// The resource limits this crate enforces while parsing JSON.
///
/// A closed enum rather than a string tag: it makes the counter dispatch below
/// exhaustive, so adding a limit cannot silently miss its counter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum LimitKind {
    PayloadBytes,
    NestingDepth,
    ExtensionValueBytes,
    ExtensionFieldCount,
    ExtensionKeyLen,
}

impl LimitKind {
    /// Stable label used for `tracing` fields and the `metrics` counter tag.
    #[cfg(any(feature = "tracing", feature = "metrics"))]
    const fn as_str(self) -> &'static str {
        match self {
            Self::PayloadBytes => "payload_bytes",
            Self::NestingDepth => "nesting_depth",
            Self::ExtensionValueBytes => "extension_value_bytes",
            Self::ExtensionFieldCount => "extension_field_count",
            Self::ExtensionKeyLen => "extension_key_len",
        }
    }

    fn counter(self) -> &'static AtomicU64 {
        match self {
            Self::PayloadBytes => &JSON_LIMIT_HIT_PAYLOAD_BYTES,
            Self::NestingDepth => &JSON_LIMIT_HIT_NESTING_DEPTH,
            Self::ExtensionValueBytes => &JSON_LIMIT_HIT_EXTENSION_VALUE_BYTES,
            Self::ExtensionFieldCount => &JSON_LIMIT_HIT_EXTENSION_FIELD_COUNT,
            Self::ExtensionKeyLen => &JSON_LIMIT_HIT_EXTENSION_KEY_LEN,
        }
    }
}

pub(super) fn trace_limit_violation(kind: LimitKind, actual: usize, limit: usize) {
    kind.counter().fetch_add(1, Ordering::Relaxed);

    #[cfg(feature = "metrics")]
    metrics::counter!("bo4e_json_limit_hit_total", "kind" => kind.as_str()).increment(1);

    #[cfg(feature = "tracing")]
    tracing::warn!(
        kind = kind.as_str(),
        actual,
        limit,
        "bo4e json parse limit exceeded"
    );
    #[cfg(not(any(feature = "tracing", feature = "metrics")))]
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

/// Hardening limits for the `*_hardened` deserialization entry points.
///
/// Start from a profile and narrow it; every field is `Option`, and `None` means
/// that particular cap is off.
///
/// ```
/// # #[cfg(feature = "json")] {
/// use rubo4e::json::JsonParseLimits;
///
/// // The recommended profile for anything arriving over a network:
/// let limits = JsonParseLimits::untrusted_defaults();
///
/// // …tightened where you know your own payloads:
/// let limits = JsonParseLimits::untrusted_defaults()
///     .with_max_payload_bytes(Some(64 * 1024))
///     .with_max_extension_field_count(Some(0));   // reject any unknown field
/// # let _ = limits;
/// # }
/// ```
///
/// The type is `#[non_exhaustive]`, so there is no struct literal: new limits get
/// added as new amplification paths are found, and a literal would make each one
/// a breaking change.
///
/// # What is *not* capped
///
/// These bound the parser, not the object graph it produces. `[{},{},{}…]` is
/// three wire bytes and one full struct per element, so a payload well inside
/// `max_payload_bytes` can still allocate two orders of magnitude more. Size the
/// cap against the expanded cost, and put a concurrency limit in front of the
/// endpoint.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct JsonParseLimits {
    /// Maximum accepted input payload size in bytes, checked before parsing.
    pub max_payload_bytes: Option<usize>,
    /// Maximum accepted JSON nesting depth.
    ///
    /// Leaving it `None` does not disable depth checking: every entry point,
    /// hardened or not, enforces [`DEFAULT_MAX_NESTING_DEPTH`] (128) so a deeply
    /// nested payload cannot overflow the stack.
    ///
    /// In practice this only ever lowers the cap. `serde_json` enforces a
    /// recursion limit of its own at the same 128, and it is the inner parser,
    /// so a value above that never takes effect.
    pub max_nesting_depth: Option<usize>,
    /// Cumulative byte budget for all captured extension values in one call,
    /// charged across every nesting level rather than only the root.
    pub max_extension_value_bytes: Option<usize>,
    /// Maximum number of extension fields accepted **per struct**.
    ///
    /// A per-call tightening of the process-wide hard cap
    /// [`crate::json::MAX_EXTENSION_FIELDS`], which applies regardless. `Some(0)`
    /// rejects any payload carrying a field the schema does not define.
    pub max_extension_field_count: Option<usize>,
}

impl JsonParseLimits {
    /// Every opt-in cap off — the always-on ones still apply.
    ///
    /// This is not "no protection": the nesting-depth default of 128 and the two
    /// extension caps ([`MAX_EXTENSION_FIELDS`], [`MAX_EXTENSION_KEY_LEN`]) hold
    /// on every path. Use it for payloads you produced yourself, or as a base
    /// for a profile with exactly one cap set.
    ///
    /// [`MAX_EXTENSION_FIELDS`]: crate::json::MAX_EXTENSION_FIELDS
    /// [`MAX_EXTENSION_KEY_LEN`]: crate::json::MAX_EXTENSION_KEY_LEN
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            max_payload_bytes: None,
            max_nesting_depth: None,
            max_extension_value_bytes: None,
            max_extension_field_count: None,
        }
    }

    /// A conservative profile for input from an untrusted caller.
    ///
    /// 1 MB payload, depth 64, 64 KB of extension values, 32 extension fields
    /// per struct. Comfortably above any BO4E document in circulation — real
    /// ones are a few kilobytes and 6–8 levels deep — and far below what it
    /// takes to hurt a service.
    #[must_use]
    pub const fn untrusted_defaults() -> Self {
        Self {
            max_payload_bytes: Some(1_000_000),
            max_nesting_depth: Some(64),
            max_extension_value_bytes: Some(64_000),
            max_extension_field_count: Some(32),
        }
    }

    /// Sets [`max_payload_bytes`](Self::max_payload_bytes).
    #[must_use]
    pub const fn with_max_payload_bytes(mut self, bytes: Option<usize>) -> Self {
        self.max_payload_bytes = bytes;
        self
    }

    /// Sets [`max_nesting_depth`](Self::max_nesting_depth).
    #[must_use]
    pub const fn with_max_nesting_depth(mut self, depth: Option<usize>) -> Self {
        self.max_nesting_depth = depth;
        self
    }

    /// Sets [`max_extension_value_bytes`](Self::max_extension_value_bytes).
    #[must_use]
    pub const fn with_max_extension_value_bytes(mut self, bytes: Option<usize>) -> Self {
        self.max_extension_value_bytes = bytes;
        self
    }

    /// Sets [`max_extension_field_count`](Self::max_extension_field_count).
    #[must_use]
    pub const fn with_max_extension_field_count(mut self, count: Option<usize>) -> Self {
        self.max_extension_field_count = count;
        self
    }
}

// ─── Parse-time extension budget ─────────────────────────────────────────────
//
// The extension caps must apply to *every* struct in the payload, not just the
// root.  A post-hoc check on the deserialized root can only ever see the root's
// own `_additional` map, so extension data hidden inside a nested COM (e.g.
// `marktlokation.lokationsadresse`) escapes it entirely.
//
// The budget is therefore installed for the duration of one hardened call and
// consulted by `LimitedExtensionMap::deserialize` at every nesting level.  That
// also makes enforcement fail-fast: an oversized payload is rejected while it is
// being parsed, instead of after the whole tree has been materialized.
//
// A thread-local is sound here because a single `from_json_*` call is entirely
// synchronous — it never yields, so no other task can observe or share the
// scope.  `BudgetGuard` saves and restores the previous value, so nested
// hardened calls compose correctly.

thread_local! {
    static EXTENSION_BUDGET: std::cell::Cell<Option<ExtensionBudget>> =
        const { std::cell::Cell::new(None) };
}

/// Remaining extension allowance for the hardened call currently in progress.
#[derive(Debug, Clone, Copy)]
pub(super) struct ExtensionBudget {
    /// Cumulative value-byte allowance left for the whole payload.
    remaining_bytes: Option<usize>,
    /// Per-struct field-count cap (not consumed; re-checked at each struct).
    max_fields_per_struct: Option<usize>,
}

/// RAII guard that installs an [`ExtensionBudget`] and restores the previous one.
pub(super) struct BudgetGuard(Option<ExtensionBudget>);

impl Drop for BudgetGuard {
    fn drop(&mut self) {
        EXTENSION_BUDGET.with(|b| b.set(self.0));
    }
}

/// Installs the extension budget described by `limits` for the current scope.
///
/// A hardened call **always** installs one, even when `limits` constrains
/// nothing: skipping it would let an inner call inherit an outer call's remaining
/// allowance instead of its own.
pub(super) fn install_extension_budget(limits: JsonParseLimits) -> BudgetGuard {
    let budget = ExtensionBudget {
        remaining_bytes: limits.max_extension_value_bytes,
        max_fields_per_struct: limits.max_extension_field_count,
    };
    let previous = EXTENSION_BUDGET.with(|b| b.replace(Some(budget)));
    BudgetGuard(previous)
}

/// Returns the per-struct extension field-count cap, if a budget is installed.
#[inline]
pub(super) fn budget_max_fields_per_struct() -> Option<usize> {
    EXTENSION_BUDGET
        .with(|b| b.get())
        .and_then(|b| b.max_fields_per_struct)
}

/// Charges `bytes` against the cumulative value-byte allowance.
///
/// Returns `Err((requested, remaining))` once the allowance is exhausted, so the
/// caller can report both halves of the overrun. A no-op when no budget is
/// installed or no byte cap was configured.
#[inline]
pub(super) fn charge_extension_bytes(bytes: usize) -> Result<(), (usize, usize)> {
    EXTENSION_BUDGET.with(|cell| {
        let Some(mut budget) = cell.get() else {
            return Ok(());
        };
        let Some(remaining) = budget.remaining_bytes else {
            return Ok(());
        };
        let Some(left) = remaining.checked_sub(bytes) else {
            return Err((bytes, remaining));
        };
        budget.remaining_bytes = Some(left);
        cell.set(Some(budget));
        Ok(())
    })
}

/// Default maximum JSON nesting depth, applied on **every** deserialization
/// path — German or snake_case, string or bytes, hardened or not.
///
/// Valid BO4E structures are at most 6–8 levels deep in practice.  128 is a
/// generous allowance that eliminates the stack-overflow DoS surface while
/// accepting all legitimate payloads.
///
/// This duplicates `serde_json`'s own recursion limit, which also defaults to
/// 128, so `serde_json` usually reports first with its own wording. Enforcing it
/// here as well keeps the limit a property of *this* crate rather than of the
/// backend's default, and keeps the message actionable — it names the hardened
/// entry point that can change it.
///
/// The `_hardened` variants accept an explicit
/// [`JsonParseLimits::max_nesting_depth`], which replaces this default. Only a
/// lower value has any effect: `serde_json`'s own limit sits at the same 128 and
/// it parses first.
pub const DEFAULT_MAX_NESTING_DEPTH: usize = 128;

pub(super) fn check_payload_limit(
    payload_len: usize,
    limits: JsonParseLimits,
) -> Result<(), serde_json::Error> {
    if let Some(max) = limits.max_payload_bytes {
        if payload_len > max {
            trace_limit_violation(LimitKind::PayloadBytes, payload_len, max);
            return Err(serde_json::Error::custom(format!(
                "payload too large: {payload_len} bytes exceeds limit {max}"
            )));
        }
    }
    Ok(())
}

/// The nesting-depth cap a call runs under: whatever `limits` asks for, or the
/// always-on default.
///
/// Depth is capped on **every** path, hardened or not — a `None` here lowers
/// nothing, it just leaves [`DEFAULT_MAX_NESTING_DEPTH`] in force.
#[inline]
pub(super) fn resolved_max_depth(limits: JsonParseLimits) -> usize {
    limits
        .max_nesting_depth
        .unwrap_or(DEFAULT_MAX_NESTING_DEPTH)
}

pub(super) fn deserialize_german_from_str<T: DeserializeOwned>(
    s: &str,
    max_depth: usize,
) -> Result<T, serde_json::Error> {
    let state = DepthState::new(max_depth);
    let mut de = serde_json::Deserializer::from_str(s);
    let value = T::deserialize(DepthLimitedDeserializer::new(&mut de, &state))?;
    // The whole input must be consumed. Without this, `{…} <anything>` decodes
    // here while `serde_json::from_str` on the same bytes rejects it — a parser
    // differential between this crate and whatever validates in front of it.
    de.end()?;
    Ok(value)
}

/// Depth-limited decode of an already-parsed [`serde_json::Value`].
///
/// The same wrapper the text readers use: `Value` is itself a `Deserializer`, so
/// the depth guard composes over it unchanged. There is no `end()` to call —
/// a `Value` is a whole document by construction, so the trailing-input
/// differential the text path guards against cannot arise.
pub(super) fn deserialize_german_from_value<T: DeserializeOwned>(
    value: serde_json::Value,
    max_depth: usize,
) -> Result<T, serde_json::Error> {
    let state = DepthState::new(max_depth);
    T::deserialize(DepthLimitedDeserializer::new(value, &state))
}

pub(super) fn deserialize_german_from_slice<T: DeserializeOwned>(
    bytes: &[u8],
    max_depth: usize,
) -> Result<T, serde_json::Error> {
    let state = DepthState::new(max_depth);
    let mut de = serde_json::Deserializer::from_slice(bytes);
    let value = T::deserialize(DepthLimitedDeserializer::new(&mut de, &state))?;
    de.end()?;
    Ok(value)
}
