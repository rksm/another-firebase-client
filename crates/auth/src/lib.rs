//! Use this got get OAuth tokens from Google Cloud service accounts, connecting
//! to the gcloud cli and the authentication available through it, or with "end
//! user" authentication via email/password or other sign-in providers.
//!
//! Example usage that generates a `bearer_token` string which can be used as an
//! Authorization header value:
//!
//! ```ignore
//! let account = auth::GoogleServiceAccount::from_env_var("GOOGLE_SERVICE_ACCOUNT")?;
//! let mut token = auth::GToken::new(account, &[auth::scopes::AUTH_LOGGING_READ]);
//! let bearer_token = token.refresh_if_necessary().await?;
//! ```

pub mod error;
pub mod scopes;

mod authorization;
mod cli;
mod end_user;
mod service_account;
mod token;

pub use authorization::Authorization;
pub use cli::CliAuthorization;
pub use end_user::{EmailSignin, WebClientConfig, WebLoginResult, WebUserAnonAuth, WebUserAuth};
pub use service_account::{GoogleServiceAccount, ServiceAccountAuthorization};
pub use token::GToken;

pub type GoogleAuth = Box<dyn Authorization + Send>;

/// Will try to load service account credentials from the
/// $GOOGLE_SERVICE_ACCOUNT env var.
/// If that fails, will try to get auth credentials from the gcloud cli utility.
pub fn auth_from_env_or_cli() -> Option<GoogleAuth> {
    match GoogleServiceAccount::from_default_env_var().map(|acct| {
        ServiceAccountAuthorization::with_account_and_scope(
            acct,
            &[
                scopes::AUTH_DATASTORE,
                scopes::AUTH_USERINFO_EMAIL,
                scopes::AUTH_CLOUD_PLATFORM,
                scopes::AUTH_LOGGING_READ,
            ],
        )
    }) {
        Err(_) => {
            tracing::debug!("No service account specified in environment variable.");
        }
        Ok(auth) => return Some(Box::new(auth)),
    };

    match CliAuthorization::new() {
        Err(err) => {
            tracing::error!("Unable to load gcloud cli auth: {err}");
        }
        Ok(auth) => return Some(Box::new(auth)),
    };

    None
}
