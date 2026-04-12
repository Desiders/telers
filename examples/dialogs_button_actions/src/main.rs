//! Checkout callback handler example for `telers-dialog`.
//!
//! Demonstrates `Button::on_click` in a cart flow where the button validates
//! dialog data before it either places the order or shows a correction message.
//!
//! Run with:
//! ```bash
//! BOT_TOKEN={your_bot_token} cargo run --package dialogs_button_actions
//! ```

use serde_json::Value;
use telers::{
    enums::UpdateType,
    errors::HandlerError,
    event::telegram::{Handler, HandlerResult},
    filters::Command,
    fsm::{MemoryStorage, Strategy::UserInChat},
    middlewares::outer::FSMContext as FSMContextMiddleware,
    Bot, Dispatcher, Router,
};
use telers_dialog::{
    dialog,
    entities::DataMap,
    widgets::{fn_text, keyboard, Button, ButtonAction, InlineKeyboard},
    window, DialogManager, DialogObserverExt, DialogRegistry, StartMode,
};
use tracing_subscriber::{fmt, layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};

const START_STATE: &str = "cart";

type Manager = DialogManager<MemoryStorage>;

fn text_value<'a>(data: &'a DataMap, key: &str, fallback: &'a str) -> &'a str {
    data.get(key).and_then(Value::as_str).unwrap_or(fallback)
}

fn cart_text(data: &DataMap) -> Box<str> {
    let delivery = text_value(data, "delivery_method", "not selected");
    let note = text_value(
        data,
        "cart_notice",
        "Choose delivery before placing the order.",
    );
    format!(
        "Coffee Subscription Checkout\n\nItem: House Blend subscription\nPrice: $12.00\nDelivery: \
         {delivery}\n\n{note}\n\n[Handler] `Button::on_click` reads dialog data and decides \
         whether to place the order or show a validation message."
    )
    .into_boxed_str()
}

fn order_placed_text(data: &DataMap) -> Box<str> {
    let delivery = text_value(data, "delivery_method", "not selected");
    format!(
        "Order Placed\n\nItem: House Blend subscription\nDelivery: {delivery}\n\nThe checkout \
         handler accepted the order because the required delivery option was \
         selected.\n\n[Handler] The same button would keep the user on the cart screen if \
         delivery was missing."
    )
    .into_boxed_str()
}

async fn handle_start(bot: Bot, manager: Manager) -> HandlerResult<()> {
    let _ = manager
        .start(
            &bot,
            START_STATE.to_owned(),
            Value::Null,
            StartMode::ResetStack,
        )
        .await
        .map_err(HandlerError::new)?;
    Ok(())
}

fn registry() -> DialogRegistry {
    let dialog = dialog([
        window(
            "cart",
            [
                fn_text(cart_text),
                keyboard(
                    InlineKeyboard::builder()
                        .row([Button::switch_to(
                            "select_delivery_method",
                            "Choose delivery",
                            "delivery_method",
                        )])
                        .row([Button::on_click("place_order", "Place order", |click| {
                            if click.dialog_data().get("delivery_method").is_none() {
                                ButtonAction::set_dialog_value(
                                    "cart_notice",
                                    "Select pickup or courier delivery before placing the order.",
                                )
                            } else {
                                ButtonAction::chain([
                                    ButtonAction::set_dialog_value(
                                        "cart_notice",
                                        "Order accepted.",
                                    ),
                                    ButtonAction::switch_to("order_placed"),
                                ])
                            }
                        })])
                        .row([Button::done("close", "Close draft")])
                        .build(),
                ),
            ],
        ),
        window(
            "delivery_method",
            [
                fn_text(|data: &DataMap| {
                    let delivery = text_value(data, "delivery_method", "not selected");
                    format!(
                        "Delivery Method\n\nCurrent delivery: {delivery}\n\nChoose how the \
                         subscription should arrive. The button writes the selected method and \
                         returns to the cart.\n\n[Action] `ButtonAction::chain` stores the value \
                         and switches back to the main screen."
                    )
                    .into_boxed_str()
                }),
                keyboard(
                    InlineKeyboard::builder()
                        .row([Button::action(
                            "pickup",
                            "Pickup from cafe",
                            ButtonAction::chain([
                                ButtonAction::set_dialog_value("delivery_method", "pickup"),
                                ButtonAction::set_dialog_value(
                                    "cart_notice",
                                    "Pickup selected. The order is ready to place.",
                                ),
                                ButtonAction::switch_to("cart"),
                            ]),
                        )])
                        .row([Button::action(
                            "courier",
                            "Courier delivery",
                            ButtonAction::chain([
                                ButtonAction::set_dialog_value("delivery_method", "courier"),
                                ButtonAction::set_dialog_value(
                                    "cart_notice",
                                    "Courier delivery selected. The order is ready to place.",
                                ),
                                ButtonAction::switch_to("cart"),
                            ]),
                        )])
                        .row([Button::back("back", "Back to cart")])
                        .build(),
                ),
            ],
        ),
        window(
            "order_placed",
            [
                fn_text(order_placed_text),
                keyboard(
                    InlineKeyboard::builder()
                        .row([Button::switch_to("back_to_cart", "Back to cart", "cart")])
                        .row([Button::done("close", "Close")])
                        .build(),
                ),
            ],
        ),
    ]);

    DialogRegistry::new().register(dialog).unwrap()
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::new("info,telers_dialog=trace"))
        .init();

    let bot = Bot::from_env();
    let storage = MemoryStorage::new();
    let registry = registry();

    let router = Router::new("dialogs_button_actions")
        .on_update(|observer| {
            observer
                .register_outer_middleware(FSMContextMiddleware::new(storage).strategy(UserInChat))
        })
        .on_message(|observer| {
            observer
                .register(Handler::new(handle_start).filter(Command::one("start")))
                .setup_dialogs::<MemoryStorage>()
        })
        .on_callback_query(DialogObserverExt::setup_dialogs::<MemoryStorage>);

    let dispatcher = Dispatcher::builder()
        .main_router(router.configure_default())
        .bot(bot)
        .extension(registry)
        .allowed_updates([UpdateType::Message, UpdateType::CallbackQuery])
        .build();

    match dispatcher.run_polling().await {
        Ok(()) => tracing::info!("Bot stopped"),
        Err(err) => tracing::error!(error = %err, "Bot stopped"),
    }
}
