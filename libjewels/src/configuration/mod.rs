use configparser::ini::Ini;
use serde::{Deserialize, Serialize};
use zbus::zvariant::Type;

const CONFIG_FILE_NAME: &str = "jewelsrc";

#[derive(Debug, Default, Clone, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize, Type)]
pub struct JewelsConfiguration {
    pub host: String,
    pub token: String,
}

fn get_config_path() -> String {
    let path = xdg::BaseDirectories::default().place_config_file(CONFIG_FILE_NAME);
    if let Ok(path) = path {
        path.display().to_string()
    } else {
        String::from("")
    }
}

pub fn write_config(config: JewelsConfiguration) -> std::io::Result<()> {
    let mut ini = Ini::new();
    ini.set("Server", "host", Some(config.host));
    ini.set("Server", "token", Some(config.token));
    ini.write(get_config_path())
}

pub fn load_config() -> JewelsConfiguration {
    let mut ini = Ini::new();
    ini.load(get_config_path()).unwrap_or_default();
    JewelsConfiguration {
        host: ini.get("Server", "host").unwrap_or_default(),
        token: ini.get("Server", "token").unwrap_or_default(),
    }
}
