//! Button helper examples for `telers-dialog`.
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
    widgets::{format_text, keyboard, text, Button, ButtonAction, InlineKeyboard},
    window, DialogManager, DialogObserverExt, DialogRegistry, StartMode,
};
use tracing_subscriber::{fmt, layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};

const START_STATE: &str = "cart";

type Manager = DialogManager<MemoryStorage>;

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
    let checkout_dialog = dialog([
        window(
            "cart",
            [
                format_text(
                    "Checkout Draft\n\nItem: House Blend\nPrice: $12.00\nDelivery: \
                     {delivery}\nOrder status: {order_status}\n\n[Helpers] `next`, `start`, `done`",
                ),
                keyboard(
                    InlineKeyboard::builder()
                        .row([Button::next("delivery_step", "Choose delivery")])
                        .row([Button::start(
                            "confirm",
                            "Confirm order",
                            "confirm_order",
                            Value::Null,
                            StartMode::Normal,
                        )])
                        .row([Button::done("close", "Close draft")])
                        .build(),
                ),
            ],
        ),
        window(
            "delivery",
            [
                format_text(
                    "Delivery Step\n\nCurrent delivery: {delivery}\n\nChoose how the order should \
                     arrive.\n\n[Helpers] `set_dialog_value`, `switch_to`, `back`",
                ),
                keyboard(
                    InlineKeyboard::builder()
                        .row([Button::action(
                            "pickup",
                            "Pickup",
                            ButtonAction::chain([
                                ButtonAction::set_dialog_value("delivery", "pickup"),
                                ButtonAction::switch_to("cart"),
                            ]),
                        )])
                        .row([Button::action(
                            "courier",
                            "Courier",
                            ButtonAction::chain([
                                ButtonAction::set_dialog_value("delivery", "courier"),
                                ButtonAction::switch_to("cart"),
                            ]),
                        )])
                        .row([Button::back("back", "Back to cart")])
                        .build(),
                ),
            ],
        ),
        window(
            "done",
            [
                format_text(
                    "Order Finished\n\nItem: House Blend\nPrice: $12.00\nDelivery: \
                     {delivery}\nOrder status: {order_status}\n\n[Helper] `done`",
                ),
                keyboard(
                    InlineKeyboard::builder()
                        .row([Button::done("close", "Close")])
                        .build(),
                ),
            ],
        ),
    ])
    .on_process_result(|_ctx, _start_data, result| {
        let status = match result.as_str() {
            Some("confirmed") => "confirmed",
            Some("changed_mind") => "draft",
            _ => "draft",
        };
        let next_state = match result.as_str() {
            Some("confirmed") => "done",
            _ => "cart",
        };

        Some(ButtonAction::chain([
            ButtonAction::set_dialog_value("order_status", status),
            ButtonAction::switch_to(next_state),
        ]))
    });

    let confirm_dialog = dialog([window(
        "confirm_order",
        [
            text(
                "Confirm Order\n\nPlace the order now or return to editing.\n\n[Helper] \
                 `done_with_result`",
            ),
            keyboard(
                InlineKeyboard::builder()
                    .row([Button::done_with_result(
                        "confirm_order",
                        "Place order",
                        "confirmed",
                    )])
                    .row([Button::done_with_result(
                        "keep_editing",
                        "Keep editing",
                        "changed_mind",
                    )])
                    .build(),
            ),
        ],
    )]);

    DialogRegistry::new()
        .register(checkout_dialog)
        .unwrap()
        .register(confirm_dialog)
        .unwrap()
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
