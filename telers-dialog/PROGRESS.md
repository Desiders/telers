# telers-dialog Progress

Updated: 2026-10-13

## Goal
- Rust-native dialog framework over `telers`; use `aiogram-dialog` as a behavior reference.
- Feature work includes the API, meaningful crate tests, and a reachable runnable example.

## Architecture
- `dialog.rs`, `window.rs`, `manager.rs`, and `message_manager.rs` handle navigation, rendering, lifecycle, and Telegram requests.
- Persistent contexts/stacks use the selected `telers` serialization backend: Serde by default or native Deser.
- `DialogRegistry` owns shared registration, access validation, media caching, and event isolation.
- `DialogContextMiddleware` precedes `DialogManagerMiddleware`; `setup_dialogs` wraps the full observer continuation. Command handlers run before dialog input handlers.
- Widgets keep owned builders and hide internal boxes/arcs. `DialogControl` provides borrowed, storage/client-independent access inside lifecycle/result hooks.

## Implemented
- Registry/state lookup, ordered windows, start/switch/next/back/done/result, Root/Exclusive/SingleTop launch modes, access settings and FSM persistence.
- `start_with_access_settings` applies access rules before the start hook and first render; child starts inherit context/stack rules. SingleTop replaces the intent with a fresh id so old callbacks/jobs expire.
- `for_stack` and `background_for_stack` select retained stacks without changing default message routing. Ordinary messages address the default stack; callbacks select their active intent's stack. Reply keyboards are restricted to the default stack.
- `BackgroundManager` binds jobs to an intent and rejects covered, closed, restarted, or relocated conversations. Synthetic events describe update/start/switch/done operations.
- Foreground middleware and background operations share process-local event isolation. Bot is the conservative default; User, Chat, and UserInChat scopes permit concurrency when they match the FSM strategy. Inline calls reenter safely; spawned work acquires its own guard; cancellation releases guards.
- Async dialog start/close and manager-aware result hooks; action-based dialog/window result hooks are also available. Plain done passes a null child result. Hooks can mutate/navigate before rendering, close errors leave the intent open, and transition chains avoid duplicate rendering.
- Ordered dialog/window `DataGetter` providers receive stored values, event metadata, and typed middleware data. Derived values remain transient. Visibility/item callbacks load current getter data, while foreign/stale intents skip providers.
- Event metadata is independent of the Telegram client type. `EVENT_BOT_KEY` exposes the actual typed bot in runtime data. Senderless updates are ignored instead of panicking in middleware.
- Text: static/computed/format text, nested map/list fields and escaped braces, MultiText, Case, Progress, ListText with item/page metadata, ScrollingText, conditional Text::when, FirstText fallback, optional MiniJinja templates.
- Input: typed MessageInput, TextInput with FromStr or an explicit custom parser, error handlers, typed stored-value access, async filters and CombinedInput. Handlers see the original message and latest accepted widget value.
- Keyboards: InlineKeyboard, Group, KeyboardGroup and repeated nested ListGroup. List groups isolate widget/page state by stable item ids, preserve parent/item data, and scope mutation chains across nested groups and intent transitions.
- Native ReplyKeyboard/ReplyButton rows, explicit removal, contact/location/poll/user/chat/managed-bot/web-app request payloads and ForceReply. Placeholders and labels can use dynamic Text. Reply choices use normal MessageInput; no hidden text callback protocol.
- Button visibility, typed URL/web-app/copy/inline-query actions, reusable conditional/dynamic ButtonAppearance, custom emoji and fallback composition. Appearance covers inline/reply buttons, collection selections, calendars, time selection and pagers.
- Checkbox, Counter, Radio, Multiselect, Toggle, TimeSelect, async state-change hooks and min/max constraints. Hooks receive the updated widget snapshot. Selection payloads must still belong to the current item set; counters keep finite values and obey bounds for either increment direction.
- Calendar configuration/custom views and label/appearance hooks, bounded month/year/day rendering, negative-year persistence, range normalization, and absolute built-in navigation targets that survive calendar/today rollovers.
- ScrollingGroup, NumberedPager, first/previous/current/next/last/fixed-page controls, StubScroll, sync_scroll; bounded callbacks, page-count changes and dimension overflow are covered.
- StaticMedia, DynamicMedia, MediaScroll, media-id caching, link previews, native photo/video/animation/audio/document/sticker/voice/video-note/dice requests and media-option propagation.
- Message snapshots include their original chat, thread, business connection, parse mode and media options. Destination/type/option changes choose edit versus resend correctly. NoUpdate does not touch transport/media cache; manager-level suppression also skips getters.
- Expected stale/deleted/noneditable Telegram messages recover safely. Expired callback acknowledgments do not discard valid actions; unrelated transport errors propagate. Reply-keyboard transitions remove the keyboard in its original destination and preserve chat-wide replacements across topics.
- Callback contract is `td:{intent_id}:{widget_id}[:payload]`. Nested lists encode item routing within payloads. Removed/offscreen/hidden/oversized nested callbacks and stale physical-message callbacks are rejected.

## Examples
`examples/dialogs_mega` contains reachable screens for every feature family above:
- Text/templates, conditional/fallback composition, custom parsers and filtered input groups.
- Dialog/window data getters, lifecycle and parent/child results.
- Nested repeated keyboards with per-item state and pagination.
- Native reply rows, requests/removal, force reply, selection hooks and dynamic appearances.
- Numbered and standalone pager controls, synchronized scrolling, calendar/time customization.
- Media/gallery/link previews, dice and user-supplied sticker/voice/video-note content.
- `/background` runs a pinned progress job; closing or restarting its intent stops updates.
The example selects UserInChat isolation to match its FSM strategy and supports both serialization backends.

## Missing Examples
- None for the implemented additions.

## Remaining Scope and Design Questions
- Python HTML/web preview and diagram tooling are not ported. Decide whether Rust preview/inspection belongs in a separate tools crate.
- Managed Python wrapper types are replaced by typed context access, explicit widget ids and actions. Programmatic raw manager writes deliberately do not invoke widget callbacks; decide whether a distinct semantic setter/typed handle API is useful before adding wrapper-like state objects.
- Manager-aware result processing is provided at dialog level; window-level result handlers return actions.
- Built-in isolation is process-local. Multiple workers sharing an FSM backend require distributed isolation around the same storage key. Manually constructed managers outside middleware must participate in equivalent isolation.
- TimeSelect headers remain static builder labels; value renderers and appearances provide runtime customization.

## Validation
- Both native Deser/all-features and Serde/template test suites: 451 unit tests pass, 1 doctest passes, 15 illustrative doctests remain ignored.
- Tests are limited to `telers-dialog`; no `telers` test suite was run for this feature task.
- `cargo check -p telers-dialog --all-features --locked` passes. `dialogs_mega` builds with both default Serde and all-feature native Deser.
- Scoped `cargo clippy -p telers-dialog --no-deps --all-features --all-targets --locked -- -D warnings` passes. Dependency-wide linting on Rust 1.99 flags the existing redundant `#[must_use]` in `telers/src/middlewares/inner/base.rs:74`; that source is unchanged.
- Telegram behavior is validated through typed request/response transport mocks; live bot polling was not exercised.
- Convention review reused the existing input traversal and callback encoder, aligned new production/test layout with neighboring code, and corrected obsolete runtime/widget documentation and the dice example's missing required text widget.
- A startup smoke check constructs and registers all demo dialogs successfully without polling or bot credentials. Rustdoc builds with warnings denied.
- No formatter was run.
