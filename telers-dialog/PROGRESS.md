# telers-dialog Progress Snapshot

Updated: 2026-04-09 (UTC)

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
  - `src/widgets/link_preview.rs`
  - `src/widgets/text.rs`
  - `src/widgets/kbd/`
    - `action.rs`
    - `base.rs`
    - `button.rs`
    - `callback.rs`
    - `group.rs`
    - `inline_keyboard.rs`
    - `pager/`
      - `common.rs`
      - `scrolling_group.rs`
      - `standalone.rs`
    - `request.rs`
    - `select.rs`
    - `stateful_select/`
      - `checkbox.rs`
      - `counter.rs`
      - `multiselect.rs`
      - `radio.rs`
      - `time_select.rs`
      - `toggle.rs`
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
  - `start`, `switch_to`, `next`, `back`, `done`, `done_with_result`, `show`
  - `handle_callback_query`, `handle_message`, `answer_callback`
  - iterative `ButtonAction::Chain` processing
  - parent dialog result propagation via `DialogImpl::on_process_result(...)`
  - internal action execution bookkeeping via `ActionOutcome { handled, already_shown }`
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
- Window composition also supports an optional link preview widget.
- Multiple text widgets are normalized into `MultiText`.
- Scroll widgets now share a small common model:
  - `BaseScroll` stores widget id + `on_page_changed`
  - `Scroll` provides page count plus shared page-state behavior
- Public Rustdoc coverage now includes crate root plus exported widget, manager, registry, and entity surfaces.
- Callback payload contract is scoped:
  - `td:{intent_id}:{widget_id}`
  - `td:{intent_id}:{widget_id}:{payload}`
- `done()` behavior follows close semantics:
  - re-render previous context after pop when stack is not empty
  - cleanup last dialog message when stack becomes empty
  - `done_with_result()` forwards child `start_data` + result to parent dialog callback
- `ShowMode::Auto` uses event/chat/stack-aware resolution.
- **Access control** enforced in `start`, `show`, `handle_callback_query`, `handle_message` paths:
  - `DialogRegistry` carries a pluggable `StackAccessValidator`.
  - Default validator matches `AccessSettings` semantics from current context or stack.
  - Private chats always allowed; group chats check `user_ids` whitelist unless a custom validator overrides it.
  - Returns `DialogError::AccessDenied { user_id }` on failure.

## Widgets available now
- Text:
  - static text (`&'static str`, `String`, `Box<str>`)
  - `FnText`
  - `FormatText`
  - `Case`
  - `MultiText`
  - `Progress`
  - `ScrollingText`
  - `ListText`
- Link preview:
  - `LinkPreview`
- Keyboard/actions:
  - `InlineKeyboard`
  - `Group`: layout wrapper to group inline keyboard buttons by row width
  - `Button::{action,next,back,switch_to,start,done,done_with_result,set_dialog_value,url,web_app,login_url,switch_inline_query,switch_inline_query_current_chat,switch_inline_query_chosen_chat,copy_text}`
  - `RequestContact`: one-button reply keyboard asking for the user's phone number
  - `RequestLocation`: one-button reply keyboard asking for the user's current location
  - `RequestPoll`: one-button reply keyboard asking the user to create and send a poll
  - `Select`
  - `Checkbox`: single-button boolean selector stored in `widget_data`
  - `Counter`: numeric stepper stored in `widget_data`
  - `TimeSelect`: hour/minute picker stored in `widget_data`
  - `Radio`: single-selection stateful widget with stored selection in `widget_data`
  - `Toggle`: single-button cyclic selector storing current item id in `widget_data`
  - `Multiselect`: multi-selection widget with `min_selected`/`max_selected` constraints
  - `ScrollingGroup`: pagination wrapper with `[1 | < | current | > | last]` pager row
  - `SwitchPage`: standalone pager button bound to shared page state
  - `FirstPage` / `PrevPage` / `CurrentPage` / `NextPage` / `LastPage`: convenience pager wrappers
  - `NumberedPager`: standalone numbered page row bound to shared page state
  - `ButtonAction::{Noop,Next,Back,SwitchTo,Start,Done,DoneWithResult,SetDialogData,SetDialogValue,SetWidgetData,SetWidgetValue,Chain}`
