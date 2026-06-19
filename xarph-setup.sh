#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════
#  xarph-setup — Build, install, and configure the Xarph desktop
#
#  Usage:
#    ./xarph-setup              Build + install (incremental)
#    ./xarph-setup --deps       Install system dependencies only
#    ./xarph-setup --build      Build only (no install)
#    ./xarph-setup --install    Install only (skip build)
#    ./xarph-setup --uninstall  Remove all Xarph files
#    ./xarph-setup --status     Show what's installed
# ═══════════════════════════════════════════════════════════════════════
set -euo pipefail

SRCDIR="$(cd "$(dirname "$0")" && pwd)"
cd "$SRCDIR"

# ── Colors ─────────────────────────────────────────────────────────────
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
BOLD='\033[1m'
NC='\033[0m'

info()  { echo -e "${CYAN}[INFO]${NC}  $*"; }
ok()    { echo -e "${GREEN}[OK]${NC}    $*"; }
warn()  { echo -e "${YELLOW}[WARN]${NC}  $*"; }
err()   { echo -e "${RED}[ERROR]${NC} $*" >&2; }
step()  { echo -e "\n${BOLD}═══ $* ═══${NC}"; }

# ── Detect package manager ─────────────────────────────────────────────
detect_pkg_manager() {
    if command -v pacman &>/dev/null; then
        echo "pacman"
    elif command -v apt &>/dev/null; then
        echo "apt"
    elif command -v dnf &>/dev/null; then
        echo "dnf"
    elif command -v zypper &>/dev/null; then
        echo "zypper"
    else
        echo "unknown"
    fi
}

PKG_MANAGER="$(detect_pkg_manager)"

# ── All binaries produced by this workspace ────────────────────────────
BINARIES=(
    xarph-wm
    xarph-shell
    xarph-settings
    xarph-lock
    xarph-admin
    xarph-services
    xarph-network
    Xarhives
)

# ═══════════════════════════════════════════════════════════════════════
#  PHASE 1: System dependencies
# ═══════════════════════════════════════════════════════════════════════
install_deps() {
    step "Phase 1: System dependencies"

    case "$PKG_MANAGER" in
        pacman)
            info "Installing packages with pacman..."

            # Runtime dependencies
            local RUNTIME_PKGS=(
                # Terminal & launcher
                alacritty
                fuzzel
                # Qt6 (shell, settings, file manager, admin, services, network)
                qt6-base
                qt6-declarative
                qt6-multimedia
                qt6-multimedia-ffmpeg
                qt6-wayland
                qt6-shadertools
                # GTK4 (xarph-lock still uses it)
                gtk4
                gtk4-layer-shell
                # Wayland core
                wayland
                libxkbcommon
                xdg-desktop-portal
                xdg-desktop-portal-gtk
                # Audio/video
                pipewire
                wireplumber
                # Session management
                polkit
                dbus
                systemd
                # Graphics
                mesa
                libglvnd
            )

            # Build dependencies
            local BUILD_PKGS=(
                cargo
                rust
                pkg-config
                clang
                cmake
                # Wayland/compositor build
                libinput
                seatd
                wayland-protocols
                libdrm
                # Graphics build
                pango
                cairo
                glib2
                # Qt6 build
                qt6-base
                qt6-declarative
                qt6-wayland
                qt6-shadertools
            )

            # Combine and deduplicate
            local ALL_PKGS=()
            for pkg in "${RUNTIME_PKGS[@]}" "${BUILD_PKGS[@]}"; do
                local found=0
                for existing in "${ALL_PKGS[@]+"${ALL_PKGS[@]}"}"; do
                    [[ "$existing" == "$pkg" ]] && found=1 && break
                done
                (( found == 0 )) && ALL_PKGS+=("$pkg")
            done

            sudo pacman -S --needed --noconfirm "${ALL_PKGS[@]}"
            ;;

        apt)
            info "Installing packages with apt..."
            sudo apt update
            sudo apt install -y \
                build-essential cargo rustc pkg-config clang cmake \
                libwayland-dev libxkbcommon-dev \
                libinput-dev libseat-dev \
                libpango1.0-dev libcairo2-dev libglib2.0-dev \
                libdrm-dev libgl-dev \
                qt6-base-dev qt6-declarative-dev qt6-wayland-dev \
                libgtk-4-dev libgtk4-layer-shell-dev \
                libpipewire-0.3-dev \
                wireplumber \
                xdg-desktop-portal xdg-desktop-portal-gtk \
                policykit-1 dbus systemd
            ;;

        dnf)
            info "Installing packages with dnf..."
            sudo dnf install -y \
                cargo rustc pkg-config clang cmake \
                wayland-devel libxkbcommon-devel \
                libinput-devel seatd-devel \
                pango-devel cairo-devel glib2-devel \
                libdrm-devel mesa-libGL-devel \
                qt6-base-devel qt6-declarative-devel qt6-wayland-devel \
                gtk4-devel gtk4-layer-shell-devel \
                pipewire-devel wireplumber \
                xdg-desktop-portal xdg-desktop-portal-gtk \
                polkit dbus-broker systemd
            ;;

        *)
            warn "Unknown package manager. Install dependencies manually:"
            echo "  - Qt6: qt6-base, qt6-declarative, qt6-multimedia, qt6-wayland, qt6-shadertools"
            echo "  - GTK4: gtk4, gtk4-layer-shell (for xarph-lock)"
            echo "  - Wayland: wayland, libxkbcommon, wayland-protocols"
            echo "  - Build: cargo, rust, pkg-config, clang, cmake"
            echo "  - Compositor: libinput, seatd, pipewire, libdrm, mesa"
            echo "  - Portal: xdg-desktop-portal, xdg-desktop-portal-gtk"
            ;;
    esac

    ok "System dependencies installed"
}

