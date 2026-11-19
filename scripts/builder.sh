#!/usr/bin/env bash
# Uso: scripts/builder.sh build-image | shell | run <comando...>
# Executa o container de build com /mnt/doors montado em /doors.
set -euo pipefail
ROOT=/mnt/doors
IMG=doors-builder:trixie
case "${1:-}" in
  build-image)
    docker build -t "$IMG" "$ROOT/repo/builder" 2>&1 | tee "$ROOT/logs/builder-image-$(date +%Y%m%d-%H%M%S).log" ;;
  shell)
    exec docker run --rm -it --privileged -v "$ROOT:/doors" -w /doors "$IMG" bash ;;
  run)
    shift; exec docker run --rm --privileged -v "$ROOT:/doors" -w /doors "$IMG" "$@" ;;
  *) echo "uso: $0 build-image | shell | run <cmd>" >&2; exit 1 ;;
esac
