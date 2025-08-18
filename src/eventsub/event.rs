use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ChannelChatMessage {
    #[serde(flatten)]
    pub broadcaster_user: super::BroadcasterUserInfo,

    pub chatter_user_id: String,
    pub chatter_user_login: String,
    pub chatter_user_name: String,
    pub color: String,

    pub message_id: String,
    pub message: ChannelChatMessageMessage,
    pub badges: Vec<ChannelChatMessageBadge>,
    // TODO: message_type, cheer, reply
    // https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchatmessage
}
#[derive(Debug, Clone, Deserialize)]
pub struct ChannelChatMessageBadge {
    pub set_id: String,
}
#[derive(Debug, Clone, Deserialize)]
pub struct ChannelChatMessageMessage {
    pub text: String,
    pub fragments: Vec<ChannelChatMessageMessageFragment>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ChannelChatMessageMessageFragment {
    Text {
        text: String,
    },
    Mention {
        text: String,
        mention: super::UserInfo,
    },
    Emote {
        text: String,
        emote: ChannelChatMessageMessageFragmentEmote,
    },
}
#[derive(Debug, Clone, Deserialize)]
pub struct ChannelChatMessageMessageFragmentEmote {
    pub id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChannelPointsCustomRewardRedemptionAdd {
    pub id: String,
    #[serde(flatten)]
    pub broadcaster_user: super::BroadcasterUserInfo,
    #[serde(flatten)]
    pub user: super::UserInfo,
    pub user_input: String,
    pub reward: ChannelPointsCustomRewardRedemptionAddReward,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChannelPointsCustomRewardRedemptionAddReward {
    pub id: String,
    pub title: String,
    pub cost: u64,
    pub prompt: String,
}