- Input:
  - `MessageInput`
  - `TextInput`
- Typed callback payload helper:
  - `CallbackPayload`

## Widget details

### Checkbox
- Builder-based: `Checkbox::builder("widget_id").checked_text(...).unchecked_text(...).build()`
- Stores current boolean value in `widget_data[widget_id]`.
- Renders a single button and toggles between `true` and `false` on click.
- Supports `default(...)`, `header_row`/`footer_row`/`header_push`/`footer_push`.

### Counter
- Builder-based: `Counter::builder("widget_id").default(...).increment(...).build()`
- Stores the current numeric value in `widget_data[widget_id]` as `f64`.
- Renders `[- | value | +]` in one row and updates the value on button clicks.
- Supports `min(...)`, `max(...)`, `increment(...)`, `default(...)`, and `cycle(...)`.
- `minus_hidden(true)` and `plus_hidden(true)` remove the side buttons when a one-sided or read-only counter layout fits better.
- Supports `header_row`/`footer_row`/`header_push`/`footer_push`.

### TimeSelect
- Builder-based: `TimeSelect::builder("widget_id").minute_precision(15).build()`
- Stores the current `(hour, minute)` pair in `widget_data[widget_id]` as `[hour, minute]`, where either side may be `null` until selected.
- Renders separate hour and minute sections with configurable `hour_header`, `minute_header`, `hour_width`, `minute_precision`, and `minute_width`.
- Clicking an hour preserves the current minute, and clicking a minute preserves the current hour.

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

### Toggle
- Builder-based: `Toggle::builder("widget_id").items_getter(...).item_renderer(...).id_getter(...).build()`
- Stores selected item id in `widget_data[widget_id]` as a string, same slot style as `Radio`.
- Renders a single button for the current item and cycles to the next item on click.
- Defaults to the first item when no stored selection exists or when stored selection is stale.
- Supports header/footer static buttons.

### Group
- Generic keyboard wrapper: `Group::new(inner_keyboard, items_per_row)`.
- Applies only to inline keyboards, regrouping all buttons into rows of `items_per_row`.
- Delegates callback handling to wrapped keyboard unchanged.
- Intended as layout layer for widgets like `Select`, `Radio`, and `Multiselect`.

### Request reply keyboards
- `RequestContact::new(text)` renders a reply keyboard with one contact-request button.
- `RequestLocation::new(text)` renders a reply keyboard with one location-request button.
- `RequestPoll::new(text).poll_type(PollType::Quiz)` renders a reply keyboard with one poll-request button.
- All three support reply-keyboard options via fluent setters:
  `is_persistent`, `resize_keyboard`, `one_time_keyboard`, `input_field_placeholder`, and `selective`.
- They do not produce callback actions; Telegram sends service/message updates after the user presses them.

### LinkPreview
- Builder-based: `LinkPreview::builder().url(...).build()`
- Supports `url(...)`, `is_disabled(...)`, `prefer_small_media(...)`, `prefer_large_media(...)`, and `show_above_text(...)`.
- Can be composed with normal `text(...)`, `keyboard(...)`, and `input(...)` widgets via `link_preview(...)`.
- `WindowImpl::link_preview_options(...)` remains available as the lower-level manual override.

