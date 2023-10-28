use super::base::Filter;

use crate::{
    client::Bot,
    context::Context,
    types::{Update, User as UserType},
};

use async_trait::async_trait;
use std::borrow::Cow;

/// Filter for checking the user.
/// This filter checks if the user username, first name, last name, language code or ID is equal to one of the specified.
/// # Notes
/// This filter checks user data step by step using the logical operator `or`,
/// so if at least one check is successful, the filter will return the value `true`.
#[derive(Debug, Clone)]
pub struct User<'a> {
    /// List of usernames of the users
    usernames: Box<[Cow<'a, str>]>,
    /// List of first names of the users
    first_names: Box<[Cow<'a, str>]>,
    /// List of last names of the users
    last_names: Box<[Cow<'a, str>]>,
    /// List of language codes of the users
    language_codes: Box<[Cow<'a, str>]>,
    /// List of user IDs of the users
    ids: Box<[i64]>,
}

impl<'a> User<'a> {
    /// Creates a new [`User`] filter
    /// # Arguments
    /// * `usernames` - List of usernames of the users
    /// * `first_names` - List of first names of the users
    /// * `last_names` - List of last names of the users
    /// * `language_codes` - List of language codes of the users
    /// * `ids` - List of user IDs of the users
    /// # Notes
    /// This filter checks user data step by step using the logical operator `or`,
    /// so if at least one check is successful, the filter will return the value `true`.
    pub fn new<T, I1, C, I2, S, I3, E, I4, I5>(
        usernames: I1,
        first_names: I2,
        last_names: I3,
        language_codes: I4,
        ids: I5,
    ) -> Self
    where
        T: Into<Cow<'a, str>>,
        I1: IntoIterator<Item = T>,
        C: Into<Cow<'a, str>>,
        I2: IntoIterator<Item = C>,
        S: Into<Cow<'a, str>>,
        I3: IntoIterator<Item = S>,
        E: Into<Cow<'a, str>>,
        I4: IntoIterator<Item = E>,
        I5: IntoIterator<Item = i64>,
    {
        Self {
            usernames: usernames.into_iter().map(Into::into).collect(),
            first_names: first_names.into_iter().map(Into::into).collect(),
            last_names: last_names.into_iter().map(Into::into).collect(),
            language_codes: language_codes.into_iter().map(Into::into).collect(),
            ids: ids.into_iter().collect(),
        }
    }

    /// Creates a new [`User`] filter with a single username
    /// # Notes
    /// This method is just a shortcut to create a filter using the builder
    #[must_use]
    pub fn username(val: impl Into<Cow<'a, str>>) -> Self {
        Self::builder().username(val).build()
    }

    /// Creates a new [`User`] filter with a list of usernames
    /// # Notes
    /// This method is just a shortcut to create a filter using the builder
    #[must_use]
    pub fn usernames<T, I>(val: I) -> Self
    where
        T: Into<Cow<'a, str>>,
        I: IntoIterator<Item = T>,
    {
        Self::builder().usernames(val).build()
    }

    /// Creates a new [`User`] filter with a single first name
    /// # Notes
    /// This method is just a shortcut to create a filter using the builder
    #[must_use]
    pub fn first_name(val: impl Into<Cow<'a, str>>) -> Self {
        Self::builder().first_name(val).build()
    }

    /// Creates a new [`User`] filter with a list of first names
    /// # Notes
    /// This method is just a shortcut to create a filter using the builder
    #[must_use]
    pub fn first_names<T, I>(val: I) -> Self
    where
        T: Into<Cow<'a, str>>,
        I: IntoIterator<Item = T>,
    {
        Self::builder().first_names(val).build()
    }

    /// Creates a new [`User`] filter with a single last name
    /// # Notes
    /// This method is just a shortcut to create a filter using the builder
    #[must_use]
    pub fn last_name(val: impl Into<Cow<'a, str>>) -> Self {
        Self::builder().last_name(val).build()
    }

    /// Creates a new [`User`] filter with a list of last names
    /// # Notes
    /// This method is just a shortcut to create a filter using the builder
    #[must_use]
    pub fn last_names<T, I>(val: I) -> Self
    where
        T: Into<Cow<'a, str>>,
        I: IntoIterator<Item = T>,
    {
        Self::builder().last_names(val).build()
    }

    /// Creates a new [`User`] filter with a single language code
    /// # Notes
    /// This method is just a shortcut to create a filter using the builder
    #[must_use]
    pub fn language_code(val: impl Into<Cow<'a, str>>) -> Self {
        Self::builder().language_code(val).build()
    }

    /// Creates a new [`User`] filter with a list of language codes
    /// # Notes
    /// This method is just a shortcut to create a filter using the builder
    #[must_use]
    pub fn language_codes<T, I>(val: I) -> Self
    where
        T: Into<Cow<'a, str>>,
        I: IntoIterator<Item = T>,
    {
        Self::builder().language_codes(val).build()
    }

    /// Creates a new [`User`] filter with a single user ID
    /// # Notes
    /// This method is just a shortcut to create a filter using the builder
    #[must_use]
    pub fn id(val: i64) -> Self {
        Self::builder().id(val).build()
    }

    /// Creates a new [`User`] filter with a list of user IDs
    /// # Notes
    /// This method is just a shortcut to create a filter using the builder
    #[must_use]
    pub fn ids(val: impl IntoIterator<Item = i64>) -> Self {
        Self::builder().ids(val).build()
    }

    #[must_use]
    pub fn builder() -> UserBuilder<'a> {
        UserBuilder::default()
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Default, Clone)]
pub struct UserBuilder<'a> {
    usernames: Vec<Cow<'a, str>>,
    first_names: Vec<Cow<'a, str>>,
    last_names: Vec<Cow<'a, str>>,
    language_codes: Vec<Cow<'a, str>>,
    ids: Vec<i64>,
}

impl<'a> UserBuilder<'a> {
    #[must_use]
    pub fn username(self, val: impl Into<Cow<'a, str>>) -> Self {
        Self {
            usernames: self.usernames.into_iter().chain(Some(val.into())).collect(),
            ..self
        }
    }

    #[must_use]
    pub fn usernames<T, I>(self, val: I) -> Self
    where
        T: Into<Cow<'a, str>>,
        I: IntoIterator<Item = T>,
    {
        Self {
            usernames: self
                .usernames
                .into_iter()
                .chain(val.into_iter().map(Into::into))
                .collect(),
            ..self
        }
    }

    #[must_use]
    pub fn first_name(self, val: impl Into<Cow<'a, str>>) -> Self {
        Self {
            first_names: self
                .first_names
                .into_iter()
                .chain(Some(val.into()))
                .collect(),
            ..self
        }
    }

    #[must_use]
    pub fn first_names<T, I>(self, val: I) -> Self
    where
        T: Into<Cow<'a, str>>,
        I: IntoIterator<Item = T>,
    {
        Self {
            first_names: self
                .first_names
                .into_iter()
                .chain(val.into_iter().map(Into::into))
                .collect(),
            ..self
        }
    }

    #[must_use]
    pub fn last_name(self, val: impl Into<Cow<'a, str>>) -> Self {
        Self {
            last_names: self
                .last_names
                .into_iter()
                .chain(Some(val.into()))
                .collect(),
            ..self
        }
    }

    #[must_use]
    pub fn last_names<T, I>(self, val: I) -> Self
    where
        T: Into<Cow<'a, str>>,
        I: IntoIterator<Item = T>,
    {
        Self {
            last_names: self
                .last_names
                .into_iter()
                .chain(val.into_iter().map(Into::into))
                .collect(),
            ..self
        }
    }

    #[must_use]
    pub fn language_code(self, val: impl Into<Cow<'a, str>>) -> Self {
        Self {
            language_codes: self
                .language_codes
                .into_iter()
                .chain(Some(val.into()))
                .collect(),
            ..self
        }
    }

    #[must_use]
    pub fn language_codes<T, I>(self, val: I) -> Self
    where
        T: Into<Cow<'a, str>>,
        I: IntoIterator<Item = T>,
    {
        Self {
            language_codes: self
                .language_codes
                .into_iter()
                .chain(val.into_iter().map(Into::into))
                .collect(),
            ..self
        }
    }

    #[must_use]
    pub fn id(self, val: i64) -> Self {
        Self {
            ids: self.ids.into_iter().chain(Some(val)).collect(),
            ..self
        }
    }

    #[must_use]
    pub fn ids(self, val: impl IntoIterator<Item = i64>) -> Self {
        Self {
            ids: self
                .ids
                .into_iter()
                .chain(val.into_iter().map(Into::into))
                .collect(),
            ..self
        }
    }

    #[must_use]
    pub fn build(self) -> User<'a> {
        User::new(
            self.usernames,
            self.first_names,
            self.last_names,
            self.language_codes,
            self.ids,
        )
    }
}

