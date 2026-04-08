use bon::bon;
use std::{fmt::Display, marker::PhantomData};
use telers::types::{InlineKeyboardButton, InlineKeyboardMarkup, ReplyMarkup};
use tracing::debug;

use super::{
    kbd::{format_callback_data, parse_callback_data},
    Button,
};
use crate::entities::{Context, DataMap};

use super::{kbd::render_button_row, ButtonAction, Keyboard};

pub struct Radio<
    WidgetId,
    ItemsGetter,
    ItemsIter,
    Item,
    CheckedRenderer,
    UncheckedRenderer,
    ItemStr,
    IdGetter,
    Id,
> {
    id: WidgetId,
    items_getter: ItemsGetter,
    checked_renderer: CheckedRenderer,
    unchecked_renderer: UncheckedRenderer,
    id_getter: IdGetter,
    header_rows: Vec<Vec<Button>>,
    footer_rows: Vec<Vec<Button>>,
    #[allow(clippy::type_complexity)]
    marker: PhantomData<fn() -> (ItemsIter, Item, ItemStr, Id)>,
}

#[bon]
impl<
        WidgetId,
        ItemsGetter,
        ItemsIter,
        Item,
        CheckedRenderer,
        UncheckedRenderer,
        ItemStr,
        IdGetter,
        Id,
    >
    Radio<
        WidgetId,
        ItemsGetter,
        ItemsIter,
        Item,
        CheckedRenderer,
        UncheckedRenderer,
        ItemStr,
        IdGetter,
        Id,
    >
{
    #[builder]
    #[must_use]
    pub fn new(
        #[builder(start_fn)] id: WidgetId,
        #[builder(field)] header_rows: Vec<Vec<Button>>,
        #[builder(field)] footer_rows: Vec<Vec<Button>>,
        items_getter: ItemsGetter,
        checked_renderer: CheckedRenderer,
        unchecked_renderer: UncheckedRenderer,
        id_getter: IdGetter,
    ) -> Self
    where
        WidgetId: Display,
        ItemsGetter: Fn(&DataMap) -> ItemsIter,
        ItemsIter: IntoIterator<Item = Item>,
        CheckedRenderer: Fn(&Item, &DataMap) -> ItemStr,
        UncheckedRenderer: Fn(&Item, &DataMap) -> ItemStr,
        ItemStr: Into<Box<str>>,
        IdGetter: Fn(&Item) -> Id,
        Id: Display,
    {
        Self {
            id,
            items_getter,
            checked_renderer,
            unchecked_renderer,
            id_getter,
            header_rows,
            footer_rows,
            marker: PhantomData,
        }
    }
}

impl<
        S,
        WidgetId,
        ItemsGetter,
        ItemsIter,
        Item,
        CheckedRenderer,
        UncheckedRenderer,
        ItemStr,
        IdGetter,
        Id,
    >
    RadioBuilder<
        WidgetId,
        ItemsGetter,
        ItemsIter,
        Item,
        CheckedRenderer,
        UncheckedRenderer,
        ItemStr,
        IdGetter,
        Id,
        S,
    >
where
    S: radio_builder::State,
    WidgetId: Display,
    ItemsGetter: Fn(&DataMap) -> ItemsIter,
    ItemsIter: IntoIterator<Item = Item>,
    CheckedRenderer: Fn(&Item, &DataMap) -> ItemStr,
    UncheckedRenderer: Fn(&Item, &DataMap) -> ItemStr,
    ItemStr: Into<Box<str>>,
    IdGetter: Fn(&Item) -> Id,
    Id: Display,
{
    pub fn header_row(mut self, buttons: impl IntoIterator<Item = Button>) -> Self {
        self.header_rows.push(buttons.into_iter().collect());
        self
    }

    pub fn header_push(mut self, button: Button) -> Self {
        match self.header_rows.last_mut() {
            Some(row) => row.push(button),
            None => self.header_rows.push(vec![button]),
        }
        self
    }

    pub fn footer_row(mut self, buttons: impl IntoIterator<Item = Button>) -> Self {
        self.footer_rows.push(buttons.into_iter().collect());
        self
    }

    pub fn footer_push(mut self, button: Button) -> Self {
        match self.footer_rows.last_mut() {
            Some(row) => row.push(button),
            None => self.footer_rows.push(vec![button]),
        }
        self
    }
}

impl<
        WidgetId,
        ItemsGetter,
        ItemsIter,
        Item,
        CheckedRenderer,
        UncheckedRenderer,
        ItemStr,
        IdGetter,
        Id,
    > Keyboard
    for Radio<
        WidgetId,
        ItemsGetter,
        ItemsIter,
        Item,
        CheckedRenderer,
        UncheckedRenderer,
        ItemStr,
        IdGetter,
        Id,
    >
