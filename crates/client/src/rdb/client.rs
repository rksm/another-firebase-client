use anyhow::Result;
use firebase_client_auth::{scopes, GoogleAuth, GoogleServiceAccount, ServiceAccountAuthorization};
use reqwest::{Body, Method, Response, Url};

pub struct RdbClient {
    pub auth: GoogleAuth,
    shallow: bool,
}

#[derive(Debug, serde::Deserialize)]
pub struct PostResult {
    pub name: String,
}

impl RdbClient {
    pub fn for_account(account: GoogleServiceAccount) -> Result<Self> {
        let auth = Box::new(ServiceAccountAuthorization::with_account_and_scope(
            account,
            &[scopes::AUTH_DATASTORE, scopes::AUTH_USERINFO_EMAIL],
        ));
        Ok(RdbClient::new(auth))
    }

    pub fn new(auth: GoogleAuth) -> Self {
        RdbClient {
            auth,
            shallow: false,
        }
    }

    pub fn shallow(mut self, shallow: bool) -> Self {
        self.shallow = shallow;
        self
    }

    pub fn project_id(&self) -> &str {
        self.auth.project_id()
    }

    async fn make_request<T: Into<Body>, S: AsRef<str>>(
        &mut self,
        method: Method,
        path: S,
        body: Option<T>,
        params: Option<&[&str]>,
    ) -> Result<Response> {
        let url = format!("https://{}.firebaseio.com/", self.project_id());
        let mut url = Url::parse(&url)?.join(path.as_ref())?;
        if let Some(params) = params {
            for query in params {
                url.set_query(Some(query));
            }
        };

        tracing::debug!("rdb request {}", url);

        let token = self.auth.get_token().await?;
        let mut client = reqwest::Client::new()
            .request(method, url)
            .bearer_auth(&token);
        if let Some(body) = body {
            client = client.body(body);
        }
        self.check_status(path, client.send().await?).await
    }

    async fn check_status<S: AsRef<str>>(&self, path: S, res: Response) -> Result<Response> {
        let status = res.status();
        if status.is_success() {
            return Ok(res);
        }
        let message = res.text().await.unwrap_or_else(|_| String::new());
        return Err(anyhow::anyhow!(
            "Error putting {}:\nStatus={}\nmessage={}",
            path.as_ref(),
            status,
            message
        ));
    }

    pub async fn get_path<T: serde::de::DeserializeOwned>(&mut self, path: &str) -> Result<T> {
        let params = if self.shallow {
            Some(["shallow=true"].as_slice())
        } else {
            None
        };
        let res = self
            .make_request::<&str, _>(Method::GET, path, None, params)
            .await?;
        Ok(res.json::<T>().await?)
    }

    pub async fn put_path<T: serde::ser::Serialize>(
        &mut self,
        path: &str,
        value: &T,
    ) -> Result<()> {
        let data = serde_json::to_string(value)?;
        self.make_request(Method::PUT, path, Some(data), None)
            .await?;
        Ok(())
    }

    pub async fn patch_path<T: serde::ser::Serialize>(
        &mut self,
        path: &str,
        value: &T,
    ) -> Result<()> {
        let data = serde_json::to_string(value)?;
        self.make_request(Method::PATCH, path, Some(data), None)
            .await?;
        Ok(())
    }

    pub async fn post_path<T: serde::ser::Serialize>(
        &mut self,
        path: &str,
        value: &T,
    ) -> Result<PostResult> {
        let data = serde_json::to_string(value)?;
        let res = self
            .make_request(Method::POST, path, Some(data), None)
            .await?;
        Ok(res.json::<PostResult>().await?)
    }

    pub async fn delete_path(&mut self, path: &str) -> Result<()> {
        self.make_request::<&str, _>(Method::DELETE, path, None, None)
            .await?;
        Ok(())
    }
}
