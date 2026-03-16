//! Smart filter core utilities.
//!
//! This module powers the generated smart filters in [`crate::filters::smart`].
//! The idea is simple: build a *path* into an [`Update`], then attach a *check*
//! (predicate) to that path.
//!
//! # How to use smart filters
//!
//! You typically start from [`crate::filters::smart::SmartFilter`], then
//! compose paths and checks:
//!
//! ```rust
//! use telers::filters::smart::SmartFilter;
//!
//! // Match updates that contain a text message with "hello".
//! let _filter = SmartFilter::update()
//!     .message()
//!     .text()
//!     .contains("hello");
//! ```
//!
//! Optional fields return a path that can be absent. Use `is_some`, `is_none`,
//! or a predicate-based check to handle that case:
//!
//! ```rust
//! use telers::filters::smart::SmartFilter;
//!
//! // Check that an optional field is present.
//! let _filter = SmartFilter::update()
//!     .message()
//!     .reply_to_message()
//!     .is_some();
//! ```
//!
//! # Path composition
//!
//! - [`SmartFilterPath`] is for borrowed data (`&T`).
//! - [`SmartFilterOwnedPath`] is for owned data (`T`), useful when you need to
//!   move values into async work or closure captures.
//!
//! Use `map` for *non-optional* accessors and `and_then` for *optional*
//! accessors. The generated code picks the correct one for you, but when
//! writing custom paths this rule matters.
//!
//! # Combining checks
//!
//! Use `.all()` or `.any()` to combine multiple checks on the same path:
//!
//! ```rust
//! use telers::filters::smart::SmartFilter;
//!
//! let _filter = SmartFilter::update()
//!     .message()
//!     .all()
//!     .branch(|m| m.text().contains("hello"))
//!     .branch(|m| m.chat().is_some());
//! ```
//!
//! If you only need a single predicate, call `matches` or `matches_async` directly.
//!
//! # Check methods cheat sheet
//!
//! These checks are available on smart filter paths (depending on the value type):
//!
//! Presence:
//! - `is_some()` and `is_none()` for optional paths.
//!
//! Equality and ordering:
//! - `eq(val)` and `ne(val)` for any `T: PartialEq`.
//! - `gt/lt/gte/lte(val)` for any `T: PartialOrd`.
//!
//! Booleans:
//! - `is_true()` and `is_false()` on `SmartFilterPath<bool>`.
//!
//! Strings and slices:
//! - `len()` and `is_empty()` on `str`, `String`, `Box<str>`, `[T]`, `Vec<T>`, `Box<[T]>`.
//! - `starts_with`, `ends_with`, `is_uppercase`, `is_lowercase`, `contains` on `str`/`String`/`Box<str>`.
//! - `contains(val)` on slices (`[T]`, `Vec<T>`, `Box<[T]>`) where `T: PartialEq`.
//!
//! Custom predicates:
//! - `matches(|v| ...)` and `matches_async(|v| async { ... })` for arbitrary logic.
#![allow(clippy::type_complexity)]

use crate::{types::Update, Filter, FilterResult, Request};

use std::{convert::Infallible, future::Future, pin::Pin, sync::Arc};

/// Accessor for borrowed smart filter paths.
type Accessor<T> = Arc<dyn for<'a> Fn(&'a Update) -> Option<&'a T> + Send + Sync>;
/// Accessor for owned smart filter paths.
type OwnedAccessor<T> = Arc<dyn Fn(&Update) -> Option<T> + Send + Sync>;

/// Matching strategy for borrowed smart filters.
pub enum SmartFilterMode<T: ?Sized> {
    /// Matches when the accessor returns `Some`.
    IsSome,
    /// Matches when the accessor returns `None`.
    IsNone,
    /// Matches when the predicate returns `true`.
    Predicate(Arc<dyn Fn(&T) -> bool + Send + Sync>),
    /// Matches when the async predicate resolves to `true`.
    AsyncPredicate(
        Arc<dyn for<'a> Fn(&'a T) -> Pin<Box<dyn Future<Output = bool> + Send + 'a>> + Send + Sync>,
    ),
}

