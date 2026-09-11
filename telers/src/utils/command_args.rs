//! Helpers for parsing the arguments of a command into typed fields.
//!
//! [`CommandArg`] parses a field of a command from an [`ArgsCursor`], so a field can take any
//! number of arguments: one (numbers, strings, `bool`, `char`, IP addresses), none or one
//! ([`Option<T>`]), all the remaining ones ([`Vec<T>`]) or the rest of the text as is ([`Rest`]).
//! It is also implemented for tuples of fields up to 16 elements, and [`parse_args`] parses
//! all the arguments of a command into one, checking that none are left:
//!
//! ```rust
//! use telers::utils::command_args::{parse_args, ArgsCursor, Rest, SplitType};
//!
//! let cursor = ArgsCursor::new("@username 30 spam in the chat", SplitType::Whitespace);
//! let (username, minutes, reason) = parse_args::<(String, Option<u32>, Rest)>(cursor).unwrap();
//! assert_eq!(username, "@username");
//! assert_eq!(minutes, Some(30));
//! assert_eq!(reason.0.as_ref(), "spam in the chat");
//! ```
//!
//! Implement [`CommandArg`] for your types to use them as fields of commands,
//! or use the [`command_arg_via_from_str`](crate::command_arg_via_from_str) macro
//! if the type implements [`FromStr`](std::str::FromStr).
//!
//! [`Commands`] and [`CommandKind`] are implemented by the [`Command`](crate::Command) derive
//! for the enum of the commands and its kinds, so the [`Command`](crate::filters::Command) filter
//! matches the messages against them.

use crate::{errors::ExtractionError, filters::CommandObject};

use std::{
    fmt::{Debug, Display},
    mem,
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
    sync::Arc,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplitType {
    Whitespace,
    Char(char),
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum CommandArgsError {
    #[error("Missing argument for the field at position {index}")]
    Missing { index: usize },
    #[error("Too many arguments: expected {expected}, but got {actual}")]
    TooMany { expected: usize, actual: usize },
    #[error("Invalid value `{value}` for the field at position {index}")]
    InvalidValue {
        index: usize,
        value: Box<str>,
        source: Arc<anyhow::Error>,
    },
    #[error(transparent)]
    Custom(Arc<anyhow::Error>),
}

/// To possible to wrap [`anyhow::Error`] error in [`CommandArgsError`] enum without boilerplate code
impl From<anyhow::Error> for CommandArgsError {
    fn from(err: anyhow::Error) -> Self {
        Self::Custom(Arc::new(err))
    }
}

impl CommandArgsError {
    /// # Arguments
    /// * `info` - The error message.
    /// # Notes
    /// This method is useful when you want to pass just a message.
    /// If you want to pass an error, you can convert it with `?` or `into`.
    pub fn from_display(info: impl Display) -> Self {
        Self::from(anyhow::anyhow!("{info}"))
    }

    /// # Arguments
    /// * `info` - The error message.
    /// # Notes
    /// This method is useful when you want to pass just a message.
    /// If you want to pass an error, you can convert it with `?` or `into`.
    pub fn from_debug(info: impl Debug) -> Self {
        Self::from(anyhow::anyhow!("{info:?}"))
    }

    #[must_use]
    pub fn at_index(self, index: usize) -> Self {
        match self {
            Self::Missing {
                ..
            } => Self::Missing {
                index,
            },
            Self::InvalidValue {
                value,
                source,
                ..
            } => Self::InvalidValue {
                index,
                value,
                source,
            },
            err => err,
        }
    }

    #[must_use]
    pub fn describe(&self, command: &str, fields: &[&str]) -> String {
        let field = |index: usize| match fields.get(index) {
            Some(name) => format!("`{name}`"),
            None => format!("at position {index}"),
        };

        match self {
            Self::Missing {
                index,
            } => {
                format!("Missing argument {} for `{command}` command", field(*index))
            }
            Self::TooMany {
                expected,
                actual,
            } => format!(
                "Too many arguments for `{command}` command: expected {expected}, but got {actual}"
            ),
            Self::InvalidValue {
                index,
                value,
                source,
            } => format!(
                "Invalid value `{value}` for argument {} of `{command}` command: {source:#}",
                field(*index)
            ),
            Self::Custom(source) => {
                format!("Invalid arguments for `{command}` command: {source:#}")
            }
        }
    }
}

/// Commands of a bot, implemented by the [`Command`](crate::Command) derive,
/// so the [`Command`](crate::filters::Command) filter checks a message against all of them
/// with [`Command::all`](crate::filters::Command::all)
/// # Notes
/// The enum must be [`Clone`], because the filter keeps the parsed command in the context
/// for the extraction in the handler when it parses the arguments
pub trait Commands: Clone + Send + Sync + 'static {
    /// Kind of the command, the variant of the enum without its arguments, one for each command
    type Kind: CommandKind<Commands = Self>;

    /// Kind of the command by its prefix and lowercase name, an alias of the command too,
    /// `None` if it isn't one of the commands
    fn kind(prefix: char, name: &str) -> Option<Self::Kind>;

    /// Parses the command with its arguments into the enum
    /// # Errors
    /// - If the command isn't one of the commands
    /// - If the arguments can't be parsed into the fields of the command
    fn parse(command: &CommandObject) -> Result<Self, ExtractionError>;
}

/// Kind of a command of [`Commands`], implemented by the [`Command`](crate::Command) derive
/// for the generated `<Enum>Type` enum, so the [`Command`](crate::filters::Command) filter
/// checks a message against the commands of the kinds
pub trait CommandKind: Copy + Debug + PartialEq + Send + Sync + 'static {
    /// Commands the kind belongs to
    type Commands: Commands<Kind = Self>;
}

