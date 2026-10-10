# telers-dialog Progress

Updated: 2026-10-10 (UTC)

## Goal
- Focused Rust dialog framework for `telers`, borrowing `aiogram-dialog` behavior only where useful.
- Core: registry, manager, windows/widgets, FSM persistence, observer middleware integration.
- Feature work is not complete until the library API, tests, examples, and this progress file are all updated.

## Architecture
- Runtime: `manager.rs`, `message_manager.rs`, `dialog.rs`, `window.rs`.
- State/entities: `context.rs`, `stack.rs`, `messages.rs`, `events.rs`, `render.rs`, `result.rs`.
- Integration: `setup.rs` with `DialogContextMiddleware`, `DialogManagerMiddleware`, and `DialogObserverExt`.
- Widgets: text, keyboard, input, link preview, media, pager, stateful select, calendar, request keyboards.

## Implemented
- Dialog context and manager middlewares prepare request data before calling the outer `Next` continuation through filters, handlers, and descendant routers.
- Serialization follows `telers`: default `serde`, or native `deser` when enabled. Stored dialog contexts, stacks, access data, media, markup snapshots, and calendar state use the selected traits and JSON values. MiniJinja receives native values directly with Deser.
- `DialogRegistry` indexes by state and rejects duplicates.
- `DialogManager` supports start/switch/next/back/done/result/show plus callback/message handling.
- Manager and action APIs include single-value (`set_dialog_value`, `set_widget_value`) and batched (`extend_dialog_data`, `extend_widget_data`) helpers, mirrored on `ButtonAction`.
- Widget builders share canonical helpers via internal macros: `impl_button_row_helpers!` covers `header_row` / `header_push` / `footer_row` / `footer_push` on `Select`, `Checkbox`, `Counter`, `Multiselect`, `Radio`, `Toggle`; `impl_reply_keyboard_options_setters!` covers the reply-keyboard option setters on `RequestContact`, `RequestLocation`, `RequestPoll`.
- `Button` stores its widget id as `Option<Cow<'static, str>>`; only callback-style buttons (`Button::action`, `Button::on_click` and friends) carry an id, and `resolve_callback` short-circuits on `None`.
- `Progress` and `Case` text widgets use `#[bon]` builders for consistency with the rest of the widget surface.
- Launch modes: `Root`, `Exclusive`, `SingleTop`.
- `ShowMode::Auto`, message cleanup/reuse, parent result propagation.
- `dialog_data` and `widget_data` mutation/read helpers.
- Access control via registry-level `StackAccessValidator`.
- Public async hooks use `async-trait` for widgets, inputs, dialogs, windows, link preview, media, and scroll/page-count traits; `telers_dialog::async_trait` re-exports the macro for downstream impls.
- Callback contract: `td:{intent_id}:{widget_id}[:payload]`; stale intent callbacks ignored.
- Media rendering flows:
  - `Window` can render media widgets into `NewMessage`.
  - `MessageManager` sends media messages, uses window text as caption, edits through `editMessageMedia` when possible, and resends when text/media message shape or media type requires it.
  - `StaticMedia`, `DynamicMedia`, `MediaScroll`, `MediaAttachment`, `MediaId`, and `MediaContentType` are exported.
- Inline button additions:
  - `ButtonStyle` and `icon_custom_emoji_id` are exposed on `Button`.
  - Dynamic URL, web app URL, copy text, and switch-inline payload constructors are available.
- Text:
  - Static text, `FnText`, `FormatText`, `Case`, `MultiText`, `Progress`, `ScrollingText`, `ListText`.
  - `ListText` supports optional `page_size` pagination: with an `id` it renders one page of items, stores the page in `widget_data[id]`, and implements `Scroll` so a `NumberedPager` can drive it.
  - Optional `template` feature provides `TemplateText` and `TemplateEnvBuilder` backed by `minijinja`.
- Keyboard:
  - `InlineKeyboard`, `Group`, `Button`, `Select`, request keyboards, `ForceReply`, `WhenCondition`.
  - Stateful widgets: `Checkbox`, `Counter`, `TimeSelect`, `Radio`, `Toggle`, `Multiselect`.
  - `Calendar` supports `CalendarConfig`, `CalendarAppearance` (label override via `text_renderer`), and `CalendarViews` (full scope replacement).
  - Pager/scroll: `ScrollingGroup`, `SwitchPage`, first/prev/current/next/last wrappers, `NumberedPager`, `StubScroll`, `sync_scroll`.
