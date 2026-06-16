# xarph-sdk

Xarph SDK — shared types, configuration, and IPC helpers for the Xarph desktop environment.

## Modules

- **`config`**: Multi-file TOML configuration system (`shell.conf` + `panels/*.conf`), theme management, widget/panel configuration, hot-reload via filesystem watcher.
- **`socket`**: Blocking IPC helper for communicating with the xarph compositor via `$XARPH_SOCKET`.
- **`state`**: Shared state management types.

## IPC Types

This crate re-exports niri IPC types as its public API (`Request`, `Response`, `Event`, `Action`, etc.). The xarph compositor (`xarph-wm`) speaks this protocol over a Unix socket.

## Usage

```toml
[dependencies]
xarph-sdk = "0.1"
```

## Backwards Compatibility

This crate follows the xarph compositor version. It is **not** API-stable in terms of Rust semver. Expect new struct fields and enum variants in patch version bumps.

Use an exact version requirement to avoid breaking changes:

```toml
[dependencies]
xarph-sdk = "=26.4.0"
```
