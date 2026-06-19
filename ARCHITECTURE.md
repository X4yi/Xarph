# Xarph Desktop Environment - Architecture

## Overview

Xarph is a custom desktop environment built with **Rust** and **Qt6/QML**, designed for Wayland compositors. It uses `cxx-qt` for Rust↔Qt6 integration, providing a safe and performant bridge between the two languages.

## Core Principles

- **Rust + Qt6 only**: No GTK4, no libadwaita, no glib/gio in UI crates
- **cxx-qt** for Rust↔Qt6 integration
- **QML for presentation**: All UI in QML, all logic in Rust
- **DesktopRegistry** as single source of truth for all visible desktop entities
- **Modular architecture**: Each component is a separate crate

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                     Qt6/QML Layer                           │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐       │
│  │  Shell   │ │ Settings │ │  Files   │ │Process   │       │
│  │  (QML)   │ │  (QML)   │ │  (QML)   │ │Admin(QML)│       │
│  └────┬─────┘ └────┬─────┘ └────┬─────┘ └────┬─────┘       │
│       │             │            │             │              │
│  ┌────┴─────────────┴────────────┴─────────────┴─────┐      │
│  │              cxx-qt Bridges (Rust)                 │      │
│  │  desktop_bridge, panel_bridge, wallpaper_bridge,   │      │
│  │  workspace_bridge, start_menu_bridge, tray_bridge, │      │
│  │  context_menu_bridge, settings_bridge,             │      │
│  │  file_browser_bridge, process_bridge,              │      │
│  │  service_bridge, network_bridge                    │      │
│  └───────────────────────┬───────────────────────────┘      │
│                          │                                   │
├──────────────────────────┼──────────────────────────────────┤
│                     Rust Layer                               │
│  ┌───────────────────────┴───────────────────────────┐      │
│  │                  xarph-sdk                        │      │
│  │  position, desktop_object, desktop_registry,      │      │
│  │  entity_provider, context_menu, wallpaper_engine, │      │
│  │  app_registry, panel_model, workspace_model,      │      │
│  │  clock_service, process_collector,                │      │
│  │  service_manager, network_monitor,                │      │
│  │  settings_model, notification_service             │      │
│  └───────────────────────┬───────────────────────────┘      │
│                          │                                   │
├──────────────────────────┼──────────────────────────────────┤
│                     System Layer                             │
│  ┌───────────────────────┴───────────────────────────┐      │
│  │  wayland-client, smithay-client-toolkit            │      │
│  │  (Layer Shell, Wayland protocols)                  │      │
│  └───────────────────────────────────────────────────┘      │
└─────────────────────────────────────────────────────────────┘
```

## Crate Structure

### Core Crates

| Crate | Description |
|-------|-------------|
| `xarph-sdk` | Shared library: data models, services, IPC, config |
| `xarph-wm` | Wayland compositor (smithay-based) |
| `xarph-lock` | Lock screen (Wayland-native, no GTK) |

### UI Crates (Qt6/QML)

| Crate | Description |
|-------|-------------|
| `xarph-shell` | Desktop shell: panel, desktop, wallpaper, start menu |
| `xarph-settings` | Settings application: wallpaper, theme, panel |
| `xarph-files` | File manager: navigation, file operations |
| `xarph-process-admin` | Process manager: view/kill processes |
| `xarph-services` | Service manager: systemd services |
| `xarph-network` | Network monitor: interfaces, stats |

### Legacy (Not in workspace)

| Crate | Status |
|-------|--------|
| `niri-visual-tests` | Testing tool with GTK4 (not migrated yet) |

## Key Components

### DesktopRegistry (`xarph-sdk/src/desktop_registry.rs`)

Single source of truth for all visible desktop entities.

- `DesktopObject` with `ObjectData` enum (11 variants: File, Folder, Application, Project, Shortcut, Widget, Workspace, Volume, Recent, Trash, Network)
- `DesktopObjectPosition` (workspace_id, container_id, zone, x/y, width/height, z_index, anchor)
- Persistence to JSON
- Snapshot/restore for workspace changes

### cxx-qt Bridges (`xarph-shell/src/bridges/`)

Rust↔Qt6 integration layer:

- `desktop_bridge.rs` — Desktop object model for QML
- `panel_bridge.rs` — Panel widget configuration
- `wallpaper_bridge.rs` — Wallpaper engine control
- `workspace_bridge.rs` — Workspace IPC events
- `start_menu_bridge.rs` — App registry search/launch
- `tray_bridge.rs` — System tray items
- `context_menu_bridge.rs` — Context menu items

### QML Components (`xarph-shell/qml/`)

- `main.qml` — Shell root (WallpaperLayer + Desktop + Panel + StartMenu + ContextMenu)
- `WallpaperLayer.qml` — Image/color/video wallpaper display
- `Panel.qml` — Top panel with Start/Clock/Tray/Workspaces
- `Desktop.qml` — Desktop objects container
- `DesktopObject.qml` — Individual desktop object (icon + label + drag + context menu)
- `StartButton.qml` — Hamburger menu button
- `ClockWidget.qml` — Clock + date display
- `WorkspaceWidget.qml` — Workspace buttons
- `TrayWidget.qml` — System tray items

## Data Flow

```
User Action (QML)
    ↓
