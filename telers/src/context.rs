//! [`Context`] is a type that is used to transmit data between processing-units when propagating an event.
//! The context is created at the start of the event propagation by the [`Dispatcher`] and passed to every processing-unit.
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
//! In a [`Handler`] the context can be passed as a parameter of the handler function.
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
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self {
            map: None,
        }
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

    // The final `expect` can't fire: the matched entry is guaranteed to hold a `T` right above.
    #[allow(clippy::missing_panics_doc)]
    pub fn get_or_insert_with<T: Clone + Send + Sync + 'static, F: FnOnce() -> T>(
        &mut self,
        key: &'static str,
        f: F,
    ) -> &mut T {
        use std::collections::hash_map::Entry;

        let out = match self.map.get_or_insert_with(Box::default).entry(key) {
            Entry::Occupied(entry) if (**entry.get()).as_any().is::<T>() => entry.into_mut(),
            Entry::Occupied(mut entry) => {
                entry.insert(Box::new(f()));
                entry.into_mut()
            }
            Entry::Vacant(entry) => entry.insert(Box::new(f())),
        };

        (**out)
            .as_any_mut()
            .downcast_mut()
            .expect("the entry was just ensured to hold a `T`")
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

    pub fn clear(&mut self) {
        if let Some(ref mut map) = self.map {
            map.clear();
        }
    }

    #[must_use]
    pub fn contains_key(&self, key: &'static str) -> bool {
        self.map.as_ref().is_some_and(|map| map.contains_key(key))
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.map.as_ref().is_none_or(|map| map.is_empty())
    }

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

#[cfg(test)]
mod tests {
    use super::Context;

    #[test]
    fn new_is_empty() {
        let ctx = Context::new();

        assert!(ctx.is_empty());
        assert_eq!(ctx.len(), 0);
        assert!(!ctx.contains_key("missing"));
        assert!(ctx.get::<i32>("missing").is_none());
    }

    #[test]
    fn insert_returns_previous_value() {
        let mut ctx = Context::new();

        assert_eq!(ctx.insert("k", 1_i32), None);
        assert_eq!(ctx.len(), 1);
        assert!(!ctx.is_empty());
        assert!(ctx.contains_key("k"));

        assert_eq!(ctx.insert("k", 2_i32), Some(1));
        assert_eq!(ctx.get::<i32>("k"), Some(&2));
        assert_eq!(ctx.len(), 1);
    }

    #[test]
    fn get_respects_type() {
        let mut ctx = Context::new();
        ctx.insert("k", String::from("v"));

        assert_eq!(ctx.get::<String>("k"), Some(&"v".to_owned()));
        // A type mismatch reads as absent rather than panicking.
        assert!(ctx.get::<i32>("k").is_none());
    }

    #[test]
    fn get_mut_allows_in_place_mutation() {
        let mut ctx = Context::new();
        ctx.insert("count", 1_i32);

        *ctx.get_mut::<i32>("count").expect("present") += 41;

        assert_eq!(ctx.get::<i32>("count"), Some(&42));
    }

    #[test]
    fn get_or_insert_with_only_inserts_when_absent() {
        let mut ctx = Context::new();

        assert_eq!(*ctx.get_or_insert_with("k", || 10_i32), 10);
        assert_eq!(*ctx.get_or_insert_with("k", || 999_i32), 10);
    }

    #[test]
    fn get_or_insert_with_replaces_a_value_of_a_different_type() {
        let mut ctx = Context::new();
        ctx.insert("k", String::from("v"));

        // The key already holds a `String`; requesting an `i32` under the same key used to panic on
        // the failed downcast. It must replace the stale value instead.
        assert_eq!(*ctx.get_or_insert_with("k", || 7_i32), 7);
        assert_eq!(ctx.get::<i32>("k"), Some(&7));
        assert!(ctx.get::<String>("k").is_none());

        // The delegating helpers inherit the same behavior.
        ctx.insert("k", String::from("v"));
        assert_eq!(*ctx.get_or_insert("k", 8_i32), 8);
        ctx.insert("k", String::from("v"));
        assert_eq!(*ctx.get_or_insert_default::<i32>("k"), 0);
    }

    #[test]
    fn get_or_insert_with_calls_f_at_most_once() {
        use std::cell::Cell;

        let calls = Cell::new(0);
        let mut ctx = Context::new();

        // Absent -> `f` runs once.
        ctx.get_or_insert_with("k", || {
            calls.set(calls.get() + 1);
            1_i32
        });
        // Present with the right type -> `f` must not run.
        ctx.get_or_insert_with("k", || {
            calls.set(calls.get() + 1);
            2_i32
        });

        assert_eq!(calls.get(), 1);
    }

    #[test]
    fn get_or_insert_variants() {
        let mut ctx = Context::new();

        assert_eq!(*ctx.get_or_insert("a", 5_i32), 5);
        assert_eq!(*ctx.get_or_insert("a", 6_i32), 5);
        assert_eq!(*ctx.get_or_insert_default::<i32>("b"), 0);
        assert_eq!(*ctx.get_or_insert_default::<String>("c"), String::new());
    }

    #[test]
    fn remove_returns_value_once() {
        let mut ctx = Context::new();
        ctx.insert("k", 7_i32);

        assert_eq!(ctx.remove::<i32>("k"), Some(7));
        assert_eq!(ctx.remove::<i32>("k"), None);
        assert!(!ctx.contains_key("k"));
    }

    #[test]
    fn clear_empties_the_context() {
        let mut ctx = Context::new();
        ctx.insert("a", 1_i32);
        ctx.insert("b", 2_i32);

        ctx.clear();

        assert!(ctx.is_empty());
        assert!(ctx.get::<i32>("a").is_none());
    }

    #[test]
    fn extend_merges_and_overwrites() {
        let mut base = Context::new();
        base.insert("a", 1_i32);
        base.insert("shared", 1_i32);

        let mut other = Context::new();
        other.insert("b", 2_i32);
        other.insert("shared", 2_i32);

        base.extend(other);

        assert_eq!(base.get::<i32>("a"), Some(&1));
        assert_eq!(base.get::<i32>("b"), Some(&2));
        // Keys from the extending context overwrite existing ones.
        assert_eq!(base.get::<i32>("shared"), Some(&2));
    }

    #[test]
    fn extend_into_empty_adopts_other_map() {
        let mut empty = Context::new();
        let mut other = Context::new();
        other.insert("a", 1_i32);

        empty.extend(other);
        assert_eq!(empty.get::<i32>("a"), Some(&1));

        empty.extend(Context::new());
        assert_eq!(empty.len(), 1);
    }
}
