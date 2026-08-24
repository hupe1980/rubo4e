//! Extension-data map with a hard deserialization count limit, and the
//! [`Bo4eExtensionData`] accessor trait for generated BO/COM structs.
//!
//! The per-call extension budget installed by the hardened entry points is
//! charged from here, in `LimitedExtensionMap::deserialize`, because that is the
//! one place every struct's extension fields pass through — at every nesting
//! level, not just the root.

use serde_json::Value;

use super::limits::{
    budget_max_fields_per_struct, charge_extension_bytes, trace_limit_violation, LimitKind,
};
use super::sealed;

/// Approximate encoded size of `value`, used to charge the extension byte budget.
///
/// Deliberately an estimate rather than a re-serialization: the goal is to bound
/// retained memory, and re-serializing every captured value to measure it would
/// cost more than the budget protects against.
pub(super) fn estimated_json_value_bytes(value: &Value) -> usize {
    match value {
        Value::Null => 4, // "null"
        Value::Bool(b) => {
            if *b {
                4
            } else {
                5
            }
        } // "true" / "false"
        Value::Number(_) => 8, // conservative (covers i64/f64)
        Value::String(s) => s.len(),
        Value::Array(items) => items.iter().map(estimated_json_value_bytes).sum(),
        Value::Object(map) => map
            .iter()
            .map(|(k, v)| k.len() + estimated_json_value_bytes(v))
            .sum(),
    }
}

// ─── Extension-data accessor trait ───────────────────────────────────────────

/// Hard upper bound on the number of unknown extension fields accepted per struct.
///
/// Enforced during deserialization by [`LimitedExtensionMap`]: payloads that carry
/// more than this many extra keys are rejected with a [`serde`] error, preventing
/// unbounded `IndexMap` growth from adversarial JSON (DoS protection).
#[cfg(feature = "json")]
pub const MAX_EXTENSION_FIELDS: usize = 128;

/// Hard upper bound on the byte length of a single extension field key.
///
/// Without this bound an adversary could craft a payload with `MAX_EXTENSION_FIELDS`
/// keys each approaching 1 MB in length, consuming ~128 MB before the count cap fires.
/// Keys longer than this limit are rejected immediately during deserialization.
#[cfg(feature = "json")]
pub const MAX_EXTENSION_KEY_LEN: usize = 256;

/// Lazily-allocated extension-data map with a hard deserialization count limit.
///
/// - **`None` state** — no unknown fields present; zero heap allocation (8 bytes).
/// - **`Some` state** — up to [`MAX_EXTENSION_FIELDS`] unknown fields stored in a
///   heap-allocated [`indexmap::IndexMap`].
///
/// All generated BO/COM structs carry this type as their `_additional` field
/// (gated on the `json` feature).  The field is serialized / deserialized via
/// `#[serde(flatten)]` so unknown keys are transparently round-tripped.
///
/// ## DoS protection
///
/// `LimitedExtensionMap`'s `Deserialize` impl aborts as soon as the entry count
/// reaches [`MAX_EXTENSION_FIELDS`], preventing an adversary from forcing
/// unbounded memory growth via a crafted JSON payload.
#[cfg(feature = "json")]
#[derive(Debug, Clone, Default)]
pub struct LimitedExtensionMap(
    // The inner option is private to this module: every path in or out goes
    // through `try_insert`, the read accessors, or `Serialize`/`Deserialize`, so
    // the two caps hold for programmatic writes as well as for parsing.
    Option<Box<indexmap::IndexMap<String, serde_json::Value>>>,
);

#[cfg(feature = "json")]
impl LimitedExtensionMap {
    /// Returns a reference to the inner map, or `None` if empty.
    ///
    /// Consumed by the generated `Bo4eExtensionData` impls, so it has no caller
    /// unless `versioned` is on (or we are compiling this module's own tests).
    #[cfg(any(feature = "versioned", test))]
    #[inline]
    pub(crate) fn as_map(&self) -> Option<&indexmap::IndexMap<String, serde_json::Value>> {
        self.0.as_deref()
    }

