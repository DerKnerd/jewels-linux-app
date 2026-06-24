use crate::api::device::get_eol_devices;
use notify_rust::{Notification, Urgency};
use std::path::PathBuf;

fn get_path(device_id: &str) -> PathBuf {
    xdg::BaseDirectories::with_prefix("jewels-desktop")
        .create_state_directory("eol_notified")
        .unwrap()
        .as_path()
        .join(device_id)
}

async fn has_been_notified(device_id: &str) -> bool {
    tokio::fs::read_to_string(get_path(device_id).as_path())
        .await
        .is_ok()
}

async fn mark_device_notified(device_id: &str) {
    tokio::fs::write(get_path(device_id).as_path(), "notified")
        .await
        .unwrap();
}

pub async fn eol_check() {
    match get_eol_devices().await {
        Ok(eol_devices) => {
            for (who, devices) in eol_devices {
                log::info!("{who} has {} devices that soon go eol", devices.len());
                for device in devices {
                    if has_been_notified(&device.id).await {
                        continue;
                    }

                    let (summary, body) = if who == "me" {
                        (
                            "Du brauchst bald was neues".to_string(),
                            format!(
                                "Dein {} {} ist bald aus dem Support raus",
                                device.manufacturer, device.model
                            ),
                        )
                    } else {
                        (
                            format!("{who} braucht bald was neues"),
                            format!(
                                "Das {} {} von {who} ist bald aus dem Support raus",
                                device.manufacturer, device.model
                            ),
                        )
                    };
                    tokio::task::spawn_blocking(move || {
                        let _ = Notification::new()
                            .summary(&summary)
                            .body(&body)
                            .appname("jewels")
                            .urgency(Urgency::Normal)
                            .icon("jewels")
                            .show();
                    });

                    mark_device_notified(&device.id).await;
                }
            }
        }
        Err(err) => {
            log::error!("Failed to get eol devices: {err}");
        }
    }
}
