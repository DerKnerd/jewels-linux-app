use tokio::io::AsyncWriteExt;
use tokio::net::UnixListener;

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
    use crate::{UPDATE_SOCKET_DIR, UPDATE_SOCKET_FILE};

    tokio::fs::create_dir_all(UPDATE_SOCKET_DIR).await?;
    let path = std::path::Path::new(UPDATE_SOCKET_DIR).join(UPDATE_SOCKET_FILE);

    tokio::fs::remove_file(path.clone()).await?;

    let listener = UnixListener::bind(path.clone())?;

    tokio::fs::set_permissions(path, Permissions::from_mode(0o777)).await?;

    Ok(listener)
}

pub async fn run_package_update() {
    log::info!("Starting package update");
    match get_listener().await {
        Ok(listener) => {
            while let Ok((mut socket, ..)) = listener.accept().await {
                match crate::alpm::update_system() {
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
