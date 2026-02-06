use libjewels::dbus::{Pacman, Wireguard, get_builder, get_bus};
use std::future::pending;

pub async fn start_jewelsd() -> std::io::Result<()> {
    env_logger::init();
    log::info!("Starting jewelsd");

    let conn = get_builder()
        .await
        .map_err(std::io::Error::other)?
        .name("cloud.ulbricht.jewels.JewelsKit")
        .map_err(std::io::Error::other)?
        .serve_at(
            "/cloud/ulbricht/jewels/Wireguard",
            Wireguard::default(),
        )
        .map_err(std::io::Error::other)?
        .serve_at(
            "/cloud/ulbricht/jewels/Pacman",
            Pacman::new(get_bus().await.map_err(std::io::Error::other)?),
        )
        .map_err(std::io::Error::other)?
        .build()
        .await;
    if let Err(err) = conn {
        log::error!("Failed to start jewelsd: {err:#?}");
        return Err(std::io::Error::other(err));
    }

    log::info!("Listening for D-Bus requests...");
    pending::<()>().await;

    Ok(())
}
