use crate::{error::EndUserLoginError, WebClientConfig};

#[derive(serde::Serialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
    grant_type: &'static str,
}

impl RefreshTokenRequest {
    pub fn new(refresh_token: impl ToString) -> Self {
        Self {
            refresh_token: refresh_token.to_string(),
            grant_type: "refresh_token",
        }
    }

    pub async fn send(
        &self,
        config: &WebClientConfig,
    ) -> Result<RefreshTokenResponse, EndUserLoginError> {
        let url = url::Url::parse_with_params(
            super::urls::REFRESH_URL,
            [("key", config.api_key.as_str())],
        )?;

        let response = reqwest::ClientBuilder::new()
            .build()?
            .post(url)
            .json(&self)
            .send()
            .await?;

        Ok(response.json().await?)
    }
}

#[derive(serde::Deserialize)]
pub struct RefreshTokenResponse {
    /// The number of seconds in which the ID token expires.
    pub expires_in: String,
    /// The type of the refresh token, always "Bearer".
    pub token_type: String,
    /// The Identity Platform refresh token provided in the request or a new refresh token.
    pub refresh_token: String,
    /// An Identity Platform ID token.
    pub id_token: String,
    /// The uid corresponding to the provided ID token.
    pub user_id: String,
    /// Your GCP project ID.
    pub project_id: String,
}
