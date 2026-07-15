//! Regression tests for `#[derive(FromContext)]` on types with an explicit `where` clause.
//!
//! The derive splices the type's `where` predicates into the generated impl. Emitting the whole
//! `WhereClause` (which carries its own `where` keyword) expanded to an unparsable
//! `where where ...`, and omitting the separator ran the predicates into the bound the derive
//! appends — so *any* type declared with a `where` clause failed to compile
//! ("proc-macro derive produced unparsable tokens").
//!
//! These are compile-only tests: if the generated impl is malformed, this file does not build.
//! Types using inline bounds (`<T: Clone>`) never exercised this path, which is why the bug went
//! unnoticed — see the other `from_context_*` tests.

#![allow(
    clippy::extra_unused_type_parameters,
    clippy::extra_unused_lifetimes,
    dead_code
)]

use telers::extractor::Extractor;
use telers_macros::FromContext;

#[derive(FromContext, Clone)]
#[context(key = "single_predicate")]
struct SinglePredicate<T>
where
    T: Clone,
{
    _field: T,
}

#[derive(FromContext, Clone)]
#[context(key = "multi_predicate")]
struct MultiPredicate<T, E>
where
    T: Clone,
    E: Clone,
{
    _field: T,
    _field2: E,
}

#[derive(FromContext, Clone)]
#[context(key = "compound_predicate")]
struct CompoundPredicate<T>
where
    T: AsRef<str> + Clone,
{
    _field: T,
}

#[derive(FromContext, Clone)]
#[context(key = "lifetime_with_where_clause")]
struct LifetimeWithWhereClause<'a, T>
where
    T: AsRef<str>,
{
    _field: &'a T,
}

/// Inline bounds and a `where` clause at the same time: both must survive into the impl.
#[derive(FromContext, Clone)]
#[context(key = "inline_bound_and_where_clause")]
struct InlineBoundAndWhereClause<T: Clone, E>
where
    E: Clone,
{
    _field: T,
    _field2: E,
}

#[derive(FromContext, Clone)]
#[context(key = "enum_with_where_clause")]
enum EnumWithWhereClause<T>
where
    T: Clone,
{
    _Variant1(T),
}

#[allow(unreachable_code)]
fn _check_bounds<Client, T: Extractor<Client>>() {
    unimplemented!("This function is only used for checking bounds");

    _check_bounds::<(), SinglePredicate<i32>>();
    _check_bounds::<(), MultiPredicate<i32, i32>>();
    _check_bounds::<(), CompoundPredicate<String>>();
    _check_bounds::<(), LifetimeWithWhereClause<'_, String>>();
    _check_bounds::<(), InlineBoundAndWhereClause<i32, i32>>();
    _check_bounds::<(), EnumWithWhereClause<i32>>();
}
