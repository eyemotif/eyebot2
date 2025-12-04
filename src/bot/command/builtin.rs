use crate::bot::ChatMessage;
use crate::bot::command::Command;
use crate::client::EventSubClient;
use crate::comet;
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
pub(super) struct Crouton;
#[async_trait]
impl Command for Crouton {
    fn description(&self, _chat_message: &ChatMessage) -> Option<String> {
        Some("A link to the source of the Crouton".to_owned())
    }
    fn is_match(&self, chat_message: &ChatMessage) -> bool {
        chat_message.message_text().starts_with("!crouton")
    }
    async fn execute(
        &self,
        _chat_message: &ChatMessage,
        client: &mut EventSubClient,
    ) -> Result<(), Box<dyn std::error::Error>> {
        client
            .send_chat_message("https://crouton.net/", None)
            .await?;
        Ok(())
    }
}

pub(super) struct Corndog;
#[async_trait]
impl Command for Corndog {
    fn description(&self, _chat_message: &ChatMessage) -> Option<String> {
        Some("A link to the source of the Corndogs".to_owned())
    }
    fn is_match(&self, chat_message: &ChatMessage) -> bool {
        chat_message.message_text().starts_with("!corndog")
    }
    async fn execute(
        &self,
        _chat_message: &ChatMessage,
        client: &mut EventSubClient,
    ) -> Result<(), Box<dyn std::error::Error>> {
        client
            .send_chat_message("https://corndog.io/", None)
            .await?;
        Ok(())
    }
}

pub(super) struct Comet;
#[async_trait]
impl Command for Comet {
    fn description(&self, _chat_message: &ChatMessage) -> Option<String> {
        None
    }
    fn is_match(&self, _chat_message: &ChatMessage) -> bool {
        true
    }
    async fn execute(
        &self,
        chat_message: &ChatMessage,
        client: &mut EventSubClient,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if !client.comet_manager.is_connected().await {
            return Ok(());
        }

        loop {
            let response = client
                .comet_manager
                .send_message(&comet::Message::Chat {
                    user_id: chat_message.chatter_user.id.clone(),
                    chat: chat_message
                        .fragments
                        .iter()
                        .map(|fragment| {
                            match fragment {
                        crate::eventsub::event::ChannelChatMessageMessageFragment::Text {
                            text,
                        }
                        | crate::eventsub::event::ChannelChatMessageMessageFragment::Mention {
                            text,
                            ..
                        } => comet::ChatFragment::Text {
                            content: text.clone(),
                        },
                        crate::eventsub::event::ChannelChatMessageMessageFragment::Emote {
                            emote,
                            ..
                        } => comet::ChatFragment::Emote {
                            emote: emote.id.clone(),
                        },
                    }
                        })
                        .collect(),
                    meta: comet::ChatMetadata::None, // TODO: check for /me
                })
                .await?;

            match response {
                comet::Response::Ok { .. } => (),
                comet::Response::Data { .. } => {
                    // comet will request info for a twitch user id if it is new
                    let response = client
                        .comet_manager
                        .send_message(&comet::Message::ChatUser {
                            user_id: chat_message.chatter_user.id.clone(),
                            chat_info: comet::Chatter {
                                display_name: chat_message.chatter_user.name.clone(),
                                name_color: match &chat_message.color {
                                    Some(it) => it.clone(),
                                    None => "#000000".to_owned(),
                                }, // TODO
                                badges: Vec::new(), // TODO
                            },
                        })
                        .await?;
                    match response {
                        comet::Response::Ok { .. } => (),
                        comet::Response::Data { .. } => {
                            unreachable!("Got Data from ChatUser message")
                        }
                        comet::Response::Error {
                            message,
                            is_internal,
                            ..
                        } => {
                            println!(
                                "Comet error sending ChatUser message: {} {}",
                                message,
                                if is_internal { "(internal)" } else { "" }
                            );
                            break;
                        }
                    }
                    continue;
                }
                comet::Response::Error {
                    is_internal,
                    message,
                    ..
                } => {
                    println!(
                        "Comet error sending Chat message: {} {}",
                        message,
                        if is_internal { "(internal)" } else { "" }
                    );
                }
            }
            break;
        }

        Ok(())
    }
}

