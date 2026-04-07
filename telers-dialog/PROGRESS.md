# telers-dialog Progress Snapshot

Updated: 2026-04-07 (UTC)

## Goal
- Build a focused, Rust-native dialog framework for `telers`.
- Keep the core stable: registry, manager, windows/widgets, FSM persistence, and explicit observer integration.
- Track `aiogram-dialog` semantics where they matter for user-facing behavior, without mirroring its full Python surface.

## Current architecture
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
- Widget surface:
- `src/widgets/text.rs`
- `src/widgets/kbd.rs`
- `src/widgets/input.rs`
- `src/widgets/list.rs`
- `src/widgets/widget.rs`
- Integration:
- `src/setup.rs`

## Implemented behavior
- `DialogRegistry` indexes dialogs by state and rejects duplicates.
- Observer-level integration helper exists: `DialogObserverExt::setup_dialogs::<S>()`.
- Setup middleware pipeline is explicit:
- `DialogContextMiddleware` derives `ChatEvent` + `EventContext`.
- `DialogManagerMiddleware<S>` injects typed `DialogManager<S>` into request context.
- `DialogManager` supports:
- `start`, `switch_to`, `next`, `back`, `done`, `show`
- `handle_callback_query`, `handle_message`, `answer_callback`
- iterative `ButtonAction::Chain` processing
- Launch mode coverage:
- `LaunchMode::Root` and `LaunchMode::Exclusive` reset current stack on start.
- `LaunchMode::Exclusive` blocks starting a different dialog while active.
- `LaunchMode::SingleTop` reuses top context id and resets per-context data.
- Context helpers expose typed reads for both data stores:
- `dialog_value(_)/dialog_value_as::<T>(_)`
- `widget_value(_)/widget_value_as::<T>(_)`
- Widget state mutation actions are implemented:
- `SetDialogData`, `SetDialogValue`, `SetWidgetData`, `SetWidgetValue`
- Window composition supports text + optional keyboard + optional input.
- Multiple text widgets are normalized into `MultiText`.
- Callback payload contract is scoped:
- `td:{intent_id}:{widget_id}`
- `td:{intent_id}:{widget_id}:{payload}`
- `done()` behavior follows close semantics:
- re-render previous context after pop when stack is not empty
- cleanup last dialog message when stack becomes empty
- `ShowMode::Auto` uses event/chat/stack-aware resolution.

## Widgets available now
- Text:
- static text (`&'static str`, `String`, `Box<str>`)
- `FnText`
- `FormatText`
- `MultiText`
- `ListText`
- Keyboard/actions:
- `InlineKeyboard`
- `Button::{action,next,back,switch_to,start,done,set_dialog_value,url}`
- `Select`
- `ButtonAction::{Noop,Next,Back,SwitchTo,Start,Done,SetDialogData,SetDialogValue,SetWidgetData,SetWidgetValue,Chain}`
- Input:
- `MessageInput`
- `TextInput`
- Typed callback payload helper:
- `CallbackPayload`

## Comparison with aiogram-dialog (actualized)

Reference baseline checked against `aiogram-dialog` stable docs on 2026-04-07.

### Aligned concepts
- Dialog stack + current context model.
- Start/switch/next/back/done manager operations.
- `ShowMode` and close/update decision model (including callback/message-driven auto decisions).
- `dialog_data` and `widget_data` separation.
- `TextInput` concept with parse success/error hooks.
- Select-style callback payload routing with widget-local ids.

### Intentional differences
- No router patching / hidden manager factories; integration stays explicit in `telers` observer middleware.
- Smaller widget surface than Python library; only core text/keyboard/input/list pieces are shipped.
- No full parity for advanced media/pagination groups and many stateful widget families yet.
- Rust APIs prioritize owned builder chaining and typed actions over dynamic handler patterns.

### Missing compared to typical aiogram-dialog usage
- Broader widget catalog (checkbox/radio/multiselect variants, richer managed widgets).
- Pagination/scrolling groups and dynamic list navigation helpers.
- Rich media rendering flows and related `ShowMode` edge-case coverage.
- Built-in result propagation patterns between stacked dialogs.
- Access policy enforcement wired to `AccessSettings`.

## Test and validation status
- In-source tests exist across manager/setup/widgets/message-manager/registry/window modules.
- Validation rerun in this session with `cargo 1.94.1` and `just 1.49.0`.
- `just test` completed successfully across workspace crates and examples.
- `telers-dialog` test target passed:
- `31 passed; 0 failed` (`target/debug/deps/telers_dialog-*`).

## Known gaps
- Add first-class stateful widgets on top of `widget_data` helpers.
- Add paging/scrolling widgets and related callback plumbing.
- Expand manager/message tests for media/reply-keyboard/`NoUpdate` edge cases.
- Define clear result-passing API between parent/child dialogs.
- Enforce access control (`AccessSettings`) in runtime flow.

## Recommended next slice
1. Implement one production-ready stateful widget family (radio/select with stored selection).
2. Add simple pager/scroll widget and cover callback payload interoperability.
3. Add access-check hook in manager start/show paths, with focused tests.
