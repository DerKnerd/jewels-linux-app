use crate::alpm::{AlpmHelper, DownloadProgress, LogMessage, UpdatablePackage, UpdateProgress};
use std::sync::Arc;
use zbus::Connection;
use zbus::message::Header;
use zbus::object_server::SignalEmitter;

pub struct Pacman {
    conn: Arc<Connection>,
}

impl Pacman {
    pub fn new(connection: Connection) -> Self {
        Self {
            conn: Arc::new(connection),
        }
    }
}

#[zbus::interface(
    name = "cloud.ulbricht.jewels.Pacman",
    proxy(
        default_service = "cloud.ulbricht.jewels.JewelsKit",
        default_path = "/cloud/ulbricht/jewels/Pacman",
    )
)]
impl Pacman {
    pub async fn get_available_updates(
        &self,
        #[zbus(header)] hdr: Header<'_>,
    ) -> zbus::fdo::Result<Vec<UpdatablePackage>> {
        let conn = self.conn.clone();
        let (download_tx, mut download_rx) = tokio::sync::mpsc::channel(16);
        let (progress_tx, mut progress_rx) = tokio::sync::mpsc::channel(16);
        let (log_tx, mut log_rx) = tokio::sync::mpsc::channel(16);
        let path = hdr.path().unwrap();
        let emitter = SignalEmitter::new(&conn, path.to_string())?;
        let alpm_helper = AlpmHelper::new(download_tx, progress_tx, log_tx);
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    Some(msg) = log_rx.recv() => {
                        let _ = Pacman::log(&emitter, msg).await;
                    }
                    Some(msg) = progress_rx.recv() => {
                        let _ = Pacman::update(&emitter, msg).await;
                    }
                    Some(msg) = download_rx.recv() => {
                        let _ = Pacman::download(&emitter, msg).await;
                    }
                    else => break
                }
            }
        });

        alpm_helper
            .get_available_updates()
            .map_err(|err| zbus::fdo::Error::Failed(err.to_string()))
    }

    pub fn install_updates(&self, #[zbus(header)] hdr: Header<'_>) -> zbus::fdo::Result<()> {
        let conn = self.conn.clone();
        let (download_tx, mut download_rx) = tokio::sync::mpsc::channel(16);
        let (progress_tx, mut progress_rx) = tokio::sync::mpsc::channel(16);
        let (log_tx, mut log_rx) = tokio::sync::mpsc::channel(16);

        let path = hdr.path().unwrap();
        let emitter = SignalEmitter::new(&conn, path.to_string())?;
        let alpm_helper = AlpmHelper::new(download_tx, progress_tx, log_tx);
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    Some(msg) = log_rx.recv() => {
                        let _ = Pacman::log(&emitter, msg).await;
                    }
                    Some(msg) = progress_rx.recv() => {
                        let _ = Pacman::update(&emitter, msg).await;
                    }
                    Some(msg) = download_rx.recv() => {
                        let _ = Pacman::download(&emitter, msg).await;
                    }
                    else => break
                }
            }
        });

        alpm_helper
            .update_system()
            .map_err(|err| zbus::fdo::Error::Failed(err.to_string()))
    }

    #[zbus(signal)]
    pub async fn log(signal_emitter: &SignalEmitter<'_>, message: LogMessage) -> zbus::Result<()>;

    #[zbus(signal)]
    pub async fn download(
        signal_emitter: &SignalEmitter<'_>,
        progress: DownloadProgress,
    ) -> zbus::Result<()>;

    #[zbus(signal)]
    pub async fn update(
        signal_emitter: &SignalEmitter<'_>,
        progress: UpdateProgress,
    ) -> zbus::Result<()>;

    #[zbus(signal)]
    pub async fn packages(
        signal_emitter: &SignalEmitter<'_>,
        packages: Vec<String>,
    ) -> zbus::Result<()>;
}
