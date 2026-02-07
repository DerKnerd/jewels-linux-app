use crate::configuration::JewelsConfiguration;

#[derive(Debug, Default, Clone)]
pub struct Wireguard {}

#[zbus::interface(
    name = "cloud.ulbricht.jewels.Wireguard",
    proxy(
        default_service = "cloud.ulbricht.jewels.JewelsKit",
        default_path = "/cloud/ulbricht/jewels/Wireguard",
    )
)]
impl Wireguard {
    async fn update_wireguard(&self, jewels_config: JewelsConfiguration) -> zbus::fdo::Result<()> {
        crate::wireguard::update_wg_config(&jewels_config)
            .await
            .map_err(|err| {
                log::error!("Failed to update Wireguard config: {err:#?}");
                zbus::fdo::Error::Failed(err.to_string())
            })
    }
}
