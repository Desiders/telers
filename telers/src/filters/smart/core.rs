#![allow(clippy::type_complexity)]

use crate::{types::Update, Filter, Request};

use std::{convert::Infallible, future::Future, pin::Pin, sync::Arc};

type Accessor<T> = Arc<dyn for<'a> Fn(&'a Update) -> Option<&'a T> + Send + Sync>;
type OwnedAccessor<T> = Arc<dyn Fn(&Update) -> Option<T> + Send + Sync>;

pub enum SmartFilterMode<T: ?Sized> {
    IsSome,
    IsNone,
    Predicate(Arc<dyn Fn(&T) -> bool + Send + Sync>),
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

pub enum SmartFilterOwnedMode<T> {
    IsSome,
    IsNone,
    Predicate(Arc<dyn Fn(T) -> bool + Send + Sync>),
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
    ) -> impl Future<Output = Result<bool, Self::Error>> + Send {
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
    ) -> impl Future<Output = Result<bool, Self::Error>> + Send {
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

#[derive(Clone, Copy)]
pub enum BranchOperator {
    All,
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
            ) -> impl Future<Output = Result<bool, Self::Error>> + Send {
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
