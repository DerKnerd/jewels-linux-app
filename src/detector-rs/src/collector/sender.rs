use crate::collector::detector::collect_device_info;

pub fn send_device_data(host: &str, token: &str) {
    let device = collect_device_info();
    let client = reqwest::blocking::Client::new();
    let res = client
        .post(format!("{host}/api/device/computer"))
        .bearer_auth(token)
        .json(&device)
        .send()
        .map_err(|err| err.to_string());
    if let Ok(response) = res {
        if response.status() != reqwest::StatusCode::NO_CONTENT {
            eprintln!("{} ", response.status());
            eprintln!("{} ", response.text().unwrap_or_default());
        }
    } else if let Err(err) = res {
        eprintln!("{err}");
    }
}
