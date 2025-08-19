pub mod comet;
mod tls;

#[derive(Debug)]
pub struct EventSubClient {
    pub broadcaster_user_id: String,
    pub chatter_user_id: String,
    pub auth: crate::auth::Auth,
    pub websocket: tokio_tungstenite::WebSocketStream<
        tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
    >,
    pub comet_manager: comet::CometManager,
}

impl EventSubClient {
    pub async fn new(
        broadcaster_user_id: String,
        chatter_user_id: String,
        auth: crate::auth::Auth,
    ) -> tokio_tungstenite::tungstenite::Result<Self> {
        let (websocket, _) = tokio_tungstenite::connect_async_tls_with_config(
            "wss://eventsub.wss.twitch.tv/ws",
            None,
            true,
            Some(tokio_tungstenite::Connector::Rustls(
                tls::create_websocket_tls_client(),
            )),
        )
        .await?;

        Ok(EventSubClient {
            broadcaster_user_id,
            chatter_user_id,
            auth,
            websocket,
            comet_manager: comet::CometManager::new(),
        })
    }

    pub async fn send_twitch_api_call<
        Payload: serde::Serialize,
        Inner: serde::de::DeserializeOwned,
    >(
        &mut self,
        url: &str,
        payload: &Payload,
    ) -> Result<crate::twitch::TwitchPostResponse<Inner>, Box<dyn std::error::Error>> {
        let response = reqwest::Client::new()
            .post(url)
            .bearer_auth(self.auth.get_access_token().await?)
            .header("Client-Id", self.auth.get_client_id())
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

        match status {
            reqwest::StatusCode::OK => Ok(serde_json::from_str(&text)?),
            status => Err(format!("Error {status} (from {url}): {text}").into()),
        }
    }

    pub async fn send_chat_message(
        &mut self,
        message: impl Into<String>,
        reply_parent_message_id: Option<String>,
    ) -> Result<
        crate::twitch::TwitchPostResponse<crate::twitch::SendChatMessageResponse>,
        Box<dyn std::error::Error>,
    > {
        self.send_twitch_api_call(
            "https://api.twitch.tv/helix/chat/messages",
            &crate::twitch::SendChatMessage {
                broadcaster_id: self.broadcaster_user_id.clone(),
                sender_id: self.chatter_user_id.clone(),
                message: message.into(),
                reply_parent_message_id,
            },
        )
        .await
    }
}
