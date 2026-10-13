use std::sync::Arc;

use async_fn_traits::AsyncFn1;

use super::{DataMap, RenderContext};
use crate::{future::BoxFuture, DialogError};

type GetterHandler =
    dyn Fn(RenderContext) -> BoxFuture<'static, Result<DataMap, DialogError>> + Send + Sync + 'static;

/// Async render data provider; returned fields override earlier providers.
#[derive(Clone)]
pub struct DataGetter(Arc<GetterHandler>);

impl DataGetter {
    #[must_use]
    pub fn new<F>(getter: F) -> Self
    where
        F: AsyncFn(RenderContext) -> Result<DataMap, DialogError>
            + AsyncFn1<RenderContext, Output = Result<DataMap, DialogError>>
            + Send
            + Sync
            + 'static,
        <F as AsyncFn1<RenderContext>>::OutputFuture: Send + 'static,
    {
        let getter = Arc::new(getter);
        Self(Arc::new(move |context| {
            let getter = getter.clone();
            Box::pin(async move { getter(context).await })
        }))
    }

    pub async fn load(&self, context: &RenderContext) -> Result<RenderContext, DialogError> {
        let mut data = context.data.as_ref().clone();
        data.extend((self.0)(context.clone()).await?);
        Ok(context.clone().with_data(data))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::DataGetter;
    use crate::{
        entities::{ChatEvent, Context, DataMap, EventContext, RenderContext},
        DialogError,
    };
    use telers::{
        client::Reqwest,
        serialization::json,
        types::{ChatPrivate, MessageText, User},
        Bot,
    };

    fn render_context() -> RenderContext {
        let context = Context::new("", "state", json!(null));
        let data = DataMap::from([("inherited".into(), json!("original"))]);
        let event = ChatEvent::Message(
            MessageText::new(1, 1, ChatPrivate::new(10), "input")
                .from(User::new(10, false, "Tester"))
                .into(),
        );
        let event_context = EventContext::new(Bot::<Reqwest>::default(), event.clone());
        let mut runtime = telers::Context::default();
        runtime.insert("prefix", String::from("hello"));
        RenderContext::new(&context, &data, &event, &event_context).with_runtime_context(&runtime)
    }

    #[tokio::test]
    async fn successful_getter_preserves_inherited_data_and_runtime_context() {
        let context = render_context();
        let getter = DataGetter::new(|context: RenderContext| async move {
            assert_eq!(context.data["inherited"].as_str(), Some("original"));
            Ok(DataMap::from([(
                "derived".into(),
                json!(context.runtime_data::<String>("prefix").unwrap()),
            )]))
        });
        let loaded = getter.load(&context).await.unwrap();
        assert_eq!(loaded.data["derived"].as_str(), Some("hello"));
        assert_eq!(loaded.data["inherited"].as_str(), Some("original"));
        assert!(!context.data.contains_key("derived"));
        assert!(Arc::ptr_eq(&context.context, &loaded.context));
        assert!(Arc::ptr_eq(&context.event, &loaded.event));
        assert!(Arc::ptr_eq(&context.event_context, &loaded.event_context));
        assert!(Arc::ptr_eq(
            &context.runtime_context,
            &loaded.runtime_context,
        ));
    }

    #[tokio::test]
    async fn failing_getter_cannot_modify_inherited_data_or_stored_context() {
        let context = render_context();
        let getter = DataGetter::new(|mut context: RenderContext| async move {
            Arc::make_mut(&mut context.data).insert("inherited".into(), json!("changed"));
            Arc::make_mut(&mut context.context)
                .dialog_data
                .insert("leaked".into(), json!(true));
            Err(DialogError::NoContext)
        });
        assert!(matches!(
            getter.load(&context).await,
            Err(DialogError::NoContext),
        ));
        assert_eq!(context.data["inherited"].as_str(), Some("original"));
        assert!(!context.context.dialog_data.contains_key("leaked"));
    }
}
