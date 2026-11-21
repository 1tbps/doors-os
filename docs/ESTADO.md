# Doors OS — Estado do Projeto

> Versão pública do arquivo de estado do projeto: decisões, versões, validação e lições aprendidas.
> Dados da máquina de desenvolvimento e anotações pessoais ficam fora deste arquivo.

- **Fase atual:** **Doors OS 0.3 lançado** (tag v0.3, 2026-11-21) — release com a ISO em https://github.com/1tbps/doors-os/releases/tag/v0.3
- **Base:** Debian 13 "trixie"; build com live-build + Calamares dentro de um container Debian (`builder/`)

## Decisões

| Data | Decisão | Status |
|---|---|---|
| 2026-11-18 | Arquivo de estado em `/mnt/disco/doors/ESTADO.md` (originalmente `/mnt/doors`) | Aprovado |
| 2026-11-18 | Base: Debian 13 "trixie" stable | Aprovado |
| 2026-11-18 | Arquitetura proposta: live-build + Calamares + GRUB (UEFI) + systemd + APT; repo próprio pequeno para branding/config | Aprovado pelo usuário junto com Debian (detalhes a refinar na Fase 2) |
| 2026-11-18 | Build dentro de container Debian 13 (Docker, privilegiado); imagem no data-root padrão; Docker iniciado sob demanda (ver repo/docs/decisoes/0002) | Aprovado |
| 2026-11-18 | Painel de acompanhamento em `/mnt/disco/doors/dashboard.html`, atualizado a cada checkpoint | Aprovado |
| 2026-11-19 | Congelar como **Doors OS 0.1** com a arte provisória | Aprovado |
| 2026-11-19 | Repositório de atualizações próprio (reprepro, chave ed25519, pacotes doors-*) — ver docs/decisoes/0004 | Aprovado — hospedagem: GitHub Pages (repo público 1tbps/doors-os-apt), opção 1 escolhida pelo usuário |
| 2026-11-19 | Papéis de parede: fotos de portas do Pexels (padrão Lavanda) | Aprovado |
| 2026-11-19 | Logo do Doors OS: porta de madeira azul em arco (branding/fontes/icone-porta-arco.svg, desenho original) — locais de aplicação a definir | Aprovado |
| 2026-11-19 | Fechar a versão 0.2 | Aprovado e concluído (tag v0.2) |
| 2026-11-19 | Repositório remoto privado no GitHub: https://github.com/1tbps/doors-os (só código; ISOs não sobem) | Aprovado — tornado público em 2026-11-21 (ver abaixo) |
| 2026-11-20 | Visual igual nas 3 edições: ícones "Doors" (elementary-xfce), tema "Doors" (Greybird + contorno azul #2f78b5 na janela ativa), coleções de papéis de parede KDE+GNOME, KDE com painel estilo Mint e barra de título azul | Aprovado ("Sim, começar") |
| 2026-11-20 | **Fim das ISOs separadas: uma ISO única** (edição `desktop`) com KDE, Cinnamon e XFCE; ambiente escolhido no menu de boot (live) e no instalador | Aprovado pelo usuário |
| 2026-11-20 | ISO única **offline completa** (os 3 ambientes dentro; o instalador remove os não escolhidos) — preferida à opção online | Escolha do usuário |
| 2026-11-20 | Lavanda deixa de ser o papel de parede principal ("muito feio para estampar a cara do sistema"); fica só na coleção | Pedido do usuário |
| 2026-11-20 | Papel de parede principal: **"Porta Encantada"**, arte do próprio usuário (2560x1440, `branding/fontes/porta-encantada.png`). A "Porta Azul" (ilustração gerada por `branding/gerar-papel-de-parede.py`) fica na coleção | Escolha do usuário |
| 2026-11-21 | **Doors Aparência** (app próprio em Rust + GTK4, pacote `doors-aparencia`, fonte em `repo/packages/doors-aparencia`): tema claro/escuro, 6 cores de destaque, painel embaixo/em cima, papel de parede (+ imagem do dia no KDE), nos 3 ambientes | Pedido do usuário |
| 2026-11-21 | Fechar como **0.3** (não publicar como 0.3-dev); tag v0.3 (commit e64a297) | Escolha do usuário |
| 2026-11-21 | Repositório de código **github.com/1tbps/doors-os tornado PÚBLICO** (antes privado; histórico verificado sem segredos) e release v0.3 com a ISO em 2 partes (limite de 2 GB por arquivo do GitHub) | Escolha do usuário |

## Etapas

| Fase | Status |
|---|---|
| 0 — Preflight | Concluído (2026-11-18) |
| 1 — Arquitetura | Concluído (2026-11-18) |
| 2 — Infraestrutura de build | Concluído (2026-11-18) |
| 3 — Base comum | Concluído (2026-11-18) |
| 4 — Edições (XFCE, KDE, Cinnamon) | Concluído (2026-11-19) |
| 5 — Testes | Concluído (2026-11-19) — ver relatório de validação |
| 6 — Candidatos a release | Concluído (2026-11-19) — 4 ISOs 0.1 validadas; tag v0.1 (8904c87); ver repo/docs/relatorio-validacao-0.1.md |

### Fase 2 — passos

| Passo | Status |
|---|---|
| Estrutura do projeto (repo, build, cache/apt, isos, logs, vms) + git | Concluído (commit 1aedd48) |
| Dockerfile do builder + scripts/builder.sh | Concluído |
| Iniciar Docker + instalar mtools/squashfs-tools no host | Concluído (docker 29.10; mtools 4.0.49, squashfs-tools 4.7.5) |
| Construir imagem doors-builder:trixie e fixar digest | Concluído (base debian:trixie@sha256:9c6e3814…; Debian 13.7; live-build 1:20250505+deb13u1) |
| Pool libvirt em /mnt/disco/doors/vms | Concluído (pool `doors`, ativo, autostart) |
| Cache de pacotes compartilhado entre edições | Concluído (/mnt/disco/doors/cache/lb-trixie) |
| Build mínimo de validação | Concluído — ISO 838 MiB, sha256 OK, El Torito BIOS+UEFI; boot UEFI c/ Secure Boot até login no QEMU/KVM |
| Política de espaço/cache | Concluído (repo/docs/decisoes/0003) |
| Script de teste de boot (scripts/test-vm-boot.sh) | Concluído |

### Fase 3 — passos

| Passo | Status |
|---|---|
| Camada comum (pacotes base, firmware, os-release Doors OS, GRUB timeout 5s, pt_BR/br/São Paulo) | Concluído (commit 160b418) |
| scripts/build.sh <base\|xfce\|kde\|cinnamon> | Concluído |
| Build e teste de boot da base | Concluído — UEFI+SB, autologin `doors`, hostname doors, kernel 6.12.117+deb13, rede NM, pt_BR |

### Fase 4 — passos

| Passo | Status |
|---|---|
| Camada desktop-common (Calamares, pt_BR, pipewire, Firefox ESR, fontes, spice/qemu agent) | Concluído (commit 55e9557) |
| Listas das edições via tasks Debian (task-xfce/kde/cinnamon-desktop) | Concluído |
| Build/teste XFCE, KDE, Cinnamon (live + instalação Calamares + 1º boot) | Concluído |
| Relatório de validação | Concluído — repo/docs/relatorio-validacao-0.1-dev.md (commit 0ef965f) |

### Problemas conhecidos

1. Calamares resumo mostra "Configure timezone para 1" (texto/tradução upstream).
2. Avisos `chcon` durante instalação (inócuos).
3. Bug upstream Debian: add-calamares-desktop-icon (aspas/sintaxe) — contornado no build; vale reportar ao Debian.
4. Cinnamon: greeter pede usuário antes da senha (comportamento normal do slick-greeter).
5. Observação: Cinnamon entrou em modo de recuperação 1× (segfault no llvmpipe da VM sob carga); não reproduziu.
6. Observação: Firefox apareceu aberto 1× na live XFCE; não reproduziu.

## Repositório de atualizações

| Item | Estado |
|---|---|
| Pacotes doors-archive-keyring / doors-release / doors-artwork / doors-xfce-settings / doors-kde-settings / doors-cinnamon-settings | 0.3 publicados (2026-11-21) |
| Pacote doors-aparencia | 0.1.1 publicado (2026-11-21) |
| Chave de assinatura | 7FB5B13D47EB2F0304DFBCF32520C04A24AC0AE3 (ed25519, expira 2029-11-18); só a chave pública é distribuída |
| Repositório | /mnt/disco/doors/apt/public → publicado em https://1tbps.github.io/doors-os-apt/ (scripts/apt-pages.sh) |
| Validações | Atualização via apt upgrade; repositório adulterado/outra chave recusados; HTTPS no Pages |

## Visual 0.2

| Item | Estado |
|---|---|
| XFCE: painel único embaixo + Greybird/elementary-xfce/Noto (pacote doors-xfce-settings) | Concluído |
| Arte: Portal (padrão), Aurora, Geométrico, Horizonte; login, bloqueio, GRUB, boot, instalador, avatar | Concluído — validado em XFCE, KDE e Cinnamon |

## Doors OS 0.2 — lançado em 2026-11-19

| ISO | Tamanho | SHA-256 (início) | Validação |
|---|---|---|---|
| doors-0.2-base | 933232640 | bf3d53d67bac9cad | boot SB, "Doors OS 0.2" |
| doors-0.2-xfce | 1980891136 | 0f47ad57f6df5ac6 | instalação completa + 1º boot |
| doors-0.2-kde | 2899623936 | a7327f6e8bfcf52a | instalação completa + 1º boot |
| doors-0.2-cinnamon | 2349416448 | 26a09dbe0450d273 | instalação completa + 1º boot |

## Doors OS 0.3 — em desenvolvimento (2026-11-20)

- **ISO única:** `isos/doors-0.3-dev-amd64.hybrid.iso` (3422552064 bytes, SHA-256 ec894709ac4ba2aa…, 0.3~dev7), build em ~8 min (`scripts/builder.sh run /doors/repo/scripts/build.sh desktop`).
- **Boot:** "Doors OS — KDE Plasma" (padrão, atalho k), "Cinnamon" (n), "XFCE" (x); espera 10 s; live padrão do live-build em Utilities. A live usa SDDM e troca a sessão por `doors.ambiente=`.
- **Instalador:** tela "Ambiente gráfico" (packagechooser@ambiente) depois do teclado; `contextualprocess@ambiente` roda `/usr/lib/doors/escolher-ambiente <kde|cinnamon|xfce>` no sistema instalado antes do displaymanager (purge dos outros + autoremove, sem download; KDE→SDDM, Cinnamon/XFCE→LightDM). Espaço mínimo 25 GB.
- **Pacotes novos:** doors-kde-settings, doors-cinnamon-settings; doors-artwork ganhou ícones e tema "Doors" e o índice de papéis de parede do KDE para o Cinnamon (gatilho dpkg).
- **Validado:** live KDE (barra de título azul), Cinnamon (contorno azul 2px) e XFCE (contorno azul) em VM UEFI; `escolher-ambiente` testado nos 3 ambientes sobre overlay do chroot (KDE 2330 pacotes, Cinnamon 2469, XFCE 1932; DM e sessão certos).
- Live dos 3 ambientes com a Porta Encantada validada na ISO dev7 (boot em VM).
- **Instalação completa validada (2026-11-20, VM UEFI, disco 40 GB, "Formatar o disco"):**
  - KDE: ~5 min; 1º boot no SDDM; "Doors OS 0.3-dev"; só KDE/SDDM instalados (Cinnamon, XFCE e LightDM removidos); barra de título azul.
  - Cinnamon: 1º boot no LightDM com o papel de parede Doors; só Cinnamon/LightDM; contorno azul na janela ativa.
  - XFCE: 1º boot no LightDM com o papel de parede Doors; só XFCE/LightDM.
- **Não validado ainda:** BIOS legado; Secure Boot da ISO 0.3; particionamento manual/dual boot.
- **Conhecido:** a tela "Ambiente gráfico" não pré-seleciona o KDE (o widget do Calamares 3.3 ignora `packageChoice`/`selected`; só a versão QML aplica); a escolha é obrigatória, então não há risco. O resumo do instalador não mostra o ambiente escolhido.
- **Lições:** no Plasma 6 os grupos `Colors:Header` mandam na cor da barra de título (removidos do "Doors Claro"); o Cinnamon 6 desenha bordas pelo CSS GTK (`.ssd decoration`), não pelo metacity; atalho `c` é reservado no GRUB.
- Edições antigas (`config/editions/{kde,cinnamon,xfce}` e `build/{kde,cinnamon,xfce}`, ~28 GB) removidas em 2026-11-21; ficam só as edições `desktop` (ISO única) e `base`. As ISOs 0.2 continuam em `isos/`.

## Doors Aparência (app em Rust) — 2026-11-21

- **Código:** `repo/packages/doors-aparencia` (Rust 2021, gtk4-rs 0.9 do Debian; build offline com `/usr/share/cargo/registry`). Módulos: `tema.rs` (funções puras, 10 testes de unidade), `estado.rs` (~/.config/doors/aparencia), `ambiente.rs` (KDE: esquema gerado em ~/.local/share/color-schemes + plasma-apply-*, painel via gdbus evaluateScript; Cinnamon: gsettings; XFCE: xfconf-query), `main.rs` (interface GTK4).
- **Tema pessoal:** nomes alternados DoorsPessoalA/B (o KDE e o GTK só recarregam quando o nome muda). Cinnamon/XFCE: ~/.local/share/themes/DoorsPessoalX com GTK3 importando Greybird(-dark) + contorno colorido e xfwm4 recolorido.
- **Build:** `scripts/build-packages.sh` agora gera todos os pacotes em `packages/*`; imagem do builder ganhou cargo/rustc/librust-gtk4-dev. `build.sh` inclui todo o pool (sem -dbgsym).
- **Validado em VM (live dos 3 ambientes):** KDE (escuro, roxo e verde na barra de título, painel em cima, papel de parede), Cinnamon (escuro, contorno roxo, painel em cima, papel de parede), XFCE (escuro, contorno roxo, painel em cima, papel de parede no monitor real).
- **Lições:** Plasma 6.3 recusa `plasma-apply-colorscheme --accent-color "#rrggbb"`; XFCE 4.20 grava o papel de parede em `monitor<SAÍDA>` (ex.: monitorVirtual-1), não em monitor0; nome do .desktop = ID do app (org.doors.Aparencia) para o ícone no Wayland.
- **Limites conhecidos:** no Cinnamon/XFCE a cor muda só o contorno da janela ativa (o Greybird não tem cor de destaque configurável); no modo escuro do Cinnamon/XFCE, apps GTK4 seguem claros (o Greybird-dark não tem versão GTK4); o terminal mostra avisos de CSS do Greybird no GTK4 (inofensivos).

## Doors OS 0.3 — lançado em 2026-11-21

| Item | Valor |
|---|---|
| ISO | `isos/doors-0.3-amd64.hybrid.iso`, 3422552064 bytes, SHA-256 c423e3f45661db62… |
| Release | https://github.com/1tbps/doors-os/releases/tag/v0.3 (parte1 + parte2 + .sha256; arquivos em `isos/release-0.3/`) |
| Repositório de atualizações | publicado em 2026-11-21 04:04Z: doors-* 0.3 e doors-aparencia 0.1.1; assinatura conferida com gpgv |
| Validação | ISO 0.3 final (2026-11-21, VM UEFI, disco 40 GB, "Formatar o disco"): instalação completa + 1º boot + login nos 3 ambientes. KDE: só KDE/SDDM, doors-kde-settings e doors-aparencia; Cinnamon: só Cinnamon/LightDM, doors-cinnamon-settings e doors-aparencia; XFCE: só XFCE/LightDM, doors-xfce-settings e doors-aparencia. Os 3 com "Doors OS 0.3" e a Porta Encantada no login |
