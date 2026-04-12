use bon::bon;
use std::{fmt::Display, marker::PhantomData, str::FromStr};

use telers::types::Message;

use super::Input;
use crate::{entities::Context, widgets::ButtonAction};

pub struct TextInput<WidgetId, ParserOk, ParserErr, OnSuccess> {
    id: WidgetId,
    #[allow(clippy::type_complexity)]
    parser: Box<dyn Fn(&str) -> Result<ParserOk, ParserErr> + Send + Sync>,
    on_success: OnSuccess,
    #[allow(clippy::type_complexity)]
    on_error: Option<Box<dyn Fn(&Context, ParserErr) -> ButtonAction + Send + Sync>>,
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
        #[builder(
            default = Box::new(str::parse),
            with = |parser: impl Fn(&str) -> Result<ParserOk, ParserErr> + Send + Sync + 'static| Box::new(parser)
        )]
        parser: Box<dyn Fn(&str) -> Result<ParserOk, ParserErr> + Send + Sync>,
        on_success: OnSuccess,
        #[builder(with = |on_error: impl Fn(&Context, ParserErr) -> ButtonAction + Send + Sync + 'static| Box::new(on_error))]
        on_error: Option<Box<dyn Fn(&Context, ParserErr) -> ButtonAction + Send + Sync>>,
    ) -> Self
    where
        ParserOk: FromStr<Err = ParserErr> + Send + Sync + 'static,
        OnSuccess: Fn(&Context, ParserOk) -> ButtonAction,
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

impl<WidgetId, ParserOk, ParserErr, OnSuccess> Input
    for TextInput<WidgetId, ParserOk, ParserErr, OnSuccess>
where
    WidgetId: Display + Send + Sync + 'static,
    ParserOk: 'static,
    ParserErr: 'static,
    OnSuccess: Fn(&Context, ParserOk) -> ButtonAction + Send + Sync + 'static,
{
    fn handle_message(&self, ctx: &Context, message: Message) -> Option<ButtonAction> {
        let text = message.text()?.to_owned();
        match (self.parser)(&text) {
            Ok(value) => Some(ButtonAction::chain([
                ButtonAction::set_widget_value(self.id.to_string(), text),
                (self.on_success)(ctx, value),
            ])),
            Err(err) => self.on_error.as_ref().map(|on_error| on_error(ctx, err)),
        }
    }
}
