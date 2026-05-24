use crate::models::packages::{DownloadStatus, Package};
use libjewels::alpm::{DownloadProgress, InstallProgress, UpdatablePackage};
use libjewels::aur::AurPackage;
use libjewels::dbus::aur::AurProxy;
use libjewels::dbus::screensaver::ScreenSaverProxy;
use libjewels::dbus::{get_bus, pacman::PacmanProxy};
use notify_rust::{Hint, Notification, Timeout, Urgency};
use qmetaobject::{
    QObject, QPointer, SimpleListModel, qt_base_class, qt_method, qt_property, qt_signal,
};
use qttypes::QString;
use std::cell::RefCell;
use tokio::select;
use zbus::Connection;
use zbus::export::ordered_stream::OrderedStreamExt;

#[allow(non_snake_case)]
#[derive(QObject, Default)]
pub struct Updates {
    base: qt_base_class!(trait QObject),
    pub updatablePackages: qt_property!(RefCell<SimpleListModel<Package>>; CONST),
    pub refreshing: qt_property!(bool; NOTIFY refreshingChanged),
    pub refreshingFailed: qt_property!(bool; NOTIFY refreshingFailedChanged),
    pub updateInProgress: qt_property!(bool; NOTIFY updateInProgressChanged),
    pub updateFinished: qt_property!(bool; NOTIFY updateFinishedChanged),
    pub updateFailed: qt_property!(bool; NOTIFY updateFailedChanged),
    pub downloadFinished: qt_property!(bool; NOTIFY downloadFinishedChanged),
    pub updateCount: qt_property!(i32; NOTIFY updateCountChanged),
    pub downloadStatus1: qt_property!(RefCell<DownloadStatus>; NOTIFY downloadStatus1Changed),
    pub downloadStatus2: qt_property!(RefCell<DownloadStatus>; NOTIFY downloadStatus2Changed),
    pub downloadStatus3: qt_property!(RefCell<DownloadStatus>; NOTIFY downloadStatus3Changed),
    pub downloadStatus4: qt_property!(RefCell<DownloadStatus>; NOTIFY downloadStatus4Changed),
    pub installPackage: qt_property!(QString; NOTIFY installPackageChanged),
    pub installPercent: qt_property!(i32; NOTIFY installPercentChanged),
    pub installHowMany: qt_property!(usize; NOTIFY installHowManyChanged),
    pub installCurrent: qt_property!(usize; NOTIFY installCurrentChanged),

    pub refreshingChanged: qt_signal!(),
    pub refreshingFailedChanged: qt_signal!(),
    pub updateInProgressChanged: qt_signal!(),
    pub updateFinishedChanged: qt_signal!(),
    pub updateFailedChanged: qt_signal!(),
    pub updateCountChanged: qt_signal!(),
    pub downloadStatus1Changed: qt_signal!(),
    pub downloadStatus2Changed: qt_signal!(),
    pub downloadStatus3Changed: qt_signal!(),
    pub downloadStatus4Changed: qt_signal!(),
    pub downloadFinishedChanged: qt_signal!(),
    pub installPackageChanged: qt_signal!(),
    pub installPercentChanged: qt_signal!(),
    pub installHowManyChanged: qt_signal!(),
    pub installCurrentChanged: qt_signal!(),

    pub updateSystem: qt_method!(
        fn updateSystem(&mut self) {
            self.update_system();
        }
    ),
    pub refreshCache: qt_method!(
        fn refreshCache(&mut self) {
            self.refresh_packages();
        }
    ),
}

enum UpdateStatus {
    Download(DownloadProgress),
    Update(InstallProgress),
    Complete,
    Error,
}

