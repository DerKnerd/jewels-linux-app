use crate::models::packages::{DownloadStatus, Package};
use futures_util::StreamExt;
use libjewels::alpm::{DownloadProgress, InstallProgress, InstallablePackage};
use libjewels::dbus::get_bus;
use libjewels::dbus::pacman::PacmanProxy;
use libjewels::dbus::screensaver::ScreenSaverProxy;
use notify_rust::{Hint, Notification, Timeout, Urgency};
use qmetaobject::{
    QObject, QPointer, SimpleListModel, qt_base_class, qt_method, qt_property, qt_signal,
};
use qttypes::QString;
use std::cell::RefCell;
use tokio::select;
use zbus::Connection;

#[allow(non_snake_case)]
#[derive(QObject, Default)]
pub struct Install {
    base: qt_base_class!(trait QObject),
    pub availablePackages: qt_property!(RefCell<SimpleListModel<Package>>; CONST),
    pub refreshing: qt_property!(bool; NOTIFY refreshingChanged),
    pub refreshingFailed: qt_property!(bool; NOTIFY refreshingFailedChanged),
    pub installInProgress: qt_property!(bool; NOTIFY installInProgressChanged),
    pub installFinished: qt_property!(bool; NOTIFY installFinishedChanged),
    pub installFailed: qt_property!(bool; NOTIFY installFailedChanged),
    pub downloadFinished: qt_property!(bool; NOTIFY downloadFinishedChanged),
    pub installCount: qt_property!(i32; NOTIFY installCountChanged),
    pub downloadStatus1: qt_property!(RefCell<DownloadStatus>; NOTIFY downloadStatus1Changed),
    pub downloadStatus2: qt_property!(RefCell<DownloadStatus>; NOTIFY downloadStatus2Changed),
    pub downloadStatus3: qt_property!(RefCell<DownloadStatus>; NOTIFY downloadStatus3Changed),
    pub downloadStatus4: qt_property!(RefCell<DownloadStatus>; NOTIFY downloadStatus4Changed),
    pub installPackage: qt_property!(QString; NOTIFY installPackageChanged),
    pub installPercent: qt_property!(i32; NOTIFY installPercentChanged),
    pub installHowMany: qt_property!(usize; NOTIFY installHowManyChanged),
    pub installCurrent: qt_property!(usize; NOTIFY installCurrentChanged),
    pub query: qt_property!(QString; NOTIFY queryChanged),

    pub refreshingChanged: qt_signal!(),
    pub refreshingFailedChanged: qt_signal!(),
    pub installInProgressChanged: qt_signal!(),
    pub installFinishedChanged: qt_signal!(),
    pub installFailedChanged: qt_signal!(),
    pub installCountChanged: qt_signal!(),
    pub downloadStatus1Changed: qt_signal!(),
    pub downloadStatus2Changed: qt_signal!(),
    pub downloadStatus3Changed: qt_signal!(),
    pub downloadStatus4Changed: qt_signal!(),
    pub downloadFinishedChanged: qt_signal!(),
    pub installPackageChanged: qt_signal!(),
    pub installPercentChanged: qt_signal!(),
    pub installHowManyChanged: qt_signal!(),
    pub installCurrentChanged: qt_signal!(),
    pub queryChanged: qt_signal!(),

    pub search: qt_method!(
        fn search(&mut self, query: QString) {
            self.query = query.clone();
            self.queryChanged();
            self.search_packages(query.to_string());
        }
    ),
    pub togglePackage: qt_method!(
        fn togglePackage(&mut self, name: QString) {
            self.toggle_package_install(name.to_string());
        }
    ),
    pub performInstall: qt_method!(
        fn performInstall(&mut self) {
            self.perform_install();
        }
    ),

    packages_to_install: Vec<String>,
}

enum InstallStatus {
    Download(DownloadProgress),
    Install(InstallProgress),
    Complete,
    Error,
}

impl Install {
    pub fn search_packages(&mut self, query: String) {
        self.refreshing = true;
        self.refreshingChanged();
        let qptr = QPointer::from(&*self);
        let refresh = qmetaobject::queued_callback(move |packages: Vec<InstallablePackage>| {
            if let Some(this) = qptr.as_pinned() {
                let mut install_ref = this.borrow_mut();
                install_ref.refreshing = false;
                install_ref.refreshingChanged();

                let mut model = install_ref.availablePackages.borrow_mut();
                model.reset_data(packages.into_iter().map(Package::from).collect());
            }
        });
        tokio::spawn(async move {
            let result = || async move {
                let bus = get_bus().await?;
                let conn = PacmanProxy::new(&bus).await?;
                let packages = conn.search_packages(query.clone()).await?;
                Ok(packages) as zbus::Result<Vec<InstallablePackage>>
            };
            if let Ok(result) = result().await {
                refresh(result);
            }
        });
    }

