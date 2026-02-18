use libjewels::configuration::load_config;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Device {
    pub id: String,
    #[serde(rename = "type")]
    pub device_type: String,
    pub model: String,
    pub manufacturer: String,
    pub storage: f64,
    pub ram: f64,
    pub cpu: Cpu,
    pub os: Os,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eol: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cpu {
    pub model: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Os {
    pub version: Option<String>,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EolDevice {
    pub id: String,
    #[serde(rename = "type")]
    pub device_type: String,
    pub hostname: String,
    pub model: String,
    pub manufacturer: String,
    pub storage: f64,
    pub ram: f64,
    pub eol: Option<String>,
}

pub type EolDeviceRegistry = HashMap<String, Vec<EolDevice>>;

pub async fn get_devices() -> Result<Vec<Device>, reqwest::Error> {
    let config = load_config();

    let client = reqwest::Client::new();
    let res = client
        .get(format!("{}/api/my-jewels", config.host))
        .bearer_auth(config.token)
        .send()
        .await?;

    let devices = res.json().await?;
    Ok(devices)
}

pub async fn get_eol_devices() -> Result<EolDeviceRegistry, reqwest::Error> {
    let config = load_config();

    let client = reqwest::Client::new();
    let res = client
        .get(format!("{}/api/eol", config.host))
        .bearer_auth(config.token)
        .send()
        .await?;

    res.json().await
}
