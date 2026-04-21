use std::{borrow::Cow, sync::Arc};

use async_fn_traits::AsyncFn1;

use crate::{
    entities::{Context, Data, DataMap},
    future::BoxFuture,
};

/// Runtime inputs available to async visibility predicates.
#[derive(Clone, Debug)]
pub struct WhenContext {
    /// Stored dialog context.
    pub context: Arc<Context>,
    /// Render data for the current window.
    pub data: Arc<DataMap>,
}

impl WhenContext {
    #[inline]
    #[must_use]
    pub fn new(context: &Context, data: &DataMap) -> Self {
        Self {
            context: Arc::new(context.clone()),
            data: Arc::new(data.clone()),
        }
    }
}

type WhenPredicate = dyn Fn(WhenContext) -> BoxFuture<'static, bool> + Send + Sync + 'static;

/// Visibility condition shared by keyboard widgets.
#[derive(Clone)]
pub struct WhenCondition(Arc<WhenPredicate>);

impl WhenCondition {
    /// Create a condition from a predicate over dialog context and render data.
    #[inline]
    #[must_use]
    pub fn new<F>(predicate: F) -> Self
    where
        F: AsyncFn(WhenContext) -> bool
            + AsyncFn1<WhenContext, Output = bool>
            + Send
            + Sync
            + 'static,
        <F as AsyncFn1<WhenContext>>::OutputFuture: Send + 'static,
    {
        let predicate = Arc::new(predicate);
        Self(Arc::new(move |when_ctx| {
            let predicate = predicate.clone();
            Box::pin(async move { predicate(when_ctx).await })
        }))
    }

    /// Show the widget when the data field exists and is truthy.
    #[must_use]
    pub fn data_field(field: impl Into<Cow<'static, str>>) -> Self {
        let field = field.into();
        Self(Arc::new(move |when_ctx| {
            let field = field.clone();
            Box::pin(async move { when_ctx.data.get(field.as_ref()).is_some_and(is_truthy) })
        }))
    }

    #[inline]
    #[must_use]
    pub(crate) fn check<'a>(&'a self, ctx: &'a Context, data: &'a DataMap) -> BoxFuture<'a, bool> {
        (self.0)(WhenContext::new(ctx, data))
    }
}

#[inline]
#[must_use]
pub(crate) fn is_allowed<'a>(
    when: Option<&'a WhenCondition>,
    ctx: &'a Context,
    data: &'a DataMap,
) -> BoxFuture<'a, bool> {
    Box::pin(async move {
        match when {
            Some(condition) => condition.check(ctx, data).await,
            None => true,
        }
    })
}

#[must_use]
fn is_truthy(value: &Data) -> bool {
    match value {
        Data::Bool(value) => *value,
        Data::Null => false,
        Data::Number(value) => value.as_f64() != Some(0.0),
        Data::String(value) => !value.is_empty(),
        Data::Array(value) => !value.is_empty(),
        Data::Object(value) => !value.is_empty(),
    }
}
