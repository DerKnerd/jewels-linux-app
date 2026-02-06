use crate::alpm::get_alpm_handle;
use alpm::{Alpm, DownloadEvent, DownloadResult, LogLevel, Question, TransFlag};
use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use tokio::sync::mpsc::{Receiver, Sender};
use zbus::zvariant::Type;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Serialize, Deserialize, Type)]
pub struct UpdateProgress {
    pub package: String,
    pub percent: i32,
    pub howmany: usize,
    pub current: usize,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Serialize, Deserialize, Type)]
pub struct DownloadProgress {
    pub filename: String,
    pub status: i64,
    pub total: i64,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Serialize, Deserialize, Type)]
pub struct LogMessage {
    pub message: String,
    pub level: String,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Serialize, Deserialize, Type)]
pub struct UpdatablePackage {
    pub name: String,
    pub new_version: String,
    pub description: String,
}

pub type UpdateProgressReceiver = Receiver<UpdateProgress>;
pub type UpdateProgressSender = Sender<UpdateProgress>;

pub type DownloadProgressReceiver = Receiver<DownloadProgress>;
pub type DownloadProgressSender = Sender<DownloadProgress>;

pub type LogMessageReceiver = Receiver<LogMessage>;
pub type LogMessageSender = Sender<LogMessage>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum FailureReason {
    PackageCorrupted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Callback {
    failure: Option<FailureReason>,
}

#[derive(Debug, Clone)]
pub struct AlpmHelper {
    download_progress_sender: DownloadProgressSender,
    update_progress_sender: UpdateProgressSender,
    log_message_sender: LogMessageSender,
}

impl AlpmHelper {
    pub fn new(
        download_progress_sender: DownloadProgressSender,
        update_progress_sender: UpdateProgressSender,
        log_message_sender: LogMessageSender,
    ) -> Self {
        Self {
            download_progress_sender,
            update_progress_sender,
            log_message_sender,
        }
    }

    fn log_callback(&self, level: LogLevel, msg: String) {
        let rust_level = match level {
            LogLevel::DEBUG => log::Level::Debug,
            LogLevel::FUNCTION => log::Level::Debug,
            LogLevel::ERROR => log::Level::Error,
            LogLevel::WARNING => log::Level::Warn,
            _ => log::Level::Info,
        };
        log::log!(rust_level, "{}", msg.trim_end());
        let sender = self.log_message_sender.clone();
        tokio::spawn(async move {
            if let Err(err) = sender
                .send(LogMessage {
                    message: msg.to_string(),
                    level: rust_level.to_string(),
                })
                .await
            {
                log::error!("Failed to send log progress: {}", err);
            }
        });
    }

    fn progress_callback(&self, name: String, percent: i32, n: usize, total: usize) {
        log::info!("{name} {percent}% ({total}/{n})");
        let sender = self.update_progress_sender.clone();
        tokio::spawn(async move {
            if let Err(err) = sender
                .send(UpdateProgress {
                    package: name.to_string(),
                    percent,
                    howmany: total,
                    current: n,
                })
                .await
            {
                log::error!("Failed to send update progress: {err}");
            }
        });
    }

    fn question_callback(&self, question: Question, ctx: &mut Rc<RefCell<Callback>>) {
        match question {
            Question::InstallIgnorepkg(mut question) => question.set_install(true),
            Question::Replace(question) => {
                let message = format!(
                    "Replacing package {} with {}",
                    question.oldpkg().name(),
                    question.newpkg().name()
                );
                log::info!("{message}");
                question.set_replace(true)
            }
            Question::Conflict(mut question) => {
                let message = format!(
                    "Cancel due to conflict between {} and {}",
                    question.conflict().package1().name(),
                    question.conflict().package2().name()
                );
                log::error!("{message}");
                question.set_remove(true)
            }
            Question::Corrupted(mut question) => {
                let message = format!("Corrupted file {}", question.filepath());
                log::error!("{message}");
                question.set_remove(true);
                ctx.borrow_mut().failure = Some(FailureReason::PackageCorrupted);
            }
            Question::RemovePkgs(mut question) => question.set_skip(false),
            Question::SelectProvider(mut question) => question.set_index(0),
            Question::ImportKey(mut question) => question.set_import(true),
        }
    }

    fn download_callback(&self, filename: String, download_event: DownloadEvent) {
        let sender = self.download_progress_sender.clone();
        tokio::spawn(async move {
            match download_event {
                DownloadEvent::Progress(evt) => {
                    log::info!("{filename}: {}/{}", evt.downloaded, evt.total);
                    if let Err(err) = sender
                        .send(DownloadProgress {
                            status: evt.downloaded,
                            total: evt.total,
                            filename: filename.to_string(),
                        })
                        .await
                    {
                        log::error!("Failed to send download progress: {}", err);
                    }
                }
                DownloadEvent::Completed(evt) => {
                    if !matches!(evt.result, DownloadResult::Failed) {
                        log::info!("{filename}: {}/{}", evt.total, evt.total);
                        if let Err(err) = sender
                            .send(DownloadProgress {
                                status: evt.total,
                                total: evt.total,
                                filename: filename.to_string(),
                            })
                            .await
                        {
                            log::error!("Failed to send download progress: {}", err);
                        }
                    }
                }
                _ => {}
            }
        });
    }

    fn resync_keyrings(self) -> Result<(), anyhow::Error> {
        let (mut handle, ..) = self.get_handle_and_callback()?;

        handle.syncdbs_mut().update(true)?;

        handle.trans_init(TransFlag::empty())?;

        if let Some(archlinux_keyring) = handle
            .syncdbs()
            .iter()
            .find_map(|db| db.pkg("archlinux-keyring").ok())
        {
            handle
                .trans_add_pkg(archlinux_keyring)
                .map_err(|err| anyhow!(err.to_string()))?;
        }
        if let Some(chaotic_keyring) = handle
            .syncdbs()
            .iter()
            .find_map(|db| db.pkg("chaotic-keyring").ok())
        {
            handle
                .trans_add_pkg(chaotic_keyring)
                .map_err(|err| anyhow!(err.to_string()))?;
        }

        handle.trans_prepare().map_err(|err| anyhow!(err.error()))?;
        handle.trans_commit().map_err(|err| anyhow!(err.error()))?;

        handle.trans_release().map_err(|err| anyhow!(err))
    }

    fn get_handle_and_callback(self) -> anyhow::Result<(Alpm, Rc<RefCell<Callback>>)> {
        let handle = get_alpm_handle()?;
        let callback = Rc::new(RefCell::new(Callback { failure: None }));

        let self_ref = Arc::new(self);

        {
            let self_ref = self_ref.clone();
            handle.set_log_cb(callback.clone(), move |level, msg, _ctx| {
                self_ref.log_callback(level.clone(), msg.to_string())
            });
        }
        {
            let self_ref = self_ref.clone();
            handle.set_progress_cb(
                callback.clone(),
                move |_progress, package, percent, howmany, current, _ctx| {
                    self_ref.progress_callback(package.to_string(), percent, howmany, current)
                },
            );
        }
        {
            let self_ref = self_ref.clone();
            handle.set_question_cb(callback.clone(), move |question, ctx| {
                self_ref.question_callback(question.question(), ctx)
            });
        }
        {
            let self_ref = self_ref.clone();
            handle.set_dl_cb(callback.clone(), move |filename, download_event, _ctx| {
                self_ref.download_callback(filename.to_string(), download_event.event().clone())
            });
        }
        handle.set_parallel_downloads(8);

        Ok((handle, callback))
    }

    pub fn update_system(self) -> Result<(), anyhow::Error> {
        let (mut handle, callback) = self.clone().get_handle_and_callback()?;

        handle.syncdbs_mut().update(true)?;

        self.resync_keyrings()?;

        handle.trans_init(TransFlag::empty())?;

        handle.sync_sysupgrade(false)?;
        if handle.trans_add().is_empty() && handle.trans_remove().is_empty() {
            handle.trans_release().map_err(|err| anyhow!(err))?;

            Ok(())
        } else {
            handle.trans_prepare().map_err(|err| anyhow!(err.error()))?;
            handle.trans_commit().map_err(|err| anyhow!(err.error()))?;

            handle.trans_release().map_err(|err| anyhow!(err))?;

            if let Some(FailureReason::PackageCorrupted) = callback.clone().borrow().failure {
                log::error!("Got corrupted packages, resync the keyrings and try again");
                Err(anyhow!("Corrupted packages"))
            } else {
                Ok(())
            }
        }
    }

    pub fn get_available_updates(self) -> anyhow::Result<Vec<UpdatablePackage>> {
        let (mut handle, ..) = self.get_handle_and_callback()?;

        handle.syncdbs_mut().update(false)?;

        handle.trans_init(TransFlag::empty())?;
        handle.sync_sysupgrade(false)?;

        let result = handle
            .trans_add()
            .iter()
            .map(|pkg| {
                UpdatablePackage {
                    name: pkg.name().to_string(),
                    new_version: pkg.version().to_string(),
                    description: pkg.desc().unwrap_or("").to_string(),
                }
            })
            .collect();

        handle.trans_release()?;

        Ok(result)
    }
}
