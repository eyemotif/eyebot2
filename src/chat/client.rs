use futures_util::StreamExt;

#[derive(Debug)]
pub struct ChatClient {
    pub websocket: tokio_tungstenite::WebSocketStream<
        tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
    >,
}

impl ChatClient {
    pub async fn new() -> tokio_tungstenite::tungstenite::Result<Self> {
        let (websocket, _) = tokio_tungstenite::connect_async_tls_with_config(
            "wss://eventsub.wss.twitch.tv/ws",
            None,
            true,
            Some(tokio_tungstenite::Connector::Rustls(
                super::tls::create_websocket_tls_client(),
            )),
        )
        .await?;

        Ok(ChatClient { websocket })
    }
}
