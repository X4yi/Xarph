# Maintainer: Xarph Developers
# PKGBUILD for Xarph Desktop Environment - local build
# Run "makepkg -si" from the project root directory

pkgname=xarph-desktop
pkgver=0.2.0
pkgrel=1
pkgdesc="Xarph Desktop Environment - A modern Wayland desktop"
arch=('x86_64')
url="https://github.com/xarph/xarph"
license=('GPL-3.0-or-later')
depends=(
    'alacritty'
    'fuzzel'
    'gtk4'
    'gtk4-layer-shell'
    'gstreamer'
    'gst-plugins-base'
    'gst-plugins-good'
    'pipewire'
    'polkit'
    'xdg-desktop-portal'
    'xdg-desktop-portal-gtk'
    'dbus'
    'systemd'
)
makedepends=(
    'cargo'
    'rust'
    'pkg-config'
    'clang'
    'libinput'
    'seatd'
    'pipewire'
    'pango'
    'cairo'
    'glib2'
    'libdrm'
    'mesa'
    'libglvnd'
    'wayland-protocols'
    'libxkbcommon'
    'gstreamer'
    'gst-plugins-base'
)
options=(!lto)

source=()
sha256sums=()

prepare() {
    cd "${startdir}"
}

build() {
    cd "${startdir}"

    cargo build --release \
        -p xarph-shell \
        -p xarph-settings \
        -p xarph-lock \
        -p xarph-services \
        -p xarph-network \
        -p xarph-admin \
        -p Xarhives \
        -p xarph-wm
}

package() {
    cd "${startdir}"

    local BINDIR="${pkgdir}/usr/bin"
    local CONFDIR="${pkgdir}/usr/share/xarph/conf"
    local SESSDIR="${pkgdir}/usr/share/wayland-sessions"
    local SYSDDIR="${pkgdir}/usr/lib/systemd/user"
    local PORTALDIR="${pkgdir}/usr/share/xdg-desktop-portal"

    # Binaries
    install -Dm755 target/release/xarph-shell         "${BINDIR}/xarph-shell"
    install -Dm755 target/release/xarph-settings       "${BINDIR}/xarph-settings"
    install -Dm755 target/release/xarph-lock           "${BINDIR}/xarph-lock"
    install -Dm755 target/release/xarph-services       "${BINDIR}/xarph-services"
    install -Dm755 target/release/xarph-network        "${BINDIR}/xarph-network"
    install -Dm755 target/release/xarph-admin       "${BINDIR}/xarph-admin"
    install -Dm755 target/release/Xarhives          "${BINDIR}/Xarhives"
    install -Dm755 target/release/xarph-wm            "${BINDIR}/xarph-wm"
    install -Dm755 xarph-wm/resources/xarph-session   "${BINDIR}/xarph-session"

    # Desktop entry
    install -Dm644 xarph-wm/resources/xarph.desktop   "${SESSDIR}/xarph.desktop"

    # Systemd units
    install -Dm644 xarph-wm/resources/xarph-wm.service          "${SYSDDIR}/xarph-wm.service"
    install -Dm644 xarph-wm/resources/xarph-wm-shutdown.target  "${SYSDDIR}/xarph-wm-shutdown.target"
    install -Dm644 xarph-wm/resources/xarph-shell.service       "${SYSDDIR}/xarph-shell.service"
    install -Dm644 data/systemd/user/xarph-session.target       "${SYSDDIR}/xarph-session.target"

    # Default shell config
    install -d "${CONFDIR}"
    cp -a data/conf/. "${CONFDIR}/"

    # XDG portal config
    install -Dm644 xarph-wm/resources/xarph-portals.conf  "${PORTALDIR}/xarph-portals.conf"
}
