use serde::Deserialize;

pub struct Auth {
    refresh_token: String,
    access_token: String,

    client_id: String,
    client_secret: String,
}

#[derive(Debug, Deserialize)]
struct RefreshResponse {
    access_token: String,
    refresh_token: String,
}

impl Auth {
    pub fn new(
        refresh_token: String,
        access_token: String,
        client_id: String,
        client_secret: String,
    ) -> Auth {
        Auth {
            refresh_token,
            access_token,
            client_id,
            client_secret,
        }
    }

    pub async fn get_access_token(&mut self) -> reqwest::Result<&String> {
        self.validate().await?;
        Ok(&self.access_token)
    }

    /// Checks if the current tokens are valid, and refreshes them if not.
    async fn validate(&mut self) -> reqwest::Result<()> {
        match reqwest::Client::new()
            .post("https://id.twitch.tv/oauth2/validate")
            .header(
                reqwest::header::AUTHORIZATION,
                format!("OAuth {}", self.access_token),
            )
            .send()
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => match err.status() {
                Some(reqwest::StatusCode::UNAUTHORIZED) => self.refresh_tokens().await,
                _ => Err(err),
            },
        }
    }

    async fn refresh_tokens(&mut self) -> reqwest::Result<()> {
        let refresh_body = format!(
            "grant_type=refresh_token&refresh_token={refresh_token}&client_id={client_id}&client_secret={client_secret}",
            refresh_token = self.refresh_token,
            client_id = self.client_id,
            client_secret = self.client_secret,
        );

        let response: RefreshResponse = reqwest::Client::new()
            .post("https://id.twitch.tv/oauth2/token")
            .header(
                reqwest::header::CONTENT_TYPE,
                reqwest::header::HeaderValue::from_static("application/x-www-form-urlencoded"),
            )
            .body(urlencoding::encode(&refresh_body).into_owned())
            .send()
            .await
            .and_then(reqwest::Response::error_for_status)?
            .json()
            .await?;

        self.access_token = response.access_token;
        self.refresh_token = response.refresh_token;

        Ok(())
    }
}
