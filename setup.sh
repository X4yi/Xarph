

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CONFIG_DIR="$SCRIPT_DIR/config"
DAEMON_DIR="$SCRIPT_DIR/daemon"
UI_DIR="$SCRIPT_DIR/ui"

DAEMON_BIN_SYSTEM="/usr/local/bin/x4shell-daemon"
DAEMON_BIN_USER="$HOME/.local/bin/x4shell-daemon"
SESSION_SCRIPT_PATH="/usr/local/bin/x4-shell-session"
DESKTOP_FILE_PATH="/usr/share/wayland-sessions/x4-shell.desktop"
SYSTEMD_USER_DIR="$HOME/.config/systemd/user"
X4SHELL_CONFIG_DIR="$HOME/.config/x4-shell"
X4SHELL_DATA_DIR="$HOME/.local/share/x4-shell"
X4SHELL_CACHE_DIR="$HOME/.cache/x4-shell"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
GRAY='\033[0;90m'
BOLD='\033[1m'
NC='\033[0m'

log_info()    { echo -e "${CYAN}[INFO]${NC} $*"; }
log_success() { echo -e "${GREEN}[OK]${NC} $*"; }
log_warn()    { echo -e "${YELLOW}[WARN]${NC} $*"; }
log_error()   { echo -e "${RED}[ERROR]${NC} $*" >&2; }
log_debug()   { [[ "${VERBOSE:-false}" == "true" ]] && echo -e "${GRAY}[DEBUG]${NC} $*" >&2 || true; }

print_header() {
    echo -e "${CYAN}${BOLD}"
    echo "╔════════════════════════════════════════════════════════════╗"
    echo "║                    X4Shell Setup v1.0                       ║"
    echo "║              Hyprland-based Wayland Shell Manager            ║"
    echo "╚════════════════════════════════════════════════════════════╝"
    echo -e "${NC}"
}

print_menu() {
    print_header
    echo -e "${BOLD}Select an option:${NC}"
    echo ""
    echo -e "  ${CYAN}[1]${NC} Install      - Full system setup"
    echo -e "  ${CYAN}[2]${NC} Repair       - Fix broken installations"
    echo -e "  ${CYAN}[3]${NC} Update       - Update to latest version"
    echo -e "  ${CYAN}[4]${NC} Uninstall    - Remove X4Shell (--purge for full wipe)"
    echo -e "  ${CYAN}[5]${NC} Status       - Check installation health"
    echo ""
    echo -e "  ${RED}[q]${NC} Quit"
    echo ""
    echo -n "Enter option [1-5/q]: "
}

print_separator() {
    echo -e "${GRAY}────────────────────────────────────────────────────────────${NC}"
}

