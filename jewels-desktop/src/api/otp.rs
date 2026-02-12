use libjewels::configuration::load_config;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::api::owner::Owner;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OneTimePassword {
    pub id: i64,
    pub account_name: String,
    pub account_issuer: String,
    pub secret_key: String,
    pub can_edit: bool,
    pub brand_icon: String,
    pub simple_icon: String,
    pub brand_icon_similarity: f64,
    pub simple_icon_similarity: f64,
    pub shared_with: Vec<Owner>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SharedOneTimePassword {
    pub id: i64,
    pub account_name: String,
    pub account_issuer: String,
    pub secret_key: String,
    pub can_edit: bool,
    pub brand_icon: String,
    pub simple_icon: String,
    pub brand_icon_similarity: f64,
    pub simple_icon_similarity: f64,
    pub shared_by: Owner,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OneTimePasswords {
    pub my_one_time_passwords: Vec<OneTimePassword>,
    pub shared_one_time_passwords: Vec<SharedOneTimePassword>,
}

pub async fn get_one_time_passwords() -> Result<OneTimePasswords, reqwest::Error> {
    let config = load_config();

    let client = reqwest::Client::new();
    let res = client
        .get(format!("{}/api/one-time-password", config.host))
        .bearer_auth(config.token)
        .send()
        .await?;

    let one_time_passwords = res.json().await?;
    Ok(one_time_passwords)
}

pub async fn delete_one_time_password(id: i64) -> Result<(), reqwest::Error> {
    let config = load_config();
    let client = reqwest::Client::new();
    let res = client
        .delete(format!("{}/api/one-time-password/{}", config.host, id))
        .bearer_auth(config.token)
        .send()
        .await?;
    res.error_for_status()?;
    Ok(())
}

pub async fn share_one_time_password(id: i64, share_with: Vec<i64>) -> Result<(), reqwest::Error> {
    let config = load_config();
    let client = reqwest::Client::new();
    let payload = HashMap::from([("sharedWith".to_string(), share_with)]);
    let res = client
        .post(format!(
            "{}/api/one-time-password/{}/share",
            config.host, id
        ))
        .bearer_auth(config.token)
        .json(&payload)
        .send()
        .await?;
    res.error_for_status()?;
    Ok(())
}

pub async fn update_one_time_password(id: i64, account_name: String) -> Result<(), reqwest::Error> {
    let config = load_config();
    let client = reqwest::Client::new();
    let payload = HashMap::from([("accountName".to_string(), account_name)]);
    let res = client
        .put(format!("{}/api/one-time-password/{}", config.host, id))
        .bearer_auth(config.token)
        .json(&payload)
        .send()
        .await?;
    res.error_for_status()?;
    Ok(())
}
