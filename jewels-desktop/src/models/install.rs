use crate::models::packages::Package;
use crate::{with_model, with_model_bool};
use cxx_qt::CxxQtType;
use cxx_qt::Threading;
use cxx_qt_lib::{QModelIndex, QString, QVariant};
use futures_util::StreamExt;
use libjewels::alpm::{DownloadProgress, InstallProgress, InstallablePackage, UpdatablePackage};
use libjewels::aur::AurPackage;
use libjewels::dbus::aur::AurProxy;
use libjewels::dbus::get_bus;
use libjewels::dbus::pacman::PacmanProxy;
use libjewels::dbus::screensaver::ScreenSaverProxy;
use notify_rust::{Hint, Notification, Timeout, Urgency};
use std::pin::Pin;
use tokio::select;
use zbus::Connection;

enum InstallStatus {
    Download(DownloadProgress),
    Install(InstallProgress),
    Complete,
    Error,
}

#[cxx_qt::bridge]
mod ffi {
    unsafe extern "C++" {
        include!(<QAbstractListModel>);
        type QAbstractListModel;

        include!("cxx-qt-lib/qmodelindex.h");
        type QModelIndex = cxx_qt_lib::QModelIndex;

        include!("cxx-qt-lib/qvariant.h");
        type QVariant = cxx_qt_lib::QVariant;

        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;

        include!("cxx-qt-lib/qhash.h");
        type QHash_i32_QByteArray = cxx_qt_lib::QHash<cxx_qt_lib::QHashPair_i32_QByteArray>;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "new_install_download_status"]
        fn new_ptr() -> *mut InstallDownloadStatus;
    }

    #[qenum(Install)]
    enum InstallRoles {
        Name,
        Version,
        Description,
    }

    impl cxx_qt::Threading for Install {}
    impl cxx_qt::Initialize for Install {}

    #[auto_cxx_name]
    #[auto_rust_name]
    unsafe extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[base = QAbstractListModel]
        #[qproperty(bool, refreshing)]
        #[qproperty(bool, refreshing_failed)]
        #[qproperty(bool, install_in_progress)]
        #[qproperty(bool, install_finished)]
        #[qproperty(bool, install_failed)]
        #[qproperty(bool, download_finished)]
        #[qproperty(*mut InstallDownloadStatus, download_status_1)]
        #[qproperty(*mut InstallDownloadStatus, download_status_2)]
        #[qproperty(*mut InstallDownloadStatus, download_status_3)]
        #[qproperty(*mut InstallDownloadStatus, download_status_4)]
        #[qproperty(QString, install_package)]
        #[qproperty(i32, install_percent)]
        #[qproperty(usize, install_how_many)]
        #[qproperty(usize, install_current)]
        type Install = super::InstallStruct;

        #[cxx_override]
        fn rowCount(&self, parent: &QModelIndex) -> i32;

        #[cxx_override]
        fn data(&self, index: &QModelIndex, role: i32) -> QVariant;

        #[cxx_override]
        fn roleNames(&self) -> QHash_i32_QByteArray;

        #[inherit]
        fn beginResetModel(self: Pin<&mut Self>);

        #[inherit]
        fn endResetModel(self: Pin<&mut Self>);

        #[qinvokable]
        fn search(self: Pin<&mut Self>, query: QString);

        #[qinvokable]
        fn performInstall(self: Pin<&mut Self>);

        #[qinvokable]
        fn togglePackage(self: Pin<&mut Self>, name: QString);
    }

    #[auto_cxx_name]
    #[auto_rust_name]
    unsafe extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(QString, name)]
        #[qproperty(f64, percent)]
        #[qproperty(f64, total)]
        #[qproperty(f64, current)]
        type InstallDownloadStatus = super::InstallDownloadStatusStruct;

        #[qinvokable]
        fn reset(self: Pin<&mut Self>);

        #[qinvokable]
        fn isFull(self: Pin<&mut Self>) -> bool;
    }
}

#[derive(Default)]
pub struct InstallStruct {
    packages: Vec<Package>,
    packages_to_install: Vec<String>,
    refreshing: bool,
    refreshing_failed: bool,
    install_in_progress: bool,
    install_finished: bool,
    install_failed: bool,
    download_finished: bool,
    download_status_1: *mut ffi::InstallDownloadStatus,
    download_status_2: *mut ffi::InstallDownloadStatus,
    download_status_3: *mut ffi::InstallDownloadStatus,
    download_status_4: *mut ffi::InstallDownloadStatus,
    install_package: QString,
    install_percent: i32,
    install_how_many: usize,
    install_current: usize,
    query: String,
    join_handle: Option<tokio::task::JoinHandle<()>>,
}

