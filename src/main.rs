use futures_util::StreamExt;
use tokio::io::AsyncWriteExt;
use tokio_tungstenite::tungstenite;

mod auth;
mod chat;
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

    let mut chat_client =
    // eye_motif = 214843364
    // eye___bot = 755534245
        match chat::client::ChatClient::new("214843364".to_owned(), "214843364".to_owned(), auth)
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

    loop {
        let Some(message) = chat_client.websocket.next().await else {
            break;
        };
        match message {
            Ok(message) => match message {
                tungstenite::Message::Text(text) => {
                    match serde_json::from_str::<eventsub::TwitchMessage>(text.as_str()) {
                        Ok(message) => match handle_message(message, &mut chat_client).await {
                            Ok(()) => (),
                            Err(err) => println!("Error handling message: {err}"),
                        },
                        Err(err) => println!("Error parsing message: {err}\n  In {text}"),
                    }
                }
                tungstenite::Message::Ping(data) => {
                    chat_client
                        .websocket
                        .get_mut()
                        .write_all(&tungstenite::Message::Pong(data).into_data())
                        .await
                        .expect("Couldn't send pong :(");
                    println!("Ping!");
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
    client: &mut chat::client::ChatClient,
) -> Result<(), Box<dyn std::error::Error>> {
    match message.metadata.message_type {
        eventsub::MessageType::SessionWelcome => {
            println!("Got welcome!");
            let payload =
                serde_json::from_value::<eventsub::payload::SessionWelcome>(message.payload)?;

            let subscriptions = [twitch::CreateEventSubSubscription {
                subscription_type: "channel.chat.message".to_owned(),
                version: "1".to_owned(),
                condition: twitch::BroadcasterAndUserCondition {
                    broadcaster_user_id: client.broadcaster_user_id.clone(),
                    user_id: client.chatter_user_id.clone(),
                },
                transport: twitch::Transport::Websocket {
                    session_id: payload.session.id,
                },
            }];
            for subscription in subscriptions.clone() {
                let response = reqwest::Client::new()
                    .post("https://api.twitch.tv/helix/eventsub/subscriptions")
                    .bearer_auth(client.auth.get_access_token().await?)
                    .header("Client-Id", client.auth.get_client_id())
                    .header("Content-Type", "application/json")
                    .json(&subscription)
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
        eventsub::MessageType::SessionKeepalive => println!("keepalive"),
        eventsub::MessageType::SessionReconnect => todo!("reconnect"),
        eventsub::MessageType::Notification => todo!("notification"),
    }

    Ok(())
}
