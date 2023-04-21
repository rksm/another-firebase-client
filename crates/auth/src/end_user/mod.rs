mod config;
mod email;
mod refresh;
mod urls;

pub use config::WebClientConfig;
pub use email::EmailSignin;

use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};
use tokio::sync::RwLock;

use self::refresh::{RefreshTokenRequest, RefreshTokenResponse};

#[derive(Clone, Debug)]
pub struct WebUserAuth {
    pub config: WebClientConfig,
    login: Arc<RwLock<WebLoginResult>>,
}

impl WebUserAuth {
    pub fn new(config: WebClientConfig, login: WebLoginResult) -> Self {
        Self {
            config,
            login: Arc::new(RwLock::new(login)),
        }
    }

    async fn refresh(&self) -> Result<(), crate::error::GCloudAuthError> {
        let mut login = self.login.write().await;
        tracing::debug!("Refreshing token that expires at {:?}", login.expires_at());
        let response = RefreshTokenRequest::new(&login.refresh_token)
            .send(&self.config)
            .await
            .map_err(|e| crate::error::GCloudAuthError::TokenRefreshError(e.to_string()))?;
        login.update_with_refresh(response);
        Ok(())
    }

    async fn refresh_if_necessary(&self) -> Result<(), crate::error::GCloudAuthError> {
        if self.login.read().await.expires_at() < SystemTime::now() {
            self.refresh().await?;
        }
        Ok(())
    }
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
        self.refresh_if_necessary().await?;
        Ok(self.login.read().await.id_token.clone())
    }
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WebLoginResult {
    /// The uid of the authenticated user.
    #[serde(rename = "localId")]
    pub uid: String,
    /// The email for the authenticated user.
    pub email: String,
    pub display_name: String,
    /// The number of seconds in which the ID token expires.
    pub expires_in: String,
    #[serde(skip, default = "SystemTime::now")]
    pub requested_at: SystemTime,
    /// An Identity Platform ID token for the authenticated user.
    pub id_token: String,
    pub kind: String,
    /// An Identity Platform refresh token for the authenticated user.
    pub refresh_token: String,
    /// Whether the email is for an existing account.
    pub registered: bool,
}

impl WebLoginResult {
    fn expires_in(&self) -> Duration {
        Duration::from_secs(self.expires_in.parse().unwrap())
    }

    fn expires_at(&self) -> SystemTime {
        self.requested_at + self.expires_in()
    }

    fn update_with_refresh(&mut self, refresh: RefreshTokenResponse) {
        self.expires_in = refresh.expires_in;
        self.requested_at = SystemTime::now();
        self.id_token = refresh.id_token;
        self.refresh_token = refresh.refresh_token;
    }
}