impl<T: ?Sized> Clone for SmartFilterMode<T> {
    fn clone(&self) -> Self {
        match self {
            Self::IsSome => Self::IsSome,
            Self::IsNone => Self::IsNone,
            Self::Predicate(val) => Self::Predicate(val.clone()),
            Self::AsyncPredicate(val) => Self::AsyncPredicate(val.clone()),
        }
    }
}

/// Matching strategy for owned smart filters.
pub enum SmartFilterOwnedMode<T> {
    /// Matches when the accessor returns `Some`.
    IsSome,
    /// Matches when the accessor returns `None`.
    IsNone,
    /// Matches when the predicate returns `true`.
    Predicate(Arc<dyn Fn(T) -> bool + Send + Sync>),
    /// Matches when the async predicate resolves to `true`.
    AsyncPredicate(Arc<dyn Fn(T) -> Pin<Box<dyn Future<Output = bool> + Send>> + Send + Sync>),
}

impl<T> Clone for SmartFilterOwnedMode<T> {
    fn clone(&self) -> Self {
        match self {
            Self::IsSome => Self::IsSome,
            Self::IsNone => Self::IsNone,
            Self::Predicate(val) => Self::Predicate(val.clone()),
            Self::AsyncPredicate(val) => Self::AsyncPredicate(val.clone()),
        }
    }
}

/// Filter wrapper for borrowed smart filters.
pub struct SmartFilterCheck<T: ?Sized> {
    accessor: Accessor<T>,
    mode: SmartFilterMode<T>,
}

impl<T: ?Sized> Clone for SmartFilterCheck<T> {
    fn clone(&self) -> Self {
        Self {
            accessor: self.accessor.clone(),
            mode: self.mode.clone(),
        }
    }
}

impl<Client, T> Filter<Client> for SmartFilterCheck<T>
where
    Client: Send + Sync + 'static,
    T: ?Sized + Send + Sync + 'static,
{
    type Error = Infallible;

    fn check(
        &mut self,
        request: &mut Request<Client>,
    ) -> impl Future<Output = FilterResult<Self::Error>> + Send {
        let val = (self.accessor)(request.update.as_ref());
        let mode = self.mode.clone();

        async move {
            if let SmartFilterMode::AsyncPredicate(f) = mode {
                match val {
                    Some(inner) => Ok(f(inner).await),
                    None => Ok(false),
                }
            } else {
                let result = match mode {
                    SmartFilterMode::IsSome => val.is_some(),
                    SmartFilterMode::IsNone => val.is_none(),
                    SmartFilterMode::Predicate(f) => val.is_some_and(|inner| f(inner)),
                    SmartFilterMode::AsyncPredicate(_) => unreachable!(),
                };
                Ok(result)
            }
        }
    }
}

/// Filter wrapper for owned smart filters.
pub struct SmartFilterOwnedCheck<T> {
    accessor: OwnedAccessor<T>,
    mode: SmartFilterOwnedMode<T>,
}

impl<T> Clone for SmartFilterOwnedCheck<T> {
    fn clone(&self) -> Self {
        Self {
            accessor: self.accessor.clone(),
            mode: self.mode.clone(),
        }
    }
}

impl<Client, T> Filter<Client> for SmartFilterOwnedCheck<T>
where
    Client: Send + Sync + 'static,
    T: Clone + Send + Sync + 'static,
{
    type Error = Infallible;

    fn check(
        &mut self,
        request: &mut Request<Client>,
    ) -> impl Future<Output = FilterResult<Self::Error>> + Send {
        let val = (self.accessor)(request.update.as_ref());
        let mode = self.mode.clone();

        async move {
            if let SmartFilterOwnedMode::AsyncPredicate(f) = mode {
                match val {
                    Some(inner) => Ok(f(inner).await),
                    None => Ok(false),
                }
            } else {
                let result = match mode {
                    SmartFilterOwnedMode::IsSome => val.is_some(),
                    SmartFilterOwnedMode::IsNone => val.is_none(),
                    SmartFilterOwnedMode::Predicate(f) => val.is_some_and(|inner| f(inner)),
                    SmartFilterOwnedMode::AsyncPredicate(_) => unreachable!(),
                };
                Ok(result)
            }
        }
    }
}

