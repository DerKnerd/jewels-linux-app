use crate::models::packages::Package;
use crate::{with_model, with_model_bool};
use cxx_qt::CxxQtType;
use cxx_qt::Threading;
use cxx_qt_lib::{QModelIndex, QString, QVariant};
use futures_util::StreamExt;
use libjewels::alpm::{DownloadProgress, InstallProgress, UpdatablePackage};
use libjewels::aur::AurPackage;
use libjewels::dbus::aur::AurProxy;
use libjewels::dbus::get_bus;
use libjewels::dbus::pacman::PacmanProxy;
use libjewels::dbus::screensaver::ScreenSaverProxy;
use notify_rust::{Hint, Notification, Timeout, Urgency};
use std::pin::Pin;
use tokio::select;
use zbus::Connection;

async fn refresh_packages_async() -> zbus::Result<Vec<UpdatablePackage>> {
    let pacman = PacmanProxy::new(&get_bus().await?).await?;

    pacman.get_available_updates().await.map_err(Into::into)
}

async fn refresh_aur_packages_async() -> zbus::Result<Vec<AurPackage>> {
    let aur = AurProxy::new(&get_bus().await?).await?;

    aur.get_available_updates().await.map_err(Into::into)
}

enum UpdateStatus {
    Download(DownloadProgress),
    Update(InstallProgress),
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

        #[rust_name = "new_update_download_status"]
        fn new_ptr() -> *mut UpdateDownloadStatus;
    }

    #[qenum(Updates)]
    enum UpdatesRoles {
        Name,
        Version,
        Description,
    }

    impl cxx_qt::Threading for Updates {}
    impl cxx_qt::Initialize for Updates {}

    #[auto_cxx_name]
    #[auto_rust_name]
    unsafe extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[base = QAbstractListModel]
        #[qproperty(bool, refreshing)]
        #[qproperty(bool, refreshing_failed)]
        #[qproperty(bool, update_in_progress)]
        #[qproperty(bool, update_finished)]
        #[qproperty(bool, update_failed)]
        #[qproperty(bool, download_finished)]
        #[qproperty(i32, update_count)]
        #[qproperty(*mut UpdateDownloadStatus, download_status_1)]
        #[qproperty(*mut UpdateDownloadStatus, download_status_2)]
        #[qproperty(*mut UpdateDownloadStatus, download_status_3)]
        #[qproperty(*mut UpdateDownloadStatus, download_status_4)]
        #[qproperty(QString, install_package)]
        #[qproperty(i32, install_percent)]
        #[qproperty(usize, install_how_many)]
        #[qproperty(usize, install_current)]
        type Updates = super::UpdatesStruct;

        #[qinvokable]
        fn updateSystem(self: Pin<&mut Self>);

        #[qinvokable]
        fn refreshCache(self: Pin<&mut Self>);

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
        type UpdateDownloadStatus = super::UpdateDownloadStatusStruct;

        #[qinvokable]
        fn reset(self: Pin<&mut Self>);

        #[qinvokable]
        fn isFull(self: Pin<&mut Self>) -> bool;
    }
}

#[derive(Default)]
pub struct UpdatesStruct {
    packages: Vec<Package>,
    refreshing: bool,
    refreshing_failed: bool,
    update_in_progress: bool,
    update_finished: bool,
    update_failed: bool,
    download_finished: bool,
    update_count: i32,
    download_status_1: *mut ffi::UpdateDownloadStatus,
    download_status_2: *mut ffi::UpdateDownloadStatus,
    download_status_3: *mut ffi::UpdateDownloadStatus,
    download_status_4: *mut ffi::UpdateDownloadStatus,
    install_package: QString,
    install_percent: i32,
    install_how_many: usize,
    install_current: usize,
    join_handle: Option<tokio::task::JoinHandle<()>>,
}

#[derive(Default)]
pub struct UpdateDownloadStatusStruct {
    name: QString,
    percent: f64,
    total: f64,
    current: f64,
}

impl ffi::UpdateDownloadStatus {
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

impl cxx_qt::Initialize for ffi::Updates {
    fn initialize(mut self: Pin<&mut Self>) {
        self.as_mut()
            .set_download_status_1(ffi::new_update_download_status());
        self.as_mut()
            .set_download_status_2(ffi::new_update_download_status());
        self.as_mut()
            .set_download_status_3(ffi::new_update_download_status());
        self.as_mut()
            .set_download_status_4(ffi::new_update_download_status());
    }
}

impl ffi::Updates {
    fn row_count(&self, _: &QModelIndex) -> i32 {
        self.packages.len() as i32
    }

