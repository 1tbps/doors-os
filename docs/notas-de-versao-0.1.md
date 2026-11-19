# Doors OS 0.1 — Notas de versão

Primeira versão do Doors OS: uma distribuição baseada no **Debian 13 "trixie"** (stable), para desktop, em português do Brasil.

## Edições

| Edição | Ambiente | ISO |
|---|---|---|
| XFCE | XFCE 4.20 | `doors-0.1-xfce-amd64.hybrid.iso` |
| KDE | KDE Plasma 6.3 | `doors-0.1-kde-amd64.hybrid.iso` |
| Cinnamon | Cinnamon 6.4 | `doors-0.1-cinnamon-amd64.hybrid.iso` |
| Base | sem ambiente gráfico (modo texto) | `doors-0.1-base-amd64.hybrid.iso` |

Os tamanhos e SHA-256 estão em `relatorio-validacao-0.1.md` e nos arquivos `.sha256` ao lado de cada ISO.

## Destaques

- Sessão live em pt_BR com teclado ABNT2 e fuso America/Sao_Paulo. Na live, o usuário é `doors`, a tela não bloqueia e há um único ícone "Instalar Doors OS".
- Instalador Calamares com a marca Doors. O padrão é ext4, swap em arquivo e partição EFI de 512 MiB.
- Boot UEFI compatível com **Secure Boot**, usando o shim e o GRUB assinados do Debian.
- Tema visual Doors no menu de boot da live, GRUB, tela de login, papel de parede e tela de bloqueio. A arte é provisória.
- Firmware não livre e microcódigo Intel/AMD incluídos, para Wi-Fi e GPUs comuns.
- Atualizações pelos repositórios oficiais do Debian 13 (main, security e updates).

## Limitações conhecidas

- Testes feitos só em máquina virtual QEMU/KVM com UEFI e Secure Boot. Hardware físico e BIOS legado não foram testados.
- Não testados: dual boot, particionamento manual e criptografia de disco.
- A arte (logo e papéis de parede) é provisória.
- Ainda não há repositório próprio do Doors OS. Os pacotes e atualizações vêm do Debian, e os links de suporte apontam para o Debian.
- O resumo do instalador mostra "Configure timezone para 1". O texto vem do Calamares upstream.
- Durante a instalação aparecem avisos `chcon`. Eles são inofensivos.
- A entrada EFI é gravada como `debian` (`\EFI\debian\shimx64.efi`), porque o shim assinado exige esse caminho para o Secure Boot.

## Como verificar a ISO

```
sha256sum -c doors-0.1-<edição>-amd64.hybrid.iso.sha256
```
