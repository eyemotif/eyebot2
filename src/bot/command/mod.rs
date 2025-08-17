pub mod builtin;

pub trait Command {
    fn description(&self) -> String;
    fn is_match(&self, chat_message: &super::ChatMessage) -> bool;
    fn execute(
        &self,
        chat_message: &super::ChatMessage,
        client: &mut crate::chat::client::ChatClient,
    );
}