pub(super) struct CometGetComponents;
#[async_trait]
#[async_trait]
impl Command for CometGetComponents {
    fn description(&self, _chat_message: &ChatMessage) -> Option<String> {
        Some(
            "!comet:get <type> -- Get the list of a type of comet component. Component types: audio"
                .to_owned(),
        )
    }
    fn is_match(&self, chat_message: &ChatMessage) -> bool {
        chat_message.message_text().starts_with("!comet:get")
    }
    async fn execute(
        &self,
        chat_message: &ChatMessage,
        client: &mut EventSubClient,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let component_type = chat_message
            .message_text()
            .strip_prefix("!comet:get")
            .expect("CometGetComponents: Checked by is_match")
            .to_owned();
        #[allow(clippy::single_match_else)] // there will be other component types in the future
        let component_type = match component_type.to_ascii_lowercase().trim() {
            "audio" => comet::ComponentType::Audio,
            _ => {
                if chat_message.chatter_is_moderator() {
                    client
                        .send_chat_message(
                            format!("Unexpected component type \"{component_type}\""),
                            Some(chat_message.message_id.clone()),
                        )
                        .await?;
                }
                return Ok(());
            }
        };

        let response = client
            .comet_manager
            .send_message(&comet::Message::GetComponents { component_type })
            .await?;
        match response {
            comet::Response::Ok { .. } => unreachable!("GetComponents will never respond Ok"),
            comet::Response::Data { payload, .. } => {
                client
                    .send_chat_message(payload, Some(chat_message.message_id.clone()))
                    .await?;
            }
            comet::Response::Error {
                is_internal,
                message,
                ..
            } => {
                println!(
                    "Comet error sending GetComponents message: {} {}",
                    message,
                    if is_internal { "(internal)" } else { "" }
                );
            }
        }

        Ok(())
    }
}

pub(super) struct CometSetAudioComponentVolume;
#[async_trait]
#[async_trait]
impl Command for CometSetAudioComponentVolume {
    fn description(&self, chat_message: &ChatMessage) -> Option<String> {
        chat_message.chatter_is_moderator().then_some("!comet:setvolume <audio-name> <volume: 0.0-1.0> -- Set the volume of a comet audio component".to_owned())
    }
    fn is_match(&self, chat_message: &ChatMessage) -> bool {
        chat_message.chatter_is_moderator()
            && chat_message.message_text().starts_with("!comet:setvolume")
    }
    async fn execute(
        &self,
        chat_message: &ChatMessage,
        client: &mut EventSubClient,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let chat_message_text = chat_message.message_text();
        let Some((audio_component_name, volume_value)) = chat_message_text
            .strip_prefix("!comet:setvolume")
            .expect("CometGetComponents: Checked by is_match")
            .trim()
            .split_once(' ')
        else {
            client
                .send_chat_message(
                    "!comet:setvolume expects 2 arguments",
                    Some(chat_message.message_id.clone()),
                )
                .await?;
            return Ok(());
        };
        let Ok(volume_value) = volume_value.parse::<f64>() else {
            client
                .send_chat_message(
                    format!("!comet:setvolume expects a number as its second argument, got {volume_value:?}"),
                    Some(chat_message.message_id.clone()),
                )
                .await?;
            return Ok(());
        };

        let response = client
            .comet_manager
            .send_message(&comet::Message::AudioVolume {
                audio_component_name: audio_component_name.to_owned(),
                volume_value,
            })
            .await?;
        match response {
            comet::Response::Ok { .. } => (),
            comet::Response::Data { .. } => unreachable!("AudioVolume will never respond Data"),
            comet::Response::Error {
                is_internal,
                message,
                ..
            } => {
                println!(
                    "Comet error sending AudioVolume message: {} {}",
                    message,
                    if is_internal { "(internal)" } else { "" }
                );
            }
        }

        Ok(())
    }
}
pub(super) struct CometClearAudioQueue;
#[async_trait]
#[async_trait]
impl Command for CometClearAudioQueue {
    fn description(&self, chat_message: &ChatMessage) -> Option<String> {
        chat_message
            .chatter_is_moderator()
            .then_some("Clear the comet audio queue".to_owned())
    }
    fn is_match(&self, chat_message: &ChatMessage) -> bool {
        chat_message.chatter_is_moderator()
            && chat_message.message_text().starts_with("!comet:clearaudio")
    }
    async fn execute(
        &self,
        _chat_message: &ChatMessage,
        client: &mut EventSubClient,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let response = client
            .comet_manager
            .send_message(&comet::Message::AudioClear {})
            .await?;
        match response {
            comet::Response::Ok { .. } => (),
            comet::Response::Data { .. } => {
                unreachable!("AudioClear will never respond Data")
            }
            comet::Response::Error {
                is_internal,
                message,
                ..
            } => {
                println!(
                    "Comet error sending AudioClear message: {} {}",
                    message,
                    if is_internal { "(internal)" } else { "" }
                );
            }
        }

        Ok(())
    }
}
