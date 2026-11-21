#!/usr/bin/env bash
# Build de uma edição Doors OS. Roda DENTRO do container (/doors = /mnt/disco/doors).
# Uso: build.sh <base|xfce|kde|cinnamon>
# Camadas: config/common  +  config/editions/<edição> (se não for "base").
set -euo pipefail
ED=${1:?edição}; . /doors/repo/config/doors.env; VER=$DOORS_VERSION
R=/doors/repo; W=/doors/build/$ED; CACHE=/doors/cache/lb-trixie
OWNER=$(stat -c %u:%g /doors/repo)
[ "$ED" = base ] || [ -d "$R/config/editions/$ED" ] || { echo "edição desconhecida: $ED" >&2; exit 1; }
FREE=$(df -BG --output=avail /doors | tail -1 | tr -dc 0-9); [ "$FREE" -ge 40 ] || { echo "espaço insuficiente: ${FREE}G (<40G)" >&2; exit 1; }
mkdir -p "$CACHE"
if [ -d "$W/chroot" ]; then (cd "$W" && lb clean --purge >/dev/null 2>&1 || true); fi
rm -rf "$W"; mkdir -p "$W"; cd "$W"
ln -s "$CACHE" cache
NAME="doors-$VER-$ED"; [ "$ED" = desktop ] && NAME="doors-$VER"  # ISO única
eval lb config $(grep -v '^#' "$R/config/common/lb-config.args" | tr '\n' ' ') \
  --iso-application "\"Doors OS\"" --iso-volume "DOORS_${ED^^}" --image-name "$NAME"
rsync -a "$R/config/common/" config/ --exclude lb-config.args
if [ "$ED" != base ]; then rsync -a "$R/config/desktop-common/" config/; rsync -a "$R/config/editions/$ED/" config/; fi
# pacotes Doors publicados no repositório (doors-release, doors-archive-keyring; doors-artwork só nas edições desktop)
mkdir -p config/packages.chroot
for d in /doors/apt/public/pool/main/d/doors-os/*.deb; do
  case "$d" in
    *doors-artwork_*) [ "$ED" = base ] && continue;;
    *doors-*-settings_*) [ "$ED" = desktop ] || case "$d" in *doors-"$ED"-settings_*) ;; *) continue;; esac;;
  esac
  cp "$d" config/packages.chroot/
done
ls config/packages.chroot/*.deb >/dev/null || { echo "sem pacotes Doors: rode build-packages.sh e apt-repo.sh publish" >&2; exit 1; }
B=config/includes.chroot/etc/calamares/branding/doors/branding.desc; if [ -f "$B" ]; then sed -i "s/@VERSION@/$VER/g" "$B"; fi
lb build
ISO="$NAME-amd64.hybrid.iso"
mkdir -p /doors/isos
cp "$ISO" /doors/isos/
cp chroot.packages.live "/doors/isos/$NAME.packages" 2>/dev/null || true
cd /doors/isos && sha256sum "$ISO" > "$ISO.sha256"
chown "$OWNER" "/doors/isos/$NAME"*
echo "ISO=/mnt/disco/doors/isos/$ISO"
