use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#videochatstarted
/// This object represents a service message about a video chat started in the chat. Currently holds no information.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct VideoChatStarted {}
