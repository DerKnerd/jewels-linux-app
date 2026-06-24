use crate::aur::consts::{JEWELS_BUILD_DIR, JEWELS_PACKAGE_DIR, JEWELS_USER};
use crate::aur::gpg::import_gpg_keys;
use srcinfo::Srcinfo;
use std::path::{Path, PathBuf};
use std::process::{ExitStatus, Stdio};
use std::str::FromStr;
use tokio::io::AsyncReadExt;
use tokio::process::Command;

pub struct AurBuilder {}

impl AurBuilder {
    async fn clone_repo(&self, url: &str, dir: &Path) -> anyhow::Result<()> {
        log::info!("Cloning {}...", url);
        Command::new("runuser")
            .arg("-u")
            .arg(JEWELS_USER)
            .arg("--")
            .arg("git")
            .arg("clone")
            .arg("--depth=1")
            .arg(url)
            .arg(dir)
            .status()
            .await?;
        Ok(())
    }

    /// Spawn a process, forward every output line as an event, and fail on
    /// non-zero exit.
    async fn execute_command(
        &self,
        program: &str,
        args: Vec<&str>,
        cwd: PathBuf,
    ) -> std::io::Result<ExitStatus> {
        let mut cmd = Command::new(program);
        let mut child = cmd
            .args(args.clone())
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .env("MAKEPKG_CONFIRM", "n")
            .env("PKGDEST", JEWELS_PACKAGE_DIR)
            .current_dir(cwd)
            .spawn()?;

        let mut stdout = child.stdout.take();
        let mut stderr = child.stderr.take();

        tokio::spawn(async move {
            if let Some(mut stdout) = stdout.take() {
                let mut buf = Vec::new();
                if stdout.read_to_end(buf.as_mut()).await.is_ok() {
                    log::debug!("{}", String::from_utf8_lossy(&buf));
                }
            }
        });
        tokio::spawn(async move {
            if let Some(mut stderr) = stderr.take() {
                let mut buf = Vec::new();
                if stderr.read_to_end(buf.as_mut()).await.is_ok() {
                    log::error!("{}", String::from_utf8_lossy(&buf));
                }
            }
        });

        log::debug!("Spawning command: {} {}", program, args.join(" "));
        child.wait().await
    }

    pub async fn build(&self, package: &str) -> anyhow::Result<()> {
        let pkg_dir = PathBuf::from_str(JEWELS_BUILD_DIR)?.join(package);

        self.clone_repo(
            &format!("https://aur.archlinux.org/{package}.git"),
            &pkg_dir,
        )
        .await?;

        let srcinfo_path = pkg_dir.join(".SRCINFO");
        if srcinfo_path.exists() && let Ok(info) = Srcinfo::from_path(srcinfo_path) {
            import_gpg_keys(package, info).await?;
        }

        log::info!("Building {}...", package);
        log::info!("Executing makepkg for {}...", package);
        self.execute_command(
            "runuser",
            vec![
                "-u",
                JEWELS_USER,
                "--",
                "makepkg",
                "--syncdeps",
                "--clean",
                "--noconfirm",
                "--noprogressbar",
            ],
            pkg_dir,
        )
        .await?;

        Ok(())
    }
}