    pub fn toggle_package_install(&mut self, name: String) {
        if self.packages_to_install.contains(&name) {
            self.packages_to_install
                .retain(|pkg| pkg.to_string() != name);
        } else {
            self.packages_to_install.push(name);
        }
    }

    pub fn perform_install(&mut self) {
        self.installInProgress = true;
        self.availablePackages
            .borrow_mut()
            .reset_data(Default::default());
        self.installInProgressChanged();
        let qptr = QPointer::from(&*self);
        let refresh_status = qmetaobject::queued_callback(move |install_status: InstallStatus| {
            if let Some(this) = qptr.as_pinned() {
                let mut install_ref = this.borrow_mut();
                match install_status {
                    InstallStatus::Download(progress) => {
                        install_ref.downloadFinished = false;
                        let download_statuses = [
                            &install_ref.downloadStatus1,
                            &install_ref.downloadStatus2,
                            &install_ref.downloadStatus3,
                            &install_ref.downloadStatus4,
                        ];
                        let percent = (progress.status as f64 / progress.total as f64) * 100f64;
                        let active_download = download_statuses
                            .iter()
                            .find(|status| status.borrow().name().to_string() == progress.filename);
                        let first_full_download = download_statuses
                            .iter()
                            .find(|status| status.borrow().total() == status.borrow().current());
                        if let Some(download) = active_download {
                            let mut download_ref = download.borrow_mut();
                            if download_ref.current() < progress.status as f64 {
                                download_ref.set_percent(percent);
                                download_ref.set_total(progress.total as f64);
                                download_ref.set_current(progress.status as f64);
                            }
                        } else if let Some(download) = first_full_download {
                            let mut download_ref = download.borrow_mut();
                            download_ref.set_percent(percent);
                            download_ref.set_total(progress.total as f64);
                            download_ref.set_current(progress.status as f64);
                            download_ref.set_name(progress.filename.into());
                        }
                        install_ref.downloadFinishedChanged();
                    }
                    InstallStatus::Install(progress) => {
                        install_ref.downloadFinished = true;
                        install_ref.installPackage = progress.package.into();
                        install_ref.installPercent = progress.percent;
                        install_ref.installHowMany = progress.howmany;
                        install_ref.installCurrent = progress.current;
                        install_ref.downloadStatus1.borrow_mut().reset();
                        install_ref.downloadStatus2.borrow_mut().reset();
                        install_ref.downloadStatus3.borrow_mut().reset();
                        install_ref.downloadStatus4.borrow_mut().reset();
                        install_ref.downloadFinishedChanged();
                        install_ref.installPackageChanged();
                        install_ref.installPercentChanged();
                        install_ref.installHowManyChanged();
                        install_ref.installCurrentChanged();
                    }
                    InstallStatus::Complete => {
                        install_ref.installFinished = true;
                        install_ref.installInProgress = false;
                        install_ref.downloadStatus1.borrow_mut().reset();
                        install_ref.downloadStatus2.borrow_mut().reset();
                        install_ref.downloadStatus3.borrow_mut().reset();
                        install_ref.downloadStatus4.borrow_mut().reset();
                        install_ref.installInProgressChanged();
                        install_ref.installFinishedChanged();
                        let query = install_ref.query.to_string();
                        install_ref.search_packages(query);
                        install_ref.packages_to_install.clear();
                    }
                    InstallStatus::Error => {
                        install_ref.installFailed = true;
                        install_ref.installInProgress = false;
                        install_ref.downloadStatus1.borrow_mut().reset();
                        install_ref.downloadStatus2.borrow_mut().reset();
                        install_ref.downloadStatus3.borrow_mut().reset();
                        install_ref.downloadStatus4.borrow_mut().reset();
                        install_ref.installInProgressChanged();
                        install_ref.installFinishedChanged();
                        let query = install_ref.query.to_string();
                        install_ref.search_packages(query);
                        install_ref.packages_to_install.clear();
                    }
                }
            }
        });

        let packages_to_install = self.packages_to_install.clone();
        tokio::spawn(async move {
            let mut screen_saver_cookie = None;
            let _ = async  {
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
