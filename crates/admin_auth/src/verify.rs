use chrono::prelude::*;
use jwt_simple::prelude::*;
use std::{collections::HashMap, sync::Arc};
use thiserror::Error;
use tokio::sync::Mutex;

pub type TokenVerificationResult<T> = std::result::Result<T, TokenVerificationError>;

#[derive(Error, Debug)]
pub enum TokenVerificationError {
    #[error("public key missing or not found")]
    PublicKeyMissingOrNotFound,
    #[error("public key processing error: {0}")]
    PublicKeyProcessingError(String),
    #[error("uid mismatch")]
    UidMismatch,
    #[error("failed to decode token: {0}")]
    TokenDecodeFailure(#[from] jwt_simple::Error),
    #[error("unsupported algorithm: {0}")]
    UnsupportedAlgorithm(String),
    #[error("failed to parse public key")]
    PublicKeyParseError(#[from] x509_certificate::X509CertificateError),
    #[error("failed to verify token: {0}")]
    TokenVerificationFailure(String),
    #[error("expected token to have field {0}")]
    TokenMissingField(&'static str),
}

fn public_key_processing_error<S1, S2>(msg: S1) -> impl FnOnce(S2) -> TokenVerificationError
where
    S1: ToString,
    S2: ToString,
{
    move |err| {
        TokenVerificationError::PublicKeyProcessingError(format!(
            "{}: {}",
            msg.to_string(),
            err.to_string()
        ))
    }
}

#[derive(Clone, Debug)]
struct SharedGooglePublicKeys(Arc<Mutex<GooglePublicKeys>>);

impl SharedGooglePublicKeys {
    async fn new() -> TokenVerificationResult<Self> {
        static INSTANCE: std::sync::OnceLock<SharedGooglePublicKeys> = std::sync::OnceLock::new();
        if let Some(this) = INSTANCE.get() {
            let this = this.clone();
            this.renew_if_needed().await?;
            return Ok(this);
        };

        let keys = Self(Arc::new(Mutex::new(GooglePublicKeys::fetch().await?)));
        INSTANCE
            .set(keys.clone())
            .expect("failed to set shared google public keys");
        Ok(keys)
    }

    async fn renew_if_needed(&self) -> TokenVerificationResult<()> {
        let mut this = self.0.lock().await;
        if this.is_expired() {
            this.renew_if_needed().await?;
        }
        Ok(())
    }

    pub async fn get_key(&self, key: impl AsRef<str>) -> Option<String> {
        let this = self.0.lock().await;
        this.get_key(key).map(|s| s.to_string())
    }
}

#[derive(Clone, Debug)]
struct GooglePublicKeys {
    expires_at: std::time::SystemTime,
    keys: std::collections::HashMap<String, String>,
}

impl GooglePublicKeys {
    async fn fetch() -> TokenVerificationResult<Self> {
        let public_keys_request = reqwest::get(
                "https://www.googleapis.com/robot/v1/metadata/x509/securetoken@system.gserviceaccount.com",
            )
            .await
            .map_err(public_key_processing_error("failed to fetch public keys"))?;

        let expires_at = public_keys_request
            .headers()
            .get("expires")
            .ok_or_else(|| {
                TokenVerificationError::PublicKeyProcessingError(
                    "missing expires header".to_string(),
                )
            })?
            .to_str()
            .map_err(public_key_processing_error(
                "failed to parse expires header",
            ))?
            .parse::<httpdate::HttpDate>()
            .map_err(public_key_processing_error(
                "failed to parse expires header",
            ))?
            .into();

        let keys = public_keys_request
            .json::<std::collections::HashMap<String, String>>()
            .await
            .map_err(public_key_processing_error("failed to parse public keys"))?;

        Ok(Self { expires_at, keys })
    }

    async fn renew_if_needed(&mut self) -> TokenVerificationResult<()> {
        if self.is_expired() {
            *self = Self::fetch().await?;
        }
        Ok(())
    }

    fn is_expired(&self) -> bool {
        self.expires_at < std::time::SystemTime::now()
    }

    fn get_key(&self, key: impl AsRef<str>) -> Option<&str> {
        self.keys.get(key.as_ref()).map(|s| s.as_str())
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct CustomClaims {
    #[serde(deserialize_with = "deserialize_datetime_from_seconds")]
    auth_time: DateTime<Utc>,
    user_id: String,
    email: String,
    email_verified: bool,
    firebase: FirebaseClaims,
}

fn deserialize_datetime_from_seconds<'de, D>(
    deserializer: D,
) -> std::result::Result<DateTime<Utc>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let seconds = i64::deserialize(deserializer)?;
    Ok(Utc
        .timestamp_opt(seconds, 0)
        .single()
        .unwrap_or_else(Utc::now))
}

#[derive(Debug, Serialize, Deserialize)]
struct FirebaseClaims {
    identities: HashMap<String, Vec<String>>,
    sign_in_provider: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub issued_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub issuer: String,
    pub subject: String,
    pub audiences: Vec<String>,

    // custom claims
    pub auth_time: DateTime<Utc>,
    pub user_id: String,
    pub email: String,
    pub email_verified: bool,
    pub firebase_identities: HashMap<String, Vec<String>>,
    pub sign_in_provider: String,
}

/// Used to verify a custom id token, generated with `user.getIdToken()`.
#[derive(typed_builder::TypedBuilder)]
pub struct TokenVerification {
    #[builder(setter(into))]
    token: String,
    #[builder(setter(into))]
    project_id: String,
    /// Can be the uid to verify
    #[builder(default)]
    required_subject: Option<String>,
}

impl TokenVerification {
    pub async fn verify(self) -> TokenVerificationResult<TokenClaims> {
        let Self {
            token,
            project_id,
            required_subject,
        } = self;

        // step 1: check token header
        let metadata =
            Token::decode_metadata(&token).map_err(TokenVerificationError::TokenDecodeFailure)?;
        let key_id = metadata.key_id();
        let algorithm = metadata.algorithm();

        if algorithm != "RS256" {
            return Err(TokenVerificationError::UnsupportedAlgorithm(
                algorithm.to_string(),
            ));
        }

        let Some(key_id) = key_id else {
            return Err(TokenVerificationError::PublicKeyMissingOrNotFound);
        };

        // step 2: if the key referring to a google public key and was it signed by
        // google?
        let keys = SharedGooglePublicKeys::new().await?;
        let Some(key) = keys.get_key(key_id).await else {
            return Err(TokenVerificationError::PublicKeyMissingOrNotFound);
        };

        let cert = x509_certificate::X509Certificate::from_pem(key)?;
        let der = cert.public_key_data();
        let key = RS256PublicKey::from_der(&der).map_err(|_| {
            TokenVerificationError::PublicKeyProcessingError(
                "failed to parse public key".to_string(),
            )
        })?;

        // step 3: verify token
        let claims = key.verify_token::<CustomClaims>(
            &token,
            Some(VerificationOptions {
                required_subject,
                allowed_issuers: Some(
                    [format!("https://securetoken.google.com/{project_id}")].into(),
                ),
                allowed_audiences: Some([project_id.to_string()].into()),
                ..Default::default()
            }),
        )?;

        // step 4: extract claims / custom claims
        let Some(issued_at) = claims.issued_at else {
            return Err(TokenVerificationError::TokenMissingField("issued_at"));
        };
        let Some(expires_at) = claims.expires_at else {
            return Err(TokenVerificationError::TokenMissingField("expires_at"));
        };

        let issued_at = DateTime::<Utc>::from_naive_utc_and_offset(
            NaiveDateTime::from_timestamp_millis(issued_at.as_millis() as i64).ok_or_else(
                || TokenVerificationError::TokenMissingField("issued_at (invalid timestamp)"),
            )?,
            Utc,
        );

        let expires_at = DateTime::<Utc>::from_naive_utc_and_offset(
            NaiveDateTime::from_timestamp_millis(expires_at.as_millis() as i64).ok_or_else(
                || TokenVerificationError::TokenMissingField("expires_at (invalid timestamp)"),
            )?,
            Utc,
        );

        let Some(issuer) = claims.issuer else {
            return Err(TokenVerificationError::TokenMissingField("issuer"));
        };

        let Some(subject) = claims.subject else {
            return Err(TokenVerificationError::TokenMissingField("subject"));
        };

        let audiences = match claims.audiences {
            Some(Audiences::AsSet(audiences)) => audiences.into_iter().collect::<Vec<_>>(),
            Some(Audiences::AsString(audience)) => vec![audience],
            None => {
                return Err(TokenVerificationError::TokenMissingField("audiences"));
            }
        };

        Ok(TokenClaims {
            issued_at,
            expires_at,
            issuer,
            subject,
            audiences,
            auth_time: claims.custom.auth_time,
            user_id: claims.custom.user_id,
            email: claims.custom.email,
            email_verified: claims.custom.email_verified,
            firebase_identities: claims.custom.firebase.identities,
            sign_in_provider: claims.custom.firebase.sign_in_provider,
        })
    }
}
