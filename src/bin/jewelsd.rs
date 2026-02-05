#[tokio::main]
async fn main() -> std::io::Result<()> {
    jewelsd::start_jewelsd().await
}
