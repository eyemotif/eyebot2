use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct TwitchMessage<T = HashMap<String, String>> {
    pub metadata: Metadata,
    pub payload: T,
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
