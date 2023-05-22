use crate::{error::EndUserLoginError, WebLoginResult};

use super::WebClientConfig;

#[derive(serde::Serialize)]
pub struct EmailSignin {
    pub email: String,
    pub password: String,
    #[serde(rename = "returnSecureToken")]
    return_secure_token: bool,
}

impl EmailSignin {
    pub fn new(email: impl ToString, password: impl ToString) -> Self {
        Self {
            email: email.to_string(),
            password: password.to_string(),
            return_secure_token: true,
        }
    }

    pub async fn send(
        &self,
        config: &WebClientConfig,
    ) -> Result<WebLoginResult, EndUserLoginError> {
        let url = url::Url::parse_with_params(
            super::urls::EMAIL_SIGNIN_URL,
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
