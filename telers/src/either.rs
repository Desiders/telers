//! [`Either`], the type that holds one of two values under a single name.

use crate::serialization::{Deserialize, Serialize};

/// One of two values, used where two alternatives have to travel under one type.
///
/// It serializes and deserializes untagged, it is an [`Extractor`](crate::Extractor) that tries
/// `T` first and falls back to `U`, a pair of filters reports the error of the first one as
/// `Left` and of the second one as `Right`, and it becomes an [`anyhow::Error`] when either
/// variant holds one.
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(not(feature = "deser"), serde(untagged))]
#[cfg_attr(feature = "deser", deser(untagged))]
pub enum Either<T, U> {
    Left(T),
    Right(U),
}

impl<T, U> From<Either<T, U>> for anyhow::Error
where
    T: Into<anyhow::Error>,
    U: Into<anyhow::Error>,
{
    fn from(value: Either<T, U>) -> Self {
        match value {
            Either::Left(err) => err.into(),
            Either::Right(err) => err.into(),
        }
    }
}
