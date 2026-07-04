//! Key-name transformation between BO4E German camelCase and Rust snake_case,
//! with serializer and deserializer wrappers that apply the transformations
//! transparently in a single pass.

// `?Sized` on the generic parameter is required by serde's SerializeMap trait;
// the `where T: Serialize` clause is also required. Both are intentional.
#![allow(clippy::multiple_bound_locations)]

use std::borrow::Cow;

use serde::de::DeserializeOwned;
use serde::de::Error as _;
use serde::de::IntoDeserializer;
use serde::Serialize;

pub(super) type KeyTransformFn = for<'a> fn(&'a str) -> Cow<'a, str>;

pub(super) fn serialize_with_key_transform<T>(
    value: &T,
    transform: KeyTransformFn,
) -> Result<String, serde_json::Error>
where
    T: Serialize,
{
    let mut out = Vec::new();
    {
        let mut ser = serde_json::Serializer::new(&mut out);
        value
            .serialize(KeyTransformSerializer {
                inner: &mut ser,
                transform,
            })
            .map_err(serde_json::Error::custom)?;
    }
    String::from_utf8(out)
        .map_err(|e| serde_json::Error::custom(format!("invalid utf-8 emitted by serializer: {e}")))
}

struct KeyTransformSerializer<S> {
    inner: S,
    transform: KeyTransformFn,
}

struct KeyTransformValue<'a, T: ?Sized> {
    value: &'a T,
    transform: KeyTransformFn,
}

impl<T: ?Sized> Serialize for KeyTransformValue<'_, T>
where
    T: Serialize,
{
    fn serialize<Sz>(&self, serializer: Sz) -> Result<Sz::Ok, Sz::Error>
    where
        Sz: serde::ser::Serializer,
    {
        self.value.serialize(KeyTransformSerializer {
            inner: serializer,
            transform: self.transform,
        })
    }
}

