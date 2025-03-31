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
use crate::Request;

use async_trait::async_trait;

pub struct And<Client> {
    filters: Vec<Box<dyn Filter<Client>>>,
}

impl<Client> Clone for And<Client> {
    fn clone(&self) -> Self {
        Self {
            filters: self.filters.clone(),
        }
    }
}

pub struct Or<Client> {
    filters: Vec<Box<dyn Filter<Client>>>,
}

impl<Client> Clone for Or<Client> {
    fn clone(&self) -> Self {
        Self {
            filters: self.filters.clone(),
        }
    }
}

pub struct Invert<Client> {
    filter: Box<dyn Filter<Client>>,
}

impl<Client> Clone for Invert<Client> {
    fn clone(&self) -> Self {
        Self {
            filter: self.filter.clone(),
        }
    }
}

/// A macro to implement methods for [`And`] and [`Or`] filters, because they have the same methods
macro_rules! impl_methods {
    ($struct_name:ident, $method_name:ident) => {
        impl<Client> $struct_name<Client>
        where
            Client: Send + Sync + 'static,
        {
            #[must_use]
            pub fn new(filter: impl Filter<Client>) -> Self {
                Self {
                    filters: vec![Box::new(filter)],
                }
            }

            /// Add a filter to the filters chain
            #[must_use]
            pub fn $method_name(self, filter: impl Filter<Client>) -> Self {
                Self {
                    filters: self
                        .filters
                        .into_iter()
                        .chain(Some(Box::new(filter) as _))
                        .collect(),
                }
            }
        }
    };
}

impl_methods!(Or, or);
impl_methods!(And, and);

impl<Client> Invert<Client>
where
    Client: Send + Sync + 'static,
{
    pub fn new(filter: impl Filter<Client>) -> Self {
        Self {
            filter: Box::new(filter),
        }
    }
}

impl<Client> And<Client>
where
    Client: 'static,
{
    pub async fn validate(&mut self, request: &mut Request<Client>) -> bool {
        for filter in &mut self.filters {
            let result = filter.check(request).await;
            if !result {
                return false;
            }
        }
        true
    }
}

impl<Client> Or<Client>
where
    Client: 'static,
{
    pub async fn validate(&mut self, request: &mut Request<Client>) -> bool {
        for filter in &mut self.filters {
            let result = filter.check(request).await;
            if result {
                return true;
            }
        }
        false
    }
}

impl<Client> Invert<Client>
where
    Client: 'static,
{
    pub async fn validate(&mut self, request: &mut Request<Client>) -> bool {
        !self.filter.check(request).await
    }
}

/// A macro to implement [`Filter`] for [`And`], [`Or`] and [`Invert`] filters, because they have the same implementation
macro_rules! impl_filter {
    ($name:ident) => {
        #[async_trait]
        impl<Client> Filter<Client> for $name<Client>
        where
            Client: Send + Sync + 'static,
        {
            async fn check(&mut self, request: &mut Request<Client>) -> bool {
                self.validate(request).await
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
    use crate::client::Reqwest;

    #[tokio::test]
    async fn test_and() {
        let mut request = Request::<Reqwest>::default();

        assert!(
            And::new(|_req: &mut Request| async { true })
                .validate(&mut request)
                .await
        );
        assert!(
            !And::new(|_req: &mut Request| async { false })
                .validate(&mut request)
                .await
        );
        assert!(
            And::new(|_req: &mut Request| async { true })
                .and(|_req: &mut Request| async { true })
                .validate(&mut request)
                .await
        );
        assert!(
            !And::new(|_req: &mut Request| async { false })
                .and(|_req: &mut Request| async { true })
                .validate(&mut request)
                .await
        );
        assert!(
            !And::new(|_req: &mut Request| async { true })
                .and(|_req: &mut Request| async { false })
                .validate(&mut request)
                .await
        );
    }

    #[tokio::test]
    async fn test_or() {
        let mut request = Request::<Reqwest>::default();

        assert!(
            Or::new(|_req: &mut Request| async { true })
                .validate(&mut request)
                .await
        );
        assert!(
            !Or::new(|_req: &mut Request| async { false })
                .validate(&mut request)
                .await
        );
        assert!(
            Or::new(|_req: &mut Request| async { true })
                .or(|_req: &mut Request| async { true })
                .validate(&mut request)
                .await
        );
        assert!(
            Or::new(|_req: &mut Request| async { false })
                .or(|_req: &mut Request| async { true })
                .validate(&mut request)
                .await
        );
        assert!(
            Or::new(|_req: &mut Request| async { true })
                .or(|_req: &mut Request| async { false })
                .validate(&mut request)
                .await
        );
    }

    #[tokio::test]
    async fn test_invert() {
        let mut request = Request::<Reqwest>::default();

        assert!(
            Invert::new(|_req: &mut Request| async { false })
                .validate(&mut request)
                .await
        );
        assert!(
            !Invert::new(|_req: &mut Request| async { true })
                .validate(&mut request)
                .await
        );
    }
}
