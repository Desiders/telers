use std::{future::Future, pin::Pin};

/// Boxed `Send` future used by async widget and dialog trait methods.
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;
