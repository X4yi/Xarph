# Xarph Changes Summary

## Audit Fixes Applied

This document summarizes the fixes applied during the 2026-06-15 technical audit.

### Security Fixes

- **SEC-01/LOCK-01**: xarph-lock — removed hardcoded `"xarph"` password, implemented real PAM authentication via `pam` crate with fallback (deny unlock if PAM unavailable)
- **PORTAL-01**: xarph-portals.conf — changed from `default=xarph` to `default=gtk` (Secret portal requires real GTK backend, not custom)

### Stability Fixes

- **SVC-01**: xarph-shell.service — added `Restart=on-failure`, `RestartSec=2s`, `StartLimitBurst=5`, `StartLimitIntervalSec=30`, `[Install] WantedBy=xarph-wm.service`
- **SVC-02**: xarph-wm.service — added `Restart=on-failure`, `RestartSec=3s`, `StartLimitBurst=3`, `StartLimitIntervalSec=60`, `[Install] WantedBy=graphical-session.target`
- **SVC-03**: xarph-session.target — added `[Install] WantedBy=graphical-session.target`

### Build System Fixes

- **BUILD-01**: xarph-wm added to root workspace; workspace-level config (deps, profiles, lints) consolidated into root Cargo.toml; removed duplicate `[workspace]` from xarph-wm/Cargo.toml; `niri-visual-tests` excluded (gtk4 0.10 vs 0.11 conflict, not a core component)
- **BUILD-03**: Binary renamed from `niri` to `xarph-wm` via `[[bin]]` section; package renamed to `xarph-wm`; lib kept as `name = "niri"` for internal module references; PKGBUILD updated to match

### CSS Fixes

- **CSS-01 lock**: Removed invalid GTK4 CSS properties (`transition`, `box-shadow`, `letter-spacing`) from `xarph-lock/src/main.rs` LOCK_CSS
- **CSS-01 shell**: Removed 19 lines of invalid GTK4 CSS (`transition`, `box-shadow`, `letter-spacing`, `text-transform`, `backdrop-filter`) from `xarph-shell/src/style.css`

### Documentation Fixes

- **DM-01**: `INSTALL_DESKTOP_ENTRIES.md` rewritten — removed Limine section, corrected GDM/LightDM paths to `/usr/share/wayland-sessions/`
- **DM-02**: `desktop-entries/` directory deleted; canonical file is `data/sessions/xarph.desktop`
- **xarph-sdk/README.md**: Rewritten to describe actual SDK contents (1379-line config system)

### Functional Fixes (Hardcoded → Real)

- **NET-01**: `xarph-network/src/main.rs` rewritten — reads real data via `nmcli`, `/sys/class/net/`, `ip -o addr show`; removed unnecessary `zbus`/`tokio` dependencies
- **xarph-services**: `src/main.rs` rewritten — reads real systemd user services via `systemctl --user list-units` and `list-unit-files`
- **xarph-process-admin**: `src/main.rs` rewritten — reads real process data from `/proc/[pid]/stat`, `/proc/[pid]/status`, `/proc/meminfo`, `/etc/passwd`

### Formatting

- **MISC-01**: Root Cargo.toml reformatted cleanly

### Extension Traits

- **MISC-02**: Created `xarph-sdk/src/compat/` module with extension traits for IPC types:
  - `WindowExt` — `display_name()`, `label()`, `is_on_output()`
  - `WorkspaceExt` — `label()`, `is_visible()`
  - `OutputExt` — `current_mode()`, `resolution_string()`, `display_string()`
  - Feature-gated via `compat` feature flag

### Session Lock Protocol

- **SEC-02**: Rewrote xarph-lock to use ext-session-lock-v1 via smithay-client-toolkit:
  - Replaced gtk4-layer-shell with proper session-lock protocol
  - Uses SHM + Cairo for rendering lock screen
  - Handles keyboard input for password entry
  - Authenticates via PAM, then calls unlock on the protocol
  - Removed gtk4, gio, glib, toml, dirs dependencies

## Files Modified