- Link preview:
  - `LinkPreview` widget renders `LinkPreviewOptions` from static or dynamic text data.
- Reply-markup transitions:
  - The stack persists the real `last_reply_markup_type` (instead of a derived `last_reply_keyboard` flag), so `ForceReply` / `ReplyKeyboardRemove` messages are no longer misclassified as inline keyboards. Leaving a `ForceReply` window no longer triggers a bogus `editMessageReplyMarkup`.

## Existing Examples
- `examples/dialogs_mega` continues to wire both dialog middlewares through `setup_dialogs`, now using the outer continuation API.
- The existing `dialogs_mega` example supports both backends; run native mode with `cargo run -p dialogs_mega --no-default-features --features deser`. No example is missing for the serialization feature.
- A single combined example crate, `examples/dialogs_mega`, bundles the previous standalone dialog examples into one bot. A root menu (`LaunchMode::Root`) starts each feature dialog and every screen returns to it with `Button::done`. Feature dialogs:
  - text widgets (`FormatText`, `FnText`, `ListText`); template text (default + custom env).
  - scrolling widgets (`ScrollingGroup`, `ScrollingText`, paged `ListText`, `StubScroll`, `sync_scroll`).
  - keyboard layouts (`Group` row widths); selection widgets (`Select`, `Radio`, `Multiselect`, `Toggle`); combined stateful widgets.
  - `Counter` + a `widget_data`-driven progress bar; `Calendar` (default + `CalendarAppearance` custom labels) and `TimeSelect`.
  - multi-step input with `Case` summary; reply-keyboard request widgets; `TextInput` + `ForceReply`.
  - inline button styles + dynamic payloads; `Button::on_click`/`action` handlers; `LinkPreview`; media (`StaticMedia`, `DynamicMedia`, `MediaScroll`).

## Missing Examples
- Standalone pager buttons (`FirstPage` / `PrevPage` / `CurrentPage` / `NextPage` / `LastPage` / `SwitchPage`) are exported but not yet demonstrated in the mega example.

## Known Gaps
- Generic plain reply-keyboard row builder/factory beyond request-only widgets is still missing.
- Async/manager-aware result hooks beyond current action-based `on_process_result`.
- Managed wrapper types from `aiogram-dialog` are intentionally not implemented yet.

## aiogram-dialog Alignment
- Aligned: stack/current context, manager navigation, show/update decisions, data separation, text input, select routing, radio/multiselect/toggle, scrolling/pagers, media widgets, access control, result propagation.
- Different: explicit `telers` middleware integration, typed Rust actions/builders, smaller widget set, no managed wrapper types.

## Validation Snapshot
- Outer middleware migration (2026-10-10): direct middleware tests now supply routing continuations. Workspace compilation is covered by `just clippy`; tests and formatters were not rerun during lint cleanup.
- Convention review: grouped serialization imports, shortened the Deser README to 16 lines, and corrected private builder links. Generator tests pass (19); model and fixture output is unchanged.
- `cargo doc -p telers -p telers-dialog --no-deps --all-features --locked`: passes without warnings.
- `cargo test -p telers-dialog --all-features`: 273 unit tests and 1 doc test pass; 15 doc tests ignored (native Deser, including templates).
- Serde mode: all 273 dialog unit tests pass with the `template` feature.
- `cargo check --workspace --all-features --all-targets --locked`: passes with the native backend (one existing unused-field warning in the commands tests).
- `cargo check -p dialogs_mega --no-default-features --features deser`: passes.
- `just clippy` passes across the workspace with all features and pedantic warnings enabled, without warnings (2026-10-10).
- Serialization regressions: 273 native dialog tests and 124 Serde keyboard tests pass.
- Direct Serde dependencies are optional; enable `serde` explicitly when disabling defaults without choosing Deser. Backend features must match those selected on `telers`.
- The existing standalone pager example gap listed above is unrelated to serialization and remains open.
