//! [`Extension`] is a type that is used to transmit data between processing-units when propagating an event.
//! [`Extensions`] creates at the start of the event propagation by [`Dispatcher`] and pass to every processing-unit.
//! Processing-units can add their own data to extension and use data from extension that was added by others.
//!
//! Modify extensions in outer middlewares if you need to pass some data to next outer/inner middlewares or to filters.
//! Usually data for handlers is passed by inner middlewares, but you can use outer middlewares for this too.
//! Check [`outer middleware module`] documentation for more information (**recommended**).
//!
//! Modify context in inner middlewares if you need to pass some data to next inner middlewares or to handler.
//! Check [`inner middleware module`] documentation for more information (**recommended**).
//!
//! In [`Handler`] extension can be passed as parameter of handler function.
//! You can use extension in handlers to get data that was added by middlewares.
//! You don't need to implement [`Extractor`] for your own types, because it's already done in case of extension.
//! Check [`extractors module`] documentation for more information (**recommended**).
//!
//! [`Dispatcher`]: telers::Dispatcher
//! [`OuterMiddleware`]: telers::middlewares::OuterMiddleware
//! [`InnerMiddleware`]: telers::middlewares::InnerMiddleware
//! [`Handler`]: telers::event::telegram::Handler
//! [`Extractor`]: telers::Extractor
//! [`outer middleware module`]: telers::middlewares::outer
//! [`inner middleware module`]: telers::middlewares::inner
//! [`filter module`]: telers::filters
//! [`extractors module`]: telers::extractor

use crate::any::AnyClone;

use std::{
    any::TypeId,
    collections::HashMap,
    fmt,
    hash::{BuildHasherDefault, Hasher},
};

type AnyMap = HashMap<TypeId, Box<dyn AnyClone + Send + Sync>, BuildHasherDefault<IdHasher>>;

#[derive(Debug, Clone, Copy, Default)]
#[must_use]
pub struct Extension<T>(pub T);

#[derive(Clone, Default)]
pub struct Extensions {
    map: Option<Box<AnyMap>>,
}

impl Extensions {
    #[must_use]
    pub const fn new() -> Self {
        Self { map: None }
    }

