use async_trait::async_trait;
use chrono::prelude::*;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{Authorization, GoogleAuth};

use super::error::GCloudAuthError;

#[derive(Debug, Clone)]
pub struct CliAuthorization {
    project_id: String,
    info: Arc<Mutex<AuthInfo>>,
}

impl CliAuthorization {
    pub fn new() -> Result<Self, GCloudAuthError> {
        let info = AuthInfo::from_cli()?;
        Ok(Self {
            project_id: info.project_id.clone(),
            info: Arc::new(Mutex::new(info)),
        })
    }
}

#[async_trait]
impl Authorization for CliAuthorization {
    fn project_id(&self) -> &str {
        self.project_id.as_str()
    }

    async fn get_token(&self) -> Result<Option<String>, GCloudAuthError> {
        tracing::debug!("[CliAuthorization] attempting to get access token");
        loop {
            match self.info.try_lock() {
                Err(_) => {
                    tracing::debug!(
                        "[CliAuthorization] token lock is being held, retrying in a bit..."
                    );
                    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
                }
                Ok(mut info) => {
                    if info.is_expired() {
                        *info = AuthInfo::from_cli()?;
                    }
                    return Ok(Some(info.access_token.clone()));
                }
            }
        }
    }

    fn box_clone(&self) -> GoogleAuth {
        Box::new(self.clone())
    }
}

// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-

#[derive(Debug)]
struct AuthInfo {
    project_id: String,
    #[allow(dead_code)]
    client_id: String,
    #[allow(dead_code)]
    client_secret: String,
    expiry: DateTime<Utc>,
    #[allow(dead_code)]
    id_token: String,
    #[allow(dead_code)]
    id_tokenb64: String,
    #[allow(dead_code)]
    refresh_token: String,
    access_token: String,
    #[allow(dead_code)]
    token_uri: String,
    #[allow(dead_code)]
    auth_type: String,
}

impl AuthInfo {
    fn from_cli() -> Result<Self, GCloudAuthError> {
        let (project_id, info) = fetch_auth_info()?;
        Self::parse_auth_info(project_id, info)
    }

    fn parse_auth_info(
        project_id: impl ToString,
        input: impl AsRef<str>,
    ) -> Result<Self, GCloudAuthError> {
        let mut client_id = String::new();
        let mut client_secret = String::new();
        let mut expiry = None;
        let mut id_token = String::new();
        let mut id_tokenb64 = String::new();
        let mut refresh_token = String::new();
        let mut access_token = String::new();
        let mut token_uri = String::new();
        let mut auth_type = String::new();

        for line in input.as_ref().trim().lines() {
            if line.starts_with("client_id:") {
                if let Some((_, val)) = line.split_once(' ') {
                    client_id = val.to_string();
                }
            }

            if line.starts_with("client_secret:") {
                if let Some((_, val)) = line.split_once(' ') {
                    client_secret = val.to_string();
                }
            }

            if line.starts_with("expiry:") {
                if let Some((_, val)) = line.split_once(' ') {
                    expiry = Some(Utc.from_utc_datetime(
                        &NaiveDateTime::parse_from_str(val, "%m-%d-%Y %H:%M:%S").map_err(
                            |err| {
                                GCloudAuthError::GCloudCliParseError(format!(
                                    "cannot parse expiry date time: {err}"
                                ))
                            },
                        )?,
                    ));
                }
            }

            if line.starts_with("id_token:") {
                if let Some((_, val)) = line.split_once(' ') {
                    id_token = val.to_string();
                }
            }

            if line.starts_with("id_tokenb64:") {
                if let Some((_, val)) = line.split_once(' ') {
                    id_tokenb64 = val.to_string();
                }
            }

            if line.starts_with("refresh_token:") {
                if let Some((_, val)) = line.split_once(' ') {
                    refresh_token = val.to_string();
                }
            }

            if line.starts_with("token:") {
                if let Some((_, val)) = line.split_once(' ') {
                    access_token = val.to_string();
                }
            }

            if line.starts_with("token_uri:") {
                if let Some((_, val)) = line.split_once(' ') {
                    token_uri = val.to_string();
                }
            }

            if line.starts_with("type:") {
                if let Some((_, val)) = line.split_once(' ') {
                    auth_type = val.to_string();
                }
            }

            if line.starts_with("expired:") {
                if let Some((_, val)) = line.split_once(' ') {
                    if val != "false" {
                        return Err(GCloudAuthError::GCloudCliParseError(
                            "gcloud cli token already expired".to_string(),
                        ));
                    }
                }
            }
        }

        let expiry = match expiry {
            None => {
                return Err(GCloudAuthError::GCloudCliParseError(
                    "gcloud auth describe did not provide expiry".to_string(),
                ))
            }
            Some(expiry) => expiry,
        };

        Ok(Self {
            project_id: project_id.to_string(),
            client_id,
            client_secret,
            expiry,
            id_token,
            id_tokenb64,
            refresh_token,
            access_token,
            token_uri,
            auth_type,
        })
    }

