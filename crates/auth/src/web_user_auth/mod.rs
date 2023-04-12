mod config;
mod email;

pub use config::WebClientConfig;
pub use email::EmailSignin;

#[derive(Clone, Debug)]
pub struct WebUserAuth {
    pub config: WebClientConfig,
    pub login: WebLoginResult,
}

#[async_trait::async_trait]
impl crate::Authorization for WebUserAuth {
    fn project_id(&self) -> &str {
        &self.config.project_id
    }

    fn box_clone(&self) -> crate::GoogleAuth {
        Box::new(self.clone())
    }

    async fn get_token(&self) -> Result<String, crate::error::GCloudAuthError> {
        Ok(self.login.id_token.clone())
    }
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WebLoginResult {
    pub display_name: String,
    pub email: String,
    pub expires_in: String,
    pub id_token: String,
    pub kind: String,
    pub local_id: String,
    pub refresh_token: String,
    pub registered: bool,
}
