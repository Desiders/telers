//! Combinations of the arguments of `#[derive(FromContext)]`, checked at runtime

use telers::{
    client::Reqwest,
    context::Context,
    types::{ChatPrivate, MessageText, Update, UpdateMessage},
    Bot, Extensions, Extractor, Request,
};
use telers_macros::FromContext;

use std::{
    future::Future,
    pin::pin,
    sync::Arc,
    task::{Context as TaskContext, Poll, Waker},
};

// 1. Struct by key
#[derive(Clone, Debug, PartialEq, FromContext)]
#[context(key = "by_key")]
struct P01 {
    field: i32,
}

// 2. Struct by key with a description, which is a part of the extraction error
#[derive(Clone, Debug, PartialEq, FromContext)]
#[context(key = "with_description", description = "the described value")]
struct P02(i32);

// 3. Unit struct
#[derive(Clone, Debug, PartialEq, FromContext)]
#[context(key = "unit")]
struct P03;

// 4. Enum by key
#[derive(Clone, Debug, PartialEq, FromContext)]
#[context(key = "by_key_enum")]
enum P04 {
    Variant(i32),
}

// 5. Struct converted into a wrapper, the `Extractor` is implemented for the wrapper
#[derive(Clone, Debug, PartialEq, FromContext)]
#[context(key = "into", into = P05Wrapper)]
struct P05(i32);

#[derive(Debug, PartialEq)]
struct P05Wrapper(P05);

impl From<P05> for P05Wrapper {
    fn from(val: P05) -> Self {
        Self(val)
    }
}

// 6. Wrapper converted from the type in the context
#[derive(Clone, Debug, PartialEq, FromContext)]
#[context(key = "from", from = P05)]
struct P06(P05);

impl From<P05> for P06 {
    fn from(val: P05) -> Self {
        Self(val)
    }
}

// 7. Enum converted into a wrapper with a description
#[derive(Clone, Debug, PartialEq, FromContext)]
#[context(key = "into_enum", into = P07Wrapper, description = "the enum")]
enum P07 {
    Variant,
}

#[derive(Debug, PartialEq)]
struct P07Wrapper(P07);

impl From<P07> for P07Wrapper {
    fn from(val: P07) -> Self {
        Self(val)
    }
}

// 8. Generic struct with a where clause converted into a generic wrapper
#[derive(Clone, Debug, PartialEq, FromContext)]
#[context(key = "generic_into", into = P08Wrapper)]
struct P08<T>
where
    T: Clone,
{
    field: T,
}

#[derive(Debug, PartialEq)]
struct P08Wrapper<T>(P08<T>)
where
    T: Clone;

impl<T> From<P08<T>> for P08Wrapper<T>
where
    T: Clone,
{
    fn from(val: P08<T>) -> Self {
        Self(val)
    }
}

// 9. Arguments in another order with a trailing comma
#[derive(Clone, Debug, PartialEq, FromContext)]
#[context(description = "reordered", key = "reordered",)]
struct P09;

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
    let mut context = Context::new();
    context.insert("by_key", P01 { field: 1 });
    context.insert("with_description", P02(2));
    context.insert("unit", P03);
    context.insert("by_key_enum", P04::Variant(4));
    context.insert("into", P05(5));
    context.insert("from", P05(6));
    context.insert("into_enum", P07::Variant);
    context.insert("generic_into", P08 { field: 8_i32 });
    context.insert("reordered", P09);
    let request = request_with_context(context);

    assert_eq!(extract::<P01>(&request).unwrap(), P01 { field: 1 });
    assert_eq!(extract::<P02>(&request).unwrap(), P02(2));
    assert_eq!(extract::<P03>(&request).unwrap(), P03);
    assert_eq!(extract::<P04>(&request).unwrap(), P04::Variant(4));
    assert_eq!(extract::<P05Wrapper>(&request).unwrap(), P05Wrapper(P05(5)));
    assert_eq!(extract::<P06>(&request).unwrap(), P06(P05(6)));
    assert_eq!(
        extract::<P07Wrapper>(&request).unwrap(),
        P07Wrapper(P07::Variant)
    );
    assert_eq!(
        extract::<P08Wrapper<i32>>(&request).unwrap(),
        P08Wrapper(P08 { field: 8 })
    );
    assert_eq!(extract::<P09>(&request).unwrap(), P09);

    // The key, the type and the description are a part of the error of a missing value
    let request = request_with_context(Context::new());
    let err = extract::<P02>(&request).unwrap_err().to_string();
    assert!(err.contains("`with_description`"), "{err}");
    assert!(err.contains("`P02`"), "{err}");
    assert!(err.contains("the described value"), "{err}");
    let err = extract::<P06>(&request).unwrap_err().to_string();
    assert!(err.contains("`P05`"), "{err}");
    assert!(err.contains("no description"), "{err}");
}
