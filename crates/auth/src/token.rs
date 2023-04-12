#![allow(dead_code)]

use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use super::error::GCloudAuthError;
use super::service_account::GoogleServiceAccount;

const GOOGLE_TOKEN_URL: &str = "https://www.googleapis.com/oauth2/v4/token";

#[derive(Debug, Serialize)]
pub struct Claims {
    iss: String,
    scope: String,
    aud: String,
    exp: usize,
    iat: usize,
}

#[derive(Debug, Serialize)]
struct TokenRequest {
    grant_type: String,
    assertion: String,
}

#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
    expires_in: u64,
    token_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenData {
    pub access_token: String,
    pub expires_in: Duration,
    pub token_type: String,
    pub requested_at: SystemTime,
    pub project_id: String,
}

impl TokenData {
    fn new(token_response: TokenResponse, project_id: String, requested_at: SystemTime) -> Self {
        let TokenResponse {
            access_token,
            expires_in,
            token_type,
        } = token_response;
        Self {
            access_token,
            token_type,
            expires_in: Duration::from_secs(expires_in),
            requested_at,
            project_id,
        }
    }

    fn expires_at(&self) -> SystemTime {
        self.requested_at + self.expires_in
    }

    /// Returns expiration time in unix epoch seconds
    fn expires_at_unix(&self) -> u64 {
        self.expires_at()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}

#[derive(Debug, Clone)]
pub struct GToken {
    service_account: GoogleServiceAccount,
    scopes: Vec<&'static str>,
    token: Option<TokenData>,
    cache_file: Option<PathBuf>,
}

/// `GToken` represents an access token used to authenticate against google
/// cloud services. It gets created from google cloud / firebase service
/// account.
///
/// Example:
///
/// ```ignore
/// let account = FirebaseServiceAccount::from_file("serviceAccount.json")?;
/// let mut gtoken = GToken::new(account);
/// gtoken.refresh_if_necessary().await?;
/// let access_token = dbg!(gtoken.access_token());
/// // ...
/// ```
impl GToken {
    pub(crate) fn new(service_account: GoogleServiceAccount, scopes: &[&'static str]) -> Self {
        GToken {
            service_account,
            scopes: Vec::from(scopes),
            token: None,
            cache_file: None,
        }
    }

    fn read_token_from_cache(&mut self) -> Result<(), GCloudAuthError> {
        if let Some(cache_file) = &self.cache_file {
            let content = match fs::read(cache_file) {
                Err(_) => {
                    tracing::debug!("Cannot read token cache file {}", cache_file.display());
                    return Ok(());
                }
                Ok(content) => content,
            };

            let token: TokenData = match serde_json::from_slice(&content) {
                Err(_) => {
                    tracing::debug!(
                        "Cannot parse token from cache file {}",
                        cache_file.display()
                    );
                    return Ok(());
                }
                Ok(token) => token,
            };

            let use_token = token.project_id == self.service_account.project_id
                && (self.token.is_none()
                    || self
                        .token
                        .as_ref()
                        .map(|my_token| token.expires_at() > my_token.expires_at())
                        .unwrap_or(false));

            if use_token {
                tracing::debug!("Token cache file {} loaded", cache_file.display());
                self.token = Some(token);
                return Ok(());
            };
        }

        Ok(())
    }

    fn write_token_to_cache(&mut self) -> Result<(), GCloudAuthError> {
        if let (Some(token), Some(cache_file)) = (&self.token, &self.cache_file) {
            tracing::debug!("Writing token to cache {}", cache_file.display());
            let contents = serde_json::to_string(token).map_err(|err| {
                GCloudAuthError::TokenRefreshError(format!(
                    "Error serializing token for caching: {err}"
                ))
            })?;
            fs::write(cache_file, contents).map_err(|err| {
                GCloudAuthError::TokenRefreshError(format!(
                    "Error writing token to cache file: {err}"
                ))
            })?;
        }

        Ok(())
    }

    pub fn cached<P: AsRef<Path>>(&mut self, file: P) -> Result<(), GCloudAuthError> {
        self.cache_file = Some(file.as_ref().to_path_buf());
        self.read_token_from_cache()?;
        Ok(())
    }

    fn access_token(&self) -> Result<String, GCloudAuthError> {
        self.token
            .as_ref()
            .map(|token| token.access_token.trim_end_matches('.').to_string())
            .ok_or_else(|| {
                GCloudAuthError::TokenRefreshError(
                    "Could not get a valid google access token".to_string(),
                )
            })
    }

    pub async fn refresh_if_necessary(&mut self) -> Result<String, GCloudAuthError> {
        match &self.token {
            Some(token) => {
                let now = SystemTime::now();
                if token.expires_at() <= now {
                    tracing::debug!("Google token is expired, refreshing");
                    return self.refresh().await;
                }
            }
            _ => return self.refresh().await,
        };
        self.access_token()
    }

    #[tracing::instrument(level = "debug", skip(self))]
    async fn refresh(&mut self) -> Result<String, GCloudAuthError> {
        let scope = self.scopes.join(" ");

        tracing::debug!("requesting refresh token for scopes: {scope}");

        let GoogleServiceAccount {
            private_key: key,
            client_email: email,
            ..
        } = &self.service_account;

        // -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-

        let now = std::time::SystemTime::now();
        let now_unix_epoch = now.duration_since(UNIX_EPOCH).map_err(|err| {
            GCloudAuthError::TokenRefreshError(format!("Cannot figure out UNIX epoch: {err}"))
        })?;
        let iat = now_unix_epoch.as_secs() as usize;

        let claims = Claims {
            iss: email.clone(),
            scope,
            aud: GOOGLE_TOKEN_URL.to_string(),
            exp: iat + 3600,
            iat,
        };
        let token_req = encode(
            &Header::new(Algorithm::RS256),
            &claims,
            &EncodingKey::from_rsa_pem(key.as_bytes()).map_err(|err| {
                GCloudAuthError::TokenRefreshError(format!("Error loading JWT signing key: {err}"))
            })?,
        )
        .map_err(|err| GCloudAuthError::TokenRefreshError(format!("Error encoding JWT: {err}")))?;
        let body = TokenRequest {
            grant_type: "urn:ietf:params:oauth:grant-type:jwt-bearer".to_owned(),
            assertion: token_req,
        };
        let body = serde_json::to_string(&body).map_err(|err| {
            GCloudAuthError::TokenRefreshError(format!("Error serializing refresh request: {err}"))
        })?;

        let res = reqwest::Client::new()
            .post(GOOGLE_TOKEN_URL)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .body(body)
            .send()
            .await
            .map_err(|err| {
                GCloudAuthError::TokenRefreshError(format!("Error requesting refresh token: {err}"))
            })?;

        let token: TokenResponse = res.json().await.map_err(|err| {
            GCloudAuthError::TokenRefreshError(format!(
                "Error parsing JSON of refresh token: {err}"
            ))
        })?;
        let project_id = self.service_account.project_id.clone();
        self.token = Some(TokenData::new(token, project_id, now));

        if self.cache_file.is_some() {
            self.write_token_to_cache()?;
        }

        self.access_token()
    }
}
