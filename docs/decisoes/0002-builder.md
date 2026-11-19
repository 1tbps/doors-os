# 0002 — Build em container Debian 13 (Docker)

- Data: 2026-11-18 — Status: Aprovado
- live-build roda em container `debian:trixie` privilegiado (necessário para montagens do live-build).
- /mnt/doors montado em /doors; builds, caches, ISOs e logs ficam em /mnt/doors.
- Imagem do builder fica no data-root padrão do Docker (/var/lib/docker, disco do host, ~1 GB) — sem alterar config do Docker.
- Serviço Docker iniciado sob demanda (`systemctl start`), não habilitado no boot.
- Base fixada: debian:trixie@sha256:9c6e3814…25ea8a6 (Debian 13.7). Ferramentas: live-build 1:20250505+deb13u1, debootstrap 1.0.141, grub-efi 2.12-9+deb13u2, shim-signed 1.51.
