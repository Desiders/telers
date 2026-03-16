use serde::{Deserialize, Serialize};
/// This object represents a service message about a user allowing a bot to write messages after adding it to the attachment menu, launching a Web App from a link, or accepting an explicit request from a Web App sent by the method requestWriteAccess.
/// # Documentation
/// <https://core.telegram.org/bots/api#writeaccessallowed>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WriteAccessAllowed {
    /// `true`, if the access was granted after the user accepted an explicit request from a Web App sent by the method requestWriteAccess
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_request: Option<bool>,
    /// Name of the Web App, if the access was granted when the Web App was launched from a link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app_name: Option<Box<str>>,
    /// `true`, if the access was granted when the bot was added to the attachment or side menu
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_attachment_menu: Option<bool>,
}
impl WriteAccessAllowed {
    /// Creates a new `WriteAccessAllowed`.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new() -> Self {
        Self {
            from_request: None,
            web_app_name: None,
            from_attachment_menu: None,
        }
    }

    /// `true`, if the access was granted after the user accepted an explicit request from a Web App sent by the method requestWriteAccess
    #[must_use]
    pub fn from_request<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.from_request = Some(val.into());
        this
    }

    /// `true`, if the access was granted after the user accepted an explicit request from a Web App sent by the method requestWriteAccess
    #[must_use]
    pub fn from_request_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.from_request = val.map(Into::into);
        this
    }

    /// Name of the Web App, if the access was granted when the Web App was launched from a link
    #[must_use]
    pub fn web_app_name<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.web_app_name = Some(val.into());
        this
    }

    /// Name of the Web App, if the access was granted when the Web App was launched from a link
    #[must_use]
    pub fn web_app_name_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.web_app_name = val.map(Into::into);
        this
    }

    /// `true`, if the access was granted when the bot was added to the attachment or side menu
    #[must_use]
    pub fn from_attachment_menu<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.from_attachment_menu = Some(val.into());
        this
    }

    /// `true`, if the access was granted when the bot was added to the attachment or side menu
    #[must_use]
    pub fn from_attachment_menu_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.from_attachment_menu = val.map(Into::into);
        this
    }
}
impl Default for WriteAccessAllowed {
    fn default() -> Self {
        Self::new()
    }
}
