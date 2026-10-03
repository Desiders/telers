use crate::serialization::{Deserialize, Serialize};

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
