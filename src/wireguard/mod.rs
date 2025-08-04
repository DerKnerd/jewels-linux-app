use crate::models::config::Config;
use machine_uid::machine_id::get_machine_id;
use reqwest::Method;
use zbus_systemd::zbus;

async fn get_wg_config() -> anyhow::Result<String> {
    let config = Config::new();
    let client = reqwest::Client::new();
    let url = format!(
        "{}/api/relay-vpn/device/{}",
        config.host,
        get_machine_id().unwrap()
    );
    log::info!("Downloading Wireguard config from  {url}");
    let req = client
        .request(Method::GET, url)
        .bearer_auth(config.token.clone());

    let res = req.send().await?;
    if res.status() != 200 {
        log::error!("Failed to download Wireguard config");
        log::error!("Status: {}", res.status());
        Err(anyhow::anyhow!(format!("Status: {}", res.status())))
    } else {
        res.text().await.map_err(|e| anyhow::anyhow!(e))
    }
}

async fn restart_wg_quick() -> zbus::Result<()> {
    log::info!("Restart Wireguard interface");
    let systemctl =
        zbus_systemd::systemd1::ManagerProxy::builder(&zbus::Connection::system().await.unwrap())
            .build()
            .await?;
    systemctl
        .restart_unit("wg-quick@vpn.service".to_string(), "replace".to_string())
        .await
        .map(|_| ())
}

pub async fn update_wg_config() {
    log::info!("Writing Wireguard config");
    let wg_conf = get_wg_config().await;
    let config_written = match wg_conf {
        Ok(wg_conf) => tokio::fs::write("vpn.conf", wg_conf).await,
        Err(err) => Err(std::io::Error::other(err)),
    };
    if let Err(err) = config_written {
        log::error!("Failed to write Wireguard config: {err}");
        return;
    }

    if let Err(err) = restart_wg_quick().await {
        log::error!("Failed to restart Wireguard interface: {err}");
    }
}
