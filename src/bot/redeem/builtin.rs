use super::Redeem;
use crate::bot::PointRedeem;
use crate::client::EventSubClient;
use crate::comet;
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

pub(super) struct PlayAudio;
#[async_trait]
impl Redeem for PlayAudio {
    fn is_match(&self, redeem: &PointRedeem) -> bool {
        redeem.reward.title == "Play Audio"
    }

    async fn execute(
        &self,
        redeem: &PointRedeem,
        client: &mut EventSubClient,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let data = redeem
            .user_input
            .split(' ')
            .take(10)
            .map(|outer| {
                outer
                    .split('+')
                    .map(|audio_name| comet::AudioComponent {
                        name: audio_name.to_owned(),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let response = client
            .comet_manager
            .send_message(&comet::Message::PlayAudio { data })
            .await?;
        match response {
            comet::Response::Ok { .. } => (),
            comet::Response::Data { .. } => unreachable!("PlayAudio will never respond Data"),
            comet::Response::Error {
                is_internal,
                message,
                ..
            } => {
                println!(
                    "Comet error sending PlayAudio message: {} {}",
                    message,
                    if is_internal { "(internal)" } else { "" }
                );
            }
        }

        Ok(())
    }
}
