//! Single-pass JSON nesting-depth enforcement via a [`Deserializer`] wrapper.
//!
//! Depth is tracked inline during a single deserialization traversal by
//! intercepting `visit_map` / `visit_seq` in a visitor wrapper.  No second
//! parse of the input is ever performed.
//!
//! All wrapper types borrow `&'state DepthState` which is stack-allocated,
//! so no heap allocation is needed for tracking state.

use super::limits::trace_limit_violation;

use std::cell::Cell;

/// Shared nesting-depth state, stack-allocated by the caller.
pub(super) struct DepthState {
    current: Cell<usize>,
    max: usize,
}

impl DepthState {
    pub(super) fn new(max: usize) -> Self {
        Self {
            current: Cell::new(0),
            max,
        }
    }

    /// Increments the depth counter; returns a serde error (and fires the
    /// violation counter / trace event) if the new depth would exceed the limit.
    #[inline]
    fn try_enter<E: serde::de::Error>(&self) -> Result<(), E> {
        let next = self.current.get() + 1;
        if next > self.max {
            trace_limit_violation("nesting_depth", next, self.max);
            return Err(E::custom(format!(
                "JSON nesting depth {next} exceeds limit {}",
                self.max
            )));
        }
        self.current.set(next);
        Ok(())
    }

    /// Decrements the depth counter; called via [`DepthGuard`] Drop.
    #[inline]
    fn exit(&self) {
        let d = self.current.get();
        debug_assert!(d > 0, "DepthState::exit called with depth=0");
        if d > 0 {
            self.current.set(d - 1);
        }
    }
}

/// RAII guard: decrements the depth counter when dropped (after map/seq is consumed).
struct DepthGuard<'state>(&'state DepthState);

impl Drop for DepthGuard<'_> {
    fn drop(&mut self) {
        self.0.exit();
    }
}

/// Deserializer wrapper that enforces a maximum nesting depth by wrapping every
/// visitor with [`DepthLimitedVisitor`].
pub(super) struct DepthLimitedDeserializer<'state, D> {
    inner: D,
    state: &'state DepthState,
}

impl<'state, D> DepthLimitedDeserializer<'state, D> {
    pub(super) fn new(inner: D, state: &'state DepthState) -> Self {
        Self { inner, state }
    }
}

/// Visitor wrapper that intercepts `visit_map` / `visit_seq` to track depth.
/// All scalar `visit_*` methods delegate directly to the inner visitor.
struct DepthLimitedVisitor<'state, V> {
    inner: V,
    state: &'state DepthState,
}

/// `DeserializeSeed` wrapper that re-wraps the incoming deserializer with
/// [`DepthLimitedDeserializer`] so nested structures are also tracked.
struct DepthLimitedSeed<'state, S> {
    inner: S,
    state: &'state DepthState,
}

impl<'de, S> serde::de::DeserializeSeed<'de> for DepthLimitedSeed<'_, S>
where
    S: serde::de::DeserializeSeed<'de>,
{
    type Value = S::Value;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.inner
            .deserialize(DepthLimitedDeserializer::new(deserializer, self.state))
    }
}

/// `MapAccess` wrapper: re-wraps value deserializers so nested maps/sequences
/// inside values are also tracked.  The `DepthGuard` decrements depth on drop.
struct DepthLimitedMap<'state, A> {
    inner: A,
    state: &'state DepthState,
    _guard: DepthGuard<'state>,
}

impl<'de, A> serde::de::MapAccess<'de> for DepthLimitedMap<'_, A>
where
    A: serde::de::MapAccess<'de>,
{
    type Error = A::Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        // JSON map keys are always primitive strings; no depth tracking needed.
        self.inner.next_key_seed(seed)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        self.inner.next_value_seed(DepthLimitedSeed {
            inner: seed,
            state: self.state,
        })
    }

    fn size_hint(&self) -> Option<usize> {
        self.inner.size_hint()
    }
}

/// `SeqAccess` wrapper: re-wraps element deserializers.  The `DepthGuard`
/// decrements depth on drop.
struct DepthLimitedSeq<'state, A> {
    inner: A,
    state: &'state DepthState,
    _guard: DepthGuard<'state>,
}

impl<'de, A> serde::de::SeqAccess<'de> for DepthLimitedSeq<'_, A>
where
    A: serde::de::SeqAccess<'de>,
{
    type Error = A::Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        self.inner.next_element_seed(DepthLimitedSeed {
            inner: seed,
            state: self.state,
        })
    }

    fn size_hint(&self) -> Option<usize> {
        self.inner.size_hint()
    }
}