/// A borrowed smart filter path that can be composed into deeper accessors.
pub struct SmartFilterPath<T: ?Sized> {
    pub(crate) accessor: Accessor<T>,
}

impl<T: ?Sized> Clone for SmartFilterPath<T> {
    fn clone(&self) -> Self {
        Self {
            accessor: self.accessor.clone(),
        }
    }
}

/// An owned smart filter path that can be composed into deeper accessors.
pub struct SmartFilterOwnedPath<T> {
    accessor: OwnedAccessor<T>,
}

impl<T> Clone for SmartFilterOwnedPath<T> {
    fn clone(&self) -> Self {
        Self {
            accessor: self.accessor.clone(),
        }
    }
}

/// Branch evaluation mode for grouped filters.
#[derive(Clone, Copy)]
pub enum BranchOperator {
    /// All conditions must match.
    All,
    /// Any condition may match.
    Any,
}

macro_rules! define_branch {
    (
        branch = $branch:ident,
        path = $path:ident,
        check = $check:ident,
        mode = $mode:ident,
        accessor = $accessor:ident,
        struct_bounds = { $($s_bounds:tt)* },
        impl_bounds = { $($i_bounds:tt)* },
        get_val = |$val_pat:ident| $get_val:expr,
    ) => {
        pub struct $branch<T: $($s_bounds)*> {
            accessor: $accessor<T>,
            conditions: Vec<$check<T>>,
            operator: BranchOperator,
        }

        impl<T: $($s_bounds)*> Clone for $branch<T> {
            fn clone(&self) -> Self {
                Self {
                    accessor: self.accessor.clone(),
                    conditions: self.conditions.clone(),
                    operator: self.operator,
                }
            }
        }

        impl<T: $($i_bounds)*> $branch<T> {
            #[must_use]
            pub fn branch(mut self, f: impl FnOnce($path<T>) -> $check<T>) -> Self {
                let path = $path { accessor: self.accessor.clone() };
                self.conditions.push(f(path));
                self
            }
        }

        impl<Client, T> Filter<Client> for $branch<T>
        where
            Client: Send + Sync + 'static,
            T: $($i_bounds)*,
        {
            type Error = Infallible;

            fn check(
                &mut self,
                request: &mut Request<Client>,
            ) -> impl Future<Output = FilterResult<Self::Error>> + Send {
                let val = (self.accessor)(request.update.as_ref());
                let conditions = self.conditions.clone();
                let operator = self.operator;

                async move {
                    macro_rules! eval {
                        ($cond:expr) => {{
                            let $val_pat = &val;
                            match $get_val {
                                Some(inner) => match &$cond.mode {
                                    $mode::IsSome => true,
                                    $mode::IsNone => false,
                                    $mode::Predicate(f) => f(inner),
                                    $mode::AsyncPredicate(f) => f(inner).await,
                                },
                                None => false,
                            }
                        }};
                    }
                    match operator {
                        BranchOperator::All => {
                            for cond in conditions { if !eval!(cond) { return Ok(false); } }
                            Ok(true)
                        }
                        BranchOperator::Any => {
                            for cond in conditions { if eval!(cond) { return Ok(true); } }
                            Ok(false)
                        }
                    }
                }
            }
        }
    };
}
define_branch! {
    branch = SmartFilterBranch,
    path = SmartFilterPath,
    check = SmartFilterCheck,
    mode = SmartFilterMode,
    accessor = Accessor,
    struct_bounds = { ?Sized },
    impl_bounds = { ?Sized + Send + Sync + 'static },
    get_val = |v| *v,
}
define_branch! {
    branch = SmartFilterOwnedBranch,
    path = SmartFilterOwnedPath,
    check = SmartFilterOwnedCheck,
    mode = SmartFilterOwnedMode,
    accessor = OwnedAccessor,
    struct_bounds = { },
    impl_bounds = { Clone + Send + Sync + 'static },
    get_val = |v| v.clone(),
}

impl<T: ?Sized + Send + Sync + 'static> SmartFilterPath<T> {
    /// Maps a non-optional accessor to a deeper borrowed path.
    #[must_use]
    pub fn map<U: ?Sized + Send + Sync + 'static>(
        self,
        f: impl for<'a> Fn(&'a T) -> &'a U + Send + Sync + 'static,
    ) -> SmartFilterPath<U> {
        let accessor = self.accessor;
        SmartFilterPath {
            accessor: Arc::new(move |update| accessor(update).map(&f)),
        }
    }

    /// Maps a non-optional accessor to an owned path.
    #[must_use]
    pub fn map_owned<U: Send + Sync + 'static>(
        self,
        f: impl Fn(&T) -> U + Send + Sync + 'static,
    ) -> SmartFilterOwnedPath<U> {
        let accessor = self.accessor;
        SmartFilterOwnedPath {
            accessor: Arc::new(move |update| accessor(update).map(&f)),
        }
    }

    /// Chains an optional accessor to a deeper borrowed path.
    #[must_use]
    pub fn and_then<U: ?Sized + Send + Sync + 'static>(
        self,
        f: impl for<'a> Fn(&'a T) -> Option<&'a U> + Send + Sync + 'static,
    ) -> SmartFilterPath<U> {
        let accessor = self.accessor;
        SmartFilterPath {
            accessor: Arc::new(move |update| accessor(update).and_then(&f)),
        }
    }
}

