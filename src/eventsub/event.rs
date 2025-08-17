use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ChannelChatMessage {
    pub broadcaster_user_id: String,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,

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
        mention: ChannelChatMessageMessageFragmentMention,
    },
    Emote {
        text: String,
        emote: ChannelChatMessageMessageFragmentEmote,
    },
}
#[derive(Debug, Clone, Deserialize)]
#[allow(clippy::struct_field_names)] // External API names
pub struct ChannelChatMessageMessageFragmentMention {
    pub user_id: String,
    pub user_login: String,
    pub user_name: String,
}
#[derive(Debug, Clone, Deserialize)]
pub struct ChannelChatMessageMessageFragmentEmote {
    pub id: String,
}
