use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Either<T, U> {
    Left(T),
    Right(U),
}
