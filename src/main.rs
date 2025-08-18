use futures_util::StreamExt;
use tokio::io::AsyncWriteExt;
use tokio_tungstenite::tungstenite;

mod auth;
mod bot;
mod client;
mod eventsub;
mod twitch;

#[tokio::main]
async fn main() {
    let access_token = tokio::fs::read_to_string("./tokens/access")
        .await
        .expect("Could not read access token")
        .trim()
        .to_owned();
    let refresh_token = tokio::fs::read_to_string("./tokens/refresh")
        .await
        .expect("Could not read refresh token")
        .trim()
        .to_owned();
    let client_id = tokio::fs::read_to_string("./tokens/clientid")
        .await
        .expect("Could not read client id")
        .trim()
        .to_owned();
    let client_secret = tokio::fs::read_to_string("./tokens/clientsecret")
        .await
        .expect("Could not read client secret")
        .trim()
        .to_owned();

    let mut auth = auth::Auth::new(refresh_token, access_token, client_id, client_secret);

    match auth.get_access_token().await {
        Ok(_) => println!("Got token!"),
        Err(err) => println!("{err:?}"),
    }

    let mut client =
    // eye_motif = 214843364
    // eye___bot = 755534245
        match client::EventSubClient::new("214843364".to_owned(), "214843364".to_owned(), auth)
            .await
        {
            Ok(it) => it,
            Err(err) => {
                if let tungstenite::Error::Http(response) = err {
                    println!(
                        "{}",
                        String::from_utf8_lossy(response.body().as_ref().unwrap())
                    );
                } else {
                    println!("{err}");
                }
                return;
            }
        };

    let builtins = bot::Builtins::new();

    loop {
        let Some(message) = client.websocket.next().await else {
            break;
        };
        match message {
            Ok(message) => match message {
                tungstenite::Message::Text(text) => {
                    match serde_json::from_str::<eventsub::TwitchMessage>(text.as_str()) {
                        Ok(message) => {
                            match handle_message(message, &mut client, &builtins).await {
                                Ok(()) => (),
                                Err(err) => println!("Error handling message: {err}"),
                            }
                        }
                        Err(err) => println!("Error parsing message: {err}\n  In {text}"),
                    }
                }
                tungstenite::Message::Ping(data) => {
                    client
                        .websocket
                        .get_mut()
                        .write_all(&tungstenite::Message::Pong(data).into_data())
                        .await
                        .expect("Couldn't send pong :(");
                }
                tungstenite::Message::Close(close_frame) => {
                    if let Some(close_frame) = close_frame {
                        println!("{close_frame:?}");
                    }
                    break;
                }
                _ => println!("message {message:?}"),
            },
            Err(err) => match err {
                tungstenite::Error::Http(response) => {
                    println!(
                        "{}",
                        String::from_utf8_lossy(response.body().as_ref().unwrap())
                    );
                }
                tungstenite::Error::AlreadyClosed | tungstenite::Error::ConnectionClosed => break,
                _ => println!("Receive error: {err}"),
            },
        }
    }
}

async fn handle_message(
    message: eventsub::TwitchMessage,
    client: &mut client::EventSubClient,
    builtins: &bot::Builtins,
) -> Result<(), Box<dyn std::error::Error>> {
    match message.metadata.message_type {
        eventsub::MessageType::SessionWelcome => {
            let payload =
                serde_json::from_value::<eventsub::payload::SessionWelcome>(message.payload)?;

            let subscriptions = [
                twitch::CreateEventSubSubscription::new(
                    "channel.chat.message",
                    "1",
                    twitch::BroadcasterAndUserCondition {
                        broadcaster_user_id: client.broadcaster_user_id.clone(),
                        user_id: client.chatter_user_id.clone(),
                    },
                    &payload.session.id,
                )
                .expect("Constant value should serialize"),
                twitch::CreateEventSubSubscription::new(
                    "channel.channel_points_custom_reward_redemption.add",
                    "1",
                    twitch::BroadcasterCondition {
                        broadcaster_user_id: client.broadcaster_user_id.clone(),
                    },
                    &payload.session.id,
                )
                .expect("Constant value should serialize"),
            ];

            for subscription in &subscriptions {
                let response = reqwest::Client::new()
                    .post("https://api.twitch.tv/helix/eventsub/subscriptions")
                    .bearer_auth(client.auth.get_access_token().await?)
                    .header("Client-Id", client.auth.get_client_id())
                    .header("Content-Type", "application/json")
                    .json(subscription)
                    .send()
                    .await?;

                match response.status() {
                    reqwest::StatusCode::ACCEPTED => (),
                    error_status => {
                        return Err(format!("{error_status}: {}", response.text().await?).into());
                    }
                }
            }

            println!(
                "subscribed to: {}",
                subscriptions
                    .map(|subscription| subscription.subscription_type)
                    .join(", ")
            );
        }
        eventsub::MessageType::SessionKeepalive => (),
        eventsub::MessageType::SessionReconnect => todo!("reconnect"),
        eventsub::MessageType::Notification => {
            let payload =
                serde_json::from_value::<eventsub::payload::Notification>(message.payload)?;
            // println!("* {payload:?}");
            match payload.subscription.subscription_type.as_str() {
                "channel.chat.message" => {
                    let event = serde_json::from_value::<eventsub::event::ChannelChatMessage>(
                        payload.event,
                    )?;

                    println!("{}>{}", event.chatter_user_name, event.message.text);

                    if event.chatter_user_id != client.chatter_user_id {
                        handle_chat_message(event, client, builtins).await?;
                    }
                }
                "channel.channel_points_custom_reward_redemption.add" => {
                    let event = serde_json::from_value::<
                        eventsub::event::ChannelPointsCustomRewardRedemptionAdd,
                    >(payload.event)?;

                    println!(
                        "{} redeemed {}{}",
                        event.user.name,
                        event.reward.title,
                        if event.user_input.is_empty() {
                            ""
                        } else {
                            &format!(": {:?}", event.user_input)
                        }
                    );

                    handle_point_redeem(event, client, builtins).await?;
                }
                unknown_subscription_type => {
                    println!("Unhandled subscription type: {unknown_subscription_type}");
                }
            }
        }
    }

    Ok(())
}

async fn handle_chat_message(
    message: eventsub::event::ChannelChatMessage,
    client: &mut client::EventSubClient,
    builtins: &bot::Builtins,
) -> Result<(), Box<dyn std::error::Error>> {
    let message = bot::ChatMessage {
        message_id: message.message_id,
        chatter_user: crate::eventsub::UserInfo {
            id: message.chatter_user_id,
            login: message.chatter_user_login,
            name: message.chatter_user_name,
        },
        badges: message.badges,
        fragments: message.message.fragments,
    };

    for builtin_command in &builtins.commands {
        if builtin_command.is_match(&message) {
            builtin_command.execute(&message, client).await?;
        }
    }

    Ok(())
}

async fn handle_point_redeem(
    redeem: eventsub::event::ChannelPointsCustomRewardRedemptionAdd,
    client: &mut client::EventSubClient,
    builtins: &bot::Builtins,
) -> Result<(), Box<dyn std::error::Error>> {
    let redeem = bot::PointRedeem {
        id: redeem.id,
        broadcaster_user: redeem.broadcaster_user.into(),
        user: redeem.user,
        user_input: redeem.user_input,
        reward: redeem.reward,
    };

    for builtin_redeem in &builtins.redeems {
        if builtin_redeem.is_match(&redeem) {
            builtin_redeem.execute(&redeem, client).await?;
        }
    }

    Ok(())
}
