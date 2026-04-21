use bon::bon;
use std::{fmt::Display, marker::PhantomData, str::FromStr, sync::Arc};

use async_fn_traits::AsyncFn2;
use telers::types::Message;

use super::Input;
use crate::{entities::Context, future::BoxFuture, widgets::ButtonAction};

type TextInputErrorHandler<ParserErr> =
    dyn Fn(TextInputContext, ParserErr) -> BoxFuture<'static, ButtonAction> + Send + Sync;

#[derive(Clone, Debug)]
pub struct TextInputContext {
    /// Stored dialog context for the active intent.
    pub context: Arc<Context>,
}

pub struct TextInput<WidgetId, ParserOk, ParserErr, OnSuccess> {
    id: WidgetId,
    #[allow(clippy::type_complexity)]
    parser: Box<dyn Fn(&str) -> Result<ParserOk, ParserErr> + Send + Sync>,
    on_success: OnSuccess,
    #[allow(clippy::type_complexity)]
    on_error: Option<Arc<TextInputErrorHandler<ParserErr>>>,
    marker: PhantomData<fn() -> (ParserOk, ParserErr)>,
}

#[bon]
impl<WidgetId, ParserOk, ParserErr, OnSuccess> TextInput<WidgetId, ParserOk, ParserErr, OnSuccess>
where
    WidgetId: Display,
{
    #[allow(clippy::type_complexity)]
    #[builder]
    #[must_use]
    pub fn new(
        #[builder(start_fn)] id: WidgetId,
        #[builder(field)] on_error: Option<Arc<TextInputErrorHandler<ParserErr>>>,
        #[builder(
            default = Box::new(str::parse),
            with = |parser: impl Fn(&str) -> Result<ParserOk, ParserErr> + Send + Sync + 'static| Box::new(parser)
        )]
        parser: Box<dyn Fn(&str) -> Result<ParserOk, ParserErr> + Send + Sync>,
        on_success: OnSuccess,
    ) -> Self
    where
        ParserOk: FromStr<Err = ParserErr> + Send + Sync + 'static,
        ParserErr: Send + 'static,
        OnSuccess: AsyncFn(TextInputContext, ParserOk) -> ButtonAction
            + AsyncFn2<TextInputContext, ParserOk, Output = ButtonAction>,
        <OnSuccess as AsyncFn2<TextInputContext, ParserOk>>::OutputFuture: Send + 'static,
    {
        Self {
            id,
            parser,
            on_success,
            on_error,
            marker: PhantomData,
        }
    }

    #[cfg(test)]
    pub(crate) fn value(&self, ctx: &Context) -> Option<ParserOk> {
        let unparsed_value = ctx.widget_value_as::<String>(&self.id.to_string())?;
        (self.parser)(&unparsed_value).ok()
    }
}

impl<WidgetId, ParserOk, ParserErr, OnSuccess, S>
    TextInputBuilder<WidgetId, ParserOk, ParserErr, OnSuccess, S>
where
    S: text_input_builder::State,
    WidgetId: Display,
    ParserOk: FromStr<Err = ParserErr> + Send + Sync + 'static,
    ParserErr: Send + 'static,
    OnSuccess: AsyncFn(TextInputContext, ParserOk) -> ButtonAction
        + AsyncFn2<TextInputContext, ParserOk, Output = ButtonAction>,
    <OnSuccess as AsyncFn2<TextInputContext, ParserOk>>::OutputFuture: Send + 'static,
{
    pub fn on_error<F>(mut self, on_error: F) -> Self
    where
        ParserErr: Send + 'static,
        F: AsyncFn(TextInputContext, ParserErr) -> ButtonAction
            + AsyncFn2<TextInputContext, ParserErr, Output = ButtonAction>
            + Send
            + Sync
            + 'static,
        <F as AsyncFn2<TextInputContext, ParserErr>>::OutputFuture: Send + 'static,
    {
        let on_error = Arc::new(on_error);
        self.on_error = Some(Arc::new(move |ctx, err| {
            let on_error = on_error.clone();
            Box::pin(async move { on_error(ctx, err).await })
        }));
        self
    }
}

impl<WidgetId, ParserOk, ParserErr, OnSuccess> Input
    for TextInput<WidgetId, ParserOk, ParserErr, OnSuccess>
where
    WidgetId: Display + Send + Sync + 'static,
    ParserOk: Send + 'static,
    ParserErr: Send + 'static,
    OnSuccess: AsyncFn(TextInputContext, ParserOk) -> ButtonAction
        + AsyncFn2<TextInputContext, ParserOk, Output = ButtonAction>
        + Send
        + Sync
        + 'static,
    <OnSuccess as AsyncFn2<TextInputContext, ParserOk>>::OutputFuture: Send + 'static,
{
    fn handle_message<'a>(
        &'a self,
        ctx: &'a Context,
        message: Message,
    ) -> BoxFuture<'a, Option<ButtonAction>> {
        Box::pin(async move {
            let text = message.text()?.to_owned();
            match (self.parser)(&text) {
                Ok(value) => Some(ButtonAction::chain([
                    ButtonAction::set_widget_value(self.id.to_string(), text),
                    (self.on_success)(
                        TextInputContext {
                            context: Arc::new(ctx.clone()),
                        },
                        value,
                    )
                    .await,
                ])),
                Err(err) => match &self.on_error {
                    Some(on_error) => Some(
                        on_error(
                            TextInputContext {
                                context: Arc::new(ctx.clone()),
                            },
                            err,
                        )
                        .await,
                    ),
                    None => None,
                },
            }
        })
    }
}
