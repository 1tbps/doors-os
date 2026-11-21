#!/usr/bin/env bash
# Gera a arte (fotos + logo) e prévias. Roda DENTRO do container (PIL, rsvg-convert, fontes Noto).
# Uso: scripts/builder.sh run bash /doors/repo/branding/renderizar.sh
set -euo pipefail
R=/doors/repo; F=$R/branding/fontes; TH=$R/packages/doors-os/artwork/doors-theme
PREV=/doors/logs/arte; mkdir -p "$PREV"
# ícones do logo (hicolor) e logo do Calamares — antes do gerador (os banners usam doors-logo.png)
IC=$R/packages/doors-os/artwork/icons/hicolor; LOGO=$F/icone-porta-arco.svg
rm -rf "$IC"
for s in 16 22 24 32 48 64 128 256 512; do
  mkdir -p "$IC/${s}x${s}/apps"; rsvg-convert -w $s -h $s "$LOGO" -o "$IC/${s}x${s}/apps/doors-logo.png"
  ln -s doors-logo.png "$IC/${s}x${s}/apps/doors-install.png"
done
mkdir -p "$IC/scalable/apps"; cp "$LOGO" "$IC/scalable/apps/doors-logo.svg"; ln -s doors-logo.svg "$IC/scalable/apps/doors-install.svg"
rsvg-convert -w 256 -h 256 "$LOGO" -o "$R/config/desktop-common/includes.chroot/etc/calamares/branding/doors/doors-logo.png"
# papel de parede principal (ilustração original) -> branding/fontes/porta-azul.png
python3 -I "$R/branding/gerar-papel-de-parede.py" "$F/porta-azul.svg"
rsvg-convert -w 3840 -h 2160 "$F/porta-azul.svg" -o "$F/porta-azul.png"
python3 -I "$R/branding/gerar-arte.py" "$R"
mkdir -p "$R/packages/doors-os/artwork/skel"
rsvg-convert -w 256 -h 256 "$F/face.svg" -o "$R/packages/doors-os/artwork/skel/.face"
rsvg-convert -w 800 -h 600 "$R/config/common/bootloaders/splash.svg" -o "$PREV/splash.png"
rsvg-convert -w 1280 -h 720 "$TH/login/background.svg" -o "$PREV/login.png"
python3 -I - "$R" "$PREV" <<'PY'
import sys, os
from PIL import Image
r, prev = sys.argv[1], sys.argv[2]
d = f"{r}/packages/doors-os/artwork/backgrounds/doors"; fs = sorted(os.listdir(d))
sheet = Image.new("RGB", (1280, 180 * ((len(fs) + 3) // 4)))
for i, f in enumerate(fs):
    im = Image.open(os.path.join(d, f)); im.thumbnail((320, 180)); sheet.paste(im, ((i % 4) * 320, (i // 4) * 180))
sheet.save(f"{prev}/colecao.png")
PY
chown -R "$(stat -c %u:%g "$R")" "$R/packages" "$R/branding" "$R/config" "$PREV"
echo renderizado