where
    WidgetId: Display + Send + Sync + 'static,
    ItemsGetter: Fn(&DataMap) -> ItemsIter + Send + Sync + 'static,
    ItemsIter: IntoIterator<Item = Item> + 'static,
    Item: 'static,
    CheckedRenderer: Fn(&Item, &DataMap) -> ItemStr + Send + Sync + 'static,
    UncheckedRenderer: Fn(&Item, &DataMap) -> ItemStr + Send + Sync + 'static,
    ItemStr: Into<Box<str>> + 'static,
    IdGetter: Fn(&Item) -> Id + Send + Sync + 'static,
    Id: Display + Send + Sync + 'static,
{
    fn render_keyboard(&self, ctx: &Context, data: &DataMap) -> Option<ReplyMarkup> {
        let widget_id = self.id.to_string();
        let checked: Option<String> = ctx.widget_value_as(&widget_id);

        let mut rows: Vec<_> = self
            .header_rows
            .iter()
            .map(|row| render_button_row(row, ctx, data))
            .collect();

        for item in (self.items_getter)(data) {
            let item_id = (self.id_getter)(&item).to_string();
            let is_checked = checked.as_deref() == Some(item_id.as_str());
            let text = if is_checked {
                (self.checked_renderer)(&item, data)
            } else {
                (self.unchecked_renderer)(&item, data)
            };
            rows.push(
                [
                    InlineKeyboardButton::new(text).callback_data(format_callback_data(
                        ctx,
                        &self.id,
                        Some(&item_id),
                    )),
                ]
                .into(),
            );
        }

        rows.extend(
            self.footer_rows
                .iter()
                .map(|row| render_button_row(row, ctx, data)),
        );

        if rows.is_empty() {
            None
        } else {
            Some(InlineKeyboardMarkup::new(rows).into())
        }
    }

    fn handle_callback(&self, ctx: &Context, callback_data: &str) -> Option<ButtonAction> {
        if let Some(action) = self
            .header_rows
            .iter()
            .chain(self.footer_rows.iter())
            .flat_map(|row| row.iter())
            .find_map(|button| button.resolve_callback(ctx, callback_data))
        {
            return Some(action);
        }

        let parsed = parse_callback_data(ctx, callback_data)?;
        if parsed.target_id != self.id.to_string() {
            return None;
        }
        let payload = parsed.payload?;
        debug!(
            context_id = %ctx.id,
            widget_id = %self.id,
            item_id = payload,
            "Resolved radio selection callback"
        );
        Some(ButtonAction::set_widget_value(self.id.to_string(), payload))
    }
}

pub struct Multiselect<
    WidgetId,
    ItemsGetter,
    ItemsIter,
    Item,
    CheckedRenderer,
    UncheckedRenderer,
    ItemStr,
    IdGetter,
    Id,
> {
    id: WidgetId,
    items_getter: ItemsGetter,
    checked_renderer: CheckedRenderer,
    unchecked_renderer: UncheckedRenderer,
    id_getter: IdGetter,
    min_selected: usize,
    max_selected: usize,
    header_rows: Vec<Vec<Button>>,
    footer_rows: Vec<Vec<Button>>,
    #[allow(clippy::type_complexity)]
    marker: PhantomData<fn() -> (ItemsIter, Item, ItemStr, Id)>,
}

#[bon]
impl<
        WidgetId,
        ItemsGetter,
        ItemsIter,
        Item,
        CheckedRenderer,
        UncheckedRenderer,
        ItemStr,
        IdGetter,
        Id,
    >
    Multiselect<
        WidgetId,
        ItemsGetter,
        ItemsIter,
        Item,
        CheckedRenderer,
        UncheckedRenderer,
        ItemStr,
        IdGetter,
        Id,
    >
{
    #[builder]
    #[must_use]
    pub fn new(
        #[builder(start_fn)] id: WidgetId,
        #[builder(field)] header_rows: Vec<Vec<Button>>,
        #[builder(field)] footer_rows: Vec<Vec<Button>>,
        items_getter: ItemsGetter,
        checked_renderer: CheckedRenderer,
        unchecked_renderer: UncheckedRenderer,
        id_getter: IdGetter,
        #[builder(default = 0)] min_selected: usize,
        #[builder(default = 0)] max_selected: usize,
    ) -> Self
    where
        WidgetId: Display,
        ItemsGetter: Fn(&DataMap) -> ItemsIter,
        ItemsIter: IntoIterator<Item = Item>,
        CheckedRenderer: Fn(&Item, &DataMap) -> ItemStr,
        UncheckedRenderer: Fn(&Item, &DataMap) -> ItemStr,
        ItemStr: Into<Box<str>>,
        IdGetter: Fn(&Item) -> Id,
        Id: Display,
    {
        Self {
            id,
            items_getter,
            checked_renderer,
            unchecked_renderer,
            id_getter,
            min_selected,
            max_selected,
            header_rows,
            footer_rows,
            marker: PhantomData,
        }
    }
}

