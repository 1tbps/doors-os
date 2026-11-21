# Doors OS — ambiente de build (Debian 13 "trixie")
# Imagem base fixada por digest (debian 13.7, pull de 2026-11-18)
FROM debian:trixie@sha256:9c6e381421d0f48d1b8133e573f33c185ee16cbf505eda9c4a8a4791b25ea8a6

ENV DEBIAN_FRONTEND=noninteractive LANG=C.UTF-8
RUN apt-get update && apt-get install -y --no-install-recommends \
      live-build debootstrap debian-archive-keyring \
      squashfs-tools xorriso mtools dosfstools \
      grub-pc-bin grub-efi-amd64-bin grub-efi-amd64-signed shim-signed \
      isolinux syslinux-common \
      ca-certificates git rsync cpio xz-utils zstd \
      dpkg-dev debhelper fakeroot reprepro gnupg \
      librsvg2-bin fonts-noto-core python3 python3-pil \
      cargo rustc librust-gtk4-dev libgtk-4-dev pkg-config \
      cmake g++ extra-cmake-modules gettext kwin-dev qt6-base-dev qt6-base-private-dev qt6-base-dev-tools \
      libkf6kcmutils-dev libkf6configwidgets-dev libkf6i18n-dev libepoxy-dev libxcb1-dev libdrm-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /work