/// Captures a map key that serializes as a string without allocating a
/// `serde_json::Value` intermediate.
///
/// M-C fix: replaces `serde_json::to_value(key)` (which allocates a
/// `Value::String`) with a minimal serializer that captures the string directly.
///
/// D-05 fix: non-string key branches are removed — BO4E JSON objects always
/// use string keys.  A `debug_assert` fires in debug builds if a non-string
/// key is ever encountered, making unexpected usage visible immediately.
fn json_key_to_string<K: ?Sized + Serialize>(key: &K) -> Result<String, serde_json::Error> {
    use serde::ser::Impossible;

    struct StringCapture;

    macro_rules! debug_assert_numeric {
        ($($method:ident($T:ty)),* $(,)?) => {
            $(fn $method(self, v: $T) -> Result<String, serde_json::Error> {
                debug_assert!(false, concat!("non-string key (", stringify!($T), ") in BO4E map serializer"));
                Ok(v.to_string())
            })*
        };
    }
    impl serde::Serializer for StringCapture {
        type Ok = String;
        type Error = serde_json::Error;
        type SerializeSeq = Impossible<String, serde_json::Error>;
        type SerializeTuple = Impossible<String, serde_json::Error>;
        type SerializeTupleStruct = Impossible<String, serde_json::Error>;
        type SerializeTupleVariant = Impossible<String, serde_json::Error>;
        type SerializeMap = Impossible<String, serde_json::Error>;
        type SerializeStruct = Impossible<String, serde_json::Error>;
        type SerializeStructVariant = Impossible<String, serde_json::Error>;

        // ── String paths (hot) ────────────────────────────────────────────────
        fn serialize_str(self, v: &str) -> Result<String, serde_json::Error> {
            Ok(v.to_owned())
        }
        fn collect_str<T: ?Sized + std::fmt::Display>(
            self,
            value: &T,
        ) -> Result<String, serde_json::Error> {
            Ok(value.to_string())
        }

        // ── Non-string paths (dead code for BO4E map keys) ───────────────────
        debug_assert_numeric! {
            serialize_bool(bool),
            serialize_i8(i8), serialize_i16(i16), serialize_i32(i32), serialize_i64(i64),
            serialize_u8(u8), serialize_u16(u16), serialize_u32(u32), serialize_u64(u64),
            serialize_f32(f32), serialize_f64(f64),
            serialize_char(char),
        }
        fn serialize_bytes(self, _v: &[u8]) -> Result<String, serde_json::Error> {
            debug_assert!(false, "non-string key (bytes) in BO4E map serializer");
            Err(serde_json::Error::custom("bytes cannot be a map key"))
        }
        fn serialize_none(self) -> Result<String, serde_json::Error> {
            debug_assert!(false, "non-string key (none) in BO4E map serializer");
            Err(serde_json::Error::custom("null cannot be a map key"))
        }
        fn serialize_some<T: ?Sized + Serialize>(self, v: &T) -> Result<String, serde_json::Error> {
            v.serialize(self)
        }
        fn serialize_unit(self) -> Result<String, serde_json::Error> {
            debug_assert!(false, "non-string key (unit) in BO4E map serializer");
            Err(serde_json::Error::custom("unit cannot be a map key"))
        }
        fn serialize_unit_struct(self, name: &'static str) -> Result<String, serde_json::Error> {
            Ok(name.to_owned())
        }
        fn serialize_unit_variant(
            self,
            _: &'static str,
            _: u32,
            v: &'static str,
        ) -> Result<String, serde_json::Error> {
            Ok(v.to_owned())
        }
        fn serialize_newtype_struct<T: ?Sized + Serialize>(
            self,
            _: &'static str,
            v: &T,
        ) -> Result<String, serde_json::Error> {
            v.serialize(self)
        }
        fn serialize_newtype_variant<T: ?Sized + Serialize>(
            self,
            _: &'static str,
            _: u32,
            _: &'static str,
            _: &T,
        ) -> Result<String, serde_json::Error> {
            Err(serde_json::Error::custom(
                "newtype variant cannot be a map key",
            ))
        }
        fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, serde_json::Error> {
            Err(serde_json::Error::custom("sequence cannot be a map key"))
        }
        fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, serde_json::Error> {
            Err(serde_json::Error::custom("tuple cannot be a map key"))
        }
        fn serialize_tuple_struct(
            self,
            _: &'static str,
            _: usize,
        ) -> Result<Self::SerializeTupleStruct, serde_json::Error> {
            Err(serde_json::Error::custom(
                "tuple struct cannot be a map key",
            ))
        }
        fn serialize_tuple_variant(
            self,
            _: &'static str,
            _: u32,
            _: &'static str,
            _: usize,
        ) -> Result<Self::SerializeTupleVariant, serde_json::Error> {
            Err(serde_json::Error::custom(
                "tuple variant cannot be a map key",
            ))
        }
        fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, serde_json::Error> {
            Err(serde_json::Error::custom("map cannot be a map key"))
        }
        fn serialize_struct(
            self,
            _: &'static str,
            _: usize,
        ) -> Result<Self::SerializeStruct, serde_json::Error> {
            Err(serde_json::Error::custom("struct cannot be a map key"))
        }
        fn serialize_struct_variant(
            self,
            _: &'static str,
            _: u32,
            _: &'static str,
            _: usize,
        ) -> Result<Self::SerializeStructVariant, serde_json::Error> {
            Err(serde_json::Error::custom(
                "struct variant cannot be a map key",
            ))
        }
    }

    key.serialize(StringCapture)
}

/// Generates a state struct + trait impl for sequence-like serialization types
/// whose only non-`end` method wraps each element/field value.
macro_rules! impl_sequence_state {
    ($(($State:ident, $Trait:ident, $method:ident)),* $(,)?) => {$(
        struct $State<S> { inner: S, transform: KeyTransformFn }
        impl<S: serde::ser::$Trait> serde::ser::$Trait for $State<S> {
            type Ok = S::Ok;
            type Error = S::Error;
            fn $method<T: ?Sized + Serialize>(
                &mut self,
                value: &T,
            ) -> Result<(), Self::Error> {
                self.inner.$method(&KeyTransformValue { value, transform: self.transform })
            }
            fn end(self) -> Result<Self::Ok, Self::Error> { self.inner.end() }
        }
    )*};
}

impl_sequence_state! {
    (KeyTransformSerializeSeq,          SerializeSeq,          serialize_element),
    (KeyTransformSerializeTuple,        SerializeTuple,        serialize_element),
    (KeyTransformSerializeTupleStruct,  SerializeTupleStruct,  serialize_field),
    (KeyTransformSerializeTupleVariant, SerializeTupleVariant, serialize_field),
}

struct KeyTransformSerializeMap<S> {
    inner: S,
    transform: KeyTransformFn,
}

