use crate::bot::command::Command;

pub(super) struct Ping;
#[async_trait::async_trait]
impl Command for Ping {
    fn description(&self, chat_message: &crate::bot::ChatMessage) -> Option<String> {
        chat_message
            .chatter_is_moderator()
            .then_some("Replies \"Pong!\"".to_owned())
    }

    fn is_match(&self, chat_message: &crate::bot::ChatMessage) -> bool {
        chat_message.chatter_is_moderator() && chat_message.message_text().starts_with("!ping")
    }

    async fn execute(
        &self,
        chat_message: &crate::bot::ChatMessage,
        client: &mut crate::chat::client::ChatClient,
    ) -> Result<(), Box<dyn std::error::Error>> {
        client
            .send_chat_message("Pong!", Some(chat_message.message_id.clone()))
            .await?;
        Ok(())
    }
}
