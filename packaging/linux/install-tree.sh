#!/usr/bin/env bash
# Install ArchiForm into a file tree under <root>/usr, the layout shared by
# the .deb package and the AppImage:
#   usr/bin/archiform, usr/bin/ccad
#   usr/share/applications/archiform.desktop
#   usr/share/icons/hicolor/{256x256,512x512}/apps/archiform.png
#   usr/share/mime/packages/archiform.xml
#   usr/share/doc/archiform/ (README, CHANGELOG, license, sample project)
#
# Usage: packaging/linux/install-tree.sh <dir with the release binaries> <root>
# Run from the repository root.
set -euo pipefail

bin_dir=$1
root=$2
here=packaging/linux

install -Dm755 "$bin_dir/archiform" "$root/usr/bin/archiform"
install -Dm755 "$bin_dir/ccad" "$root/usr/bin/ccad"
install -Dm644 "$here/archiform.desktop" "$root/usr/share/applications/archiform.desktop"
install -Dm644 assets/icons/app-icon-256.png "$root/usr/share/icons/hicolor/256x256/apps/archiform.png"
install -Dm644 assets/icons/app-icon.png "$root/usr/share/icons/hicolor/512x512/apps/archiform.png"
install -Dm644 "$here/archiform.xml" "$root/usr/share/mime/packages/archiform.xml"
doc="$root/usr/share/doc/archiform"
install -Dm644 README.md "$doc/README.md"
install -Dm644 README.pt-BR.md "$doc/README.pt-BR.md"
install -Dm644 CHANGELOG.md "$doc/CHANGELOG.md"
install -Dm644 LICENSE "$doc/LICENSE"
install -Dm644 examples/casa_simples.ccad "$doc/examples/casa_simples.ccad"
