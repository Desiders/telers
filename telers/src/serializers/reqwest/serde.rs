//! Multipart encoding through Serde serializer traits.

use super::Error;
use ::serde::{
    ser::{
        Error as SerError, Impossible, SerializeMap, SerializeSeq, SerializeStruct,
        SerializeStructVariant, SerializeTuple, SerializeTupleStruct, SerializeTupleVariant,
    },
    Serialize, Serializer,
};
use reqwest::multipart::{Form, Part};
use std::{cell::RefCell, fmt::Display};

impl SerError for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Self::Custom(msg.to_string().into())
    }
}

struct FormSerializer {
    form: RefCell<Form>,
    key: Option<String>,
}

struct PartSerializer<'a> {
    json: &'a mut serde_json::Serializer<Vec<u8>>,
}

struct JsonPartSerializer<'a>(
    serde_json::ser::Compound<'a, Vec<u8>, serde_json::ser::CompactFormatter>,
);

fn part(value: &(impl Serialize + ?Sized)) -> Result<Part, Error> {
    let mut json = serde_json::Serializer::new(Vec::new());
    match value.serialize(PartSerializer {
        json: &mut json,
    })? {
        Some(part) => Ok(part),
        None => Ok(Part::bytes(json.into_inner())),
    }
}

pub(super) fn serialize(data: &impl Serialize) -> Result<Form, Error> {
    data.serialize(FormSerializer::new())
}

impl FormSerializer {
    fn new() -> Self {
        Self {
            form: RefCell::new(Form::new()),
            key: None,
        }
    }
}

impl Serializer for FormSerializer {
    type Error = Error;
    type Ok = Form;
    type SerializeMap = Self;
    type SerializeSeq = Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = Self;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;

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

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize + ?Sized,
    {
        value.serialize(self)
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
        )))
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize + ?Sized,
    {
        value.serialize(self)
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
            "newtype_variant: name: {name}, variant_index: {variant_index}, variant: {variant}, \
             value: (...)"
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
            "tuple_struct: name: {name}, len: {len}"
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
            "tuple_variant: name: {name}, variant_index: {variant_index}, variant: {variant}, \
             len: {len}"
        )))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(Error::top_level(format!(
            "struct_variant: name: {name}, variant_index: {variant_index}, variant: {variant}, \
             len: {len}"
        )))
    }
}

impl SerializeStruct for FormSerializer {
    type Error = Error;
    type Ok = Form;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        let part = part(value)?;
        self.form.replace(self.form.take().part(key, part));

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.form.into_inner())
    }
}

impl SerializeMap for FormSerializer {
    type Error = Error;
    type Ok = Form;

    fn serialize_key<T: Serialize + ?Sized>(&mut self, key: &T) -> Result<(), Error> {
        self.key = Some(serde_json::from_str(&serde_json::to_string(key)?)?);
        Ok(())
    }

    fn serialize_value<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), Error> {
        let key = self
            .key
            .take()
            .ok_or_else(|| Error::custom("missing map key"))?;
        let part = part(value)?;
        self.form.replace(self.form.take().part(key, part));
        Ok(())
    }

    fn end(self) -> Result<Form, Error> {
        Ok(self.form.into_inner())
    }
}

impl<'a> Serializer for PartSerializer<'a> {
    type Error = Error;
    type Ok = Option<Part>;
    type SerializeMap = JsonPartSerializer<'a>;
    type SerializeSeq = JsonPartSerializer<'a>;
    type SerializeStruct = JsonPartSerializer<'a>;
    type SerializeStructVariant = JsonPartSerializer<'a>;
    type SerializeTuple = JsonPartSerializer<'a>;
    type SerializeTupleStruct = JsonPartSerializer<'a>;
    type SerializeTupleVariant = JsonPartSerializer<'a>;

    fn serialize_bool(self, val: bool) -> Result<Self::Ok, Self::Error> {
        Ok(Some(Part::text(val.to_string())))
    }

    fn serialize_i8(self, val: i8) -> Result<Self::Ok, Self::Error> {
        Ok(Some(Part::text(val.to_string())))
    }

    fn serialize_i16(self, val: i16) -> Result<Self::Ok, Self::Error> {
        Ok(Some(Part::text(val.to_string())))
    }

    fn serialize_i32(self, val: i32) -> Result<Self::Ok, Self::Error> {
        Ok(Some(Part::text(val.to_string())))
    }

    fn serialize_i64(self, val: i64) -> Result<Self::Ok, Self::Error> {
        Ok(Some(Part::text(val.to_string())))
    }

    fn serialize_i128(self, val: i128) -> Result<Self::Ok, Self::Error> {
        Ok(Some(Part::text(val.to_string())))
    }

    fn serialize_u8(self, val: u8) -> Result<Self::Ok, Self::Error> {
        Ok(Some(Part::text(val.to_string())))
    }

    fn serialize_u16(self, val: u16) -> Result<Self::Ok, Self::Error> {
        Ok(Some(Part::text(val.to_string())))
    }

