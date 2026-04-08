# telers-dialog Progress Snapshot

Updated: 2026-04-08 (UTC)

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
  - `src/widgets/stateful_select.rs` ← NEW
  - `src/widgets/pager.rs` ← NEW
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
- **Access control** enforced in `start`, `show`, `handle_callback_query`, `handle_message` paths:
  - `check_access()` validates user against `AccessSettings` from context.
  - Private chats always allowed; group chats check `user_ids` whitelist.
  - Returns `DialogError::AccessDenied { user_id }` on failure.

## Widgets available now
- Text:
  - static text (`&'static str`, `String`, `Box<str>`)
  - `FnText`
  - `FormatText`
  - `MultiText`
  - `ListText`
- Keyboard/actions:
  - `InlineKeyboard`
  - `Group` ← NEW: layout wrapper to group inline keyboard buttons by row width
  - `Button::{action,next,back,switch_to,start,done,set_dialog_value,url}`
  - `Select`
  - `Radio` ← NEW: single-selection stateful widget with stored selection in `widget_data`
  - `Multiselect` ← NEW: multi-selection widget with `min_selected`/`max_selected` constraints
  - `ScrollingGroup` ← NEW: pagination wrapper with `[1 | < | current | > | last]` pager row
  - `ButtonAction::{Noop,Next,Back,SwitchTo,Start,Done,SetDialogData,SetDialogValue,SetWidgetData,SetWidgetValue,Chain}`
- Input:
  - `MessageInput`
  - `TextInput`
- Typed callback payload helper:
  - `CallbackPayload`

## New widgets detail

### Radio
- Builder-based: `Radio::builder("widget_id").items_getter(...).checked_renderer(...).unchecked_renderer(...).id_getter(...).build()`
- Stores selected item id in `widget_data[widget_id]` as a string.
- Renders items with `checked_renderer` or `unchecked_renderer` based on selection state.
- Clicking a radio item produces `ButtonAction::SetWidgetValue` to persist selection.
- Supports `header_row`/`footer_row`/`header_push`/`footer_push` for additional buttons.
- Layout grouping is handled by wrapping with `Group` (for example `Group::new(Radio::builder(...).build(), 3)`).

### Multiselect
- Builder-based: `Multiselect::builder("widget_id").items_getter(...).checked_renderer(...).unchecked_renderer(...).id_getter(...).build()`
- Stores checked item ids in `widget_data[widget_id]` as a JSON array of strings.
- Toggle semantics: clicking a checked item unchecks it, clicking unchecked checks it.
- `min_selected` prevents unchecking below minimum (returns `Noop`).
- `max_selected` prevents checking above maximum (returns `Noop`).
- Supports header/footer static buttons.

### Group
- Generic keyboard wrapper: `Group::new(inner_keyboard, items_per_row)`.
- Applies only to inline keyboards, regrouping all buttons into rows of `items_per_row`.
- Delegates callback handling to wrapped keyboard unchanged.
- Intended as layout layer for widgets like `Select`, `Radio`, and `Multiselect`.

### ScrollingGroup
- Wraps any `Keyboard` widget and adds height-based pagination.
- `ScrollingGroup::new("id", inner_keyboard, height)` with `height` rows per page.
- Stores current page (0-indexed) in `widget_data[widget_id]`.
- Built-in pager row: `[1 | < | current | > | last]` (mirroring aiogram-dialog).
- `hide_on_single_page(true)` hides pager when only 1 page.
- `hide_pager(true)` suppresses pager entirely.
- Pages beyond max are clamped to last page.
- Inner keyboard callbacks are delegated transparently.
- Planned options:
  - configurable pager width (fixed slot count for stable layout)
  - blank/filler non-clickable buttons for empty pager slots

## Comparison with aiogram-dialog (actualized)

Reference baseline checked against `aiogram-dialog` stable docs on 2026-04-07.

