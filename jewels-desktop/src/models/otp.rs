use cxx_qt_lib::QString;
use std::time::{SystemTime, UNIX_EPOCH};
use totp_rs::{Algorithm, Secret};

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
        let totp = totp_rs::Builder::new()
            .with_algorithm(Algorithm::SHA1)
            .with_digits(6)
            .with_skew(0)
            .with_step_duration(30)
            .with_secret(Secret::try_from_base32(secret_key)?)
            .build()?;
        Ok(totp.generate_current().to_string())
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
