#!/usr/bin/env bash
# Build mínimo de validação (sem desktop). Roda DENTRO do container (/doors = /mnt/disco/doors).
set -euo pipefail
W=/doors/build/minimal
CACHE=/doors/cache/lb-trixie
mkdir -p "$CACHE"
rm -rf "$W"; mkdir -p "$W"; cd "$W"
ln -s "$CACHE" cache
lb config \
  --distribution trixie \
  --archive-areas "main contrib non-free non-free-firmware" \
  --mirror-bootstrap http://deb.debian.org/debian/ \
  --mirror-binary http://deb.debian.org/debian/ \
  --security true --updates true \
  --binary-images iso-hybrid \
  --debian-installer none \
  --apt-recommends false \
  --iso-application "Doors OS" --iso-volume "DOORS_MINIMAL" \
  --iso-publisher "Doors OS Project" \
  --image-name doors-minimal
lb build
cp -v doors-minimal-amd64.hybrid.iso /doors/isos/
cd /doors/isos && sha256sum doors-minimal-amd64.hybrid.iso > doors-minimal-amd64.hybrid.iso.sha256
