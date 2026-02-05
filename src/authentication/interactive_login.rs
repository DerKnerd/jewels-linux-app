use crate::models::config::{JewelsConfiguration, write_config};
use warp::Filter;

async fn handle_login(
    tx: tokio::sync::mpsc::Sender<bool>,
    data: JewelsConfiguration,
) -> Result<impl warp::Reply, warp::Rejection> {
    if write_config(data).is_ok() {
        let _ = tx.send(true).await;
        Ok(warp::reply::with_status(
            warp::reply(),
            warp::http::StatusCode::NO_CONTENT,
        ))
    } else {
        Ok(warp::reply::with_status(
            warp::reply(),
            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
        ))
    }
}

pub async fn start_listener() {
    let (tx, mut rx) = tokio::sync::mpsc::channel(1);
    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["content-type"])
        .allow_methods(vec!["POST"]);

    let login_route = warp::post()
        .and(warp::path::end())
        .and(warp::any().map(move || tx.clone()))
        .and(warp::body::json())
        .and_then(handle_login)
        .with(cors);

    open::that(format!(
        "{}/desktop-login",
        std::env::var("JEWELS_SERVER").unwrap_or("https://jewels.ulbricht.cloud".to_string()),
    ))
    .unwrap();

    warp::serve(login_route)
        .bind(([127, 0, 0, 1], 10523))
        .await
        .graceful(async move {
            let _ = rx.recv().await;
        })
        .run()
        .await;
}
