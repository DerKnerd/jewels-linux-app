mod packages;

pub use packages::*;

use alpm::{Alpm, SigLevel, Usage};
use configparser::ini::Ini;

struct Repository {
    name: String,
    urls: Vec<String>,
    sig_level: SigLevel,
}

fn get_repositories() -> Result<Vec<Repository>, anyhow::Error> {
    let mut repositories = vec![];
    let mut pacman_conf = Ini::new();
    let config = pacman_conf
        .load("/etc/pacman.conf")
        .map_err(|err| anyhow::anyhow!(err))?;

    let sections = pacman_conf
        .sections()
        .into_iter()
        .filter(|section| section != "options")
        .collect::<Vec<_>>();
    for section in sections {
        if let Some(keys) = config.get(&section) {
            if let Some(Some(server)) = keys.get("server") {
                let sig_level = if let Some(Some(sig_level)) = keys.get("siglevel") {
                    SigLevel::from_name(sig_level).unwrap_or(SigLevel::USE_DEFAULT)
                } else {
                    SigLevel::USE_DEFAULT
                };
                repositories.push(Repository {
                    name: section,
                    urls: vec![server.to_string()],
                    sig_level,
                });
            } else if let Some(Some(include)) = keys.get("include")
                && std::fs::exists(include).is_ok_and(|res| res)
            {
                let include = std::fs::read_to_string(include)?;
                repositories.push(Repository {
                    name: section.clone(),
                    urls: include
                        .lines()
                        .filter(|line| line.starts_with("Server") && line.contains("://"))
                        .map(|url| {
                            url.trim_start_matches("Server = ")
                                .replace("$arch", "x86_64")
                                .replace("$repo", section.as_str())
                        })
                        .collect::<Vec<_>>(),
                    sig_level: SigLevel::USE_DEFAULT,
                })
            }
        }
    }

    Ok(repositories)
}

pub(crate) fn get_alpm_handle() -> Result<Alpm, anyhow::Error> {
    let mut handle = Alpm::new("/", "/var/lib/pacman/")?;
    handle.add_cachedir("/var/lib/pacman/cache")?;

    for repo in get_repositories()? {
        log::info!("Registering repository: {}", repo.name);
        let db = handle.register_syncdb_mut(repo.name, repo.sig_level);
        if let Err(err) = db {
            log::error!("Failed to register syncdb core: {err}");
        } else if let Ok(db) = db {
            for url in repo.urls {
                db.add_server(url)?;
            }
            db.set_usage(Usage::ALL)?;
        }
    }

    Ok(handle)
}
