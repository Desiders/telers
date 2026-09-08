//! Helpers for parsing the arguments of a command into typed fields.
//!
//! [`CommandArgs`] parses all fields of a command and [`CommandArg`] parses a single one.
//! The arguments are taken from an [`ArgsCursor`], so a field can take any number of them:
//! one (numbers, strings, `bool`, `char`, IP addresses), none or one ([`Option<T>`]),
//! all the remaining ones ([`Vec<T>`]) or the rest of the text as is ([`Rest`]).
//!
//! `CommandArgs` is implemented for tuples of `CommandArg` fields up to 16 elements:
//!
//! ```rust
//! use telers::utils::command_args::{ArgsCursor, CommandArgs, Rest, SplitKind};
//!
//! let cursor = ArgsCursor::new("@username 30 spam in the chat", SplitKind::Whitespace);
//! let (username, minutes, reason) = <(String, Option<u32>, Rest)>::parse_args(cursor).unwrap();
//! assert_eq!(username, "@username");
//! assert_eq!(minutes, Some(30));
//! assert_eq!(reason.0.as_ref(), "spam in the chat");
//! ```
//!
//! Implement [`CommandArg`] for your types to use them as fields of commands,
//! or use the [`command_arg_via_from_str`](crate::command_arg_via_from_str) macro
//! if the type implements [`FromStr`](std::str::FromStr).
//! Implement [`CommandArgs`] for your type to parse all the arguments manually.

use std::{
    mem,
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplitKind {
    Whitespace,
    Char(char),
}

/// Cursor over the arguments of a command.
///
/// [`next_arg`](Self::next_arg) takes one argument, [`take_rest`](Self::take_rest) takes
/// everything that is left as is. Separators before an argument are skipped,
/// so `a  b` gives the same arguments as `a b`.
#[derive(Debug, Clone, Copy)]
pub struct ArgsCursor<'a> {
    rest: &'a str,
    split: SplitKind,
}

impl<'a> ArgsCursor<'a> {
    #[must_use]
    pub const fn new(rest: &'a str, split: SplitKind) -> Self {
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
            SplitKind::Whitespace => self.rest.find(char::is_whitespace),
            SplitKind::Char(separator) => self.rest.find(separator),
        };
        let (arg, rest) = match separator {
            Some(index) => self.rest.split_at(index),
            None => (self.rest, ""),
        };
        self.rest = rest;

        Some(arg)
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
            SplitKind::Whitespace => self.rest.trim_start(),
            SplitKind::Char(separator) => self.rest.trim_start_matches(separator),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CommandArgsError {
    #[error("Missing argument for the field at position {index}")]
    Missing { index: usize },
    #[error("Too many arguments: expected {expected}, but got {actual}")]
    TooMany { expected: usize, actual: usize },
    #[error("Invalid value `{value}` for the field at position {index}")]
    InvalidValue {
        index: usize,
        value: Box<str>,
        source: anyhow::Error,
    },
    #[error(transparent)]
    Custom(#[from] anyhow::Error),
}

impl CommandArgsError {
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

/// Parses a single field of a command from its arguments.
///
/// This trait is implemented for numbers, strings, `bool`, `char`, IP addresses and
/// the combinators [`Option<T>`], [`Vec<T>`] and [`Rest`].
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

/// Parses all fields of a command from its arguments.
///
/// This trait is implemented for tuples of [`CommandArg`] fields up to 16 elements.
/// Implement it for your type to parse all the arguments manually.
pub trait CommandArgs: Sized {
    /// Parses all fields, checking that no arguments are left
    ///
    /// # Errors
    /// - If there is no argument for a field
    /// - If there are more arguments than fields
    /// - If an argument can't be parsed to the field type
    fn parse_args(cursor: ArgsCursor<'_>) -> Result<Self, CommandArgsError>;
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
                            source: err.into(),
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
    fn parse_arg(cursor: &mut ArgsCursor<'_>) -> Result<Self, CommandArgsError> {
        Ok(Self(cursor.take_rest().into()))
    }
}

fn ensure_exhausted(cursor: &ArgsCursor<'_>, expected: usize) -> Result<(), CommandArgsError> {
    match cursor.remaining_count() {
        0 => Ok(()),
        extra => Err(CommandArgsError::TooMany {
            expected,
            actual: expected + extra,
        }),
    }
}

macro_rules! impl_command_args {
    ([]) => {
        impl CommandArgs for () {
            fn parse_args(cursor: ArgsCursor<'_>) -> Result<Self, CommandArgsError> {
                ensure_exhausted(&cursor, 0)
            }
        }
    };
    ([$($ty:ident),+]) => {
        impl<$($ty: CommandArg),+> CommandArgs for ($($ty,)+) {
            fn parse_args(mut cursor: ArgsCursor<'_>) -> Result<Self, CommandArgsError> {
                let mut index = 0;
                let args = ($(
                    {
                        let arg = $ty::parse_arg(&mut cursor).map_err(|err| err.at_index(index))?;
                        index += 1;
                        arg
                    },
                )+);

                ensure_exhausted(&cursor, index)?;

                Ok(args)
            }
        }
    };
}

all_the_tuples!(impl_command_args);
