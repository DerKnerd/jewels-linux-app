use cstr::cstr;
use libjewels::configuration::load_config;
use libjewels::dbus::{WireguardProxy, get_bus};
use qmetaobject::prelude::*;
use qmetaobject::qml_register_singleton_instance;
use crate::models::{Jewels, Login, Updates, OneTimePasswords, Clipboard, Otp, Owners};
use crate::qml_exports::qml_exports;

pub mod authentication;
pub mod models;
pub mod qt;
mod api;
pub mod qml_exports;

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
    macro_rules! do_register {
        (
            module_uri: $uri:literal,
            major: $maj:literal,
            minor: $min:literal,
            types: [ $( ($ty:ty, $name:literal) ),* $(,)? ],
            singletons: [ $( ($sty:ty, $sname:literal) ),* $(,)? ],
        ) => {
            $(
                qml_register_type::<$ty>(cstr!($uri), $maj, $min, cstr!($name));
            )*
            $(
                qml_register_singleton_instance(
                    cstr!($uri),
                    $maj,
                    $min,
                    cstr!($sname),
                    <$sty>::default(),
                );
            )*
        }
    }

    qml_exports!(do_register);
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

async fn update_wireguard() -> zbus::Result<()> {
    let connection = get_bus().await?;
    let proxy = WireguardProxy::new(&connection).await?;
    proxy
        .update_wireguard(load_config())
        .await
        .map_err(Into::into)
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
