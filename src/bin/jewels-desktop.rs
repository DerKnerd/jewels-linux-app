use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Update,
    Wireguard,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        None => jewels_desktop::jewels_desktop(),
        Some(Commands::Update) => jewels_desktop::update_system()
            .await
            .map_err(std::io::Error::other),
        Some(Commands::Wireguard) => jewels_desktop::update_wireguard()
            .await
            .map_err(std::io::Error::other),
    }
}
