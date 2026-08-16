// This module is compiled only when the `json` feature is active.

//! JSON serialization helpers for BO4E types.
//!
//! Import `Bo4eJsonExt` (or `use rubo4e::prelude::*`) to gain `.to_json_german()`,
//! `.to_json_snake_case()`, and `.to_json_canonical()` on any BO or COM struct.
//!
//! # Serialization modes
//!
//! | Method                    | Key order        | Key names                        |
//! |---------------------------|------------------|----------------------------------|
//! | [`to_json_german`]        | insertion order  | BO4E German camelCase            |
//! | [`to_json_snake_case`]    | insertion order  | Rust snake_case field names      |
//! | [`to_json_canonical`]     | alphabetical     | BO4E German camelCase            |
//!
//! The "snake_case" format uses the Rust struct field names as JSON keys
//! (e.g. `marktlokationsId` → `marktlokations_id`).  A true German→English
//! translation is intentionally out of scope; use [`to_json_german`] for
//! wire-compatible BO4E exchange.
//!
//! # Key mapping is exact, not heuristic
//!
//! Both directions are table lookups into a mapping the generator emits from the
//! same field data it uses to emit the structs, so
//! `from_json_snake_case(to_json_snake_case(x))` returns `x` for every generated
//! type.  This matters because several BO4E names have no heuristic inverse —
//! `hoechstpreisHT`, `kundengruppeKA`, and `Sigmoidparameter`'s `A`/`B`/`C`/`D`
//! all render to a snake form that an algorithm would map back to a different
//! camelCase name, silently diverting the value into `_additional`.
//!
//! Two kinds of key are deliberately left alone in both directions:
//!
//! - **BO4E metadata keys** (`_typ`, `_version`, `_id`) keep their leading
//!   underscore in every mode; they are wire metadata, not Rust field names.
//! - **Extension keys** — anything the schema does not define — pass through
//!   byte-for-byte, so unknown fields round-trip exactly as they arrived instead
//!   of being rewritten into a name their producer would not recognize.
//!
//! [`to_json_german`]: crate::json::Bo4eJsonExt::to_json_german
//! [`to_json_snake_case`]: crate::json::Bo4eJsonExt::to_json_snake_case
//! [`to_json_canonical`]: crate::json::Bo4eJsonExt::to_json_canonical

pub(crate) mod depth;
pub(crate) mod extension;
pub(crate) mod key_transform;
pub(crate) mod limits;

// ── Public re-exports ──────────────────────────────────────────────────
pub use extension::{
    ext_map_is_empty, Bo4eExtensionData, LimitedExtensionMap, MAX_EXTENSION_FIELDS,
    MAX_EXTENSION_KEY_LEN,
};
pub use limits::{json_limit_hit_counters, JsonLimitHitCounters, JsonParseLimits};

// ── Internal imports from submodules ───────────────────────────────────
use depth::{DepthLimitedDeserializer, DepthState};
use key_transform::{
    camel_to_snake, deserialize_with_key_transform_from_slice,
    deserialize_with_key_transform_from_str, serialize_with_key_transform, snake_to_camel,
    KeyTransformDeserializer,
};
#[cfg(feature = "tracing")]
use limits::trace_json_outcome;
use limits::{
    check_payload_limit, deserialize_german_from_slice, deserialize_german_from_str,
    install_extension_budget, trace_deser_error,
};
#[cfg(feature = "tracing")]
use std::time::Instant;

use serde::de::DeserializeOwned;
use serde::Serialize;

// ─── Shared instrumentation ──────────────────────────────────────────────────
//
// Every entry point below emits the same tracing event around the same shape of
// work.  Threading that through two helpers keeps each method's body down to the
// part that actually differs, and compiles away entirely without `tracing`.

/// Runs a serializing entry point, recording its outcome when `tracing` is on.
#[inline]
fn traced_serialize<T: ?Sized>(
    mode: &'static str,
    op: impl FnOnce() -> Result<String, serde_json::Error>,
) -> Result<String, serde_json::Error> {
    #[cfg(not(feature = "tracing"))]
    {
        let _ = mode;
        op()
    }
    #[cfg(feature = "tracing")]
    {
        let started = Instant::now();
        let result = op();
        trace_json_outcome(
            "serialize",
            mode,
            std::any::type_name::<T>(),
            None,
            result.as_ref().ok().map(String::len),
            started,
            result.is_ok(),
        );
        result
    }
}

/// Runs a deserializing entry point, recording the error (always) and the
/// outcome event (when `tracing` is on).
#[inline]
fn traced_deserialize<S: ?Sized, T>(
    operation: &'static str,
    mode: &'static str,
    input_len: usize,
    context: &'static str,
    op: impl FnOnce() -> Result<T, serde_json::Error>,
) -> Result<T, serde_json::Error> {
    #[cfg(feature = "tracing")]
    let started = Instant::now();
    let result = op();
    trace_deser_error(&result, context);
    #[cfg(feature = "tracing")]
    trace_json_outcome(
        operation,
        mode,
        std::any::type_name::<S>(),
        Some(input_len),
        None,
        started,
        result.is_ok(),
    );
    #[cfg(not(feature = "tracing"))]
    let _ = (operation, mode, input_len);
    result
}

// ─── Sealed trait machinery ───────────────────────────────────────────────────

/// Prevents external crates from implementing [`Bo4eJsonExt`] on arbitrary types.
///
/// Only BO4E-generated structs (and types that opt in via the generator) carry
/// this impl.  This is the standard Rust sealed-trait pattern.
pub(crate) mod sealed {
    /// Marker trait — implement on a type to unlock [`super::Bo4eJsonExt`].
    pub trait Sealed {}
}

