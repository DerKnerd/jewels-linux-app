use crate::aur::consts::JEWELS_USER;
use srcinfo::Srcinfo;
use std::process::Stdio;
use tokio::process::Command;

pub async fn import_gpg_keys(package: &str, info: Srcinfo) -> std::io::Result<()> {
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
            .await?;
        if !result.success() {
            log::error!("Failed to import GPG key {}: {}", key_id, result);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to import GPG key",
            ));
        }
    }

    Ok(())
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
