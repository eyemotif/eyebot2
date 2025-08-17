use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct TwitchPostResponse<Inner> {
    pub data: Vec<Inner>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateEventSubSubscription<Condition> {
    #[serde(rename = "type")]
    pub subscription_type: String,
    pub version: String,
    pub condition: Condition,
    pub transport: Transport,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "method")]
#[serde(rename_all = "snake_case")]
pub enum Transport {
    Websocket { session_id: String },
}

#[derive(Debug, Clone, Serialize)]
pub struct BroadcasterAndUserCondition {
    pub broadcaster_user_id: String,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct SendChatMessage {
    pub broadcaster_id: String,
    pub sender_id: String,
    pub message: String,
    pub reply_parent_message_id: Option<String>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct SendChatMessageResponse {
    pub message_id: String,
    pub is_sent: bool,
}
