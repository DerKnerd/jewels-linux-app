use libjewels::collector::send_device_data;
use qmetaobject::prelude::*;

#[allow(non_snake_case)]
#[derive(QObject, Default)]
pub struct Jewels {
    base: qt_base_class!(trait QObject),
    sendData: qt_method!(
        fn sendData(&self) {
            self.send_data();
        }
    ),
}

impl Jewels {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn send_data(&self) {
        tokio::spawn(async move {
            send_device_data().await
        });
    }
}
