#!/usr/bin/env bash
# Gera os .deb do Doors OS (packages/doors-os). Roda DENTRO do container.
# Saída: /doors/apt/incoming/
set -euo pipefail
R=/doors/repo; . "$R/config/doors.env"
OUT=/doors/apt/incoming; W=$(mktemp -d)
cp -a "$R/packages/doors-os" "$W/"
cd "$W/doors-os"
DOORS_VERSION="$DOORS_VERSION" DOORS_APT_URL="$DOORS_APT_URL" dpkg-buildpackage -b -us -uc -tc -d >"$W/build.log" 2>&1 || { cat "$W/build.log"; exit 1; }
mkdir -p "$OUT"; rm -f "$OUT"/*.deb
cp "$W"/*.deb "$OUT"/
chown -R "$(stat -c %u:%g "$R")" /doors/apt
ls -1 "$OUT"
