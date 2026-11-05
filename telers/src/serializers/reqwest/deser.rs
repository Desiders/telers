//! Multipart parameters serialized directly from native Deser emitters.

use super::Error;
use crate::serialization::Serialize;
use ::deser::{
    ser::{Emit, Layer, Next, SerializeDriver, SerializeRef, Serializer},
    Atom, BytesFormat, Context, Event, State,
};
use reqwest::multipart::{Form, Part};

pub(super) fn serialize<T: Serialize + ?Sized>(data: &T) -> Result<Form, Error> {
    let mut state = State::new();
    let emitted = T::serialize(data, &mut state)?;
    let result = (|| {
        let mut form = Form::new();
        match emitted {
            Emit::Struct(mut fields) => {
                while let Some((key, value)) = fields.next(&mut state)? {
                    form = form.part(key.into_owned(), part(value.get())?);
                }
            }
            Emit::Map(mut fields) => loop {
                let key = {
                    let Some(key) = fields.next_key(&mut state)? else {
                        break;
                    };
                    let json = deser_json::to_string(&key.get())?;
                    deser_json::from_str::<String>(&json)?
                };
                let value = fields.next_value(&mut state)?;
                form = form.part(key, part(value.get())?);
            },
            Emit::Forward(value) => return serialize(&value.get()),
            _ => return Err(Error::top_level("expected a struct or string-keyed map")),
        }
        Ok(form)
    })();
    let finished = T::finish(data, &mut state);
    result.and_then(|form| {
        finished?;
        Ok(form)
    })
}

#[derive(Debug, Default)]
struct PartBytes(Option<Vec<u8>>);

struct ScalarLayer {
    first: bool,
}

impl Layer for ScalarLayer {
    fn event(&mut self, event: Event<'_>, next: &mut Next<'_>) -> Result<(), ::deser::Error> {
        if self.first {
            self.first = false;
            match event {
                Event::Atom(Atom::F32(value)) => return next.emit(value.to_string().into()),
                Event::Atom(Atom::F64(value)) => return next.emit(value.to_string().into()),
                Event::Atom(Atom::Bytes(value)) => {
                    next.state_mut().get_mut::<PartBytes>().0 = Some(value.to_vec());
                    return next.emit(Event::Atom(Atom::Null));
                }
                _ => {}
            }
        }
        next.emit(event)
    }
}

fn part(value: SerializeRef<'_>) -> Result<Part, Error> {
    let mut driver = SerializeDriver::from_ref(value);
    // Scalar floats use Display and scalar bytes remain raw, as with Serde.
    driver.push_layer(ScalarLayer {
        first: true,
    });
    let config = deser_json::SerializerConfig::builder()
        .context(Context::with(BytesFormat::SEQ))
        .build();
    let mut serializer = deser_json::Serializer::with_config(config);
    Serializer::drive(&mut serializer, &mut driver)?;
    if let Some(bytes) = driver.state_mut().get_mut::<PartBytes>().0.take() {
        return Ok(Part::bytes(bytes));
    }
    let json = serializer.finish();
    // Telegram expects scalar strings as plain text and compounds as JSON.
    if json.starts_with('"') {
        Ok(Part::text(deser_json::from_str::<String>(&json)?))
    } else {
        Ok(Part::text(json))
    }
}
