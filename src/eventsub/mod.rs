use serde::Deserialize;

pub mod event;
pub mod payload;

#[derive(Debug, Clone, Deserialize)]
pub struct TwitchMessage {
    pub metadata: Metadata,
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Metadata {
    pub message_type: MessageType,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageType {
    SessionWelcome,
    SessionKeepalive,
    SessionReconnect,

    Notification,
}
