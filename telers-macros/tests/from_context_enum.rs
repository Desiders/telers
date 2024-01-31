use telers::extractors::FromEventAndContext;
use telers_macros::FromContext;

#[derive(FromContext, Clone)]
#[context(key = "no_generic")]
enum NoGeneric {
    _Variant1,
}

#[derive(FromContext, Clone)]
#[context(key = "single_generic")]
enum SingleGeneric<T> {
    _Variant1(T),
}

#[derive(FromContext, Clone)]
#[context(key = "multi_generic")]
enum MultiGeneric<T, E> {
    _Variant1(T),
    _Variant2(E),
}

#[derive(FromContext, Clone)]
#[context(key = "no_generic_with_lifetime")]
enum NoGenericWithLifetime<'a> {
    _Variant1(&'a str),
}

#[derive(FromContext, Clone)]
#[context(key = "single_generic_with_lifetime")]
enum SingleGenericWithLifetime<'a, T> {
    _Variant1(&'a T),
}

#[derive(FromContext, Clone)]
#[context(key = "multi_generic_with_lifetime")]
enum MultiGenericWithLifetime<'a, T, E> {
    _Variant1(&'a T),
    _Variant2(&'a E),
}

#[derive(FromContext, Clone)]
#[context(key = "single_generic_with_trait_bound")]
enum SingleGenericWithTraitBound<T: Clone> {
    _Variant1(T),
}

#[derive(FromContext, Clone)]
#[context(key = "multi_generic_with_trait_bound")]
enum MultiGenericWithTraitBound<T: Clone, E: Clone> {
    _Variant1(T),
    _Variant2(E),
}

#[derive(FromContext, Clone)]
#[context(key = "single_generic_with_lifetime_and_trait_bound")]
enum SingleGenericWithLifetimeAndTraitBound<'a, T: AsRef<str>> {
    _Variant1(&'a T),
}

#[derive(FromContext, Clone)]
#[context(key = "multi_generic_with_lifetime_and_trait_bound")]
enum MultiGenericWithLifetimeAndTraitBound<'a, T: AsRef<str>, E: AsRef<str>> {
    _Variant1(&'a T),
    _Variant2(&'a E),
}

#[derive(FromContext, Clone)]
#[context(key = "single_generic_with_lifetime_and_multi_trait_bound")]
enum SingleGenericWithLifetimeAndMultiTraitBound<'a, T: AsRef<str> + Clone> {
    _Variant1(&'a T),
}

#[derive(FromContext, Clone)]
#[context(key = "multi_generic_with_lifetime_and_multi_trait_bound")]
enum MultiGenericWithLifetimeAndMultiTraitBound<
    'a,
    'b,
    T: AsRef<str> + Clone,
    E: AsRef<str> + Clone,
> {
    _Variant1(&'a T),
    _Variant2(&'b E),
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
