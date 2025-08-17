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
    // TODO: badges, message_type, cheer, reply
    // https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchatmessage
}
#[derive(Debug, Clone, Deserialize)]
pub struct ChannelChatMessageMessage {
    pub text: String,
    pub fragments: Vec<ChannelChatMessageMessageFragment>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct ChannelChatMessageMessageFragment {
    #[serde(rename = "type")]
    pub fragment_type: String,
    pub text: String,
    // TODO: cheermote, emote, mention https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchatmessage
}
