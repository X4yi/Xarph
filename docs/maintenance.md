# Maintenance

## Clean uninstall

Use the uninstaller to remove Xarph packages and system files:

```bash
./xarph-uninstall.sh
```

To remove user configuration and local data as well:

```bash
./xarph-uninstall.sh --purge
```

## Clean reinstall

Use the reinstall helper from the repository root to purge the current installation, clean build artifacts, and rebuild/install the package:

```bash
./xarph-reinstall.sh --purge
```

If you want to keep user data, omit `--purge`.