impl<T: Send + Sync + 'static> SmartFilterOwnedPath<T> {
    /// Maps a non-optional accessor to a deeper owned path.
    #[must_use]
    pub fn map<U: Send + Sync + 'static>(
        self,
        f: impl for<'a> Fn(T) -> U + Send + Sync + 'static,
    ) -> SmartFilterOwnedPath<U> {
        let accessor = self.accessor;
        SmartFilterOwnedPath {
            accessor: Arc::new(move |update| accessor(update).map(&f)),
        }
    }

    /// Chains an optional accessor to a deeper owned path.
    #[must_use]
    pub fn and_then<U: Send + Sync + 'static>(
        self,
        f: impl for<'a> Fn(T) -> Option<U> + Send + Sync + 'static,
    ) -> SmartFilterOwnedPath<U> {
        let accessor = self.accessor;
        SmartFilterOwnedPath {
            accessor: Arc::new(move |update| accessor(update).and_then(&f)),
        }
    }
}

macro_rules! impl_path_base {
    (
        path = $path_ty:ident,
        check = $check_ty:ident,
        mode = $mode_ty:ident,
        branch = $branch_ty:ident,
        predicate_arg = $pred_arg:ty,
        bounds = { $($t_bounds:tt)* },
        branch_bounds = { $($b_bounds:tt)* },
        deref_in_compare = { $($d_bounds:tt)* },
    ) => {
        impl<T> $path_ty<T>
        where
            T: $($t_bounds)*,
        {
            #[must_use]
            pub fn all(self) -> $branch_ty<T>
            where
                T: $($b_bounds)*
            {
                $branch_ty { accessor: self.accessor, conditions: Vec::new(), operator: BranchOperator::All }
            }

            #[must_use]
            pub fn any(self) -> $branch_ty<T>
            where
                T: $($b_bounds)*
            {
                $branch_ty { accessor: self.accessor, conditions: Vec::new(), operator: BranchOperator::Any }
            }

            #[must_use]
            pub fn is_some(self) -> $check_ty<T> {
                $check_ty { accessor: self.accessor, mode: $mode_ty::IsSome }
            }

            #[must_use]
            pub fn is_none(self) -> $check_ty<T> {
                $check_ty { accessor: self.accessor, mode: $mode_ty::IsNone }
            }

            #[must_use]
            pub fn matches(
                self,
                f: impl Fn($pred_arg) -> bool + Send + Sync + 'static,
            ) -> $check_ty<T> {
                $check_ty {
                    accessor: self.accessor,
                    mode: $mode_ty::Predicate(Arc::new(f)),
                }
            }

            #[must_use]
            pub fn matches_async<F, Fut>(self, f: F) -> $check_ty<T>
            where
                F: Fn($pred_arg) -> Fut + Send + Sync + 'static,
                Fut: Future<Output = bool> + Send + 'static,
            {
                $check_ty {
                    accessor: self.accessor,
                    mode: $mode_ty::AsyncPredicate(Arc::new(move |val| Box::pin(f(val)))),
                }
            }

            #[must_use]
            pub fn eq<V>(self, val: V) -> $check_ty<T>
            where
                T: PartialEq<V>,
                V: Send + Sync + 'static,
            {
                self.matches(move |inner| inner.eq(&val))
            }

            #[must_use]
            pub fn ne<V>(self, val: V) -> $check_ty<T>
            where
                T: PartialEq<V>,
                V: Send + Sync + 'static,
            {
                self.matches(move |inner| !inner.eq(&val))
            }

            #[must_use]
            pub fn gt<V>(self, val: V) -> $check_ty<T>
            where
                T: PartialOrd<V>,
                V: Send + Sync + 'static,
            {
                self.matches(move |inner| $($d_bounds)* inner > val)
            }

            #[must_use]
            pub fn lt<V>(self, val: V) -> $check_ty<T>
            where
                T: PartialOrd<V>,
                V: Send + Sync + 'static,
            {
                self.matches(move |inner| $($d_bounds)* inner < val)
            }

            #[must_use]
            pub fn gte<V>(self, val: V) -> $check_ty<T>
            where
                T: PartialOrd<V>,
                V: Send + Sync + 'static,
            {
                self.matches(move |inner| ($($d_bounds)* inner) >= val)
            }

            #[must_use]
            pub fn lte<V>(self, val: V) -> $check_ty<T>
            where
                T: PartialOrd<V>,
                V: Send + Sync + 'static,
            {
                self.matches(move |inner| ($($d_bounds)* inner) <= val)
            }
        }

        impl $path_ty<bool> {
            #[must_use]
            pub fn is_true(self) -> $check_ty<bool> {
                self.matches(|inner| ($($d_bounds)* inner))
            }

            #[must_use]
            pub fn is_false(self) -> $check_ty<bool> {
                self.matches(|inner| !($($d_bounds)* inner))
            }
        }
    };
}
impl_path_base! {
    path = SmartFilterPath,
    check = SmartFilterCheck,
    mode = SmartFilterMode,
    branch = SmartFilterBranch,
    predicate_arg = &T,
    bounds = { ?Sized + Send + Sync + 'static },
    branch_bounds = { },
    deref_in_compare = { * },
}
impl_path_base! {
    path = SmartFilterOwnedPath,
    check = SmartFilterOwnedCheck,
    mode = SmartFilterOwnedMode,
    branch = SmartFilterOwnedBranch,
    predicate_arg = T,
    bounds = { Send + Sync + 'static },
    branch_bounds = { Clone + Send + Sync + 'static },
    deref_in_compare = { },
}

macro_rules! impl_path_len {
    (Owned, $($ty:ty),+) => {
        $(
            impl SmartFilterOwnedPath<$ty> {
                #[must_use]
                pub fn len(self) -> SmartFilterOwnedPath<usize> {
                   self.map(|val| val.len())
                }

                #[must_use]
                pub fn is_empty(self) -> SmartFilterOwnedPath<bool> {
                    self.map(|val| val.is_empty())
                }
            }
        )+
    };
    (Owned, $($ty:ty => [$($generic:ident),+]),+) => (
        $(
            impl<$($generic: Send + Sync + 'static),+> SmartFilterOwnedPath<$ty> {
                #[must_use]
                pub fn len(self) -> SmartFilterOwnedPath<usize> {
                   self.map(|val| val.len())
                }

                #[must_use]
                pub fn is_empty(self) -> SmartFilterOwnedPath<bool> {
                    self.map(|val| val.is_empty())
                }
            }
        )+
    );
    ($($ty:ty),+) => {
        $(
            impl SmartFilterPath<$ty> {
                #[must_use]
                pub fn len(self) -> SmartFilterOwnedPath<usize> {
                   self.map_owned(<$ty>::len)
                }

                #[must_use]
                pub fn is_empty(self) -> SmartFilterOwnedPath<bool> {
                    self.map_owned(<$ty>::is_empty)
                }
            }
        )+
    };
    ($($ty:ty => [$($generic:ident),+]),+) => (
        $(
            impl<$($generic: Send + Sync + 'static),+> SmartFilterPath<$ty> {
                #[must_use]
                pub fn len(self) -> SmartFilterOwnedPath<usize> {
                   self.map_owned(<$ty>::len)
                }

                #[must_use]
                pub fn is_empty(self) -> SmartFilterOwnedPath<bool> {
                    self.map_owned(<$ty>::is_empty)
                }
            }
        )+
    );
}
impl_path_len!(str);
impl_path_len!([T] => [T]);
impl_path_len!(Owned, String, Box<str>);
impl_path_len!(Owned, Vec<T> => [T], Box<[T]> => [T]);

macro_rules! impl_str_methods {
    ($path_ty:ident, $check_ty:ident, $str_ty:ty) => {
        impl $path_ty<$str_ty> {
            #[must_use]
            pub fn starts_with(self, prefix: impl Into<Box<str>>) -> $check_ty<$str_ty> {
                let prefix = prefix.into();
                self.matches(move |inner| inner.starts_with(prefix.as_ref()))
            }

            #[must_use]
            pub fn ends_with(self, suffix: impl Into<Box<str>>) -> $check_ty<$str_ty> {
                let suffix = suffix.into();
                self.matches(move |inner| inner.ends_with(suffix.as_ref()))
            }

            #[must_use]
            pub fn is_uppercase(self) -> $check_ty<$str_ty> {
                self.matches(|inner| inner.chars().all(char::is_uppercase))
            }

            #[must_use]
            pub fn is_lowercase(self) -> $check_ty<$str_ty> {
                self.matches(|inner| inner.chars().all(char::is_lowercase))
            }
        }
    };
}
impl_str_methods!(SmartFilterPath, SmartFilterCheck, str);
impl_str_methods!(SmartFilterOwnedPath, SmartFilterOwnedCheck, String);
impl_str_methods!(SmartFilterOwnedPath, SmartFilterOwnedCheck, Box<str>);

macro_rules! impl_contains {
    (str, $path_ty:ident, $check_ty:ident, $($ty:ty),+) => {
        $(
            impl $path_ty<$ty> {
                #[must_use]
                pub fn contains(self, pat: impl Into<Box<str>>) -> $check_ty<$ty> {
                    let pat = pat.into();
                    self.matches(move |inner| inner.contains(pat.as_ref()))
                }
            }
        )+
    };
    (slice, $path_ty:ident, $check_ty:ident, $($ty:ty => [$item:ident]),+) => {
        $(
            impl<$item: PartialEq + Send + Sync + 'static> $path_ty<$ty> {
                #[must_use]
                pub fn contains(self, val: $item) -> $check_ty<$ty> {
                    self.matches(move |inner| inner.contains(&val))
                }
            }
        )+
    };
}
impl_contains!(str, SmartFilterPath, SmartFilterCheck, str);
impl_contains!(
    str,
    SmartFilterOwnedPath,
    SmartFilterOwnedCheck,
    String,
    Box<str>
);
impl_contains!(slice, SmartFilterPath, SmartFilterCheck, [T] => [T]);
impl_contains!(slice, SmartFilterOwnedPath, SmartFilterOwnedCheck, Vec<T> => [T], Box<[T]> => [T]);

macro_rules! impl_check_invert {
    ($check:ident, $mode:ident, $accessor:ident, bounds = { $($bounds:tt)* }) => {
        impl<T> $check<T>
        where
            T: $($bounds)*
        {
            #[must_use]
            pub fn invert(self) -> Self {
                let accessor = self.accessor.clone();
                let mode = self.mode;

                let inverted = match mode {
                    $mode::IsSome => $mode::IsNone,
                    $mode::IsNone => $mode::IsSome,
                    $mode::Predicate(f) => $mode::Predicate(Arc::new(move |val| !f(val))),
                    $mode::AsyncPredicate(f) => $mode::AsyncPredicate(Arc::new(move |val| {
                        let fut = f(val);
                        Box::pin(async move { !fut.await })
                    })),
                };

                Self {
                    accessor,
                    mode: inverted,
                }
            }
        }
    };
}
impl_check_invert!(SmartFilterCheck, SmartFilterMode, Accessor, bounds = { ?Sized + 'static });
impl_check_invert!(SmartFilterOwnedCheck, SmartFilterOwnedMode, OwnedAccessor, bounds = { 'static });
