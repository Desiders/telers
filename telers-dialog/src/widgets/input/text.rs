use bon::bon;
use std::{fmt::Display, marker::PhantomData, str::FromStr, sync::Arc};

use async_fn_traits::AsyncFn2;
use async_trait::async_trait;
use telers::types::Message;

use super::Input;
use crate::{entities::Context, future::BoxFuture, widgets::ButtonAction};

type TextInputErrorHandler<ParserErr> =
    dyn Fn(TextInputContext, ParserErr) -> BoxFuture<'static, ButtonAction> + Send + Sync;

/// Runtime inputs available to [`TextInput`] success and error handlers.
#[derive(Clone, Debug)]
pub struct TextInputContext {
    /// Stored dialog context for the active intent.
    pub context: Arc<Context>,
    pub message: Arc<Message>,
}

/// Input widget that parses message text into a typed value via [`FromStr`] or
/// a custom parser closure.
///
/// On a successful parse, the raw message text is stored in `widget_data[id]`
/// and `on_success` is invoked with the parsed value. When the parser fails,
/// the optional `on_error` handler runs; if it is not set the widget returns
/// `None` so other inputs on the window may still claim the message.
///
/// [`FromStr`]: std::str::FromStr
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
    /// Build a text-input widget bound to a widget id.
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

    /// Build with a custom parser for values which need not implement [`FromStr`].
    #[allow(clippy::type_complexity)]
    #[builder(start_fn = builder_with_parser, builder_type = TextInputWithParserBuilder, state_mod = text_input_with_parser_builder, finish_fn = build)]
    #[must_use]
    pub fn new_with_parser<Parser>(
        #[builder(start_fn)] id: WidgetId,
        #[builder(start_fn)] parser: Parser,
        #[builder(field)] on_error: Option<Arc<TextInputErrorHandler<ParserErr>>>,
        on_success: OnSuccess,
    ) -> Self
    where
        ParserOk: Send + 'static,
        ParserErr: Send + 'static,
        Parser: Fn(&str) -> Result<ParserOk, ParserErr> + Send + Sync + 'static,
        OnSuccess: AsyncFn(TextInputContext, ParserOk) -> ButtonAction
            + AsyncFn2<TextInputContext, ParserOk, Output = ButtonAction>,
        <OnSuccess as AsyncFn2<TextInputContext, ParserOk>>::OutputFuture: Send + 'static,
    {
        Self {
            id,
            parser: Box::new(parser),
            on_success,
            on_error,
            marker: PhantomData,
        }
    }

    #[must_use]
    pub fn value(&self, ctx: &Context) -> Option<ParserOk> {
        let unparsed_value = ctx.widget_value_as::<String>(&self.id.to_string())?;
        (self.parser)(&unparsed_value).ok()
    }
}

impl<WidgetId, ParserOk, ParserErr, OnSuccess, Parser, S>
    TextInputWithParserBuilder<WidgetId, ParserOk, ParserErr, OnSuccess, Parser, S>
where
    S: text_input_with_parser_builder::State,
    WidgetId: Display,
    ParserOk: Send + 'static,
    ParserErr: Send + 'static,
    Parser: Fn(&str) -> Result<ParserOk, ParserErr> + Send + Sync + 'static,
    OnSuccess: AsyncFn(TextInputContext, ParserOk) -> ButtonAction
        + AsyncFn2<TextInputContext, ParserOk, Output = ButtonAction>,
    <OnSuccess as AsyncFn2<TextInputContext, ParserOk>>::OutputFuture: Send + 'static,
{
    pub fn on_error<F>(mut self, on_error: F) -> Self
    where
        F: AsyncFn(TextInputContext, ParserErr) -> ButtonAction
            + AsyncFn2<TextInputContext, ParserErr, Output = ButtonAction>
            + Send
            + Sync
            + 'static,
        <F as AsyncFn2<TextInputContext, ParserErr>>::OutputFuture: Send + 'static,
    {
        self.on_error = Some(error_handler(on_error));
        self
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
    /// Register an async handler invoked when the parser returns `Err`.
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
        self.on_error = Some(error_handler(on_error));
        self
    }
}

fn error_handler<ParserErr, F>(on_error: F) -> Arc<TextInputErrorHandler<ParserErr>>
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
    Arc::new(move |ctx, err| {
        let on_error = on_error.clone();
        Box::pin(async move { on_error(ctx, err).await })
    })
}

#[async_trait]
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
    async fn handle_message(&self, ctx: &Context, message: Message) -> Option<ButtonAction> {
        let text = message.text()?.to_owned();
        match (self.parser)(&text) {
            Ok(value) => {
                let mut input_context = ctx.clone();
                input_context.widget_data.insert(self.id.to_string(), text.clone().into());
                Some(ButtonAction::chain([
                    ButtonAction::set_widget_value(self.id.to_string(), text),
                    (self.on_success)(
                        TextInputContext {
                            context: Arc::new(input_context),
                            message: Arc::new(message),
                        },
                        value,
                    )
                    .await,
                ]))
            }
            Err(err) => match &self.on_error {
                Some(on_error) => Some(
                    on_error(
                        TextInputContext {
                            context: Arc::new(ctx.clone()),
                            message: Arc::new(message),
                        },
                        err,
                    )
                    .await,
                ),
                None => None,
            },
        }
    }
}
