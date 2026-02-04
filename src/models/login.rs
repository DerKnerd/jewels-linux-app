use crate::models::config::{JewelsConfiguration, load_config, write_config};
use qmetaobject::{QObject, QPointer, qt_base_class, qt_method, qt_property, qt_signal};
use qttypes::QString;
use crate::commands::collection::run_collection;

#[allow(non_snake_case)]
#[derive(QObject)]
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
    pub loginSuccessful: qt_signal!(),
    pub triggerLogin: qt_method!(
        fn triggerLogin(&mut self) {
            self.perform_login();
        }
    ),
    pub logout: qt_method!(
        fn logout(&mut self) {
            self.perform_logout();
        }
    ),
}

impl Default for Login {
    fn default() -> Self {
        let config = load_config();

        Self {
            loggedIn: !config.host.is_empty(),
            host: config.host.into(),
            token: config.token.into(),
            loginInProgress: false,
            base: Default::default(),
            triggerLogin: Default::default(),
            logout: Default::default(),
            host_changed: Default::default(),
            token_changed: Default::default(),
            loginSuccessful: Default::default(),
            loggedInChanged: Default::default(),
            loginInProgressChanged: Default::default(),
        }
    }
}

impl Login {
    fn perform_logout(&mut self) {
        self.host = "".into();
        self.token = "".into();
        let _ = write_config(JewelsConfiguration::default());
        self.loggedIn = false;
        self.loginInProgress = false;
        self.loggedInChanged();
        self.loginInProgressChanged();
    }

    fn perform_login(&mut self) {
        self.loginInProgress = true;
        self.loginInProgressChanged();
        let qptr = QPointer::from(&*self);
        let reload_configuration = qmetaobject::queued_callback(move |()| {
            let config = load_config();
            qptr.as_pinned().map(|this| {
                this.borrow_mut().loggedIn = !config.host.is_empty();
                this.borrow_mut().loginInProgress = false;
                this.borrow_mut().host = config.host.into();
                this.borrow_mut().token = config.token.into();
                this.borrow().host_changed();
                this.borrow().token_changed();
                this.borrow().loggedInChanged();
                this.borrow().loginInProgressChanged();
                this.borrow().loginSuccessful();
            });
            tokio::spawn(async move {
                run_collection().await;
            });
        });
        tokio::spawn(async move {
            crate::authentication::start_listener().await;
            reload_configuration(());
        });
    }
}
