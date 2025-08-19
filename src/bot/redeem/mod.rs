mod builtin;

pub type BuiltinRedeems = Vec<Box<dyn Redeem + 'static>>;

#[async_trait::async_trait]
pub trait Redeem {
    fn is_match(&self, redeem: &super::PointRedeem) -> bool;
    async fn execute(
        &self,
        chat_message: &super::PointRedeem,
        client: &mut crate::client::EventSubClient,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

pub fn builtin_redeems() -> BuiltinRedeems {
    let builtin_commands: BuiltinRedeems = vec![
        Box::new(builtin::Pop),
        Box::new(builtin::First),
        ];

    builtin_commands
}
