# Xarph Project Summary

## Overview

Xarph is a modular desktop environment for Linux built in Rust, with Wayland-first architecture. The project uses Niri as the compositor base (xarph-wm).

## Core Components

### 1. xarph-wm (Wayland Compositor)
- Fork of Niri compositor
- Located at: `xarph-wm/`
- Package name: `xarph-wm`, library name: `niri` (for internal module references)
- Binary: `xarph-wm`
- Responsibilities: Wayland protocol, window management, workspaces, animations, IPC
- Session file: `data/sessions/xarph.desktop`
- Systemd services: `xarph-wm.service`, `xarph-shell.service`, `xarph-session.target`

### 2. xarph-shell (Shell)
- GTK4-based shell with layer-shell
- Located at: `xarph-shell/`
- Features: Panel with widgets (Start Button, Workspaces, Clock, Tray)
- Start menu with search, pinned apps, recent apps, all apps grid
- Desktop widgets (mini-clock, calendar, system monitor) with drag support
- Hot-reload configuration support

### 3. xarph-settings (Settings)
- GTK4-based settings panel with 4 pages
- Located at: `xarph-settings/`
- Pages: General (theme), Panel (widget visibility), Theme & Wallpaper, Shortcuts (keybind reference)
- Wallpaper gallery with search and favorites

### 4. xarph-sdk (Software Development Kit)
- Located at: `xarph-sdk/`
- Provides configuration management via xarph-sdk/src/config.rs
- Re-exports niri-ipc types
- Extension traits for IPC types (WindowExt, WorkspaceExt, OutputExt)

## Implemented Components

### xarph-lock
- Lock screen with PAM authentication
- Uses ext-session-lock-v1 via smithay-client-toolkit (sctk)
- SHM + Cairo rendering (no GTK4)

### xarph-network
- Real network data via nmcli, sysfs, ip commands
- GTK4-based UI with periodic refresh

### xarph-services
- Real systemd user services via systemctl
- Start/Stop/Restart button support

### xarph-process-admin
- Real process data from /proc
- Kill process support via libc::kill

## Build System

- Root workspace with 10 members (excludes niri-visual-tests)
- Workspace-level deps, profiles, lints in root Cargo.toml
- `cargo check --workspace` passes with zero errors

## Version

- Current version: 0.2.0

## Project Structure

```
Xarph/
├── Cargo.toml              # Root workspace
├── data/
│   ├── sessions/xarph.desktop
│   └── systemd/user/xarph-session.target
├── xarph-wm/               # Wayland compositor (Niri fork)
│   ├── Cargo.toml
│   ├── src/
│   └── resources/          # Systemd services, portals.conf
├── xarph-shell/            # Shell with widgets
├── xarph-launcher/         # Application launcher
├── xarph-settings/         # Settings panel
├── xarph-sdk/              # Configuration SDK
├── xarph-lock/             # Lock screen
├── xarph-network/          # Network monitor
├── xarph-process-admin/    # Process administrator
├── xarph-services/         # Service manager
├── Xarph-Specification/    # Project specifications
├── rules.md                # AI agent development rules
├── PKGBUILD                # Arch Linux packaging
├── PROJECT_SUMMARY.md      # This file
└── CHANGES_SUMMARY.md      # Audit fixes log
```

## Architecture Principles

Based on `rules.md`:
- Human Control First
- Long-Term Maintainability
- No Magic
- Respect Component Boundaries
- Single Responsibility
- Modular Design
- Rust Preferred
- Performance Rules (no polling, events preferred)
- UI Rules (function before appearance)
- Linux Integration (systemd, NetworkManager, XDG)

## Current Limitations

1. Visual identity: UI is functional prototype
2. Widget system: Needs public `ShellWidget` trait
3. Theme system: Needs proper Xarph Theme API
4. MISC-02: xarph-sdk niri-ipc wrapper types (approved, deferred)
5. SEC-02: ext-session-lock-v1 in xarph-wm (requires smithay protocol handler)
