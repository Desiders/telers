use telers::extractors::FromEventAndContext;
use telers_macros::FromContext;

#[derive(Clone)]
struct NoGeneric {
    _field: i32,
}

#[derive(FromContext)]
#[context(key = "no_generic", from = NoGeneric)]
struct NoGenericWrapper(NoGeneric);

impl From<NoGeneric> for NoGenericWrapper {
    fn from(data: NoGeneric) -> Self {
        Self(data)
    }
}

#[derive(Clone)]
struct SingleGeneric<T> {
    _field: T,
}

#[derive(FromContext)]
#[context(key = "single_generic", from = SingleGeneric)]
struct SingleGenericWrapper<T>(SingleGeneric<T>);

impl<T> From<SingleGeneric<T>> for SingleGenericWrapper<T> {
    fn from(data: SingleGeneric<T>) -> Self {
        Self(data)
    }
}

#[derive(Clone)]
struct MultiGeneric<T, E> {
    _field: T,
    _field2: E,
}

#[derive(FromContext)]
#[context(key = "multi_generic", from = MultiGeneric)]
struct MultiGenericWrapper<T, E>(MultiGeneric<T, E>);

impl<T, E> From<MultiGeneric<T, E>> for MultiGenericWrapper<T, E> {
    fn from(data: MultiGeneric<T, E>) -> Self {
        Self(data)
    }
}

#[derive(Clone)]
struct NoGenericWithLifetime<'a> {
    _field: &'a str,
}

#[derive(FromContext)]
#[context(key = "no_generic_with_lifetime", from = NoGenericWithLifetime)]
struct NoGenericWithLifetimeWrapper<'a>(NoGenericWithLifetime<'a>);

impl<'a> From<NoGenericWithLifetime<'a>> for NoGenericWithLifetimeWrapper<'a> {
    fn from(data: NoGenericWithLifetime<'a>) -> Self {
        Self(data)
    }
}

#[derive(Clone)]
struct SingleGenericWithLifetime<'a, T> {
    _field: &'a T,
}

#[derive(FromContext)]
#[context(key = "single_generic_with_lifetime", from = SingleGenericWithLifetime)]
struct SingleGenericWithLifetimeWrapper<'a, T>(SingleGenericWithLifetime<'a, T>);

impl<'a, T> From<SingleGenericWithLifetime<'a, T>> for SingleGenericWithLifetimeWrapper<'a, T> {
    fn from(data: SingleGenericWithLifetime<'a, T>) -> Self {
        Self(data)
    }
}

#[derive(Clone)]
struct MultiGenericWithLifetime<'a, T, E> {
    _field: &'a T,
    _field2: &'a E,
}

#[derive(FromContext)]
#[context(key = "multi_generic_with_lifetime", from = MultiGenericWithLifetime)]
struct MultiGenericWithLifetimeWrapper<'a, T, E>(MultiGenericWithLifetime<'a, T, E>);

impl<'a, T, E> From<MultiGenericWithLifetime<'a, T, E>>
    for MultiGenericWithLifetimeWrapper<'a, T, E>
{
    fn from(data: MultiGenericWithLifetime<'a, T, E>) -> Self {
        Self(data)
    }
}

#[derive(Clone)]
struct SingleGenericWithLifetimeAndTraitBound<'a, T: AsRef<str>> {
    _field: &'a T,
}

#[derive(FromContext)]
#[context(key = "single_generic_with_lifetime_and_trait_bound", from = SingleGenericWithLifetimeAndTraitBound)]
struct SingleGenericWithLifetimeAndTraitBoundWrapper<'a, T: AsRef<str>>(
    SingleGenericWithLifetimeAndTraitBound<'a, T>,
);

impl<'a, T: AsRef<str>> From<SingleGenericWithLifetimeAndTraitBound<'a, T>>
    for SingleGenericWithLifetimeAndTraitBoundWrapper<'a, T>
{
    fn from(data: SingleGenericWithLifetimeAndTraitBound<'a, T>) -> Self {
        Self(data)
    }
}

#[derive(Clone)]
struct MultiGenericWithLifetimeAndTraitBound<'a, T: AsRef<str>, E: AsRef<str>> {
    _field: &'a T,
    _field2: &'a E,
}

