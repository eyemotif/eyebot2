use serde::Deserialize;

pub mod command;

pub type ChatMessageFragment = crate::eventsub::event::ChannelChatMessageMessageFragment;

#[derive(Debug, Clone, Deserialize)]
pub struct ChatMessage {
    pub chatter_user_id: String,
    pub chatter_user_login: String,
    pub chatter_user_name: String,
    pub badges: Vec<ChatMessageBadge>,
    pub fragments: Vec<ChatMessageFragment>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct ChatMessageBadge {
    pub set_id: String,
}

impl ChatMessage {
    pub fn message_text(&self) -> String {
        let mut message = String::new();
        for fragment in &self.fragments {
            match fragment {
                crate::eventsub::event::ChannelChatMessageMessageFragment::Emote {
                    text, ..
                }
                | crate::eventsub::event::ChannelChatMessageMessageFragment::Text { text }
                | crate::eventsub::event::ChannelChatMessageMessageFragment::Mention {
                    text, ..
                } => {
                    message += text;
                }
            }
        }
        message
    }
    pub fn chatter_is_moderator(&self) -> bool {
        self.badges
            .iter()
            .any(|badge| badge.set_id == "moderator" || badge.set_id == "broadcaster")
    }
}
