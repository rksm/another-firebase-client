use crate::{AdminAuthError, User};
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

    pub async fn fetch(self) -> impl Stream<Item = Result<User, AdminAuthError>> {
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
) -> Result<T, AdminAuthError>
where
    T: DeserializeOwned,
{
    let token = auth
        .get_token()
        .await?
        .ok_or_else(|| AdminAuthError::NoToken)?;

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

    if !res.status().is_success() {
        let status = res.status();
        let body = res.text().await?;
        return Err(AdminAuthError::BadStatus(status, body));
    }

    Ok(res.json().await?)
}
