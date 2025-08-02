use crate::collector::sender::send_device_data;
use qmetaobject::prelude::*;

#[allow(non_snake_case)]
#[derive(QObject, Default)]
pub struct Jewels {
    base: qt_base_class!(trait QObject),
    sendData: qt_method!(
        fn sendData(&self, host: QString, token: QString) {
            self.send_data(host, token);
        }
    ),
}

impl Jewels {
    pub fn new() -> Self {
        Self::default()
    }

    fn send_data(&self, host: QString, token: QString) {
        let host = host.clone();
        let token = token.clone();

        std::thread::spawn(move || {
            send_device_data(host.to_string().as_str(), token.to_string().as_str());
        });
    }
}
