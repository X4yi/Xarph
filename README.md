# X4Shell

A custom Wayland desktop shell built on top of Hyprland, featuring a Rust-based daemon backend and a QML-based UI frontend.

> **Status**: Early prototype - Not ready for production use.

## Architecture

```
X4Shell/
├── setup.sh          # Main setup script (install/repair/update/uninstall)
├── daemon/           # Rust backend daemon (x4shell-daemon)
├── ui/               # QML frontend (Quickshell)
├── xgit/             # Git helper tool (Bash)
├── config/           # Configuration templates and defaults
│   ├── hyprland/    # Hyprland configs
│   ├── daemon/       # Daemon settings
│   ├── systemd/      # Systemd service templates
│   ├── session/      # Session scripts and .desktop files
│   └── defaults/     # Default user configs
└── docs/             # Documentation
```

For detailed architecture information, see [docs/architecture.md](docs/architecture.md).

## Features (So Far)

### Backend (daemon/)
- Connection to Hyprland socket via tokio
- Event parsing: workspace, windowOpen, windowClose, monitorAdded, monitorRemoved
- D-Bus session bus interface at `/org/x4yi/X4Shell/v1`
- Shared state management with `ArcStateStore`
- Graceful shutdown via SIGINT/SIGTERM

### UI (ui/)
- QML-based interface with Quickshell
- Workspace management components
- System indicators and clock
- Themeable (see `ui/themes/`)
- D-Bus service integration

### Setup Script (setup.sh)
- **Install**: Full system setup with dependency checking
- **Repair**: Fix broken installations
- **Update**: Pull latest changes and recompile
- **Uninstall**: Remove X4Shell (with `--purge` option)
- **Status**: Check installation health
- Colored CLI with interactive menu

## Quick Start

### Installation
```bash
cd X4Shell/
./setup.sh install
```

Or use the interactive menu:
```bash
./setup.sh
```

### Post-Install
1. Logout and select **"X4 Shell"** from your display manager (GDM, SDDM, etc.)
2. Or run manually: `/usr/local/bin/x4-shell-session`

## Requirements

- **Hyprland** (window manager)
- **Quickshell** (QML runtime)
- **systemd** (service management)
- **D-Bus** (IPC)
- **Rust & Cargo** (to compile the daemon)

## Documentation

- [Installation Guide](docs/installation.md)
- [Architecture Overview](docs/architecture.md)

## Development

### Build Daemon Only
```bash
cd daemon/
cargo build --release
```

### Run Daemon Manually
```bash
./daemon/target/release/x4shell-daemon
```

### Test D-Bus Interface
```bash
busctl --user call org.x4yi.X4Shell.v1 /org/x4yi/X4Shell/v1 org.x4yi.X4Shell.v1 Ping
```

## Setup Script Usage

```bash
./setup.sh [command] [options]

Commands:
  install     Full system setup
  repair      Fix broken installations
  update      Update to latest version
  uninstall   Remove X4Shell (add --purge to delete config/data)
  status      Check installation health

Options:
  --dry-run   Simulate without making changes
  --verbose   Show debug output
  --force     Overwrite existing configs
  --purge     Delete all config and data (with uninstall)
```

## Known Issues

- End-to-end sync between daemon and UI is incomplete
- UI may not listen to D-Bus signals correctly
- `SwitchWorkspace` doesn't verify response
- Some components are placeholders (e.g., WindowService, config loader)

## License

(Add your license here)

## Contributing

This is a personal project in early stages. Feel free to fork and experiment!
