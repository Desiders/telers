//! Combinations of the arguments of `#[derive(FromEvent)]`, checked at runtime

use telers::{
    client::Reqwest,
    context::Context,
    errors::{ConvertToTypeError, ExtractionError},
    types::{ChatPrivate, MessageText, Update, UpdateMessage},
    Bot, Extensions, Extractor, Request,
};
use telers_macros::FromEvent;

use std::{
    convert::Infallible,
    fmt::{self, Display, Formatter},
    future::Future,
    pin::pin,
    sync::Arc,
    task::{Context as TaskContext, Poll, Waker},
};

// 1. Converted from the update infallibly
#[derive(Debug, PartialEq, FromEvent)]
#[event(from = Update)]
struct P01(i64);

impl From<Update> for P01 {
    fn from(update: Update) -> Self {
        Self(update.update_id())
    }
}

// 2. `from` with a description
#[derive(Debug, PartialEq, FromEvent)]
#[event(from = Update, description = "the update id")]
struct P02(i64);

impl From<Update> for P02 {
    fn from(update: Update) -> Self {
        Self(update.update_id())
    }
}

// 3. `try_from` with the default error type
#[derive(Debug, PartialEq, FromEvent)]
#[event(try_from = Update)]
struct P03(i64);

impl TryFrom<Update> for P03 {
    type Error = ConvertToTypeError;

    fn try_from(update: Update) -> Result<Self, Self::Error> {
        match update.update_id() {
            0 => Err(ConvertToTypeError::new("Update", "P03")),
            update_id => Ok(Self(update_id)),
        }
    }
}

// 4. `try_from` with a custom error type
#[derive(Debug, PartialEq, FromEvent)]
#[event(try_from = Update, error = Infallible)]
struct P04(i64);

impl TryFrom<Update> for P04 {
    type Error = Infallible;

    fn try_from(update: Update) -> Result<Self, Self::Error> {
        Ok(Self(update.update_id()))
    }
}

// 5. `try_from` with a custom error type of the user and a description
#[derive(Debug, PartialEq, FromEvent)]
#[event(try_from = Update, error = P05Error, description = "always fails")]
struct P05(i64);

#[derive(Debug, PartialEq)]
struct P05Error;

impl Display for P05Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("P05 error")
    }
}

impl From<P05Error> for ExtractionError {
    fn from(err: P05Error) -> Self {
        Self::new(err.to_string())
    }
}

impl TryFrom<Update> for P05 {
    type Error = P05Error;

    fn try_from(_: Update) -> Result<Self, Self::Error> {
        Err(P05Error)
    }
}

// 6. Enum converted from the update
#[derive(Debug, PartialEq, FromEvent)]
#[event(from = Update)]
enum P06 {
    Message,
    Other,
}

impl From<Update> for P06 {
    fn from(update: Update) -> Self {
        match update {
            Update::Message(_) => Self::Message,
            _ => Self::Other,
        }
    }
}

// 7. Generic struct with a where clause and `try_from`
#[derive(Debug, PartialEq, FromEvent)]
#[event(try_from = Update)]
struct P07<T>
where
    T: From<i64>,
{
    field: T,
}

impl<T> TryFrom<Update> for P07<T>
where
    T: From<i64>,
{
    type Error = ConvertToTypeError;

    fn try_from(update: Update) -> Result<Self, Self::Error> {
        Ok(Self {
            field: update.update_id().into(),
        })
    }
}

// 8. Arguments in another order with a trailing comma
#[derive(Debug, PartialEq, FromEvent)]
#[event(description = "reordered", from = Update,)]
struct P08;

impl From<Update> for P08 {
    fn from(_: Update) -> Self {
        Self
    }
}

fn request_with_update_id(update_id: i64) -> Request {
    Request {
        bot: Bot::default(),
        update: Arc::new(Update::Message(UpdateMessage::new(
            update_id,
            MessageText::new(0, 0, ChatPrivate::new(0), ""),
        ))),
        context: Context::new(),
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
    let request = request_with_update_id(42);

    assert_eq!(extract::<P01>(&request).unwrap(), P01(42));
    assert_eq!(extract::<P02>(&request).unwrap(), P02(42));
    assert_eq!(extract::<P03>(&request).unwrap(), P03(42));
    assert_eq!(extract::<P04>(&request).unwrap(), P04(42));
    assert_eq!(extract::<P05>(&request).unwrap_err(), P05Error);
    assert_eq!(extract::<P06>(&request).unwrap(), P06::Message);
    assert_eq!(extract::<P07<i64>>(&request).unwrap(), P07 { field: 42 });
    assert_eq!(extract::<P08>(&request).unwrap(), P08);

    // The error of the fallible conversion is the error of the extraction
    let request = request_with_update_id(0);
    let err = extract::<P03>(&request).unwrap_err().to_string();
    assert!(err.contains("`Update`"), "{err}");
    assert!(err.contains("`P03`"), "{err}");
}