impl<S> serde::ser::SerializeMap for KeyTransformSerializeMap<S>
where
    S: serde::ser::SerializeMap,
{
    type Ok = S::Ok;
    type Error = S::Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let key = json_key_to_string(key).map_err(<S::Error as serde::ser::Error>::custom)?;
        let key = (self.transform)(&key);
        self.inner.serialize_key(key.as_ref())
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.inner.serialize_value(&KeyTransformValue {
            value,
            transform: self.transform,
        })
    }

    fn serialize_entry<K: ?Sized, V: ?Sized>(
        &mut self,
        key: &K,
        value: &V,
    ) -> Result<(), Self::Error>
    where
        K: Serialize,
        V: Serialize,
    {
        let key = json_key_to_string(key).map_err(<S::Error as serde::ser::Error>::custom)?;
        let key = (self.transform)(&key);
        self.inner.serialize_entry(
            key.as_ref(),
            &KeyTransformValue {
                value,
                transform: self.transform,
            },
        )
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.inner.end()
    }
}

struct KeyTransformSerializeStruct<S> {
    inner: S,
    transform: KeyTransformFn,
}

impl<S> serde::ser::SerializeStruct for KeyTransformSerializeStruct<S>
where
    S: serde::ser::SerializeMap,
{
    type Ok = S::Ok;
    type Error = S::Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let key = (self.transform)(key);
        self.inner.serialize_entry(
            key.as_ref(),
            &KeyTransformValue {
                value,
                transform: self.transform,
            },
        )
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.inner.end()
    }
}

struct KeyTransformSerializeStructVariant<S> {
    inner: S,
    transform: KeyTransformFn,
}

impl<S> serde::ser::SerializeStructVariant for KeyTransformSerializeStructVariant<S>
where
    S: serde::ser::SerializeStructVariant,
{
    type Ok = S::Ok;
    type Error = S::Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.inner.serialize_field(
            key,
            &KeyTransformValue {
                value,
                transform: self.transform,
            },
        )
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.inner.end()
    }
}

macro_rules! delegate_ser_scalar {
    ($($method:ident($T:ty)),* $(,)?) => {
        $(fn $method(self, v: $T) -> Result<Self::Ok, Self::Error> {
            self.inner.$method(v)
        })*
    };
}

impl<S> serde::ser::Serializer for KeyTransformSerializer<S>
where
    S: serde::ser::Serializer,
{
    type Ok = S::Ok;
    type Error = S::Error;
    type SerializeSeq = KeyTransformSerializeSeq<S::SerializeSeq>;
    type SerializeTuple = KeyTransformSerializeTuple<S::SerializeTuple>;
    type SerializeTupleStruct = KeyTransformSerializeTupleStruct<S::SerializeTupleStruct>;
    type SerializeTupleVariant = KeyTransformSerializeTupleVariant<S::SerializeTupleVariant>;
    type SerializeMap = KeyTransformSerializeMap<S::SerializeMap>;
    type SerializeStruct = KeyTransformSerializeStruct<S::SerializeMap>;
    type SerializeStructVariant = KeyTransformSerializeStructVariant<S::SerializeStructVariant>;

    // ── Scalars pass through unchanged ────────────────────────────────────────
    delegate_ser_scalar! {
        serialize_bool(bool),
        serialize_i8(i8), serialize_i16(i16), serialize_i32(i32), serialize_i64(i64),
        serialize_u8(u8), serialize_u16(u16), serialize_u32(u32), serialize_u64(u64),
        serialize_f32(f32), serialize_f64(f64),
        serialize_char(char), serialize_str(&str), serialize_bytes(&[u8]),
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_none()
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        self.inner.serialize_some(&KeyTransformValue {
            value,
            transform: self.transform,
        })
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_unit()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_unit_struct(name)
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.inner
            .serialize_unit_variant(name, variant_index, variant)
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        self.inner.serialize_newtype_struct(
            name,
            &KeyTransformValue {
                value,
                transform: self.transform,
            },
        )
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        self.inner.serialize_newtype_variant(
            name,
            variant_index,
            variant,
            &KeyTransformValue {
                value,
                transform: self.transform,
            },
        )
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(KeyTransformSerializeSeq {
            inner: self.inner.serialize_seq(len)?,
            transform: self.transform,
        })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(KeyTransformSerializeTuple {
            inner: self.inner.serialize_tuple(len)?,
            transform: self.transform,
        })
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(KeyTransformSerializeTupleStruct {
            inner: self.inner.serialize_tuple_struct(name, len)?,
            transform: self.transform,
        })
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(KeyTransformSerializeTupleVariant {
            inner: self
                .inner
                .serialize_tuple_variant(name, variant_index, variant, len)?,
            transform: self.transform,
        })
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(KeyTransformSerializeMap {
            inner: self.inner.serialize_map(len)?,
            transform: self.transform,
        })
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(KeyTransformSerializeStruct {
            inner: self.inner.serialize_map(Some(len))?,
            transform: self.transform,
        })
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(KeyTransformSerializeStructVariant {
            inner: self
                .inner
                .serialize_struct_variant(name, variant_index, variant, len)?,
            transform: self.transform,
        })
    }

    fn collect_str<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: std::fmt::Display,
    {
        self.inner.collect_str(value)
    }

    fn is_human_readable(&self) -> bool {
        self.inner.is_human_readable()
    }
}

