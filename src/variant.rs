use linked_hash_map::LinkedHashMap;
use serde::de;
use serde::ser;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Variant {
    Bool(bool),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    #[cfg(not(no_integer128))]
    I128(i128),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    #[cfg(not(no_integer128))]
    U128(u128),
    F32(f32),
    F64(f64),
    Char(char),
    Str(String),
    Bytes(Vec<u8>),
    Option(Option<Box<Variant>>),
    Unit,
    NewtypeStruct(String, Box<Variant>),
    Seq(Vec<Variant>),
    Map(LinkedHashMap<String, Variant>),
}

impl<'de> de::Deserialize<'de> for Variant {
    fn deserialize<D: de::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct VisitorImpl;

        impl<'de> de::Visitor<'de> for VisitorImpl {
            type Value = Variant;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("expecting deserializable data")
            }

            fn visit_bool<E: de::Error>(self, v: bool) -> Result<Self::Value, E> {
                Ok(Variant::Bool(v))
            }

            fn visit_i8<E: de::Error>(self, v: i8) -> Result<Self::Value, E> {
                Ok(Variant::I8(v))
            }

            fn visit_i16<E: de::Error>(self, v: i16) -> Result<Self::Value, E> {
                Ok(Variant::I16(v))
            }

            fn visit_i32<E: de::Error>(self, v: i32) -> Result<Self::Value, E> {
                Ok(Variant::I32(v))
            }

            fn visit_i64<E: de::Error>(self, v: i64) -> Result<Self::Value, E> {
                Ok(Variant::I64(v))
            }

            #[cfg(not(no_integer128))]
            fn visit_i128<E: de::Error>(self, v: i128) -> Result<Self::Value, E> {
                Ok(Variant::I128(v))
            }

            fn visit_u8<E: de::Error>(self, v: u8) -> Result<Self::Value, E> {
                Ok(Variant::U8(v))
            }

            fn visit_u16<E: de::Error>(self, v: u16) -> Result<Self::Value, E> {
                Ok(Variant::U16(v))
            }

            fn visit_u32<E: de::Error>(self, v: u32) -> Result<Self::Value, E> {
                Ok(Variant::U32(v))
            }

            fn visit_u64<E: de::Error>(self, v: u64) -> Result<Self::Value, E> {
                Ok(Variant::U64(v))
            }

            #[cfg(not(no_integer128))]
            fn visit_u128<E: de::Error>(self, v: u128) -> Result<Self::Value, E> {
                Ok(Variant::U128(v))
            }

            fn visit_f32<E: de::Error>(self, v: f32) -> Result<Self::Value, E> {
                Ok(Variant::F32(v))
            }

            fn visit_f64<E: de::Error>(self, v: f64) -> Result<Self::Value, E> {
                Ok(Variant::F64(v))
            }

            fn visit_char<E: de::Error>(self, v: char) -> Result<Self::Value, E> {
                Ok(Variant::Char(v))
            }

            fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(Variant::Str(v.to_string()))
            }

            fn visit_bytes<E: de::Error>(self, v: &[u8]) -> Result<Self::Value, E> {
                Ok(Variant::Bytes(v.to_vec()))
            }

            fn visit_none<E: de::Error>(self) -> Result<Self::Value, E> {
                Ok(Variant::Option(None))
            }

            fn visit_some<D: de::Deserializer<'de>>(self, d: D) -> Result<Self::Value, D::Error> {
                let v = d.deserialize_any(VisitorImpl)?;
                Ok(Variant::Option(Some(Box::new(v))))
            }

            fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E> {
                Ok(Variant::Unit)
            }

            fn visit_newtype_struct<D: de::Deserializer<'de>>(
                self,
                d: D,
            ) -> Result<Self::Value, D::Error> {
                let v: (String, Box<Variant>) = de::Deserialize::deserialize(d)?;
                Ok(Variant::NewtypeStruct(v.0, v.1))
            }

            fn visit_seq<A: de::SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                let mut v = if let Some(len) = seq.size_hint() {
                    Vec::with_capacity(len)
                } else {
                    Vec::new()
                };

                while let Some(e) = seq.next_element()? {
                    v.push(e);
                }

                Ok(Variant::Seq(v))
            }

            fn visit_map<A: de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut v = if let Some(len) = map.size_hint() {
                    LinkedHashMap::with_capacity(len)
                } else {
                    LinkedHashMap::new()
                };

                while let Some((key, value)) = map.next_entry::<String, Variant>()? {
                    v.insert(key, value);
                }

                Ok(Variant::Map(v))
            }
        }

        d.deserialize_any(VisitorImpl)
    }
}