/// Parses a single field of a command from its arguments.
///
/// This trait is implemented for numbers, strings, `bool`, `char`, IP addresses,
/// the combinators [`Option<T>`], [`Vec<T>`] and [`Rest`], and tuples of fields up to 16 elements.
/// Implement it for your types to use them as fields of commands,
/// or use the [`command_arg_via_from_str`](crate::command_arg_via_from_str) macro
/// if the type implements [`FromStr`](std::str::FromStr).
pub trait CommandArg: Sized {
    /// Parses the field, taking as many arguments as it needs
    ///
    /// # Errors
    /// - If there is no argument for the field
    /// - If an argument can't be parsed to the field type
    fn parse_arg(cursor: &mut ArgsCursor<'_>) -> Result<Self, CommandArgsError>;
}

/// Cursor over the arguments of a command.
///
/// [`next_arg`](Self::next_arg) takes one argument, [`take_rest`](Self::take_rest) takes
/// everything that is left as is. Separators before an argument are skipped,
/// so `a  b` gives the same arguments as `a b`. In [`SplitType::Char`] mode whitespace
/// around an argument is skipped too, so `a, b` gives the same arguments as `a,b`.
#[derive(Debug, Clone, Copy)]
pub struct ArgsCursor<'a> {
    rest: &'a str,
    split: SplitType,
}

impl<'a> ArgsCursor<'a> {
    #[must_use]
    pub const fn new(rest: &'a str, split: SplitType) -> Self {
        Self {
            rest,
            split,
        }
    }

    pub fn next_arg(&mut self) -> Option<&'a str> {
        self.skip_separators();

        if self.rest.is_empty() {
            return None;
        }

        let separator = match self.split {
            SplitType::Whitespace => self.rest.find(char::is_whitespace),
            SplitType::Char(separator) => self.rest.find(separator),
        };
        let (arg, rest) = match separator {
            Some(index) => self.rest.split_at(index),
            None => (self.rest, ""),
        };
        self.rest = rest;

        Some(arg.trim_end())
    }

    pub fn take_rest(&mut self) -> &'a str {
        self.skip_separators();

        mem::take(&mut self.rest)
    }

    #[must_use]
    pub fn is_exhausted(&self) -> bool {
        self.without_separators().is_empty()
    }

    #[must_use]
    pub fn remaining_count(&self) -> usize {
        let mut cursor = *self;
        let mut count = 0;

        while cursor.next_arg().is_some() {
            count += 1;
        }

        count
    }

    fn skip_separators(&mut self) {
        self.rest = self.without_separators();
    }

    fn without_separators(&self) -> &'a str {
        match self.split {
            SplitType::Whitespace => self.rest.trim_start(),
            SplitType::Char(separator) => self
                .rest
                .trim_start_matches(|ch: char| ch == separator || ch.is_whitespace()),
        }
    }
}

