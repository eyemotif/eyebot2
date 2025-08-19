mod builtin;

pub type BuiltinCommands = Vec<Box<dyn Command + 'static>>;

#[async_trait::async_trait]
pub trait Command {
    fn description(&self, chat_message: &super::ChatMessage) -> Option<String>;
    fn is_match(&self, chat_message: &super::ChatMessage) -> bool;
    async fn execute(
        &self,
        chat_message: &super::ChatMessage,
        client: &mut crate::client::EventSubClient,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

pub fn builtin_commands() -> BuiltinCommands {
    let builtin_commands: BuiltinCommands = vec![
        Box::new(builtin::Ping),
        Box::new(builtin::Egg),
        Box::new(builtin::Crouton),
        Box::new(builtin::Corndog),
        Box::new(builtin::Comet),
    ];

    builtin_commands
}
