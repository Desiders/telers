pub mod base;
#[cfg(feature = "memory-storage")]
pub mod memory;
#[cfg(feature = "redis-storage")]
pub mod redis;

#[cfg(feature = "redis-storage")]
pub use self::redis::{DefaultKeyBuilder, Redis, StorageError as RedisStorageError};
#[allow(clippy::module_name_repetitions)]
pub use base::{Storage, StorageKey};
#[cfg(feature = "memory-storage")]
pub use memory::Memory;
