#!/usr/bin/env bash
# Teste de boot UEFI de uma ISO em VM transitória (QEMU/KVM, qemu:///system), roda no HOST.
# Uso: test-vm-boot.sh <iso> [segundos_espera] [secboot:0|1]
# Gera screenshot em /mnt/doors/logs/ e destrói a VM ao final. Discos só em pool doors.
set -euo pipefail
ISO=$(realpath "$1"); WAIT=${2:-90}; SB=${3:-0}
C=qemu:///system
NAME="doors-test-$(basename "$ISO" .iso)-$(date +%H%M%S)"
TS=$(date +%Y%m%d-%H%M%S)
SHOT=/mnt/doors/logs/boot-$(basename "$ISO" .iso)-sb$SB-$TS.png
DISK=/mnt/doors/vms/$NAME.qcow2
qemu-img create -q -f qcow2 "$DISK" 20G
FW="uefi"; [ "$SB" = 1 ] && FW="uefi,firmware.feature0.name=secure-boot,firmware.feature0.enabled=yes,firmware.feature1.name=enrolled-keys,firmware.feature1.enabled=yes"
virt-install --connect $C --name "$NAME" --transient --noautoconsole \
  --memory 4096 --vcpus 2 --osinfo debian13 \
  --boot "$FW" --cdrom "$ISO" --disk "path=$DISK,bus=virtio" \
  --network network=default --graphics vnc --video virtio >/dev/null
sleep 20
virsh -c $C send-key "$NAME" KEY_ENTER >/dev/null  # seleciona a 1ª entrada do menu
sleep "$WAIT"
virsh -c $C screenshot "$NAME" /tmp/claude-shot.ppm >/dev/null
python3 -I -c 'import sys;from PIL import Image;Image.open(sys.argv[1]).save(sys.argv[2])' /tmp/claude-shot.ppm "$SHOT" 2>/dev/null || cp /tmp/claude-shot.ppm "${SHOT%.png}.ppm"
virsh -c $C dumpxml "$NAME" | grep -E "<loader|<nvram" || true
virsh -c $C destroy "$NAME" >/dev/null
rm -f "$DISK" /tmp/claude-shot.ppm
echo "SCREENSHOT=$(ls ${SHOT%.png}.* )"
