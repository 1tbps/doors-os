#!/usr/bin/env bash
# VM de teste interativa (host). Uso:
#  vm.sh create <nome> <iso>   | shot <nome> [arquivo]  | keys <nome> KEY_A KEY_B ... | type <nome> "texto"
#  vm.sh destroy <nome>        (remove VM, NVRAM e disco do pool doors)
set -euo pipefail
C=qemu:///system; cmd=$1; N=$2; shift 2
case $cmd in
 create) ISO=$(realpath "$1"); qemu-img create -q -f qcow2 /mnt/disco/doors/vms/$N.qcow2 25G
   virt-install --connect $C --name "$N" --noautoconsole --memory 4096 --vcpus 4 --osinfo debian13 \
     --boot uefi --cdrom "$ISO" --disk path=/mnt/disco/doors/vms/$N.qcow2,bus=virtio \
     --network network=default --graphics vnc --video virtio >/dev/null; echo created;;
 shot) F=${1:-/mnt/disco/doors/logs/$N-$(date +%H%M%S).png}; virsh -c $C screenshot "$N" /tmp/vm-$N.ppm >/dev/null
   python3 -I -c 'import sys;from PIL import Image;Image.open(sys.argv[1]).save(sys.argv[2])' /tmp/vm-$N.ppm "$F"; rm -f /tmp/vm-$N.ppm; echo "$F";;
 keys) for k in "$@"; do virsh -c $C send-key "$N" ${k//+/ } >/dev/null; sleep 0.3; done;;
 type) s=$1; for ((i=0;i<${#s};i++)); do k=${s:i:1}; case "$k" in " ") w=KEY_SPACE;; [a-z]) w=KEY_${k^^};; [A-Z]) w="KEY_LEFTSHIFT KEY_$k";; [0-9]) w=KEY_$k;; -) w=KEY_MINUS;; .) w=KEY_DOT;; *) continue;; esac; virsh -c $C send-key "$N" $w >/dev/null; sleep 0.12; done;;
 click) # click <nome> X Y  (pixels numa tela 1280x800)
   X=$(( $1 * 32767 / 1280 )); Y=$(( $2 * 32767 / 800 ))
   virsh -c $C qemu-monitor-command "$N" "{\"execute\":\"input-send-event\",\"arguments\":{\"events\":[{\"type\":\"abs\",\"data\":{\"axis\":\"x\",\"value\":$X}},{\"type\":\"abs\",\"data\":{\"axis\":\"y\",\"value\":$Y}}]}}" >/dev/null
   for d in true false; do virsh -c $C qemu-monitor-command "$N" "{\"execute\":\"input-send-event\",\"arguments\":{\"events\":[{\"type\":\"btn\",\"data\":{\"down\":$d,\"button\":\"left\"}}]}}" >/dev/null; sleep 0.1; done;;
 destroy) virsh -c $C destroy "$N" >/dev/null 2>&1 || true; virsh -c $C undefine "$N" --nvram >/dev/null 2>&1 || true; rm -f /mnt/disco/doors/vms/$N.qcow2; echo destroyed;;
esac