cxx-qt Bridge (Rust method call)
    ↓
xarph-sdk Service (business logic)
    ↓
System (IPC/filesystem/Wayland)
    ↓
State Update (property change)
    ↓
QML Re-render (automatic via cxx-qt bindings)
```

## Build System

- **Rust**: `cargo build --workspace`
- **Qt6**: 6.11.1 (system packages)
- **cxx-qt**: 0.8 (workspace dependency)
- **cxx-qt-build**: 0.8 (build-time code generation)

## Dependencies

### Qt6 Packages (Arch Linux)

```
qt6-base qt6-declarative qt6-multimedia qt6-wayland qt6-shadertools qt6-multimedia-ffmpeg
```

### Rust Crates (key)

```
cxx-qt = "0.8"
cxx-qt-lib = "0.8"
freedesktop-desktop-entry = "0.8"
wayland-client = "0.31"
smithay-client-toolkit = "0.19"
```

## Configuration

Config file: `~/.config/xarph/shell.conf` (JSON)

- Wallpaper settings (per-workspace)
- Panel configuration (position, widgets)
- Theme settings
- Keybindings

## IPC Protocol

Shell communicates with the compositor (xarph-wm) via Unix socket:

- `Request::Workspaces` — Get workspace list
- `Request::Action(FocusWorkspace)` — Focus workspace
- `Request::Action(MoveWindowToWorkspace)` — Move window
- `Event::WorkspaceActivated` — Workspace changed
- `Event::WorkspaceActiveWindowChanged` — Active window changed

## Status

### Completed

- [x] xarph-sdk: 15 new modules (position, desktop_object, desktop_registry, etc.)
- [x] xarph-shell: Qt6/QML structure with 7 cxx-qt bridges
- [x] xarph-settings: Qt6/QML settings app
- [x] xarph-files: Qt6/QML file manager
- [x] xarph-process-admin: Qt6/QML process manager
- [x] xarph-services: Qt6/QML service manager
- [x] xarph-network: Qt6/QML network monitor
- [x] Full workspace compiles (no GTK4 in UI crates)

### In Progress

- [ ] Wire QML files to Qt event loop
- [ ] Implement Wayland layer shell integration
- [ ] Desktop object drag-and-drop
- [ ] Context menu actions
- [ ] Start menu app launching
- [ ] System tray D-Bus integration

### Not Started

- [ ] Video wallpaper support (Qt Multimedia)
- [ ] Desktop widget system
- [ ] Notification system
- [ ] Settings persistence
- [ ] Theme engine
