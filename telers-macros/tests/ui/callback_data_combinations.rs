//! Combinations of the arguments and the fields of `#[derive(CallbackData)]`, checked at runtime

use telers::{
    client::Reqwest,
    context::Context,
    types::{ChatPrivate, MessageText, Update, UpdateMessage},
    utils::callback_data::{CallbackData as _, CallbackDataValue},
    Bot, Extensions, Extractor, Request,
};
use telers_macros::CallbackData;

use std::{
    future::Future,
    pin::pin,
    sync::Arc,
    task::{Context as TaskContext, Poll, Waker},
};

// 1. One string field with the default separator
#[derive(CallbackData, Clone, Debug, PartialEq)]
#[callback_data(prefix = "one")]
struct P01 {
    value: String,
}

// 2. Every kind of value: `bool`, integers, `Box<str>` and `Option`
#[derive(CallbackData, Clone, Debug, PartialEq)]
#[callback_data(prefix = "kinds")]
struct P02 {
    flag: bool,
    count: u8,
    id: i64,
    text: Box<str>,
    maybe: Option<u32>,
}

// 3. Custom separator
#[derive(CallbackData, Clone, Debug, PartialEq)]
#[callback_data(prefix = "sep", separator = '|')]
struct P03 {
    a: String,
    b: String,
}

// 4. Arguments in another order with a trailing comma
#[derive(CallbackData, Clone, Debug, PartialEq)]
#[callback_data(separator = ';', prefix = "reordered",)]
struct P04 {
    value: u8,
}

// 5. Generic struct
#[derive(CallbackData, Clone, Debug, PartialEq)]
#[callback_data(prefix = "generic")]
struct P05<T>
where
    T: CallbackDataValue + Clone,
{
    value: T,
}

fn request_with_context(context: Context) -> Request {
    Request {
        bot: Bot::default(),
        update: Arc::new(Update::Message(UpdateMessage::new(
            0,
            MessageText::new(0, 0, ChatPrivate::new(0), ""),
        ))),
        context,
        extensions: Extensions::default(),
    }
}

/// The extraction of the derived types is immediate, so one poll is enough
fn extract<T: Extractor<Reqwest>>(request: &Request) -> Result<T, T::Error> {
    let mut fut = pin!(T::extract(request));
    match fut.as_mut().poll(&mut TaskContext::from_waker(Waker::noop())) {
        Poll::Ready(res) => res,
        Poll::Pending => panic!("the extraction must be immediate"),
    }
}

fn main() {
    // 1
    let one = P01 {
        value: "value".to_owned(),
    };
    assert_eq!(P01::PREFIX, "one");
    assert_eq!(P01::SEPARATOR, ':');
    assert_eq!(one.pack().unwrap(), "one:value");
    assert_eq!(P01::unpack("one:value").unwrap(), one);

    // 2
    let kinds = P02 {
        flag: true,
        count: 7,
        id: -3,
        text: "text".into(),
        maybe: None,
    };
    assert_eq!(kinds.pack().unwrap(), "kinds:1:7:-3:text:");
    assert_eq!(P02::unpack("kinds:1:7:-3:text:").unwrap(), kinds);
    let kinds = P02 {
        maybe: Some(5),
        ..kinds
    };
    assert_eq!(kinds.pack().unwrap(), "kinds:1:7:-3:text:5");
    assert_eq!(P02::unpack("kinds:1:7:-3:text:5").unwrap(), kinds);

    // 3
    let sep = P03 {
        a: "x,y".to_owned(),
        b: "b".to_owned(),
    };
    assert_eq!(P03::SEPARATOR, '|');
    assert_eq!(sep.pack().unwrap(), "sep|x,y|b");
    assert_eq!(P03::unpack("sep|x,y|b").unwrap(), sep);
    // The values can't contain the separator, and the default one is rejected by the values
    // themselves regardless of the custom one
    for a in ["x|y", "x:y"] {
        let with_separator = P03 {
            a: a.to_owned(),
            b: "b".to_owned(),
        };
        assert!(with_separator.pack().is_err(), "{a}");
    }

    // 4
    let reordered = P04 { value: 1 };
    assert_eq!(reordered.pack().unwrap(), "reordered;1");
    assert_eq!(P04::unpack("reordered;1").unwrap(), reordered);

    // 5
    let generic = P05 { value: 1_i32 };
    assert_eq!(generic.pack().unwrap(), "generic:1");
    assert_eq!(P05::<i32>::unpack("generic:1").unwrap(), generic);

    // 6. Unpacking errors: wrong prefix, wrong number of values, wrong value type, too long data
    assert!(P01::unpack("other:value").is_err());
    assert!(P01::unpack("one:a:b").is_err());
    assert!(P02::unpack("kinds:x:7:-3:text:").is_err());
    let too_long = P01 {
        value: "x".repeat(64),
    };
    assert!(too_long.pack().is_err());

    // 7. Extraction from the context by the `callback_data` key
    let mut context = Context::new();
    context.insert("callback_data", one.clone());
    let request = request_with_context(context);
    assert_eq!(extract::<P01>(&request).unwrap(), one);
    let request = request_with_context(Context::new());
    let err = extract::<P01>(&request).unwrap_err().to_string();
    assert!(err.contains("`callback_data`"), "{err}");
    assert!(err.contains("`P01`"), "{err}");
}
