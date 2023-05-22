/// Firebase client configuration as found at https://console.firebase.google.com/
#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WebClientConfig {
    pub api_key: String,
    pub auth_domain: String,
    #[serde(rename = "databaseURL")]
    pub database_url: String,
    pub project_id: String,
    pub storage_bucket: String,
    pub messaging_sender_id: String,
    pub app_id: String,
}
