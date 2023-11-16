use async_trait::async_trait;
use serde::Deserialize;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;

use super::error::GCloudAuthError;
use crate::{Authorization, GToken, GoogleAuth};

#[derive(Clone, Debug, Deserialize)]
pub struct GoogleServiceAccount {
    #[serde(rename = "type")]
    pub service_account_type: String,
    pub project_id: String,
    pub private_key_id: String,
    pub private_key: String,
    pub client_email: String,
    pub client_id: String,
    pub auth_uri: String,
    pub token_uri: String,
    pub auth_provider_x509_cert_url: String,
    pub client_x509_cert_url: String,
}

impl GoogleServiceAccount {
    pub fn from_file<P: AsRef<Path>>(service_account_file: P) -> Result<Self, GCloudAuthError> {
        let service_account_file = std::fs::File::open(service_account_file).map_err(|e| {
            GCloudAuthError::CredentialsError(format!("cannot open service account file: {}", e))
        })?;
        let account = serde_json::from_reader(service_account_file)?;
        Ok(account)
    }

    pub fn from_json(json: serde_json::Value) -> Result<Self, GCloudAuthError> {
        Ok(serde_json::from_value(json)?)
    }

    pub fn from_json_str(json: &str) -> Result<Self, GCloudAuthError> {
        Ok(serde_json::from_str(json)?)
    }

    pub fn from_env_var<S: AsRef<str>>(name: S) -> Result<Self, GCloudAuthError> {
        let env_var_content = std::env::var(name.as_ref()).map_err(|e| {
            GCloudAuthError::CredentialsError(format!(
                "cannot read environment variable {}: {}",
                name.as_ref(),
                e
            ))
        })?;
        Self::from_json_str(&env_var_content)
    }

    pub fn from_default_env_var() -> Result<Self, GCloudAuthError> {
        Self::from_env_var("GOOGLE_SERVICE_ACCOUNT")
    }

    pub fn token(self, scopes: &[&'static str]) -> GToken {
        GToken::new(self, scopes)
    }
}

#[derive(Debug, Clone)]
pub struct ServiceAccountAuthorization {
    project_id: String,
    token: Arc<Mutex<GToken>>,
}

impl ServiceAccountAuthorization {
    pub fn with_account_and_scope(account: GoogleServiceAccount, scopes: &[&'static str]) -> Self {
        Self {
            project_id: account.project_id.clone(),
            token: Arc::new(Mutex::new(account.token(scopes))),
        }
    }
}

#[async_trait]
impl Authorization for ServiceAccountAuthorization {
    fn project_id(&self) -> &str {
        self.project_id.as_str()
    }

    async fn get_token(&self) -> Result<Option<String>, GCloudAuthError> {
        tracing::debug!("[ServiceAccountAuthorization] attempting to get a token");
        loop {
            match self.token.try_lock() {
                Err(_) => {
                    tracing::debug!(
                        "[ServiceAccountAuthorization] token lock is being held, retrying in a bit..."
                    );
                    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
                }
                Ok(mut token) => break token.refresh_if_necessary().await.map(Some),
            }
        }
    }

    fn box_clone(&self) -> GoogleAuth {
        Box::new(self.clone())
    }
}
