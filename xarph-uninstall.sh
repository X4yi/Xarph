#!/bin/sh
# xarph-uninstall - Remove all Xarph desktop environment files installed by PKGBUILD
#
# Usage: xarph-uninstall [--purge]
#   --purge   Also remove user config and data directories

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

PURGE=false
if [ "$1" = "--purge" ]; then
    PURGE=true
fi

PACKAGE_NAME="xarph-desktop"

remove_path() {
    if [ -e "$1" ] || [ -L "$1" ]; then
        rm -rf "$1"
    fi
}

remove_package() {
    if command -v pacman >/dev/null 2>&1 && pacman -Qq "$PACKAGE_NAME" >/dev/null 2>&1; then
        echo -e "${YELLOW}Removing pacman package ${PACKAGE_NAME}...${NC}"
        if command -v sudo >/dev/null 2>&1; then
            sudo pacman -Rns --noconfirm "$PACKAGE_NAME"
        else
            pacman -Rns --noconfirm "$PACKAGE_NAME"
        fi
        return 0
    fi
    return 1
}

echo -e "${GREEN}Xarph Desktop Environment Uninstaller${NC}"
echo "======================================"
echo ""

if ! remove_package; then
    # ── Binaries ───────────────────────────────────────────────────
    echo -e "${YELLOW}Removing binaries...${NC}"
    remove_path /usr/bin/xarph-shell
    remove_path /usr/bin/xarph-lock
    remove_path /usr/bin/xarph-admin
    remove_path /usr/bin/Xarhives
    remove_path /usr/bin/xarph-services
    remove_path /usr/bin/xarph-network
    remove_path /usr/bin/xarph-settings
    remove_path /usr/bin/xarph-wm
    remove_path /usr/bin/xarph-session

    # ── Desktop entries ────────────────────────────────────────────
    echo -e "${YELLOW}Removing desktop entries...${NC}"
    remove_path /usr/share/wayland-sessions/xarph.desktop

    # ── Systemd units ─────────────────────────────────────────────
    echo -e "${YELLOW}Removing systemd units...${NC}"
    remove_path /usr/lib/systemd/user/xarph-session.target
    remove_path /usr/lib/systemd/user/xarph-wm.service
    remove_path /usr/lib/systemd/user/xarph-wm-shutdown.target
    remove_path /usr/lib/systemd/user/xarph-shell.service

    # ── XDG portal config ──────────────────────────────────────────
    echo -e "${YELLOW}Removing portal config...${NC}"
    remove_path /usr/share/xdg-desktop-portal/xarph-portals.conf
fi

remove_path /usr/share/xarph/conf
systemctl --user daemon-reload 2>/dev/null || true

# ── User data (optional) ──────────────────────────────────────────
if [ "$PURGE" = true ]; then
    echo -e "${YELLOW}Removing user config and data...${NC}"
    remove_path "$HOME/.config/xarph"
    remove_path "$HOME/.local/share/xarph"
    remove_path "$HOME/.local/share/xarph-wm"
    remove_path "$HOME/.cache/xarph"
    echo -e "${GREEN}User config and data removed.${NC}"
else
    echo ""
    echo -e "${YELLOW}User config preserved at:${NC}"
    echo "  ~/.config/xarph/"
    echo "  ~/.local/share/xarph/"
    echo "  ~/.local/share/xarph-wm/"
    echo "  ~/.cache/xarph/"
    echo ""
    echo -e "${YELLOW}To remove user data too, run:${NC}"
    echo "  $0 --purge"
fi

echo ""
echo -e "${GREEN}Xarph Desktop Environment uninstalled successfully.${NC}"
echo "Note: System dependencies (gtk4, wayland, pipewire, etc.) were NOT removed."
