# telers-dialog Progress

Updated: 2026-06-15 (UTC)

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
- `cargo check -p telers-dialog --all-features`: passes.
- `cargo test -p telers-dialog --all-features`: 157 unit tests pass, 1 doc test passes, 15 doc tests ignored.
- `cargo check --workspace --all-features`: passes (the sole dialog example crate is `dialogs_mega`).
- `cargo fmt -p dialogs_mega`: applied.
- `cargo clippy -p telers-dialog --all-features` and `cargo clippy -p dialogs_mega`: no warnings in either crate (3 pre-existing `map_or` warnings remain in the `telers` lib).
