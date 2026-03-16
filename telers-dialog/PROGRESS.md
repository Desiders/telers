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
  - `src/widgets/widget.rs`
- Integration:
  - `src/setup.rs`

## Implemented Core
- `DialogRegistry` stores dialogs by state and rejects duplicate states.
- `setup` module provides the intended minimal `telers` integration path:
  - `Dialogs` extension container
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
  - `answer_callback`
  - iterative `ButtonAction::Chain` processing without recursive async calls
- `LaunchMode` is partially enforced:
  - `Root` and `Exclusive` reset the current stack on start
  - starting a different dialog over an active `Exclusive` dialog returns an error
  - `SingleTop` replaces the top context when starting the same dialog again on the same stack
- `WindowImpl` renders text plus optional keyboard and resolves callback actions from its keyboard.
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

## Main Widgets Available
- Text widgets:
  - static text via `&'static str`, `String`, or `Box<str>`
  - `FnText`
  - `FormatText`
  - `MultiText`
- Keyboard widgets:
  - `InlineKeyboard`
  - `Button::action`
  - `Button::url`
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
- Store `Dialogs` in dispatcher extensions.
- Use `telers` FSM middleware so handlers can extract `telers::fsm::Context<S>`.
- Register `DialogContextMiddleware` on `message`, `callback_query`, and other supported observers when you only need normalized dialog event context.
- Prefer `DialogManagerMiddleware<S>` when the handler will extract `DialogManager<S>` and you want setup to prebuild the typed manager in request context.
- `DialogManagerMiddleware<S>` does not derive `ChatEvent` or `EventContext` by itself; install `DialogContextMiddleware` first on the same observer chain.
- `DialogObserverExt::setup_dialogs::<S>()` is the preferred observer-level integration helper because it registers `DialogContextMiddleware` and `DialogManagerMiddleware<S>` in the correct order.
- `DialogManager<S>` extraction currently expects `DialogManagerMiddleware<S>` to have populated request context.
- Start dialogs from message or command handlers with `manager.start(...)`.
- Handle inline keyboard transitions from callback query handlers with `manager.handle_callback_query(...)`.

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
    Button, ButtonAction, DialogImpl, DialogManager, DialogObserverExt, Dialogs, InlineKeyboard,
    WindowImpl,
};

async fn start_dialog(
    bot: Bot,
    manager: DialogManager<MemoryStorage>,
) -> HandlerResult<()> {
    let _ = manager.start(&bot, "main", serde_json::Value::Null, telers_dialog::StartMode::ResetStack).await?;
    Ok(())
}

async fn handle_dialog_callback(
    bot: Bot,
    callback: CallbackQuery,
    manager: DialogManager<MemoryStorage>,
) -> HandlerResult<()> {
    let _ = manager.handle_callback_query(&bot, &callback).await?;
    Ok(())
}

fn dialogs() -> Dialogs {
    let dialog = DialogImpl::new(vec![
        WindowImpl::new(
            "main",
            [
                telers_dialog::widgets::WidgetKind::text("Hello"),
                telers_dialog::widgets::WidgetKind::keyboard(
                    InlineKeyboard::row([Button::action("next", "Next", ButtonAction::Next)]),
                ),
            ],
        ),
        WindowImpl::new(
            "second",
            [telers_dialog::widgets::WidgetKind::text("Second window")],
        ),
    ]);

    Dialogs::new().register(dialog).expect("dialog")
}

fn router() -> Router {
    Router::new("dialogs")
        .on_message(|observer| observer.setup_dialogs::<MemoryStorage>())
        .on_callback_query(|observer| observer.setup_dialogs::<MemoryStorage>())
}
```

## Important Design Decisions
- Callback button payloads are encoded as `td:{intent_id}:{button_id}`.
- This keeps callbacks scoped to the currently active dialog context and prevents stale buttons from another intent from being accepted.
- The crate is intentionally not trying to mirror the Python router inheritance model.
- Public builder-like APIs prefer owned `self -> Self` chaining over `&mut self` mutation for setup and widget composition.
- Public dialog/window registration accepts concrete values and wraps them internally instead of requiring manual `Arc::new(...)` at call sites.
- Integration with `telers` is intentionally explicit and lightweight:
  - put `Dialogs` into dispatcher extensions
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
  - select/list widgets
  - message input widgets
  - scrolling and paging widgets
- Widget-local state helpers built on top of `widget_data`.
- Result propagation between stacked dialogs.
- Access control enforcement using `AccessSettings`.
- Media rendering support.
- More example bots beyond the current runnable integration example in `examples/dialogs`.
- Higher-value tests for stack transitions and manager behavior.

## Validation Status
- `cargo check -p telers-dialog` passes.
- `cargo test -p telers-dialog` passes.
- `cargo check -p dialogs` passes.

## Recommended Next Slice
1. Add manager tests for:
   - `done` re-render behavior
   - `done` cleanup of the last dialog message
   - `SingleTop`
   - `Exclusive`
   - `next` and `back`
2. Design the next “main” widgets:
   - button variants with typed payload helpers
   - select/list widget
   - message input widget
3. Decide whether to keep setup fully manual or add a very small router/helper layer on top of `DialogObserverExt::setup_dialogs::<S>()`.
