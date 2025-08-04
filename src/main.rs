use crate::wireguard::update_wg_config;
use clap::{Parser, Subcommand};
use cstr::cstr;
use models::config::Config;
use models::jewels::Jewels;
use qmetaobject::prelude::*;
use qmetaobject::qml_register_singleton_instance;

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

fn run_collection() {
    log::info!("Starting collection");
    log::info!("Load config");
    let config = Config::new();
    let jewels = Jewels::new();

    log::info!("Send data to server");
    jewels.send_data(config.host, config.token)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let cli = Cli::parse();
    match cli.command {
        None => run_app(),
        Some(Commands::Collect) => run_collection(),
        Some(Commands::Wireguard) => update_wg_config().await,
    }
    Ok(())
}
