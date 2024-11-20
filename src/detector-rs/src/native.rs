use crate::collector::sender::send_device_data;

#[cxx::bridge]
pub mod ffi {
    extern "Rust" {
        #[cxx_name = "sendData"]
        fn send_data(host: &str, token: &str);
    }
}

fn send_data(host: &str, token: &str) {
    send_device_data(host, token);
}