impl Updates {
    pub fn update_system(&mut self) {
        self.updateInProgress = true;
        self.updatablePackages
            .borrow_mut()
            .reset_data(Default::default());
        self.updateInProgressChanged();

        let qptr = QPointer::from(&*self);
        let refresh_status = qmetaobject::queued_callback(move |updates: UpdateStatus| {
            if let Some(this) = qptr.as_pinned() {
                let mut updates_ref = this.borrow_mut();
                match updates {
                    UpdateStatus::Download(progress) => {
                        updates_ref.downloadFinished = false;
                        let download_statuses = [
                            &updates_ref.downloadStatus1,
                            &updates_ref.downloadStatus2,
                            &updates_ref.downloadStatus3,
                            &updates_ref.downloadStatus4,
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
                        updates_ref.downloadFinishedChanged();
                    }
                    UpdateStatus::Update(progress) => {
                        updates_ref.downloadFinished = true;
                        updates_ref.installPackage = progress.package.into();
                        updates_ref.installPercent = progress.percent;
                        updates_ref.installHowMany = progress.howmany;
                        updates_ref.installCurrent = progress.current;
                        updates_ref.downloadStatus1.borrow_mut().reset();
                        updates_ref.downloadStatus2.borrow_mut().reset();
                        updates_ref.downloadStatus3.borrow_mut().reset();
                        updates_ref.downloadStatus4.borrow_mut().reset();
                        updates_ref.downloadFinishedChanged();
                        updates_ref.installPackageChanged();
                        updates_ref.installPercentChanged();
                        updates_ref.installHowManyChanged();
                        updates_ref.installCurrentChanged();
                    }
                    UpdateStatus::Complete => {
                        updates_ref.updateFinished = true;
                        updates_ref.updateInProgress = false;
                        updates_ref.downloadStatus1.borrow_mut().reset();
                        updates_ref.downloadStatus2.borrow_mut().reset();
                        updates_ref.downloadStatus3.borrow_mut().reset();
                        updates_ref.downloadStatus4.borrow_mut().reset();
                        updates_ref.updateInProgressChanged();
                        updates_ref.updateFinishedChanged();
                    }
                    UpdateStatus::Error => {
                        updates_ref.updateFailed = true;
                        updates_ref.updateInProgress = false;
                        updates_ref.downloadStatus1.borrow_mut().reset();
                        updates_ref.downloadStatus2.borrow_mut().reset();
                        updates_ref.downloadStatus3.borrow_mut().reset();
                        updates_ref.downloadStatus4.borrow_mut().reset();
                        updates_ref.updateInProgressChanged();
                        updates_ref.updateFinishedChanged();
                    }
                }
            }
        });
        tokio::spawn(async move {
            let mut screen_saver_cookie = None;
            let _ = async  {
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
                            let mut update = if let Ok(update) = aur_proxy.receive_update().await {
                                update
                            } else {
                                return;
                            };
                            let mut failure = if let Ok(failure) = aur_proxy.receive_failure().await
                            {
                                failure
                            } else {
                                return;
                            };
                            let mut finished =
                                if let Ok(finished) = aur_proxy.receive_finished().await {
                                    finished
                                } else {
                                    return;
                                };
                            let mut build_started = if let Ok(build_started) =
                                aur_proxy.receive_build_started().await
                            {
                                build_started
                            } else {
                                return;
                            };
                            let mut built = if let Ok(built) = aur_proxy.receive_built().await {
                                built
                            } else {
                                return;
                            };
                            let mut failed = if let Ok(failed) = aur_proxy.receive_failed().await {
                                failed
                            } else {
                                return;
                            };

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
                                update_aur().await;
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

    async fn refresh_packages_async() -> zbus::Result<Vec<UpdatablePackage>> {
        let pacman = PacmanProxy::new(&get_bus().await?).await?;

        pacman.get_available_updates().await.map_err(Into::into)
    }

    async fn refresh_aur_packages_async() -> zbus::Result<Vec<AurPackage>> {
        let aur = AurProxy::new(&get_bus().await?).await?;

        aur.get_available_updates().await.map_err(Into::into)
    }

    pub fn refresh_packages(&mut self) {
        self.refreshing = true;
        self.refreshingChanged();

        let qptr = QPointer::from(&*self);
        let refresh =
            qmetaobject::queued_callback(move |updates: Option<Vec<UpdatablePackage>>| {
                if let Some(this) = qptr.as_pinned() {
                    let mut updates_ref = this.borrow_mut();
                    if let Some(updates) = updates {
                        updates_ref.updateCount = updates.len() as i32;
                        updates_ref.refreshingFailed = false;
                        let mut model = updates_ref.updatablePackages.borrow_mut();
                        model.reset_data(updates.into_iter().map(Package::from).collect());
                        updates_ref.updateCountChanged();
                        updates_ref.refreshingFailedChanged();
                    } else {
                        updates_ref.refreshingFailed = true;
                        updates_ref.refreshingFailedChanged();
                    }
                    updates_ref.refreshing = false;
                    updates_ref.refreshingChanged();
                }
            });

        let qptr = QPointer::from(&*self);
        let append_aur = qmetaobject::queued_callback(move |updates: Option<Vec<AurPackage>>| {
            if let Some(this) = qptr.as_pinned() {
                let mut updates_ref = this.borrow_mut();
                if let Some(updates) = updates {
                    updates_ref.updateCount = updates.len() as i32;
                    updates_ref.refreshingFailed = false;
                    let mut model = updates_ref.updatablePackages.borrow_mut();
                    model.reset_data(updates.into_iter().map(Package::from).collect());
                    updates_ref.updateCountChanged();
                    updates_ref.refreshingFailedChanged();
                } else {
                    updates_ref.refreshingFailed = true;
                    updates_ref.refreshingFailedChanged();
                }
                updates_ref.refreshing = false;
                updates_ref.refreshingChanged();
            }
        });

        tokio::spawn(async move {
            let (pacman, aur) = futures_util::future::join(
                Self::refresh_packages_async(),
                Self::refresh_aur_packages_async(),
            )
            .await;
            match pacman {
                Ok(updates) => refresh(Some(updates)),
                Err(err) => {
                    log::error!("Failed to refresh pacman packages: {err:#?}");
                    refresh(None)
                }
            }
            match aur {
                Ok(updates) => append_aur(Some(updates)),
                Err(err) => {
                    log::error!("Failed to refresh aur packages: {err:#?}");
                    append_aur(None)
                }
            }
        });
    }
}
