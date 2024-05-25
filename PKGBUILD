pkgname="jewels"
pkgver=${CI_COMMIT_TAG:-1.0.0}
pkgrel=${CI_PIPELINE_IID:-1}
arch=("x86_64")
source=(
    "jewels"
    "dev.imanuel.jewels.desktop"
    "dev.imanuel.jewels.autostart.desktop"
    "sc-apps-jewels.svg"
)
sha512sums=(
    "SKIP"
    "SKIP"
    "SKIP"
    "SKIP"
)
depends=(
    "kconfig"
    "kcoreaddons"
    "gcc-libs"
    "glib2"
    "glibc"
    "kconfig"
    "kcoreaddons"
    "kcrash"
    "kdbusaddons"
    "ki18n"
    "kirigami"
    "kirigami-addons"
    "kwidgetsaddons"
    "qt6-base"
    "qt6-declarative"
    "qt6-webview"
)
package() {
  iconsDir="${pkgdir}/usr/share/icons/hicolor/scalable/apps"
  binDir="${pkgdir}/usr/bin"
  desktopDir="${pkgdir}/usr/share/applications"
  autostartDir="${pkgdir}/etc/xdg/autostart"

  mkdir -p "${iconsDir}"
  mkdir -p "${binDir}"
  mkdir -p "${desktopDir}"
  mkdir -p "${autostartDir}"

  install -Dm755 "${srcdir}/jewels" "${binDir}/jewels"
  install -Dm644 "${srcdir}/dev.imanuel.jewels.desktop" "${desktopDir}/dev.imanuel.jewels.desktop"
  install -Dm644 "${srcdir}/dev.imanuel.jewels.autostart.desktop" "${autostartDir}/dev.imanuel.jewels.autostart.desktop"
  install -Dm644 "${srcdir}/sc-apps-jewels.svg" "${iconsDir}/jewels.svg"
  install -Dm644 "${srcdir}/sc-apps-jewels.svg" "${iconsDir}/dev.imanuel.jewels.svg"
}
