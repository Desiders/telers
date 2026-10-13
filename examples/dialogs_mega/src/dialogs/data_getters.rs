use telers::serialization::json;
use telers_dialog::{
    entities::{DataMap, RenderContext},
    widgets::{format_text, keyboard, Button, InlineKeyboard, WhenCondition},
    window, DataGetter, Dialog,
};

pub const STATE: &str = "data_getters";

pub fn dialog() -> impl Dialog {
    telers_dialog::dialog([window(
        STATE,
        [
            format_text("Hello {profile[name]}!\n{summary}\n{{Derived data is not persisted.}}"),
            keyboard(InlineKeyboard::builder()
                .push(Button::action("refresh", "Refresh", telers_dialog::widgets::ButtonAction::noop()))
                .push(Button::done("leave", "Main menu").when(WhenCondition::data_field("can_leave")))
                .build()),
        ],
    )
    .getter(DataGetter::new(|render: RenderContext| async move {
        let name = render.data["profile"]["name"].as_str().unwrap_or("visitor");
        Ok(DataMap::from([
            ("summary".into(), json!(format!("Window getter received {name} from the dialog getter."))),
            ("can_leave".into(), json!(true)),
        ]))
    }))])
    .getter(DataGetter::new(|render: RenderContext| async move {
        let name = render.event_context.user.first_name.as_ref();
        Ok(DataMap::from([("profile".into(), json!({"name": name}))]))
    }))
}
