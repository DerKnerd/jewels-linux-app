mod pacman;
mod wireguard;

pub use pacman::*;

pub use wireguard::*;

pub async fn get_bus() -> zbus::Result<zbus::Connection> {
    get_builder().await?.build().await
}

pub async fn get_builder<'a>() -> zbus::Result<zbus::connection::Builder<'a>> {
    zbus::connection::Builder::system()
}
