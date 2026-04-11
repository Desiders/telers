use std::{borrow::Cow, sync::Arc};

use crate::entities::{Context, Data, DataMap};

type WhenPredicate = dyn Fn(&Context, &DataMap) -> bool + Send + Sync + 'static;

/// Visibility condition shared by keyboard widgets.
#[derive(Clone)]
pub struct WhenCondition(Arc<WhenPredicate>);

impl WhenCondition {
    /// Create a condition from a predicate over dialog context and render data.
    #[inline]
    #[must_use]
    pub fn new<F>(predicate: F) -> Self
    where
        F: Fn(&Context, &DataMap) -> bool + Send + Sync + 'static,
    {
        Self(Arc::new(predicate))
    }

    /// Show the widget when the data field exists and is truthy.
    #[must_use]
    pub fn data_field(field: impl Into<Cow<'static, str>>) -> Self {
        let field = field.into();
        Self::new(move |_ctx, data| data.get(field.as_ref()).is_some_and(is_truthy))
    }

    #[inline]
    #[must_use]
    pub(crate) fn check(&self, ctx: &Context, data: &DataMap) -> bool {
        (self.0)(ctx, data)
    }
}

#[inline]
#[must_use]
pub(crate) fn is_allowed(when: Option<&WhenCondition>, ctx: &Context, data: &DataMap) -> bool {
    when.is_none_or(|condition| condition.check(ctx, data))
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