#[derive(Default)]
pub struct InstallDownloadStatusStruct {
    name: QString,
    percent: f64,
    total: f64,
    current: f64,
}

impl ffi::InstallDownloadStatus {
    fn reset(mut self: Pin<&mut Self>) {
        self.as_mut().set_name("".into());
        self.as_mut().set_percent(0f64);
        self.as_mut().set_total(0f64);
        self.as_mut().set_current(0f64);
    }

    fn is_full(self: Pin<&mut Self>) -> bool {
        self.current() == self.total()
    }
}

impl cxx_qt::Initialize for ffi::Install {
    fn initialize(mut self: Pin<&mut Self>) {
        self.as_mut()
            .set_download_status_1(ffi::new_install_download_status());
        self.as_mut()
            .set_download_status_2(ffi::new_install_download_status());
        self.as_mut()
            .set_download_status_3(ffi::new_install_download_status());
        self.as_mut()
            .set_download_status_4(ffi::new_install_download_status());
    }
}

impl ffi::Install {
    fn row_count(&self, _: &QModelIndex) -> i32 {
        self.packages.len() as i32
    }

    fn role_names(&self) -> ffi::QHash_i32_QByteArray {
        let mut hash = ffi::QHash_i32_QByteArray::default();
        hash.insert(ffi::InstallRoles::Name.repr, "name".into());
        hash.insert(ffi::InstallRoles::Version.repr, "version".into());
        hash.insert(ffi::InstallRoles::Description.repr, "description".into());
        hash
    }

    fn data(&self, index: &ffi::QModelIndex, role: i32) -> QVariant {
        let role = ffi::InstallRoles { repr: role };

        if let Some(Package {
            name,
            version,
            description,
        }) = self.packages.get(index.row() as usize)
        {
            match role {
                ffi::InstallRoles::Name => return name.into(),
                ffi::InstallRoles::Version => return version.into(),
                ffi::InstallRoles::Description => return description.into(),
                _ => {}
            }
        }
        QVariant::default()
    }

    fn search(mut self: Pin<&mut Self>, query: QString) {
        self.as_mut().set_refreshing(true);
        self.as_mut().rust_mut().query = query.to_string();
        let qt_thread = self.qt_thread();
        tokio::spawn(async move {
            let packages = || async move {
                let bus = get_bus().await?;
                let conn = PacmanProxy::new(&bus).await?;
                let packages = conn.search_packages(query.to_string()).await?;
                Ok(packages) as zbus::Result<Vec<InstallablePackage>>
            };
            if let Ok(packages) = packages().await {
                let packages = packages.into_iter().map(Package::from).collect();
                qt_thread
                    .queue(move |mut install| {
                        install.as_mut().set_refreshing(false);
                        install.as_mut().begin_reset_model();
                        install.as_mut().rust_mut().packages = packages;
                        install.as_mut().end_reset_model();
                    })
                    .unwrap();
            }
        });
    }

    fn toggle_package(mut self: Pin<&mut Self>, name: QString) {
        let name = name.to_string();
        if self.as_mut().rust_mut().packages_to_install.contains(&name) {
            self.as_mut()
                .rust_mut()
                .packages_to_install
                .retain(|pkg| pkg.to_string() != name);
        } else {
            self.as_mut().rust_mut().packages_to_install.push(name);
        }
    }

