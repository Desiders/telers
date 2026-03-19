# telers-dialog Session Snapshot

## Goal
- Build a stable, smaller Rust-native dialog framework for `telers`.
- Do not port all of `aiogram-dialog`.
- Keep the core explicit and composable: dialog registry, manager, window rendering, text widgets, inline action buttons, and FSM-backed dialog state.

## Current Architecture
- Runtime core:
  - `src/manager.rs`
  - `src/message_manager.rs`
  - `src/dialog.rs`
  - `src/window.rs`
- State model:
  - `src/entities/context.rs`
  - `src/entities/stack.rs`
  - `src/entities/messages.rs`
  - `src/entities/events.rs`
- Widgets:
  - `src/widgets/text.rs`
  - `src/widgets/kbd.rs`
  - `src/widgets/input.rs`
  - `src/widgets/list.rs`
  - `src/widgets/widget.rs`
- Integration:
  - `src/setup.rs`

## Implemented Core
- `DialogRegistry` stores dialogs by state and rejects duplicate states.
- `setup` module provides the intended minimal `telers` integration path:
  - `DialogRegistry` as the shared dispatcher extension
  - `DialogContextMiddleware`
  - `DialogManagerMiddleware<S>`
  - `DialogObserverExt<Client>` for wiring both middlewares on a telegram observer
  - direct `DialogManager<S>` extraction in handlers
- `DialogManager` supports:
  - `start`
  - `switch_to`
  - `next`
  - `back`
  - `done`
  - `show`
  - `handle_callback_query`
  - `handle_message`
  - `answer_callback`
  - iterative `ButtonAction::Chain` processing without recursive async calls
- `LaunchMode` is partially enforced:
  - `Root` and `Exclusive` reset the current stack on start
  - starting a different dialog over an active `Exclusive` dialog returns an error
  - `SingleTop` replaces the top context when starting the same dialog again on the same stack
- The built-in `window(...)` type renders text plus optional keyboard and optional message input.
- Multiple text widgets in a single window are normalized into `MultiText`.
- `MessageManager` now tracks enough message snapshot data to compare rendered output more reliably:
  - text
  - reply markup snapshot
  - link preview snapshot
  - message id
  - protected content flag
- `done()` now follows `aiogram-dialog` close semantics:
  - re-render previous dialog when a stack still has a context after pop
  - otherwise clean up the last dialog message according to `ShowMode`
  - `ShowMode::Auto` resolves like `aiogram-dialog` for callback-driven updates
- Manager tests now cover the main stack and launch-mode behaviors:
  - `done()` re-render of the previous context
  - `done()` cleanup of the last dialog message
  - `SingleTop` top-context reuse
  - `Exclusive` reset and blocking semantics
  - `next()` / `back()` transitions and boundary errors

## Main Widgets Available
- Text widgets:
  - static text via `&'static str`, `String`, or `Box<str>`
  - `FnText`
  - `FormatText`
  - `MultiText`
  - `ListText`
- Keyboard widgets:
  - `InlineKeyboard`
  - `Button::action`
  - `Button::next`
  - `Button::back`
  - `Button::switch_to`
  - `Button::start`
  - `Button::done`
  - `Button::set_dialog_value`
  - `Button::url`
  - `Select`
- Input widgets:
  - `MessageInput`
- Typed callback payload helpers:
  - `CallbackPayload` for compact typed select payloads
- Button actions:
  - `Noop`
  - `Next`
  - `Back`
  - `SwitchTo`
  - `Start`
  - `Done`
  - `SetDialogData`
  - `SetDialogValue`
  - `Chain`

## Integration Pattern
- Store `DialogRegistry` in dispatcher extensions.
- Use `telers` FSM middleware so handlers can extract `telers::fsm::Context<S>`.
- Register `DialogContextMiddleware` on `message`, `callback_query`, and other supported observers when you only need normalized dialog event context.
- Prefer `DialogManagerMiddleware<S>` when the handler will extract `DialogManager<S>` and you want setup to prebuild the typed manager in request context.
- `DialogManagerMiddleware<S>` does not derive `ChatEvent` or `EventContext` by itself; install `DialogContextMiddleware` first on the same observer chain.
- `DialogObserverExt::setup_dialogs::<S>()` is the preferred observer-level integration helper because it registers `DialogContextMiddleware`, `DialogManagerMiddleware<S>`, and the default internal dialog event dispatcher in the correct order.
- `DialogManager<S>` extraction currently expects `DialogManagerMiddleware<S>` to have populated request context.
- Start dialogs from message or command handlers with `manager.start(...)`.
- `setup_dialogs::<S>()` automatically dispatches dialog messages and callback queries for the observer where it is installed.
- When a command or custom message handler must take priority over dialog message input on the same observer, register that handler before calling `setup_dialogs::<S>()`.
- The runnable `telers-dialog` examples live as separate widget-focused example packages under `examples/dialogs_*`.

