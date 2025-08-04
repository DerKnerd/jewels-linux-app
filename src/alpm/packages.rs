use crate::alpm::get_alpm_handle;
use alpm::{Alpm, LogLevel, Question, TransFlag};
use anyhow::anyhow;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum FailureReason {
    PackageCorrupted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Callback {
    failure: Option<FailureReason>,
}

fn log_callback(level: LogLevel, msg: &str, _ctx: &mut Rc<RefCell<Callback>>) {
    let rust_level = match level {
        LogLevel::DEBUG => log::Level::Debug,
        LogLevel::FUNCTION => log::Level::Debug,
        LogLevel::ERROR => log::Level::Error,
        LogLevel::WARNING => log::Level::Warn,
        _ => log::Level::Info,
    };
    log::log!(rust_level, "{}", msg.trim_end());
}

fn progress_callback(
    _progress: alpm::Progress,
    name: &str,
    percent: i32,
    n: usize,
    total: usize,
    _ctx: &mut Rc<RefCell<Callback>>,
) {
    log::info!("{name} {percent}% ({total}/{n})");
}

fn question_callback(question: alpm::AnyQuestion, ctx: &mut Rc<RefCell<Callback>>) {
    match question.question() {
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

fn perform_update(tries: usize) -> Result<(), anyhow::Error> {
    if tries == 3 {
        return Err(anyhow!("Failed to update system"));
    }

    let (mut handle, callback) = get_handle_and_callback()?;

    handle.syncdbs_mut().update(true)?;

    resync_keyrings()?;

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
            log::info!("Got corrupted packages, resync the keyrings and try again");
            resync_keyrings()?;
            perform_update(tries + 1)
        } else {
            Ok(())
        }
    }
}

fn resync_keyrings() -> Result<(), anyhow::Error> {
    let (mut handle, ..) = get_handle_and_callback()?;

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

fn get_handle_and_callback() -> anyhow::Result<(Alpm, Rc<RefCell<Callback>>)> {
    let handle = get_alpm_handle()?;
    let callback = Rc::new(RefCell::new(Callback { failure: None }));
    handle.set_log_cb(callback.clone(), log_callback);
    handle.set_progress_cb(callback.clone(), progress_callback);
    handle.set_question_cb(callback.clone(), question_callback);
    handle.set_parallel_downloads(8);

    Ok((handle, callback))
}

pub fn update_system() -> Result<(), anyhow::Error> {
    perform_update(0)
}
