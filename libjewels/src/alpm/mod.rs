pub mod config;
mod packages;

pub use packages::*;

use alpm::{Alpm, Usage};
use sysinfo::System;

pub(crate) fn get_alpm_handle() -> Result<Alpm, anyhow::Error> {
    let conf = config::get_config()?;
    let mut handle = Alpm::new(conf.root_dir, conf.db_path)?;
    handle.add_cachedir(conf.cache_dir.first().cloned().unwrap())?;

    for repo in conf.repos {
        log::info!("Registering repository: {}", repo.name);
        let db = handle.register_syncdb_mut(
            repo.name,
            alpm::SigLevel::from_name(
                repo.sig_level
                    .first()
                    .cloned()
                    .unwrap_or("default".to_string())
                    .as_str(),
            )
            .unwrap_or(alpm::SigLevel::USE_DEFAULT),
        );
        if let Err(err) = db {
            log::error!("Failed to register syncdb core: {err}");
        } else if let Ok(db) = db {
            for url in repo.servers {
                db.add_server(url)?;
            }
            db.set_usage(Usage::ALL)?;
        }
    }

    Ok(handle)
}

pub(crate) fn clear_stale_pacman_lock() -> anyhow::Result<()> {
    let lock_path = "/var/lib/pacman/db.lck";

    if !std::path::Path::new(lock_path).exists() {
        return Ok(());
    }

    let mut sys = System::new_all();
    sys.refresh_all();

    for process in sys.processes().values() {
        let name = process
            .name()
            .to_str()
            .map(|name| name.to_lowercase())
            .unwrap_or_default();
        if name == "pacman"
            || name == "pamac"
            || name == "yay"
            || name == "paru"
            || name == "trizen"
        {
            anyhow::bail!("Database locked by active process: {}", name);
        }
    }

    log::warn!("Removing stale pacman lock file!");
    std::fs::remove_file(lock_path)?;

    Ok(())
}
