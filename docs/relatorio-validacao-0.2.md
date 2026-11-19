# Relatório de validação: Doors OS 0.2

- **Data:** 2026-11-19
- **Ambiente:** QEMU/KVM (libvirt 12.0.0) com UEFI OVMF e Secure Boot (chaves Microsoft inscritas), 4 GiB de RAM, 4 vCPUs e disco virtio descartável de 25 GiB.
- **Base:** Debian 13 "trixie", kernel 6.12.117-1, Calamares 3.3.14-1, Firefox ESR 153.7.0esr.
- **Pacotes Doors:** `0.2` (doors-release, doors-artwork, doors-archive-keyring, doors-xfce-settings), publicados em https://1tbps.github.io/doors-os-apt/.
- **Tag:** `v0.2`.

## Artefatos (`/mnt/doors/isos`)

| Edição | Ambiente | Tamanho (bytes) | SHA-256 |
|---|---|---|---|
| base | (texto) | 933232640 | bf3d53d67bac9cad75ea24813a4d574ac95e9818a5375c6ad73256d4791c8d6d |
| xfce | XFCE 4.20.2 + Greybird 3.23.3 + elementary-xfce 0.21 | 1980891136 | 0f47ad57f6df5ac6ca71e9d4571b7f10da439bf0d625eb013f668b388d8dc8ca |
| kde | Plasma 6.3.6 | 2899623936 | a7327f6e8bfcf52a8e76b0235c4f51f4a34786ad298a3d5e0e8e44871e34ddb8 |
| cinnamon | Cinnamon 6.4.10 | 2349416448 | 26a09dbe0450d2739f1cc06364f753671172e0e9003843932363a0dbb3ec2e10 |

## Matriz de testes (ISOs finais 0.2)

| Teste | base | XFCE | KDE | Cinnamon |
|---|---|---|---|---|
| Build sem erro e SHA-256 conferido | ✅ | ✅ | ✅ | ✅ |
| Boot UEFI com Secure Boot | ✅ | ✅ | ✅ | ✅ |
| Identificação "Doors OS 0.2" (`/etc/os-release` → `/usr/lib/os-release`) | ✅ | ✅ | ✅ | ✅ |
| Live: foto Lavanda, logo em arco, ícone "Instalar Doors OS" | — | ✅ | ✅ | ✅ |
| Instalador: logo, foto, "Doors OS 0.2" nas boas-vindas e no resumo | — | ✅ | ✅ | ✅ |
| Instalação (EFI 512 MiB + ext4 + swapfile de 512 MiB) | — | ✅ | ✅ | ✅ |
| 1º boot: SecureBoot ativo, fuso America/Sao_Paulo, usuário no grupo sudo | — | ✅ | ✅ | ✅ |
| Tela de login com a foto e o logo | — | ✅ | ✅ | ✅ |
| Repositório Doors configurado; `apt update` via HTTPS (GitHub Pages) | ✅ (sim.) | ✅ | ✅ | ✅ |
| Pacotes Doors 0.2 instalados | ✅ | ✅ (4) | ✅ (3) | ✅ (3) |
| XFCE: painel embaixo, Whisker, Greybird, elementary-xfce, Noto | — | ✅ | — | — |

Validado antes, na série 0.2-dev, e sem mudança desde então:
- **Atualização pelo repositório:** a `0.2~dev1` subiu para a `0.2~dev2`, e a `0.2~dev1` para a `0.2~dev3`, com troca automática da URL para o GitHub Pages.
- **Recusa de repositório inválido:** o APT recusou o repositório adulterado e também o assinado por outra chave.
- **Rede, `sudo` e `apt install`:** testados nas três edições.

## Corrigido durante a 0.2

- **Live se identificava como Debian:** o live-build deixava em `/etc/os-release` uma cópia do arquivo do Debian. Agora o `doors-release` cria o link para `/usr/lib/os-release`.
- **Resumo do instalador mostrava "0.1":** o `shortVersion` estava fixo no branding.
- **Ícone do instalador sumia no KDE:** era uma disputa de tempo (race) com o `user-dirs` no login. Agora o `doors-live-setup` cria as pastas e o ícone antes do login.
- **Rótulos com caixas brancas na área de trabalho do XFCE:** o Greybird pintava o fundo deles no XFCE 4.20. Agora são definidos em branco, sem fundo, em `xfce4-desktop.xml`.

## Não testado / limitações

- Hardware físico e BIOS legado.
- Dual boot, particionamento manual, criptografia, btrfs e xfs.
- Atualização 0.1 → 0.2: a 0.1 não tem o repositório. A recomendação é reinstalar.
- Cinnamon em VM sem 3D: entrou em modo de recuperação uma vez na série 0.2-dev (llvmpipe sob carga). Não se reproduziu.
- O build depende da internet para dois pacotes de firmware (`firmware-b43*`). Houve uma falha passageira de DNS durante a série dev, resolvida ao repetir o build.
