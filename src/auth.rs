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

    pub fn get_client_id(&self) -> &String {
        &self.client_id
    }
    pub async fn get_access_token(&mut self) -> reqwest::Result<&String> {
        self.validate().await?;
        Ok(&self.access_token)
    }

    /// Checks if the current tokens are valid, and refreshes them if not.
    async fn validate(&mut self) -> reqwest::Result<()> {
        match reqwest::Client::new()
            .get("https://id.twitch.tv/oauth2/validate")
            .header(
                reqwest::header::AUTHORIZATION,
                format!("OAuth {}", self.access_token),
            )
            .send()
            .await?
            .error_for_status()
        {
            Ok(_) => Ok(()),
            Err(err) => match err.status() {
                Some(reqwest::StatusCode::UNAUTHORIZED) => self.refresh_tokens().await,
                _ => Err(err),
            },
        }
    }

    async fn refresh_tokens(&mut self) -> reqwest::Result<()> {
        let response: RefreshResponse = reqwest::Client::new()
            .post("https://id.twitch.tv/oauth2/token")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .query(&[
                ("grant_type", "refresh_token".to_owned()),
                (
                    "refresh_token",
                    urlencoding::encode(&self.refresh_token).into_owned(),
                ),
                (
                    "client_id",
                    urlencoding::encode(&self.client_id).into_owned(),
                ),
                (
                    "client_secret",
                    urlencoding::encode(&self.client_secret).into_owned(),
                ),
            ])
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

impl std::fmt::Debug for Auth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Auth").finish()
    }
}
