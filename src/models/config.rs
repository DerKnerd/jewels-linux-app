use configparser::ini::Ini;
use qmetaobject::prelude::*;

const CONFIG_FILE_NAME: &str = "jewelsrc";

#[derive(QObject, Default)]
pub struct Config {
    base: qt_base_class!(trait QObject),
    pub host: qt_property!(QString; NOTIFY host_changed),
    pub token: qt_property!(QString; NOTIFY token_changed),
    pub host_changed: qt_signal!(),
    pub token_changed: qt_signal!(),
    pub saved: qt_property!(bool; NOTIFY saved_changed),
    pub saved_changed: qt_signal!(),
    pub save: qt_method!(
        fn save(&mut self) {
            self.write_config();
        }
    ),
}

impl Config {
    pub fn new() -> Self {
        Self::load_config()
    }

    fn get_config_path() -> String {
        let path = xdg::BaseDirectories::default().place_config_file(CONFIG_FILE_NAME);
        if let Ok(path) = path {
            path.display().to_string()
        } else {
            String::from("")
        }
    }

    fn load_config() -> Self {
        let path = Self::get_config_path();
        if path.is_empty() {
            Self::default()
        } else {
            let mut config = Ini::new();
            let map = config.load(path.as_str()).unwrap_or_default();
            let server = map.get("server").cloned().unwrap_or_default();
            let host = server
                .get("host")
                .cloned()
                .unwrap_or(None)
                .unwrap_or_default();
            let token = server
                .get("token")
                .cloned()
                .unwrap_or(None)
                .unwrap_or_default();

            Self {
                host: host.into(),
                token: token.into(),
                ..Self::default()
            }
        }
    }

    fn write_config(&mut self) {
        let path = Self::get_config_path();
        let mut config = Ini::new();
        config.load(path.as_str()).unwrap_or_default();
        config.set("Server", "host", Some(self.host.to_string()));
        config.set("Server", "token", Some(self.token.to_string()));
        self.saved = config.write(path.as_str()).is_ok();
        self.saved_changed()
    }
}