spinner() {
    local pid=$1
    local msg="${2:-Processing...}"
    local spin='⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏'
    local i=0
    
    while kill -0 "$pid" 2>/dev/null; do
        printf "\r${CYAN}[${spin:$i:1}]${NC} $msg"
        ((i = (i + 1) % ${#spin}))
        sleep 0.1
    done
    printf "\r${GREEN}[✓]${NC} $msg\n"
}

cleanup() {
    log_warn "Interrupted. Cleaning up..."
    exit 130
}
trap cleanup SIGINT SIGTERM

error_handler() {
    local exit_code=$?
    log_error "An error occurred (exit code: $exit_code)"
    log_error "Line: ${BASH_LINENO[-2]}, Command: '${BASH_COMMAND}'"
    exit $exit_code
}
trap 'error_handler' ERR

SUDO_PASSWORD=""
HAS_SUDO=false

check_sudo() {
    if [[ $EUID -eq 0 ]]; then
        HAS_SUDO=true
        return
    fi
    
    if sudo -n true 2>/dev/null; then
        HAS_SUDO=true
        return
    fi
    
    HAS_SUDO=false
}

get_sudo_password() {
    if [[ $HAS_SUDO == true ]]; then
        return
    fi
    
    read -s -p "Enter sudo password (or press Enter to skip system-wide install): " SUDO_PASSWORD
    echo ""
    
    if [[ -z "$SUDO_PASSWORD" ]]; then
        log_warn "No sudo password provided. Will install user-local only."
        return
    fi
    
    if echo "$SUDO_PASSWORD" | sudo -S true 2>/dev/null; then
        HAS_SUDO=true
        log_success "Sudo authenticated"
    else
        log_error "Invalid sudo password"
        SUDO_PASSWORD=""
        HAS_SUDO=false
    fi
}

run_sudo() {
    if [[ $HAS_SUDO == true ]]; then
        sudo "$@"
    elif [[ -n "$SUDO_PASSWORD" ]]; then
        echo "$SUDO_PASSWORD" | sudo -S "$@"
    else
        log_error "No sudo access. Cannot run: $*"
        return 1
    fi
}

process_template() {
    local input="$1"
    local output="$2"
    local daemon_path="${3:-$DAEMON_BIN_SYSTEM}"
    local ui_path="${4:-$X4SHELL_DATA_DIR/ui}"
    local session_script_path="${5:-$SESSION_SCRIPT_PATH}"
    
    log_debug "Processing template: $input → $output"
    
    sed -e "s|@DAEMON_PATH@|$daemon_path|g" \
         -e "s|@UI_PATH@|$ui_path|g" \
         -e "s|@SESSION_SCRIPT_PATH@|$session_script_path|g" \
         -e "s|@SESSION_BUS@|session|g" \
         "$input" > "$output"
}

check_dependencies() {
    local missing=()
    
    log_info "Checking dependencies..."
    
    for cmd in hyprland quickshell systemctl dbus-daemon; do
        if ! command -v "$cmd" &>/dev/null; then
            missing+=("$cmd")
        fi
    done
    
    if [[ ! -x "$DAEMON_BIN_SYSTEM" && ! -x "$DAEMON_BIN_USER" ]]; then
        if ! command -v cargo &>/dev/null; then
            missing+=("rust/cargo (needed to compile daemon)")
        fi
    fi
    
    if [[ ${#missing[@]} -gt 0 ]]; then
        log_error "Missing dependencies:"
        for dep in "${missing[@]}"; do
            echo -e "  ${RED}✗${NC} $dep"
        done
        return 1
    fi
    
    log_success "All dependencies satisfied"
    return 0
}


do_install() {
    local dry_run="${1:-false}"
    
    print_separator
    log_info "Starting X4Shell installation..."
    [[ $dry_run == "true" ]] && log_warn "DRY RUN MODE - No changes will be made"
    print_separator
    
    if ! check_dependencies; then
        if [[ $dry_run != "true" ]]; then
            log_error "Cannot continue without dependencies"
            return 1
        fi
    fi
    
    if [[ ! -x "$DAEMON_DIR/target/release/x4shell-daemon" ]]; then
        log_info "Compiling daemon..."
        if [[ $dry_run != "true" ]]; then
            (cd "$DAEMON_DIR" && cargo build --release) &
            spinner $! "Compiling x4shell-daemon"
            wait $!
        else
            log_info "[dry-run] Would compile daemon with: cd $DAEMON_DIR && cargo build --release"
        fi
    fi
    
    local daemon_src="$DAEMON_DIR/target/release/x4shell-daemon"
    local daemon_installed=false
    
    if [[ -x "$daemon_src" ]]; then
        if [[ $HAS_SUDO == true ]]; then
            log_info "Installing daemon to $DAEMON_BIN_SYSTEM..."
            [[ $dry_run != "true" ]] && run_sudo cp "$daemon_src" "$DAEMON_BIN_SYSTEM" && run_sudo chmod 755 "$DAEMON_BIN_SYSTEM/x4shell-daemon"
            daemon_installed=true
        fi
        
        if [[ $daemon_installed != true ]]; then
            mkdir -p "$HOME/.local/bin"
            log_info "Installing daemon to $DAEMON_BIN_USER..."
            [[ $dry_run != "true" ]] && cp "$daemon_src" "$DAEMON_BIN_USER" && chmod 755 "$DAEMON_BIN_USER"
            daemon_installed=true
        fi
    fi
    
    log_info "Creating directories..."
    if [[ $dry_run != "true" ]]; then
        mkdir -p "$X4SHELL_CONFIG_DIR/hypr"
        mkdir -p "$X4SHELL_CONFIG_DIR/shell"
        mkdir -p "$X4SHELL_DATA_DIR/ui"
        mkdir -p "$X4SHELL_CACHE_DIR"
        mkdir -p "$SYSTEMD_USER_DIR"
    fi
    
    log_info "Installing configuration files..."
    if [[ $dry_run != "true" ]]; then
        if [[ ! -f "$X4SHELL_CONFIG_DIR/hypr/hyprland.conf" ]] || [[ "${FORCE:-false}" == "true" ]]; then
            cp "$CONFIG_DIR/hyprland/hyprland.conf" "$X4SHELL_CONFIG_DIR/hypr/hyprland.conf"
        fi
        
        if [[ ! -f "$X4SHELL_CONFIG_DIR/shell/settings.json" ]] || [[ "${FORCE:-false}" == "true" ]]; then
            process_template "$CONFIG_DIR/daemon/settings.json" "$X4SHELL_CONFIG_DIR/shell/settings.json"
        fi
    fi
    
    log_info "Installing UI files..."
    if [[ $dry_run != "true" ]]; then
        cp -r "$UI_DIR/"* "$X4SHELL_DATA_DIR/ui/" 2>/dev/null || true
    fi
    
    log_info "Installing session script..."
    if [[ $dry_run != "true" ]]; then
        local installed_daemon_path="$DAEMON_BIN_SYSTEM"
        [[ -x "$DAEMON_BIN_USER" ]] && installed_daemon_path="$DAEMON_BIN_USER"
        
        process_template "$CONFIG_DIR/session/x4-shell-session" "/tmp/x4-shell-session.tmp" "$installed_daemon_path" "$X4SHELL_DATA_DIR/ui" "$SESSION_SCRIPT_PATH"
        
        if [[ $HAS_SUDO == true ]]; then
            run_sudo cp "/tmp/x4-shell-session.tmp" "$SESSION_SCRIPT_PATH"
            run_sudo chmod 755 "$SESSION_SCRIPT_PATH"
        fi
        rm -f "/tmp/x4-shell-session.tmp"
    fi
    
    log_info "Installing desktop file..."
    if [[ $dry_run != "true" ]]; then
        if [[ $HAS_SUDO == true ]]; then
            process_template "$CONFIG_DIR/session/x4-shell.desktop" "/tmp/x4-shell.desktop" "" "" "$SESSION_SCRIPT_PATH"
            run_sudo cp "/tmp/x4-shell.desktop" "$DESKTOP_FILE_PATH"
            rm -f "/tmp/x4-shell.desktop"
        fi
    fi
    
    log_info "Installing systemd service..."
    if [[ $dry_run != "true" ]]; then
        local installed_daemon_path="$DAEMON_BIN_SYSTEM"
        [[ -x "$DAEMON_BIN_USER" ]] && installed_daemon_path="$DAEMON_BIN_USER"
        
        process_template "$CONFIG_DIR/systemd/x4-shell-daemon.service" "$SYSTEMD_USER_DIR/x4-shell-daemon.service" "$installed_daemon_path"
        
        systemctl --user daemon-reload 2>/dev/null || true
        systemctl --user enable x4-shell-daemon.service 2>/dev/null || true
        systemctl --user start x4-shell-daemon.service 2>/dev/null || true
    fi
    
    print_separator
    log_success "Installation complete!"
    log_info "You can now select 'X4 Shell' from your display manager or run: $SESSION_SCRIPT_PATH"
    print_separator
}

do_repair() {
    print_separator
    log_info "Starting repair..."
    print_separator
    
    local needs_repair=false
    
    for dir in "$X4SHELL_CONFIG_DIR" "$X4SHELL_CONFIG_DIR/hypr" "$X4SHELL_DATA_DIR" "$X4SHELL_CACHE_DIR"; do
        if [[ ! -d "$dir" ]]; then
            log_warn "Missing directory: $dir"
            mkdir -p "$dir"
            needs_repair=true
        fi
    done
    
    if [[ ! -f "$X4SHELL_CONFIG_DIR/hypr/hyprland.conf" ]]; then
        log_warn "Missing hyprland.conf, regenerating..."
        cp "$CONFIG_DIR/hyprland/hyprland.conf" "$X4SHELL_CONFIG_DIR/hypr/hyprland.conf"
        needs_repair=true
    fi
    
    if [[ ! -f "$X4SHELL_CONFIG_DIR/shell/settings.json" ]]; then
        log_warn "Missing settings.json, regenerating..."
        process_template "$CONFIG_DIR/daemon/settings.json" "$X4SHELL_CONFIG_DIR/shell/settings.json"
        needs_repair=true
    fi
    
    if [[ ! -x "$DAEMON_BIN_SYSTEM" && ! -x "$DAEMON_BIN_USER" ]]; then
        log_warn "Daemon binary not found, will reinstall..."
        needs_repair=true
    fi
    
    if [[ ! -f "$SESSION_SCRIPT_PATH" ]]; then
        log_warn "Session script missing, will reinstall..."
        needs_repair=true
    fi
    
    if [[ ! -f "$DESKTOP_FILE_PATH" ]]; then
        log_warn "Desktop file missing, will reinstall..."
        needs_repair=true
    fi
    
    if [[ ! -f "$SYSTEMD_USER_DIR/x4-shell-daemon.service" ]]; then
        log_warn "Systemd service missing, will reinstall..."
        needs_repair=true
    fi
    
    if [[ $needs_repair == true ]]; then
        log_info "Repairing installation..."
        FORCE=true do_install false
    else
        log_success "No repairs needed, installation looks good!"
    fi
    
    log_info "Restarting daemon service..."
    systemctl --user restart x4-shell-daemon.service 2>/dev/null || true
    
    print_separator
    log_success "Repair complete!"
    print_separator
}

do_update() {
    print_separator
    log_info "Starting update..."
    print_separator
    
    log_info "Pulling latest changes..."
    if git -C "$SCRIPT_DIR" pull origin main 2>/dev/null; then
        log_success "Repository updated"
    else
        log_warn "Git pull failed or not a git repo, continuing..."
    fi
    
    if [[ -f "$DAEMON_DIR/Cargo.toml" ]]; then
        log_info "Recompiling daemon..."
        (cd "$DAEMON_DIR" && cargo build --release) &
        spinner $! "Recompiling x4shell-daemon"
        wait $!
    fi
    
    FORCE=true do_install false
    
    print_separator
    log_success "Update complete!"
    print_separator
}

do_uninstall() {
    local purge="${1:-false}"
    
    print_separator
    log_warn "Starting uninstallation..."
    [[ $purge == "true" ]] && log_warn "PURGE mode: Will delete ALL config and data!"
    print_separator
    
    log_info "Stopping and disabling systemd service..."
    systemctl --user stop x4-shell-daemon.service 2>/dev/null || true
    systemctl --user disable x4-shell-daemon.service 2>/dev/null || true
    rm -f "$SYSTEMD_USER_DIR/x4-shell-daemon.service"
    systemctl --user daemon-reload 2>/dev/null || true
    
    log_info "Removing session files..."
    if [[ $HAS_SUDO == true ]]; then
        run_sudo rm -f "$SESSION_SCRIPT_PATH"
        run_sudo rm -f "$DESKTOP_FILE_PATH"
    fi
    
    log_info "Removing daemon binary..."
    rm -f "$DAEMON_BIN_USER"
    if [[ $HAS_SUDO == true ]]; then
        run_sudo rm -f "$DAEMON_BIN_SYSTEM"
    fi
    
    if [[ $purge == "true" ]]; then
        log_warn "Purging config and data directories..."
        rm -rf "$X4SHELL_CONFIG_DIR"
        rm -rf "$X4SHELL_DATA_DIR"
        rm -rf "$X4SHELL_CACHE_DIR"
    else
        log_info "Keeping config and data (use --purge to delete)"
    fi
    
    print_separator
    log_success "Uninstallation complete!"
    print_separator
}

do_status() {
    print_separator
    log_info "Checking X4Shell installation status..."
    print_separator
    
    local all_good=true
    
    if systemctl --user is-active --quiet x4-shell-daemon.service 2>/dev/null; then
        log_success "Daemon service: RUNNING"
    else
        log_warn "Daemon service: NOT RUNNING"
        all_good=false
    fi
    
    if [[ -x "$DAEMON_BIN_SYSTEM" ]]; then
        log_success "Daemon binary (system): FOUND at $DAEMON_BIN_SYSTEM"
    elif [[ -x "$DAEMON_BIN_USER" ]]; then
        log_success "Daemon binary (user): FOUND at $DAEMON_BIN_USER"
    else
        log_error "Daemon binary: NOT FOUND"
        all_good=false
    fi
    
    if [[ -f "$SESSION_SCRIPT_PATH" ]]; then
        log_success "Session script: FOUND"
    else
        log_warn "Session script: NOT FOUND"
        all_good=false
    fi
    
    if [[ -f "$DESKTOP_FILE_PATH" ]]; then
        log_success "Desktop file: FOUND"
    else
        log_warn "Desktop file: NOT FOUND"
        all_good=false
    fi
    
    if [[ -f "$SYSTEMD_USER_DIR/x4-shell-daemon.service" ]]; then
        log_success "Systemd service: INSTALLED"
    else
        log_warn "Systemd service: NOT INSTALLED"
        all_good=false
    fi
    
    if [[ -f "$X4SHELL_CONFIG_DIR/hypr/hyprland.conf" ]]; then
        log_success "Hyprland config: FOUND"
    else
        log_warn "Hyprland config: NOT FOUND"
        all_good=false
    fi
    
    if [[ -f "$X4SHELL_CONFIG_DIR/shell/settings.json" ]]; then
        log_success "Shell config: FOUND"
    else
        log_warn "Shell config: NOT FOUND"
        all_good=false
    fi
    
    if [[ -d "$X4SHELL_DATA_DIR/ui" ]]; then
        log_success "UI files: INSTALLED"
    else
        log_warn "UI files: NOT FOUND"
        all_good=false
    fi
    
    if command -v busctl &>/dev/null; then
        if busctl --user call org.x4yi.X4Shell.v1 /org/x4yi/X4Shell/v1 org.x4yi.X4Shell.v1 Ping 2>/dev/null | grep -q "u"; then
            log_success "D-Bus connectivity: OK"
        else
            log_warn "D-Bus connectivity: FAILED (daemon may not be running)"
            all_good=false
        fi
    fi
    
    print_separator
    if [[ $all_good == true ]]; then
        log_success "All checks passed! X4Shell is properly installed."
    else
        log_warn "Some checks failed. Run 'repair' to fix issues."
    fi
    print_separator
}

main() {
    local action=""
    local dry_run=false
    local verbose=false
    local force=false
    
    while [[ $# -gt 0 ]]; do
        case $1 in
            install|repair|update|uninstall|status)
                action="$1"
                shift
                ;;
            --dry-run)
                dry_run=true
                shift
                ;;
            --verbose|-v)
                verbose=true
                shift
                ;;
            --force|-f)
                force=true
                shift
                ;;
            --purge)
                shift
                ;;
            -h|--help)
                print_header
                echo "Usage: $0 [option] [command]"
                echo ""
                echo "Commands:"
                echo "  install     Full system setup"
                echo "  repair      Fix broken installations"
                echo "  update      Update to latest version"
                echo "  uninstall   Remove X4Shell (add --purge to delete config/data)"
                echo "  status      Check installation health"
                echo ""
                echo "Options:"
                echo "  --dry-run   Simulate without making changes"
                echo "  --verbose   Show debug output"
                echo "  --force     Overwrite existing configs"
                echo "  --purge     Delete all config and data (with uninstall)"
                exit 0
                ;;
            *)
                log_error "Unknown option: $1"
                exit 1
                ;;
        esac
    done
    
    export VERBOSE=$verbose
    export FORCE=$force
    
    check_sudo
    
    if [[ -z "$action" ]]; then
        while true; do
            print_menu
            read -r option
            case $option in
                1) do_install $dry_run ;;
                2) do_repair ;;
                3) do_update ;;
                4) 
                    read -p "Purge all config and data? [y/N]: " purge_confirm
                    [[ "$purge_confirm" =~ ^[Yy]$ ]] && do_uninstall true || do_uninstall false
                    ;;
                5) do_status ;;
                q|Q) echo "Goodbye!"; exit 0 ;;
                *) log_error "Invalid option" ;;
            esac
            echo ""
            read -p "Press Enter to continue..."
            clear
        done
    else
        case $action in
            install) do_install $dry_run ;;
            repair) do_repair ;;
            update) do_update ;;
            uninstall) 
                if [[ "${PURGE:-false}" == "true" ]]; then
                    do_uninstall true
                else
                    do_uninstall false
                fi
                ;;
            status) do_status ;;
        esac
    fi
}

export PURGE=false
for arg in "$@"; do
    [[ "$arg" == "--purge" ]] && export PURGE=true
done

main "$@"
