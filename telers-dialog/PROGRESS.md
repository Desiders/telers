# telers-dialog Progress

Updated: 2026-04-24 (UTC)

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
- Current dialog examples cover button actions, calendar, message input, pager widgets, request widgets, select, stateful select, sync scroll, and text widgets.
- Missing example coverage is now the main gap for the newer widget/API work listed below.

## Missing Examples
- Media:
  - Static media from URL/file id.
  - Dynamic media from render data.
  - Media scrolling with pager controls.
- Inline button additions:
  - Danger/success/primary styled buttons.
  - Custom emoji button.
  - Dynamic URL/copy/switch-inline/web-app payloads from render data.
- Template text:
  - `TemplateText` with the `template` feature enabled.
  - Custom `TemplateEnvBuilder` filter/global example.
- Reply markup:
  - Plain reply-keyboard rows beyond request-only widgets, if/when implemented.
  - `ForceReply` prompt flow.

## Known Gaps
- Generic plain reply-keyboard row builder/factory beyond request-only widgets is still missing.
- Async/manager-aware result hooks beyond current action-based `on_process_result`.
- Managed wrapper types from `aiogram-dialog` are intentionally not implemented yet.

## aiogram-dialog Alignment
- Aligned: stack/current context, manager navigation, show/update decisions, data separation, text input, select routing, radio/multiselect/toggle, scrolling/pagers, media widgets, access control, result propagation.
- Different: explicit `telers` middleware integration, typed Rust actions/builders, smaller widget set, no managed wrapper types.

## Validation Snapshot
- Current validation after style cleanup: `cargo check -p telers-dialog`, `cargo test -p telers-dialog`, and `git diff --check`.
- `cargo fmt --check` could not run because `rustfmt` is not installed for toolchain `1.95.0-x86_64-unknown-linux-gnu`.
- `cargo clippy` could not run because `clippy` is not installed for toolchain `1.95.0-x86_64-unknown-linux-gnu`.
