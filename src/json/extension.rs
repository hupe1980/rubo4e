//! Extension-data map with a hard deserialization count limit, and the
//! [`Bo4eExtensionData`] accessor trait for generated BO/COM structs.
//!
//! Also houses limit-enforcement helpers that depend on [`Bo4eExtensionData`]:
//! [`check_extension_budget`] and the private `estimated_json_value_bytes`.

use serde::de::Error as _;
use serde_json::Value;

use super::limits::{trace_limit_violation, JsonParseLimits};
use super::sealed;

fn estimated_json_value_bytes(value: &Value) -> usize {
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

pub(super) fn check_extension_budget<T>(
    value: &T,
    limits: JsonParseLimits,
) -> Result<(), serde_json::Error>
where
    T: Bo4eExtensionData,
{
    let data = value.extension_data();
    if let Some(max) = limits.max_extension_field_count {
        let count = data.len();
        if count > max {
            trace_limit_violation("extension_field_count", count, max);
            return Err(serde_json::Error::custom(format!(
                "extension field count {count} exceeds per-call limit {max}"
            )));
        }
    }
    if let Some(max) = limits.max_extension_value_bytes {
        let used: usize = data
            .iter()
            .map(|(k, v)| k.len() + estimated_json_value_bytes(v))
            .sum();
        if used > max {
            trace_limit_violation("extension_value_bytes", used, max);
            return Err(serde_json::Error::custom(format!(
                "extension value budget exceeded: estimated {used} bytes exceeds limit {max}"
            )));
        }
    }
    Ok(())
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
#[derive(Debug, Clone, Default, PartialEq)]
pub struct LimitedExtensionMap(
    // The inner option is private to this module; all external access goes through
    // the `as_map()`, `try_insert()`, `is_empty()`, and `Serialize/Deserialize` impls.
    Option<Box<indexmap::IndexMap<String, serde_json::Value>>>,
);

#[cfg(feature = "json")]
impl LimitedExtensionMap {
    /// Returns a reference to the inner map, or `None` if empty.
    #[inline]
    pub(crate) fn as_map(&self) -> Option<&indexmap::IndexMap<String, serde_json::Value>> {
        self.0.as_deref()
    }

    /// Inserts an extension field, enforcing [`MAX_EXTENSION_FIELDS`] and
    /// [`MAX_EXTENSION_KEY_LEN`].
    ///
    /// Returns `true` if the entry was inserted, `false` if either cap would
    /// be exceeded (the map is left unchanged in that case).
    #[inline]
    pub fn try_insert(&mut self, key: String, value: serde_json::Value) -> bool {
        if key.len() > MAX_EXTENSION_KEY_LEN {
            return false;
        }
        let map = self
            .0
            .get_or_insert_with(|| Box::new(indexmap::IndexMap::new()));
        if map.len() >= MAX_EXTENSION_FIELDS {
            return false;
        }
        map.insert(key, value);
        true
    }

    /// Returns `true` when no extension fields are present.
    #[inline]
    pub(crate) fn is_empty(&self) -> bool {
        self.0.as_ref().is_none_or(|m| m.is_empty())
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
                while let Some(key) = access.next_key::<String>()? {
                    // Reject oversized keys before they enter the IndexMap to prevent
                    // memory exhaustion from adversarial payloads with huge key strings.
                    if key.len() > MAX_EXTENSION_KEY_LEN {
                        trace_limit_violation(
                            "extension_key_len",
                            key.len(),
                            MAX_EXTENSION_KEY_LEN,
                        );
                        return Err(serde::de::Error::custom(format!(
                            "extension field key too long: {} bytes exceeds limit {MAX_EXTENSION_KEY_LEN}",
                            key.len()
                        )));
                    }
                    if map.len() >= MAX_EXTENSION_FIELDS {
                        trace_limit_violation(
                            "extension_field_count",
                            map.len() + 1,
                            MAX_EXTENSION_FIELDS,
                        );
                        return Err(serde::de::Error::custom(format!(
                            "extension field count exceeds the limit of {MAX_EXTENSION_FIELDS} \
                             — rejecting payload to prevent unbounded memory growth"
                        )));
                    }
                    let value = access.next_value::<serde_json::Value>()?;
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
/// To insert extension fields programmatically use
/// [`LimitedExtensionMap::try_insert`] on the `_additional` field of the struct,
/// which enforces the [`MAX_EXTENSION_FIELDS`] cap.  Direct `&mut IndexMap`
/// access is intentionally not exposed through this trait to prevent bypassing
/// the cap.
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
/// a separate `static` per struct (I-5 fix).  The contained `IndexMap` is never
/// mutated; `extension_data()` returns a reference to it only when the struct's
/// `_additional` field is `None`.
#[cfg(feature = "json")]
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
