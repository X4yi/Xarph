# Configuration

Xarph now loads shell layout from a config directory instead of a single `xarph.toml` file.

## Locations

- User config: `$XDG_CONFIG_HOME/xarph/conf/`
- System defaults: `/usr/share/xarph/conf/`
- Legacy file: `$XDG_CONFIG_HOME/xarph/xarph.toml` remains supported for older settings

## Layout

- `shell.conf` defines the shell manifest.
- `panels/*.conf` defines one panel per file.
- Each panel file uses a `[panel]` table plus repeated `[[widget]]` tables.

## Example

The shipped defaults live in [`data/conf/`](../data/conf).

`shell.conf` looks like this:

```toml
[shell]
version = 1
metrics_interval_secs = 2
includes = ["panels/*.conf"]
```

Panel fragments may define widgets like `start_button`, `workspaces`, `clock`, `network`, `system`, `tray`, and `config_button`.

## Notes

- Panels are loaded in deterministic sorted order from the include patterns.
- Duplicate panel IDs are rejected.
- If no panel fragments are found, Xarph falls back to a default top panel.