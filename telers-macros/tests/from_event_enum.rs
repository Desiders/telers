use std::convert::Infallible;
use telers::{errors::ConvertToTypeError, extractors::FromEventAndContext, types::Update};
use telers_macros::FromEvent;

#[derive(FromEvent)]
#[event(from = Update)]
enum NoGeneric {
    _Field(i32),
}

impl From<Update> for NoGeneric {
    fn from(_: Update) -> Self {
        unimplemented!()
    }
}

#[derive(FromEvent)]
#[event(from = Update)]
enum SingleGeneric<T> {
    _Field(T),
}

impl<T> From<Update> for SingleGeneric<T> {
    fn from(_: Update) -> Self {
        unimplemented!()
    }
}

#[derive(FromEvent)]
#[event(from = Update)]
enum MultiGeneric<T, E> {
    _Field(T),
    _Field2(E),
}

impl<T, E> From<Update> for MultiGeneric<T, E> {
    fn from(_: Update) -> Self {
        unimplemented!()
    }
}

#[derive(FromEvent)]
#[event(from = Update)]
enum NoGenericWithLifetime<'a> {
    _Field(&'a str),
}

impl From<Update> for NoGenericWithLifetime<'_> {
    fn from(_: Update) -> Self {
        unimplemented!()
    }
}

#[derive(FromEvent)]
#[event(from = Update)]
enum SingleGenericWithLifetime<'a, T> {
    _Field(&'a T),
}

impl<T> From<Update> for SingleGenericWithLifetime<'_, T> {
    fn from(_: Update) -> Self {
        unimplemented!()
    }
}

#[derive(FromEvent)]
#[event(from = Update)]
enum MultiGenericWithLifetime<'a, T, E> {
    _Field(&'a T),
    _Field2(&'a E),
}

impl<T, E> From<Update> for MultiGenericWithLifetime<'_, T, E> {
    fn from(_: Update) -> Self {
        unimplemented!()
    }
}

#[derive(FromEvent)]
#[event(from = Update)]
enum SingleGenericWithTraitBound<T: Clone> {
    _Field(T),
}

impl<T: Clone> From<Update> for SingleGenericWithTraitBound<T> {
    fn from(_: Update) -> Self {
        unimplemented!()
    }
}

#[derive(FromEvent)]
#[event(from = Update)]
enum MultiGenericWithTraitBound<T: Clone, E: Clone> {
    _Field(T),
    _Field2(E),
}

impl<T: Clone, E: Clone> From<Update> for MultiGenericWithTraitBound<T, E> {
    fn from(_: Update) -> Self {
        unimplemented!()
    }
}

#[derive(FromEvent)]
#[event(from = Update)]
enum SingleGenericWithLifetimeAndTraitBound<'a, T: AsRef<str>> {
    _Field(&'a T),
}

impl<T: AsRef<str>> From<Update> for SingleGenericWithLifetimeAndTraitBound<'_, T> {
    fn from(_: Update) -> Self {
        unimplemented!()
    }
}

#[derive(FromEvent)]
#[event(from = Update)]
enum MultiGenericWithLifetimeAndTraitBound<'a, T: AsRef<str>, E: AsRef<str>> {
    _Field(&'a T),
    _Field2(&'a E),
}

impl<T: AsRef<str>, E: AsRef<str>> From<Update>
    for MultiGenericWithLifetimeAndTraitBound<'_, T, E>
{
    fn from(_: Update) -> Self {
        unimplemented!()
    }
}

#[derive(FromEvent)]
#[event(from = Update)]
enum SingleGenericWithLifetimeAndMultiTraitBound<'a, T: AsRef<str> + Clone> {
    _Field(&'a T),
}

impl<T: AsRef<str> + Clone> From<Update> for SingleGenericWithLifetimeAndMultiTraitBound<'_, T> {
    fn from(_: Update) -> Self {
        unimplemented!()
    }
}

#[derive(FromEvent)]
#[event(from = Update)]
enum MultiGenericWithLifetimeAndMultiTraitBound<
    'a,
    'b,
    T: AsRef<str> + Clone,
    E: AsRef<str> + Clone,
> {
    _Field(&'a T),
    _Field2(&'b E),
}

impl<'a, 'b, T: AsRef<str> + Clone, E: AsRef<str> + Clone> From<Update>
    for MultiGenericWithLifetimeAndMultiTraitBound<'_, '_, T, E>
{
    fn from(_: Update) -> Self {
        unimplemented!()
    }
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

#[derive(FromEvent)]
#[event(try_from = Update)]
enum NoGenericTry {
    _Field(i32),
}

impl TryFrom<Update> for NoGenericTry {
    type Error = ConvertToTypeError;

    fn try_from(_: Update) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}

#[derive(FromEvent)]
#[event(try_from = Update, error = Infallible)]
enum NoGenericTryWithInfallible {
    _Field(i32),
}

impl TryFrom<Update> for NoGenericTryWithInfallible {
    type Error = Infallible;

    fn try_from(_: Update) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}

#[derive(FromEvent)]
#[event(try_from = Update)]
enum SingleGenericTry<T> {
    _Field(T),
}

impl<T> TryFrom<Update> for SingleGenericTry<T> {
    type Error = ConvertToTypeError;

    fn try_from(_: Update) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}

#[derive(FromEvent)]
#[event(try_from = Update, error = Infallible)]
enum SingleGenericTryWithInfallible<T> {
    _Field(T),
}

impl<T> TryFrom<Update> for SingleGenericTryWithInfallible<T> {
    type Error = Infallible;

    fn try_from(_: Update) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}

#[derive(FromEvent)]
#[event(try_from = Update)]
enum MultiGenericTry<T, E> {
    _Field(T),
    _Field2(E),
}

impl<T, E> TryFrom<Update> for MultiGenericTry<T, E> {
    type Error = ConvertToTypeError;

    fn try_from(_: Update) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}

#[derive(FromEvent)]
#[event(try_from = Update, error = Infallible)]
enum MultiGenericTryWithInfallible<T, E> {
    _Field(T),
    _Field2(E),
}

impl<T, E> TryFrom<Update> for MultiGenericTryWithInfallible<T, E> {
    type Error = Infallible;

    fn try_from(_: Update) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}

#[allow(unreachable_code)]
fn _check_bounds_try<Client, T: FromEventAndContext<Client>>() {
    unimplemented!("This function is only used for checking bounds");

    _check_bounds_try::<(), NoGenericTry>();
    _check_bounds_try::<(), NoGenericTryWithInfallible>();
    _check_bounds_try::<(), SingleGenericTry<i32>>();
    _check_bounds_try::<(), SingleGenericTryWithInfallible<i32>>();
    _check_bounds_try::<(), MultiGenericTry<i32, i32>>();
    _check_bounds_try::<(), MultiGenericTryWithInfallible<i32, i32>>();
}
