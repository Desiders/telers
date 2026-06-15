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

#[cfg(test)]
mod tests {
    use super::{And, Invert, Or};
    use crate::{
        client::Reqwest,
        filters::{Filter, FilterResult},
        types::{ChatPrivate, MessageText, Update, UpdateMessage},
        Bot, Context, Either, Extensions, Request,
    };

    use std::{convert::Infallible, future::Future, sync::Arc};

    /// Filter that always returns a fixed result.
    #[derive(Clone)]
    struct Const(bool);

    impl<Client: Send + Sync + 'static> Filter<Client> for Const {
        type Error = Infallible;

        fn check(
            &mut self,
            _request: &mut Request<Client>,
        ) -> impl Future<Output = FilterResult<Self::Error>> + Send {
            let res = self.0;
            async move { Ok(res) }
        }
    }

    /// Filter that always fails; used to detect whether it is evaluated.
    #[derive(Clone)]
    struct Boom;

    impl<Client: Send + Sync + 'static> Filter<Client> for Boom {
        type Error = anyhow::Error;

        fn check(
            &mut self,
            _request: &mut Request<Client>,
        ) -> impl Future<Output = FilterResult<Self::Error>> + Send {
            async move { Err(anyhow::anyhow!("boom")) }
        }
    }

    fn request() -> Request<Reqwest> {
        Request {
            update: Arc::new(Update::Message(UpdateMessage::new(
                0,
                MessageText::new(0, 0, ChatPrivate::new(0), ""),
            ))),
            bot: Bot::default(),
            context: Context::default(),
            extensions: Extensions::default(),
        }
    }

    #[tokio::test]
    async fn and_truth_table() {
        let mut req = request();

        assert!(And(Const(true), Const(true)).check(&mut req).await.unwrap());
        assert!(!And(Const(true), Const(false))
            .check(&mut req)
            .await
            .unwrap());
        assert!(!And(Const(false), Const(true))
            .check(&mut req)
            .await
            .unwrap());
    }

    #[tokio::test]
    async fn and_short_circuits_on_false() {
        // A `false` left side must not evaluate the (failing) right side.
        let mut req = request();
        assert!(!And(Const(false), Boom).check(&mut req).await.unwrap());
    }

    #[tokio::test]
    async fn and_propagates_errors_with_side() {
        let mut req = request();

        let left = And(Boom, Const(true)).check(&mut req).await;
        assert!(matches!(left, Err(Either::Left(_))));

        let right = And(Const(true), Boom).check(&mut req).await;
        assert!(matches!(right, Err(Either::Right(_))));
    }

    #[tokio::test]
    async fn or_truth_table() {
        let mut req = request();

        assert!(!Or(Const(false), Const(false))
            .check(&mut req)
            .await
            .unwrap());
        assert!(Or(Const(false), Const(true)).check(&mut req).await.unwrap());
        assert!(Or(Const(true), Const(false)).check(&mut req).await.unwrap());
    }

    #[tokio::test]
    async fn or_short_circuits_on_true() {
        // A `true` left side must not evaluate the (failing) right side.
        let mut req = request();
        assert!(Or(Const(true), Boom).check(&mut req).await.unwrap());
    }

    #[tokio::test]
    async fn or_propagates_errors_with_side() {
        let mut req = request();

        let left = Or(Boom, Const(false)).check(&mut req).await;
        assert!(matches!(left, Err(Either::Left(_))));

        let right = Or(Const(false), Boom).check(&mut req).await;
        assert!(matches!(right, Err(Either::Right(_))));
    }

    #[tokio::test]
    async fn invert_negates_and_forwards_errors() {
        let mut req = request();

        assert!(!Invert(Const(true)).check(&mut req).await.unwrap());
        assert!(Invert(Const(false)).check(&mut req).await.unwrap());
        assert!(Invert(Boom).check(&mut req).await.is_err());
    }
}
