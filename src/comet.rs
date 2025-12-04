use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", content = "payload", rename_all = "snake_case")]
pub enum Message {
    Register {
        state: String,
    },
    GetComponents {
        #[serde(rename = "type")]
        component_type: ComponentType,
    },
    PlayAudio {
        data: Vec<Vec<AudioComponent>>,
    },
    AudioVolume {
        #[serde(rename = "name")]
        audio_component_name: String,
        #[serde(rename = "value")]
        /// A floating point value between 0.0 and 1.0
        volume_value: f64,
    },
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

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ComponentType {
    Audio,
}
#[derive(Debug, Clone, Serialize)]
pub struct AudioComponent {
    pub name: String,
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