impl<'a> User<'a> {
    #[must_use]
    pub fn validate_username(&self, username: &str) -> bool {
        self.usernames
            .iter()
            .any(|allowed_username| allowed_username.as_ref() == username)
    }

    #[must_use]
    pub fn validate_first_name(&self, first_name: &str) -> bool {
        self.first_names
            .iter()
            .any(|allowed_first_name| allowed_first_name.as_ref() == first_name)
    }

    #[must_use]
    pub fn validate_last_name(&self, last_name: &str) -> bool {
        self.last_names
            .iter()
            .any(|allowed_last_name| allowed_last_name.as_ref() == last_name)
    }

    #[must_use]
    pub fn validate_language_code(&self, language_code: &str) -> bool {
        self.language_codes
            .iter()
            .any(|allowed_language_code| allowed_language_code.as_ref() == language_code)
    }

    #[must_use]
    pub fn validate_id(&self, id: i64) -> bool {
        self.ids.iter().any(|allowed_id| *allowed_id == id)
    }

    #[must_use]
    pub fn validate(&self, user: &UserType) -> bool {
        user.username
            .as_deref()
            .map_or(false, |username| self.validate_username(username))
            || self.validate_id(user.id)
            || self.validate_first_name(&user.first_name)
            || user
                .last_name
                .as_deref()
                .map_or(false, |last_name| self.validate_last_name(last_name))
            || user
                .language_code
                .as_deref()
                .map_or(false, |language_code| {
                    self.validate_language_code(language_code)
                })
    }
}

