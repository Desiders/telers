#[cfg(not(feature = "deser"))]
mod deserialization;

pub mod enums;
pub mod errors;
pub mod serialization;
pub mod types;
