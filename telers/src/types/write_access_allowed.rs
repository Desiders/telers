use serde::Deserialize;

/// This object represents a service message about a user allowing a bot to write messages after adding it to the attachment menu, launching a Web App from a link, or accepting an explicit request from a Web App sent by the method [requestWriteAccess](https://core.telegram.org/bots/webapps#initializing-mini-apps).
/// # Documentation
/// <https://core.telegram.org/bots/api#writeaccessallowed>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
pub struct WriteAccessAllowed {
    /// `true`, if the access was granted after the user accepted an explicit request from a Web App sent by the method [requestWriteAccess](https://core.telegram.org/bots/webapps#initializing-mini-apps)
    pub from_request: Option<bool>,
    /// Name of the Web App which was launched from a link
    pub web_app_name: Option<Box<str>>,
    /// `true`, if the access was granted when the bot was added to the attachment or side menu
    pub from_attachment_menu: Option<bool>,
}