### ScrollingGroup
- Wraps any `Keyboard` widget and adds height-based pagination.
- `ScrollingGroup::new("id", inner_keyboard, height)` with `height` rows per page.
- Stores current page (0-indexed) in `widget_data[widget_id]`.
- Uses `BaseScroll` internally, so page-state reads and `on_page_changed` handling match other scroll widgets.
- Is cloneable when the wrapped keyboard is cloneable, which lets standalone pagers bind to the same scroll widget without repeating page-count logic.
- Built-in pager row: `[1 | < | current | > | last]` (mirroring aiogram-dialog).
- `width` controls fixed content width for the paged keyboard; when set, buttons are regrouped into rows of that width before paging.
- Last pages are padded with filler buttons using `filler_text` (default blank space) to preserve the configured grid shape.
- `hide_on_single_page(true)` hides pager when only 1 page.
- `hide_pager(true)` suppresses pager entirely.
- `on_page_changed(...)` can emit additional actions after page changes.
- `OnPageChanged::new(...)` exposes `widget_id`, `old_page`, and `new_page` for richer page-change side effects.
- Pages beyond max are clamped to last page.
- Inner keyboard callbacks are delegated transparently.
- Filler buttons are inert placeholders targeting the current page; Telegram inline keyboards do not support truly disabled buttons.

### Standalone pager widgets
- `SwitchPage::builder("pager_id")...build()` renders a single button targeting first/prev/current/next/last page using shared page state in `widget_data`.
- `FirstPage`, `PrevPage`, `CurrentPage`, `NextPage`, and `LastPage` provide default-label convenience wrappers over the same shared page-state contract.
- `NumberedPager::builder("pager_id")...build()` renders numbered page buttons and highlights the current page with a separate renderer.
- Page-count closures now receive both `ctx` and `data`, so standalone pagers can follow context-sensitive scroll widgets cleanly.
- Standalone pagers can be built either from a plain widget id or directly from a `Scroll` widget through the same `builder(...)` entry point; `dialogs_pager_widgets` and `dialogs_sync_scroll` use this for `ScrollingGroup` bindings.
- All pager widgets support `on_page_changed(...)` for page-change side effects.
- Rich hooks can inspect `PageChange { widget_id, old_page, new_page }`.
- Both widgets reuse the same callback/state contract as `ScrollingGroup`, so they can coordinate through a shared widget id.

### sync_scroll
- `sync_scroll("other_id")` returns a page-change hook that copies the new page into another widget's `widget_data`.
- `sync_scrolls(["a", "b"])` does the same for multiple widget ids.
- Intended for synchronized pagination across multiple scrollable widgets without manual `ButtonAction::chain(...)` boilerplate.

### ScrollingText
- Builder-based: `ScrollingText::builder("widget_id").text(...).page_size(...).build()`
- Reads the current page from `widget_data[widget_id]` and slices the inner text accordingly.
- Implements the shared `Scroll` behavior via `BaseScroll`, so standalone pagers can bind to it directly.
- Page slicing is char-based rather than byte-based, so non-ASCII text stays safe.

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
- **Toggle** as a radio-style cyclic selector.
- **Standalone pager widgets** via `SwitchPage` and `NumberedPager`.
- **sync_scroll** utility for synchronized pagination across widgets.
- **Access control** enforcement matching `DefaultAccessValidator` semantics.
- **Parent result propagation** via `done_with_result()` + `on_process_result`.

### Intentional differences
- No router patching / hidden manager factories; integration stays explicit in `telers` observer middleware.
- Smaller widget surface than Python library; core text/keyboard/input/list/select pieces are shipped.
- Rust APIs prioritize owned builder chaining and typed actions over dynamic handler patterns.
- No `ManagedRadio`/`ManagedMultiselect` wrapper types; actions flow through `ButtonAction` enum.

### Missing compared to typical aiogram-dialog usage
- Rich media rendering flows and related `ShowMode` edge-case coverage.
- Dialog/window-level async result callbacks; current `on_process_result` is sync and action-based.

### Widget backlog (not implemented yet)
- Keyboard widgets:
  - `Calendar`
  - `StubScroll`
- Text widgets:

