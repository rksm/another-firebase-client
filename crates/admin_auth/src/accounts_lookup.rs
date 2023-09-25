use crate::{AdminAuthError, User};
use firebase_client_auth::GoogleAuth;
use serde::{Deserialize, Serialize};

// See https://cloud.google.com/identity-platform/docs/reference/rest/v1/projects.accounts/lookup
// Payload like:
// {
//   "idToken": string,
//   "localId": [string],
//   "email": [string],
//   "delegatedProjectNumber": string,
//   "phoneNumber": [string],
//   "federatedUserId": [{object (FederatedUserIdentifier)}],
//   "tenantId": string,
//   "initialEmail": [string]
// }
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct AccountsLookupRequest {
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    emails: Option<Vec<String>>,

    #[serde(rename = "initialEmail", skip_serializing_if = "Option::is_none")]
    initial_emails: Option<Vec<String>>,

    #[serde(rename = "localId", skip_serializing_if = "Option::is_none")]
    uids: Option<Vec<String>>,

    #[serde(rename = "phoneNumber", skip_serializing_if = "Option::is_none")]
    phone_numbers: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AccountsLookupResponse {
    users: Vec<User>,
}

pub struct AccountLookup {
    pub emails: Vec<String>,
    pub uids: Vec<String>,
    pub phone_numbers: Vec<String>,
    pub auth: GoogleAuth,
}

impl AccountLookup {
    pub fn new(auth: GoogleAuth) -> Self {
        Self {
            emails: Vec::new(),
            uids: Vec::new(),
            phone_numbers: Vec::new(),
            auth,
        }
    }

    pub fn emails(mut self, values: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.emails.extend(values.into_iter().map(|s| s.into()));
        self
    }

    pub fn uids(mut self, values: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.uids.extend(values.into_iter().map(|s| s.into()));
        self
    }

    pub fn phone_numbers(mut self, values: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.phone_numbers
            .extend(values.into_iter().map(|s| s.into()));
        self
    }

    pub async fn fetch(self) -> Result<Vec<User>, AdminAuthError> {
        let Self {
            emails,
            auth,
            uids,
            phone_numbers,
        } = self;
        let req = AccountsLookupRequest {
            emails: if emails.is_empty() {
                None
            } else {
                Some(emails)
            },
            initial_emails: None,
            uids: if uids.is_empty() { None } else { Some(uids) },
            phone_numbers: if phone_numbers.is_empty() {
                None
            } else {
                Some(phone_numbers)
            },
        };
        let response = account_lookup(&auth, req).await?;
        Ok(response.users)
    }
}

async fn account_lookup(
    auth: &GoogleAuth,
    req: AccountsLookupRequest,
) -> Result<AccountsLookupResponse, AdminAuthError> {
    let token = auth
        .get_token()
        .await?
        .ok_or_else(|| AdminAuthError::NoToken)?;

    let project_id = auth.project_id();

    let client = reqwest::Client::new();

    let body = serde_json::to_string(&req)?;

    let req = client
        .post(format!(
            "https://identitytoolkit.googleapis.com/v1/projects/{project_id}/accounts:lookup",
        ))
        .query(&[("access_token", token)])
        .body(body)
        .build()?;

    let res = client.execute(req).await?;

    if !res.status().is_success() {
        let status = res.status();
        let body = res.text().await?;
        return Err(AdminAuthError::BadStatus(status, body));
    }

    Ok(res.json().await?)
}
