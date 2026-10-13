#!/usr/bin/env bash
set -e

echo "=== Setting Up Input Device Permissions ==="

# 1. Grant immediate access via ACL for the current session (no reboot needed!)
echo "Applying immediate permissions to /dev/input/event*..."
sudo setfacl -m u:"$USER":rw /dev/input/event* 2>/dev/null || true

# 2. Create permanent udev rule with TAG+="uaccess" (systemd modern desktop standard)
RULE_FILE="/etc/udev/rules.d/99-wasd-anywhere.rules"
echo "Creating permanent udev rule at $RULE_FILE..."
cat << 'EOF' | sudo tee "$RULE_FILE" > /dev/null
# WASD Anywhere - Permissions for virtual gamepad and keyboard input
KERNEL=="uinput", MODE="0660", GROUP="input", TAG+="uaccess"
SUBSYSTEM=="input", KERNEL=="event*", TAG+="uaccess"
EOF

# 3. Reload and trigger udev rules
sudo udevadm control --reload-rules
sudo udevadm trigger

# 4. Ensure user is in input group for future sessions
if ! id -nG "$USER" | grep -qw "input"; then
    sudo usermod -aG input "$USER"
fi

echo "=========================================================="
echo "✓ Permissions configured successfully and applied NOW!"
echo "You can now run: ./run.sh"
echo "=========================================================="
