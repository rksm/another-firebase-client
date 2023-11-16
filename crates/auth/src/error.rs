use thiserror::Error;

#[derive(Error, Debug)]
pub enum GCloudAuthError {
    #[error("The gcloud command line is not installed. See https://cloud.google.com/sdk/docs/install for install instructions.")]
    GCloudCliNotInstalled,

    #[error("There is no gcloud cli project set. Run gcloud config set project. https://cloud.google.com/sdk/gcloud/reference/config/set")]
    GCloudCliNoProject,

    #[error("Not logged in to gcloud command line. Run gcloud login.")]
    GCloudCliNotLoggedIn,

    #[error("Unable to parse gcloud cli output: {0}")]
    GCloudCliParseError(String),

    #[error("Error running gcloud cli: {0}")]
    GCloudCliCommandError(String),

    #[error("Credentials not provided: {0}")]
    CredentialsError(String),

    #[error("Error refreshing token: {0}")]
    TokenRefreshError(String),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
}

#[derive(Error, Debug)]
pub enum EndUserLoginError {
    #[error("Url parse error: {0}")]
    Url(#[from] url::ParseError),

    #[error("Email login network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("Email login deserialization error: {0}")]
    Deserialization(#[from] serde_json::Error),
}