    fn is_expired(&self) -> bool {
        Utc::now() > self.expiry - chrono::Duration::seconds(30)
    }
}

fn run_cmd(cmd_string: impl AsRef<str>) -> Result<String, GCloudAuthError> {
    let mut args = cmd_string.as_ref().split(' ');
    let cmd = args.next().expect("run_cmd args is empty");
    let result = std::process::Command::new(cmd)
        .args(args)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .spawn()
        .and_then(|proc| proc.wait_with_output())
        .map_err(|err| GCloudAuthError::GCloudCliCommandError(format!("{err}")))?;
    let stdout = String::from_utf8_lossy(&result.stdout);
    Ok(stdout.to_string())
}

fn fetch_auth_info() -> Result<(String, String), GCloudAuthError> {
    if which::which("gcloud").is_err() {
        return Err(GCloudAuthError::GCloudCliNotInstalled);
    }

    let project_id_thread = std::thread::spawn(move || {
        run_cmd("gcloud config get project").map(|out| out.trim().to_string())
    });

    let info_thread = std::thread::spawn(move || {
        let list_output = run_cmd("gcloud auth list")?;
        let active = list_output.lines().find(|line| line.starts_with('*'));
        let email = match active {
            None => return Err(GCloudAuthError::GCloudCliNotLoggedIn),
            Some(line) => {
                let email = line.trim_start_matches(['*', ' ']);
                email.to_string()
            }
        };

        run_cmd(format!("gcloud auth describe {email}"))
    });

    let project_id = project_id_thread
        .join()
        .map_err(|err| GCloudAuthError::GCloudCliCommandError(format!("{err:?}")))??;
    let info = info_thread
        .join()
        .map_err(|err| GCloudAuthError::GCloudCliCommandError(format!("{err:?}")))??;

    Ok((project_id, info))
}

#[cfg(test)]
mod tests {
    use super::AuthInfo;

