pkgname="jewels"
arch=("x86_64")
pkgver="${CI_COMMIT_TAG:-0.0.0_alpha}"
pkgrel=1
source=(
    "jewels"
    "dev.imanuel.jewels.desktop"
    "dev.imanuel.jewels.autostart.desktop"
    "sc-apps-jewels.svg"
    "jewels-wg-config.service"
    "jewels-update.service"
    "jewels-update.socket"
)
sha512sums=(
    "SKIP"
    "SKIP"
    "SKIP"
    "SKIP"
    "SKIP"
    "SKIP"
    "SKIP"
)
depends=(
    "kirigami"
    "kirigami-addons"
    "qt6-base"
    "qt6-declarative"
    "qt6-webview"
)
package() {
  iconsDir="${pkgdir}/usr/share/icons/hicolor/scalable/apps"
  binDir="${pkgdir}/usr/bin"
  desktopDir="${pkgdir}/usr/share/applications"
  autostartDir="${pkgdir}/etc/xdg/autostart"
  systemdDir="${pkgdir}/usr/lib/systemd/system"

  mkdir -p "${iconsDir}"
  mkdir -p "${binDir}"
  mkdir -p "${desktopDir}"
  mkdir -p "${autostartDir}"
  mkdir -p "${systemdDir}"

  install -Dm644 ${srcdir}/jewels-wg-config.service ${systemdDir}/jewels-wg-config.service
  install -Dm644 ${srcdir}/jewels-update.service ${systemdDir}/jewels-update.service
  install -Dm644 ${srcdir}/jewels-update.socket ${systemdDir}/jewels-update.socket
  install -Dm755 "${srcdir}/jewels" "${binDir}/jewels"
  install -Dm644 "${srcdir}/dev.imanuel.jewels.desktop" "${desktopDir}/dev.imanuel.jewels.desktop"
  install -Dm644 "${srcdir}/dev.imanuel.jewels.autostart.desktop" "${autostartDir}/dev.imanuel.jewels.autostart.desktop"
  install -Dm644 "${srcdir}/sc-apps-jewels.svg" "${iconsDir}/jewels.svg"
  install -Dm644 "${srcdir}/sc-apps-jewels.svg" "${iconsDir}/dev.imanuel.jewels.svg"
}

post_install() {
  systemctl daemon-reload
  systemctl enable --now jewels-wg-config.service
  systemctl enable --now jewels-update.socket
}