use libjewels::dbus::{Wireguard, get_builder};
use std::future::pending;

pub async fn start_jewelsd() -> std::io::Result<()> {
    env_logger::init();
    let conn = get_builder()
        .await
        .map_err(std::io::Error::other)?
        .name("cloud.ulbricht.jewels.Wireguard")
        .map_err(std::io::Error::other)?
        .serve_at("/cloud/ulbricht/jewels/Wireguard", Wireguard::new())
        .map_err(std::io::Error::other)?
        .build()
        .await;
    if let Err(err) = conn {
        log::error!("Failed to start JewelsD: {err:#?}");
        return Err(std::io::Error::other(err));
    }

    pending::<()>().await;

    Ok(())
}
