use cxx_qt::{CxxQtType, Threading};
use cxx_qt_lib::QString;
use libjewels::configuration::{JewelsConfiguration, load_config, write_config};
use std::pin::Pin;

#[cxx_qt::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    impl cxx_qt::Threading for Login {}

    #[auto_cxx_name]
    unsafe extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(QString, host)]
        #[qproperty(QString, token)]
        #[qproperty(bool, logged_in, cxx_name = "loggedIn")]
        #[qproperty(bool, login_in_progress, cxx_name = "loginInProgress")]
        type Login = super::LoginStruct;

        #[qinvokable]
        fn logout(self: Pin<&mut Self>);

        #[qinvokable]
        fn login(self: Pin<&mut Self>);
    }
}

pub struct LoginStruct {
    host: QString,
    token: QString,
    logged_in: bool,
    login_in_progress: bool,
    pub(crate) join_handle: Option<tokio::task::JoinHandle<()>>,
}

impl Default for LoginStruct {
    fn default() -> Self {
        let config = load_config();

        Self {
            logged_in: !config.host.is_empty(),
            host: config.host.into(),
            token: config.token.into(),
            login_in_progress: false,
            join_handle: None,
        }
    }
}

impl ffi::Login {
    fn logout(mut self: Pin<&mut Self>) {
        self.as_mut().set_host("".into());
        self.as_mut().set_token("".into());
        let _ = write_config(JewelsConfiguration::default());
        self.as_mut().set_logged_in(false);
        self.as_mut().set_login_in_progress(false);
    }

    fn login(mut self: Pin<&mut Self>) {
        self.as_mut().set_login_in_progress(true);
        let qt_thread = self.qt_thread();
        self.rust_mut().join_handle = Some(tokio::spawn(async move {
            crate::authentication::start_listener().await;
            let config = load_config();
            qt_thread
                .queue(move |mut login| {
                    login.as_mut().set_logged_in(!config.host.is_empty());
                    login.as_mut().set_login_in_progress(false);
                    login.as_mut().set_host(config.host.into());
                    login.as_mut().set_token(config.token.into());
                })
                .unwrap();
            if libjewels::collector::send_device_data().await.is_err() {
                log::error!("Error sending device data");
                tokio::time::sleep(std::time::Duration::from_secs(300)).await;
            }
        }));
    }
}
