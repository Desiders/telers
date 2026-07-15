//! Regression tests for `#[derive(FromEvent)]` on types with an explicit `where` clause.
//!
//! Same codegen bug as `from_context_where_clause.rs`: the derive emitted the whole `WhereClause`
//! (including its `where` keyword) into an impl template that already had a literal `where`,
//! expanding to an unparsable `where where ...`, with no separator before the bounds the derive
//! appends. Any type declared with a `where` clause failed to compile.
//!
//! These are compile-only tests: if the generated impl is malformed, this file does not build.

#![allow(
    clippy::extra_unused_type_parameters,
    clippy::extra_unused_lifetimes,
    dead_code
)]

use std::convert::Infallible;
use telers::{extractor::Extractor, types::Update};
use telers_macros::FromEvent;

#[derive(FromEvent)]
#[event(from = Update)]
struct SinglePredicate<T>
where
    T: Clone,
{
    _field: T,
}

impl<T> From<Update> for SinglePredicate<T>
where
    T: Clone,
{
    fn from(_: Update) -> Self {
        unimplemented!()
    }
}

#[derive(FromEvent)]
#[event(from = Update)]
struct MultiPredicate<T, E>
where
    T: Clone,
    E: Clone,
{
    _field: T,
    _field2: E,
}

impl<T, E> From<Update> for MultiPredicate<T, E>
where
    T: Clone,
    E: Clone,
{
    fn from(_: Update) -> Self {
        unimplemented!()
    }
}

/// Inline bounds and a `where` clause at the same time: both must survive into the impl.
#[derive(FromEvent)]
#[event(from = Update)]
struct InlineBoundAndWhereClause<T: Clone, E>
where
    E: Clone,
{
    _field: T,
    _field2: E,
}

impl<T: Clone, E> From<Update> for InlineBoundAndWhereClause<T, E>
where
    E: Clone,
{
    fn from(_: Update) -> Self {
        unimplemented!()
    }
}

#[derive(FromEvent)]
#[event(try_from = Update, error = Infallible)]
struct TryWithWhereClause<T>
where
    T: Clone,
{
    _field: T,
}

impl<T> TryFrom<Update> for TryWithWhereClause<T>
where
    T: Clone,
{
    type Error = Infallible;

    fn try_from(_: Update) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}

#[derive(FromEvent)]
#[event(from = Update)]
enum EnumWithWhereClause<T>
where
    T: Clone,
{
    _Variant1(T),
}

impl<T> From<Update> for EnumWithWhereClause<T>
where
    T: Clone,
{
    fn from(_: Update) -> Self {
        unimplemented!()
    }
}

#[allow(unreachable_code)]
fn _check_bounds<Client, T: Extractor<Client>>() {
    unimplemented!("This function is only used for checking bounds");

    _check_bounds::<(), SinglePredicate<i32>>();
    _check_bounds::<(), MultiPredicate<i32, i32>>();
    _check_bounds::<(), InlineBoundAndWhereClause<i32, i32>>();
    _check_bounds::<(), TryWithWhereClause<i32>>();
    _check_bounds::<(), EnumWithWhereClause<i32>>();
}
