WASD Anywhere — Virtual Gamepad Mapper for Linux
=================================================

HOW TO RUN:
1. Portable Mode:
   ./run.sh
   (or directly ./wasd-gui)

2. Create Desktop Shortcut:
   ./install.sh
   (Adds WASD Anywhere to your KDE/GNOME application menu pointing to this directory)

3. Remove Desktop Shortcut:
   ./uninstall.sh

4. Permissions (First time only):
   If you see a permission error, run:
   ./setup_permissions.sh

REQUIREMENTS:
- Linux kernel with /dev/uinput enabled (standard on modern Linux)
- Qt 6 Widgets and libevdev
