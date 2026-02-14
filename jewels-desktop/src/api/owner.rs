use serde::{Deserialize, Serialize};
use libjewels::configuration::load_config;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Owner {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub is_admin: bool,
    pub profile_picture: String,
}

pub async fn get_owners() -> anyhow::Result<Vec<Owner>> {
    let config = load_config();

    let client = reqwest::Client::new();
    let res = client
        .get(format!("{}/api/owner/other", config.host))
        .bearer_auth(config.token)
        .send()
        .await?;

    let owners = res.json().await?;
    Ok(owners)
}
