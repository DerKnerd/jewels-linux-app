use libjewels::alpm::{DownloadProgress, UpdatablePackage, UpdateProgress};
use libjewels::dbus::{PacmanProxy, get_bus};
use notify_rust::{Hint, Notification, Timeout, Urgency};
use qmetaobject::{
    QObject, QPointer, SimpleListItem, SimpleListModel, qt_base_class, qt_method, qt_property,
    qt_signal,
};
use qttypes::{QByteArray, QString, QVariant};
use std::cell::RefCell;
use tokio::select;
use zbus::export::ordered_stream::OrderedStreamExt;

#[derive(Clone, Default)]
pub struct Package {
    name: QString,
    version: QString,
    description: QString,
}

#[allow(non_snake_case)]
#[derive(QObject, Default)]
pub struct DownloadStatus {
    base: qt_base_class!(trait QObject),
    name: qt_property!(QString; NOTIFY nameChanged),
    percent: qt_property!(f64; NOTIFY percentChanged),
    total: qt_property!(f64; NOTIFY totalChanged),
    current: qt_property!(f64; NOTIFY currentChanged),
    nameChanged: qt_signal!(),
    percentChanged: qt_signal!(),
    totalChanged: qt_signal!(),
    currentChanged: qt_signal!(),
}

impl DownloadStatus {
    pub fn reset(&mut self) {
        self.name = QString::default();
        self.percent = 0f64;
        self.total = 0f64;
        self.current = 0f64;
        self.nameChanged();
        self.percentChanged();
        self.totalChanged();
        self.currentChanged();
    }
}

impl From<UpdatablePackage> for Package {
    fn from(updatable: UpdatablePackage) -> Self {
        Package {
            name: QString::from(updatable.name),
            version: QString::from(updatable.new_version),
            description: QString::from(updatable.description),
        }
    }
}

impl SimpleListItem for Package {
    fn get(&self, role: i32) -> QVariant {
        match role {
            0 => self.name.clone().into(),
            1 => self.version.clone().into(),
            2 => self.description.clone().into(),
            _ => QVariant::default(),
        }
    }

    fn names() -> Vec<QByteArray> {
        vec![
            QByteArray::from("name"),
            QByteArray::from("version"),
            QByteArray::from("description"),
        ]
    }
}

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
    Update(UpdateProgress),
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
                            .find(|status| status.borrow().name.to_string() == progress.filename);
                        let first_full_download = download_statuses
                            .iter()
                            .find(|status| status.borrow().total == status.borrow().current);
                        if let Some(download) = active_download {
                            let mut download_ref = download.borrow_mut();
                            if download_ref.current < progress.status as f64 {
                                download_ref.percent = percent;
                                download_ref.total = progress.total as f64;
                                download_ref.current = progress.status as f64;
                                download_ref.totalChanged();
                                download_ref.currentChanged();
                                download_ref.percentChanged();
                            }
                        } else if let Some(download) = first_full_download {
                            let mut download_ref = download.borrow_mut();
                            download_ref.percent = percent;
                            download_ref.total = progress.total as f64;
                            download_ref.current = progress.status as f64;
                            download_ref.name = progress.filename.into();
                            download_ref.totalChanged();
                            download_ref.currentChanged();
                            download_ref.percentChanged();
                            download_ref.nameChanged();
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
            let conn = if let Ok(conn) = get_bus().await {
                conn
            } else {
                return;
            };
            let pacman = if let Ok(pacman) = PacmanProxy::new(&conn).await {
                pacman
            } else {
                return;
            };

            let mut download = if let Ok(download) = pacman.receive_download().await {
                download
            } else {
                return;
            };
            let mut update = if let Ok(update) = pacman.receive_update().await {
                update
            } else {
                return;
            };
            let mut failure = if let Ok(failure) = pacman.receive_failure().await {
                failure
            } else {
                return;
            };
            let mut finished = if let Ok(finished) = pacman.receive_finished().await {
                finished
            } else {
                return;
            };

            if pacman.install_updates().await.is_err() {
                refresh_status(UpdateStatus::Error);
            }

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
                        refresh_status(UpdateStatus::Complete);
                            let _ = Notification::new()
                            .summary("Dein Rechner ist aktuell")
                            .body("Super, die Updates wurden erfolgreich installiert und dein Rechner ist jetzt auf dem neuesten Stand.")
                            .appname("jewels")
                            .icon("jewels")
                            .show_async()
                            .await;
                        break;
                    }
                    Some(_) = failure.next() => {
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
                        break;
                    }
                    else => break
                }
            }
        });
    }

    async fn refresh_packages_async() -> zbus::Result<Vec<UpdatablePackage>> {
        let pacman = PacmanProxy::new(&get_bus().await?).await?;

        pacman.get_available_updates().await.map_err(Into::into)
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

        tokio::spawn(async move {
            match Self::refresh_packages_async().await {
                Ok(updates) => refresh(Some(updates)),
                Err(err) => {
                    log::error!("Failed to refresh packages: {err:#?}");
                    refresh(None)
                }
            }
        });
    }
}
