# 0002 — Build em container Debian 13 (Docker)

- Data: 2026-11-18 — Status: Aprovado
- live-build roda em container `debian:trixie` privilegiado (necessário para montagens do live-build).
- /mnt/doors montado em /doors; builds, caches, ISOs e logs ficam em /mnt/doors.
- Imagem do builder fica no data-root padrão do Docker (/var/lib/docker, disco do host, ~1 GB) — sem alterar config do Docker.
- Serviço Docker iniciado sob demanda (`systemctl start`), não habilitado no boot.
- Pendente: fixar digest de debian:trixie após o primeiro pull.
