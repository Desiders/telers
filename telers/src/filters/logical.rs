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

use super::base::{boxed_filter_factory, BoxedCloneFilterService, Filter};
use crate::{event::service::Service as _, Request};

use async_trait::async_trait;

pub struct And<Client> {
    filters: Vec<BoxedCloneFilterService<Client>>,
}

impl<Client> Clone for And<Client> {
    fn clone(&self) -> Self {
        Self {
            filters: self.filters.clone(),
        }
    }
}

pub struct Or<Client> {
    filters: Vec<BoxedCloneFilterService<Client>>,
}

impl<Client> Clone for Or<Client> {
    fn clone(&self) -> Self {
        Self {
            filters: self.filters.clone(),
        }
    }
}

pub struct Invert<Client> {
    filter: BoxedCloneFilterService<Client>,
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
                    filters: vec![boxed_filter_factory(filter)],
                }
            }

            /// Add a filter to the filters chain
            #[must_use]
            pub fn $method_name(self, filter: impl Filter<Client>) -> Self {
                Self {
                    filters: self
                        .filters
                        .into_iter()
                        .chain(Some(boxed_filter_factory(filter)))
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
            filter: boxed_filter_factory(filter),
        }
    }
}

impl<Client> And<Client> {
    #[allow(clippy::missing_panics_doc)]
    pub async fn validate(&mut self, mut request: Request<Client>) -> (bool, Request<Client>) {
        for filter in &mut self.filters {
            let (result, new_request) = filter.call(request).await.unwrap();
            if !result {
                return (false, new_request);
            }
            request = new_request;
        }
        (true, request)
    }
}

impl<Client> Or<Client> {
    #[allow(clippy::missing_panics_doc)]
    pub async fn validate(&mut self, mut request: Request<Client>) -> (bool, Request<Client>) {
        for filter in &mut self.filters {
            let (result, new_request) = filter.call(request).await.unwrap();
            if result {
                return (true, new_request);
            }
            request = new_request;
        }
        (false, request)
    }
}

impl<Client> Invert<Client> {
    #[allow(clippy::missing_panics_doc)]
    pub async fn validate(&mut self, request: Request<Client>) -> (bool, Request<Client>) {
        let (result, request) = self.filter.call(request).await.unwrap();
        (!result, request)
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
            async fn check(&mut self, request: Request<Client>) -> (bool, Request<Client>) {
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
        let request = Request::<Reqwest>::default();

        assert!(
            And::new(|req| async { (true, req) })
                .validate(request.clone())
                .await
                .0
        );
        assert!(
            !And::new(|req| async { (false, req) })
                .validate(request.clone())
                .await
                .0
        );
        assert!(
            And::new(|req| async { (true, req) })
                .and(|req| async { (true, req) })
                .validate(request.clone())
                .await
                .0
        );
        assert!(
            !And::new(|req| async { (false, req) })
                .and(|req| async { (true, req) })
                .validate(request.clone())
                .await
                .0
        );
        assert!(
            !And::new(|req| async { (true, req) })
                .and(|req| async { (false, req) })
                .validate(request)
                .await
                .0
        );
    }

    #[tokio::test]
    async fn test_or() {
        let request = Request::<Reqwest>::default();

        assert!(
            Or::new(|req| async { (true, req) })
                .validate(request.clone())
                .await
                .0
        );
        assert!(
            !Or::new(|req| async { (false, req) })
                .validate(request.clone())
                .await
                .0
        );
        assert!(
            Or::new(|req| async { (true, req) })
                .or(|req| async { (true, req) })
                .validate(request.clone())
                .await
                .0
        );
        assert!(
            Or::new(|req| async { (false, req) })
                .or(|req| async { (true, req) })
                .validate(request.clone())
                .await
                .0
        );
        assert!(
            Or::new(|req| async { (true, req) })
                .or(|req| async { (false, req) })
                .validate(request)
                .await
                .0
        );
    }

    #[tokio::test]
    async fn test_invert() {
        let request = Request::<Reqwest>::default();

        assert!(
            Invert::new(|req| async { (false, req) })
                .validate(request.clone())
                .await
                .0
        );
        assert!(
            !Invert::new(|req| async { (true, req) })
                .validate(request)
                .await
                .0
        );
    }
}
