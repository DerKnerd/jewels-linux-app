use libjewels::dbus::{Wireguard, get_builder, pacman::Pacman};
use std::future::pending;
use libjewels::dbus::aur::Aur;

pub async fn start_jewelsd() -> std::io::Result<()> {
    env_logger::init();
    log::info!("Starting jewelsd");

    let conn = get_builder()
        .await
        .name("cloud.ulbricht.jewels.JewelsKit")
        .serve_at("/cloud/ulbricht/jewels/Wireguard", Wireguard::default())
        .serve_at("/cloud/ulbricht/jewels/Pacman", Pacman::default())
        .serve_at("/cloud/ulbricht/jewels/Aur", Aur::default())
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