    /// Inserts an extension field, enforcing [`MAX_EXTENSION_FIELDS`] and
    /// [`MAX_EXTENSION_KEY_LEN`].
    ///
    /// Returns the value this key previously held, the way
    /// [`HashMap::insert`](std::collections::HashMap::insert) does, or an
    /// [`ExtensionInsertError`] naming the cap that stopped it — in which case
    /// the map is unchanged.
    ///
    /// **Replacing** an existing key succeeds even at capacity: it does not grow
    /// the map, and refusing it would make a full map's contents unwritable.
    ///
    /// ```
    /// # #[cfg(feature = "json")] {
    /// use rubo4e::json::LimitedExtensionMap;
    /// use serde_json::json;
    ///
    /// let mut ext = LimitedExtensionMap::default();
    /// assert_eq!(ext.try_insert("meineId".into(), json!("A-1")), Ok(None));
    /// assert_eq!(
    ///     ext.try_insert("meineId".into(), json!("A-2")),
    ///     Ok(Some(json!("A-1"))),   // the displaced value comes back
    /// );
    /// assert_eq!(ext.len(), 1);
    /// # }
    /// ```
    ///
    /// # Errors
    /// [`ExtensionInsertError::KeyTooLong`] or [`ExtensionInsertError::Full`].
    #[inline]
    pub fn try_insert(
        &mut self,
        key: String,
        value: serde_json::Value,
    ) -> Result<Option<serde_json::Value>, ExtensionInsertError> {
        if key.len() > MAX_EXTENSION_KEY_LEN {
            return Err(ExtensionInsertError::KeyTooLong { len: key.len() });
        }
        // Check capacity before allocating, so a rejected insert cannot leave an
        // empty map behind where there was none.
        if let Some(map) = self.0.as_deref() {
            if map.len() >= MAX_EXTENSION_FIELDS && !map.contains_key(&key) {
                return Err(ExtensionInsertError::Full);
            }
        }
        let map = self
            .0
            .get_or_insert_with(|| Box::new(indexmap::IndexMap::new()));
        Ok(map.insert(key, value))
    }

    /// Returns the value stored under `key`, if any.
    #[inline]
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&serde_json::Value> {
        self.0.as_deref()?.get(key)
    }

    /// Returns the number of extension fields present.
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.as_deref().map_or(0, indexmap::IndexMap::len)
    }

    /// Returns `true` when no extension fields are present.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.as_ref().is_none_or(|m| m.is_empty())
    }

    /// Iterates the extension fields in the order they arrived.
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = (&String, &serde_json::Value)> {
        self.0.as_deref().into_iter().flatten()
    }
}

/// Why [`LimitedExtensionMap::try_insert`] refused an entry.
#[cfg(feature = "json")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum ExtensionInsertError {
    /// The key exceeds [`MAX_EXTENSION_KEY_LEN`].
    #[error(
        "extension key is {len} bytes, over the {} byte limit",
        MAX_EXTENSION_KEY_LEN
    )]
    KeyTooLong {
        /// Length of the offending key, in bytes.
        len: usize,
    },
    /// The map already holds [`MAX_EXTENSION_FIELDS`] entries and `key` is not
    /// one of them.
    #[error(
        "extension map already holds the maximum of {} fields",
        MAX_EXTENSION_FIELDS
    )]
    Full,
}

/// Equality ignores whether the inner map has been allocated: a map with no
/// entries equals one that never allocated. Nothing in this module produces the
/// allocated-but-empty state today — `Deserialize` returns `None` for an empty
/// map, and `try_insert` only allocates when it is about to succeed — and this
/// impl is what keeps that an implementation detail rather than an invariant a
/// future edit can silently break.
#[cfg(feature = "json")]
impl PartialEq for LimitedExtensionMap {
    fn eq(&self, other: &Self) -> bool {
        match (self.0.as_deref(), other.0.as_deref()) {
            (Some(a), Some(b)) => a == b,
            (Some(m), None) | (None, Some(m)) => m.is_empty(),
            (None, None) => true,
        }
    }
}

