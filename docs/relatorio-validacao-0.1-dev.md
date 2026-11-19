# Relatório de validação — Doors OS 0.1-dev (rodada r5)

Data: 2026-11-19 · Ambiente: QEMU/KVM (libvirt 12.0.0), UEFI OVMF com Secure Boot (chaves MS), 4 GiB RAM, disco virtio 25 GiB descartável.
Base: Debian 13.7 "trixie" · kernel 6.12.117+deb13 · live-build 1:20250505+deb13u1 · Calamares 3.3.14 · repo commit e1f3133.

## Artefatos (r5, /mnt/doors/isos)

| Edição | Tamanho (bytes) | SHA-256 |
|---|---|---|
| XFCE 4.20 | 1953349632 | 6f6513f78e1f17314b3a0e0fe08882fb500414c94326843b1acb6f194818f450 |
| KDE Plasma 6.3.6 | 2885681152 | 5b49a3f9076ba2e883ea1dade0eeddb453ce0af28967e418c513f91d8d24acf6 |
| Cinnamon 6.4.10 | 2337341440 | 2e5d454b192bd01696ee188706bba7c494a553e62283149be2f4675f68055c92 |

Manifestos de pacotes: `doors-0.1-dev-<edição>.packages` ao lado de cada ISO.

## Matriz de testes

| Teste | XFCE | KDE | Cinnamon | Rodada |
|---|---|---|---|---|
| Build sem erro + sha256 | ✅ | ✅ | ✅ | r5 |
| Boot UEFI + Secure Boot (menu Doors, timeout 5 s) | ✅ | ✅ | ✅ | r5 |
| Sessão live gráfica, pt_BR, autologin "Doors" | ✅ | ✅ | ✅ | r5 |
| Ícone único "Instalar Doors OS" abre o instalador | ✅ | ✅ | ✅ | r5 |
| Sem bloqueio de tela na live (10+ min ocioso) | ✅ | ✅ | ✅ | r4 (inalterado na r5) |
| Instalador com marca Doors, padrão São Paulo/pt_BR | ✅ | ✅ | ✅ | r2/r4/r5 |
| Instalação em disco (EFI 512M + ext4 + swapfile) | ✅ r2 | ✅ r4 | ✅ r5 | — |
| 1º boot: GRUB "Doors OS", SB ativo, shim | ✅ r2 | ✅ r4 | ✅ r5 | — |
| Login gráfico com senha, tema Doors no greeter | ✅ r1 | ✅ r4 | ✅ r5 | — |
| Rede (NetworkManager) | ✅ r1 | ✅ r1 | ✅ r1 | — |
| sudo + apt update/install | ✅ r1 | ✅ r1 | ✅ r1 | — |

## Não testado / limitações conhecidas
- Hardware físico (somente VM). BIOS legado não testado (só UEFI).
- Instalação em disco com outro sistema (dual boot), particionamento manual, criptografia, btrfs/xfs.
- Atualização de versão do Doors OS (não há repositório próprio ainda).
- Arte (logo, wallpaper, splash) é provisória, gerada por `branding/gerar-arte.py`.
- Calamares: textos "Configure timezone para 1" / "modelo teclado ... para 1" no resumo (upstream).
- Swapfile padrão de 512 MiB (padrão do Calamares).
- Bug do Debian em `add-calamares-desktop-icon` contornado no build (vale reportar upstream).
- ISO `base` (sem desktop) é da r1: não tem o tema nem as URLs atualizadas.
