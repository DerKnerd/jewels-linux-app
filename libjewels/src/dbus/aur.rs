use crate::alpm::{DownloadProgress, LogMessage, UpdateProgress};
use crate::aur::{AurHelper, AurPackage};
use zbus::message::Header;
use zbus::object_server::SignalEmitter;

#[derive(Debug, Clone, Default)]
pub struct Aur {}

#[zbus::interface(
    name = "cloud.ulbricht.jewels.Aur",
    proxy(
        default_service = "cloud.ulbricht.jewels.JewelsKit",
        default_path = "/cloud/ulbricht/jewels/Aur",
    )
)]
impl Aur {
    pub async fn get_available_updates(
        &self,
        #[zbus(header)] hdr: Header<'_>,
        #[zbus(signal_emitter)] emitter: SignalEmitter<'_>,
    ) -> zbus::fdo::Result<Vec<AurPackage>> {
        let conn = emitter.connection();
        let (download_tx, mut download_rx) = tokio::sync::mpsc::channel(16);
        let (progress_tx, mut progress_rx) = tokio::sync::mpsc::channel(16);
        let (log_tx, mut log_rx) = tokio::sync::mpsc::channel(16);
        let (built_tx, _) = tokio::sync::mpsc::channel(16);
        let (failed_tx, _) = tokio::sync::mpsc::channel(16);
        let (done_tx, mut done_rx) = tokio::sync::mpsc::channel(16);

        let path = hdr.path().unwrap();
        let emitter = SignalEmitter::new(conn, path.to_string())?;
        let aur_helper = AurHelper::new(download_tx, progress_tx, log_tx, built_tx, failed_tx);
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    Some(msg) = log_rx.recv() => {
                        let _ = Aur::log(&emitter, msg).await;
                    }
                    Some(msg) = progress_rx.recv() => {
                        let _ = Aur::update(&emitter, msg).await;
                    }
                    Some(msg) = download_rx.recv() => {
                        let _ = Aur::download(&emitter, msg).await;
                    }
                    Some(_) = done_rx.recv() => {}
                    else => break
                }
            }
        });

        let res = aur_helper
            .get_upgradable_packages()
            .await
            .map_err(|err| zbus::fdo::Error::Failed(err.to_string()));
        let _ = done_tx.send(()).await;

        res
    }

    pub fn install_updates(
        &self,
        #[zbus(header)] hdr: Header<'_>,
        #[zbus(signal_emitter)] emitter: SignalEmitter<'_>,
    ) -> zbus::fdo::Result<()> {
        let conn = emitter.connection();
        let (download_tx, mut download_rx) = tokio::sync::mpsc::channel(16);
        let (progress_tx, mut progress_rx) = tokio::sync::mpsc::channel(16);
        let (log_tx, mut log_rx) = tokio::sync::mpsc::channel(16);
        let (built_tx, mut built_rx) = tokio::sync::mpsc::channel(16);
        let (failed_tx, mut failed_rx) = tokio::sync::mpsc::channel(16);
        let (done_tx, mut done_rx) = tokio::sync::mpsc::channel(16);

        let path = hdr.path().unwrap();
        let emitter = SignalEmitter::new(conn, path.to_string())?;
        let aur_helper = AurHelper::new(download_tx, progress_tx, log_tx, built_tx, failed_tx);
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    Some(msg) = log_rx.recv() => {
                        let _ = Aur::log(&emitter, msg).await;
                    }
                    Some(msg) = progress_rx.recv() => {
                        let _ = Aur::update(&emitter, msg).await;
                    }
                    Some(msg) = download_rx.recv() => {
                        let _ = Aur::download(&emitter, msg).await;
                    }
                    Some(msg) = built_rx.recv() => {
                        let _ = Aur::built(&emitter, msg).await;
                    }
                    Some(msg) = failed_rx.recv() => {
                        let _ = Aur::failed(&emitter, msg).await;
                    }
                    Some(_) = done_rx.recv() => {
                        break;
                    }
                    else => break
                }
            }
        });

        let emitter = SignalEmitter::new(conn, path.to_string())?;
        tokio::spawn(async move {
            if let Err(err) = aur_helper.upgrade_aur_packages().await {
                log::error!("Failed to update the packages {err}");
                let _ = Aur::failure(&emitter).await;
            } else {
                log::info!("Successfully updated the system");
                let _ = Aur::finished(&emitter).await;
            }
            let _ = done_tx.send(()).await;
        });
        Ok(())
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
    pub async fn finished(signal_emitter: &SignalEmitter<'_>) -> zbus::Result<()>;

    #[zbus(signal)]
    pub async fn failure(signal_emitter: &SignalEmitter<'_>) -> zbus::Result<()>;

    #[zbus(signal)]
    pub async fn built(signal_emitter: &SignalEmitter<'_>, package: String) -> zbus::Result<()>;

    #[zbus(signal)]
    pub async fn failed(signal_emitter: &SignalEmitter<'_>, package: String) -> zbus::Result<()>;
}