#[derive(FromContext)]
#[context(key = "multi_generic_with_lifetime_and_trait_bound", from = MultiGenericWithLifetimeAndTraitBound)]
struct MultiGenericWithLifetimeAndTraitBoundWrapper<'a, T: AsRef<str>, E: AsRef<str>>(
    MultiGenericWithLifetimeAndTraitBound<'a, T, E>,
);

impl<'a, T: AsRef<str>, E: AsRef<str>> From<MultiGenericWithLifetimeAndTraitBound<'a, T, E>>
    for MultiGenericWithLifetimeAndTraitBoundWrapper<'a, T, E>
{
    fn from(data: MultiGenericWithLifetimeAndTraitBound<'a, T, E>) -> Self {
        Self(data)
    }
}

#[derive(Clone)]
struct SingleGenericWithLifetimeAndMultiTraitBound<'a, T: AsRef<str> + Clone> {
    _field: &'a T,
}

#[derive(FromContext)]
#[context(key = "single_generic_with_lifetime_and_multi_trait_bound", from = SingleGenericWithLifetimeAndMultiTraitBound)]
struct SingleGenericWithLifetimeAndMultiTraitBoundWrapper<'a, T: AsRef<str> + Clone>(
    SingleGenericWithLifetimeAndMultiTraitBound<'a, T>,
);

impl<'a, T: AsRef<str> + Clone> From<SingleGenericWithLifetimeAndMultiTraitBound<'a, T>>
    for SingleGenericWithLifetimeAndMultiTraitBoundWrapper<'a, T>
{
    fn from(data: SingleGenericWithLifetimeAndMultiTraitBound<'a, T>) -> Self {
        Self(data)
    }
}

#[derive(Clone)]
struct MultiGenericWithLifetimeAndMultiTraitBound<
    'a,
    'b,
    T: AsRef<str> + Clone,
    E: AsRef<str> + Clone,
> {
    _field: &'a T,
    _field2: &'b E,
}

#[derive(FromContext)]
#[context(key = "multi_generic_with_lifetime_and_multi_trait_bound", from = MultiGenericWithLifetimeAndMultiTraitBound)]
struct MultiGenericWithLifetimeAndMultiTraitBoundWrapper<
    'a,
    'b,
    T: AsRef<str> + Clone,
    E: AsRef<str> + Clone,
>(MultiGenericWithLifetimeAndMultiTraitBound<'a, 'b, T, E>);

impl<'a, 'b, T: AsRef<str> + Clone, E: AsRef<str> + Clone>
    From<MultiGenericWithLifetimeAndMultiTraitBound<'a, 'b, T, E>>
    for MultiGenericWithLifetimeAndMultiTraitBoundWrapper<'a, 'b, T, E>
{
    fn from(data: MultiGenericWithLifetimeAndMultiTraitBound<'a, 'b, T, E>) -> Self {
        Self(data)
    }
}

#[allow(unreachable_code)]
fn _check_bounds<Client, T: FromEventAndContext<Client>>() {
    unimplemented!("This function is only used for checking bounds");

    _check_bounds::<(), NoGenericWrapper>();
    _check_bounds::<(), SingleGenericWrapper<i32>>();
    _check_bounds::<(), MultiGenericWrapper<i32, i32>>();
    _check_bounds::<(), NoGenericWithLifetimeWrapper<'_>>();
    _check_bounds::<(), SingleGenericWithLifetimeWrapper<'_, String>>();
    _check_bounds::<(), MultiGenericWithLifetimeWrapper<'_, String, String>>();
    _check_bounds::<(), SingleGenericWithLifetimeAndTraitBoundWrapper<'_, String>>();
    _check_bounds::<(), MultiGenericWithLifetimeAndTraitBoundWrapper<'_, String, String>>();
    _check_bounds::<(), SingleGenericWithLifetimeAndMultiTraitBoundWrapper<'_, String>>();
    _check_bounds::<(), MultiGenericWithLifetimeAndMultiTraitBoundWrapper<'_, '_, String, String>>(
    );
}
