use futures_util::StreamExt;
use tokio::io::AsyncWriteExt;
use tokio_tungstenite::tungstenite;

mod auth;
mod chat;
mod message;

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

    let mut chat_client = match chat::client::ChatClient::new().await {
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
                tungstenite::Message::Text(text) => println!("{text}"),
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