## Example Usage Sketch
```rust
use telers::{
    enums::UpdateType,
    event::telegram::{Handler, HandlerResult},
    filters::Command,
    fsm::MemoryStorage,
    middlewares::outer::FSMContext,
    types::{CallbackQuery, Message},
    Bot, Dispatcher, Extension, Router,
};
use telers_dialog::{
    dialog, window, DialogManager, DialogObserverExt, DialogRegistry,
};
use telers_dialog::widgets::{keyboard, text, Button, ButtonAction, InlineKeyboard};

async fn start_dialog(
    bot: Bot,
    manager: DialogManager<MemoryStorage>,
) -> HandlerResult<()> {
    let _ = manager.start(&bot, "main", serde_json::Value::Null, telers_dialog::StartMode::ResetStack).await?;
    Ok(())
}

fn registry() -> DialogRegistry {
    let dialog = dialog(vec![
        window(
            "main",
            [
                text("Hello"),
                keyboard(
                    InlineKeyboard::row([Button::action("next", "Next", ButtonAction::Next)]),
                ),
            ],
        ),
        window("second", [text("Second window")]),
    ]);

    DialogRegistry::new().register(dialog).expect("dialog")
}

fn router() -> Router {
    Router::new("dialogs")
        .on_message(|observer| {
            observer
                .register(Handler::new(start_dialog).filter(Command::one("start")))
                .setup_dialogs::<MemoryStorage>()
        })
        .on_callback_query(|observer| observer.setup_dialogs::<MemoryStorage>())
}
```

## Important Design Decisions
- Callback button payloads are encoded as `td:{intent_id}:{widget_id}` with an optional typed payload suffix:
  - `td:{intent_id}:{widget_id}`
  - `td:{intent_id}:{widget_id}:{payload}`
- This keeps callbacks scoped to the currently active dialog context and prevents stale buttons from another intent from being accepted.
- The crate is intentionally not trying to mirror the Python router inheritance model.
- Public builder-like APIs prefer owned `self -> Self` chaining over `&mut self` mutation for setup and widget composition.
- Public dialog/window registration accepts concrete values and wraps them internally instead of requiring manual `Arc::new(...)` at call sites.
- Integration with `telers` is intentionally explicit and lightweight:
  - put `DialogRegistry` into dispatcher extensions
  - register `DialogContextMiddleware` or `DialogManagerMiddleware<S>` on relevant observers
  - keep using `telers` FSM middleware for storage-backed dialog state
- Avoid `aiogram-dialog`-style hidden manager factories and router patching unless `telers` later proves they are necessary.

## Open Decisions
- Whether to add a small builder helper for common router wiring, or keep setup fully manual.
- How much typed callback payload support should exist beyond `ButtonAction`.
- Whether widget-local state should stay as raw `serde_json::Value` maps or gain typed helpers.
- How result propagation between stacked dialogs should look in idiomatic Rust.

## Known Gaps
- More widgets:
  - richer text composition
  - scrolling and paging widgets
- Widget-local state helpers built on top of `widget_data`.
- Result propagation between stacked dialogs.
- Access control enforcement using `AccessSettings`.
- Media rendering support.
- More example bots beyond the current widget-focused `examples/dialogs_*` packages.
- More manager coverage for callback-action flows and show-mode edge cases around media, reply keyboards, and `NoUpdate`.

## Validation Status
- `cargo check -p telers-dialog` passes.
- `cargo test -p telers-dialog` passes.
- `cargo check -p dialogs_text_widgets -p dialogs_button_actions -p dialogs_select_widget -p dialogs_message_input` passes.
- `just fmt` passes.
- `just clippy` passes with existing workspace and crate warnings.

## Recommended Next Slice
1. Add widget-local state helpers built on top of `widget_data`, especially for select/input widgets.
2. Add scrolling and paging widgets on top of the new callback payload plumbing.
3. Decide whether to keep setup fully manual or add a very small router/helper layer on top of `DialogObserverExt::setup_dialogs::<S>()`.
