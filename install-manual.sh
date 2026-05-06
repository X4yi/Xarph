#!/bin/bash
# Manual installation script for X4Shell
# Run with: sudo bash install-manual.sh

set -e

X4SHELL_DIR="/home/x4yi/Documentos/X4Shell"
SESSION_SCRIPT="/usr/local/bin/x4-shell-session"
DESKTOP_FILE="/usr/share/wayland-sessions/x4-shell.desktop"

echo "Installing X4Shell manually..."
echo ""

# Install session script
echo "[1/3] Installing session script..."
cp "$X4SHELL_DIR/config/session/x4-shell-session" "$SESSION_SCRIPT"
chmod 755 "$SESSION_SCRIPT"
echo "  Installed to: $SESSION_SCRIPT"
echo ""

# Process and install .desktop file
echo "[2/3] Installing .desktop file..."
sed "s|@SESSION_SCRIPT_PATH@|$SESSION_SCRIPT|g" "$X4SHELL_DIR/config/session/x4-shell.desktop" > "$DESKTOP_FILE"
echo "  Installed to: $DESKTOP_FILE"
echo ""

# Check SDDM configuration
echo "[3/3] Checking SDDM configuration..."
if command -v sddm >/dev/null 2>&1; then
    echo "  SDDM detected"
    
    # Check if SDDM is configured for Wayland
    if [ -f /etc/sddm.conf ]; then
        if grep -q "DisplayServer=wayland" /etc/sddm.conf; then
            echo "  SDDM already configured for Wayland"
        else
            echo "  WARNING: SDDM may not be configured for Wayland"
            echo "  Add 'DisplayServer=wayland' to /etc/sddm.conf [General] section"
        fi
    else
        echo "  Consider creating /etc/sddm.conf.d/wayland.conf with:"
        echo "    [General]"
        echo "    DisplayServer=wayland"
    fi
else
    echo "  SDDM not found (using GDM or other greeter?)"
fi

echo ""
echo "Installation complete!"
echo ""
echo "You can now:"
echo "  1. Restart SDDM: sudo systemctl restart sddm"
echo "  2. Or logout and select 'X4 Shell' from your greeter"
