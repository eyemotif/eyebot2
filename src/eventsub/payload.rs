use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct SessionWelcome {
    pub session: SessionWelcomeSession,
}
#[derive(Debug, Clone, Deserialize)]
pub struct SessionWelcomeSession {
    pub id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Notification {
    pub subscription: NotificationSubscription,
    pub event: serde_json::Value,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NotificationSubscription {
    #[serde(rename = "type")]
    pub subscription_type: String,
}
