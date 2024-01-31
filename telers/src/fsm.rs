//! This module contains the implementations of the finite state machine.
//!
//! Finite state machine (FSM) used to represent and control execution flow,
//! which is a set of states and transitions between them.
//!
//! Components of the FSM:
//! * [`Strategy`]:
//! Strategy is a rules to determine necessary data to identify the object (user, chat, etc.) as a key in the storage.
//! For example, if you use [`UserInChat`] strategy, then the user will have the same data and state in all chats,
//! but if you use [`Chat`] strategy, then the user will have different data and state in each chat.
//! You may also need to distinguish state between different threads in the chat, then you can use [`UserInThread`] strategy
//! or use one state in any chats and threads for user using [`GlobalUser`] strategy.
//! The default strategy is [`UserInChat`] that used in the most cases.
//! * [`Storage`]:
//! Storage is used to store state and data of the user.
//! You can save data in the storage to use it in the next state, for example,
//! to ask the user's first name age, etc. on registration process and save it in the storage in the last state.
//! Be aware, storage is part of the FSM pattern, so don't use it for other purposes like database and store user data not related with state machine.
//! Storage is a trait, so you can implement it for any database or use one of the ready-made implementations.
//! Check out the [`storage module`] for more information about ready-made implementations (`RedisStorage`, `MemoryStorage`, etc.).
//! * [`Context`]:
//! Context is an FSM implementation detail that used to manage state and data of the user in specified storage.
//! [`Storage`] trait accepts [`StorageKey`] as a key of the user/chat/thread in the storage to store state and data for it.
//! But we don't want to use [`StorageKey`] directly in the code for the sake of convenience,
//! so we create [`StorageKey`] in the [`FSMContext middleware`] and pass it to the [`Context`]
//! that wrap the [`StorageKey`] and [`Storage`] to provide more convenient API to work with the storage.
//!
//! You can check example of using FSM in the [`examples/finite_state_machine`].
//!
//! [`UserInChat`]: Strategy::UserInChat
//! [`Chat`]: Strategy::Chat
//! [`GlobalUser`]: Strategy::GlobalUser
//! [`UserInThread`]: Strategy::UserInThread
//! [`storage module`]: storage
//! [`FSMContext middleware`]: crate::middlewares::outer::fsm_context::FSMContext

pub mod context;
pub mod storage;
pub mod strategy;

pub use context::Context;
pub use storage::{Storage, StorageKey};
pub use strategy::Strategy;

#[cfg(feature = "redis-storage")]
pub use storage::{DefaultKeyBuilder as RedisDefaultKeyBuilder, Redis as RedisStorage};

#[cfg(feature = "memory-storage")]
pub use storage::Memory as MemoryStorage;
