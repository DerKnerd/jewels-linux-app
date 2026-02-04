use crate::models::config::{JewelsConfiguration, load_config, write_config};
use qmetaobject::{QObject, qt_base_class, qt_method, qt_property, qt_signal};
use qttypes::QString;

#[allow(non_snake_case)]
#[derive(QObject, Default)]
pub struct Login {
    base: qt_base_class!(trait QObject),
    pub host: qt_property!(QString; NOTIFY host_changed),
    pub token: qt_property!(QString; NOTIFY token_changed),
    pub loggedIn: qt_property!(bool; NOTIFY loggedInChanged),
    pub loginInProgress: qt_property!(bool;  NOTIFY loginInProgressChanged),
    pub loggedInChanged: qt_signal!(),
    pub loginInProgressChanged: qt_signal!(),
    pub host_changed: qt_signal!(),
    pub token_changed: qt_signal!(),
    pub triggerLogin: qt_method!(
        fn triggerLogin(&mut self) {
            self.loginInProgress = true;
            self.loginInProgressChanged();
        }
    ),
    pub logout: qt_method!(
        fn logout(&mut self) {
            self.perform_logout();
        }
    ),
}

impl Login {
    pub fn new() -> Self {
        let config = load_config();

        Self {
            loggedIn: !config.host.is_empty(),
            host: config.host.into(),
            token: config.token.into(),
            loginInProgress: false,
            ..Self::default()
        }
    }

    fn perform_logout(&mut self) {
        self.host = "".into();
        self.token = "".into();
        let _ = write_config(JewelsConfiguration::default());
        self.loggedIn = false;
        self.loginInProgress = false;
        self.loggedInChanged();
        self.loginInProgressChanged();
    }
}
