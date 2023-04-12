use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;

use super::error::GCloudAuthError;
use crate::{Authorization, GToken, GoogleAuth, GoogleServiceAccount};

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

    async fn get_token(&self) -> Result<String, GCloudAuthError> {
        tracing::debug!("[ServiceAccountAuthorization] attempting to get a token");
        loop {
            match self.token.try_lock() {
                Err(_) => {
                    tracing::debug!(
                        "[ServiceAccountAuthorization] token lock is being held, retrying in a bit..."
                    );
                    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
                }
                Ok(mut token) => break token.refresh_if_necessary().await,
            }
        }
    }

    fn box_clone(&self) -> GoogleAuth {
        Box::new(self.clone())
    }
}