# ═══════════════════════════════════════════════════════════════════════
#  PHASE 2: Build (incremental — no cargo clean)
# ═══════════════════════════════════════════════════════════════════════
build_all() {
    step "Phase 2: Build (incremental)"

    info "Building ${#BINARIES[@]} release binaries..."

    # Build only the specific binaries we need — incremental by default
    cargo build --release \
        -p xarph-wm \
        -p xarph-shell \
        -p xarph-settings \
        -p xarph-lock \
        -p xarph-admin \
        -p xarph-services \
        -p xarph-network \
        -p Xarhives

    # Verify all binaries exist
    info "Verifying binaries..."
    local missing=0
    for bin in "${BINARIES[@]}"; do
        if [[ ! -f "target/release/$bin" ]]; then
            err "Missing: target/release/$bin"
            missing=1
        fi
    done

    if (( missing )); then
        err "Build failed — missing binaries"
        exit 1
    fi

    ok "All ${#BINARIES[@]} binaries built successfully"
}

# ═══════════════════════════════════════════════════════════════════════
#  PHASE 3: Install
# ═══════════════════════════════════════════════════════════════════════
install_all() {
    step "Phase 3: Install"

    local BINDIR=/usr/bin
    local CONFDIR=/usr/share/xarph/conf
    local SESSDIR=/usr/share/wayland-sessions
    local SYSDDIR=/usr/lib/systemd/user
    local PORTALDIR=/usr/share/xdg-desktop-portal

    # ── Binaries ───────────────────────────────────────────────────────
    info "Installing binaries to ${BINDIR}..."
    for bin in "${BINARIES[@]}"; do
        install -Dm755 "target/release/${bin}" "${BINDIR}/${bin}"
    done

    # Session launcher script (not a compiled binary)
    install -Dm755 "xarph-wm/resources/xarph-session" "${BINDIR}/xarph-session"

    # ── Desktop entry ──────────────────────────────────────────────────
    info "Installing desktop entry..."
    install -Dm644 "xarph-wm/resources/xarph.desktop" "${SESSDIR}/xarph.desktop"

    # ── Systemd units ──────────────────────────────────────────────────
    info "Installing systemd units..."
    install -Dm644 "xarph-wm/resources/xarph-wm.service"          "${SYSDDIR}/xarph-wm.service"
    install -Dm644 "xarph-wm/resources/xarph-wm-shutdown.target"  "${SYSDDIR}/xarph-wm-shutdown.target"
    install -Dm644 "xarph-wm/resources/xarph-shell.service"       "${SYSDDIR}/xarph-shell.service"
    install -Dm644 "data/systemd/user/xarph-session.target"       "${SYSDDIR}/xarph-session.target"

    # ── Default shell config ───────────────────────────────────────────
    info "Installing default shell config..."
    install -d "${CONFDIR}"
    cp -a data/conf/. "${CONFDIR}/"

    # ── XDG portal config ──────────────────────────────────────────────
    info "Installing XDG portal config..."
    install -Dm644 "xarph-wm/resources/xarph-portals.conf" "${PORTALDIR}/xarph-portals.conf"

    # ── Default compositor config (first install only) ─────────────────
    local USER_CONF="${HOME}/.config/xarph/niri-config.kdl"
    if [[ ! -f "${USER_CONF}" ]]; then
        info "Installing default compositor config..."
        install -d "${HOME}/.config/xarph"
        cp "xarph-wm/resources/default-config.kdl" "${USER_CONF}"
    fi

    # ── Reload systemd ─────────────────────────────────────────────────
    info "Reloading systemd..."
    systemctl --user daemon-reload 2>/dev/null || true

    ok "Xarph installed to ${BINDIR}"
}