#[cfg(feature = "json")]
impl serde::Serialize for LimitedExtensionMap {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap as _;
        match &self.0 {
            None => serializer.serialize_map(Some(0))?.end(),
            Some(map) => map.serialize(serializer),
        }
    }
}

#[cfg(feature = "json")]
impl<'de> serde::Deserialize<'de> for LimitedExtensionMap {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct LimitedVisitor;

        impl<'de> serde::de::Visitor<'de> for LimitedVisitor {
            type Value = LimitedExtensionMap;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    "a map with at most {MAX_EXTENSION_FIELDS} extension entries"
                )
            }

            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                mut access: A,
            ) -> Result<Self::Value, A::Error> {
                let hint = access.size_hint().unwrap_or(0).min(MAX_EXTENSION_FIELDS);
                let mut map = indexmap::IndexMap::with_capacity(hint);

                // Per-struct field cap: the process-wide hard cap, tightened by the
                // per-call budget when a hardened entry point installed one.  Applies
                // at *every* nesting level, because this visitor runs for every
                // struct's `_additional` field.
                let field_cap = budget_max_fields_per_struct()
                    .map_or(MAX_EXTENSION_FIELDS, |b| b.min(MAX_EXTENSION_FIELDS));

                while let Some(key) = access.next_key::<String>()? {
                    // Reject oversized keys before they enter the IndexMap to prevent
                    // memory exhaustion from adversarial payloads with huge key strings.
                    if key.len() > MAX_EXTENSION_KEY_LEN {
                        trace_limit_violation(
                            LimitKind::ExtensionKeyLen,
                            key.len(),
                            MAX_EXTENSION_KEY_LEN,
                        );
                        return Err(serde::de::Error::custom(format!(
                            "extension field key too long: {} bytes exceeds limit {MAX_EXTENSION_KEY_LEN}",
                            key.len()
                        )));
                    }
                    if map.len() >= field_cap {
                        trace_limit_violation(
                            LimitKind::ExtensionFieldCount,
                            map.len() + 1,
                            field_cap,
                        );
                        return Err(serde::de::Error::custom(format!(
                            "extension field count exceeds the limit of {field_cap} \
                             — rejecting payload to prevent unbounded memory growth"
                        )));
                    }
                    let value = access.next_value::<serde_json::Value>()?;

                    // Charge the cumulative value-byte budget as we go, so an
                    // oversized payload is rejected mid-parse rather than after the
                    // whole tree has been built.
                    let cost = key.len() + estimated_json_value_bytes(&value);
                    if let Err((requested, remaining)) = charge_extension_bytes(cost) {
                        trace_limit_violation(LimitKind::ExtensionValueBytes, requested, remaining);
                        return Err(serde::de::Error::custom(format!(
                            "extension value budget exceeded: field {key:?} needs {requested} \
                             bytes but only {remaining} remain in this call's allowance"
                        )));
                    }
                    map.insert(key, value);
                }
                Ok(LimitedExtensionMap(if map.is_empty() {
                    None
                } else {
                    Some(Box::new(map))
                }))
            }
        }

        deserializer.deserialize_map(LimitedVisitor)
    }
}

