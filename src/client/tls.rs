use std::sync::Arc;

pub fn create_websocket_tls_client() -> Arc<rustls::ClientConfig> {
    let root_store = webpki_roots::TLS_SERVER_ROOTS
        .iter()
        .cloned()
        .collect::<rustls::RootCertStore>();
    let client = rustls::ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    Arc::new(client)
}