    fn perform_install(mut self: Pin<&mut Self>) {
        self.as_mut().set_install_in_progress(true);
        self.as_mut().begin_reset_model();
        self.as_mut().rust_mut().packages = vec![];
        self.as_mut().end_reset_model();

        let qt_thread = self.qt_thread();
        let refresh_status = move |install_status: InstallStatus| {
            qt_thread
                .queue(move |mut this| match install_status {
                    InstallStatus::Download(progress) => {
                        this.as_mut().set_download_finished(false);
                        let download_statuses = [
                            this.download_status_1(),
                            this.download_status_2(),
                            this.download_status_3(),
                            this.download_status_4(),
                        ];
                        let percent = (progress.status as f64 / progress.total as f64) * 100f64;
                        let active_download = download_statuses.into_iter().find(|status| {
                            with_model_bool!(*status, |model| {
                                model.name() == &QString::from(&progress.filename)
                            })
                        });
                        let first_full_download = download_statuses
                            .iter()
                            .find(|status| with_model_bool!(*status, |model| model.is_full()));
                        if let Some(download) = active_download {
                            with_model!(*download, |download| {
                                if *download.current() < progress.status as f64 {
                                    download.as_mut().set_percent(percent);
                                    download.as_mut().set_total(progress.total as f64);
                                    download.as_mut().set_current(progress.status as f64);
                                }
                            });
                        } else if let Some(download) = first_full_download {
                            with_model!(*download, |download| {
                                download.as_mut().set_percent(percent);
                                download.as_mut().set_total(progress.total as f64);
                                download.as_mut().set_current(progress.status as f64);
                                download.as_mut().set_name(progress.filename.into());
                            });
                        }
                    }
                    InstallStatus::Install(progress) => {
                        this.as_mut().set_download_finished(true);
                        this.as_mut().set_install_package(progress.package.into());
                        this.as_mut().set_install_percent(progress.percent);
                        this.as_mut().set_install_how_many(progress.howmany);
                        this.as_mut().set_install_current(progress.current);
                    }
                    InstallStatus::Complete | InstallStatus::Error => {
                        this.as_mut().set_install_finished(true);
                        this.as_mut().set_install_in_progress(false);
                        let query = QString::from(this.as_mut().query.clone());
                        this.as_mut().search(query);
                    }
                })
                .unwrap();
        };

        let packages_to_install = self.rust().packages_to_install.clone();
        tokio::spawn(async move {
            let mut screen_saver_cookie = None;
            let _ = async {
                let session_bus = Connection::session().await?;
                let screen_saver_proxy = ScreenSaverProxy::new(&session_bus).await?;
                let cookie = screen_saver_proxy
                    .inhibit("Jewels Desktop", "Software installieren")
                    .await;
                screen_saver_cookie = cookie.ok();
                let conn = get_bus().await?;
                let pacman = PacmanProxy::new(&conn).await?;

                let mut download = pacman.receive_download().await?;
                let mut update = pacman.receive_update().await?;
                let mut failure = pacman.receive_failure().await?;
                let mut finished = pacman.receive_finished().await?;

                if pacman.install_package(packages_to_install, 4).await.is_err() {
                    refresh_status(InstallStatus::Error);
                }

                let notify_success = {
                    let refresh_status = refresh_status.clone();
                    || async move {
                        refresh_status(InstallStatus::Complete);
                        let _ = Notification::new()
                            .summary("Software installer")
                            .body("Super, die Software wurden erfolgreich installiert.")
                            .appname("jewels")
                            .icon("jewels")
                            .show_async()
                            .await;
                    }
                };
                let notify_error = {
                    let refresh_status = refresh_status.clone();
                    || async move {
                        refresh_status(InstallStatus::Error);
                        let _ = Notification::new()
                            .summary("Fehler beim Installieren")
                            .body("Die Installation hat leider nicht geklappt. Du kannst es noch einmal versuchen, wenn das auch nicht hilft, wende dich an den Support.")
                            .appname("jewels")
                            .urgency(Urgency::Critical)
                            .icon("jewels")
                            .hint(Hint::Resident(true))
                            .timeout(Timeout::Never)
                            .show();
                    }
                };

                loop {
                    select! {
                        Some(signal) = download.next() => {
                            if let Ok(args) = signal.args() {
                                refresh_status(InstallStatus::Download(args.progress));
                            }
                        },
                        Some(signal) = update.next() => {
                            if let Ok(args) = signal.args() {
                                refresh_status(InstallStatus::Install(args.progress));
                            }
                        },
                        Some(_) = finished.next() => {
                            notify_success().await;
                            break;
                        }
                        Some(_) = failure.next() => {
                            notify_error().await;
                            break;
                        }
                        else => break
                    }
                }

                Ok(()) as zbus::Result<()>
            }.await;

            if let Ok(session_bus) = Connection::session().await
                && let Ok(screen_saver_proxy) = ScreenSaverProxy::new(&session_bus).await
                && let Some(cookie) = screen_saver_cookie
            {
                let _ = screen_saver_proxy.uninhibit(cookie).await;
            }
        });
    }
}
