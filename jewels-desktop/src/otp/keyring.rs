use crate::otp::OneTimePasswords;
use secret_service::{EncryptionType, SecretService};
use std::collections::HashMap;

const OTP_ATTRIBUTES: [(&str, &str); 4] = [
    ("server", "Jewels"),
    ("type", "plaintext"),
    ("user", "jewels-user"),
    ("xdg:schema", "org.qt.keychain"),
];

pub async fn get_cached_one_time_passwords() -> Result<OneTimePasswords, std::io::Error> {
    let ss = SecretService::connect(EncryptionType::Dh)
        .await
        .map_err(std::io::Error::other)?;
    let collection = ss
        .get_default_collection()
        .await
        .map_err(std::io::Error::other)?;

    if let Some(item) = collection
        .search_items(HashMap::from(OTP_ATTRIBUTES))
        .await
        .map_err(std::io::Error::other)?
        .first()
    {
        serde_json::from_slice(
            item.get_secret()
                .await
                .map_err(std::io::Error::other)?
                .as_slice(),
        )
        .map_err(std::io::Error::other)
    } else {
        Ok(OneTimePasswords::default())
    }
}

pub async fn store_cached_one_time_passwords(
    passwords: OneTimePasswords,
) -> Result<(), std::io::Error> {
    let json = serde_json::to_vec(&passwords)?;
    let ss = SecretService::connect(EncryptionType::Dh)
        .await
        .map_err(std::io::Error::other)?;
    let collection = ss
        .get_default_collection()
        .await
        .map_err(std::io::Error::other)?;

    let attributes = HashMap::from(OTP_ATTRIBUTES);
    collection
        .create_item(
            "Jewels One Time Passwords",
            HashMap::from(attributes),
            &json,
            true,
            "text/plain",
        )
        .await
        .map_err(std::io::Error::other)?;

    Ok(())
}