    fn serialize_u32(self, val: u32) -> Result<Self::Ok, Self::Error> {
        Ok(Some(Part::text(val.to_string())))
    }

    fn serialize_u64(self, val: u64) -> Result<Self::Ok, Self::Error> {
        Ok(Some(Part::text(val.to_string())))
    }

    fn serialize_u128(self, val: u128) -> Result<Self::Ok, Self::Error> {
        Ok(Some(Part::text(val.to_string())))
    }

    fn serialize_f32(self, val: f32) -> Result<Self::Ok, Self::Error> {
        Ok(Some(Part::text(val.to_string())))
    }

    fn serialize_f64(self, val: f64) -> Result<Self::Ok, Self::Error> {
        Ok(Some(Part::text(val.to_string())))
    }

    fn serialize_char(self, val: char) -> Result<Self::Ok, Self::Error> {
        Ok(Some(Part::text(val.to_string())))
    }

    fn serialize_str(self, val: &str) -> Result<Self::Ok, Self::Error> {
        Ok(Some(Part::text(val.to_owned())))
    }

    fn serialize_bytes(self, val: &[u8]) -> Result<Self::Ok, Self::Error> {
        Ok(Some(Part::bytes(val.to_owned())))
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
        Ok(Some(Part::text(variant)))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(Some(Part::text("null")))
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_none()
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_none()
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize + ?Sized,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize + ?Sized,
    {
        self.json
            .serialize_newtype_variant(name, index, variant, value)?;
        Ok(None)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(JsonPartSerializer(self.json.serialize_seq(len)?))
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(JsonPartSerializer(self.json.serialize_tuple(len)?))
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(JsonPartSerializer(
            self.json.serialize_tuple_struct(name, len)?,
        ))
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(JsonPartSerializer(
            self.json
                .serialize_tuple_variant(name, index, variant, len)?,
        ))
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(JsonPartSerializer(self.json.serialize_map(len)?))
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(JsonPartSerializer(self.json.serialize_struct(name, len)?))
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(JsonPartSerializer(
            self.json
                .serialize_struct_variant(name, index, variant, len)?,
        ))
    }
}

impl SerializeSeq for JsonPartSerializer<'_> {
    type Error = Error;
    type Ok = Option<Part>;

    fn serialize_element<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), Error> {
        Ok(SerializeSeq::serialize_element(&mut self.0, value)?)
    }

    fn end(self) -> Result<Self::Ok, Error> {
        SerializeSeq::end(self.0)?;
        Ok(None)
    }
}

impl SerializeTuple for JsonPartSerializer<'_> {
    type Error = Error;
    type Ok = Option<Part>;

    fn serialize_element<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), Error> {
        Ok(SerializeTuple::serialize_element(&mut self.0, value)?)
    }

    fn end(self) -> Result<Self::Ok, Error> {
        SerializeTuple::end(self.0)?;
        Ok(None)
    }
}

impl SerializeTupleStruct for JsonPartSerializer<'_> {
    type Error = Error;
    type Ok = Option<Part>;

    fn serialize_field<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), Error> {
        Ok(SerializeTupleStruct::serialize_field(&mut self.0, value)?)
    }

    fn end(self) -> Result<Self::Ok, Error> {
        SerializeTupleStruct::end(self.0)?;
        Ok(None)
    }
}

impl SerializeTupleVariant for JsonPartSerializer<'_> {
    type Error = Error;
    type Ok = Option<Part>;

    fn serialize_field<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), Error> {
        Ok(SerializeTupleVariant::serialize_field(&mut self.0, value)?)
    }

    fn end(self) -> Result<Self::Ok, Error> {
        SerializeTupleVariant::end(self.0)?;
        Ok(None)
    }
}

impl SerializeStruct for JsonPartSerializer<'_> {
    type Error = Error;
    type Ok = Option<Part>;

    fn serialize_field<T: Serialize + ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Error> {
        Ok(SerializeStruct::serialize_field(&mut self.0, key, value)?)
    }

    fn end(self) -> Result<Self::Ok, Error> {
        SerializeStruct::end(self.0)?;
        Ok(None)
    }
}

impl SerializeStructVariant for JsonPartSerializer<'_> {
    type Error = Error;
    type Ok = Option<Part>;

    fn serialize_field<T: Serialize + ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Error> {
        Ok(SerializeStructVariant::serialize_field(
            &mut self.0,
            key,
            value,
        )?)
    }

    fn end(self) -> Result<Self::Ok, Error> {
        SerializeStructVariant::end(self.0)?;
        Ok(None)
    }
}

impl SerializeMap for JsonPartSerializer<'_> {
    type Error = Error;
    type Ok = Option<Part>;

    fn serialize_key<T: Serialize + ?Sized>(&mut self, key: &T) -> Result<(), Error> {
        Ok(SerializeMap::serialize_key(&mut self.0, key)?)
    }

    fn serialize_value<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), Error> {
        Ok(SerializeMap::serialize_value(&mut self.0, value)?)
    }

    fn end(self) -> Result<Self::Ok, Error> {
        SerializeMap::end(self.0)?;
        Ok(None)
    }
}
