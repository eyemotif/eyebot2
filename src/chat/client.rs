#[derive(Debug)]
pub struct ChatClient {
    pub broadcaster_user_id: String,
    pub chatter_user_id: String,
    pub auth: crate::auth::Auth,
    pub websocket: tokio_tungstenite::WebSocketStream<
        tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
    >,
}

impl ChatClient {
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
                super::tls::create_websocket_tls_client(),
            )),
        )
        .await?;

        Ok(ChatClient {
            broadcaster_user_id,
            chatter_user_id,
            auth,
            websocket,
        })
    }
}
