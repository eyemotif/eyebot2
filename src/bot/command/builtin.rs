use crate::bot::ChatMessage;
use crate::bot::command::Command;
use crate::client::EventSubClient;
use async_trait::async_trait;

pub(super) struct Ping;
#[async_trait]
impl Command for Ping {
    fn description(&self, chat_message: &ChatMessage) -> Option<String> {
        chat_message
            .chatter_is_moderator()
            .then_some("Replies \"Pong!\"".to_owned())
    }

    fn is_match(&self, chat_message: &ChatMessage) -> bool {
        chat_message.chatter_is_moderator() && chat_message.message_text().starts_with("!ping")
    }

    async fn execute(
        &self,
        chat_message: &ChatMessage,
        client: &mut EventSubClient,
    ) -> Result<(), Box<dyn std::error::Error>> {
        client
            .send_chat_message("Pong!", Some(chat_message.message_id.clone()))
            .await?;
        Ok(())
    }
}

pub(super) struct Egg;
#[async_trait]
impl Command for Egg {
    fn description(&self, _chat_message: &ChatMessage) -> Option<String> {
        None
    }

    fn is_match(&self, chat_message: &ChatMessage) -> bool {
        chat_message.message_text().contains("egg") || chat_message.message_text().contains("🥚")
    }

    async fn execute(
        &self,
        chat_message: &ChatMessage,
        client: &mut EventSubClient,
    ) -> Result<(), Box<dyn std::error::Error>> {
        client
            .send_chat_message(
                if chat_message.message_text().contains("egg") {
                    "🥚"
                } else {
                    "egg"
                },
                None,
            )
            .await?;
        Ok(())
    }
}
