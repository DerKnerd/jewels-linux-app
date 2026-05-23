mod build;
pub mod consts;
mod gpg;

use std::collections::BTreeMap;

use crate::alpm::AlpmHelper;
use crate::aur::build::AurBuilder;
use raur::Raur;
use tokio::sync::mpsc;

#[derive(Debug, Clone)]
pub enum AurEvent {
    /// Package successfully upgraded.
    PackageDone { package: String },

    /// Something went wrong for one package (non-fatal; others continue).
    PackageError { package: String, reason: String },
}

pub struct AurPackage {
    pub name: String,
    pub version: String,
    pub description: String,
}

pub struct AurHelper {}

impl AurHelper {
    pub async fn get_upgradable_packages(&self) -> anyhow::Result<Vec<AurPackage>> {
        log::info!("Checking for AUR packages to upgrade...");
        let (dummy_dl_rx, _) = mpsc::channel(1);
        let (dummy_pr_rx, _) = mpsc::channel(1);
        let (dummy_lm_rx, _) = mpsc::channel(1);

        let alpm_helper = AlpmHelper::new(dummy_dl_rx, dummy_pr_rx, dummy_lm_rx);

        log::info!("Getting foreign packages...");
        let foreign_packages = alpm_helper.get_foreign_packages()?;

        let raur = raur::Handle::new();
        let names = foreign_packages
            .iter()
            .map(|(n, _)| n.as_str())
            .collect::<Vec<_>>();

        log::info!("Getting AUR packages...");
        let aur_pkgs = raur.info(&names).await?;

        let aur_map = aur_pkgs
            .iter()
            .map(|p| (p.name.as_str(), p))
            .collect::<BTreeMap<_, _>>();

        let mut packages_to_upgrade = vec![];

        log::info!("Compiling packages to upgrade...");
        for (name, local_ver) in foreign_packages {
            if let Some(&aur_pkg) = aur_map.get(name.as_str()) {
                if alpm::vercmp(aur_pkg.version.as_str(), local_ver.as_str())
                    == std::cmp::Ordering::Greater
                {
                    packages_to_upgrade.push(AurPackage {
                        name: aur_pkg.name.clone(),
                        version: aur_pkg.version.clone(),
                        description: aur_pkg.description.clone().unwrap_or_default(),
                    })
                }
            }
        }

        if packages_to_upgrade.is_empty() {
            log::info!("No packages to upgrade!");
        }

        Ok(packages_to_upgrade)
    }

    pub async fn upgrade_aur_packages(
        &self,
        package_sender: &mpsc::Sender<AurEvent>,
    ) -> anyhow::Result<()> {
        log::info!("Upgrading AUR packages...");
        log::info!("Creating build directory...");
        tokio::fs::create_dir_all("/tmp/jewels/build").await?;
        let upgradable_packages = self.get_upgradable_packages().await?;
        let aur_builder = AurBuilder {};
        for pkg in upgradable_packages {
            log::info!("Building {}...", pkg.name);
            match aur_builder.build(&pkg.name, &package_sender).await {
                Ok(()) => {
                    log::info!("Successfully built {}", pkg.name);
                    let _ = package_sender
                        .send(AurEvent::PackageDone {
                            package: pkg.name.clone(),
                        })
                        .await;
                }
                Err(e) => {
                    log::error!("Failed to build {}: {}", pkg.name, e);
                    let _ = package_sender
                        .send(AurEvent::PackageError {
                            package: pkg.name.clone(),
                            reason: format!("{e}"),
                        })
                        .await;
                }
            }
        }

        Ok(())
    }
}
