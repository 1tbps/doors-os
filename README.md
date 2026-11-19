# Doors OS — repositório de build

- `builder/` — Dockerfile do ambiente de build (Debian 13)
- `scripts/` — automação (`builder.sh`)
- `config/common/` — camada comum (live-build)
- `config/editions/{xfce,kde,cinnamon}/` — camadas por edição
- `docs/decisoes/` — registros de decisões arquiteturais

Fora do repo, em /mnt/doors: `build/` (trabalho), `cache/` (pacotes), `isos/`, `logs/`, `vms/` (discos de VM).
