//! Default serializer for `reqwest` crate.
//!
//! Check [`serializers module`] documentation for more information about serializers.
//!
//! [`serializers module`]: crate::serializers

use reqwest::multipart::{Form, Part};
use serde::{
    ser::{Error as SerError, Impossible, SerializeSeq, SerializeStruct},
    Serialize, Serializer,
};
use std::{
    borrow::Cow,
    cell::RefCell,
    fmt::{Debug, Display, Error as FmtError, Write},
};

#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    #[error("Cannot serialize a field, custom error: {0}")]
    Custom(Cow<'static, str>),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Fmt(#[from] FmtError),
}

impl Error {
    fn top_level(val: impl Debug) -> Self {
        Self::Custom(format!("Cannot serialize a top-level struct: {val:?}").into())
    }
}

impl SerError for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Self::Custom(msg.to_string().into())
    }
}

pub(crate) struct MultipartSerializer {
    form: RefCell<Form>,
}

struct PartSerializer;

struct JsonPartSerializer {
    buf: String,
    state: PartSerializerStructState,
}

enum PartSerializerStructState {
    Empty,
    Rest,
}

impl MultipartSerializer {
    pub(crate) fn new() -> Self {
        Self {
            form: RefCell::new(Form::new()),
        }
    }
}

impl Serializer for MultipartSerializer {
    type Ok = Form;
    type Error = Error;

    type SerializeStruct = Self;
    type SerializeSeq = Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = Impossible<Self::Ok, Self::Error>;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(self)
    }

    fn serialize_bool(self, val: bool) -> Result<Self::Ok, Self::Error> {
        Err(Error::top_level(val))
    }

    fn serialize_i8(self, val: i8) -> Result<Self::Ok, Self::Error> {
        Err(Error::top_level(val))
    }

    fn serialize_i16(self, val: i16) -> Result<Self::Ok, Self::Error> {
        Err(Error::top_level(val))
    }

    fn serialize_i32(self, val: i32) -> Result<Self::Ok, Self::Error> {
        Err(Error::top_level(val))
    }

    fn serialize_i64(self, val: i64) -> Result<Self::Ok, Self::Error> {
        Err(Error::top_level(val))
    }

    fn serialize_u8(self, val: u8) -> Result<Self::Ok, Self::Error> {
        Err(Error::top_level(val))
    }

    fn serialize_u16(self, val: u16) -> Result<Self::Ok, Self::Error> {
        Err(Error::top_level(val))
    }

    fn serialize_u32(self, val: u32) -> Result<Self::Ok, Self::Error> {
        Err(Error::top_level(val))
    }

    fn serialize_u64(self, val: u64) -> Result<Self::Ok, Self::Error> {
        Err(Error::top_level(val))
    }

    fn serialize_f32(self, val: f32) -> Result<Self::Ok, Self::Error> {
        Err(Error::top_level(val))
    }

    fn serialize_f64(self, val: f64) -> Result<Self::Ok, Self::Error> {
        Err(Error::top_level(val))
    }

    fn serialize_char(self, val: char) -> Result<Self::Ok, Self::Error> {
        Err(Error::top_level(val))
    }

    fn serialize_str(self, val: &str) -> Result<Self::Ok, Self::Error> {
        Err(Error::top_level(val))
    }

    fn serialize_bytes(self, val: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(Error::top_level(val))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::top_level("none"))
    }

    fn serialize_some<T>(self, _val: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize + ?Sized,
    {
        Err(Error::top_level("some: (...)"))
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::top_level("unit"))
    }

    fn serialize_unit_struct(self, val: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(Error::top_level(format!("unit_struct: {val}")))
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(Error::top_level(format!(
            "unit_variant: name: {name}, variant_index: {variant_index}, variant: {variant}",
            name = name,
            variant_index = variant_index,
            variant = variant
        )))
    }

    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize + ?Sized,
    {
        Err(Error::top_level(format!(
            "newtype_struct: name: {name}, value: (...)"
        )))
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize + ?Sized,
    {
        Err(Error::top_level(format!(
            "newtype_variant: name: {name}, variant_index: {variant_index}, variant: {variant}, value: (...)"
        )))
    }

    fn serialize_seq(self, val: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(Error::top_level(val))
    }

    fn serialize_tuple(self, val: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(Error::top_level(val))
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(Error::top_level(format!(
            "tuple_struct: name: {name}, len: {len}",
            name = name,
            len = len
        )))
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(Error::top_level(format!(
            "tuple_variant: name: {name}, variant_index: {variant_index}, variant: {variant}, len: {len}"
        )))
    }

    fn serialize_map(self, val: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(Error::top_level(val))
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(Error::top_level(format!(
            "struct_variant: name: {name}, variant_index: {variant_index}, variant: {variant}, len: {len}"
        )))
    }
}

