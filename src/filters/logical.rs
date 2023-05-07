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

use super::base::Filter;

use crate::{client::Bot, context::Context, types::Update};

use async_trait::async_trait;
use std::sync::Arc;

pub struct And<Client> {
    filters: Vec<Arc<dyn Filter<Client>>>,
}

pub struct Or<Client> {
    filters: Vec<Arc<dyn Filter<Client>>>,
}

pub struct Invert<Client> {
    filter: Arc<dyn Filter<Client>>,
}

/// A macro to implement methods for [`And`] and [`Or`] filters, because they have the same methods
macro_rules! impl_methods {
    ($struct_name:ident, $method_name:ident) => {
        impl<Client> $struct_name<Client> {
            #[must_use]
            pub fn new(filter: impl Filter<Client> + 'static) -> Self {
                Self {
                    filters: vec![Arc::new(filter)],
                }
            }

            /// Add a filter to the filters chain
            #[must_use]
            pub fn $method_name(self, filter: impl Filter<Client> + 'static) -> Self {
                Self {
                    filters: self
                        .filters
                        .into_iter()
                        .chain(Some(Arc::new(filter) as _))
                        .collect(),
                }
            }
        }
    };
}

impl_methods!(Or, or);
impl_methods!(And, and);

impl<Client> Invert<Client> {
    pub fn new(filter: impl Filter<Client> + 'static) -> Self {
        Self {
            filter: Arc::new(filter),
        }
    }
}

impl<Client> And<Client>
where
    Client: Sync,
{
    pub async fn validate(&self, bot: &Bot<Client>, update: &Update, context: &Context) -> bool {
        for filter in &self.filters {
            if !filter.check(bot, update, context).await {
                return false;
            }
        }

        true
    }
}

impl<Client> Or<Client>
where
    Client: Sync,
{
    pub async fn validate(&self, bot: &Bot<Client>, update: &Update, context: &Context) -> bool {
        for filter in &self.filters {
            if filter.check(bot, update, context).await {
                return true;
            }
        }

        false
    }
}

impl<Client> Invert<Client>
where
    Client: Sync,
{
    pub async fn validate(&self, bot: &Bot<Client>, update: &Update, context: &Context) -> bool {
        !self.filter.check(bot, update, context).await
    }
}

/// A macro to implement [`Filter`] for [`And`], [`Or`] and [`Invert`] filters, because they have the same implementation
macro_rules! impl_filter {
    ($name:ident) => {
        #[async_trait]
        impl<Client> Filter<Client> for $name<Client>
        where
            Client: Sync,
        {
            async fn check(&self, bot: &Bot<Client>, update: &Update, context: &Context) -> bool {
                self.validate(bot, update, context).await
            }
        }
    };
}

impl_filter!(And);
impl_filter!(Or);
impl_filter!(Invert);

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        client::{Bot, Reqwest},
        context::Context,
        types::Update,
    };

    #[tokio::test]
    async fn test_and() {
        let bot = Bot::<Reqwest>::default();
        let context = Context::new();
        let update = Update::default();

        assert!(
            And::new(|_: &Bot, _: &Update, _: &Context| async { true })
                .validate(&bot, &update, &context)
                .await
        );
        assert!(
            !And::new(|_: &Bot, _: &Update, _: &Context| async { false })
                .validate(&bot, &update, &context)
                .await
        );
        assert!(
            And::new(|_: &Bot, _: &Update, _: &Context| async { true })
                .and(|_: &Bot, _: &Update, _: &Context| async { true })
                .validate(&bot, &update, &context)
                .await
        );
        assert!(
            !And::new(|_: &Bot, _: &Update, _: &Context| async { false })
                .and(|_: &Bot, _: &Update, _: &Context| async { true })
                .validate(&bot, &update, &context)
                .await
        );
        assert!(
            !And::new(|_: &Bot, _: &Update, _: &Context| async { true })
                .and(|_: &Bot, _: &Update, _: &Context| async { false })
                .validate(&bot, &update, &context)
                .await
        );
    }

    #[tokio::test]
    async fn test_or() {
        let bot = Bot::<Reqwest>::default();
        let context = Context::new();
        let update = Update::default();

        assert!(
            Or::new(|_: &Bot, _: &Update, _: &Context| async { true })
                .validate(&bot, &update, &context)
                .await
        );
        assert!(
            !Or::new(|_: &Bot, _: &Update, _: &Context| async { false })
                .validate(&bot, &update, &context)
                .await
        );
        assert!(
            Or::new(|_: &Bot, _: &Update, _: &Context| async { true })
                .or(|_: &Bot, _: &Update, _: &Context| async { true })
                .validate(&bot, &update, &context)
                .await
        );
        assert!(
            Or::new(|_: &Bot, _: &Update, _: &Context| async { false })
                .or(|_: &Bot, _: &Update, _: &Context| async { true })
                .validate(&bot, &update, &context)
                .await
        );
        assert!(
            Or::new(|_: &Bot, _: &Update, _: &Context| async { true })
                .or(|_: &Bot, _: &Update, _: &Context| async { false })
                .validate(&bot, &update, &context)
                .await
        );
    }

    #[tokio::test]
    async fn test_invert() {
        let bot = Bot::<Reqwest>::default();
        let context = Context::new();
        let update = Update::default();

        assert!(
            Invert::new(|_: &Bot, _: &Update, _: &Context| async { false })
                .validate(&bot, &update, &context)
                .await
        );
        assert!(
            !Invert::new(|_: &Bot, _: &Update, _: &Context| async { true })
                .validate(&bot, &update, &context)
                .await
        );
    }
}
