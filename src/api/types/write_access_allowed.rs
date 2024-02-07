use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#writeaccessallowed
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct WriteAccessAllowed {
    from_request: Option<bool>,
    web_app_name: Option<String>,
    from_attachment_menu: Option<bool>,
}