### Critical Path
- `Cargo.toml` — root workspace: added xarph-wm members, workspace deps/profiles/lints
- `xarph-wm/Cargo.toml` — renamed package, added `[[bin]]` and `[lib]` sections, removed `[workspace]`
- `xarph-wm/src/main.rs` — binary entry point
- `xarph-wm/src/lib.rs` — library entry point (`pub mod niri` preserved)
- `xarph-wm/niri-visual-tests/Cargo.toml` — updated to `package = "xarph-wm"`

### Services
- `xarph-wm/resources/xarph-shell.service` — Restart + Install
- `xarph-wm/resources/xarph-wm.service` — Restart + Install
- `xarph-wm/resources/xarph-portals.conf` — default=gtk
- `data/systemd/user/xarph-session.target` — Install section

### Core Components
- `xarph-lock/src/main.rs` — PAM auth, ext-session-lock-v1 via sctk, SHM+Cairo rendering
- `xarph-lock/Cargo.toml` — replaced gtk4 deps with sctk, cairo-rs, wayland-protocols
- `xarph-network/src/main.rs` — real nmcli/sysfs data
- `xarph-network/Cargo.toml` — removed zbus/tokio
- `xarph-services/src/main.rs` — real systemctl data
- `xarph-services/Cargo.toml` — minimal deps
- `xarph-process-admin/src/main.rs` — real /proc data
- `xarph-process-admin/Cargo.toml` — minimal deps
- `xarph-shell/src/style.css` — removed invalid CSS
- `xarph-sdk/src/compat/` — NEW: extension traits for IPC types
- `xarph-sdk/Cargo.toml` — added compat feature

### Build/Package
- `PKGBUILD` — updated binary references to xarph-wm
- `xarph-sdk/README.md` — rewritten
- `data/sessions/xarph.desktop` — canonical session file

### Deleted
- `desktop-entries/` — entire directory (consolidated to data/sessions/)
- `INSTALL_DESKTOP_ENTRIES.md` — rewritten (not deleted)

## Build Status

`cargo check --workspace` passes with zero errors.

---

## v0.2.0 Integration Release

### Bug Fixes
- **Double-drag**: Removed duplicate GestureDrag in widget_object.rs (drag handled by desktop/mod.rs)
- **CPU calculation**: Fixed single-snapshot ratio to delta-based calculation matching system.rs
- **Empty recent label**: Fixed start_menu.rs to append actual label instead of empty FlowBoxChild
- **Wallpaper panic**: Replaced Display::expect with graceful if-let fallback

### Version Bumps
- All crates updated from 0.1.0 to 0.2.0
- Root workspace version: 26.4.0 → 0.2.0
- PKGBUILD version: 26.04 → 0.2.0

### Naming Standardization
- NIRI_CONFIG → XARPH_CONFIG (main.rs, cli.rs)
- NIRI_DISABLE_SYSTEM_MANAGER_NOTIFY → XARPH_DISABLE_SYSTEM_MANAGER_NOTIFY
- niri=debug → xarph_wm=debug (log filter)
- spawn "swaylock" → spawn "xarph-lock" (default keybind)
- Removed NIRI_SOCKET from xarph-session unset-env
- Replaced niri wiki links with generic comments in default-config.kdl

### Component Changes
- **xarph-launcher**: Deleted entirely (redundant with start menu)
- **xarph-shell panel**: Removed network and system widgets from top.conf
- **xarph-shell tray**: Rewrote to render real StatusNotifierItem icons via D-Bus
- **xarph-settings**: Expanded with 4 pages (General, Panel, Theme & Wallpaper, Shortcuts)
- **xarph-services**: Wired Start/Stop/Restart buttons to systemctl
- **xarph-process-admin**: Wired Kill button to libc::kill, Refresh to re-read /proc
- **xarph-network**: Added periodic refresh every 5 seconds via glib::timeout

### SDK Changes
- Added KeybindConfig struct with 18 keybind fields and defaults
- Added keybind_config field to XarphConfig (serde support)

### CSS Cleanup
- Scoped separator selector to .panel separator (was global)
- Added .tray-widget, .tray-btn, .clock-widget, .date-label classes
- Removed dead .launcher-btn CSS

## Known Remaining Items

- Real tray icon pixmap data (only icon names supported now)
- Desktop context menu actions (Properties, Delete, Configure — items exist but are dead)
- Widget object configure/delete from context menu