impl<'de, V> serde::de::Visitor<'de> for DepthLimitedVisitor<'_, V>
where
    V: serde::de::Visitor<'de>,
{
    type Value = V::Value;

    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.expecting(f)
    }

    fn visit_map<A>(self, map: A) -> Result<V::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        self.state.try_enter()?;
        self.inner.visit_map(DepthLimitedMap {
            inner: map,
            state: self.state,
            _guard: DepthGuard(self.state),
        })
    }

    fn visit_seq<A>(self, seq: A) -> Result<V::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        self.state.try_enter()?;
        self.inner.visit_seq(DepthLimitedSeq {
            inner: seq,
            state: self.state,
            _guard: DepthGuard(self.state),
        })
    }

    fn visit_bool<E: serde::de::Error>(self, v: bool) -> Result<V::Value, E> {
        self.inner.visit_bool(v)
    }
    fn visit_i8<E: serde::de::Error>(self, v: i8) -> Result<V::Value, E> {
        self.inner.visit_i8(v)
    }
    fn visit_i16<E: serde::de::Error>(self, v: i16) -> Result<V::Value, E> {
        self.inner.visit_i16(v)
    }
    fn visit_i32<E: serde::de::Error>(self, v: i32) -> Result<V::Value, E> {
        self.inner.visit_i32(v)
    }
    fn visit_i64<E: serde::de::Error>(self, v: i64) -> Result<V::Value, E> {
        self.inner.visit_i64(v)
    }
    fn visit_i128<E: serde::de::Error>(self, v: i128) -> Result<V::Value, E> {
        self.inner.visit_i128(v)
    }
    fn visit_u8<E: serde::de::Error>(self, v: u8) -> Result<V::Value, E> {
        self.inner.visit_u8(v)
    }
    fn visit_u16<E: serde::de::Error>(self, v: u16) -> Result<V::Value, E> {
        self.inner.visit_u16(v)
    }
    fn visit_u32<E: serde::de::Error>(self, v: u32) -> Result<V::Value, E> {
        self.inner.visit_u32(v)
    }
    fn visit_u64<E: serde::de::Error>(self, v: u64) -> Result<V::Value, E> {
        self.inner.visit_u64(v)
    }
    fn visit_u128<E: serde::de::Error>(self, v: u128) -> Result<V::Value, E> {
        self.inner.visit_u128(v)
    }
    fn visit_f32<E: serde::de::Error>(self, v: f32) -> Result<V::Value, E> {
        self.inner.visit_f32(v)
    }
    fn visit_f64<E: serde::de::Error>(self, v: f64) -> Result<V::Value, E> {
        self.inner.visit_f64(v)
    }
    fn visit_char<E: serde::de::Error>(self, v: char) -> Result<V::Value, E> {
        self.inner.visit_char(v)
    }
    fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<V::Value, E> {
        self.inner.visit_str(v)
    }
    fn visit_borrowed_str<E: serde::de::Error>(self, v: &'de str) -> Result<V::Value, E> {
        self.inner.visit_borrowed_str(v)
    }
    fn visit_string<E: serde::de::Error>(self, v: String) -> Result<V::Value, E> {
        self.inner.visit_string(v)
    }
    fn visit_bytes<E: serde::de::Error>(self, v: &[u8]) -> Result<V::Value, E> {
        self.inner.visit_bytes(v)
    }
    fn visit_borrowed_bytes<E: serde::de::Error>(self, v: &'de [u8]) -> Result<V::Value, E> {
        self.inner.visit_borrowed_bytes(v)
    }
    fn visit_byte_buf<E: serde::de::Error>(self, v: Vec<u8>) -> Result<V::Value, E> {
        self.inner.visit_byte_buf(v)
    }
    fn visit_none<E: serde::de::Error>(self) -> Result<V::Value, E> {
        self.inner.visit_none()
    }
    fn visit_unit<E: serde::de::Error>(self) -> Result<V::Value, E> {
        self.inner.visit_unit()
    }

    fn visit_some<D>(self, deserializer: D) -> Result<V::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.inner
            .visit_some(DepthLimitedDeserializer::new(deserializer, self.state))
    }

    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<V::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.inner
            .visit_newtype_struct(DepthLimitedDeserializer::new(deserializer, self.state))
    }

    fn visit_enum<A>(self, data: A) -> Result<V::Value, A::Error>
    where
        A: serde::de::EnumAccess<'de>,
    {
        // Enum variants don't add a structural nesting level.
        self.inner.visit_enum(data)
    }
}

