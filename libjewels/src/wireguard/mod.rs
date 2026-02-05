use crate::configuration::JewelsConfiguration;
use machine_uid::machine_id::get_machine_id;
use reqwest::Method;

async fn get_wg_config(config: &JewelsConfiguration) -> std::io::Result<String> {
    let client = reqwest::Client::new();
    let url = format!(
        "{}/api/relay-vpn/device/{}",
        config.host.clone(),
        get_machine_id().unwrap()
    );

    log::info!("Downloading Wireguard config from  {url}");
    let req = client
        .request(Method::GET, url)
        .bearer_auth(config.token.clone());

    let res = req.send().await.map_err(std::io::Error::other)?;
    if res.status() != 200 {
        log::error!("Failed to download Wireguard config");
        log::error!("Status: {}", res.status());
        Err(std::io::Error::other(format!(
            "Failed to download Wireguard config: {}",
            res.status()
        )))
    } else {
        res.text().await.map_err(std::io::Error::other)
    }
}

async fn get_wg_config_path() -> String {
    let name = get_wg_config_name().await;

    #[cfg(not(debug_assertions))]
    return format!("/etc/wireguard/{name}.conf");
    #[cfg(debug_assertions)]
    return format!("./etc/wireguard/{name}.conf");
}

async fn get_wg_config_name() -> String {
    if tokio::fs::try_exists("/etc/wireguard/VPN.conf")
        .await
        .is_ok_and(|res| res)
    {
        "VPN"
    } else {
        "vpn"
    }.to_string()
}

pub async fn update_wg_config(config: &JewelsConfiguration) -> std::io::Result<()> {
    log::info!("Writing Wireguard config");
    let wg_conf = get_wg_config(config).await?;
    tokio::fs::write(get_wg_config_path().await, wg_conf)
        .await
        .map_err(std::io::Error::other)?;

    restart_wg_quick().await.map_err(std::io::Error::other)
}

async fn restart_wg_quick() -> std::io::Result<()> {
    log::info!("Restart Wireguard interface");
    let systemctl =
        zbus_systemd::systemd1::ManagerProxy::builder(&zbus::Connection::system().await.unwrap())
            .build()
            .await
            .map_err(std::io::Error::other)?;
    systemctl
        .restart_unit(
            format!("wg-quick@{}.service", get_wg_config_name().await),
            "replace".to_string(),
        )
        .await
        .map(|_| ())
        .map_err(std::io::Error::other)
}
