# telers-dialog Progress

Updated: 2026-04-21 (UTC)

## Goal
- Focused Rust dialog framework for `telers`, borrowing `aiogram-dialog` behavior only where useful.
- Core: registry, manager, windows/widgets, FSM persistence, observer middleware integration.

## Architecture
- Runtime: `manager.rs`, `message_manager.rs`, `dialog.rs`, `window.rs`.
- State/entities: `context.rs`, `stack.rs`, `messages.rs`, `events.rs`, `render.rs`.
- Integration: `setup.rs` with `DialogContextMiddleware`, `DialogManagerMiddleware`, and `DialogObserverExt`.
- Widgets: text, keyboard, input, link preview, pager, stateful select, calendar, request keyboards.

## Implemented
- `DialogRegistry` indexes by state and rejects duplicates.
- `DialogManager` supports start/switch/next/back/done/result/show plus callback/message handling.
- Launch modes: `Root`, `Exclusive`, `SingleTop`.
- `ShowMode::Auto`, message cleanup/reuse, parent result propagation.
- `dialog_data` and `widget_data` mutation/read helpers.
- Access control via registry-level `StackAccessValidator`.
- Public async hooks use `async-trait` for widgets, inputs, dialogs, windows, link preview, and scroll/page-count traits; `telers_dialog::async_trait` re-exports the macro for downstream impls.
- Callback contract: `td:{intent_id}:{widget_id}[:payload]`; stale intent callbacks ignored.

## Widget Surface
- Text: static, `FnText`, `FormatText`, `Case`, `MultiText`, `Progress`, `ScrollingText`, `ListText`.
- Keyboard: `InlineKeyboard`, `Group`, `Button`, `Select`, request keyboards, `WhenCondition`.
- Stateful: `Checkbox`, `Counter`, `TimeSelect`, `Radio`, `Toggle`, `Multiselect`.
- Pager/scroll: `ScrollingGroup`, `SwitchPage`, first/prev/current/next/last wrappers, `NumberedPager`, `StubScroll`, `sync_scroll`.
- Other: `Calendar`, `LinkPreview`, `MessageInput`, `TextInput`, `CallbackPayload`.

## aiogram-dialog Alignment
- Aligned: stack/current context, manager navigation, show/update decisions, data separation, text input, select routing, radio/multiselect/toggle, scrolling/pagers, access control, result propagation.
- Different: explicit `telers` middleware integration, smaller widget set, typed Rust actions/builders, no managed wrapper types.

## Known Gaps
- Rich media rendering flows.
- Async/manager-aware result hooks beyond current action-based `on_process_result`.
- Shared button styling abstraction beyond calendar-specific styling.

## Validation Snapshot
- Last broad validation recorded: `cargo test -p telers-dialog`, `cargo doc -p telers-dialog --no-deps`, `cargo check -p telers-dialog`, and dialog examples.
- Current validation: `cargo check -p telers-dialog --all-features`, `cargo clippy -p telers-dialog --all-features -- -W clippy::pedantic`, `cargo test -p telers-dialog --all-features`, and `cargo check -p dialogs_pager_widgets`.
