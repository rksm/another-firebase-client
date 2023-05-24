use firebase_client_auth::error::GCloudAuthError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AccountBatchGetError {
    #[error("Auth token error {0}")]
    Token(#[from] GCloudAuthError),
    #[error("Authentication did not provide a token")]
    NoToken,
    #[error("Request error {0}")]
    Request(#[from] reqwest::Error),
    #[error("Serialization Error {0}")]
    Serde(#[from] serde_json::Error),
}
