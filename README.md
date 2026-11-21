# Doors OS: repositório de build

O Doors OS é uma distribuição baseada no Debian 13 "trixie". Desde a 0.3 há uma ISO única com KDE Plasma, Cinnamon e XFCE: o ambiente é escolhido no menu de boot e no instalador.

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
- `config/common/`, `config/desktop-common/` e `config/editions/desktop/`: camadas do live-build (a "desktop" é a ISO única).
- `packages/doors-os/`: fonte dos pacotes `doors-archive-keyring`, `doors-release`, `doors-artwork` e `doors-{kde,cinnamon,xfce}-settings`.
- `packages/doors-aparencia/`: Doors Aparência, app de personalização em Rust + GTK4.
- `branding/`: gerador da arte (logo, papéis de parede, login, GRUB, instalador).
- `docs/`:
  - decisões arquiteturais (`decisoes/`);
  - notas de versão;
  - relatórios de validação;
  - guia do repositório APT (`repositorio-apt.md`).

Fora do repositório, em `/mnt/disco/doors`, ficam:
- `build/`, `cache/`, `isos/`, `logs/` e `vms/`;
- `apt/`: estado do reprepro e árvore pública do repositório;
- `keys/`: **chave secreta de assinatura, que nunca vai para o git**.

## Fluxo de build

```
scripts/builder.sh run bash /doors/repo/scripts/build-packages.sh
scripts/builder.sh run bash /doors/repo/scripts/apt-repo.sh publish
scripts/apt-pages.sh            # publica em https://1tbps.github.io/doors-os-apt/
scripts/builder.sh run bash /doors/repo/scripts/build.sh desktop   # ISO única (KDE, Cinnamon e XFCE); "base" = sem ambiente gráfico
```
