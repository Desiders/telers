pub mod base;
pub mod memory;
pub mod redis;

pub use self::redis::{
    DefaultKeyBuilder, Redis as RedisStorage, StorageError as RedisStorageError,
};
pub use base::{Storage, StorageKey};