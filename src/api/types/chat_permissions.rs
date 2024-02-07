use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#chatpermissions
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ChatPermissions {
    can_send_messages: Option<bool>,
    can_send_audios: Option<bool>,
    can_send_documents: Option<bool>,
    can_send_photos: Option<bool>,
    can_send_videos: Option<bool>,
    can_send_video_notes: Option<bool>,
    can_send_voice_notes: Option<bool>,
    can_send_polls: Option<bool>,
    can_send_other_messages: Option<bool>,
    can_add_web_page_previews: Option<bool>,
    can_change_info: Option<bool>,
    can_invite_users: Option<bool>,
    can_pin_messages: Option<bool>,
    can_manage_topics: Option<bool>,
}
