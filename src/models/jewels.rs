use crate::collector::sender::send_device_data;
use crate::{UPDATE_SOCKET_DIR, UPDATE_SOCKET_FILE};
use notify_rust::{Hint, Notification, Urgency};
use qmetaobject::prelude::*;
use std::path::Path;
use tokio::io::AsyncReadExt;

#[allow(non_snake_case)]
#[derive(QObject, Default)]
pub struct Jewels {
    base: qt_base_class!(trait QObject),
    sendData: qt_method!(
        fn sendData(&self, host: QString, token: QString) {
            let host = host.to_string();
            let token = token.to_string();

            self.send_data(host, token);
        }
    ),
    updateSystem: qt_method!(
        fn updateSystem(&self) {
            self.update_system();
        }
    ),
}

impl Jewels {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn send_data(&self, host: String, token: String) {
        tokio::spawn(async move {
            send_device_data(host.to_string().as_str(), token.to_string().as_str()).await
        });
    }

    pub fn update_system(&self) {
        let _ = Notification::new()
            .summary("Updates werden installiert")
            .body("Lehn dich zurück und lass Jewels die Arbeit machen, du bekommst eine Benachrichtigung, wenn alle Updates installiert sind.")
            .icon("jewels")
            .show();
        tokio::spawn(async move {
            match tokio::net::UnixStream::connect(
                Path::new(UPDATE_SOCKET_DIR).join(UPDATE_SOCKET_FILE),
            )
            .await
            {
                Ok(mut stream) => {
                    let data = {
                        let mut data = vec![];
                        let segment = &mut [0u8; 1];
                        while stream.read_exact(segment).await.is_ok() {
                            if segment[0] == 0 {
                                break;
                            }
                            data.push(segment[0]);
                        }

                        data
                    };
                    let data = String::from_utf8_lossy(&data);
                    log::info!("Update data: {data}");
                    if data.contains("OK") {
                        let _ = Notification::new()
                            .summary("Updates sind fertig")
                            .body("Super, dein Rechner ist jetzt auf dem neuesten Stand.")
                            .icon("jewels")
                            .show();
                    } else {
                        let _ = Notification::new()
                            .summary("Das Update hat nicht geklappt")
                            .body("Hm, leider gab es beim Aktualisieren einen Fehler. Versuch es nochmal, falls das Problem weiterhin besteht, meld dich beim Support.")
                            .icon("jewels")
                            .hint(Hint::Urgency(Urgency::Critical))
                            .timeout(0)
                            .show();
                    }
                }
                Err(err) => {
                    log::error!("Error: {err}");
                }
            }
        });
    }
}