impl ser::Serialize for Variant {
    fn serialize<S: ser::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self {
            Variant::Bool(v) => s.serialize_bool(*v),
            Variant::I8(v) => s.serialize_i8(*v),
            Variant::I16(v) => s.serialize_i16(*v),
            Variant::I32(v) => s.serialize_i32(*v),
            Variant::I64(v) => s.serialize_i64(*v),
            #[cfg(not(no_integer128))]
            Variant::I128(v) => s.serialize_i128(*v),
            Variant::U8(v) => s.serialize_u8(*v),
            Variant::U16(v) => s.serialize_u16(*v),
            Variant::U32(v) => s.serialize_u32(*v),
            Variant::U64(v) => s.serialize_u64(*v),
            #[cfg(not(no_integer128))]
            Variant::U128(v) => s.serialize_u128(*v),
            Variant::F32(v) => s.serialize_f32(*v),
            Variant::F64(v) => s.serialize_f64(*v),
            Variant::Char(v) => s.serialize_char(*v),
            Variant::Str(v) => s.serialize_str(v),
            Variant::Bytes(v) => s.serialize_bytes(v),

            Variant::Option(v) => {
                if let Some(v) = v {
                    s.serialize_some(v)
                } else {
                    s.serialize_none()
                }
            }

            Variant::Unit => s.serialize_unit(),
            Variant::NewtypeStruct(n, v) => s.serialize_newtype_struct("_", &(n, v)),

            Variant::Seq(v) => {
                use ser::SerializeSeq;

                let mut seq = s.serialize_seq(Some(v.len()))?;
                for e in v {
                    seq.serialize_element(e)?;
                }
                seq.end()
            }

            Variant::Map(m) => {
                use ser::SerializeMap;

                let mut map = s.serialize_map(Some(m.len()))?;
                for (k, v) in m {
                    map.serialize_entry(k, v)?;
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
        assert_tokens(&Variant::Str("str".to_string()), &[Token::Str("str")]);
        assert_tokens(&Variant::Bytes(vec![0]), &[Token::Bytes(&[0])]);
        assert_tokens(&Variant::Option(None), &[Token::None]);
        assert_tokens(
            &[Variant::Option(Some(Box::new(Variant::Unit)))],
            &[
                Token::Tuple { len: 1 },
                Token::Some,
                Token::Unit,
                Token::TupleEnd,
            ],
        );
        assert_tokens(&Variant::Unit, &[Token::Unit]);
        assert_tokens(
            &Variant::NewtypeStruct("Type".to_string(), Box::new(Variant::Unit)),
            &[
                Token::NewtypeStruct { name: "_" },
                Token::Tuple { len: 2 },
                Token::Str("Type"),
                Token::Unit,
                Token::TupleEnd,
            ],
        );
        assert_tokens(
            &Variant::Seq(vec![Variant::Unit]),
            &[Token::Seq { len: Some(1) }, Token::Unit, Token::SeqEnd],
        );
        assert_tokens(
            &Variant::Map(LinkedHashMap::new()),
            &[Token::Map { len: Some(0) }, Token::MapEnd],
        );
    }
}
