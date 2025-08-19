use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct CometManager(Arc<Mutex<CometManagerInternals>>);

#[derive(Debug)]
struct CometManagerInternals {
    websocket: Option<SplitSocket>,
    state: String,
}
#[derive(Debug)]
struct SplitSocket {
    sender: futures_util::stream::SplitSink<
        tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>,
        tokio_tungstenite::tungstenite::Message,
    >,
    receiver: futures_util::stream::SplitStream<
        tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>,
    >,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", content = "payload", rename_all = "snake_case")]
pub enum CometMessage {
    Register {
        state: String,
    },
    GetComponents {}, // TODO
    PlayAudio {},     // TODO
    AudioVolume {},   // TODO
    AudioClear {},
    ChatSetEmotes {
        username: String,
    },
    Chat {
        user_id: String,
        chat: Vec<CometChatFragment>,
        meta: CometChatMetadata,
    },
    ChatUser {
        user_id: String,
        chat_info: CometChatter,
    },
    ChatClear {
        user_id: Option<String>,
    },
    Features {},
}
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CometChatFragment {
    Text { content: String },
    Emote { emote: String },
}
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct CometChatter {
    pub display_name: String,
    pub name_color: String,
    pub badges: Vec<String>,
}
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CometChatMetadata {
    None,
    Action,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CometResponse {
    Ok {
        state: String,
    },
    Data {
        state: String,
        payload: String,
    },
    Error {
        state: String,
        is_internal: bool,
        message: String,
    },
}

impl CometManager {
    pub fn new() -> Self {
        let internals = Arc::new(Mutex::new(CometManagerInternals {
            websocket: None,
            state: "test".to_owned(), // TODO: random state
        }));
        let self_internals = internals.clone();

        tokio::spawn(async move {
            let listener = tokio::net::TcpListener::bind("0.0.0.0:8000")
                .await
                .expect("Could not bind to port 8000");

            let Ok((stream, _)) = listener.accept().await else {
                return;
            };
            let ws = tokio_tungstenite::accept_async(stream)
                .await
                .expect("Could not listen for comet connection");

            println!("Comet connected!");

            let (sender, receiver) = ws.split();
            internals.lock().await.websocket = Some(SplitSocket { sender, receiver });
        });

        Self(self_internals)
    }
    pub async fn is_connected(&self) -> bool {
        self.0.lock().await.websocket.is_some()
    }
    pub async fn send_message(
        &self,
        message: &CometMessage,
    ) -> Result<CometResponse, Box<dyn std::error::Error>> {
        let outbound = serde_json::to_string(&message)?;
        self.0
            .lock()
            .await
            .websocket
            .as_mut()
            .ok_or("No comet connection")?
            .sender
            .send(tokio_tungstenite::tungstenite::Message::Text(
                outbound.into(),
            ))
            .await?;

        loop {
            let inbound = loop {
                let Some(inbound) = self
                    .0
                    .lock()
                    .await
                    .websocket
                    .as_mut()
                    .ok_or("No comet connection")?
                    .receiver
                    .next()
                    .await
                    .transpose()?
                else {
                    continue;
                };
                break inbound;
            };

            match inbound {
                tokio_tungstenite::tungstenite::Message::Text(utf8_bytes) => {
                    let response = serde_json::from_slice::<CometResponse>(utf8_bytes.as_bytes())?;
                    return Ok(response);
                }
                tokio_tungstenite::tungstenite::Message::Ping(data) => {
                    self.0
                        .lock()
                        .await
                        .websocket
                        .as_mut()
                        .ok_or("No comet connection")?
                        .sender
                        .send(tokio_tungstenite::tungstenite::Message::Pong(data))
                        .await?;
                }
                tokio_tungstenite::tungstenite::Message::Close(_) => {
                    self.0.lock().await.websocket = None;
                    return Err("Comet socket closed while trying to receive reply".into());
                }
                _ => return Err("Invalid message from Comet socket".into()),
            }
        }
    }
}