### Aligned concepts
- Dialog stack + current context model.
- Start/switch/next/back/done manager operations.
- `ShowMode` and close/update decision model (including callback/message-driven auto decisions).
- `dialog_data` and `widget_data` separation.
- `TextInput` concept with parse success/error hooks.
- Select-style callback payload routing with widget-local ids.
- **Radio** single-selection with stored state in widget_data.
- **Multiselect** multi-selection with min/max constraints.
- **ScrollingGroup** with height-based pagination and built-in pager.
- **Access control** enforcement matching `DefaultAccessValidator` semantics.

### Intentional differences
- No router patching / hidden manager factories; integration stays explicit in `telers` observer middleware.
- Smaller widget surface than Python library; core text/keyboard/input/list/select pieces are shipped.
- Rust APIs prioritize owned builder chaining and typed actions over dynamic handler patterns.
- No `ManagedRadio`/`ManagedMultiselect` wrapper types; actions flow through `ButtonAction` enum.

### Missing compared to typical aiogram-dialog usage
- Toggle widget (radio variant showing only selected + next item).
- Rich media rendering flows and related `ShowMode` edge-case coverage.
- Built-in result propagation patterns between stacked dialogs.
- NumberedPager and SwitchPage standalone pager widgets.
- Custom `StackAccessValidator` trait for pluggable access control.
- `sync_scroll` utility for synchronized pagination across widgets.

### Widget backlog (not implemented yet)
- Keyboard widgets:
  - `Toggle`
  - `Checkbox`
  - `Counter`
  - `Calendar`
- Pager widgets (standalone controls):
  - `SwitchPage`
  - `CurrentPage`
  - `NextPage`
  - `PrevPage`
  - `FirstPage`
  - `LastPage`
  - `NumberedPager`
- Managed helper layer parity (post-MVP):
  - managed wrappers/helpers for stateful widgets (similar scope to `ManagedRadio` / `ManagedMultiselect` in `aiogram-dialog`)

## Test and validation status
- In-source tests exist across manager/setup/widgets/message-manager/registry/window modules.
- Validation rerun in this session with `rustc 1.94.1` via `docker run rust:latest`.
- `telers-dialog` test target passed:
  - **56 passed; 0 failed** (up from 31 → +25 new tests).
- New test coverage includes:
  - Radio: rendering checked/unchecked, callback action, foreign intent rejection, header/footer buttons (5 tests).
  - Multiselect: rendering, check/uncheck toggle, max_selected constraint, min_selected constraint (5 tests).
  - ScrollingGroup: first page, page from widget_data, last page, pager callback, inner callback delegation, hide_on_single_page, hide_pager, page clamping (8 tests).
  - Access control: no settings, private chat bypass, group deny, group allow, empty user_ids, async handle_message denial (6 tests).

## Known gaps
- Expand manager/message tests for media/reply-keyboard/`NoUpdate` edge cases.
- Define clear result-passing API between parent/child dialogs.
- Refactor widget module structure:
  - move keyboard-related widgets into `src/widgets/kbd/` directory
  - split by concern (`button`, `inline_keyboard`, `select`, `stateful_select`, `pager`, `group`, shared callback helpers)
  - keep a thin `widgets.rs` re-export layer with stable public API
- Add `Toggle` widget (radio variant showing one item at a time).
- Add standalone pager widgets (`NumberedPager`, `SwitchPage`).
- Pluggable `StackAccessValidator` trait for custom access logic.
- `sync_scroll` utility for synchronized pagination.

## Recommended next slice
1. Add result propagation between parent/child dialogs (`on_process_result` callback).
2. Refactor keyboard widget module layout into `src/widgets/kbd/` with focused files and stable re-exports.
3. Add `Toggle` widget as a Radio variant.
4. Add standalone `NumberedPager` and `SwitchPage` pager widgets.
5. Add pluggable `StackAccessValidator` trait for custom access policies.
