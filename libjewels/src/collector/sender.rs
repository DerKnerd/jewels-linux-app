use crate::collector::detector::collect_device_info;
use anyhow::anyhow;

pub async fn send_device_data() -> anyhow::Result<()> {
    log::info!("Collecting device info");
    let device = collect_device_info();
    let client = reqwest::Client::new();

    let config = crate::configuration::load_config();

    log::info!("Sending device info");
    let res = client
        .post(format!("{}/api/device/computer", config.host))
        .bearer_auth(config.token)
        .json(&device)
        .send()
        .await?;

    log::info!("Response: {:?}", res.status());
    if res.status() != reqwest::StatusCode::NO_CONTENT {
        log::error!("{}", res.status());
        log::error!("{}", res.text().await.unwrap_or_default());
        Err(anyhow!("Error sending device info"))
    } else {
        log::info!("Device info sent");
        Ok(())
    }
}
