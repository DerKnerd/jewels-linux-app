use crate::collector::detector::collect_device_info;

pub fn send_device_data(host: &str, token: &str) {
    log::info!("Collecting device info");
    let device = collect_device_info();
    let client = reqwest::blocking::Client::new();

    log::info!("Sending device info");
    let res = client
        .post(format!("{host}/api/device/computer"))
        .bearer_auth(token)
        .json(&device)
        .send()
        .map_err(|err| err.to_string());

    if let Ok(response) = res {
        log::info!("Response: {:?}", response.status());
        if response.status() != reqwest::StatusCode::NO_CONTENT {
            log::error!("{}", response.status());
            log::error!("{}", response.text().unwrap_or_default());
        }
    } else if let Err(err) = res {
        log::error!("{err}");
    }
}