/// Read access to unknown JSON fields captured during deserialization.
///
/// All generated BO and COM struct types implement this trait when the `json`
/// feature is active.  Extension fields are stored in a [`LimitedExtensionMap`]
/// that enforces [`MAX_EXTENSION_FIELDS`] at deserialization time and allocates
/// lazily: when no unknown fields arrive during deserialization the inner map
/// is `None` and no heap allocation is made.
///
/// ## Mutation
///
/// Write through [`LimitedExtensionMap::try_insert`] on the struct's
/// `_additional` field, which enforces [`MAX_EXTENSION_FIELDS`] and
/// [`MAX_EXTENSION_KEY_LEN`] and returns an [`ExtensionInsertError`] rather than
/// growing past either.  A `&mut IndexMap` is deliberately not exposed anywhere:
/// handing one out would make both caps advisory.
///
/// This trait is **sealed**: only types in this crate may implement it.
/// Downstream code may call the provided methods but cannot add new implementors.
#[cfg(feature = "json")]
pub trait Bo4eExtensionData: sealed::Sealed {
    /// Returns the unknown JSON fields captured during deserialization,
    /// or an empty map if none were present.
    fn extension_data(&self) -> &indexmap::IndexMap<String, serde_json::Value>;

    /// Returns `true` if any unknown extension fields were captured.
    fn has_extension_data(&self) -> bool;
}

/// A single shared empty-map sentinel used by all generated `Bo4eExtensionData` impls.
///
/// Sharing one `LazyLock` across all ~200 generated struct types avoids allocating
/// a separate `static` per struct.  The contained `IndexMap` is never
/// mutated; `extension_data()` returns a reference to it only when the struct's
/// `_additional` field is `None`.
// Referenced only from generated `Bo4eExtensionData` impls.
#[cfg(all(feature = "json", feature = "versioned"))]
pub(crate) static EMPTY_EXTENSION_MAP: std::sync::LazyLock<
    indexmap::IndexMap<String, serde_json::Value>,
> = std::sync::LazyLock::new(indexmap::IndexMap::new);

/// `schemars::JsonSchema` impl for [`LimitedExtensionMap`].
///
/// Delegates schema generation to `IndexMap<String, serde_json::Value>`, which
/// produces `{"type":"object","additionalProperties":true}` — the correct
/// schema for an opaque extension-field bag.
#[cfg(all(feature = "json", feature = "schemars"))]
impl schemars::JsonSchema for LimitedExtensionMap {
    fn inline_schema() -> bool {
        true
    }

    fn schema_name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("LimitedExtensionMap")
    }

    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        <indexmap::IndexMap<String, serde_json::Value>>::json_schema(generator)
    }
}

/// `utoipa::ToSchema` impl for [`LimitedExtensionMap`].
///
/// Represents the extension-field bag as an `object` with free additional
/// properties in the OpenAPI schema, mirroring the schemars implementation.
#[cfg(all(feature = "json", feature = "utoipa"))]
impl utoipa::ToSchema for LimitedExtensionMap {
    fn name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("LimitedExtensionMap")
    }
}

#[cfg(all(feature = "json", feature = "utoipa"))]
impl utoipa::PartialSchema for LimitedExtensionMap {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        utoipa::openapi::ObjectBuilder::new()
            .additional_properties(Some(
                utoipa::openapi::schema::AdditionalProperties::FreeForm(true),
            ))
            .into()
    }
}

/// `skip_serializing_if` helper for the [`LimitedExtensionMap`] extension field.
///
/// Returns `true` (skip) when the map contains no entries.
/// Used in the `#[serde(skip_serializing_if = …)]` attribute on `_additional`.
#[cfg(feature = "json")]
#[doc(hidden)]
#[inline]
pub fn ext_map_is_empty(m: &LimitedExtensionMap) -> bool {
    m.is_empty()
}

