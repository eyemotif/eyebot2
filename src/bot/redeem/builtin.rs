use super::Redeem;
use crate::bot::{redeem, PointRedeem};
use crate::client::EventSubClient;
use async_trait::async_trait;
use rustls::pki_types::{SubjectPublicKeyInfo, SubjectPublicKeyInfoDer};

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
pub(super) struct First;
#[async_trait]
impl Redeem for First{
    fn is_match(&self, redeem: &PointRedeem) -> bool {
        redeem.reward.title == "First"
    }
    async fn execute(
        &self,
        redeem: &PointRedeem,
        client: &mut EventSubClient,
    ) -> Result<(), Box<dyn std::error::Error>> {
        client.send_chat_message(format!("@{} Congrats on being first to the stream!",redeem.user.name), None).await?;
        Ok(())
    }
}
pub(super) struct Lurky;
#[async_trait]
impl Redeem for Lurky{
    fn is_match(&self, redeem: &PointRedeem) -> bool {
        redeem.reward.title=="I'm Lurky"
    }
    async fn execute(
        &self,
        redeem: &PointRedeem,
        client: &mut EventSubClient,
    ) -> Result<(),Box<dyn std::error::Error>> {
        client.send_chat_message(format!("Have a good lurky @{}!",redeem.user.name), None).await?;
        Ok(())
    }
}
pub(super) struct Posture;
#[async_trait]
impl Redeem for Posture{
    fn is_match(&self, redeem: &PointRedeem) -> bool {
        redeem.reward.title == "Posture Check"
    }
    async fn execute(
        &self,
        redeem: &PointRedeem,
        client: &mut EventSubClient,
    ) -> Result<(), Box<dyn std::error::Error>> {
        client.send_chat_message(format!("@{} fix your posture NOW.",redeem.broadcaster_user.name), None).await?;
        Ok(())
    }
}