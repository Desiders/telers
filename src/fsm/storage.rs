pub mod base;
pub mod memory;
pub mod redis;

pub use self::redis::{DefaultKeyBuilder, Redis, StorageError as RedisStorageError};
#[allow(clippy::module_name_repetitions)]
pub use base::{Storage, StorageKey};
pub use memory::Memory;