pub(super) fn deserialize_with_key_transform_from_str<T, F>(
    input: &str,
    transform: &F,
) -> Result<T, serde_json::Error>
where
    T: DeserializeOwned,
    F: Fn(&str) -> Cow<'_, str>,
{
    let mut de = serde_json::Deserializer::from_str(input);
    T::deserialize(KeyTransformDeserializer::new(&mut de, transform))
}

pub(super) fn deserialize_with_key_transform_from_slice<T, F>(
    input: &[u8],
    transform: &F,
) -> Result<T, serde_json::Error>
where
    T: DeserializeOwned,
    F: Fn(&str) -> Cow<'_, str>,
{
    let mut de = serde_json::Deserializer::from_slice(input);
    T::deserialize(KeyTransformDeserializer::new(&mut de, transform))
}

pub(super) struct KeyTransformDeserializer<D, F> {
    inner: D,
    transform: F,
}

impl<D, F> KeyTransformDeserializer<D, F> {
    pub(super) fn new(inner: D, transform: F) -> Self {
        Self { inner, transform }
    }
}

struct KeyTransformVisitor<V, F> {
    inner: V,
    transform: F,
}

struct KeyTransformSeed<S, F> {
    inner: S,
    transform: F,
}

impl<'de, S, F> serde::de::DeserializeSeed<'de> for KeyTransformSeed<S, F>
where
    S: serde::de::DeserializeSeed<'de>,
    F: Copy + Fn(&str) -> Cow<'_, str>,
{
    type Value = S::Value;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.inner
            .deserialize(KeyTransformDeserializer::new(deserializer, self.transform))
    }
}

struct KeyTransformMapAccess<A, F> {
    inner: A,
    transform: F,
}

impl<'de, A, F> serde::de::MapAccess<'de> for KeyTransformMapAccess<A, F>
where
    A: serde::de::MapAccess<'de>,
    F: Copy + Fn(&str) -> Cow<'_, str>,
{
    type Error = A::Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        match self.inner.next_key::<String>()? {
            Some(key) => {
                let transformed = match (self.transform)(&key) {
                    Cow::Borrowed(_) => key,
                    Cow::Owned(s) => s,
                };
                seed.deserialize(transformed.into_deserializer()).map(Some)
            }
            None => Ok(None),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        self.inner.next_value_seed(KeyTransformSeed {
            inner: seed,
            transform: self.transform,
        })
    }

    fn size_hint(&self) -> Option<usize> {
        self.inner.size_hint()
    }
}

struct KeyTransformSeqAccess<A, F> {
    inner: A,
    transform: F,
}

impl<'de, A, F> serde::de::SeqAccess<'de> for KeyTransformSeqAccess<A, F>
where
    A: serde::de::SeqAccess<'de>,
    F: Copy + Fn(&str) -> Cow<'_, str>,
{
    type Error = A::Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        self.inner.next_element_seed(KeyTransformSeed {
            inner: seed,
            transform: self.transform,
        })
    }

    fn size_hint(&self) -> Option<usize> {
        self.inner.size_hint()
    }
}

macro_rules! delegate_visit {
    ($($method:ident($T:ty)),* $(,)?) => {
        $(fn $method<E: serde::de::Error>(self, v: $T) -> Result<Self::Value, E> {
            self.inner.$method(v)
        })*
    };
}

