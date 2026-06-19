#!/bin/sh
# Xarph manual installer — run with sudo
set -e

SRCDIR="$(cd "$(dirname "$0")" && pwd)"
BINDIR=/usr/bin
CONFDIR=/usr/share/xarph/conf
SESSDIR=/usr/share/wayland-sessions
SYSDDIR=/usr/lib/systemd/user
PORTALDIR=/usr/share/xdg-desktop-portal

echo "Installing Xarph binaries..."
install -Dm755 "${SRCDIR}/target/release/xarph-shell"         "${BINDIR}/xarph-shell"
install -Dm755 "${SRCDIR}/target/release/xarph-settings"      "${BINDIR}/xarph-settings"
install -Dm755 "${SRCDIR}/target/release/xarph-lock"          "${BINDIR}/xarph-lock"
install -Dm755 "${SRCDIR}/target/release/xarph-services"      "${BINDIR}/xarph-services"
install -Dm755 "${SRCDIR}/target/release/xarph-network"       "${BINDIR}/xarph-network"
install -Dm755 "${SRCDIR}/target/release/xarph-admin"        "${BINDIR}/xarph-admin"
install -Dm755 "${SRCDIR}/target/release/Xarhives"           "${BINDIR}/Xarhives"
install -Dm755 "${SRCDIR}/target/release/xarph-wm"            "${BINDIR}/xarph-wm"
install -Dm755 "${SRCDIR}/xarph-wm/resources/xarph-session"   "${BINDIR}/xarph-session"

echo "Installing desktop entry..."
install -Dm644 "${SRCDIR}/xarph-wm/resources/xarph.desktop" "${SESSDIR}/xarph.desktop"

echo "Installing systemd units..."
install -Dm644 "${SRCDIR}/xarph-wm/resources/xarph-wm.service"         "${SYSDDIR}/xarph-wm.service"
install -Dm644 "${SRCDIR}/xarph-wm/resources/xarph-wm-shutdown.target" "${SYSDDIR}/xarph-wm-shutdown.target"
install -Dm644 "${SRCDIR}/xarph-wm/resources/xarph-shell.service"      "${SYSDDIR}/xarph-shell.service"
install -Dm644 "${SRCDIR}/data/systemd/user/xarph-session.target"      "${SYSDDIR}/xarph-session.target"

echo "Installing default shell config..."
install -d "${CONFDIR}"
cp -a "${SRCDIR}/data/conf/." "${CONFDIR}/"

echo "Installing XDG portal config..."
install -Dm644 "${SRCDIR}/xarph-wm/resources/xarph-portals.conf" "${PORTALDIR}/xarph-portals.conf"

echo "Reloading systemd..."
systemctl --user daemon-reload 2>/dev/null || true

echo ""
echo "Xarph installed successfully!"
echo "To start: select 'Xarph' from your display manager, or run 'xarph-session' from a TTY."
