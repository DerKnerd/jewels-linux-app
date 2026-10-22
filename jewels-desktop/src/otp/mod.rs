mod database;

pub use crate::api::otp as api;
pub use crate::api::otp::MyOneTimePassword;
pub use crate::api::otp::OneTimePasswords;
pub use crate::api::otp::SharedOneTimePassword;
pub use crate::api::otp::delete_one_time_password;
pub use crate::api::otp::share_one_time_password;
pub use crate::api::otp::update_one_time_password;

pub use crate::otp::database::open_otp_cache;

pub async fn get_one_time_passwords() -> Result<OneTimePasswords, std::io::Error> {
    if let Ok(otps) = api::get_one_time_passwords().await {
        if let Err(err) = database::store_cached_one_time_passwords(otps.clone()) {
            log::warn!("Failed to store cached one-time passwords: {}", err);
        }
        Ok(otps)
    } else {
        database::get_cached_one_time_passwords()
    }
}
