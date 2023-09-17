//! Filters are main part of the library used to filter incoming updates and allow call handlers by their data (text, chat, user, command, etc.),
//! [`context`] (state, db, etc.) and other conditions.
//!
//! [`Filter`] is a trait that accepts [`bot`], [`update`] and [`context`] and returns `true` if the filter passes, otherwise `false`.
//! You can use [`Filter`] trait to create your own filters or use one of the ready-made implementations.
//! Most likely you will have to write your filters, so we recommend you to check out the `examples/text_case_filter` to see how to create your own filters
//! and check ready-made implementations.
//!
//! Filters can be combined with logical operators [`And`] and [`Or`] and inverted with [`Invert`].
//! Each filter has a method [`Filter::invert`], [`Filter::and`] and [`Filter::or`] to create [`Invert`], [`And`] and [`Or`] filters respectively.
//!
//! Ready-made implementations:
//! * [`ChatType`]:
//! Filter for checking the type of chat.
//! Usually used with [`ChatTypeEnum`] (or its string representation) to check the type of chat.
//! Creates with `one` or `many` methods.
//! * [`Command`]:
//! This filter checks if the message is a command.
//! Filter accepts [`command pattern type`] that represents a command pattern type for verification,
//! for example, text, [`BotCommand`] (just alias to text of command) or [`Regex`].
//! You can create a filter with `new` method with transferring all necessary data at once, or use [`CommandBuilder`] to create a filter step by step.
//! Instead of [`CommandBuilder`] you can use [`Command`] `one`, `one_with_prefix`, `many`, `many_with_prefix` methods.
//! * [`ContentType`]:
//! Filter for checking the type of content.
//! Usually used with [`ContentTypeEnum`] (or its string representation) to check the type of content.
//! Creates with `one` or `many` methods.
//! * [`State`]:
//! Filter for checking the state of the user/chat/etc.
//! Filter accepts [`StateType`] that represents a state type for verification,
//! for example, equal, any or none.
//! You can create a filter with `one` or `many` if you want to check the state with the exact value
//! or use `any` or `none` if you want to check the state with any value or without state, respectively.
//! * [`Text`]:
//! This filter checks if the text matches the specified pattern.
//! Gets the text from the [`update`], the text of the message, the text of the inline query, the data of the callback query, etc.
//! Filter accepts [`text pattern type`] that represents a text pattern type to check for equality, so you can use [`Regex`] or [`Cow`] to check the text.
//! You can create a filter with `one` or `many` if you want to check the text with the exact value.
//! If you want to check the text with `contains`, `starts_with` or `ends_with` methods that accept only [`Cow`],
//! you can create a filter with `contains_single`, `contains`, `starts_with_single`, `starts_with`, `ends_with_single, or `ends_with` methods,
//! or use [`TextBuilder`] to create a filter step by step.
//! * [`User`]:
//! Filter for checking the user.
//! This filter checks if the user username, first name, last name, language code or ID is equal to one of the specified.
//! You can create a filter with `new` method with transferring all necessary data at once, or use [`UserBuilder`] to create a filter step by step.
//! Instead of [`UserBuilder`] you can use [`User`] `username`, `usernames`, `first_name`, `first_names`, `last_name`, `last_names`,
//! `language_code`, `language_codes`, `id` or `ids` methods.
//! This filter checks user data step by step using the logical operator `or`,
//! so if at least one check is successful, the filter will return the value `true`.
//!
//! [`Cow`]: std::borrow::Cow
//! [`Regex`]: regex::Regex
//! [`ChatTypeEnum`]: crate::enums::ChatType
//! [`ContentTypeEnum`]: crate::enums::ContentType
//! [`BotCommand`]: crate::types::BotCommand
//! [`Regex`]: regex::Regex
//! [`context`]: crate::context::Context
//! [`command pattern type`]: command::PatternType
//! [`text pattern type`]: text::PatternTyp
//! [`bot`]: crate::client::Bot
//! [`update`]: crate::types::Update

pub mod base;
pub mod chat_type;
pub mod command;
pub mod content_type;
pub mod logical;
pub mod state;
pub mod text;
pub mod user;

pub use base::Filter;
pub use chat_type::ChatType;
pub use command::{Command, CommandBuilder, CommandObject};
pub use content_type::ContentType;
pub use logical::{And, Invert, Or};
pub use state::{State, StateType};
pub use text::{Text, TextBuilder};
pub use user::{User, UserBuilder};
