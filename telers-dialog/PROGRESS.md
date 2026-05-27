# telers-dialog Progress

Updated: 2026-05-16 (UTC)

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
  - Optional `template` feature provides `TemplateText` and `TemplateEnvBuilder` backed by `minijinja`.
- Keyboard:
  - `InlineKeyboard`, `Group`, `Button`, `Select`, request keyboards, `ForceReply`, `WhenCondition`.
  - Stateful widgets: `Checkbox`, `Counter`, `TimeSelect`, `Radio`, `Toggle`, `Multiselect`.
  - Pager/scroll: `ScrollingGroup`, `SwitchPage`, first/prev/current/next/last wrappers, `NumberedPager`, `StubScroll`, `sync_scroll`.
- Link preview:
  - `LinkPreview` widget renders `LinkPreviewOptions` from static or dynamic text data.

## Existing Examples
- Current dialog examples cover button actions, calendar, message input, pager widgets, request widgets, select, stateful select, sync scroll, text widgets, media widgets (static/dynamic/scroll), inline button styles + dynamic payloads, template text (default and custom env), and `ForceReply` prompt flow.

## Missing Examples
- Reply markup:
  - Plain reply-keyboard rows beyond request-only widgets — pending the underlying widget (see Known Gaps).

## Known Gaps
- Generic plain reply-keyboard row builder/factory beyond request-only widgets is still missing.
- Async/manager-aware result hooks beyond current action-based `on_process_result`.
- Managed wrapper types from `aiogram-dialog` are intentionally not implemented yet.

## aiogram-dialog Alignment
- Aligned: stack/current context, manager navigation, show/update decisions, data separation, text input, select routing, radio/multiselect/toggle, scrolling/pagers, media widgets, access control, result propagation.
- Different: explicit `telers` middleware integration, typed Rust actions/builders, smaller widget set, no managed wrapper types.

## Validation Snapshot
- `cargo check -p telers-dialog --all-features`: passes.
- `cargo test -p telers-dialog --all-features`: 154 unit tests pass, 1 doc test passes, 15 doc tests ignored.
- `cargo check --workspace --all-features`: passes (covers all 13 dialog example crates).
- `cargo fmt --check` not run; `rustfmt` not installed for the current toolchain.
- `cargo clippy` not run; `clippy` not installed for the current toolchain.