// ─── Public trait ─────────────────────────────────────────────────────────────

/// Extension methods for JSON serialization of BO4E types.
///
/// Implemented only on generated BO and COM structs via the sealed-trait pattern.
/// Import this trait (or `use rubo4e::prelude::*`) to access the methods.
pub trait Bo4eJsonExt: sealed::Sealed + Serialize + DeserializeOwned + Sized {
    /// Serializes `self` to a JSON string using the canonical BO4E German camelCase
    /// field names.
    ///
    /// Keys are written in insertion order (as defined by the generated serde impls).
    /// `None` / empty fields are omitted.
    ///
    /// # Errors
    /// Returns [`serde_json::Error`] if the value cannot be serialized.
    fn to_json_german(&self) -> Result<String, serde_json::Error> {
        traced_serialize::<Self>("german", || serde_json::to_string(self))
    }

    /// Serializes `self` to a JSON string with **alphabetically sorted** keys.
    ///
    /// This produces a deterministic byte representation regardless of struct
    /// field order, which is useful for content-addressed hashing, digital
    /// signatures, and test assertions.  Keys are sorted recursively throughout
    /// all nested objects.
    ///
    /// # Performance
    ///
    /// Each map level buffers `(key, serialized_value_bytes)` pairs, sorts them
    /// by key, then flushes them into the output.  No intermediate
    /// [`serde_json::Value`] tree is ever allocated.
    ///
    /// It is still meaningfully more expensive than [`to_json_german`]: each
    /// nested value is buffered and re-checked as a raw JSON fragment once per
    /// enclosing level, so cost grows with nesting depth as well as size.  BO4E
    /// documents are shallow (6–8 levels), which keeps that factor small — but
    /// on a hot path where key ordering is not required, prefer
    /// [`to_json_german`], which completes in one pass with one allocation.
    ///
    /// # Deviations from RFC 8785 / JCS
    ///
    /// This method produces **deterministic**, **sorted-key** JSON but does **not**
    /// implement the full [RFC 8785 JSON Canonicalization Scheme (JCS)].  The
    /// known deviations are:
    ///
    /// | JCS requirement | This implementation |
    /// |---|---|
    /// | Keys sorted by UTF-16 code units | Keys sorted by Rust `str` byte order (UTF-8); differs for non-ASCII keys above U+007F |
    /// | IEEE 754 decimal number serialization (e.g. `1.0e1`) | serde_json number formatting (e.g. `10`) |
    /// | Unicode normalization not required (JCS passes through) | No normalization applied — same behaviour |
    /// | Output encoding: UTF-8 | UTF-8 |
    ///
    /// For **BO4E data** (field names are ASCII-only, numeric values are integers
    /// or rust_decimal decimals serialized as strings) these deviations are
    /// irrelevant in practice.  For cross-language JCS interoperability, use a
    /// dedicated JCS library.
    ///
    /// [RFC 8785 JSON Canonicalization Scheme (JCS)]: https://www.rfc-editor.org/rfc/rfc8785
    ///
    /// # Errors
    /// Returns [`serde_json::Error`] if the value cannot be serialized.
    ///
    /// [`to_json_german`]: Bo4eJsonExt::to_json_german
    fn to_json_canonical(&self) -> Result<String, serde_json::Error> {
        traced_serialize::<Self>("canonical", || {
            let mut out = Vec::new();
            {
                let mut ser = serde_json::Serializer::new(&mut out);
                self.serialize(SortedSerializer { inner: &mut ser })?;
            }
            // serde_json::Serializer always produces valid UTF-8.
            Ok(String::from_utf8(out).expect("serde_json always produces valid UTF-8"))
        })
    }

    /// Serializes `self` to a JSON string using Rust **snake_case** field names as keys.
    ///
    /// The output keys are the snake_case equivalents of the BO4E German camelCase names
    /// (e.g. `"marktlokationsId"` → `"marktlokations_id"`).  Fields whose names start
    /// with `_` (such as `_typ`, `_version`, `_additional`) are preserved as-is.
    ///
    /// Pair with [`from_json_snake_case`] to round-trip data in this format.
    ///
    /// Internally serialization is performed in a single pass with a key-transform
    /// serializer wrapper. This avoids constructing an intermediate
    /// [`serde_json::Value`] tree.
    ///
    /// # Errors
    /// Returns [`serde_json::Error`] if the value cannot be serialized.
    ///
    /// [`from_json_snake_case`]: Bo4eJsonExt::from_json_snake_case
    fn to_json_snake_case(&self) -> Result<String, serde_json::Error> {
        traced_serialize::<Self>("snake_case", || {
            serialize_with_key_transform(self, camel_to_snake)
        })
    }

    /// Deserializes from a JSON string produced by [`to_json_snake_case`].
    ///
    /// Converts snake_case keys back to German camelCase with a streaming
    /// key-transform deserializer before handing off to serde-derived BO4E types.
    ///
    /// This avoids constructing an intermediate `serde_json::Value` tree.
    ///
    /// # Errors
    /// Returns [`serde_json::Error`] on malformed JSON or type mismatches.
    ///
    /// # Diagnostics
    ///
    /// Structured debug logs are emitted only when the `tracing` feature is enabled.
    /// Without `tracing`, errors are still returned normally but no internal log event
    /// is emitted by this crate.
    ///
    /// [`to_json_snake_case`]: Bo4eJsonExt::to_json_snake_case
    /// [`from_json_snake_case_bytes`]: Bo4eJsonExt::from_json_snake_case_bytes
    fn from_json_snake_case(s: &str) -> Result<Self, serde_json::Error> {
        traced_deserialize::<Self, _>(
            "deserialize",
            "snake_case_str",
            s.len(),
            "BO4E deserialization failed in from_json_snake_case",
            || deserialize_with_key_transform_from_str::<Self, _>(s, &snake_to_camel),
        )
    }

