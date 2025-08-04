use crate::collector::sender::send_device_data;
use crate::wireguard::update_wg_config;
use clap::{Parser, Subcommand};
use cstr::cstr;
use models::config::Config;
use models::jewels::Jewels;
use qmetaobject::prelude::*;
use qmetaobject::qml_register_singleton_instance;
use tokio::io::AsyncWriteExt;
use tokio::net::UnixListener;

pub const UPDATE_SOCKET_DIR: &str = "/tmp/jewels/";
pub const UPDATE_SOCKET_FILE: &str = "update.sock";

mod alpm;
pub mod collector;
pub mod models;
mod qt;
mod wireguard;

qrc!(pages,
    "cloud/ulbricht/jewels" {
        "qml/ui/main.qml",
        "qml/ui/pages/jewels.qml",
        "icons/jewels.svg"
    }
);

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Collect,
    Update,
    Wireguard,
}

fn run_app() {
    qmetaobject::log::init_qt_to_rust();

    pages();

    qt::app::set_desktop_file("dev.imanuel.jewels");

    qml_register_singleton_instance(
        cstr!("cloud.ulbricht.jewels"),
        1,
        0,
        cstr!("Jewels"),
        Jewels::new(),
    );
    qml_register_singleton_instance(
        cstr!("cloud.ulbricht.jewels"),
        1,
        0,
        cstr!("Config"),
        Config::new(),
    );

    let mut engine = QmlEngine::new();
    engine.load_file("qrc:/cloud/ulbricht/jewels/qml/ui/main.qml".into());

    engine.exec();
}

async fn run_collection() {
    log::info!("Starting collection");
    log::info!("Load config");
    let config = Config::new();

    log::info!("Send data to server");
    send_device_data(
        config.host.to_string().as_str(),
        config.token.to_string().as_str(),
    )
    .await;
}

#[cfg(feature = "systemd")]
async fn get_listener() -> std::io::Result<UnixListener> {
    use listenfd::ListenFd;
    let mut listenfd = ListenFd::from_env();
    if let Ok(Some(listener)) = listenfd.take_unix_listener(0) {
        listener.set_nonblocking(true)?;
        UnixListener::from_std(listener)
    } else {
        Err(std::io::Error::other(
            "Needs to be launched from socket activation",
        ))
    }
}

#[cfg(not(feature = "systemd"))]
async fn get_listener() -> std::io::Result<UnixListener> {
    use std::fs::Permissions;
    use std::os::unix::fs::PermissionsExt;

    tokio::fs::create_dir_all(UPDATE_SOCKET_DIR).await?;
    let path = std::path::Path::new(UPDATE_SOCKET_DIR).join(UPDATE_SOCKET_FILE);

    tokio::fs::remove_file(path.clone()).await?;

    let listener = UnixListener::bind(path.clone())?;

    tokio::fs::set_permissions(path, Permissions::from_mode(0o777)).await?;

    Ok(listener)
}

async fn run_package_update() {
    log::info!("Starting package update");
    match get_listener().await {
        Ok(listener) => {
            while let Ok((mut socket, ..)) = listener.accept().await {
                match alpm::update_system() {
                    Ok(_) => {
                        log::info!("Update finished");
                        let _ = socket.write(b"OK").await;
                    }
                    Err(err) => {
                        log::error!("Update failed: {err}");
                        let _ = socket.write(b"Error\n").await;
                        let _ = socket.write(format!("{err}").as_bytes()).await;
                    }
                }
                socket.shutdown().await.unwrap();
            }
        }
        Err(err) => {
            log::error!("Failed to bind socket: {err}");
        }
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let cli = Cli::parse();
    match cli.command {
        None => run_app(),
        Some(Commands::Collect) => {
            run_collection().await;
            run_app()
        }
        Some(Commands::Update) => run_package_update().await,
        Some(Commands::Wireguard) => update_wg_config().await,
    }
    Ok(())
}
