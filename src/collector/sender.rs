use crate::collector::detector::collect_device_info;
use anyhow::anyhow;
use notify_rust::{Hint, Notification, Urgency};

async fn send_device_data_impl(host: &str, token: &str) -> anyhow::Result<()> {
    log::info!("Collecting device info");
    let device = collect_device_info();
    let client = reqwest::Client::new();

    log::info!("Sending device info");
    let res = client
        .post(format!("{host}/api/device/computer"))
        .bearer_auth(token)
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

pub async fn send_device_data(host: &str, token: &str) {
    if let Err(err) = send_device_data_impl(host, token).await {
        log::error!("{err}");
        let _ = Notification::new()
            .summary("Das hat nicht geklappt")
            .body("Hm, beim Senden der Informationen deines Rechners ist ein Fehler aufgetreten, bitte meld dich beim Support.")
            .icon("jewels")
            .hint(Hint::Urgency(Urgency::Critical))
            .timeout(0)
            .show();
    } else {
        let _ = Notification::new()
            .summary("Die Daten wurden gesendet")
            .body("Super, der Jewels Server hat jetzt alle wichtigen Informationen zu deinem Rechner.")
            .icon("jewels")
            .show();
    }
}