    /// Byte-slice variant of [`from_json_snake_case`] for callers that already hold
    /// JSON data in memory.
    ///
    /// Accepts an immutable `&[u8]` reference.  Uses the same streaming
    /// key-transform deserializer and avoids constructing an intermediate
    /// `serde_json::Value` tree.
    ///
    /// # Errors
    /// Returns [`serde_json::Error`] on malformed JSON or type mismatches.
    ///
    /// [`from_json_snake_case`]: Bo4eJsonExt::from_json_snake_case
    fn from_json_snake_case_bytes(bytes: &[u8]) -> Result<Self, serde_json::Error> {
        traced_deserialize::<Self, _>(
            "deserialize",
            "snake_case_bytes",
            bytes.len(),
            "BO4E deserialization failed in from_json_snake_case_bytes",
            // Streamed key transformation avoids building an intermediate Value tree.
            || deserialize_with_key_transform_from_slice::<Self, _>(bytes, &snake_to_camel),
        )
    }

    /// Deserializes from a JSON string produced by [`to_json_german`] or any other
    /// BO4E-compatible JSON serializer.
    ///
    /// With `simd-json` enabled this method uses an adaptive strategy:
    ///
    /// - **small payloads**: deserialize via `serde_json::from_str` to avoid the
    ///   temporary `Vec<u8>` copy needed by simd-json's mutable-slice API.
    /// - **larger payloads**: copy once into `Vec<u8>` and delegate to
    ///   [`from_json_german_bytes`] (SIMD parser).
    ///
    /// If you already own or can borrow a mutable buffer, call
    /// [`from_json_german_bytes`] directly to avoid this heuristic path.
    ///
    /// # Errors
    /// Returns [`serde_json::Error`] if the value cannot be deserialized.
    ///
    /// # Diagnostics
    ///
    /// Structured debug logs are emitted only when the `tracing` feature is enabled.
    /// Without `tracing`, errors are still returned normally but no internal log event
    /// is emitted by this crate.
    ///
    /// [`to_json_german`]: Bo4eJsonExt::to_json_german
    /// [`from_json_german_bytes`]: Bo4eJsonExt::from_json_german_bytes
    fn from_json_german(s: &str) -> Result<Self, serde_json::Error> {
        traced_deserialize::<Self, _>(
            "deserialize",
            "german_str",
            s.len(),
            "BO4E deserialization failed in from_json_german",
            || deserialize_german_from_str::<Self>(s),
        )
    }

    /// Byte-slice variant of [`from_json_german`] for callers that already hold
    /// JSON data in memory as a byte slice.
    ///
    /// Accepts an immutable `&[u8]` reference.  When the `simd-json` feature is
    /// enabled and the payload exceeds the SIMD activation threshold (≥ 1,536 bytes
    /// by default), the slice is cloned into a temporary `Vec<u8>` buffer internally
    /// so that `simd_json` can mutate it in-place.  The cost of that copy is small
    /// relative to the parse speedup on larger payloads.
    ///
    /// # When to prefer this over [`from_json_german`]
    ///
    /// - You read JSON bytes from I/O into a `Vec<u8>` or `Bytes` buffer.
    /// - You want to avoid the UTF-8 validation that converting to `&str` requires.
    ///
    /// ```
    /// # #[cfg(feature = "versioned")] {
    /// use rubo4e::{current::Marktlokation, json::Bo4eJsonExt};
    ///
    /// let buf: Vec<u8> = br#"{"marktlokationsId":"51238696781"}"#.to_vec();
    /// let malo = Marktlokation::from_json_german_bytes(&buf).unwrap();
    /// assert!(malo.marktlokations_id.is_some());
    /// # }
    /// ```
    ///
    /// # Errors
    /// Returns [`serde_json::Error`] if the value cannot be deserialized.
    ///
    /// [`from_json_german`]: Bo4eJsonExt::from_json_german
    fn from_json_german_bytes(bytes: &[u8]) -> Result<Self, serde_json::Error> {
        traced_deserialize::<Self, _>(
            "deserialize",
            "german_bytes",
            bytes.len(),
            "BO4E deserialization failed in from_json_german_bytes",
            || deserialize_german_from_slice(bytes),
        )
    }

    /// Hardened deserialization from BO4E German camelCase JSON with optional
    /// resource limits for untrusted inputs.
    ///
    /// Compared to [`Bo4eJsonExt::from_json_german`], this variant additionally enforces:
    /// - optional payload byte-size cap,
    /// - optional maximum nesting depth,
    /// - optional extension-data value budget.
    ///
    /// # Errors
    /// Returns [`serde_json::Error`] on malformed JSON, type mismatch, or
    /// when any configured limit is exceeded.
    fn from_json_german_hardened(
        s: &str,
        limits: JsonParseLimits,
    ) -> Result<Self, serde_json::Error> {
        check_payload_limit(s.len(), limits)?;
        traced_deserialize::<Self, _>(
            "deserialize_hardened",
            "german_str",
            s.len(),
            "BO4E hardened deserialization failed in from_json_german_hardened",
            || {
                let _budget = install_extension_budget(limits);
                match limits.max_nesting_depth {
                    // Depth is tracked inline by DepthLimitedDeserializer.  When
                    // simd-json is enabled this path still uses serde_json, because
                    // simd-json cannot wrap visitors; correctness takes priority.
                    Some(max_depth) => {
                        let state = DepthState::new(max_depth);
                        let mut de = serde_json::Deserializer::from_str(s);
                        Self::deserialize(DepthLimitedDeserializer::new(&mut de, &state))
                    }
                    None => deserialize_german_from_str::<Self>(s),
                }
            },
        )
    }

