use crate::AccountBatchGetError;
use firebase_client_auth::GoogleAuth;
use futures::stream::Stream;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AccountsBatchGetResponse {
    kind: String,
    next_page_token: Option<String>,
    users: Vec<User>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum User {
    FullUser(RegisteredUser),
    AnonUser(AnonUser),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnonUser {
    #[serde(rename = "localId")]
    pub uid: String,
    #[serde(with = "timestamp_serialization")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_refresh_at: chrono::DateTime<chrono::Utc>,
    pub last_login_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisteredUser {
    #[serde(rename = "localId")]
    pub uid: String,
    pub email: String,
    pub email_verified: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_since: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    #[serde(with = "timestamp_serialization_opt")]
    pub last_login_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(with = "timestamp_serialization")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_auth: Option<bool>,
    pub provider_user_info: Vec<ProviderUserInfo>,
    pub last_refresh_at: chrono::DateTime<chrono::Utc>,
}

mod timestamp_serialization {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    // serialize to milliseconds since the epoch
    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let millis = date.timestamp_millis().to_string();
        serializer.serialize_str(&millis)
    }

    // de serialize to milliseconds since the epoch
    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let millis = String::deserialize(deserializer)?;
        let millis = millis.parse::<i64>().map_err(serde::de::Error::custom)?;
        Utc.timestamp_millis_opt(millis)
            .earliest()
            .ok_or_else(|| serde::de::Error::custom(format!("invalid timestamp: {}", millis)))
    }
}

mod timestamp_serialization_opt {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    // serialize to milliseconds since the epoch
    pub fn serialize<S>(date: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let Some (date) = date else {
            return serializer.serialize_none();
        };
        let millis = date.timestamp_millis().to_string();
        serializer.serialize_str(&millis)
    }

    // de serialize to milliseconds since the epoch
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let millis = String::deserialize(deserializer)?;
        let millis = millis.parse::<i64>().map_err(serde::de::Error::custom)?;
        Ok(Some(
            Utc.timestamp_millis_opt(millis).earliest().ok_or_else(|| {
                serde::de::Error::custom(format!("invalid timestamp: {}", millis))
            })?,
        ))
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderUserInfo {
    pub provider_id: String,
    pub federated_id: String,
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_url: Option<String>,
}

/// Makes projects.accounts.batchGet requests. Streams the response.
/// https://cloud.google.com/identity-platform/docs/reference/rest/v1/projects.accounts/batchGet
///
/// Example:
///
/// ```ignore
///  let auth = auth::auth_from_env_or_cli().expect("Failed to get auth");
///  let stream = admin_auth::AccountBatchGet::new(auth.box_clone())
///      .max_results(500)
///      .fetch()
///      .await;
///  let mut stream = std::pin::pin!(stream);
///  let mut users = Vec::new();
///  while let Some(next) = stream.next().await {
///      let user = match next {
///          Err(e) => {
///              tracing::error!("Error getting user: {:?}", e);
///              continue;
///          }
///          Ok(next) => next,
///      };
///      if let User::FullUser(user) = user {
///          users.push(user);
///      }
///  }
///
///  println!("{}", serde_json::to_string_pretty(&users).unwrap());
///```
pub struct AccountBatchGet {
    pub page_size: Option<u32>,
    pub auth: GoogleAuth,
}

impl AccountBatchGet {
    pub fn new(auth: GoogleAuth) -> Self {
        Self {
            page_size: None,
            auth,
        }
    }

    pub fn max_results(mut self, max_results: u32) -> Self {
        self.page_size = Some(max_results);
        self
    }

    pub async fn fetch(self) -> impl Stream<Item = Result<User, AccountBatchGetError>> {
        let Self {
            page_size: max_results,
            auth,
        } = self;

        let mut next_page_token = None;

        async_stream::try_stream! {
            loop {

                let AccountsBatchGetResponse {users, next_page_token: t, ..} =
                    account_batch_get(&auth, max_results.unwrap_or(20), next_page_token).await?;
                next_page_token = t;
                for user in users {
                    yield user;
                }

                if next_page_token.is_none() {
                    break;
                }
            }
        }
    }
}

pub async fn account_batch_get<T>(
    auth: &GoogleAuth,
    page_size: u32,
    next_page_token: Option<String>,
) -> Result<T, AccountBatchGetError>
where
    T: DeserializeOwned,
{
    let token = auth
        .get_token()
        .await?
        .ok_or_else(|| AccountBatchGetError::NoToken)?;

    let project_id = auth.project_id();

    let client = reqwest::Client::new();

    let req = client
        .get(format!(
            "https://identitytoolkit.googleapis.com/v1/projects/{project_id}/accounts:batchGet",
        ))
        .query(&[
            ("maxResults", page_size.to_string()),
            ("nextPageToken", next_page_token.unwrap_or_default()),
            ("access_token", token),
        ])
        .build()?;

    let res = client.execute(req).await?;
    Ok(res.json().await?)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    #[test]
    fn serialization_test() {
        let json = json!({
            "localId": "BPP9y7zZ2mcDhiSLjFtkdLgeYpw2",
            "email": "foo2@bar.com",
            "emailVerified": false,
            "passwordHash": "2ksZ7ZaEqjdvqLh0VECETvrxqp44G0rEuBC6bWQGESRWCDLjd2uNjn3QYgQj6unuBuEaGzYqJpdx7XtTJb3AhQ==",
            "validSince": "1684862103",
            "lastLoginAt": "1684862103954",
            "createdAt": "1684862103954",
            "providerUserInfo": [
                {
                    "providerId": "password",
                    "federatedId": "foo2@bar.com",
                    "email": "foo2@bar.com",
                    "rawId": "foo2@bar.com"
                }
            ],
            "lastRefreshAt": "2023-05-23T17:15:03.954Z"
        });

        let user: super::RegisteredUser = serde_json::from_value(json).unwrap();
        serde_json::to_string_pretty(&user).unwrap();
    }
}
