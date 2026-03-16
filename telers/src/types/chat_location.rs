use serde::{Deserialize, Serialize};
/// Represents a location to which a chat is connected.
/// # Documentation
/// <https://core.telegram.org/bots/api#chatlocation>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatLocation {
    /// The location to which the supergroup is connected. Can't be a live location.
    pub location: crate::types::Location,
    /// Location address; 1-64 characters, as defined by the chat owner
    pub address: Box<str>,
}
impl ChatLocation {
    /// Creates a new `ChatLocation`.
    ///
    /// # Arguments
    /// * `location` - The location to which the supergroup is connected. Can't be a live location.
    /// * `address` - Location address; 1-64 characters, as defined by the chat owner
    #[must_use]
    pub fn new<T0: Into<crate::types::Location>, T1: Into<Box<str>>>(
        location: T0,
        address: T1,
    ) -> Self {
        Self {
            location: location.into(),
            address: address.into(),
        }
    }

    /// The location to which the supergroup is connected. Can't be a live location.
    #[must_use]
    pub fn location<T: Into<crate::types::Location>>(self, val: T) -> Self {
        let mut this = self;
        this.location = val.into();
        this
    }

    /// Location address; 1-64 characters, as defined by the chat owner
    #[must_use]
    pub fn address<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.address = val.into();
        this
    }
}
