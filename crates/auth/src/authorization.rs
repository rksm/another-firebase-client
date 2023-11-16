use async_trait::async_trait;

use crate::GoogleAuth;

use super::error::GCloudAuthError;

#[async_trait]
pub trait Authorization: Send + Sync + std::fmt::Debug {
    fn project_id(&self) -> &str;
    async fn get_token(&self) -> Result<Option<String>, GCloudAuthError>;
    fn box_clone(&self) -> GoogleAuth;
}