    /// Hardened deserialization from snake_case JSON with optional resource
    /// limits for untrusted inputs.
    ///
    /// Keys are rewritten from snake_case to BO4E camelCase in the same pass that
    /// deserializes them — there is no intermediate parse — and the same limits
    /// as [`Bo4eJsonExt::from_json_german_hardened`] apply, with
    /// `max_payload_bytes` checked up front and the rest enforced during parsing.
    ///
    /// # Errors
    /// Returns [`serde_json::Error`] on malformed JSON, type mismatch, or
    /// when any configured limit is exceeded.
    fn from_json_snake_case_hardened(
        s: &str,
        limits: JsonParseLimits,
    ) -> Result<Self, serde_json::Error> {
        check_payload_limit(s.len(), limits)?;
        traced_deserialize::<Self, _>(
            "deserialize_hardened",
            "snake_case_str",
            s.len(),
            "BO4E hardened deserialization failed in from_json_snake_case_hardened",
            || {
                let _budget = install_extension_budget(limits);
                match limits.max_nesting_depth {
                    Some(max_depth) => {
                        let state = DepthState::new(max_depth);
                        let mut de = serde_json::Deserializer::from_str(s);
                        Self::deserialize(KeyTransformDeserializer::new(
                            DepthLimitedDeserializer::new(&mut de, &state),
                            &snake_to_camel,
                        ))
                    }
                    None => deserialize_with_key_transform_from_str::<Self, _>(s, &snake_to_camel),
                }
            },
        )
    }

    /// Hardened byte-slice variant of [`from_json_german_bytes`].
    ///
    /// Applies all limits in `limits`: payload size, nesting depth, and extension-field
    /// budget.  See [`JsonParseLimits::untrusted_defaults`] for the recommended settings
    /// when processing input from untrusted callers.
    ///
    /// Accepts an immutable `&[u8]` reference.
    ///
    /// Setting `max_nesting_depth` pins this call to the `serde_json` backend even
    /// when `simd-json` is enabled, because depth is enforced by wrapping the
    /// visitor and `simd-json` does not support that. Since
    /// [`JsonParseLimits::untrusted_defaults`] sets a depth cap, the recommended
    /// hardened configuration never takes the SIMD path — correctness over
    /// throughput on untrusted input. Leave `max_nesting_depth` at `None` to keep
    /// the adaptive `simd-json` behaviour of [`from_json_german_bytes`], which
    /// still enforces the default depth cap via a pre-scan.
    ///
    /// # Errors
    /// Returns [`serde_json::Error`] on malformed JSON, type mismatch, or when
    /// any configured limit is exceeded.
    ///
    /// [`from_json_german_bytes`]: Bo4eJsonExt::from_json_german_bytes
    fn from_json_german_bytes_hardened(
        bytes: &[u8],
        limits: JsonParseLimits,
    ) -> Result<Self, serde_json::Error> {
        check_payload_limit(bytes.len(), limits)?;
        traced_deserialize::<Self, _>(
            "deserialize_hardened",
            "german_bytes",
            bytes.len(),
            "BO4E hardened deserialization failed in from_json_german_bytes_hardened",
            || {
                let _budget = install_extension_budget(limits);
                match limits.max_nesting_depth {
                    Some(max_depth) => {
                        let state = DepthState::new(max_depth);
                        let mut de = serde_json::Deserializer::from_slice(bytes);
                        Self::deserialize(DepthLimitedDeserializer::new(&mut de, &state))
                    }
                    None => deserialize_german_from_slice::<Self>(bytes),
                }
            },
        )
    }

    /// Hardened byte-slice variant of [`Bo4eJsonExt::from_json_snake_case_bytes`].
    ///
    /// # Errors
    /// Returns [`serde_json::Error`] on malformed JSON, type mismatch, or when
    /// any configured limit is exceeded.
    fn from_json_snake_case_bytes_hardened(
        bytes: &[u8],
        limits: JsonParseLimits,
    ) -> Result<Self, serde_json::Error> {
        check_payload_limit(bytes.len(), limits)?;
        traced_deserialize::<Self, _>(
            "deserialize_hardened",
            "snake_case_bytes",
            bytes.len(),
            "BO4E hardened deserialization failed in from_json_snake_case_bytes_hardened",
            || {
                let _budget = install_extension_budget(limits);
                match limits.max_nesting_depth {
                    Some(max_depth) => {
                        let state = DepthState::new(max_depth);
                        let mut de = serde_json::Deserializer::from_slice(bytes);
                        Self::deserialize(KeyTransformDeserializer::new(
                            DepthLimitedDeserializer::new(&mut de, &state),
                            &snake_to_camel,
                        ))
                    }
                    None => {
                        deserialize_with_key_transform_from_slice::<Self, _>(bytes, &snake_to_camel)
                    }
                }
            },
        )
    }
}

// ─── Single-pass sorted serializer ───────────────────────────────────────────
//
// Sorts object keys without ever building an intermediate serde_json::Value tree.
//
// Strategy per map level:
//   1. Serialize each entry's value into a temporary Vec<u8> buffer.
//   2. Collect (key, bytes) pairs.
//   3. Sort by key.
//   4. Flush all pairs into the real writer in sorted order.
//
// This halves the allocations for canonical serialization: no Value nodes are
// allocated, only one String per map key and one Vec<u8> per map value (which is
// drained as soon as the map level is done).  For arrays and scalars we delegate
// directly to the inner serializer with no intermediate buffering.

