#!/usr/bin/env bash
# Gera os .deb do Doors OS (todos os pacotes em packages/). Roda DENTRO do container.
# Saída: /doors/apt/incoming/
set -euo pipefail
R=/doors/repo; . "$R/config/doors.env"
OUT=/doors/apt/incoming; W=$(mktemp -d)
# todos os pacotes-fonte em packages/* (doors-os, doors-aparencia, ...)
for src in "$R"/packages/*/; do
  [ -f "$src/debian/control" ] || continue
  n=$(basename "$src"); cp -a "$src" "$W/$n"
  (cd "$W/$n" && DOORS_VERSION="$DOORS_VERSION" DOORS_APT_URL="$DOORS_APT_URL" \
     dpkg-buildpackage -b -us -uc -tc -d >"$W/build-$n.log" 2>&1) || { cat "$W/build-$n.log"; exit 1; }
done
mkdir -p "$OUT"; rm -f "$OUT"/*.deb
cp "$W"/*.deb "$OUT"/
chown -R "$(stat -c %u:%g "$R")" /doors/apt
ls -1 "$OUT"
