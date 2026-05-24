use crate::models::{Clipboard, Jewels, Login, OneTimePasswords, Otp, Owners, Updates};
use cstr::cstr;
use libjewels::configuration::load_config;
use libjewels::dbus::{WireguardProxy, get_bus};
use qmetaobject::{QmlEngine, qml_register_singleton_instance, qml_register_type, qrc};

mod api;
pub mod authentication;
mod eol;
pub mod models;
pub mod qt;
pub mod updater;

qrc!(pages,
    "cloud/ulbricht/jewels" {
        "qml/ui/MainApp.qml",
        "qml/ui/MainPage.qml",
        "qml/ui/pages/JewelsPage.qml",
        "qml/ui/pages/LoginPage.qml",
        "qml/ui/pages/UpdatesPage.qml",
        "qml/ui/pages/TwoFactorPage.qml",
        "qml/ui/pages/TotpCard.qml",
        "icons/jewels.svg"
    }
);

pub fn register_qml_types() {
    qml_register_type::<Jewels>(cstr!("cloud.ulbricht.jewels"), 1, 0, cstr!("Jewels"));
    qml_register_type::<Login>(cstr!("cloud.ulbricht.jewels"), 1, 0, cstr!("Login"));
    qml_register_type::<Updates>(cstr!("cloud.ulbricht.jewels"), 1, 0, cstr!("Updates"));
    qml_register_type::<OneTimePasswords>(
        cstr!("cloud.ulbricht.jewels"),
        1,
        0,
        cstr!("OneTimePasswords"),
    );

    qml_register_singleton_instance(
        cstr!("cloud.ulbricht.jewels"),
        1,
        0,
        cstr!(Clipboard),
        Clipboard::default(),
    );
    qml_register_singleton_instance(
        cstr!("cloud.ulbricht.jewels"),
        1,
        0,
        cstr!(Otp),
        Otp::default(),
    );
    qml_register_singleton_instance(
        cstr!("cloud.ulbricht.jewels"),
        1,
        0,
        cstr!(Owners),
        Owners::default(),
    );
}

fn run_app() {
    qmetaobject::log::init_qt_to_rust();

    pages();

    qt::app::set_desktop_file("dev.imanuel.jewels");

    register_qml_types();

    let mut engine = QmlEngine::new();
    engine.load_file("qrc:/cloud/ulbricht/jewels/qml/ui/MainApp.qml".into());

    engine.exec();
}

pub async fn update_wireguard() -> zbus::Result<()> {
    let connection = get_bus().await?;
    let proxy = WireguardProxy::new(&connection).await?;
    proxy
        .update_wireguard(load_config())
        .await
        .map_err(Into::into)
}

pub async fn update_system() -> std::io::Result<()> {
    updater::update_system()
        .await
        .map_err(std::io::Error::other)
}

pub fn jewels_desktop() -> std::io::Result<()> {
    env_logger::init();
    tokio::spawn(async move {
        if let Err(err) = update_wireguard().await {
            log::error!("Failed to update Wireguard config: {err}. Will retry in 5 minutes");
            tokio::time::sleep(tokio::time::Duration::from_mins(5)).await;
            if let Err(err) = update_wireguard().await {
                log::error!("Failed to update Wireguard config again: {err}. Will not retry again");
            }
        }
    });
    run_app();

    Ok(())
}
