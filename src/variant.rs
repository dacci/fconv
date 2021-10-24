use linked_hash_map::LinkedHashMap;
use serde::de::{Deserialize, Deserializer, Error, MapAccess, SeqAccess, Visitor};
use serde::ser::{Serialize, SerializeMap, SerializeSeq, Serializer};
use std::fmt::{Error as FmtError, Formatter};

#[derive(Debug, PartialEq)]
pub enum Variant {
    Bool(bool),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    F32(f32),
    F64(f64),
    Char(char),
    String(String),
    Bytes(Vec<u8>),
    Null(),
    Array(Vec<Variant>),
    Map(LinkedHashMap<String, Variant>),
}

impl<'de> Deserialize<'de> for Variant {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct VisitorImpl;

        impl<'de> Visitor<'de> for VisitorImpl {
            type Value = Variant;

            fn expecting(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
                f.write_str("expecting deserializable data")
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Variant::Bool(v))
            }

            fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Variant::I8(v))
            }

            fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Variant::I16(v))
            }

            fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Variant::I32(v))
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Variant::I64(v))
            }

            fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Variant::I128(v))
            }

            fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Variant::U8(v))
            }

            fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Variant::U16(v))
            }

            fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Variant::U32(v))
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Variant::U64(v))
            }

            fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Variant::U128(v))
            }

            fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Variant::F32(v))
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Variant::F64(v))
            }

            fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Variant::Char(v))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Variant::String(String::from(v)))
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Variant::String(v))
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Variant::Bytes(Vec::from(v)))
            }

            fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Variant::Bytes(v))
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Variant::Null())
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut v = Vec::new();
                while let Some(e) = seq.next_element()? {
                    v.push(e)
                }
                Ok(Variant::Array(v))
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut v = LinkedHashMap::new();
                while let Some((key, value)) = map.next_entry()? {
                    v.insert(key, value);
                }
                Ok(Variant::Map(v))
            }
        }

        deserializer.deserialize_any(VisitorImpl)
    }
}

impl Serialize for Variant {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Variant::Bool(v) => serializer.serialize_bool(*v),
            Variant::I8(v) => serializer.serialize_i8(*v),
            Variant::I16(v) => serializer.serialize_i16(*v),
            Variant::I32(v) => serializer.serialize_i32(*v),
            Variant::I64(v) => serializer.serialize_i64(*v),
            Variant::I128(v) => serializer.serialize_i128(*v),
            Variant::U8(v) => serializer.serialize_u8(*v),
            Variant::U16(v) => serializer.serialize_u16(*v),
            Variant::U32(v) => serializer.serialize_u32(*v),
            Variant::U64(v) => serializer.serialize_u64(*v),
            Variant::U128(v) => serializer.serialize_u128(*v),
            Variant::F32(v) => serializer.serialize_f32(*v),
            Variant::F64(v) => serializer.serialize_f64(*v),
            Variant::Char(v) => serializer.serialize_char(*v),
            Variant::String(v) => serializer.serialize_str(v),
            Variant::Bytes(v) => serializer.serialize_bytes(v),
            Variant::Null() => serializer.serialize_unit(),

            Variant::Array(v) => {
                let mut seq = serializer.serialize_seq(Some(v.len()))?;
                for element in v {
                    seq.serialize_element(element)?;
                }
                seq.end()
            }

            Variant::Map(v) => {
                let mut map = serializer.serialize_map(Some(v.len()))?;
                for (key, value) in v {
                    map.serialize_entry(key, value)?;
                }
                map.end()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_test::{assert_tokens, Token};

    #[test]
    fn test_ser_de() {
        assert_tokens(&Variant::Bool(true), &[Token::Bool(true)]);

        assert_tokens(&Variant::I8(-1), &[Token::I8(-1)]);
        assert_tokens(&Variant::I16(-1), &[Token::I16(-1)]);
        assert_tokens(&Variant::I32(-1), &[Token::I32(-1)]);
        assert_tokens(&Variant::I64(-1), &[Token::I64(-1)]);
        // assert_tokens(&Variant::I128(-1), &[Token::I128(-1)]);

        assert_tokens(&Variant::U8(1), &[Token::U8(1)]);
        assert_tokens(&Variant::U16(1), &[Token::U16(1)]);
        assert_tokens(&Variant::U32(1), &[Token::U32(1)]);
        assert_tokens(&Variant::U64(1), &[Token::U64(1)]);
        // assert_tokens(&Variant::U128(1), &[Token::U128(1)]);

        assert_tokens(&Variant::F32(0.0), &[Token::F32(0.0)]);
        assert_tokens(&Variant::F64(0.0), &[Token::F64(0.0)]);
        assert_tokens(&Variant::Char('\n'), &[Token::Char('\n')]);

        assert_tokens(
            &Variant::String(String::from("str")),
            &[Token::String("str")],
        );

        assert_tokens(&Variant::Bytes(vec![0]), &[Token::Bytes(&[0])]);
        assert_tokens(&Variant::Null(), &[Token::Unit]);

        assert_tokens(
            &Variant::Array(vec![Variant::Bool(false)]),
            &[
                Token::Seq { len: Some(1) },
                Token::Bool(false),
                Token::SeqEnd,
            ],
        );

        let mut map = LinkedHashMap::new();
        map.insert(String::from("answer"), Variant::U16(42));
        assert_tokens(
            &Variant::Map(map),
            &[
                Token::Map { len: Some(1) },
                Token::String("answer"),
                Token::U16(42),
                Token::MapEnd,
            ],
        );
    }
}
