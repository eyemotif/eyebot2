use super::Redeem;
use crate::bot::PointRedeem;
use crate::client::EventSubClient;
use async_trait::async_trait;

pub(super) struct Pop;
#[async_trait]
impl Redeem for Pop {
    fn is_match(&self, redeem: &PointRedeem) -> bool {
        redeem.reward.title == "Pop"
    }

    async fn execute(
        &self,
        redeem: &PointRedeem,
        client: &mut EventSubClient,
    ) -> Result<(), Box<dyn std::error::Error>> {
        client.send_chat_message("Pop pop pop", None).await?;
        Ok(())
    }
}