#[cfg(all(test, feature = "json"))]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn a_fresh_map_is_empty_and_allocates_nothing() {
        let ext = LimitedExtensionMap::default();
        assert!(ext.is_empty());
        assert_eq!(ext.len(), 0);
        assert_eq!(ext.get("anything"), None);
        assert_eq!(ext.iter().count(), 0);
        assert!(ext.as_map().is_none(), "an empty map must not allocate");
    }

    #[test]
    fn insert_returns_the_displaced_value_and_keeps_arrival_order() {
        let mut ext = LimitedExtensionMap::default();
        assert_eq!(ext.try_insert("b".into(), json!(1)), Ok(None));
        assert_eq!(ext.try_insert("a".into(), json!(2)), Ok(None));
        assert_eq!(ext.try_insert("b".into(), json!(3)), Ok(Some(json!(1))));

        assert_eq!(ext.len(), 2, "replacing must not grow the map");
        assert_eq!(ext.get("b"), Some(&json!(3)));
        assert_eq!(
            ext.iter().map(|(k, _)| k.as_str()).collect::<Vec<_>>(),
            ["b", "a"],
            "iteration follows insertion order, not sort order"
        );
    }

    #[test]
    fn an_oversized_key_is_refused_without_allocating() {
        let mut ext = LimitedExtensionMap::default();
        let key = "k".repeat(MAX_EXTENSION_KEY_LEN + 1);
        assert_eq!(
            ext.try_insert(key.clone(), json!(1)),
            Err(ExtensionInsertError::KeyTooLong { len: key.len() })
        );
        assert!(ext.is_empty());
        assert!(
            ext.as_map().is_none(),
            "a refused insert must not leave an allocated empty map behind"
        );
    }

    /// A key exactly at the limit is accepted; the cap is inclusive.
    #[test]
    fn a_key_at_the_limit_is_accepted() {
        let mut ext = LimitedExtensionMap::default();
        let key = "k".repeat(MAX_EXTENSION_KEY_LEN);
        assert_eq!(ext.try_insert(key, json!(1)), Ok(None));
        assert_eq!(ext.len(), 1);
    }

    /// At capacity, a *new* key is refused but an existing one can still be
    /// rewritten — otherwise a full map's contents would be frozen.
    #[test]
    fn a_full_map_refuses_new_keys_but_still_accepts_replacements() {
        let mut ext = LimitedExtensionMap::default();
        for i in 0..MAX_EXTENSION_FIELDS {
            assert_eq!(ext.try_insert(format!("k{i}"), json!(i)), Ok(None));
        }
        assert_eq!(ext.len(), MAX_EXTENSION_FIELDS);

        assert_eq!(
            ext.try_insert("one_too_many".into(), json!(0)),
            Err(ExtensionInsertError::Full)
        );
        assert_eq!(ext.len(), MAX_EXTENSION_FIELDS);

        assert_eq!(
            ext.try_insert("k0".into(), json!("new")),
            Ok(Some(json!(0)))
        );
        assert_eq!(ext.get("k0"), Some(&json!("new")));
    }

    /// Equality is by contents, so the unallocated and allocated-empty states
    /// cannot be told apart by `==`.
    #[test]
    fn emptiness_compares_equal_however_it_arose() {
        let unallocated = LimitedExtensionMap::default();

        let mut allocated = LimitedExtensionMap::default();
        assert_eq!(allocated.try_insert("x".into(), json!(1)), Ok(None));
        assert_ne!(allocated, unallocated);

        // Deserializing an empty object yields the unallocated state.
        let from_empty: LimitedExtensionMap = serde_json::from_str("{}").expect("valid");
        assert_eq!(from_empty, unallocated);
        assert!(from_empty.as_map().is_none());
    }

    #[test]
    fn deserialization_rejects_more_fields_than_the_hard_cap() {
        let body: String = (0..=MAX_EXTENSION_FIELDS)
            .map(|i| format!(r#""k{i}":{i}"#))
            .collect::<Vec<_>>()
            .join(",");
        let err = serde_json::from_str::<LimitedExtensionMap>(&format!("{{{body}}}"))
            .expect_err("over the hard cap");
        assert!(
            err.to_string().contains("extension field count"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn deserialization_rejects_an_oversized_key() {
        let key = "k".repeat(MAX_EXTENSION_KEY_LEN + 1);
        let err = serde_json::from_str::<LimitedExtensionMap>(&format!(r#"{{"{key}":1}}"#))
            .expect_err("over the key-length cap");
        assert!(
            err.to_string().contains("key too long"),
            "unexpected error: {err}"
        );
    }
}
