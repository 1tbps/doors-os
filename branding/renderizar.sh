#!/usr/bin/env bash
# Renderiza os PNGs da arte (GRUB, Calamares) e prévias. Roda DENTRO do container (rsvg-convert + fontes Noto).
# Uso: scripts/builder.sh run bash /doors/repo/branding/renderizar.sh
set -euo pipefail
R=/doors/repo; F=$R/branding/fontes; TH=$R/packages/doors-os/artwork/doors-theme
CAL=$R/config/desktop-common/includes.chroot/etc/calamares/branding/doors
PREV=/doors/logs/arte; mkdir -p "$PREV"
python3 -I "$R/branding/gerar-arte.py" "$R"
rsvg-convert -w 1920 -h 1080 "$F/grub-16x9.svg" -o "$TH/grub/grub-16x9.png"
rsvg-convert -w 1280 -h 1024 "$F/grub-4x3.svg"  -o "$TH/grub/grub-4x3.png"
for s in welcome slide1 slide2 slide3; do rsvg-convert -w 800 -h 400 "$F/$s.svg" -o "$CAL/$s.png"; done
for k in portal aurora geometrico horizonte; do rsvg-convert -w 1280 -h 720 "$R/packages/doors-os/artwork/backgrounds/doors/$k.svg" -o "$PREV/$k.png"; done
mkdir -p "$R/packages/doors-os/artwork/skel"; rsvg-convert -w 256 -h 256 "$F/face.svg" -o "$R/packages/doors-os/artwork/skel/.face"
rsvg-convert -w 800 -h 600 "$R/config/common/bootloaders/splash.svg" -o "$PREV/splash.png"
rsvg-convert -w 1280 -h 720 "$TH/login/background.svg" -o "$PREV/login.png"
rsvg-convert -w 540 -h 1080 "$TH/wallpaper/contents/images/1080x2160.svg" -o "$PREV/vertical.png"
chown -R "$(stat -c %u:%g "$R")" "$R/packages" "$R/branding" "$R/config" "$PREV"
echo renderizado
