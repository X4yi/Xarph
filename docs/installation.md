# X4Shell Installation Guide

## Prerequisites

### Required Dependencies
- **Hyprland** - Wayland compositor
- **Quickshell** - QML runtime
- **systemd** - Service management
- **D-Bus** - IPC communication
- **Rust & Cargo** - For compiling the daemon

### Optional
- **git** - For updates via `setup.sh update`

## Installation Methods

### Method 1: Interactive (Recommended)
```bash
cd X4Shell/
./setup.sh
# Then select option [1] Install
```

### Method 2: Direct Command
```bash
./setup.sh install
```

### Method 3: Dry Run (Test without changes)
```bash
./setup.sh --dry-run install
```

## What Gets Installed

### 1. Daemon Binary
- Compiles from `daemon/` directory
- Installs to `/usr/local/bin/x4shell-daemon` (system) or `~/.local/bin/x4shell-daemon` (user)

### 2. UI Files
- Copies `ui/` to `~/.local/share/x4-shell/ui/`

### 3. Configuration
- Creates `~/.config/x4-shell/` with:
  - `hypr/hyprland.conf` - Hyprland config
  - `shell/settings.json` - Daemon settings
- Creates `~/.local/share/x4-shell/` for runtime data
- Creates `~/.cache/x4-shell/` for cache

### 4. Session Integration
- Installs `/usr/local/bin/x4-shell-session` script
- Installs `/usr/share/wayland-sessions/x4-shell.desktop` for display managers

### 5. Systemd Service
- Creates `~/.config/systemd/user/x4-shell-daemon.service`
- Enables and starts the service

## Post-Installation

1. **Logout and select "X4 Shell"** from your display manager
2. Or run manually: `/usr/local/bin/x4-shell-session`

## Repair Installation
```bash
./setup.sh repair
```

## Update Installation
```bash
./setup.sh update
```

## Uninstall
```bash
# Keep config files
./setup.sh uninstall

# Delete everything (purge)
./setup.sh uninstall --purge
```

## Troubleshooting

### Daemon not starting?
```bash
./setup.sh status
journalctl --user -u x4-shell-daemon.service
```

### UI not showing?
Check if Quickshell is installed: `which quickshell`

### Hyprland config issues?
Reset to default: `./setup.sh repair` (use `--force` to overwrite configs)
