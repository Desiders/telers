//! Locks FSM state per key so two updates of one key cannot interleave.

use crate::fsm::storage::base::StorageKey;

use futures_util::future::BoxFuture;
use std::{
    collections::HashMap,
    fmt::{self, Debug, Formatter},
    sync::{Arc, Mutex, PoisonError},
};

/// Keeps the lock held until it is dropped.
pub type IsolationGuard = Box<dyn Send>;

/// Locks a state key for the duration of an update.
///
/// This is the same shape as aiogram's `BaseEventIsolation`: [`DisabledEventIsolation`] does
/// nothing, [`SimpleEventIsolation`] locks per key in the current process, and a Redis-backed
/// implementation only has to be added here.
pub trait EventIsolation: Send + Sync + 'static {
    /// Locks `key`, returning a guard that holds it until dropped
    fn lock(&self, key: &StorageKey) -> BoxFuture<'_, IsolationGuard>;
}

/// Isolation that locks nothing, which is the default.
#[derive(Debug, Default, Clone, Copy)]
pub struct DisabledEventIsolation;

impl EventIsolation for DisabledEventIsolation {
    fn lock(&self, _key: &StorageKey) -> BoxFuture<'_, IsolationGuard> {
        Box::pin(async { Box::new(()) as IsolationGuard })
    }
}

/// Isolation by one lock per state key, shared by every clone.
#[derive(Clone, Default)]
pub struct SimpleEventIsolation {
    locks: Arc<Mutex<HashMap<StorageKey, Arc<tokio::sync::Mutex<()>>>>>,
}

impl SimpleEventIsolation {
    /// Creates isolation that has not locked any key yet
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl Debug for SimpleEventIsolation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("SimpleEventIsolation")
            .finish_non_exhaustive()
    }
}

impl EventIsolation for SimpleEventIsolation {
    fn lock(&self, key: &StorageKey) -> BoxFuture<'_, IsolationGuard> {
        let lock = Arc::clone(
            self.locks
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .entry(key.clone())
                .or_default(),
        );

        Box::pin(async move { Box::new(lock.lock_owned().await) as IsolationGuard })
    }
}