/// Serializer wrapper that sorts map/object keys before writing them.
///
/// Values are pre-serialized into byte buffers, sorted by key, then flushed.
struct SortedSerializer<S> {
    inner: S,
}

struct SortedMap<S> {
    inner: S,
    entries: Vec<(String, Vec<u8>)>,
}

impl<S: serde::ser::Serializer> serde::ser::SerializeMap for SortedMap<S> {
    type Ok = S::Ok;
    type Error = S::Error;

    fn serialize_key<T: ?Sized + Serialize>(&mut self, _key: &T) -> Result<(), Self::Error> {
        // serialize_entry is the only supported map API; serde's derived impls always
        // use it.  This path is structurally unreachable for BO4E types.
        debug_assert!(
            false,
            "SortedMap: serialize_key called without serialize_value; use serialize_entry"
        );
        Err(serde::ser::Error::custom(
            "internal: SortedMap requires serialize_entry; serialize_key/serialize_value are unsupported",
        ))
    }

    fn serialize_value<T: ?Sized + Serialize>(&mut self, _value: &T) -> Result<(), Self::Error> {
        debug_assert!(
            false,
            "SortedMap: serialize_value called without serialize_key; use serialize_entry"
        );
        Err(serde::ser::Error::custom(
            "internal: SortedMap requires serialize_entry; serialize_key/serialize_value are unsupported",
        ))
    }

    fn serialize_entry<K: ?Sized + Serialize, V: ?Sized + Serialize>(
        &mut self,
        key: &K,
        value: &V,
    ) -> Result<(), Self::Error> {
        // Capture the key as a String.  We serialize into a temp serde_json buffer;
        // the concrete serde_json::Error is bridged to S::Error via custom().
        let key_str = {
            let mut kbuf = Vec::with_capacity(32);
            let mut kser = serde_json::Serializer::new(&mut kbuf);
            key.serialize(&mut kser)
                .map_err(|e| serde::ser::Error::custom(e.to_string()))?;
            // JSON-serialised string includes surrounding quotes — strip them.
            let raw = String::from_utf8(kbuf)
                .map_err(|e| serde::ser::Error::custom(format!("key utf-8: {e}")))?;
            if raw.starts_with('"') && raw.ends_with('"') && raw.len() >= 2 {
                raw[1..raw.len() - 1].to_owned()
            } else {
                raw
            }
        };
        // Recursively serialize the value through SortedSerializer into a temp buffer.
        let mut vbuf = Vec::new();
        {
            let mut vser = serde_json::Serializer::new(&mut vbuf);
            value
                .serialize(SortedSerializer { inner: &mut vser })
                .map_err(|e| serde::ser::Error::custom(e.to_string()))?;
        }
        self.entries.push((key_str, vbuf));
        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        self.entries.sort_unstable_by(|(a, _), (b, _)| a.cmp(b));
        let mut map = self.inner.serialize_map(Some(self.entries.len()))?;
        // Drain entries so value_bytes is moved out rather than cloned.
        for (key, value_bytes) in self.entries.drain(..) {
            // Write the pre-serialised value as a raw JSON fragment.
            // serde_json::Serializer always produces valid UTF-8 and valid JSON.
            let raw_value = serde_json::value::RawValue::from_string(
                String::from_utf8(value_bytes).expect("serde_json always produces valid UTF-8"),
            )
            .map_err(serde::ser::Error::custom)?;
            map.serialize_entry(key.as_str(), &raw_value)?;
        }
        map.end()
    }
}

struct SortedSeq<S> {
    inner: S,
    buf: Vec<Vec<u8>>,
}

impl<S: serde::ser::Serializer> serde::ser::SerializeSeq for SortedSeq<S> {
    type Ok = S::Ok;
    type Error = S::Error;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        let mut vbuf = Vec::new();
        {
            let mut vser = serde_json::Serializer::new(&mut vbuf);
            value
                .serialize(SortedSerializer { inner: &mut vser })
                .map_err(|e| serde::ser::Error::custom(e.to_string()))?;
        }
        self.buf.push(vbuf);
        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        let mut seq = self.inner.serialize_seq(Some(self.buf.len()))?;
        // Drain buffers so vbuf is moved out rather than cloned.
        for vbuf in self.buf.drain(..) {
            // serde_json::Serializer always produces valid UTF-8 and valid JSON.
            let raw_value = serde_json::value::RawValue::from_string(
                String::from_utf8(vbuf).expect("serde_json always produces valid UTF-8"),
            )
            .map_err(serde::ser::Error::custom)?;
            seq.serialize_element(&raw_value)?;
        }
        seq.end()
    }
}

impl<S: serde::ser::Serializer> serde::ser::Serializer for SortedSerializer<S> {
    type Ok = S::Ok;
    type Error = S::Error;
    type SerializeSeq = SortedSeq<S>;
    type SerializeTuple = S::SerializeTuple;
    type SerializeTupleStruct = S::SerializeTupleStruct;
    type SerializeTupleVariant = S::SerializeTupleVariant;
    type SerializeMap = SortedMap<S>;
    type SerializeStruct = SortedMap<S>;
    type SerializeStructVariant = S::SerializeStructVariant;

