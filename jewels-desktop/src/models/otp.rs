use qmetaobject::{QObject, QString, qt_base_class, qt_method};
use std::time::{SystemTime, UNIX_EPOCH};
use totp_rs::{Algorithm, Secret, TOTP};

#[allow(non_snake_case)]
#[derive(QObject, Default)]
pub struct Otp {
    base: qt_base_class!(trait QObject),

    pub generate: qt_method!(
        fn generate(&self, secret_key: QString) -> QString {
            match self.generate_otp_code(secret_key.to_string()) {
                Ok(otp_code) => otp_code,
                Err(err) => {
                    log::error!("Error generating totp: {err}");
                    "Fehler".into()
                }
            }
        }
    ),
    pub timeStep: qt_method!(
        fn timeStep(&self) -> u64 {
            let step = self.get_time_step();
            log::info!("Time step: {step}");
            step
        }
    ),
}

impl Otp {
    fn generate_otp_code(&self, secret_key: String) -> anyhow::Result<QString> {
        let totp = TOTP::new_unchecked(
            Algorithm::SHA1,
            6,
            0,
            30,
            Secret::Encoded(secret_key).to_bytes()?,
        );
        Ok(totp.generate_current()?.to_string().into())
    }

    fn get_time_step(&self) -> u64 {
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        time % 30
    }
}