    fn role_names(&self) -> ffi::QHash_i32_QByteArray {
        let mut hash = ffi::QHash_i32_QByteArray::default();
        hash.insert(ffi::UpdatesRoles::Name.repr, "name".into());
        hash.insert(ffi::UpdatesRoles::Version.repr, "version".into());
        hash.insert(ffi::UpdatesRoles::Description.repr, "description".into());
        hash
    }

    fn data(&self, index: &ffi::QModelIndex, role: i32) -> QVariant {
        let role = ffi::UpdatesRoles { repr: role };

        if let Some(Package {
            name,
            version,
            description,
        }) = self.packages.get(index.row() as usize)
        {
            match role {
                ffi::UpdatesRoles::Name => return name.into(),
                ffi::UpdatesRoles::Version => return version.into(),
                ffi::UpdatesRoles::Description => return description.into(),
                _ => {}
            }
        }
        QVariant::default()
    }

    fn update_system(mut self: Pin<&mut Self>) {
        self.as_mut().set_update_in_progress(true);
        self.as_mut().begin_reset_model();
        self.as_mut().rust_mut().packages = vec![];
        self.as_mut().end_reset_model();

        let qt_thread = self.qt_thread();
        let refresh_status = move |updates: UpdateStatus| {
            qt_thread
                .queue(move |mut this| match updates {
                    UpdateStatus::Download(progress) => {
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
                    UpdateStatus::Update(progress) => {
                        this.as_mut().set_download_finished(true);
                        this.as_mut().set_install_package(progress.package.into());
                        this.as_mut().set_install_percent(progress.percent);
                        this.as_mut().set_install_how_many(progress.howmany);
                        this.as_mut().set_install_current(progress.current);
                    }
                    UpdateStatus::Complete | UpdateStatus::Error => {
                        this.as_mut().set_update_finished(true);
                        this.as_mut().set_update_in_progress(false);
                    }
                })
                .unwrap();
        };
        tokio::spawn(async move {
            let mut screen_saver_cookie = None;
            let _ = async {
                let session_bus = Connection::session().await?;
                let screen_saver_proxy = ScreenSaverProxy::new(&session_bus).await?;
                let cookie = screen_saver_proxy
                    .inhibit("Jewels Desktop", "Rechner updaten")
                    .await;
                screen_saver_cookie = cookie.ok();
                let conn = get_bus().await?;
                let pacman = PacmanProxy::new(&conn).await?;

                let mut download = pacman.receive_download().await?;
                let mut update = pacman.receive_update().await?;
                let mut failure = pacman.receive_failure().await?;
                let mut finished = pacman.receive_finished().await?;

                let aur_proxy = AurProxy::new(&conn).await.ok();
                let upgrade_aur_packages = if let Some(ref aur_proxy) = aur_proxy
                    && let Ok(upgrade_aur_packages) = aur_proxy.get_available_updates().await
                {
                    upgrade_aur_packages
                } else {
                    vec![]
                };
                let aur_packages_count = upgrade_aur_packages.len();

                if pacman.install_updates(4).await.is_err() {
                    refresh_status(UpdateStatus::Error);
                }

                let notify_success = {
                    let refresh_status = refresh_status.clone();
                    || async move {
                        refresh_status(UpdateStatus::Complete);
                        let _ = Notification::new()
                            .summary("Dein Rechner ist aktuell")
                            .body("Super, die Updates wurden erfolgreich installiert und dein Rechner ist jetzt auf dem neuesten Stand.")
                            .appname("jewels")
                            .icon("jewels")
                            .show_async()
                            .await;
                    }
                };
                let notify_error = {
                    let refresh_status = refresh_status.clone();
                    || async move {
                        refresh_status(UpdateStatus::Error);
                        let _ = Notification::new()
                            .summary("Fehler beim Updaten")
                            .body("Die Updates haben leider nicht geklappt. Du kannst es noch einmal versuchen, wenn das auch nicht hilft, wende dich an den Support.")
                            .appname("jewels")
                            .urgency(Urgency::Critical)
                            .icon("jewels")
                            .hint(Hint::Resident(true))
                            .timeout(Timeout::Never)
                            .show();
                    }
                };
                let update_aur = {
                    let refresh_status = refresh_status.clone();
                    let notify_error = notify_error.clone();
                    let notify_success = notify_success.clone();

                    move || async move {
                        if let Some(aur_proxy) = aur_proxy {
                            let mut update = aur_proxy.receive_update().await?;
                            let mut failure = aur_proxy.receive_failure().await?;
                            let mut finished = aur_proxy.receive_finished().await?;
                            let mut build_started = aur_proxy.receive_build_started().await?;
                            let mut built = aur_proxy.receive_built().await?;
                            let mut failed = aur_proxy.receive_failed().await?;

                            if aur_proxy.install_updates().await.is_err() {
                                refresh_status(UpdateStatus::Error);
                            }

                            let mut current_package = 0;
                            loop {
                                select! {
                                    Some(signal) = update.next() => {
                                        if let Ok(args) = signal.args() {
                                            refresh_status(UpdateStatus::Update(args.progress));
                                        }
                                    },
                                    Some(signal) = build_started.next() => {
                                        if let Ok(args) = signal.args() {
                                            refresh_status(UpdateStatus::Update(InstallProgress {
                                                package: args.package.clone(),
                                                percent: (((aur_packages_count as f64) / (current_package as f64)) * 100f64) as i32,
                                                howmany: aur_packages_count,
                                                current: current_package,
                                            }));
                                        }
                                    },
                                    Some(signal) = built.next() => {
                                        if let Ok(args) = signal.args() {
                                            current_package += 1;
                                            refresh_status(UpdateStatus::Update(InstallProgress {
                                                package: args.package.clone(),
                                                percent: (((aur_packages_count as f64) / (current_package as f64)) * 100f64) as i32,
                                                howmany: aur_packages_count,
                                                current: current_package,
                                            }));
                                        }
                                    },
                                    Some(signal) = failed.next() => {
                                        if let Ok(args) = signal.args() {
                                            current_package += 1;
                                            refresh_status(UpdateStatus::Update(InstallProgress {
                                                package: args.package.clone(),
                                                percent: (((aur_packages_count as f64) / (current_package as f64)) * 100f64) as i32,
                                                howmany: aur_packages_count,
                                                current: current_package,
                                            }));
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
                        } else {
                            refresh_status(UpdateStatus::Complete);
                            let _ = Notification::new()
                                .summary("Dein Rechner ist aktuell")
                                .body("Super, die Updates wurden erfolgreich installiert und dein Rechner ist jetzt auf dem neuesten Stand.")
                                .appname("jewels")
                                .icon("jewels")
                                .show_async()
                                .await;
                        }

                        Ok(()) as zbus::Result<()>
                    }
                };

                loop {
                    select! {
                        Some(signal) = download.next() => {
                            if let Ok(args) = signal.args() {
                                refresh_status(UpdateStatus::Download(args.progress));
                            }
                        },
                        Some(signal) = update.next() => {
                            if let Ok(args) = signal.args() {
                                refresh_status(UpdateStatus::Update(args.progress));
                            }
                        },
                        Some(_) = finished.next() => {
                            if aur_packages_count > 0 {
                                let _ = update_aur().await;
                            } else {
                                notify_success().await;
                            }
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

    fn refresh_cache(mut self: Pin<&mut Self>) {
        if let Some(join_handle) = &self.as_mut().join_handle {
            join_handle.abort();
        }
        self.as_mut().set_refreshing(true);
        self.as_mut().set_refreshing_failed(false);

        let qt_thread = self.qt_thread();
        self.as_mut().rust_mut().join_handle = Some(tokio::spawn(async move {
            let (pacman, aur) =
                futures_util::future::join(refresh_packages_async(), refresh_aur_packages_async())
                    .await;
            let mut packages = vec![];
            let mut failed = false;
            if let Ok(pacman) = pacman {
                let mut pacman = pacman.into_iter().map(Package::from).collect::<Vec<_>>();
                packages.append(&mut pacman);
            } else {
                failed = true;
            }
            if !failed && let Ok(aur) = aur {
                let mut aur = aur.into_iter().map(Package::from).collect::<Vec<_>>();
                packages.append(&mut aur);
            } else {
                failed = true;
            }
            qt_thread
                .queue(move |mut updates| {
                    updates.as_mut().set_update_count(packages.len() as i32);
                    updates.as_mut().set_refreshing(false);
                    updates.as_mut().set_refreshing_failed(failed);
                    updates.as_mut().begin_reset_model();
                    updates.as_mut().rust_mut().packages = packages;
                    updates.as_mut().end_reset_model();
                })
                .unwrap();
        }));
    }
}
