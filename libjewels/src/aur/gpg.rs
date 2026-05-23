use crate::aur::AurEvent;
use crate::aur::consts::JEWELS_USER;
use srcinfo::Srcinfo;
use std::process::Stdio;
use tokio::process::Command;
use tokio::sync::mpsc;

pub async fn import_gpg_keys(package: &str, info: Srcinfo, tx: &mpsc::Sender<AurEvent>) {
    log::info!("Importing GPG keys for {}...", package);
    let keys = info
        .base
        .valid_pgp_keys
        .into_iter()
        .collect::<Vec<String>>();

    for key_id in keys {
        if gpg_key_known(&key_id).await {
            continue;
        }
        
        log::info!("Importing GPG key {}...", key_id);

        let result = Command::new("runuser")
            .args(["-u", JEWELS_USER, "--", "gpg", "--recv-keys", &key_id])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .await;

        match result {
            Ok(s) if !s.success() => {
                let _ = tx
                    .send(AurEvent::PackageError {
                        package: package.to_string(),
                        reason: format!("gpg exited {}", s.code().unwrap_or(-1)),
                    })
                    .await;
            }
            Err(e) => {
                let _ = tx
                    .send(AurEvent::PackageError {
                        package: package.to_string(),
                        reason: e.to_string(),
                    })
                    .await;
            }
            _ => {}
        }
    }
}

async fn gpg_key_known(key_id: &str) -> bool {
    Command::new("runuser")
        .args(["-u", JEWELS_USER, "--", "gpg", "--list-keys", key_id])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .await
        .map(|s| s.success())
        .unwrap_or(false)
}