#[async_trait]
impl<Client> Filter<Client> for User<'_> {
    async fn check(&self, _bot: &Bot<Client>, update: &Update, _context: &Context) -> bool {
        update.user().map_or(false, |user| self.validate(user))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_username() {
        let user = User::username("test");

        assert!(user.validate_username("test"));
        assert!(!user.validate_username("test2"));

        let user = User::usernames(["test", "test2"]);

        assert!(user.validate_username("test"));
        assert!(user.validate_username("test2"));
    }

    #[test]
    fn test_validate_first_name() {
        let user = User::first_name("test");

        assert!(user.validate_first_name("test"));
        assert!(!user.validate_first_name("test2"));

        let user = User::first_names(["test", "test2"]);

        assert!(user.validate_first_name("test"));
        assert!(user.validate_first_name("test2"));
    }

    #[test]
    fn test_validate_last_name() {
        let user = User::last_name("test");

        assert!(user.validate_last_name("test"));
        assert!(!user.validate_last_name("test2"));

        let user = User::last_names(["test", "test2"]);

        assert!(user.validate_last_name("test"));
        assert!(user.validate_last_name("test2"));
    }

    #[test]
    fn test_validate_language_code() {
        let user = User::language_code("test");

        assert!(user.validate_language_code("test"));
        assert!(!user.validate_language_code("test2"));

        let user = User::language_codes(["test", "test2"]);

        assert!(user.validate_language_code("test"));
        assert!(user.validate_language_code("test2"));
    }

    #[test]
    fn test_validate_id() {
        let user = User::id(1);

        assert!(user.validate_id(1));
        assert!(!user.validate_id(2));

        let user = User::ids([1, 2]);

        assert!(user.validate_id(1));
        assert!(user.validate_id(2));
    }
}
