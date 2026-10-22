use crate::otp::api::OneTimePasswords;
use microxdg::Xdg;
use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use std::path::Path;

pub fn open_otp_cache() -> Result<PickleDb, std::io::Error> {
    let xdg = Xdg::new().map_err(std::io::Error::other)?;
    let path = Path::new(xdg.state().map_err(std::io::Error::other)?.as_path())
        .join("jewels_otp_cache.db");

    if std::fs::exists(&path).is_ok_and(|exists| exists) {
        Ok(PickleDb::load(
            path,
            PickleDbDumpPolicy::AutoDump,
            SerializationMethod::Cbor,
        )
        .map_err(std::io::Error::other)?)
    } else {
        Ok(PickleDb::new(
            path,
            PickleDbDumpPolicy::AutoDump,
            SerializationMethod::Cbor,
        ))
    }
}

pub fn get_cached_one_time_passwords() -> Result<OneTimePasswords, std::io::Error> {
    let passwords = open_otp_cache()?
        .get::<OneTimePasswords>("one_time_passwords")
        .unwrap_or_default();
    Ok(passwords)
}

pub fn store_cached_one_time_passwords(passwords: OneTimePasswords) -> Result<(), std::io::Error> {
    open_otp_cache()?
        .set("one_time_passwords", &passwords)
        .map_err(std::io::Error::other)
}