impl<'de, V, F> serde::de::Visitor<'de> for KeyTransformVisitor<V, F>
where
    V: serde::de::Visitor<'de>,
    F: Copy + Fn(&str) -> Cow<'_, str>,
{
    type Value = V::Value;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.expecting(formatter)
    }

    fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        self.inner.visit_seq(KeyTransformSeqAccess {
            inner: seq,
            transform: self.transform,
        })
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        self.inner.visit_map(KeyTransformMapAccess {
            inner: map,
            transform: self.transform,
        })
    }

    // ── Scalar delegation ────────────────────────────────────────────────────
    delegate_visit! {
        visit_bool(bool),
        visit_i8(i8), visit_i16(i16), visit_i32(i32), visit_i64(i64),
        visit_u8(u8), visit_u16(u16), visit_u32(u32), visit_u64(u64),
        visit_f32(f32), visit_f64(f64),
        visit_char(char),
        visit_str(&str), visit_borrowed_str(&'de str), visit_string(String),
        visit_bytes(&[u8]), visit_borrowed_bytes(&'de [u8]), visit_byte_buf(Vec<u8>),
    }

    fn visit_none<E: serde::de::Error>(self) -> Result<Self::Value, E> {
        self.inner.visit_none()
    }
    fn visit_unit<E: serde::de::Error>(self) -> Result<Self::Value, E> {
        self.inner.visit_unit()
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.inner
            .visit_some(KeyTransformDeserializer::new(deserializer, self.transform))
    }

    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.inner
            .visit_newtype_struct(KeyTransformDeserializer::new(deserializer, self.transform))
    }

    fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::EnumAccess<'de>,
    {
        self.inner.visit_enum(data)
    }
}

macro_rules! delegate_deser {
    ($($method:ident),* $(,)?) => {
        $(fn $method<V: serde::de::Visitor<'de>>(
            self,
            visitor: V,
        ) -> Result<V::Value, Self::Error> {
            self.inner.$method(KeyTransformVisitor {
                inner: visitor,
                transform: self.transform,
            })
        })*
    };
}

impl<'de, D, F> serde::de::Deserializer<'de> for KeyTransformDeserializer<D, F>
where
    D: serde::de::Deserializer<'de>,
    F: Copy + Fn(&str) -> Cow<'_, str>,
{
    type Error = D::Error;

    // ── Simple visitor-wrapping delegation ────────────────────────────────────
    delegate_deser! {
        deserialize_any, deserialize_bool,
        deserialize_i8, deserialize_i16, deserialize_i32, deserialize_i64,
        deserialize_u8, deserialize_u16, deserialize_u32, deserialize_u64,
        deserialize_f32, deserialize_f64,
        deserialize_char, deserialize_str, deserialize_string,
        deserialize_bytes, deserialize_byte_buf,
        deserialize_option, deserialize_unit,
        deserialize_seq, deserialize_map,
        deserialize_identifier, deserialize_ignored_any,
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
            KeyTransformVisitor {
                inner: visitor,
                transform: self.transform,
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
            KeyTransformVisitor {
                inner: visitor,
                transform: self.transform,
            },
        )
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.inner.deserialize_tuple(
            len,
            KeyTransformVisitor {
                inner: visitor,
                transform: self.transform,
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
            KeyTransformVisitor {
                inner: visitor,
                transform: self.transform,
            },
        )
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
            KeyTransformVisitor {
                inner: visitor,
                transform: self.transform,
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
            KeyTransformVisitor {
                inner: visitor,
                transform: self.transform,
            },
        )
    }

    fn is_human_readable(&self) -> bool {
        self.inner.is_human_readable()
    }
}

// ─── Key renaming (German camelCase ↔ snake_case) ─────────────────────────────

/// Converts a German camelCase JSON key to its snake_case equivalent.
///
/// Returns a borrowed `Cow` (zero allocation) when the key starts with `_`
/// or contains no uppercase letters.  Only allocates when a conversion is
/// actually needed — no intermediate `Vec<char>` is created.
///
/// Keys starting with `_` (such as `_typ`, `_version`, `_additional`) are
/// returned unchanged to preserve the BO4E convention for meta-fields.
///
/// Uses an acronym-aware sliding-window algorithm: an underscore is inserted
/// only at an uppercase→lowercase transition or at the end of an uppercase run
/// preceding a lowercase letter, not between consecutive uppercase letters.
/// Examples:
/// - `"marktlokationsId"` → `"marktlokations_id"`
/// - `"EICCode"` → `"eic_code"` (not `"e_i_c_code"`)
pub(super) fn camel_to_snake(key: &str) -> Cow<'_, str> {
    // Fast path: preserve _typ, _version, _additional etc.
    if key.starts_with('_') {
        return Cow::Borrowed(key);
    }
    // Fast path: already snake_case or a single-word name with no uppercase.
    if !key.chars().any(|c| c.is_uppercase()) {
        return Cow::Borrowed(key);
    }
    let mut result = String::with_capacity(key.len() + 4);
    let mut chars = key.chars().peekable();
    let mut prev: Option<char> = None;
    while let Some(c) = chars.next() {
        if c.is_uppercase() && prev.is_some() {
            // Insert underscore when:
            //   - previous char was lowercase/digit (e.g. marktlokations|I|d → …_i…)
            //   - or next char is lowercase and prev was uppercase (end of acronym:
            //     EIC|C|ode → …_c…)
            let prev_lower = prev.is_some_and(|p| p.is_lowercase() || p.is_ascii_digit());
            let next_lower = chars.peek().is_some_and(|n| n.is_lowercase());
            if prev_lower || (next_lower && prev.is_some_and(|p| p.is_uppercase())) {
                result.push('_');
            }
        }
        result.extend(c.to_lowercase());
        prev = Some(c);
    }
    Cow::Owned(result)
}