## Test and validation status
- In-source tests exist across manager/setup/widgets/message-manager/registry/window modules.
- Validation rerun in this session with local `cargo test -p telers-dialog`.
- Example validation rerun in this session with:
  - `cargo check -p dialogs_button_actions`
  - `cargo check -p dialogs_request_widgets`
  - `cargo check -p dialogs_pager_widgets`
  - `cargo check -p dialogs_select_widget`
  - `cargo check -p dialogs_stateful_select_widgets`
  - `cargo check -p dialogs_text_widgets`
  - `cargo check -p dialogs_sync_scroll`
- `telers-dialog` test target passed:
  - **102 passed; 0 failed**.
- New test coverage includes:
  - Convenience pager wrappers: render targets and callback payloads for first/prev/current/next/last controls (1 test).
  - ScrollingGroup width options: fixed-grid regrouping and last-page filler padding behavior (2 tests).
  - sync_scroll hooks: synchronizing one and many target widget ids from pager callbacks (2 tests).
  - Result propagation: parent `on_process_result` receives child `start_data` and result (1 test).
  - Toggle: default render, selected render/cycle, callback action (3 tests).
  - Standalone pager widgets: `SwitchPage` render/callback wiring, `NumberedPager` render/callback wiring (3 tests).
  - Custom access validator override (1 test).
  - Radio: rendering checked/unchecked, callback action, foreign intent rejection, header/footer buttons (5 tests).
  - Multiselect: rendering, check/uncheck toggle, max_selected constraint, min_selected constraint (5 tests).
  - ScrollingGroup: first page, page from widget_data, last page, pager callback, inner callback delegation, hide_on_single_page, hide_pager, page clamping (8 tests).
  - Access control: no settings, private chat bypass, group deny, group allow, empty user_ids, async handle_message denial (6 tests).
  - Message manager: `NoUpdate` snapshot reuse/failure, reply-keyboard edit restrictions, reply-keyboard detection, protect-content/link-preview change detection (5 tests).
  - Show mode calculation: delete-and-send after reply keyboard, send/edit behavior for private media-group messages (3 tests).
  - Rich page-change hooks: `OnPageChanged::new(...)` receives widget id plus old/new page values (1 test).
  - Text widgets: `Case` keyed/default selection, `Progress` bar rendering/clamping, and `ScrollingText` page slicing plus direct pager binding (8 tests).
  - Simple widget additions: `Checkbox` render/toggle behavior and inline non-callback button variants (4 tests).
  - Request reply keyboards: contact/location/poll reply markup rendering and option propagation (3 tests).
  - TimeSelect: render/state behavior for header rows plus hour/minute callbacks (3 tests).
  - Link preview widget: option rendering plus window integration (2 tests).

## Known gaps
- Keep `widgets.rs` as a thin re-export layer with stable public API.
- Keep examples focused by audience:
  - `dialogs_text_widgets` demonstrates a two-step broadcast preview flow, with several text widgets composing one ready-to-send message.
  - `dialogs_request_widgets` demonstrates a pop-up launch setup flow for `RequestContact`, `RequestLocation`, and `RequestPoll`.
  - `dialogs_select_widget` stays a beginner `Select` example.
  - `dialogs_stateful_select_widgets` covers `Checkbox`, `Counter`, `Radio`, `Toggle`, and `Multiselect` with a realistic subscription-settings flow.
  - `dialogs_pager_widgets` carries built-in pager and standalone pager demos.
  - `dialogs_sync_scroll` demonstrates a related two-block page where one pager keeps a compact picker and a details block aligned.
- `pager` and `stateful_select` are now split into finer-grained submodules under `src/widgets/kbd/`, while `pager.rs` and `stateful_select.rs` remain the module roots.

## Planned follow-ups
- Consider async/manager-aware result hooks beyond action-based `on_process_result`.

## Recommended next slice
1. Continue the missing-widget backlog with the remaining unresolved widgets: `Calendar` and `StubScroll`.