    fn serialize_bool(self, v: bool) -> Result<S::Ok, S::Error> {
        self.inner.serialize_bool(v)
    }
    fn serialize_i8(self, v: i8) -> Result<S::Ok, S::Error> {
        self.inner.serialize_i8(v)
    }
    fn serialize_i16(self, v: i16) -> Result<S::Ok, S::Error> {
        self.inner.serialize_i16(v)
    }
    fn serialize_i32(self, v: i32) -> Result<S::Ok, S::Error> {
        self.inner.serialize_i32(v)
    }
    fn serialize_i64(self, v: i64) -> Result<S::Ok, S::Error> {
        self.inner.serialize_i64(v)
    }
    fn serialize_u8(self, v: u8) -> Result<S::Ok, S::Error> {
        self.inner.serialize_u8(v)
    }
    fn serialize_u16(self, v: u16) -> Result<S::Ok, S::Error> {
        self.inner.serialize_u16(v)
    }
    fn serialize_u32(self, v: u32) -> Result<S::Ok, S::Error> {
        self.inner.serialize_u32(v)
    }
    fn serialize_u64(self, v: u64) -> Result<S::Ok, S::Error> {
        self.inner.serialize_u64(v)
    }
    fn serialize_f32(self, v: f32) -> Result<S::Ok, S::Error> {
        self.inner.serialize_f32(v)
    }
    fn serialize_f64(self, v: f64) -> Result<S::Ok, S::Error> {
        self.inner.serialize_f64(v)
    }
    fn serialize_char(self, v: char) -> Result<S::Ok, S::Error> {
        self.inner.serialize_char(v)
    }
    fn serialize_str(self, v: &str) -> Result<S::Ok, S::Error> {
        self.inner.serialize_str(v)
    }
    fn serialize_bytes(self, v: &[u8]) -> Result<S::Ok, S::Error> {
        self.inner.serialize_bytes(v)
    }
    fn serialize_none(self) -> Result<S::Ok, S::Error> {
        self.inner.serialize_none()
    }
    fn serialize_unit(self) -> Result<S::Ok, S::Error> {
        self.inner.serialize_unit()
    }
    fn serialize_unit_struct(self, name: &'static str) -> Result<S::Ok, S::Error> {
        self.inner.serialize_unit_struct(name)
    }
    fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<S::Ok, S::Error> {
        self.inner.serialize_some(&SortedSerializeValue(value))
    }
    fn serialize_unit_variant(
        self,
        name: &'static str,
        vi: u32,
        v: &'static str,
    ) -> Result<S::Ok, S::Error> {
        self.inner.serialize_unit_variant(name, vi, v)
    }
    fn serialize_newtype_struct<T: ?Sized + Serialize>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<S::Ok, S::Error> {
        self.inner
            .serialize_newtype_struct(name, &SortedSerializeValue(value))
    }
    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        self,
        name: &'static str,
        vi: u32,
        v: &'static str,
        value: &T,
    ) -> Result<S::Ok, S::Error> {
        self.inner
            .serialize_newtype_variant(name, vi, v, &SortedSerializeValue(value))
    }
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, S::Error> {
        Ok(SortedSeq {
            inner: self.inner,
            buf: Vec::new(),
        })
    }
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, S::Error> {
        self.inner.serialize_tuple(len)
    }
    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, S::Error> {
        self.inner.serialize_tuple_struct(name, len)
    }
    fn serialize_tuple_variant(
        self,
        name: &'static str,
        vi: u32,
        v: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, S::Error> {
        self.inner.serialize_tuple_variant(name, vi, v, len)
    }
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, S::Error> {
        Ok(SortedMap {
            inner: self.inner,
            entries: Vec::new(),
        })
    }
    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, S::Error> {
        Ok(SortedMap {
            inner: self.inner,
            entries: Vec::new(),
        })
    }
    fn serialize_struct_variant(
        self,
        name: &'static str,
        vi: u32,
        v: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, S::Error> {
        self.inner.serialize_struct_variant(name, vi, v, len)
    }
    fn is_human_readable(&self) -> bool {
        self.inner.is_human_readable()
    }
    fn collect_str<T: ?Sized + std::fmt::Display>(self, value: &T) -> Result<S::Ok, S::Error> {
        self.inner.collect_str(value)
    }
}

// `SerializeStruct` delegates to `SortedMap` — structs become sorted objects.
impl<S: serde::ser::Serializer> serde::ser::SerializeStruct for SortedMap<S> {
    type Ok = S::Ok;
    type Error = S::Error;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        // Reuse serialize_entry from SerializeMap
        serde::ser::SerializeMap::serialize_entry(self, key, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        serde::ser::SerializeMap::end(self)
    }
}

/// Newtype wrapper so we can pass `&T` through a sorted serializer without
/// boxing the value.
struct SortedSerializeValue<'a, T: ?Sized>(&'a T);

