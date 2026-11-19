# Relatório de validação: Doors OS 0.1

- **Data:** 2026-11-19
- **Ambiente:** QEMU/KVM (libvirt 12.0.0) com UEFI OVMF e Secure Boot (chaves Microsoft inscritas), 4 GiB de RAM, 4 vCPUs e disco virtio descartável de 25 GiB.
- **Base:** Debian 13 "trixie", kernel 6.12.117-1, live-build 1:20250505+deb13u1, Calamares 3.3.14-1 e Firefox ESR 153.7.0esr.
- **Commit:** 8904c87 (tag `v0.1`).

## Artefatos (`/mnt/doors/isos`)

| Edição | Ambiente | Tamanho (bytes) | SHA-256 |
|---|---|---|---|
| base | (texto) | 932184064 | ee256255cf1ffe1dac601f93a95f0910704ae5697984f070e831e5b2fc3ee0ce |
| xfce | XFCE 4.20.2 | 1953349632 | 9f6a84cdafa498c2673164f0bd4cd163fe82d08f5c37134bc37bb96b1141ca02 |
| kde | Plasma 6.3.6 | 2885681152 | 60c8c292d71fba7599b849cf9d56fb4ea52540e54db16eb0536f0f8af237d123 |
| cinnamon | Cinnamon 6.4.10 | 2337341440 | e2003fc57429d6159b6861c1647b32564af1e98549d9d4ac2b8ce19d1061358a |

O manifesto de pacotes de cada ISO fica em `doors-0.1-<edição>.packages`.

## Matriz de testes (ISOs finais)

| Teste | base | XFCE | KDE | Cinnamon |
|---|---|---|---|---|
| Build sem erro e SHA-256 conferido | ✅ | ✅ | ✅ | ✅ |
| Boot UEFI com Secure Boot | ✅ | ✅ | ✅ | ✅ |
| `os-release`: "Doors OS 0.1" | ✅ | ✅ | ✅ | ✅ |
| Sessão live em pt_BR, usuário "Doors" | ✅ (texto) | ✅ | ✅ (3 boots) | ✅ (4 boots) |
| Ícone "Instalar Doors OS" presente e abrindo o instalador | — | ✅ | ✅ | ✅ |
| Instalador "Doors OS 0.1" com padrão São Paulo e pt_BR | — | ✅ | ✅ | ✅ |
| Instalação (EFI 512 MiB + ext4 + swapfile de 511 MiB) | — | ✅ | ✅ | ✅ |
| 1º boot: SecureBoot ativo, fuso America/Sao_Paulo | — | ✅ | ✅ | ✅ |
| Login gráfico com tema Doors (KDE: logo Doors no SDDM) | — | ✅ | ✅ | ✅ |
| Rede, `sudo` e `apt-get install hello` ("Olá, mundo!") | — | ✅ | ✅ | ✅ |

A base foi testada só como sessão live em modo texto: ela não traz instalador gráfico.

## Correções feitas durante o congelamento

- **Versão 0.1 em todos os pontos:** `os-release`, menu de boot e textos do instalador. O build agora carimba a versão automaticamente.
- **Logo no login do KDE:** o tema SDDM `debian-breeze` mostrava o logo do Debian. Agora mostra o logo Doors, via `theme.conf.user`.
- **Ícone do instalador sumia no KDE:** era uma disputa de tempo (race) no início da sessão. O `user-dirs.dirs` podia ser regravado com `$HOME/Desktop`, e o ícone ficava em "Área de trabalho", que o Plasma não mostrava. Agora o serviço `doors-live-setup` cria as pastas do usuário no idioma da live e põe o ícone antes do login gráfico.

## Observações dos testes

- **Cinnamon:** em 1 de 5 boots live, com o host rodando três VMs ao mesmo tempo, o Cinnamon entrou em "modo de recuperação". A causa foi um segfault no `llvmpipe`, o renderizador por software do Mesa usado em VM sem aceleração 3D. Não se repetiu em outros dois boots com o host sem carga. Em hardware real o renderizador é outro (driver de GPU).
- **XFCE:** em 1 de 3 boots, o Firefox apareceu aberto ao entrar na sessão live. Nenhum autostart aponta para o navegador e a causa não foi identificada. Provavelmente foi uma entrada de teclado ou mouse residual da automação.

## Não testado / limitações conhecidas

- Hardware físico (só VM) e BIOS legado (só UEFI).
- Dual boot, particionamento manual, criptografia, btrfs e xfs.
- Atualização entre versões do Doors OS: ainda não há repositório próprio.
- A arte é provisória, gerada por `branding/gerar-arte.py`.
- O resumo do Calamares mostra "Configure timezone para 1" e "modelo teclado … para 1". O texto vem do Calamares upstream.
- O bug do Debian em `add-calamares-desktop-icon` está contornado no build. Vale reportar ao Debian.