    #[test]
    fn parse_auth_info_test() {
        let input = "
client_id: 32555940559.apps.googleusercontent.com
client_secret: ZmssLNjJy2998hD4CTg2ejr2
default_scopes: null
expired: false
expiry: 06-14-2022 01:29:21
id_token: eyJhbGciOiJSUzI1NiIsImtpZCI6IjU4MGFkYjBjMzJhMTc1ZDk1MGExYzE5MDFjMTgyZmMxNzM0MWRkYzQiLCJ0eXAiOiJKV1QifQ.eyJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLCJhenAiOiIzMjU1NTk0MDU1OS5hcHBzLmdvb2dsZXVzZXJjb250ZW50LmNvbSIsImF1ZCI6IjMyNTU1OTQwNTU5LmFwcHMuZ29vZ2xldXNlcmNvbnRlbnQuY29tIiwic3ViIjoiMTExMDgwMzEwMTc1NTQ4NzA4Njc2IiwiaGQiOiJkYXRhZG9naHEuY29tIiwiZW1haWwiOiJyb2JlcnQua3JhaG5AZGF0YWRvZ2hxLmNvbSIsImVtYWlsX3ZlcmlmaWVkIjp0cnVlLCJhdF9oYXNoIjoicHQ4X3NOUlU5c2M4eTNZN0d3OEZIZyIsImlhdCI6MTY1NTE2NjU2MiwiZXhwIjoxNjU1MTcwMTYyfQ.s9rOhzS29fVMRuDG9KZuIOjTlz0ozkJa2snZzR3UbTEHVzIG1NjXJ3npdSs8jFZuVm41QthkCZCw51hD5etp3LiHAcDbY0F1i9QqrhGEKlQBIuPFieWT-kNaw_UJaajR_elhLEK5zMvPuBv3hA5h4OzhYiD38IwOpa59U8v90B6egrHr5MkB-EbkGIURof-1-v8SUk9XD-gxC_GOz_NuMjSOOs_X7i4FDvn8IWVIm3YgAnmIuA4O4eWMDym6UHJfu5azuaC7Trn8CIhDx7hKPGTHyPtmuM40tL6nXC7eqn_JSgVqlOQ8Ml7EVdcVQCNxWdHBvxKxVvHaC2KtEqra0A
id_tokenb64: eyJhbGciOiJSUzI1NiIsImtpZCI6IjU4MGFkYjBjMzJhMTc1ZDk1MGExYzE5MDFjMTgyZmMxNzM0MWRkYzQiLCJ0eXAiOiJKV1QifQ.eyJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLCJhenAiOiIzMjU1NTk0MDU1OS5hcHBzLmdvb2dsZXVzZXJjb250ZW50LmNvbSIsImF1ZCI6IjMyNTU1OTQwNTU5LmFwcHMuZ29vZ2xldXNlcmNvbnRlbnQuY29tIiwic3ViIjoiMTExMDgwMzEwMTc1NTQ4NzA4Njc2IiwiaGQiOiJkYXRhZG9naHEuY29tIiwiZW1haWwiOiJyb2JlcnQua3JhaG5AZGF0YWRvZ2hxLmNvbSIsImVtYWlsX3ZlcmlmaWVkIjp0cnVlLCJhdF9oYXNoIjoicHQ4X3NOUlU5c2M4eTNZN0d3OEZIZyIsImlhdCI6MTY1NTE2NjU2MiwiZXhwIjoxNjU1MTcwMTYyfQ.s9rOhzS29fVMRuDG9KZuIOjTlz0ozkJa2snZzR3UbTEHVzIG1NjXJ3npdSs8jFZuVm41QthkCZCw51hD5etp3LiHAcDbY0F1i9QqrhGEKlQBIuPFieWT-kNaw_UJaajR_elhLEK5zMvPuBv3hA5h4OzhYiD38IwOpa59U8v90B6egrHr5MkB-EbkGIURof-1-v8SUk9XD-gxC_GOz_NuMjSOOs_X7i4FDvn8IWVIm3YgAnmIuA4O4eWMDym6UHJfu5azuaC7Trn8CIhDx7hKPGTHyPtmuM40tL6nXC7eqn_JSgVqlOQ8Ml7EVdcVQCNxWdHBvxKxVvHaC2KtEqra0A
quota_project_id: null
rapt_token: null
refresh_handler: null
refresh_token: 1//097bDqajxGiaYCgYIARAAGAkSNwF-L9Ir074UVexnjRQZAa6IdGFL_wduXEB0rkBz-8B4VcZTn0vc_JTqj2xDNH8PLlp-fQNvv9c
requires_scopes: false
scopes:
- openid
- https://www.googleapis.com/auth/userinfo.email
- https://www.googleapis.com/auth/cloud-platform
- https://www.googleapis.com/auth/appengine.admin
- https://www.googleapis.com/auth/compute
- https://www.googleapis.com/auth/accounts.reauth
token: ya29.a0ARrdaM_3U5vY1fN9My33Z2QZCz7cVD6wRoTth6ztR48Y1sohrV0Sj9EKRDQVkiROjdi4VOpUxiWf22Nyrte1VVXi1NhGeFittxDNEGq-0ZzoMWUSdRk0DkzmktF2tFGcu_NCKjrPeiJTnoDgMqOP1SW2sLMVrXmkvozALQ
token_uri: https://oauth2.googleapis.com/token
type: authorized_user
valid: true
";

        let info = AuthInfo::parse_auth_info("test", input).expect("parse auth info");
        assert!(info.is_expired());
        assert_eq!(info.access_token, "ya29.a0ARrdaM_3U5vY1fN9My33Z2QZCz7cVD6wRoTth6ztR48Y1sohrV0Sj9EKRDQVkiROjdi4VOpUxiWf22Nyrte1VVXi1NhGeFittxDNEGq-0ZzoMWUSdRk0DkzmktF2tFGcu_NCKjrPeiJTnoDgMqOP1SW2sLMVrXmkvozALQ");
    }
}