impl<'de, D> serde::de::Deserializer<'de> for DepthLimitedDeserializer<'_, D>
where
    D: serde::de::Deserializer<'de>,
{
    type Error = D::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.inner.deserialize_any(DepthLimitedVisitor {
            inner: visitor,
            state: self.state,
        })
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.inner.deserialize_bool(DepthLimitedVisitor {
            inner: visitor,
            state: self.state,
        })
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.inner.deserialize_i8(DepthLimitedVisitor {
            inner: visitor,
            state: self.state,
        })
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.inner.deserialize_i16(DepthLimitedVisitor {
            inner: visitor,
            state: self.state,
        })
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.inner.deserialize_i32(DepthLimitedVisitor {
            inner: visitor,
            state: self.state,
        })
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.inner.deserialize_i64(DepthLimitedVisitor {
            inner: visitor,
            state: self.state,
        })
    }

    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.inner.deserialize_i128(DepthLimitedVisitor {
            inner: visitor,
            state: self.state,
        })
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.inner.deserialize_u8(DepthLimitedVisitor {
            inner: visitor,
            state: self.state,
        })
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.inner.deserialize_u16(DepthLimitedVisitor {
            inner: visitor,
            state: self.state,
        })
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.inner.deserialize_u32(DepthLimitedVisitor {
            inner: visitor,
            state: self.state,
        })
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.inner.deserialize_u64(DepthLimitedVisitor {
            inner: visitor,
            state: self.state,
        })
    }

    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.inner.deserialize_u128(DepthLimitedVisitor {
            inner: visitor,
            state: self.state,
        })
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.inner.deserialize_f32(DepthLimitedVisitor {
            inner: visitor,
            state: self.state,
        })
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.inner.deserialize_f64(DepthLimitedVisitor {
            inner: visitor,
            state: self.state,
        })
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.inner.deserialize_char(DepthLimitedVisitor {
            inner: visitor,
            state: self.state,
        })
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.inner.deserialize_str(DepthLimitedVisitor {
            inner: visitor,
            state: self.state,
        })
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.inner.deserialize_string(DepthLimitedVisitor {
            inner: visitor,
            state: self.state,
        })
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.inner.deserialize_bytes(DepthLimitedVisitor {
            inner: visitor,
            state: self.state,
        })
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.inner.deserialize_byte_buf(DepthLimitedVisitor {
            inner: visitor,
            state: self.state,
        })
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.inner.deserialize_option(DepthLimitedVisitor {
            inner: visitor,
            state: self.state,
        })
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.inner.deserialize_unit(DepthLimitedVisitor {
            inner: visitor,
            state: self.state,
        })
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.inner.deserialize_unit_struct(
            name,
            DepthLimitedVisitor {
                inner: visitor,
                state: self.state,
            },
        )
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.inner.deserialize_newtype_struct(
            name,
            DepthLimitedVisitor {
                inner: visitor,
                state: self.state,
            },
        )
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.inner.deserialize_seq(DepthLimitedVisitor {
            inner: visitor,
            state: self.state,
        })
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.inner.deserialize_tuple(
            len,
            DepthLimitedVisitor {
                inner: visitor,
                state: self.state,
            },
        )
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.inner.deserialize_tuple_struct(
            name,
            len,
            DepthLimitedVisitor {
                inner: visitor,
                state: self.state,
            },
        )
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.inner.deserialize_map(DepthLimitedVisitor {
            inner: visitor,
            state: self.state,
        })
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.inner.deserialize_struct(
            name,
            fields,
            DepthLimitedVisitor {
                inner: visitor,
                state: self.state,
            },
        )
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.inner.deserialize_enum(
            name,
            variants,
            DepthLimitedVisitor {
                inner: visitor,
                state: self.state,
            },
        )
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.inner.deserialize_identifier(DepthLimitedVisitor {
            inner: visitor,
            state: self.state,
        })
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.inner.deserialize_ignored_any(DepthLimitedVisitor {
            inner: visitor,
            state: self.state,
        })
    }

    fn is_human_readable(&self) -> bool {
        self.inner.is_human_readable()
    }
}
