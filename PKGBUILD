pkgname="jewels"
arch=("x86_64")
pkgver="${CI_COMMIT_TAG:-0.0.0_alpha}"
pkgrel=1
install=package/jewels.install
source=()
sha512sums=()
depends=(
    "kirigami"
    "kirigami-addons"
    "qt6-base"
    "qt6-declarative"
    "qt6-webview"
)

package() {
  systemdDir="${pkgdir}/usr/lib/systemd/system"
  dbusPolicyDir="${pkgdir}/usr/share/dbus-1/system.d"
  desktopDir="${pkgdir}/usr/share/applications"
  iconsDir="${pkgdir}/usr/share/icons/hicolor/scalable/apps"
  binDir="${pkgdir}/usr/bin"

  # systemd unit
  install -Dm644 "${startdir}/package/systemd/jewelsd.service" "${systemdDir}/jewelsd.service"

  # dbus-1 policy
  install -Dm644 "${startdir}/package/dbus-1/cloud.ulbricht.jewels.conf" "${dbusPolicyDir}/cloud.ulbricht.jewels.conf"

  # desktop file
  install -Dm644 "${startdir}/package/desktop/dev.imanuel.jewels.desktop" "${desktopDir}/dev.imanuel.jewels.desktop"

  # icons (same source file, two target names)
  install -Dm644 "${startdir}/package/desktop/sc-apps-jewels.svg" "${iconsDir}/jewels.svg"
  install -Dm644 "${startdir}/package/desktop/sc-apps-jewels.svg" "${iconsDir}/cloud.ulbricht,jewels.svg"

  # binaries
  install -Dm755 "${startdir}/target/release/jewelsd" "${binDir}/jewelsd"
  install -Dm755 "${startdir}/target/release/jewels-desktop" "${binDir}/jewels-desktop"
}