impl<T: ?Sized + Serialize> Serialize for SortedSerializeValue<'_, T> {
    fn serialize<S: serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(SortedSerializer { inner: serializer })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indexmap::IndexMap;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Sample {
        #[serde(rename = "zField")]
        z_field: String,
        #[serde(rename = "aField")]
        a_field: u32,
        #[serde(rename = "mField", skip_serializing_if = "Option::is_none")]
        m_field: Option<String>,
    }

    // Test-local types must opt in explicitly (sealed trait + Bo4eJsonExt).
    impl sealed::Sealed for Sample {}
    impl Bo4eJsonExt for Sample {}

    #[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
    struct HardenedSample {
        #[serde(rename = "name")]
        name: String,
        #[serde(flatten)]
        #[serde(skip_serializing_if = "crate::json::ext_map_is_empty")]
        _additional: LimitedExtensionMap,
    }

    impl sealed::Sealed for HardenedSample {}
    impl Bo4eJsonExt for HardenedSample {}
    impl Bo4eExtensionData for HardenedSample {
        fn extension_data(&self) -> &IndexMap<String, serde_json::Value> {
            static EMPTY: std::sync::LazyLock<IndexMap<String, serde_json::Value>> =
                std::sync::LazyLock::new(IndexMap::new);
            self._additional.as_map().unwrap_or(&EMPTY)
        }

        fn has_extension_data(&self) -> bool {
            !self._additional.is_empty()
        }
    }

    #[test]
    fn to_json_german_round_trips() {
        let v = Sample {
            z_field: "hello".into(),
            a_field: 42,
            m_field: None,
        };
        let json = v.to_json_german().unwrap();
        let back: Sample = Sample::from_json_german(&json).unwrap();
        assert_eq!(v, back);
    }

    #[test]
    fn from_json_german_bytes_round_trips() {
        let v = Sample {
            z_field: "hello".into(),
            a_field: 42,
            m_field: None,
        };
        let json = v.to_json_german().unwrap();
        let back: Sample = Sample::from_json_german_bytes(&json.into_bytes()).unwrap();
        assert_eq!(v, back);
    }

    #[test]
    fn from_json_snake_case_bytes_round_trips() {
        let v = Sample {
            z_field: "x".into(),
            a_field: 7,
            m_field: Some("y".into()),
        };
        let json = v.to_json_snake_case().unwrap();
        let back = Sample::from_json_snake_case_bytes(&json.into_bytes()).unwrap();
        assert_eq!(v, back);
    }

    #[test]
    fn bytes_and_str_variants_are_equivalent() {
        let v = Sample {
            z_field: "abc".into(),
            a_field: 99,
            m_field: Some("z".into()),
        };
        let german = v.to_json_german().unwrap();
        let snake = v.to_json_snake_case().unwrap();
        assert_eq!(
            Sample::from_json_german(&german).unwrap(),
            Sample::from_json_german_bytes(&german.clone().into_bytes()).unwrap(),
        );
        assert_eq!(
            Sample::from_json_snake_case(&snake).unwrap(),
            Sample::from_json_snake_case_bytes(&snake.clone().into_bytes()).unwrap(),
        );
    }

    #[test]
    fn to_json_german_uses_serde_rename() {
        let v = Sample {
            z_field: "hello".into(),
            a_field: 42,
            m_field: None,
        };
        let json = v.to_json_german().unwrap();
        assert!(
            json.contains("\"zField\""),
            "expected German camelCase key: {json}"
        );
        assert!(
            json.contains("\"aField\""),
            "expected German camelCase key: {json}"
        );
    }

    /// Fixture for the snake_case mode, which only rewrites keys the generator
    /// put in the key map — so it has to be built from real BO4E wire names.
    ///
    /// `hoechstpreisHT` is deliberate: it is one of the keys whose snake form is
    /// not recoverable by a heuristic, so it pins the regression.
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct SnakeSample {
        #[serde(rename = "marktlokationsId")]
        marktlokations_id: String,
        #[serde(rename = "anteilProzent")]
        anteil_prozent: u32,
        #[serde(rename = "hoechstpreisHT", skip_serializing_if = "Option::is_none")]
        hoechstpreis_ht: Option<String>,
    }

    impl sealed::Sealed for SnakeSample {}
    impl Bo4eJsonExt for SnakeSample {}

    #[test]
    fn to_json_snake_case_uses_snake_case_keys() {
        let v = SnakeSample {
            marktlokations_id: "51238696781".into(),
            anteil_prozent: 42,
            hoechstpreis_ht: None,
        };
        let json = v.to_json_snake_case().unwrap();
        assert!(
            json.contains("\"marktlokations_id\""),
            "expected snake_case key: {json}"
        );
        assert!(
            json.contains("\"anteil_prozent\""),
            "expected snake_case key: {json}"
        );
        assert!(
            !json.contains("\"marktlokationsId\""),
            "must not contain camelCase: {json}"
        );
    }

    #[test]
    fn to_json_snake_case_round_trips() {
        let v = SnakeSample {
            marktlokations_id: "51238696781".into(),
            anteil_prozent: 7,
            hoechstpreis_ht: Some("12.50".into()),
        };
        let json = v.to_json_snake_case().unwrap();
        let back = SnakeSample::from_json_snake_case(&json).unwrap();
        assert_eq!(v, back);
    }

    /// A wire key whose snake form a heuristic cannot invert must still land in
    /// its typed field, not in the extension-data bag.
    #[test]
    fn acronym_keys_survive_snake_case_round_trip() {
        let v = SnakeSample {
            marktlokations_id: "51238696781".into(),
            anteil_prozent: 1,
            hoechstpreis_ht: Some("99.00".into()),
        };
        let json = v.to_json_snake_case().unwrap();
        assert!(
            json.contains("\"hoechstpreis_ht\""),
            "expected snake_case key: {json}"
        );
        let back = SnakeSample::from_json_snake_case(&json).unwrap();
        assert_eq!(back.hoechstpreis_ht.as_deref(), Some("99.00"));
        assert_eq!(v, back);
    }

    #[test]
    fn underscore_prefixed_keys_survive_snake_case_round_trip() {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct WithMeta {
            #[serde(rename = "_typ")]
            typ: String,
            #[serde(rename = "anteilProzent")]
            anteil_prozent: u32,
        }
        impl sealed::Sealed for WithMeta {}
        impl Bo4eJsonExt for WithMeta {}
        let v = WithMeta {
            typ: "Vertrag".into(),
            anteil_prozent: 1,
        };
        let json = v.to_json_snake_case().unwrap();
        // _typ is BO4E metadata, not a Rust field name — it must NOT be renamed.
        assert!(json.contains("\"_typ\""), "_typ must survive: {json}");
        assert!(
            json.contains("\"anteil_prozent\""),
            "anteil_prozent must appear: {json}"
        );
        let back = WithMeta::from_json_snake_case(&json).unwrap();
        assert_eq!(v, back);
    }

    #[test]
    fn to_json_canonical_sorts_keys() {
        let v = Sample {
            z_field: "z".into(),
            a_field: 1,
            m_field: Some("m".into()),
        };
        let canonical = v.to_json_canonical().unwrap();
        // Keys must appear in alphabetical order: aField, mField, zField.
        let a = canonical.find("aField").unwrap();
        let m = canonical.find("mField").unwrap();
        let z = canonical.find("zField").unwrap();
        assert!(a < m && m < z, "keys not sorted: {canonical}");
    }

    #[test]
    fn to_json_canonical_nested_objects_sorted() {
        #[derive(Serialize, Deserialize)]
        struct Outer {
            #[serde(rename = "zField")]
            z: Inner,
            #[serde(rename = "aField")]
            a: u32,
        }
        #[derive(Serialize, Deserialize)]
        struct Inner {
            #[serde(rename = "zInner")]
            z_inner: String,
            #[serde(rename = "aInner")]
            a_inner: u32,
        }
        impl sealed::Sealed for Outer {}
        impl Bo4eJsonExt for Outer {}
        impl sealed::Sealed for Inner {}
        impl Bo4eJsonExt for Inner {}

        let v = Outer {
            z: Inner {
                z_inner: "z".into(),
                a_inner: 1,
            },
            a: 99,
        };
        let canonical = v.to_json_canonical().unwrap();
        let outer_a = canonical.find("\"aField\"").unwrap();
        let outer_z = canonical.find("\"zField\"").unwrap();
        assert!(outer_a < outer_z, "outer keys not sorted: {canonical}");

        let inner_a = canonical.find("aInner").unwrap();
        let inner_z = canonical.find("zInner").unwrap();
        assert!(inner_a < inner_z, "inner keys not sorted: {canonical}");
    }

    #[test]
    fn to_json_canonical_is_deterministic() {
        let v = Sample {
            z_field: "abc".into(),
            a_field: 7,
            m_field: None,
        };
        assert_eq!(
            v.to_json_canonical().unwrap(),
            v.to_json_canonical().unwrap()
        );
    }

    #[test]
    fn hardened_german_rejects_payload_limit() {
        let json = r#"{"name":"abc"}"#;
        let err = HardenedSample::from_json_german_hardened(
            json,
            JsonParseLimits {
                max_payload_bytes: Some(5),
                ..JsonParseLimits::default()
            },
        )
        .unwrap_err();
        assert!(
            err.to_string().contains("payload too large"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn hardened_german_rejects_depth_limit() {
        let json = r#"{"name":"ok","nested":{"a":{"b":1}}}"#;
        let err = HardenedSample::from_json_german_hardened(
            json,
            JsonParseLimits {
                max_nesting_depth: Some(2),
                ..JsonParseLimits::default()
            },
        )
        .unwrap_err();
        assert!(
            err.to_string().contains("nesting depth"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn hardened_german_rejects_extension_budget() {
        let json = r#"{"name":"ok","x":{"very":"large extension payload"}}"#;
        let err = HardenedSample::from_json_german_hardened(
            json,
            JsonParseLimits {
                max_extension_value_bytes: Some(4),
                ..JsonParseLimits::default()
            },
        )
        .unwrap_err();
        assert!(
            err.to_string().contains("extension value budget exceeded"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn hardened_snake_case_accepts_within_limits() {
        let json = r#"{"name":"ok","custom_field":"x"}"#;
        let parsed = HardenedSample::from_json_snake_case_hardened(
            json,
            JsonParseLimits {
                max_payload_bytes: Some(1024),
                max_nesting_depth: Some(8),
                max_extension_value_bytes: Some(64),
                max_extension_field_count: Some(16),
            },
        )
        .unwrap();
        assert_eq!(parsed.name, "ok");
        assert!(parsed.has_extension_data());
    }

    #[test]
    fn hardened_bytes_paths_round_trip() {
        let json = r#"{"name":"ok","extra":{"x":1}}"#.as_bytes().to_vec();
        let parsed_german = HardenedSample::from_json_german_bytes_hardened(
            &json.clone(),
            JsonParseLimits::untrusted_defaults(),
        )
        .unwrap();
        assert_eq!(parsed_german.name, "ok");

        let snake = r#"{"name":"ok","extra_field":1}"#.as_bytes().to_vec();
        let parsed_snake = HardenedSample::from_json_snake_case_bytes_hardened(
            &snake.clone(),
            JsonParseLimits::untrusted_defaults(),
        )
        .unwrap();
        assert_eq!(parsed_snake.name, "ok");
    }

    #[test]
    fn hardened_limit_counters_increment() {
        let before = json_limit_hit_counters();

        let json = r#"{"name":"abc"}"#;
        let _ = HardenedSample::from_json_german_hardened(
            json,
            JsonParseLimits {
                max_payload_bytes: Some(1),
                ..JsonParseLimits::default()
            },
        );

        let after = json_limit_hit_counters();
        assert!(
            after.payload_bytes > before.payload_bytes,
            "payload limit counter did not increase: before={before:?} after={after:?}"
        );
    }

    #[cfg(feature = "serde")]
    #[test]
    fn identifier_deser_failure_counter_increments() {
        let before = crate::identifiers::identifier_deser_failure_count();
        // Correct length and character set, but the check digit is wrong
        // (`41373559241` is the valid form).
        let _ = serde_json::from_str::<crate::identifiers::MaloId>("\"41373559242\"");
        let after = crate::identifiers::identifier_deser_failure_count();
        assert!(
            after > before,
            "identifier failure counter did not increase: before={before} after={after}"
        );
    }

    // ─── Helper function unit tests ───────────────────────────────────────────
}
