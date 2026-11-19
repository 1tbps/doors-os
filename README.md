# Doors OS: repositório de build

O Doors OS é uma distribuição baseada no Debian 13 "trixie", com edições XFCE, KDE Plasma, Cinnamon e base.

## Estrutura

- `builder/`: Dockerfile do ambiente de build (Debian 13).
- `scripts/`: automação.
  - `builder.sh`: container de build.
  - `build.sh`: gera uma ISO.
  - `build-packages.sh`: gera os pacotes `.deb` do Doors.
  - `apt-repo.sh`: publica no repositório APT local.
  - `apt-pages.sh`: envia o repositório APT ao GitHub Pages.
  - `vm.sh` e `test-vm-boot.sh`: testes em VM.
- `config/doors.env`: versão da distribuição e URL do repositório de atualizações.
- `config/common/`, `config/desktop-common/` e `config/editions/{xfce,kde,cinnamon}/`: camadas do live-build.
- `packages/doors-os/`: fonte dos pacotes `doors-archive-keyring`, `doors-release` e `doors-artwork`.
- `branding/`: gerador da arte, que por enquanto é provisória.
- `docs/`:
  - decisões arquiteturais (`decisoes/`);
  - notas de versão;
  - relatórios de validação;
  - guia do repositório APT (`repositorio-apt.md`).

Fora do repositório, em `/mnt/doors`, ficam:
- `build/`, `cache/`, `isos/`, `logs/` e `vms/`;
- `apt/`: estado do reprepro e árvore pública do repositório;
- `keys/`: **chave secreta de assinatura, que nunca vai para o git**.

## Fluxo de build

```
scripts/builder.sh run bash /doors/repo/scripts/build-packages.sh
scripts/builder.sh run bash /doors/repo/scripts/apt-repo.sh publish
scripts/apt-pages.sh            # publica em https://1tbps.github.io/doors-os-apt/
scripts/builder.sh run bash /doors/repo/scripts/build.sh <base|xfce|kde|cinnamon>
```