impl<
        S,
        WidgetId,
        ItemsGetter,
        ItemsIter,
        Item,
        CheckedRenderer,
        UncheckedRenderer,
        ItemStr,
        IdGetter,
        Id,
    >
    MultiselectBuilder<
        WidgetId,
        ItemsGetter,
        ItemsIter,
        Item,
        CheckedRenderer,
        UncheckedRenderer,
        ItemStr,
        IdGetter,
        Id,
        S,
    >
where
    S: multiselect_builder::State,
    WidgetId: Display,
    ItemsGetter: Fn(&DataMap) -> ItemsIter,
    ItemsIter: IntoIterator<Item = Item>,
    CheckedRenderer: Fn(&Item, &DataMap) -> ItemStr,
    UncheckedRenderer: Fn(&Item, &DataMap) -> ItemStr,
    ItemStr: Into<Box<str>>,
    IdGetter: Fn(&Item) -> Id,
    Id: Display,
{
    pub fn header_row(mut self, buttons: impl IntoIterator<Item = Button>) -> Self {
        self.header_rows.push(buttons.into_iter().collect());
        self
    }

    pub fn header_push(mut self, button: Button) -> Self {
        match self.header_rows.last_mut() {
            Some(row) => row.push(button),
            None => self.header_rows.push(vec![button]),
        }
        self
    }

    pub fn footer_row(mut self, buttons: impl IntoIterator<Item = Button>) -> Self {
        self.footer_rows.push(buttons.into_iter().collect());
        self
    }

    pub fn footer_push(mut self, button: Button) -> Self {
        match self.footer_rows.last_mut() {
            Some(row) => row.push(button),
            None => self.footer_rows.push(vec![button]),
        }
        self
    }
}

impl<
        WidgetId,
        ItemsGetter,
        ItemsIter,
        Item,
        CheckedRenderer,
        UncheckedRenderer,
        ItemStr,
        IdGetter,
        Id,
    > Keyboard
    for Multiselect<
        WidgetId,
        ItemsGetter,
        ItemsIter,
        Item,
        CheckedRenderer,
        UncheckedRenderer,
        ItemStr,
        IdGetter,
        Id,
    >
where
    WidgetId: Display + Send + Sync + 'static,
    ItemsGetter: Fn(&DataMap) -> ItemsIter + Send + Sync + 'static,
    ItemsIter: IntoIterator<Item = Item> + 'static,
    Item: 'static,
    CheckedRenderer: Fn(&Item, &DataMap) -> ItemStr + Send + Sync + 'static,
    UncheckedRenderer: Fn(&Item, &DataMap) -> ItemStr + Send + Sync + 'static,
    ItemStr: Into<Box<str>> + 'static,
    IdGetter: Fn(&Item) -> Id + Send + Sync + 'static,
    Id: Display + Send + Sync + 'static,
{
    fn render_keyboard(&self, ctx: &Context, data: &DataMap) -> Option<ReplyMarkup> {
        let widget_id = self.id.to_string();
        let checked = read_checked_list(ctx, &widget_id);

        let mut rows: Vec<_> = self
            .header_rows
            .iter()
            .map(|row| render_button_row(row, ctx, data))
            .collect();

        for item in (self.items_getter)(data) {
            let item_id = (self.id_getter)(&item).to_string();
            let is_checked = checked.iter().any(|id| id == &item_id);
            let text = if is_checked {
                (self.checked_renderer)(&item, data)
            } else {
                (self.unchecked_renderer)(&item, data)
            };
            rows.push(
                [
                    InlineKeyboardButton::new(text).callback_data(format_callback_data(
                        ctx,
                        &self.id,
                        Some(&item_id),
                    )),
                ]
                .into(),
            );
        }

        rows.extend(
            self.footer_rows
                .iter()
                .map(|row| render_button_row(row, ctx, data)),
        );

        if rows.is_empty() {
            None
        } else {
            Some(InlineKeyboardMarkup::new(rows).into())
        }
    }

    fn handle_callback(&self, ctx: &Context, callback_data: &str) -> Option<ButtonAction> {
        if let Some(action) = self
            .header_rows
            .iter()
            .chain(self.footer_rows.iter())
            .flat_map(|row| row.iter())
            .find_map(|button| button.resolve_callback(ctx, callback_data))
        {
            return Some(action);
        }

        let parsed = parse_callback_data(ctx, callback_data)?;
        if parsed.target_id != self.id.to_string() {
            return None;
        }
        let payload = parsed.payload?;
        let widget_id = self.id.to_string();
        let mut checked = read_checked_list(ctx, &widget_id);

        if let Some(pos) = checked.iter().position(|id| id == payload) {
            if self.min_selected > 0 && checked.len() <= self.min_selected {
                debug!(
                    context_id = %ctx.id,
                    widget_id = %self.id,
                    item_id = payload,
                    min = self.min_selected,
                    "Multiselect min_selected constraint prevents uncheck"
                );
                return Some(ButtonAction::noop());
            }
            checked.remove(pos);
        } else {
            if self.max_selected > 0 && checked.len() >= self.max_selected {
                debug!(
                    context_id = %ctx.id,
                    widget_id = %self.id,
                    item_id = payload,
                    max = self.max_selected,
                    "Multiselect max_selected constraint prevents check"
                );
                return Some(ButtonAction::noop());
            }
            checked.push(payload.to_owned());
        }

        debug!(
            context_id = %ctx.id,
            widget_id = %self.id,
            item_id = payload,
            count = checked.len(),
            "Resolved multiselect toggle callback"
        );
        Some(ButtonAction::set_widget_value(
            widget_id,
            serde_json::Value::Array(checked.into_iter().map(serde_json::Value::String).collect()),
        ))
    }
}

