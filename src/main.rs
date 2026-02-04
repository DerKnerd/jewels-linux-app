use commands::wireguard::update_wg_config;
use clap::{Parser, Subcommand};
use cstr::cstr;
use models::jewels::Jewels;
use qmetaobject::prelude::*;
use crate::commands::collection::run_collection;
use crate::commands::updater::run_package_update;
use crate::models::login::Login;

pub const UPDATE_SOCKET_DIR: &str = "/tmp/jewels/";
pub const UPDATE_SOCKET_FILE: &str = "update.sock";

mod alpm;
pub mod collector;
pub mod models;
mod qt;
mod commands;
mod authentication;

qrc!(pages,
    "cloud/ulbricht/jewels" {
        "qml/ui/MainApp.qml",
        "qml/ui/MainPage.qml",
        "qml/ui/pages/JewelsPage.qml",
        "qml/ui/pages/LoginPage.qml",
        "qml/ui/pages/UpdatesPage.qml",
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

    qml_register_type::<Jewels>(
        cstr!("cloud.ulbricht.jewels"),
        1,
        0,
        cstr!("Jewels"),
    );
    qml_register_type::<Login>(
        cstr!("cloud.ulbricht.jewels"),
        1,
        0,
        cstr!("Login"),
    );

    let mut engine = QmlEngine::new();
    engine.load_file("qrc:/cloud/ulbricht/jewels/qml/ui/MainApp.qml".into());

    engine.exec();
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let cli = Cli::parse();
    match cli.command {
        None => run_app(),
        Some(Commands::Collect) => run_collection().await,
        Some(Commands::Update) => run_package_update().await,
        Some(Commands::Wireguard) => update_wg_config().await,
    }
    Ok(())
}
