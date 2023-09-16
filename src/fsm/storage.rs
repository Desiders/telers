//! This module contains the storage implementations for the FSM.
//!
//! Storage is used to store state and data of the user.
//! Be aware, storage is part of the FSM pattern, so don't use it for other purposes like database and store user data not related with state machine.
//!
//! Ready-made implementations:
//! * [`Memory`]:
//! In-memory storage implementation.
//! This is a simple thread-safe in-memory storage implementation used for testing purposes usually, because it doesn't persist data between restarts
//! and isn't recommended for production use. (feature: `memory-storage`)
//! * [`Redis`]:
//! Redis storage implementation.
//! This is a thread-safe Redis storage implementation that persists data between restarts.
//! This is the recommended storage implementation for production use. (feature: `redis-storage`)

pub mod base;
#[cfg(feature = "memory-storage")]
pub mod memory;
#[cfg(feature = "redis-storage")]
pub mod redis;

#[cfg(feature = "redis-storage")]
pub use self::redis::{DefaultKeyBuilder, Redis};
#[allow(clippy::module_name_repetitions)]
pub use base::{Error, Storage, StorageKey};
#[cfg(feature = "memory-storage")]
pub use memory::Memory;
