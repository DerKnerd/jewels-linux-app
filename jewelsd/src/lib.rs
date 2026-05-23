use libjewels::dbus::{Wireguard, get_builder, pacman::Pacman};
use std::future::pending;
use libjewels::dbus::aur::Aur;

pub async fn start_jewelsd() -> std::io::Result<()> {
    env_logger::init();
    log::info!("Starting jewelsd");

    let conn = get_builder()
        .await
        .map_err(std::io::Error::other)?
        .name("cloud.ulbricht.jewels.JewelsKit")
        .map_err(std::io::Error::other)?
        .serve_at("/cloud/ulbricht/jewels/Wireguard", Wireguard::default())
        .map_err(std::io::Error::other)?
        .serve_at("/cloud/ulbricht/jewels/Pacman", Pacman::default())
        .map_err(std::io::Error::other)?
        .serve_at("/cloud/ulbricht/jewels/Aur", Aur::default())
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
