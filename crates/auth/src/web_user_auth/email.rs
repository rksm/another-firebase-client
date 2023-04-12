use crate::{error::EmailLoginError, WebLoginResult};

use super::WebClientConfig;

const EMAIL_SIGNIN_URL: &str =
    "https://identitytoolkit.googleapis.com/v1/accounts:signInWithPassword";

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

    pub async fn login(&self, config: &WebClientConfig) -> Result<WebLoginResult, EmailLoginError> {
        let url =
            url::Url::parse_with_params(EMAIL_SIGNIN_URL, [("key", config.api_key.as_str())])?;
        let response = reqwest::ClientBuilder::new()
            .build()?
            .post(url)
            .json(&self)
            .send()
            .await?;

        let response = response.json::<serde_json::Value>().await?;

        Ok(serde_json::from_value(response)?)
    }
}
