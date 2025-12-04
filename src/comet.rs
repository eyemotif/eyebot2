use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", content = "payload", rename_all = "snake_case")]
pub enum Message {
    Register {
        state: String,
    },
    GetComponents {}, // TODO
    PlayAudio {},     // TODO
    AudioVolume {},   // TODO
    AudioClear {},
    ChatSetEmotes {
        username: String,
    },
    Chat {
        user_id: String,
        chat: Vec<ChatFragment>,
        meta: ChatMetadata,
    },
    ChatUser {
        user_id: String,
        chat_info: Chatter,
    },
    ChatClear {
        user_id: Option<String>,
    },
    Features {},
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ChatFragment {
    Text { content: String },
    Emote { emote: String },
}
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct Chatter {
    pub display_name: String,
    pub name_color: String,
    pub badges: Vec<String>,
}
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ChatMetadata {
    None,
    Action,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Response {
    Ok {
        state: String,
    },
    Data {
        state: String,
        payload: String,
    },
    Error {
        state: String,
        is_internal: bool,
        message: String,
    },
}
