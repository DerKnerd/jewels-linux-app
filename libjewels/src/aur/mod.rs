mod build;
pub mod consts;
mod gpg;

use crate::alpm::{AlpmHelper, DownloadProgressSender, LogMessageSender, UpdateProgressSender};
use crate::aur::build::AurBuilder;
use crate::aur::consts::{JEWELS_BUILD_DIR, JEWELS_PACKAGE_DIR, JEWELS_USER};
use raur::Raur;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use tokio::sync::mpsc::{Receiver, Sender};
use zbus::zvariant::Type;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Serialize, Deserialize, Type)]
pub struct AurPackage {
    pub name: String,
    pub version: String,
    pub description: String,
}

pub type PackageBuildStartedReceiver = Receiver<String>;
pub type PackageBuildStartedSender = Sender<String>;

pub type PackageBuiltReceiver = Receiver<String>;
pub type PackageBuiltSender = Sender<String>;

pub type PackageFailedReceiver = Receiver<String>;
pub type PackageFailedSender = Sender<String>;

#[derive(Debug, Clone)]
pub struct AurHelper {
    download_progress_sender: DownloadProgressSender,
    update_progress_sender: UpdateProgressSender,
    log_message_sender: LogMessageSender,
    package_build_started_sender: PackageBuildStartedSender,
    package_built_sender: PackageBuiltSender,
    package_failed_sender: PackageFailedSender,
}

impl AurHelper {
    pub fn new(
        download_progress_sender: DownloadProgressSender,
        update_progress_sender: UpdateProgressSender,
        log_message_sender: LogMessageSender,
        package_build_started_sender: PackageBuildStartedSender,
        package_built_sender: PackageBuiltSender,
        package_failed_sender: PackageFailedSender,
    ) -> Self {
        Self {
            download_progress_sender,
            update_progress_sender,
            log_message_sender,
            package_build_started_sender,
            package_built_sender,
            package_failed_sender,
        }
    }

    pub async fn get_upgradable_packages(&self) -> anyhow::Result<Vec<AurPackage>> {
        log::info!("Checking for AUR packages to upgrade...");
        let alpm_helper = AlpmHelper::new(
            self.download_progress_sender.clone(),
            self.update_progress_sender.clone(),
            self.log_message_sender.clone(),
        );

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

    pub async fn build_aur_packages(&self) -> anyhow::Result<()> {
        log::info!("Upgrading AUR packages...");
        log::info!("Creating build directory...");
        tokio::fs::create_dir_all(JEWELS_BUILD_DIR).await?;
        tokio::fs::create_dir_all(JEWELS_PACKAGE_DIR).await?;
        file_owner::set_owner(JEWELS_BUILD_DIR, JEWELS_USER)?;
        file_owner::set_owner(JEWELS_PACKAGE_DIR, JEWELS_USER)?;
        let upgradable_packages = self.get_upgradable_packages().await?;
        let aur_builder = AurBuilder {};
        for pkg in upgradable_packages {
            log::info!("Building {}...", pkg.name);
            self.package_build_started_sender
                .send(pkg.name.clone())
                .await?;
            match aur_builder.build(&pkg.name).await {
                Ok(()) => {
                    log::info!("Successfully built {}", pkg.name);
                    let _ = self.package_built_sender.send(pkg.name.clone()).await;
                }
                Err(e) => {
                    log::error!("Failed to build {}: {}", pkg.name, e);
                    let _ = self.package_failed_sender.send(pkg.name.clone()).await;
                }
            }
            break;
        }

        Ok(())
    }

    pub async fn install_aur_packages(&self) -> anyhow::Result<()> {
        log::info!("Installing AUR packages...");
        let alpm_helper = AlpmHelper::new(
            self.download_progress_sender.clone(),
            self.update_progress_sender.clone(),
            self.log_message_sender.clone(),
        );
        let mut packages = tokio::fs::read_dir(JEWELS_PACKAGE_DIR).await?;
        let mut packages_to_install = vec![];
        while let Ok(Some(pkg)) = packages.next_entry().await {
            packages_to_install.push(pkg.path().into_os_string().into_string().unwrap());
        }

        alpm_helper.install_packages(packages_to_install)
    }

    pub async fn cleanup(&self) -> anyhow::Result<()> {
        log::info!("Cleaning up...");
        tokio::fs::remove_dir_all(JEWELS_BUILD_DIR).await?;
        tokio::fs::remove_dir_all(JEWELS_PACKAGE_DIR).await?;
        Ok(())
    }
}