/// Converts a snake_case key back to the BO4E German camelCase form.
///
/// Returns a borrowed `Cow` (zero allocation) when the key starts with `_`
/// or contains no underscores (already camelCase or a single word).  Only
/// allocates when a conversion is actually needed.
///
/// This is the inverse of [`camel_to_snake`].  Keys starting with `_` are
/// passed through unchanged.
pub(super) fn snake_to_camel(key: &str) -> Cow<'_, str> {
    if key.starts_with('_') {
        return Cow::Borrowed(key);
    }
    // Fast path: no underscore means nothing to convert.
    if !key.contains('_') {
        return Cow::Borrowed(key);
    }
    let mut result = String::with_capacity(key.len());
    let mut capitalize_next = false;
    for c in key.chars() {
        if c == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            for uc in c.to_uppercase() {
                result.push(uc);
            }
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }
    Cow::Owned(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Exhaustive round-trip matrix for every transformation path.
    /// Each entry is `(camelCase, snake_case)`.
    ///
    /// **Round-trip invariant (camel→snake→camel)** only holds for entries where the
    /// camelCase form is unambiguously reconstructable from snake_case, i.e. no
    /// leading-uppercase acronym sequences.  Entries in `ACRONYM_ONE_WAY` are
    /// intentionally one-way: `camel_to_snake` is correct and lossy for acronyms,
    /// so `snake_to_camel(camel_to_snake(s)) != s` for those entries.
    const FIELD_TABLE: &[(&str, &str)] = &[
        // ── underscore-prefixed system keys (pass-through both ways) ───────────
        ("_typ", "_typ"),
        ("_version", "_version"),
        ("_additional", "_additional"),
        // ── single all-lowercase words (no change) ────────────────────────────
        ("lokationsid", "lokationsid"),
        ("marktteilnehmercode", "marktteilnehmercode"),
        ("vertragsbeginn", "vertragsbeginn"),
        ("vertragsnummer", "vertragsnummer"),
        ("zeitraum", "zeitraum"),
        ("messzeitpunkt", "messzeitpunkt"),
        // ── standard camelCase (one transition each) ─────────────────────────
        ("boTyp", "bo_typ"),
        ("lokationsId", "lokations_id"),
        ("marktlokationsId", "marktlokations_id"),
        ("messlokationsId", "messlokations_id"),
        ("netzlokationsId", "netzlokations_id"),
        ("istBilanzierungsgebiet", "ist_bilanzierungsgebiet"),
        ("gesamtNetto", "gesamt_netto"),
        ("gesamtBrutto", "gesamt_brutto"),
        ("rechnungsDatum", "rechnungs_datum"),
        ("lieferantCode", "lieferant_code"),
        ("lieferbeginn", "lieferbeginn"),
        ("lieferende", "lieferende"),
        ("zeitreihenTyp", "zeitreihen_typ"),
        ("statusZusatzInfo", "status_zusatz_info"),
        ("einheitlichesBilanzgebiet", "einheitliches_bilanzgebiet"),
        ("grundlageZurVerrechnung", "grundlage_zur_verrechnung"),
        // ── digit-containing field names ──────────────────────────────────────
        ("marktplatz2030", "marktplatz2030"),
        ("version202401", "version202401"),
        // ── BO4E field names that start with a lowercase acronym prefix ───────
        ("obisKennzahl", "obis_kennzahl"),
        ("ediCode", "edi_code"),
    ];

    /// These entries are one-way only: `camel_to_snake` is correct, but
    /// `snake_to_camel` cannot reconstruct the original because leading-uppercase
    /// acronym sequences lose their casing when lowercased.
    const ACRONYM_ONE_WAY: &[(&str, &str)] = &[
        ("EICCode", "eic_code"),
        ("XMLParser", "xml_parser"),
        ("HTMLToText", "html_to_text"),
        ("ISO8601datum", "iso8601datum"),
    ];

    #[test]
    fn camel_to_snake_table() {
        for &(camel, snake) in FIELD_TABLE {
            assert_eq!(
                camel_to_snake(camel).as_ref(),
                snake,
                "camel_to_snake({camel:?}) should be {snake:?}"
            );
        }
        for &(camel, snake) in ACRONYM_ONE_WAY {
            assert_eq!(
                camel_to_snake(camel).as_ref(),
                snake,
                "camel_to_snake({camel:?}) should be {snake:?}"
            );
        }
    }

    #[test]
    fn snake_to_camel_table() {
        for &(camel, snake) in FIELD_TABLE {
            assert_eq!(
                snake_to_camel(snake).as_ref(),
                camel,
                "snake_to_camel({snake:?}) should be {camel:?}"
            );
        }
        // Acronym entries: snake_to_camel produces lowercase initial segment, which is
        // correct for the snake→camel direction (e.g. "eic_code" → "eicCode").
        for &(_camel, snake) in ACRONYM_ONE_WAY {
            // Just ensure it doesn't panic and produces something non-empty.
            let result = snake_to_camel(snake);
            assert!(
                !result.is_empty(),
                "snake_to_camel({snake:?}) returned empty string"
            );
        }
    }

    #[test]
    fn round_trip_camel_snake_camel() {
        for &(camel, _) in FIELD_TABLE {
            let snake = camel_to_snake(camel);
            let back = snake_to_camel(snake.as_ref());
            assert_eq!(
                back.as_ref(),
                camel,
                "camel→snake→camel round-trip failed for {camel:?}"
            );
        }
    }

    #[test]
    fn round_trip_snake_camel_snake() {
        for &(_, snake) in FIELD_TABLE {
            let camel = snake_to_camel(snake);
            let back = camel_to_snake(camel.as_ref());
            assert_eq!(
                back.as_ref(),
                snake,
                "snake→camel→snake round-trip failed for {snake:?}"
            );
        }
    }

    /// All underscore-prefixed keys must be borrowed (zero allocation).
    #[test]
    fn system_keys_are_borrowed() {
        for key in ["_typ", "_version", "_additional"] {
            assert!(
                matches!(camel_to_snake(key), Cow::Borrowed(_)),
                "camel_to_snake({key:?}) should borrow"
            );
            assert!(
                matches!(snake_to_camel(key), Cow::Borrowed(_)),
                "snake_to_camel({key:?}) should borrow"
            );
        }
    }

    /// All-lowercase / already-snake keys must be borrowed (zero allocation).
    #[test]
    fn already_snake_keys_are_borrowed() {
        for key in ["lokationsid", "marktteilnehmercode", "vertragsbeginn"] {
            assert!(
                matches!(camel_to_snake(key), Cow::Borrowed(_)),
                "camel_to_snake({key:?}) should borrow"
            );
        }
    }

    /// Keys without underscores must be borrowed by snake_to_camel (zero allocation).
    #[test]
    fn no_underscore_keys_are_borrowed() {
        for key in ["lokationsid", "marktteilnehmercode", "vertragsbeginn"] {
            assert!(
                matches!(snake_to_camel(key), Cow::Borrowed(_)),
                "snake_to_camel({key:?}) should borrow"
            );
        }
    }
}
