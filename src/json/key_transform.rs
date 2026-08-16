//! Key-name transformation between BO4E German camelCase and Rust snake_case,
//! with serializer and deserializer wrappers that apply the transformations
//! transparently in a single pass.

// `?Sized` on the generic parameter is required by serde's SerializeMap trait;
// the `where T: Serialize` clause is also required. Both are intentional.
#![allow(clippy::multiple_bound_locations)]

use serde::de::DeserializeOwned;
use serde::de::Error as _;
use serde::de::IntoDeserializer;
use serde::Serialize;

/// Maps a JSON key to its counterpart in the other naming mode.
///
/// `None` means "no mapping — pass this key through byte-for-byte", which is the
/// case for extension-data keys and the `_`-prefixed BO4E metadata keys. A
/// `Some` result is always a `&'static str` borrowed from the generated key map,
/// so renaming a key never allocates on either the serialize or deserialize path.
///
/// The `Option` carries the "unchanged" case explicitly on purpose. An earlier
/// version returned `Cow` and treated `Cow::Borrowed` as "unchanged"; once the
/// transform started borrowing its *result* from a static table, that inference
/// silently dropped every rewrite.
pub(super) type KeyTransformFn = fn(&str) -> Option<&'static str>;

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
/// non-string key branches are removed — BO4E JSON objects always
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
        self.inner
            .serialize_key((self.transform)(&key).unwrap_or(&key))
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
        self.inner.serialize_entry(
            (self.transform)(&key).unwrap_or(&key),
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
        self.inner.serialize_entry(
            (self.transform)(key).unwrap_or(key),
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
        // Rename the field exactly as `SerializeStruct` does. No BO4E type is a
        // struct variant today, so this branch is unreachable from generated
        // code — but leaving the key untransformed here while transforming it
        // for plain structs would make the wrapper's behaviour depend on which
        // serde shape a type happens to use.
        self.inner.serialize_field(
            (self.transform)(key).unwrap_or(key),
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
    F: Fn(&str) -> Option<&'static str>,
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
    F: Fn(&str) -> Option<&'static str>,
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
    F: Copy + Fn(&str) -> Option<&'static str>,
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
    F: Copy + Fn(&str) -> Option<&'static str>,
{
    type Error = A::Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        match self.inner.next_key::<String>()? {
            // A mapped key is `&'static`, so the rename costs no allocation; an
            // unmapped key reuses the `String` the parser already produced.
            Some(key) => match (self.transform)(&key) {
                Some(mapped) => seed.deserialize(mapped.into_deserializer()).map(Some),
                None => seed.deserialize(key.into_deserializer()).map(Some),
            },
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
    F: Copy + Fn(&str) -> Option<&'static str>,
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
    F: Copy + Fn(&str) -> Option<&'static str>,
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
    F: Copy + Fn(&str) -> Option<&'static str>,
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
//
// Both directions are exact table lookups into the generated key map, never
// heuristics.  A heuristic cannot be correct here: `hoechstpreis_ht` is an
// equally valid snake_case rendering of `hoechstpreisHt` and `hoechstpreisHT`,
// and `a` of both `a` and `A`.  BO4E contains all of those shapes
// (`Tarifberechnungsparameter.hoechstpreisHT`, `Sigmoidparameter.A`,
// `PreisblattKonzessionsabgabe.kundengruppeKA`), so a heuristic inverse maps
// them back to a name no field answers to and the value lands in the
// extension-data bag instead of its typed field — silent data loss on a
// `to_json_snake_case` → `from_json_snake_case` round-trip.
//
// The generator knows both names for every field, so it emits the mapping
// (`src/generated/key_map.rs`) and the round-trip is lossless by construction.
// Keys outside the table — extension data, and BO4E metadata keys like `_typ` —
// pass through untouched, which is also what keeps *those* lossless.

use crate::generated::key_map::{SNAKE_TO_WIRE, WIRE_TO_SNAKE};

/// Converts a BO4E wire key (German camelCase) to its Rust snake_case field name.
///
/// Returns `None` for keys with no mapping — extension data, and the
/// `_`-prefixed BO4E metadata keys (`_typ`, `_version`, `_id`) — which must pass
/// through unchanged to survive a round-trip.
///
/// - `"marktlokationsId"` → `Some("marktlokations_id")`
/// - `"hoechstpreisHT"` → `Some("hoechstpreis_ht")`
/// - `"A"` → `Some("a")`
/// - `"_typ"` → `None` (metadata key, passes through)
pub(super) fn camel_to_snake(key: &str) -> Option<&'static str> {
    WIRE_TO_SNAKE
        .binary_search_by_key(&key, |&(wire, _)| wire)
        .ok()
        .map(|i| WIRE_TO_SNAKE[i].1)
}

/// Converts a Rust snake_case field name back to its BO4E wire key.
///
/// The exact inverse of [`camel_to_snake`] for every generated field; keys with
/// no mapping return `None` and pass through unchanged.
///
/// - `"marktlokations_id"` → `Some("marktlokationsId")`
/// - `"hoechstpreis_ht"` → `Some("hoechstpreisHT")`
/// - `"a"` → `Some("A")`
pub(super) fn snake_to_camel(key: &str) -> Option<&'static str> {
    SNAKE_TO_WIRE
        .binary_search_by_key(&key, |&(snake, _)| snake)
        .ok()
        .map(|i| SNAKE_TO_WIRE[i].1)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::generated::key_map::{SNAKE_TO_WIRE, WIRE_TO_SNAKE};

    /// Both generated tables must be sorted — `binary_search_by_key` silently
    /// returns wrong answers on unsorted input, which would corrupt keys rather
    /// than fail loudly.
    #[test]
    fn generated_tables_are_sorted() {
        assert!(
            WIRE_TO_SNAKE.windows(2).all(|w| w[0].0 < w[1].0),
            "WIRE_TO_SNAKE must be strictly sorted by wire name"
        );
        assert!(
            SNAKE_TO_WIRE.windows(2).all(|w| w[0].0 < w[1].0),
            "SNAKE_TO_WIRE must be strictly sorted by snake name"
        );
    }

    /// The two tables must describe the same bijection, read in either direction.
    #[test]
    fn generated_tables_are_exact_inverses() {
        assert_eq!(WIRE_TO_SNAKE.len(), SNAKE_TO_WIRE.len());
        for &(wire, snake) in WIRE_TO_SNAKE {
            assert_eq!(
                camel_to_snake(wire),
                Some(snake),
                "camel_to_snake({wire:?})"
            );
            assert_eq!(
                snake_to_camel(snake),
                Some(wire),
                "snake_to_camel({snake:?})"
            );
        }
    }

    /// The property that actually matters: every BO4E wire key survives a
    /// camel→snake→camel round-trip, so `to_json_snake_case` followed by
    /// `from_json_snake_case` can never move a typed field into extension data.
    ///
    /// The heuristic this table replaced failed exactly here, for
    /// `Sigmoidparameter.A`, `Tarifberechnungsparameter.hoechstpreisHT`, and
    /// `PreisblattKonzessionsabgabe.kundengruppeKA`.
    #[test]
    fn every_wire_key_round_trips() {
        for &(wire, _) in WIRE_TO_SNAKE {
            let snake = camel_to_snake(wire).unwrap_or(wire);
            assert_eq!(
                snake_to_camel(snake).unwrap_or(snake),
                wire,
                "round-trip broke {wire:?}"
            );
        }
        for &(snake, _) in SNAKE_TO_WIRE {
            let wire = snake_to_camel(snake).unwrap_or(snake);
            assert_eq!(
                camel_to_snake(wire).unwrap_or(wire),
                snake,
                "round-trip broke {snake:?}"
            );
        }
    }

    /// The three shapes a heuristic inverse cannot recover.
    #[test]
    fn ambiguous_shapes_map_exactly() {
        for (wire, snake) in [
            ("A", "a"),
            ("B", "b"),
            ("hoechstpreisHT", "hoechstpreis_ht"),
            ("hoechstpreisNT", "hoechstpreis_nt"),
            ("kundengruppeKA", "kundengruppe_ka"),
            ("marktlokationsId", "marktlokations_id"),
        ] {
            assert_eq!(camel_to_snake(wire), Some(snake));
            assert_eq!(snake_to_camel(snake), Some(wire));
        }
    }

    /// BO4E metadata keys are not Rust field names and must survive verbatim in
    /// both directions, in every output mode.
    #[test]
    fn metadata_keys_pass_through() {
        for key in ["_typ", "_version", "_id", "_additional"] {
            assert_eq!(camel_to_snake(key), None, "camel_to_snake({key:?})");
            assert_eq!(snake_to_camel(key), None, "snake_to_camel({key:?})");
        }
    }

    /// Keys the schema does not define — extension data — must not be rewritten.
    /// Rewriting them with a heuristic is what made unknown fields lossy before.
    #[test]
    fn unknown_keys_pass_through_unchanged() {
        for key in [
            "someVendorField",
            "some_vendor_field",
            "fooBAR",
            "XMLPayload",
            "already_snake",
            "",
        ] {
            assert_eq!(camel_to_snake(key), None, "camel_to_snake({key:?})");
            assert_eq!(snake_to_camel(key), None, "snake_to_camel({key:?})");
        }
    }

    /// Drift guard, read straight from the schema snapshots rather than from the
    /// generator's own output: **every** BO4E property name must survive
    /// camel→snake→camel.
    ///
    /// This is the invariant that failed before the key map existed. Checking it
    /// against the schemas — not against `WIRE_TO_SNAKE` — is what makes it a
    /// guard: a property the generator forgot to put in the table shows up here,
    /// whereas a table-only test would happily pass on an incomplete table.
    #[test]
    fn every_schema_property_round_trips() {
        let schemas = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("generator/schemas");

        let mut checked = 0usize;
        let mut stack = vec![schemas.clone()];
        while let Some(dir) = stack.pop() {
            let Ok(entries) = std::fs::read_dir(&dir) else {
                continue;
            };
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    stack.push(path);
                    continue;
                }
                if path.extension().is_none_or(|e| e != "json") {
                    continue;
                }
                let raw = std::fs::read_to_string(&path).expect("readable schema");
                let doc: serde_json::Value = serde_json::from_str(&raw).expect("valid schema JSON");
                let Some(props) = doc.get("properties").and_then(|p| p.as_object()) else {
                    continue;
                };
                for wire in props.keys() {
                    let snake = camel_to_snake(wire).unwrap_or(wire);
                    let back = snake_to_camel(snake).unwrap_or(snake);
                    assert_eq!(
                        back,
                        wire.as_str(),
                        "{}: property {wire:?} does not survive a snake_case round-trip \
                         (became {back:?} via {snake:?}); `just generate` may be stale",
                        path.display(),
                    );
                    checked += 1;
                }
            }
        }

        assert!(
            checked > 400,
            "expected to check the whole BO4E property set, only saw {checked} \
             — is {} populated?",
            schemas.display(),
        );
    }

    /// A mapped key must never be reported as "unchanged", and an unmapped key
    /// must never be reported as mapped-to-itself: the deserializer distinguishes
    /// the two cases by `Option`, so a blurred boundary would drop renames.
    #[test]
    fn mapping_and_passthrough_never_overlap() {
        for &(wire, snake) in WIRE_TO_SNAKE {
            assert_ne!(wire, snake, "identity pairs do not belong in the table");
            assert_eq!(camel_to_snake(wire), Some(snake));
        }
        for key in ["_typ", "unknownKey", "vertragsbeginn"] {
            assert_eq!(camel_to_snake(key), None);
            assert_eq!(snake_to_camel(key), None);
        }
    }
}
