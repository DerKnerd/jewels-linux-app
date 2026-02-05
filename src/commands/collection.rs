use crate::collector::sender::send_device_data;
use crate::models::config::load_config;

pub async fn run_collection() {
    log::info!("Starting collection");
    log::info!("Load config");
    let config = load_config();

    log::info!("Send data to server");
    send_device_data(
        config.host.to_string().as_str(),
        config.token.to_string().as_str(),
    )
        .await;
}
