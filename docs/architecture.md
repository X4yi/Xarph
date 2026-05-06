# X4Shell Architecture

## Overview
X4Shell is a custom Wayland desktop shell built on top of Hyprland, consisting of three main components:

```
┌─────────────────────────────────────────────────────┐
│                    Display Manager                   │
│                   (GDM, SDDM, etc)                 │
└──────────────────┬──────────────────────────────────┘
                   │ starts
┌──────────────────▼──────────────────────────────────┐
│              x4-shell-session script                │
│  (starts daemon, UI, and Hyprland)                │
└─────┬──────────────┬───────────────┬──────────────┘
      │              │               │
┌─────▼─────┐ ┌────▼─────┐ ┌─────▼──────┐
│  Daemon   │ │    UI    │ │  Hyprland  │
│ (Rust)    │ │ (QML)   │ │ (Wayland)  │
└─────┬─────┘ └────┬─────┘ └─────┬──────┘
      │              │               │
      └────── D-Bus ─┼──────────────┘
                (IPC communication)
```

## Components

### 1. Daemon (`daemon/`)
- **Language**: Rust
- **Purpose**: Backend service managing workspaces, windows, and system state
- **Communication**: D-Bus session bus
- **Key Files**:
  - `src/main.rs` - Entry point
  - `src/core/` - Core types and traits
  - `src/services/` - Workspace and window services
  - `src/adapters/` - Hyprland socket adapter
  - `src/ipc/` - D-Bus server

### 2. UI (`ui/`)
- **Language**: QML (Qt Quick)
- **Runtime**: Quickshell
- **Purpose**: User interface panels, widgets, and themes
- **Key Files**:
  - `main.qml` - Entry point
  - `panels/` - UI panels (sidebar, etc.)
  - `components/` - Reusable components
  - `services/` - D-Bus service clients
  - `themes/` - Visual themes

### 3. Setup Script (`setup.sh`)
- **Language**: Bash
- **Purpose**: Installation, repair, update, and uninstallation
- **Features**: Colored CLI, template processing, systemd integration

### 4. Configuration (`config/`)
- **Purpose**: All templates and defaults for X4Shell
- **Subfolders**:
  - `hyprland/` - Hyprland configs
  - `daemon/` - Daemon settings
  - `systemd/` - Service files
  - `session/` - Session scripts and .desktop files
  - `ui/` - UI themes and layouts
  - `defaults/` - Default user configs

## Data Flow

1. **User logs in** → Display manager starts `x4-shell-session`
2. **Session script** starts:
   - `x4shell-daemon` (background)
   - `quickshell` with UI (background)
   - `Hyprland` (foreground, takes over session)
3. **Daemon** connects to Hyprland socket, exposes D-Bus interface
4. **UI** connects to daemon via D-Bus, displays workspace info, etc.

## D-Bus Interface

- **Bus**: Session bus
- **Service**: `org.x4yi.X4Shell.v1`
- **Path**: `/org/x4yi/X4Shell/v1`

### Methods:
- `GetWorkspaces() -> Vec<WorkspaceDBus>`
- `SwitchWorkspace(id: u32) -> ()`
- `Ping() -> u64` (keepalive)

### Signals:
- `WorkspaceChanged(WorkspaceDBus)`

## File Locations

| Component | User Install | System Install |
|------------|--------------|----------------|
| Daemon binary | `~/.local/bin/x4shell-daemon` | `/usr/local/bin/x4shell-daemon` |
| UI files | `~/.local/share/x4-shell/ui/` | - |
| Config | `~/.config/x4-shell/` | - |
| Data | `~/.local/share/x4-shell/` | - |
| Cache | `~/.cache/x4-shell/` | - |
| Systemd service | `~/.config/systemd/user/` | - |
| Session script | - | `/usr/local/bin/x4-shell-session` |
| Desktop file | - | `/usr/share/wayland-sessions/` |

## Build & Install

```bash
# Build daemon
cd daemon/
cargo build --release

# Install/Setup
./setup.sh install
```
