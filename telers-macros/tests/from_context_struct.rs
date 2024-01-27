use telers::extractors::FromEventAndContext;
use telers_macros::FromContext;

#[derive(FromContext, Clone)]
#[context(key = "no_generic")]
struct NoGeneric {
    _field: i32,
}

#[derive(FromContext, Clone)]
#[context(key = "single_generic")]
struct SingleGeneric<T> {
    _field: T,
}

#[derive(FromContext, Clone)]
#[context(key = "multi_generic")]
struct MultiGeneric<T, E> {
    _field: T,
    _field2: E,
}

#[derive(FromContext, Clone)]
#[context(key = "no_generic_with_lifetime")]
struct NoGenericWithLifetime<'a> {
    _field: &'a str,
}

#[derive(FromContext, Clone)]
#[context(key = "single_generic_with_lifetime")]
struct SingleGenericWithLifetime<'a, T> {
    _field: &'a T,
}

#[derive(FromContext, Clone)]
#[context(key = "multi_generic_with_lifetime")]
struct MultiGenericWithLifetime<'a, T, E> {
    _field: &'a T,
    _field2: &'a E,
}

#[derive(FromContext, Clone)]
#[context(key = "single_generic_with_trait_bound")]
struct SingleGenericWithTraitBound<T: Clone> {
    _field: T,
}

#[derive(FromContext, Clone)]
#[context(key = "multi_generic_with_trait_bound")]
struct MultiGenericWithTraitBound<T: Clone, E: Clone> {
    _field: T,
    _field2: E,
}

#[derive(FromContext, Clone)]
#[context(key = "single_generic_with_lifetime_and_trait_bound")]
struct SingleGenericWithLifetimeAndTraitBound<'a, T: AsRef<str>> {
    _field: &'a T,
}

#[derive(FromContext, Clone)]
#[context(key = "multi_generic_with_lifetime_and_trait_bound")]
struct MultiGenericWithLifetimeAndTraitBound<'a, T: AsRef<str>, E: AsRef<str>> {
    _field: &'a T,
    _field2: &'a E,
}

#[derive(FromContext, Clone)]
#[context(key = "single_generic_with_lifetime_and_multi_trait_bound")]
struct SingleGenericWithLifetimeAndMultiTraitBound<'a, T: AsRef<str> + Clone> {
    _field: &'a T,
}

#[derive(FromContext, Clone)]
#[context(key = "multi_generic_with_lifetime_and_multi_trait_bound")]
struct MultiGenericWithLifetimeAndMultiTraitBound<
    'a,
    'b,
    T: AsRef<str> + Clone,
    E: AsRef<str> + Clone,
> {
    _field: &'a T,
    _field2: &'b E,
}

#[allow(unreachable_code)]
fn _check_bounds<Client, T: FromEventAndContext<Client>>() {
    unimplemented!("This function is only used for checking bounds");

    _check_bounds::<(), NoGeneric>();
    _check_bounds::<(), SingleGeneric<i32>>();
    _check_bounds::<(), MultiGeneric<i32, i32>>();
    _check_bounds::<(), NoGenericWithLifetime<'_>>();
    _check_bounds::<(), SingleGenericWithLifetime<'_, i32>>();
    _check_bounds::<(), MultiGenericWithLifetime<'_, i32, i32>>();
    _check_bounds::<(), SingleGenericWithTraitBound<i32>>();
    _check_bounds::<(), MultiGenericWithTraitBound<i32, i32>>();
    _check_bounds::<(), SingleGenericWithLifetimeAndTraitBound<'_, String>>();
    _check_bounds::<(), MultiGenericWithLifetimeAndTraitBound<'_, String, String>>();
    _check_bounds::<(), SingleGenericWithLifetimeAndMultiTraitBound<'_, String>>();
    _check_bounds::<(), MultiGenericWithLifetimeAndMultiTraitBound<'_, '_, String, String>>();
}
