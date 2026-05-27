//! Button action handlers: `ButtonAction::action`/`chain` for declarative
//! transitions, and `Button::on_click` for an async handler that validates
//! dialog data before deciding what to do.

use serde_json::Value;
use telers_dialog::{
    entities::DataMap,
    widgets::{fn_text, keyboard, Button, ButtonAction, InlineKeyboard},
    window, Dialog,
};

use crate::common::main_menu_button;

pub const STATE: &str = "actions_cart";

fn value<'a>(data: &'a DataMap, key: &str, fallback: &'a str) -> &'a str {
    data.get(key).and_then(Value::as_str).unwrap_or(fallback)
}

pub fn dialog() -> impl Dialog {
    telers_dialog::dialog([
        window(
            STATE,
            [
                fn_text(|data: &DataMap| {
                    let delivery = value(data, "delivery_method", "not selected");
                    let notice = value(data, "cart_notice", "Choose delivery before ordering.");
                    format!(
                        "Checkout\n\nItem: House Blend subscription\nDelivery: \
                         {delivery}\n\n{notice}\n\n[Handler] `Button::on_click` validates dialog \
                         data and decides whether to place the order."
                    )
                }),
                keyboard(
                    InlineKeyboard::builder()
                        .row([Button::switch_to(
                            "choose_delivery",
                            "Choose delivery",
                            "actions_delivery",
                        )])
                        .row([Button::on_click(
                            "place_order",
                            "Place order",
                            |click| async move {
                                if click.dialog_data().get("delivery_method").is_none() {
                                    ButtonAction::set_dialog_value(
                                        "cart_notice",
                                        "Select a delivery option before placing the order.",
                                    )
                                } else {
                                    ButtonAction::chain([
                                        ButtonAction::set_dialog_value(
                                            "cart_notice",
                                            "Order accepted.",
                                        ),
                                        ButtonAction::switch_to("actions_done"),
                                    ])
                                }
                            },
                        )])
                        .row([main_menu_button()])
                        .build(),
                ),
            ],
        ),
        window(
            "actions_delivery",
            [
                fn_text(|data: &DataMap| {
                    let delivery = value(data, "delivery_method", "not selected");
                    format!(
                        "Delivery method\n\nCurrent: {delivery}\n\n[Action] `ButtonAction::chain` \
                         stores the value with `extend_dialog_data` and switches back to the cart."
                    )
                }),
                keyboard(
                    InlineKeyboard::builder()
                        .row([Button::action(
                            "pickup",
                            "Pickup from cafe",
                            ButtonAction::chain([
                                ButtonAction::extend_dialog_data([
                                    ("delivery_method", "pickup"),
                                    ("cart_notice", "Pickup selected. Ready to place the order."),
                                ]),
                                ButtonAction::switch_to(STATE),
                            ]),
                        )])
                        .row([Button::action(
                            "courier",
                            "Courier delivery",
                            ButtonAction::chain([
                                ButtonAction::extend_dialog_data([
                                    ("delivery_method", "courier"),
                                    ("cart_notice", "Courier selected. Ready to place the order."),
                                ]),
                                ButtonAction::switch_to(STATE),
                            ]),
                        )])
                        .row([Button::back("back", "Back to cart")])
                        .build(),
                ),
            ],
        ),
        window(
            "actions_done",
            [
                fn_text(|data: &DataMap| {
                    let delivery = value(data, "delivery_method", "not selected");
                    format!(
                        "Order placed\n\nItem: House Blend subscription\nDelivery: \
                         {delivery}\n\nThe handler accepted the order because the required \
                         delivery option was selected."
                    )
                }),
                keyboard(
                    InlineKeyboard::builder()
                        .row([Button::switch_to("back_to_cart", "Back to cart", STATE)])
                        .row([main_menu_button()])
                        .build(),
                ),
            ],
        ),
    ])
}
