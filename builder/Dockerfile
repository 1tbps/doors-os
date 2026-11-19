# Doors OS — ambiente de build (Debian 13 "trixie")
# O digest da imagem base é fixado após o primeiro pull (ver docs/decisoes/0002-builder.md)
FROM debian:trixie

ENV DEBIAN_FRONTEND=noninteractive LANG=C.UTF-8
RUN apt-get update && apt-get install -y --no-install-recommends \
      live-build debootstrap debian-archive-keyring \
      squashfs-tools xorriso mtools dosfstools \
      grub-pc-bin grub-efi-amd64-bin grub-efi-amd64-signed shim-signed \
      isolinux syslinux-common \
      ca-certificates git rsync cpio xz-utils zstd \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /work