/// Implements [`CommandArg`] for types that implement [`FromStr`]:
/// the field takes one argument and parses it.
///
/// # Examples
/// ```rust
/// use std::{num::ParseIntError, str::FromStr};
///
/// struct UserId(i64);
///
/// impl FromStr for UserId {
///     type Err = ParseIntError;
///
///     fn from_str(value: &str) -> Result<Self, Self::Err> {
///         value.trim_start_matches('#').parse().map(Self)
///     }
/// }
///
/// telers::command_arg_via_from_str!(UserId);
/// ```
///
/// [`CommandArg`]: crate::utils::command_args::CommandArg
/// [`FromStr`]: std::str::FromStr
#[macro_export]
macro_rules! command_arg_via_from_str {
    ($($ty:ty),* $(,)?) => {
        $(
            impl $crate::utils::command_args::CommandArg for $ty {
                #[inline]
                fn parse_arg(
                    cursor: &mut $crate::utils::command_args::ArgsCursor<'_>,
                ) -> ::std::result::Result<Self, $crate::utils::command_args::CommandArgsError> {
                    let value = cursor.next_arg().ok_or(
                        $crate::utils::command_args::CommandArgsError::Missing { index: 0 },
                    )?;

                    value.parse::<Self>().map_err(|err| {
                        $crate::utils::command_args::CommandArgsError::InvalidValue {
                            index: 0,
                            value: value.into(),
                            source: ::std::sync::Arc::new(err.into()),
                        }
                    })
                }
            }
        )*
    };
}

command_arg_via_from_str!(
    String, char, bool, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64,
    IpAddr, Ipv4Addr, Ipv6Addr,
);

impl CommandArg for Box<str> {
    #[inline]
    fn parse_arg(cursor: &mut ArgsCursor<'_>) -> Result<Self, CommandArgsError> {
        cursor
            .next_arg()
            .map(Into::into)
            .ok_or(CommandArgsError::Missing {
                index: 0,
            })
    }
}

impl<T: CommandArg> CommandArg for Option<T> {
    #[inline]
    fn parse_arg(cursor: &mut ArgsCursor<'_>) -> Result<Self, CommandArgsError> {
        if cursor.is_exhausted() {
            Ok(None)
        } else {
            T::parse_arg(cursor).map(Some)
        }
    }
}

impl<T: CommandArg> CommandArg for Vec<T> {
    fn parse_arg(cursor: &mut ArgsCursor<'_>) -> Result<Self, CommandArgsError> {
        let mut values = Vec::new();

        while !cursor.is_exhausted() {
            values.push(T::parse_arg(cursor)?);
        }

        Ok(values)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rest(pub Box<str>);

impl CommandArg for Rest {
    #[inline]
    fn parse_arg(cursor: &mut ArgsCursor<'_>) -> Result<Self, CommandArgsError> {
        Ok(Self(cursor.take_rest().into()))
    }
}

/// Parses the field at `index` and moves `index` to the next one
fn parse_field<T: CommandArg>(
    cursor: &mut ArgsCursor<'_>,
    index: &mut usize,
) -> Result<T, CommandArgsError> {
    let arg = T::parse_arg(cursor).map_err(|err| err.at_index(*index))?;
    *index += 1;

    Ok(arg)
}

/// The fields are parsed in order, each taking as many arguments as it needs
macro_rules! impl_command_arg_tuple {
    ([]) => {
        impl CommandArg for () {
            fn parse_arg(_: &mut ArgsCursor<'_>) -> Result<Self, CommandArgsError> {
                Ok(())
            }
        }
    };
    ([$($ty:ident),+]) => {
        impl<$($ty: CommandArg),+> CommandArg for ($($ty,)+) {
            fn parse_arg(cursor: &mut ArgsCursor<'_>) -> Result<Self, CommandArgsError> {
                let mut index = 0;

                Ok(($(parse_field::<$ty>(cursor, &mut index)?,)+))
            }
        }
    };
}

all_the_tuples!(impl_command_arg_tuple);

/// Parses all the arguments of a command into `T`, usually a tuple of the fields
///
/// # Errors
/// - If there is no argument for a field
/// - If there are more arguments than the fields take
/// - If an argument can't be parsed to the field type
pub fn parse_args<T: CommandArg>(mut cursor: ArgsCursor<'_>) -> Result<T, CommandArgsError> {
    let count = cursor.remaining_count();
    let args = T::parse_arg(&mut cursor)?;

    match cursor.remaining_count() {
        0 => Ok(args),
        extra => Err(CommandArgsError::TooMany {
            expected: count - extra,
            actual: count,
        }),
    }
}
