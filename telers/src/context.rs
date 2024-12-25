//! [`Context`] is a type that is used to transmit data between processing-units when propagating an event.
//! Context creates at the start of the event propagation by [`Dispatcher`] and pass to every processing-unit.
//! Processing-units can add their own data to context and use data from context that was added by others.
//!
//! Modify context in outer middlewares if you need to pass some data to next outer/inner middlewares or to filters.
//! Usually data for handlers is passed by inner middlewares, but you can use outer middlewares for this too.
//! Check [`outer middleware module`] documentation for more information (**recommended**).
//!
//! Modify context in inner middlewares if you need to pass some data to next inner middlewares or to handler.
//! Check [`inner middleware module`] documentation for more information (**recommended**).
//!
//! Usually you don't need to change the context in filters, and it's better to use middleware for that, but you can do it.
//! Check [`filter module`] documentation for more information.
//!
//! In [`Handler`] context is can be passed as parameter of handler function.
//! You can use context in handlers to get data that was added by middlewares and filters.
//! For convenience, you can implement [`Extractor`] for your own types and use them as handler arguments,
//! so you don't need to pass context as parameter of handler and extract data from context manually.
//! Check [`extractors module`] documentation for more information (**recommended**).
//!
//! [`Dispatcher`]: telers::Dispatcher
//! [`OuterMiddleware`]: telers::middlewares::OuterMiddleware
//! [`InnerMiddleware`]: telers::middlewares::InnerMiddleware
//! [`Filter::check`]: telers::filters::Filter#method.check
//! [`Handler`]: telers::event::telegram::Handler
//! [`Extractor`]: telers::Extractor
//! [`outer middleware module`]: telers::middlewares::outer
//! [`inner middleware module`]: telers::middlewares::inner
//! [`filter module`]: telers::filters
//! [`extractors module`]: telers::extractor

use crate::any::AnyClone;

use std::{collections::HashMap, fmt};

type AnyMap = HashMap<&'static str, Box<dyn AnyClone + Send + Sync>>;

#[derive(Clone, Default)]
pub struct Context {
    map: Option<Box<AnyMap>>,
}

impl Context {
    #[must_use]
    pub const fn new() -> Self {
        Self { map: None }
    }

    pub fn insert<T: Clone + Send + Sync + 'static>(
        &mut self,
        key: &'static str,
        val: T,
    ) -> Option<T> {
        self.map
            .get_or_insert_with(Box::default)
            .insert(key, Box::new(val))
            .and_then(|boxed| boxed.into_any().downcast().ok().map(|boxed| *boxed))
    }

    #[must_use]
    pub fn get<T: 'static>(&self, key: &'static str) -> Option<&T> {
        self.map
            .as_ref()
            .and_then(|map| map.get(key))
            .and_then(|boxed| (**boxed).as_any().downcast_ref())
    }

    pub fn get_mut<T: 'static>(&mut self, key: &'static str) -> Option<&mut T> {
        self.map
            .as_mut()
            .and_then(|map| map.get_mut(key))
            .and_then(|boxed| (**boxed).as_any_mut().downcast_mut())
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn get_or_insert_with<T: Clone + Send + Sync + 'static, F: FnOnce() -> T>(
        &mut self,
        key: &'static str,
        f: F,
    ) -> &mut T {
        let out = self
            .map
            .get_or_insert_with(Box::default)
            .entry(key)
            .or_insert_with(|| Box::new(f()));
        (**out).as_any_mut().downcast_mut().unwrap()
    }

    pub fn get_or_insert<T: Clone + Send + Sync + 'static>(
        &mut self,
        key: &'static str,
        value: T,
    ) -> &mut T {
        self.get_or_insert_with(key, || value)
    }

    pub fn get_or_insert_default<T: Default + Clone + Send + Sync + 'static>(
        &mut self,
        key: &'static str,
    ) -> &mut T {
        self.get_or_insert_with(key, T::default)
    }

    pub fn remove<T: 'static>(&mut self, key: &'static str) -> Option<T> {
        self.map
            .as_mut()
            .and_then(|map| map.remove(key))
            .and_then(|boxed| boxed.into_any().downcast().ok().map(|boxed| *boxed))
    }

    #[inline]
    pub fn clear(&mut self) {
        if let Some(ref mut map) = self.map {
            map.clear();
        }
    }

    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.map.as_ref().map_or(true, |map| map.is_empty())
    }

    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.map.as_ref().map_or(0, |map| map.len())
    }

    pub fn extend(&mut self, other: Self) {
        if let Some(other) = other.map {
            if let Some(map) = &mut self.map {
                map.extend(*other);
            } else {
                self.map = Some(other);
            }
        }
    }
}

impl fmt::Debug for Context {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Context").finish()
    }
}
