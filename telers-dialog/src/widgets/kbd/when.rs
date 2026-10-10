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
    pub(crate) fn check<'a>(&'a self, ctx: &'a Context, data: &'a DataMap) -> BoxFuture<'a, bool> {
        (self.0)(WhenContext::new(ctx, data))
    }
}

#[inline]
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
    if value.is_null() {
        false
    } else if let Some(value) = value.as_bool() {
        value
    } else if let Some(value) = value.as_str() {
        !value.is_empty()
    } else if let Some(value) = value.as_f64() {
        value != 0.0
    } else if let Some(values) = telers::serialization::as_array(value) {
        !values.is_empty()
    } else {
        #[cfg(feature = "deser")]
        {
            value.as_map().is_some_and(|values| !values.is_empty())
        }
        #[cfg(not(feature = "deser"))]
        {
            value.as_object().is_some_and(|values| !values.is_empty())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{is_allowed, WhenCondition, WhenContext};
    use crate::entities::{Context, DataMap};
    use telers::serialization::{json, Value};

    fn context() -> Context {
        Context::new("", "state", Value::default())
    }

    #[tokio::test]
    async fn none_is_always_allowed() {
        let ctx = context();
        let data = DataMap::new();

        assert!(is_allowed(None, &ctx, &data).await);
    }

    #[tokio::test]
    async fn data_field_missing_is_false() {
        let ctx = context();
        let data = DataMap::new();
        let condition = WhenCondition::data_field("f");

        assert!(!is_allowed(Some(&condition), &ctx, &data).await);
    }

    #[tokio::test]
    async fn data_field_truthiness_matrix() {
        let ctx = context();
        let condition = WhenCondition::data_field("f");

        let cases = [
            (json!(true), true),
            (json!(false), false),
            (json!(null), false),
            (json!(0), false),
            (json!(5), true),
            (json!(""), false),
            (json!("x"), true),
            (json!([]), false),
            (json!([1]), true),
            (json!({}), false),
            (json!({ "k": 1 }), true),
        ];

        for (value, expected) in cases {
            let mut data = DataMap::new();
            data.insert("f".into(), value.clone());

            assert_eq!(
                is_allowed(Some(&condition), &ctx, &data).await,
                expected,
                "value {value:?} should be {expected}",
            );
        }
    }

    #[tokio::test]
    async fn custom_predicate_reads_when_context_data_present() {
        let ctx = context();
        let mut data = DataMap::new();
        data.insert("go".into(), json!(true));

        let condition =
            WhenCondition::new(|wc: WhenContext| async move { wc.data.get("go").is_some() });

        assert!(is_allowed(Some(&condition), &ctx, &data).await);
    }

    #[tokio::test]
    async fn custom_predicate_reads_when_context_data_absent() {
        let ctx = context();
        let data = DataMap::new();

        let condition =
            WhenCondition::new(|wc: WhenContext| async move { wc.data.get("go").is_some() });

        assert!(!is_allowed(Some(&condition), &ctx, &data).await);
    }
}
