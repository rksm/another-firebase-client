use anyhow::Result;
use serde::Deserialize;
use std::path::Path;

use crate::GToken;

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
    pub fn from_file<P: AsRef<Path>>(service_account_file: P) -> Result<Self> {
        let service_account_file = std::fs::File::open(service_account_file)?;
        let account = serde_json::from_reader(service_account_file)?;
        Ok(account)
    }

    pub fn from_json(json: serde_json::Value) -> Result<Self> {
        Ok(serde_json::from_value(json)?)
    }

    pub fn from_env_var<S: AsRef<str>>(name: S) -> Result<Self> {
        let env_var_content = std::env::var(name.as_ref())
            .map_err(|e| anyhow::anyhow!("environment varibale {}: {}", name.as_ref(), e))?;
        let account = serde_json::from_str(&env_var_content)?;
        Ok(account)
    }

    pub fn from_default_env_var() -> Result<Self> {
        Self::from_env_var("GOOGLE_SERVICE_ACCOUNT")
    }

    pub fn token(self, scopes: &[&'static str]) -> GToken {
        GToken::new(self, scopes)
    }
}
