//! Logical filters that allow you to combine other filters.
//!
//! By default, the following logical filters are available:
//! - [`And`] - allows you to combine filters with the logical AND operation.
//! - [`Or`] - allows you to combine filters with the logical OR operation.
//! - [`Invert`] - allows you to invert the result of the filter.
//!
//! But using these filters directly isn't very convenient,
//! [`Filter`] trait has methods that allow you to combine filters in a more convenient way,
//! see [`Filter::and`], [`Filter::or`] and [`Filter::invert`] methods.

use super::{Filter, FilterResult};
use crate::{Either, Request};

#[derive(Clone)]
pub struct And<F, S>(pub F, pub S);

impl<Client, F, S> Filter<Client> for And<F, S>
where
    F: Filter<Client>,
    S: Filter<Client>,
    Client: Send,
{
    type Error = Either<F::Error, S::Error>;

    async fn check(&mut self, request: &mut Request<Client>) -> Result<bool, Self::Error> {
        Ok(self.0.check(request).await.map_err(Self::Error::Left)?
            && self.1.check(request).await.map_err(Self::Error::Right)?)
    }
}

#[derive(Clone)]
pub struct Or<F, S>(pub F, pub S);

impl<Client, F, S> Filter<Client> for Or<F, S>
where
    F: Filter<Client>,
    S: Filter<Client>,
    Client: Send,
{
    type Error = Either<F::Error, S::Error>;

    async fn check(&mut self, request: &mut Request<Client>) -> FilterResult<Self::Error> {
        Ok(self.0.check(request).await.map_err(Self::Error::Left)?
            || self.1.check(request).await.map_err(Self::Error::Right)?)
    }
}

#[derive(Clone)]
pub struct Invert<F>(pub F);

impl<Client, F> Filter<Client> for Invert<F>
where
    F: Filter<Client>,
    Client: Send,
{
    type Error = F::Error;

    async fn check(&mut self, request: &mut Request<Client>) -> FilterResult<Self::Error> {
        Ok(!self.0.check(request).await?)
    }
}