    /// Insert a type into this [`Extensions`].
    /// If the map did not have this key present, [`None`] is returned.
    /// If a extension of this type already existed, it will be returned and replaced with the new one.
    /// # Examples
    /// ```
    /// let mut ext = Extensions::new();
    /// assert!(ext.insert(5i32).is_none());
    /// assert!(ext.insert(4u8).is_none());
    /// assert_eq!(ext.insert(9i32), Some(5i32));
    /// ```
    pub fn insert<T: Clone + Send + Sync + 'static>(&mut self, val: T) -> Option<T> {
        self.map
            .get_or_insert_with(Box::default)
            .insert(TypeId::of::<T>(), Box::new(val))
            .and_then(|boxed| boxed.into_any().downcast().ok().map(|boxed| *boxed))
    }

    /// Get a reference to a type previously inserted on this [`Extensions`]
    /// # Examples
    /// ```
    /// let mut ext = Extensions::new();
    /// assert!(ext.get::<i32>().is_none());
    /// ext.insert(5i32);
    ///
    /// assert_eq!(ext.get::<i32>(), Some(&5i32));
    /// ```
    #[must_use]
    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.map
            .as_ref()
            .and_then(|map| map.get(&TypeId::of::<T>()))
            .and_then(|boxed| (**boxed).as_any().downcast_ref())
    }

    /// Get a mutable reference to a type previously inserted on this [`Extensions`]
    /// # Examples
    /// ```
    /// let mut ext = Extensions::new();
    /// ext.insert(String::from("Hello"));
    /// ext.get_mut::<String>().unwrap().push_str(" World");
    ///
    /// assert_eq!(ext.get::<String>().unwrap(), "Hello World");
    /// ```
    pub fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.map
            .as_mut()
            .and_then(|map| map.get_mut(&TypeId::of::<T>()))
            .and_then(|boxed| (**boxed).as_any_mut().downcast_mut())
    }

    /// Get a mutable reference to a type, inserting the value created by `f` if not already present on this [`Extensions`]
    /// # Examples
    /// ```
    /// let mut ext = Extensions::new();
    /// *ext.get_or_insert_with(|| 1i32) += 2;
    ///
    /// assert_eq!(*ext.get::<i32>().unwrap(), 3);
    /// ```
    #[allow(clippy::missing_panics_doc)]
    pub fn get_or_insert_with<T: Clone + Send + Sync + 'static, F: FnOnce() -> T>(
        &mut self,
        f: F,
    ) -> &mut T {
        let out = self
            .map
            .get_or_insert_with(Box::default)
            .entry(TypeId::of::<T>())
            .or_insert_with(|| Box::new(f()));
        (**out).as_any_mut().downcast_mut().unwrap()
    }

    /// Get a mutable reference to a type, inserting `value` if not already present on this [`Extensions`]
    /// # Examples
    /// ```
    /// let mut ext = Extensions::new();
    /// *ext.get_or_insert(1i32) += 2;
    ///
    /// assert_eq!(*ext.get::<i32>().unwrap(), 3);
    /// ```
    pub fn get_or_insert<T: Clone + Send + Sync + 'static>(&mut self, value: T) -> &mut T {
        self.get_or_insert_with(|| value)
    }

    /// Get a mutable reference to a type, inserting the type's default value if not already present on this [`Extensions`]
    /// # Examples
    /// ```
    /// let mut ext = Extensions::new();
    /// *ext.get_or_insert_default::<i32>() += 2;
    ///
    /// assert_eq!(*ext.get::<i32>().unwrap(), 2);
    /// ```
    pub fn get_or_insert_default<T: Default + Clone + Send + Sync + 'static>(&mut self) -> &mut T {
        self.get_or_insert_with(T::default)
    }

    /// Remove a type from this [`Extensions`].
    /// If a extension of this type existed, it will be returned.
    /// # Examples
    /// ```
    /// let mut ext = Extensions::new();
    /// ext.insert(5i32);
    /// assert_eq!(ext.remove::<i32>(), Some(5i32));
    /// assert!(ext.get::<i32>().is_none());
    /// ```
    pub fn remove<T: 'static>(&mut self) -> Option<T> {
        self.map
            .as_mut()
            .and_then(|map| map.remove(&TypeId::of::<T>()))
            .and_then(|boxed| boxed.into_any().downcast().ok().map(|boxed| *boxed))
    }

    /// Clear the [`Extensions`] of all inserted extensions
    /// # Examples
    /// ```
    /// let mut ext = Extensions::new();
    /// ext.insert(5i32);
    /// ext.clear();
    ///
    /// assert!(ext.get::<i32>().is_none());
    /// ```
    #[inline]
    pub fn clear(&mut self) {
        if let Some(ref mut map) = self.map {
            map.clear();
        }
    }

    /// Check whether the extension set is empty or not
    /// # Examples
    /// ```
    /// let mut ext = Extensions::new();
    /// assert!(ext.is_empty());
    /// ext.insert(5i32);
    /// assert!(!ext.is_empty());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.map.as_ref().map_or(true, |map| map.is_empty())
    }

    /// Get the number of extensions available
    /// # Examples
    /// ```
    /// let mut ext = Extensions::new();
    /// assert_eq!(ext.len(), 0);
    /// ext.insert(5i32);
    /// assert_eq!(ext.len(), 1);
    /// ```
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.map.as_ref().map_or(0, |map| map.len())
    }

    /// Extends `self` with another [`Extensions`].
    /// If an instance of a specific type exists in both, the one in `self` is overwritten with the one from `other`.
    /// # Examples
    /// ```
    /// let mut ext_a = Extensions::new();
    /// ext_a.insert(8u8);
    /// ext_a.insert(16u16);
    ///
    /// let mut ext_b = Extensions::new();
    /// ext_b.insert(4u8);
    /// ext_b.insert("hello");
    ///
    /// ext_a.extend(ext_b);
    /// assert_eq!(ext_a.len(), 3);
    /// assert_eq!(ext_a.get::<u8>(), Some(&4u8));
    /// assert_eq!(ext_a.get::<u16>(), Some(&16u16));
    /// assert_eq!(ext_a.get::<&'static str>().copied(), Some("hello"));
    /// ```
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

impl fmt::Debug for Extensions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Extensions").finish()
    }
}

/// With [`TypeId`]s as keys, there's no need to hash them. They are already hashes
/// themselves, coming from the compiler. The [`IdHasher`] just holds the u64 of
/// the [`TypeId`], and then returns it, instead of doing any bit fiddling.
#[derive(Default)]
struct IdHasher(u64);

impl Hasher for IdHasher {
    fn write(&mut self, _: &[u8]) {
        unreachable!("TypeId calls write_u64");
    }

    #[inline]
    fn write_u64(&mut self, id: u64) {
        self.0 = id;
    }

    #[inline]
    fn finish(&self) -> u64 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::Extensions;

    #[test]
    fn extensions() {
        #[derive(Clone, Debug, PartialEq)]
        struct MyType(i32);

        let mut extensions = Extensions::new();

        extensions.insert(5i32);
        extensions.insert(MyType(10));

        assert_eq!(extensions.get(), Some(&5i32));
        assert_eq!(extensions.get_mut(), Some(&mut 5i32));

        let ext2 = extensions.clone();

        assert_eq!(extensions.remove::<i32>(), Some(5i32));
        assert!(extensions.get::<i32>().is_none());

        // clone still has it
        assert_eq!(ext2.get(), Some(&5i32));
        assert_eq!(ext2.get(), Some(&MyType(10)));

        assert_eq!(extensions.get::<bool>(), None);
        assert_eq!(extensions.get(), Some(&MyType(10)));
    }
}