# ═══════════════════════════════════════════════════════════════════════
#  PHASE 4: Uninstall
# ═══════════════════════════════════════════════════════════════════════
uninstall_all() {
    step "Uninstalling Xarph"

    local BINDIR=/usr/bin
    local CONFDIR=/usr/share/xarph/conf
    local SESSDIR=/usr/share/wayland-sessions
    local SYSDDIR=/usr/lib/systemd/user
    local PORTALDIR=/usr/share/xdg-desktop-portal

    # Stop services if running
    info "Stopping services..."
    systemctl --user stop xarph-shell.service 2>/dev/null || true
    systemctl --user stop xarph-wm.service 2>/dev/null || true

    # Remove binaries
    info "Removing binaries..."
    for bin in "${BINARIES[@]}"; do
        rm -f "${BINDIR}/${bin}"
    done
    rm -f "${BINDIR}/xarph-session"

    # Remove desktop entry
    rm -f "${SESSDIR}/xarph.desktop"

    # Remove systemd units
    info "Removing systemd units..."
    rm -f "${SYSDDIR}/xarph-wm.service"
    rm -f "${SYSDDIR}/xarph-wm-shutdown.target"
    rm -f "${SYSDDIR}/xarph-shell.service"
    rm -f "${SYSDDIR}/xarph-session.target"

    # Remove config
    rm -rf "${CONFDIR}"

    # Remove portal config
    rm -f "${PORTALDIR}/xarph-portals.conf"

    # Reload systemd
    systemctl --user daemon-reload 2>/dev/null || true

    ok "Xarph uninstalled (user data in ~/.config/xarph/ preserved)"
}

# ═══════════════════════════════════════════════════════════════════════
#  PHASE 5: Status
# ═══════════════════════════════════════════════════════════════════════
show_status() {
    step "Xarph Desktop Status"

    echo ""
    echo -e "${BOLD}Binaries:${NC}"
    for bin in "${BINARIES[@]}" xarph-session; do
        local path="/usr/bin/${bin}"
        if [[ -x "$path" ]]; then
            echo -e "  ${GREEN}✓${NC} ${path}"
        else
            echo -e "  ${RED}✗${NC} ${path} (not installed)"
        fi
    done

    echo ""
    echo -e "${BOLD}Systemd units:${NC}"
    for unit in xarph-wm.service xarph-shell.service xarph-wm-shutdown.target xarph-session.target; do
        local path="/usr/lib/systemd/user/${unit}"
        if [[ -f "$path" ]]; then
            echo -e "  ${GREEN}✓${NC} ${path}"
        else
            echo -e "  ${RED}✗${NC} ${path} (not installed)"
        fi
    done

    echo ""
    echo -e "${BOLD}Desktop entry:${NC}"
    if [[ -f "/usr/share/wayland-sessions/xarph.desktop" ]]; then
        echo -e "  ${GREEN}✓${NC} /usr/share/wayland-sessions/xarph.desktop"
    else
        echo -e "  ${RED}✗${NC} /usr/share/wayland-sessions/xarph.desktop (not installed)"
    fi

    echo ""
    echo -e "${BOLD}Config:${NC}"
    if [[ -d "/usr/share/xarph/conf" ]]; then
        echo -e "  ${GREEN}✓${NC} /usr/share/xarph/conf/"
    else
        echo -e "  ${RED}✗${NC} /usr/share/xarph/conf/ (not installed)"
    fi
    if [[ -f "${HOME}/.config/xarph/niri-config.kdl" ]]; then
        echo -e "  ${GREEN}✓${NC} ~/.config/xarph/niri-config.kdl"
    else
        echo -e "  ${YELLOW}~${NC} ~/.config/xarph/niri-config.kdl (not created yet)"
    fi

    echo ""
    echo -e "${BOLD}Services:${NC}"
    if systemctl --user is-active xarph-wm.service &>/dev/null; then
        echo -e "  ${GREEN}●${NC} xarph-wm.service (running)"
    else
        echo -e "  ${RED}○${NC} xarph-wm.service (stopped)"
    fi
    if systemctl --user is-active xarph-shell.service &>/dev/null; then
        echo -e "  ${GREEN}●${NC} xarph-shell.service (running)"
    else
        echo -e "  ${RED}○${NC} xarph-shell.service (stopped)"
    fi
}

# ═══════════════════════════════════════════════════════════════════════
#  Main
# ═══════════════════════════════════════════════════════════════════════
main() {
    echo -e "${BOLD}═══════════════════════════════════════════════════════${NC}"
    echo -e "${BOLD}  Xarph Desktop — Setup${NC}"
    echo -e "${BOLD}═══════════════════════════════════════════════════════${NC}"

    case "${1:-}" in
        --deps)
            install_deps
            ;;
        --build)
            build_all
            ;;
        --install)
            install_all
            ;;
        --uninstall)
            uninstall_all
            ;;
        --status)
            show_status
            ;;
        "")
            # Default: deps + build + install
            install_deps
            build_all
            install_all
            echo ""
            echo -e "${GREEN}${BOLD}Done!${NC} Select 'Xarph' from your display manager,"
            echo -e "or run ${CYAN}xarph-session${NC} from a TTY."
            ;;
        *)
            echo "Usage: $0 [--deps|--build|--install|--uninstall|--status]"
            echo ""
            echo "  (no args)    Install deps + build + install"
            echo "  --deps       Install system dependencies only"
            echo "  --build      Build binaries only (incremental)"
            echo "  --install    Install files only (skip build)"
            echo "  --uninstall  Remove all Xarph files"
            echo "  --status     Show what's installed"
            exit 1
            ;;
    esac
}

main "$@"
