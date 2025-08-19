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

#[derive(Debug, Clone, Deserialize)]
pub struct BroadcasterUserInfo {
    #[serde(rename = "broadcaster_user_id")]
    pub id: String,
    #[serde(rename = "broadcaster_user_login")]
    pub login: String,
    #[serde(rename = "broadcaster_user_name")]
    pub name: String,
}
#[derive(Debug, Clone, Deserialize)]
pub struct UserInfo {
    #[serde(rename = "user_id")]
    pub id: String,
    #[serde(rename = "user_login")]
    pub login: String,
    #[serde(rename = "user_name")]
    pub name: String,
}

impl From<BroadcasterUserInfo> for UserInfo {
    fn from(val: BroadcasterUserInfo) -> Self {
        UserInfo {
            id: val.id,
            login: val.login,
            name: val.name,
        }
    }
}
