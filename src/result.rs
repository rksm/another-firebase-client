pub type Result<T> = std::result::Result<T, FirestoreError>;

#[derive(thiserror::Error, Debug)]
pub enum FirestoreError {
    #[error("Authentication error: {0}")]
    AuthenticationError(#[from] crate::auth::error::GCloudAuthError),

    #[error("GRPC error: {0}")]
    GrpcError(#[from] firestore_grpc::tonic::Status),

    #[error("Invalid GRPC metadata: {0}")]
    InvalidMetadata(#[from] firestore_grpc::tonic::metadata::errors::InvalidMetadataValue),

    #[error("GRPC connection error")]
    ConnectionError(#[from] firestore_grpc::tonic::transport::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Conversion error: {0}")]
    ConversionError(#[from] FirestoreConversionError),
}

#[derive(thiserror::Error, Debug)]
pub enum RealtimeDBError {
    #[error("Authentication error: {0}")]
    AuthenticationError(#[from] crate::auth::error::GCloudAuthError),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("Request failure, status: {0}, message: {1}")]
    RequestFailure(reqwest::StatusCode, String),

    #[error("URL error: {0}")]
    UrlError(#[from] url::ParseError),

    #[error("error: {0}")]
    Other(&'static str),
}

#[derive(thiserror::Error, Debug)]
pub enum FirestoreConversionError {
    #[error("IntoFirestoreDocument conversion error: {0}")]
    IntoFirestoreError(String),

    #[error("FromFirestoreDocument conversion error: {0}")]
    FromFirestoreError(String),
}

#[derive(thiserror::Error, Debug)]
pub enum CachedCollectionError {
    #[error("Error reading cache file: {0}")]
    CacheReadError(#[from] std::io::Error),

    #[error("No cache file found")]
    NoCacheFile,

    #[error("Error in cache serialization: {0}")]
    CacheSerializationError(#[from] serde_json::Error),

    #[error("Firestore error")]
    FirestoreError(#[from] FirestoreError),
}