#[inline]
#[must_use]
fn read_checked_list(ctx: &Context, widget_id: &str) -> Vec<String> {
    ctx.widget_value_as::<Vec<String>>(widget_id)
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use serde_json::{json, Value};

    use super::{Multiselect, Radio};
    use crate::{
        entities::{Context, DataMap},
        widgets::{Button, ButtonAction, Keyboard},
    };

    #[test]
    fn radio_renders_checked_and_unchecked_items() {
        let ctx = Context::new("", "state", Value::Null);
        let radio = Radio::builder("color")
            .items_getter(|_data| ["red", "blue", "green"])
            .checked_renderer(|&item, _data| format!("* {item}"))
            .unchecked_renderer(|&item, _data| item.to_owned())
            .id_getter(|&item| item)
            .build();

        let markup = radio.render_keyboard(&ctx, &DataMap::new()).unwrap();
        let rows = markup.inline_keyboard().unwrap();

        assert_eq!(&*rows[0][0].text, "red");
        assert_eq!(&*rows[1][0].text, "blue");
    }

    #[test]
    fn radio_renders_selected_item_as_checked() {
        let mut ctx = Context::new("", "state", Value::Null);
        ctx.widget_data.insert("color".into(), json!("blue"));

        let radio = Radio::builder("color")
            .items_getter(|_data| ["red", "blue", "green"])
            .checked_renderer(|&item, _data| format!("* {item}"))
            .unchecked_renderer(|&item, _data| item.to_owned())
            .id_getter(|&item| item)
            .build();

        let markup = radio.render_keyboard(&ctx, &DataMap::new()).unwrap();
        let rows = markup.inline_keyboard().unwrap();

        assert_eq!(&*rows[0][0].text, "red");
        assert_eq!(&*rows[1][0].text, "* blue");
        assert_eq!(&*rows[2][0].text, "green");
    }

    #[test]
    fn radio_callback_produces_set_widget_value() {
        let ctx = Context::new("", "state", Value::Null);
        let radio = Radio::builder("color")
            .items_getter(|_data| ["red", "blue"])
            .checked_renderer(|&item, _data| format!("* {item}"))
            .unchecked_renderer(|&item, _data| item.to_owned())
            .id_getter(|&item| item)
            .build();

        let action = radio
            .handle_callback(&ctx, &format!("td:{}:color:blue", ctx.id))
            .unwrap();

        assert!(matches!(
            action,
            ButtonAction::SetWidgetValue { ref key, ref value }
                if key.as_ref() == "color" && value == "blue"
        ));
    }

    #[test]
    fn radio_ignores_foreign_intent_callbacks() {
        let ctx = Context::new("", "state", Value::Null);
        let radio = Radio::builder("color")
            .items_getter(|_data| ["red"])
            .checked_renderer(|&item, _data| item.to_owned())
            .unchecked_renderer(|&item, _data| item.to_owned())
            .id_getter(|&item| item)
            .build();

        assert!(radio.handle_callback(&ctx, "td:other:color:red").is_none());
    }

    #[test]
    fn radio_allows_header_and_footer_buttons() {
        let ctx = Context::new("", "state", Value::Null);
        let radio = Radio::builder("color")
            .items_getter(|_data| ["red"])
            .checked_renderer(|&item, _data| item.to_owned())
            .unchecked_renderer(|&item, _data| item.to_owned())
            .id_getter(|&item| item)
            .footer_push(Button::done("done", "Done"))
            .build();

        let action = radio
            .handle_callback(&ctx, &format!("td:{}:done", ctx.id))
            .unwrap();

        assert!(matches!(action, ButtonAction::Done));
    }

    #[test]
    fn multiselect_renders_checked_and_unchecked_items() {
        let mut ctx = Context::new("", "state", Value::Null);
        ctx.widget_data.insert("fruits".into(), json!(["apple"]));

        let ms = Multiselect::builder("fruits")
            .items_getter(|_data| ["apple", "pear", "grape"])
            .checked_renderer(|&item, _data| format!("[x] {item}"))
            .unchecked_renderer(|&item, _data| format!("[ ] {item}"))
            .id_getter(|&item| item)
            .build();

        let markup = ms.render_keyboard(&ctx, &DataMap::new()).unwrap();
        let rows = markup.inline_keyboard().unwrap();

        assert_eq!(&*rows[0][0].text, "[x] apple");
        assert_eq!(&*rows[1][0].text, "[ ] pear");
        assert_eq!(&*rows[2][0].text, "[ ] grape");
    }

    #[test]
    fn multiselect_toggle_checks_unchecked_item() {
        let ctx = Context::new("", "state", Value::Null);
        let ms = Multiselect::builder("fruits")
            .items_getter(|_data| ["apple", "pear"])
            .checked_renderer(|&item, _data| format!("[x] {item}"))
            .unchecked_renderer(|&item, _data| format!("[ ] {item}"))
            .id_getter(|&item| item)
            .build();

        let action = ms
            .handle_callback(&ctx, &format!("td:{}:fruits:apple", ctx.id))
            .unwrap();

        assert!(matches!(
            action,
            ButtonAction::SetWidgetValue { ref key, ref value }
                if key.as_ref() == "fruits" && value == &json!(["apple"])
        ));
    }

    #[test]
    fn multiselect_toggle_unchecks_checked_item() {
        let mut ctx = Context::new("", "state", Value::Null);
        ctx.widget_data
            .insert("fruits".into(), json!(["apple", "pear"]));

        let ms = Multiselect::builder("fruits")
            .items_getter(|_data| ["apple", "pear"])
            .checked_renderer(|&item, _data| format!("[x] {item}"))
            .unchecked_renderer(|&item, _data| format!("[ ] {item}"))
            .id_getter(|&item| item)
            .build();

        let action = ms
            .handle_callback(&ctx, &format!("td:{}:fruits:apple", ctx.id))
            .unwrap();

        assert!(matches!(
            action,
            ButtonAction::SetWidgetValue { ref key, ref value }
                if key.as_ref() == "fruits" && value == &json!(["pear"])
        ));
    }

    #[test]
    fn multiselect_respects_max_selected() {
        let mut ctx = Context::new("", "state", Value::Null);
        ctx.widget_data
            .insert("fruits".into(), json!(["apple", "pear"]));

        let ms = Multiselect::builder("fruits")
            .items_getter(|_data| ["apple", "pear", "grape"])
            .checked_renderer(|&item, _data| format!("[x] {item}"))
            .unchecked_renderer(|&item, _data| format!("[ ] {item}"))
            .id_getter(|&item| item)
            .max_selected(2)
            .build();

        let action = ms
            .handle_callback(&ctx, &format!("td:{}:fruits:grape", ctx.id))
            .unwrap();

        assert!(matches!(action, ButtonAction::Noop));
    }

    #[test]
    fn multiselect_respects_min_selected() {
        let mut ctx = Context::new("", "state", Value::Null);
        ctx.widget_data.insert("fruits".into(), json!(["apple"]));

        let ms = Multiselect::builder("fruits")
            .items_getter(|_data| ["apple", "pear"])
            .checked_renderer(|&item, _data| format!("[x] {item}"))
            .unchecked_renderer(|&item, _data| format!("[ ] {item}"))
            .id_getter(|&item| item)
            .min_selected(1)
            .build();

        let action = ms
            .handle_callback(&ctx, &format!("td:{}:fruits:apple", ctx.id))
            .unwrap();

        assert!(matches!(action, ButtonAction::Noop));
    }
}
