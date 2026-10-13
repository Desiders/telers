<div align="center">

# `telers-dialog`

[![telers](https://img.shields.io/crates/v/telers?style=flat-square)](https://crates.io/crates/telers)
<a href="https://core.telegram.org/bots/api" target="_blank" rel="noopener noreferrer"><picture><source media="(prefers-color-scheme: dark)" srcset="https://shieldcn.dev/badge/dynamic/json.svg?url=https://raw.githubusercontent.com/Desiders/telers/refs/heads/dev-1.x/telers-codegen/schemas/api.json&query=$.version&variant=outline&size=xs&logo=ri:FaTelegramPlane&logoColor=24A1DE&label=Telegram&mode=dark"><img alt="Telegram Bot API" src="https://shieldcn.dev/badge/dynamic/json.svg?url=https://raw.githubusercontent.com/Desiders/telers/refs/heads/dev-1.x/telers-codegen/schemas/api.json&query=$.version&variant=outline&size=xs&mode=light&theme=zinc&logo=ri:FaTelegramPlane&logoColor=24A1DE&label=Telegram"></picture></a>

### Dialog framework for [telers](https://crates.io/crates/telers)

</div>

## Highlights
 - **Windows and dialogs**. A dialog is a set of windows, each window is a set of widgets, and the dialog owns the navigation between them.
 - **Widget library**. Text, inline and reply keyboards, input fields, link previews, media, calendars, pagers and stateful widgets are included.
 - **Stack navigation**. Dialogs are kept on a stack, so `start`, `switch_to`, `next`, `back` and `done` behave like in `aiogram-dialog`.
 - **FSM storage**. Dialog data lives in `telers::fsm::Context`. Persistence follows your chosen storage backend; memory storage keeps conversations while the process runs.
 - **Extractors**. `DialogManager` is a handler argument, exactly like `FSMContext` in `telers`.
 - **Middleware integration**. One call, `setup_dialogs::<S>()`, registers the middlewares and the event handler on an observer.

## Getting started

`telers-dialog` is not on crates.io yet, so point Cargo at the repository for now:

```toml
[dependencies]
telers-dialog = { git = "https://github.com/Desiders/telers", branch = "dev-1.x" }
```

`telers-dialog` follows `telers` for serialization, so the backend features have to match the ones you enabled on `telers`. The default is serde, and the deser variant looks like this:

```toml
telers-dialog = { git = "https://github.com/Desiders/telers", branch = "dev-1.x", default-features = false, features = ["deser"] }
```

Once the crate gets published, replace the `git` part with `version = "0.1"`.

To enable logging, you may also want to add `tracing` and `tracing-subscriber`.

The optional `template` feature adds `TemplateText`, which renders messages with [MiniJinja](https://github.com/mitsuhiko/minijinja) templates.

Create a bot with [@BotFather](https://t.me/BotFather) to get a token, then put it in the `BOT_TOKEN` environment variable.

Build the registry once during startup, and put it into the dispatcher extensions. Register the `telers` FSM middleware yourself, since the storage is yours to choose:

```rust,ignore
use telers::{
    enums::UpdateType,
    errors::HandlerError,
    event::telegram::{Handler, HandlerResult},
    filters::CommandStart,
    fsm::{MemoryStorage, Strategy::UserInChat},
    middlewares::outer::FSMContext as FSMContextMiddleware,
    serialization::Value,
    Bot, Dispatcher, Router,
};
use telers_dialog::{DialogManager, DialogObserverExt, DialogRegistry, StartMode};

type Manager = DialogManager<MemoryStorage>;

/// `/start` resets the whole stack and shows the main menu.
async fn handle_start(bot: Bot, manager: Manager) -> HandlerResult<()> {
    manager
        .start(&bot, "main_menu".to_owned(), Value::default(), StartMode::ResetStack)
        .await
        .map_err(HandlerError::new)?;
    Ok(())
}

fn registry() -> DialogRegistry {
    DialogRegistry::new()
        .register(main_menu::dialog())
        .expect("all dialogs register without duplicate states")
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::fmt().init();

    let bot = Bot::from_env();
    let storage = MemoryStorage::new();
    let registry = registry();

    let router = Router::new("main")
        .on_update(|observer| {
            observer.register_outer_middleware(
                FSMContextMiddleware::new(storage).strategy(UserInChat),
            )
        })
        .on_message(|observer| {
            // Register your own handlers before `setup_dialogs`, so they win over dialog input.
            observer
                .register(Handler::new(handle_start).filter(CommandStart::default()))
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
```

A dialog is built from windows, and a window is built from widgets:

```rust,ignore
use telers::serialization::Value;
use telers_dialog::{
    dialog,
    widgets::{keyboard, text, Button, InlineKeyboard},
    window, Dialog, LaunchMode, StartMode,
};

/// A menu button that starts the feature dialog owning `state`.
fn link(id: &'static str, label: impl telers_dialog::widgets::Text, state: &'static str) -> Button {
    Button::start(id, label, state, Value::default(), StartMode::Normal)
}

pub fn dialog() -> impl Dialog {
    dialog([window(
        "main_menu",
        [
            text("Pick a feature below."),
            keyboard(
                InlineKeyboard::builder()
                    .row([link("m_text", "Text widgets", "text_widgets")])
                    .row([link("m_scroll", "Scrolling", "scrolls")])
                    .build(),
            ),
        ],
    )])
    // Root makes `/start` reset the current stack and return to a clean menu.
    .with_launch_mode(LaunchMode::Root)
}
```

`LaunchMode` decides how a dialog behaves when it is started: `Standard` (the default) puts no restrictions on it, `Root` resets the target stack, `Exclusive` additionally forbids starting any other dialog on top of it, and `SingleTop` is not repeated on top of the stack, so starting the same dialog again replaces it. `StartMode::NewStack` creates a separate stack without clearing the current one, including for `Root` and `Exclusive` dialogs.

For more, check out the [example below][examples/dialogs_mega] and the [source][source].

## Widgets
 - **Text**: `text`, `FormatText`, `FnText`, `ListText` (with optional pagination), `ScrollingText`, `MultiText`, `Case`, `Progress`, `FirstText`, conditional `Text::when`, and `TemplateText` behind the `template` feature.
 - **Keyboard**: `InlineKeyboard`, `Group`, `KeyboardGroup`, item-scoped `ListGroup`, `ReplyKeyboard` / `ReplyButton` / `ReplyKeyboardRemove`, `Button` with `ButtonStyle` and reusable `ButtonAppearance`, `Select`, `WhenCondition`, `ForceReply`, and the request widgets `RequestContact`, `RequestLocation` and `RequestPoll`.
 - **Stateful widgets**: `Checkbox`, `Counter`, `Multiselect`, `Radio`, `TimeSelect` and `Toggle`. They keep their state in the dialog context and support async state-change hooks.
 - **Paging and scrolling**: `ScrollingGroup`, `NumberedPager`, `FirstPage`, `PrevPage`, `CurrentPage`, `NextPage`, `LastPage`, `SwitchPage`, `StubScroll`, `sync_scroll`.
 - **Calendar**: `Calendar` with `CalendarConfig` and `CalendarAppearance` for custom labels and views.
 - **Input**: `TextInput` (including custom parsers), `MessageInput`, and `CombinedInput` with async `InputFilter`, for multi step forms.
 - **Media and previews**: `StaticMedia`, `DynamicMedia`, `MediaScroll`, and `LinkPreview`, including native stickers, voice, video notes, and dice.

## Runtime hooks and data

Dialog `.on_start`, `.on_close`, and `.on_process_result_with_manager` handlers receive a borrowed `DialogControl`, so hooks can read or update the active context without depending on a storage or client type. Action-returning result hooks support declarative transitions.

Dialog and window `.getter(DataGetter::new(...))` providers run in order. Window providers receive dialog-derived fields; callbacks load the same current data for visibility and item routing. Derived data is transient, while `start_data`, `dialog_data`, and `widget_data` are available separately. `FormatText` supports nested paths such as `{profile[name]}` and `{items[0].title}`, plus escaped braces.

Use `manager.background(&bot).await?` before spawning a job. Its controller stays bound to the current intent and rejects updates after that intent closes, is covered, restarts, or moves to another message destination. `background_for_stack` targets a separate named stack. Incoming messages stay on the default stack; callbacks select the retained stack owning their intent. Reply keyboards are restricted to the default stack because they apply to the whole chat.

Dialog middleware and background controllers share process-local event isolation. The default serializes by bot; `.with_event_isolation(EventIsolation::UserInChat)` allows separate conversations to run concurrently when the FSM uses `Strategy::UserInChat`. Choose a scope at least as broad as the FSM storage key. Separate worker processes need distributed isolation around that same key.

## Examples
 - [Dialogs mega][examples/dialogs_mega]. One bot that demonstrates the widget library: a root menu starts each feature dialog, `Button::done` returns to the menu, and `/start` resets the stack from any screen.

You may consider checking out [this directory][examples] for more examples.

## Community
### Telegram
- 🇺🇸 [@telers_en](https://t.me/telers_en)
- 🇷🇺 [@telers_ru](https://t.me/telers_ru)

## License
This project is licensed under either of the following licenses, at your option:
 - [Apache License, Version 2.0][licence_apache]
 - [MIT License][licence_mit]

[source]: https://github.com/Desiders/telers/tree/dev-1.x/telers-dialog
[examples]: https://github.com/Desiders/telers/tree/dev-1.x/examples
[examples/dialogs_mega]: https://github.com/Desiders/telers/tree/dev-1.x/examples/dialogs_mega
[licence_mit]: https://github.com/Desiders/telers/blob/dev-1.x/LICENSE-MIT
[licence_apache]: https://github.com/Desiders/telers/blob/dev-1.x/LICENSE-APACHE