impl SerializeStruct for MultipartSerializer {
    type Ok = Form;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        let part = value.serialize(PartSerializer {})?;
        self.form.replace(self.form.take().part(key, part));

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.form.into_inner())
    }
}

impl Serializer for PartSerializer {
    type Ok = Part;
    type Error = Error;

    type SerializeStruct = JsonPartSerializer;
    type SerializeSeq = JsonPartSerializer;
    type SerializeTuple = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = Impossible<Self::Ok, Self::Error>;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, val: bool) -> Result<Self::Ok, Self::Error> {
        Ok(Part::text(val.to_string()))
    }

    fn serialize_i8(self, val: i8) -> Result<Self::Ok, Self::Error> {
        Ok(Part::text(val.to_string()))
    }

    fn serialize_i16(self, val: i16) -> Result<Self::Ok, Self::Error> {
        Ok(Part::text(val.to_string()))
    }

    fn serialize_i32(self, val: i32) -> Result<Self::Ok, Self::Error> {
        Ok(Part::text(val.to_string()))
    }

    fn serialize_i64(self, val: i64) -> Result<Self::Ok, Self::Error> {
        Ok(Part::text(val.to_string()))
    }

    fn serialize_u8(self, val: u8) -> Result<Self::Ok, Self::Error> {
        Ok(Part::text(val.to_string()))
    }

    fn serialize_u16(self, val: u16) -> Result<Self::Ok, Self::Error> {
        Ok(Part::text(val.to_string()))
    }

    fn serialize_u32(self, val: u32) -> Result<Self::Ok, Self::Error> {
        Ok(Part::text(val.to_string()))
    }

    fn serialize_u64(self, val: u64) -> Result<Self::Ok, Self::Error> {
        Ok(Part::text(val.to_string()))
    }

    fn serialize_f32(self, val: f32) -> Result<Self::Ok, Self::Error> {
        Ok(Part::text(val.to_string()))
    }

    fn serialize_f64(self, val: f64) -> Result<Self::Ok, Self::Error> {
        Ok(Part::text(val.to_string()))
    }

    fn serialize_char(self, val: char) -> Result<Self::Ok, Self::Error> {
        Ok(Part::text(val.to_string()))
    }

    fn serialize_str(self, val: &str) -> Result<Self::Ok, Self::Error> {
        Ok(Part::text(val.to_owned()))
    }

    fn serialize_bytes(self, val: &[u8]) -> Result<Self::Ok, Self::Error> {
        Ok(Part::bytes(val.to_owned()))
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize + ?Sized,
    {
        value.serialize(self)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Ok(Part::text(variant))
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(JsonPartSerializer {
            buf: String::new(),
            state: PartSerializerStructState::Empty,
        })
    }

    fn serialize_seq(self, _val: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(JsonPartSerializer {
            buf: String::new(),
            state: PartSerializerStructState::Empty,
        })
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_unit_struct(self, _val: &'static str) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize + ?Sized,
    {
        unimplemented!()
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize + ?Sized,
    {
        unimplemented!()
    }

    fn serialize_tuple(self, _val: usize) -> Result<Self::SerializeTuple, Self::Error> {
        unimplemented!()
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        unimplemented!()
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        unimplemented!()
    }

    fn serialize_map(self, _val: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        unimplemented!()
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        unimplemented!()
    }
}

impl SerializeStruct for JsonPartSerializer {
    type Ok = Part;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        let value = serde_json::to_string(value)?;
        match self.state {
            PartSerializerStructState::Empty => {
                self.state = PartSerializerStructState::Rest;

                write!(&mut self.buf, "{{\"{key}\":{value}")?;
            }
            PartSerializerStructState::Rest => write!(&mut self.buf, ",\"{key}\":{value}")?,
        }

        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        match self.state {
            PartSerializerStructState::Empty => Ok(Part::text("{{}}")),
            PartSerializerStructState::Rest => {
                self.buf += "}";

                Ok(Part::text(self.buf))
            }
        }
    }
}

impl SerializeSeq for JsonPartSerializer {
    type Ok = Part;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        let value = serde_json::to_string(value)?;
        match self.state {
            PartSerializerStructState::Empty => {
                self.state = PartSerializerStructState::Rest;

                write!(&mut self.buf, "[{value}")?;
            }
            PartSerializerStructState::Rest => write!(&mut self.buf, ",{value}")?,
        }

        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        match self.state {
            PartSerializerStructState::Empty => Ok(Part::text("[]")),
            PartSerializerStructState::Rest => {
                self.buf += "]";

                Ok(Part::text(self.buf))
            }
        }
    }
}
