mod wireguard;

pub use wireguard::*;

pub async fn get_bus() -> zbus::Result<zbus::Connection> {
    get_builder().await?.build().await
}

#[cfg(not(debug_assertions))]
pub async fn get_builder<'a>() -> zbus::Result<zbus::connection::Builder<'a>> {
    zbus::connection::Builder::system()
}

#[cfg(debug_assertions)]
pub async fn get_builder<'a>() -> zbus::Result<zbus::connection::Builder<'a>> {
    zbus::connection::Builder::session()
}
