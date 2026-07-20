//! This module contains the storage implementations for the FSM.
//!
//! Storage is used to store state and data of the user.
//! Be aware, the storage is part of the FSM pattern, so don't use it as a database for data not related to the state machine.
//!
//! Ready-made implementations:
//! * Memory (feature: `memory-storage`):
//!   In-memory storage implementation.
//!   This is a simple thread-safe in-memory storage implementation, usually used for testing purposes, because it doesn't persist data between restarts
//!   and isn't recommended for production use.
//! * Redis (feature: `redis-storage`):
//!   Redis storage implementation.
//!   This is a thread-safe Redis storage implementation that persists data between restarts.
//!   This is the recommended storage implementation for production use.

pub mod base;
#[cfg(feature = "memory-storage")]
pub mod memory;
#[cfg(feature = "redis-storage")]
pub mod redis;

#[cfg(feature = "redis-storage")]
pub use self::redis::{KeyBuilderImpl, Redis};
#[allow(clippy::module_name_repetitions)]
pub use base::{Error, Storage, StorageKey};
#[cfg(feature = "memory-storage")]
pub use memory::Memory;
