use crate::eventsub::UserInfo;

pub mod command;
pub mod redeem;

pub type ChatMessageFragment = crate::eventsub::event::ChannelChatMessageMessageFragment;
pub type ChatMessageBadge = crate::eventsub::event::ChannelChatMessageBadge;
pub type PointRedeemReward = crate::eventsub::event::ChannelPointsCustomRewardRedemptionAddReward;

pub struct Builtins {
    pub commands: command::BuiltinCommands,
    pub redeems: redeem::BuiltinRedeems,
}

#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub message_id: String,
    pub chatter_user: UserInfo,
    pub badges: Vec<ChatMessageBadge>,
    pub fragments: Vec<ChatMessageFragment>,
}

#[derive(Debug, Clone)]
pub struct PointRedeem {
    pub id: String,
    pub broadcaster_user: UserInfo,
    pub user: UserInfo,
    pub user_input: String,
    pub reward: PointRedeemReward,
}

impl Builtins {
    pub fn new() -> Self {
        Self {
            commands: command::builtin_commands(),
            redeems: redeem::builtin_redeems(),
        }
    }
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
