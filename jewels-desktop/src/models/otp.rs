use std::time::{SystemTime, UNIX_EPOCH};
use cxx_qt_lib::QString;
use totp_rs::{Algorithm, Secret, TOTP};

#[cxx_qt::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    #[auto_cxx_name]
    #[auto_rust_name]
    unsafe extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qml_singleton]
        type Otp = super::OtpStruct;

        #[qinvokable]
        fn generate(&self, secret_key: QString) -> QString;

        #[qinvokable]
        fn time_step(&self) -> u64;
    }
}

#[derive(Default)]
pub struct OtpStruct {}

impl ffi::Otp {
    fn generate_code(&self, secret_key: String) -> anyhow::Result<String> {
        let totp = TOTP::new_unchecked(
            Algorithm::SHA1,
            6,
            0,
            30,
            Secret::Encoded(secret_key.to_string()).to_bytes()?,
        );
        Ok(totp.generate_current()?.to_string())
    }

    fn generate(&self, secret_key: QString) -> QString {
        if let Ok(code) = self.generate_code(secret_key.to_string()) {
            code.into()
        } else {
            "Fehler".into()
        }
    }

    fn time_step(&self) -> u64 {
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        time % 30
    }
}
