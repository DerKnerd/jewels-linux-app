use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QQuickStyle, QString, QUrl};
use cxx_qt_lib_extras::QApplication;
use libjewels::configuration::load_config;
use libjewels::dbus::{WireguardProxy, get_bus};

mod api;
pub mod authentication;
mod eol;
pub mod models;
pub mod updater;

extern crate cxx;

fn run_app() {
    let mut app = QApplication::new();

    QGuiApplication::set_desktop_file_name(&QString::from("dev.imanuel.jewels"));

    if std::env::var("QT_QUICK_CONTROLS_STYLE").is_err() {
        QQuickStyle::set_style(&QString::from("org.kde.desktop"));
    }

    let mut engine = QQmlApplicationEngine::new();
    if let Some(engine) = engine.as_mut() {
        engine.load(&QUrl::from(
            "qrc:/qt/qml/cloud/ulbricht/jewels/qml/ui/MainApp.qml",
        ));
    }

    if let Some(app) = app.as_mut() {
        app.exec();
    }
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
