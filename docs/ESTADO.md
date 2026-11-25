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
| 2026-11-21 | Papéis de parede: saem as fotos do Pexels e a "Porta Azul"; entram 12 ilustrações de portas do autor do projeto (`branding/fontes/papeis/`). Padrão: **"Colina ensolarada"** ("Portal na floresta" é a antiga Porta Encantada) | Pedido do usuário |
| 2026-11-21 | Tela de inicialização (Plymouth) própria, tema `doors`, no lugar da Ceratopsian do Debian 13 (live e instalado) | Pedido do usuário |
| 2026-11-21 | Janelas com cantos arredondados nos 4 cantos e contorno colorido com brilho, como no COSMIC. KDE: efeito KDE-Rounded-Corners 0.11.0 (GPL-3, github.com/matinlotfali/KDE-Rounded-Corners) empacotado como `doors-kwin-cantos`; Cinnamon: CSS; XFCE: peças do xfwm4 | Aprovado pelo usuário (download do código autorizado) |
| 2026-11-21 | Doors Aparência 0.2 no estilo do Zorin Appearance: menu lateral (Tema, Fundo, Layout, Fontes) e cartões com prévia | Pedido do usuário |
| 2026-11-21 | Versão em desenvolvimento: **0.4-dev**. Incluir o **GNOME** como 4º ambiente fica para a próxima compilação | Pedido do usuário |
| 2026-11-21 | Próxima compilação: tirar o **LibreOffice** da instalação principal; trocar o Firefox pelo **Chrome** só se for possível (o Chrome não pode vir embutido na ISO: opções = repositório do Google com instalação online, ou Chromium do Debian) | Pedido do usuário — escolhido: **Chromium** no lugar do Firefox (Chrome continua instalável depois pelo .deb do Google; avaliar atalho "Instalar Google Chrome") |
| 2026-11-21 | Próxima compilação: opção no instalador "Instalar software de terceiros" (codecs + drivers como NVIDIA, de contrib/non-free; detecta a placa; precisa de internet) | Pedido do usuário |
| 2026-11-21 | Próxima compilação: **curso de digitação próprio**, todo em português do Brasil (telas, lições, testes e jogos), com evolução por níveis, pontuação, velocidade (PPM) e precisão, no teclado ABNT2. Inspirado na ideia de typing.com e TypingClub, mas com conteúdo e visual originais. Deve detectar o layout do teclado (ABNT2, inglês internacional com teclas mortas etc.) e permitir trocar; lições adaptadas ao layout (ex.: ç = ABNT2 tecla própria; US-intl = ' + c) | Pedido do usuário |
| 2026-11-21 | Usuário usa acesso remoto (AnyDesk, RustDesk), que não funciona (ou funciona mal) no Wayland: proposta de KDE iniciar em **X11** por padrão (Wayland disponível no menu de sessão) e esconder as sessões Wayland experimentais do Cinnamon e do XFCE | Aguardando decisão do usuário |
| 2026-11-21 | A testar/corrigir no Doors: teclado inglês internacional com teclas mortas deve gerar **ç com ' + c** (no Kubuntu do usuário sai ć e o ç só com Alt+,). Correção prevista: tabela Compose do Doors (dead_acute + c = ç) valendo para apps GTK e Qt nos 3 ambientes. ~/.XCompose com `include "%L"` + `<dead_acute> <c> : "ç"` validado pelo usuário no Kubuntu (Kate) em 2026-11-21. **Implementado** no doors-release 0.4~dev2 (/etc/skel/.XCompose + XCOMPOSEFILE para KDE e sessões X11); testar na próxima ISO, inclusive Chromium no Wayland | Pedido do usuário |

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

## Doors OS 0.4-dev — validação (2026-11-21)

ISO `doors-0.4-dev-amd64.hybrid.iso` em VM UEFI:
- Live KDE, Cinnamon e XFCE: papel de parede padrão "Colina ensolarada"; coleção com as 12 ilustrações; Doors Aparência 0.2 (Tema, Fundo, Layout, Fontes) aplicando cor/escuro nos 3 ambientes.
- Tela de inicialização Doors (Plymouth `doors`) no live (entradas com `quiet splash`) e no sistema instalado; nenhuma imagem do Debian 13.
- Instalação KDE ("Formatar o disco"): GRUB, Plymouth e login com a arte nova; só KDE instalado; `doors-kwin-cantos` ativo (4 cantos arredondados + contorno na cor de destaque).
- Bordas: KDE completo; XFCE com cantos de baixo arredondados (claro e escuro); Cinnamon com contorno arredondado, mas o conteúdo da janela continua com canto reto por dentro (o muffin não recorta a janela).
- Não testado nesta rodada: instalação de Cinnamon/XFCE, BIOS, hardware físico.

### 0.4-dev — segunda rodada (2026-11-21, tarde)

Implementado: X11 padrão no KDE (Wayland do Cinnamon/XFCE escondido); ç de fábrica (doors-release); Chromium no lugar do Firefox; sem LibreOffice; **GNOME** como 4º ambiente (Dash to Panel + ArcMenu, X11, doors-gnome-settings); opção "Codecs e drivers" no instalador (codecs + NVIDIA, contrib/non-free, precisa de internet); **Doors Digitação** (doors-digitacao 0.1.1); Doors Aparência 0.3 com GNOME; logos do sistema (vendor-logos) do Doors.

Validado em VM (UEFI): GNOME live; instalação GNOME com "Codecs e drivers" (codecs instalados, contrib/non-free, só GNOME, X11, Chromium, Digitação, ~/.XCompose); login do GDM com o logo do Doors; Digitação (lição, teclado na tela, jogo). Não testado: driver NVIDIA (VM sem placa), instalação KDE/Cinnamon/XFCE desta ISO, teclado internacional na VM.
Padrões do Dash to Panel via dconf validados na ISO final (0.4~dev5): sem aviso em inglês, sem visão geral ao entrar, sem botão de grade. Commit 808a949.

### 0.4-dev — terceira rodada (2026-11-21, noite): testes do usuário em PC físico

- GNOME refeito **do jeito do GNOME** (barra no topo, visão geral, dock, Adwaita): saem Dash to Panel e ArcMenu; ficam os papéis de parede do Doors e cantos arredondados com borda colorida só na janela ativa (novo pacote `doors-gnome-cantos` = Rounded Window Corners Reborn 12, GPL-3, com alteração do Doors para a cor por foco). Doors Aparência 0.3.2 sem página Layout no GNOME. Validado em VM (Aparência, Calculadora).
- KDE: quase todos os ícones sumiam (menus, botões). Causa: tema "Doors" sem pasta própria → o Qt/Plasma o considerava inválido. Corrigido (pasta `scalable/apps`); validado no menu do KDE live.
- XFCE: o seletor só mostrava 1 papel de parede (xfce-x.svg apontava para a pasta do tema). Agora aponta para /usr/share/backgrounds/doors.
- Chromium: usa a barra de título e a borda do sistema (perfis novos), para ganhar os cantos arredondados e a borda colorida.
- Em PCs já instalados: KDE → Ícones "elementary Xfce dark" ou Breeze; XFCE → pasta /usr/share/backgrounds/doors; Chromium → Aparência → "Usar a barra de título e as bordas do sistema".

### 0.4-dev — quarta rodada (2026-11-21, noite): lista de defeitos do teste em PC (PC físico do usuário)

Corrigido e validado em VM: instalador instala o ambiente do live (tela "Ambiente gráfico" removida; menu de boot "experimentar e instalar"); senhas curtas aceitas (instalado com "123"); KDE instalado só com KDE; GNOME com borda de 3 px; Cinnamon sem brilho/arredondamento extra no contorno (faixas pretas ao abrir); apps GTK 4 com contorno no XFCE/Cinnamon (~/.config/gtk-4.0/gtk.css); ícones próprios (Aparência = paleta, Digitação = teclado); KDE com categorias do menu coloridas; Doors Digitação 0.2.0 (5 linhas fixas, modos linha fixa/rolagem e parar/seguir no erro); Flameshot na Print Screen (GNOME, Cinnamon, XFCE), Spectacle no KDE.
Pendente: XFCE "conteúdo furando a borda" não reproduzido em VM (aguardando print do usuário); menu novo do Cinnamon (precisa baixar o menu do linuxmint/cinnamon — aguardando autorização). Faixas pretas do Cinnamon só confirmáveis em PC físico.
- Cinnamon: menu novo (Cinnamon 6.6.0, github.com/linuxmint/cinnamon, GPL-2+, baixado com autorização do usuário) adaptado ao 6.4 e empacotado como `menu-doors@doors-os` no doors-cinnamon-settings; menu padrão do painel. Validado no live (VM).
- Menu de boot BIOS/legado (e Ventoy em Legacy) com os 4 ambientes, 10 s (antes só "Live system" = KDE). Contorno de 2 px em todos os ambientes (padrão): XFCE com peças do xfwm4 geradas (branding/gerar-bordas-xfwm4.py), GNOME 2 px. Validado em VM (BIOS e XFCE).
- Uso corporativo (próxima ISO): winbind/Kerberos, cifs-utils, pam_mount, smbclient, gvfs-backends, sane-airscan e simple-scan instalados; winbind desligado até o ingresso (hook 0360). Assistente `doors-entrar-dominio` (menu "Entrar no domínio"; também `--estado` e `--sair`): relógio pelo domínio, smb.conf com idmap rid, pasta pessoal /home/DOMINIO/usuario, login sem rede (cache), pastas da rede em ~/Rede no login, grupo do domínio com sudo. LightDM aceita digitar o usuário ("Outro..."). Validado em container contra um Samba AD de teste (ingresso, login, pasta pessoal, ticket Kerberos, montagem da pasta, saída). Ainda não testado numa ISO nem num AD Windows real.
- XFCE: barra de título com o mesmo degradê dos botões (title-*.png, top-*.png) e topo dos botões com contorno de 2 px; antes os botões apareciam como quadrados mais claros. Cinnamon: contorno da janela sem foco cinza sólido de 2 px (antes preto; o muffin desenha pelo gtk.css e só relê o arquivo ao reiniciar). Cantos de baixo no Cinnamon: o muffin não arredonda (testado border-radius no gtk.css e rounded_bottom no metacity em VM) — ficam retos. GNOME e KDE sem defeitos no PC do usuário.
- Bandeja: XFCE com ícones da bandeja em 24 px (como volume e notificações; rede/Bluetooth seguem coloridos do elementary-xfce, que também aparecem nos menus claros do nm-applet); KDE com todos os ícones de status monocromáticos (cópia do Breeze escuro em /usr/share/doors/kde-menu/icons, só no Plasma). ISO 0.4~dev14 (SHA-256 83482d8fdd5d943d…) com corporativo, botões do XFCE, borda cinza do Cinnamon e ícones do KDE; validada em VM (XFCE, KDE, Cinnamon).
- XFCE: painel flutuante padrão (8 px da borda, 99% da largura, cantos de 8 px em #XfcePanelWindow via ~/.config/gtk-3.0/gtk.css, separador vazio no início); posição calculada por /usr/bin/doors-painel-flutuante (autostart --vigiar, reposiciona ao mudar a resolução; margem do xfwm4 para janelas maximizadas); Doors Aparência 0.3.5 usa o script para em cima/embaixo. Timeshift de fábrica. ISO 0.4~dev16 (SHA-256 360c641296b23b02…) validada em VM (1280x800 e 1920x1080).
- Pasta do scanner (impressora "digitalizar para pasta"): assistente `doors-pasta-scanner` (menu "Pasta do scanner"; também `--remover`). Cria /home/scanner (2770, setgid) compartilhada como \\PC\scanner só em SMB3; a impressora entra com o usuário de sistema `scanner` (sem login, sem shell, senha só do Samba). Quem abre os arquivos: usuários/grupos locais ou do AD por ACL (setfacl pelo UID/GID do winbind, com ACL padrão para arquivos novos). Config em /etc/samba/doors-scanner.conf incluído no smb.conf (o assistente do domínio mantém a linha). samba e acl instalados, smbd desligado até o assistente ser usado. Validado em container no domínio de teste: impressora grava como PC\scanner (com o domínio da empresa recusa, pois o AD não conhece a conta — a mensagem final avisa), SMB2 recusado, usuário do AD com ACL abre/renomeia/apaga, outro usuário do AD sem ACL recebe "permissão negada". ISO: 0.4~dev16 com o assistente (SHA-256 de92a7ecb9283bdd…), compilada; não testada em VM.
- GNOME: "Entrar no domínio" e "Pasta do scanner" no começo da pasta Sistema da grade (pastas padrão do GNOME definidas no dconf do doors-gnome-settings); validado em VM (dev18). Instalador com **btrfs padrão** (subvolumes @, @home, @swap; noatime, compress=zstd:1; Calamares mount.conf pelo hook 0200) e Timeshift já no modo btrfs quando a raiz é btrfs (ambiente-no-instalador). ISO comprimida com zstd 15 (compilação ~10 min; ISO 3,75 GB, +7% sobre xz) e com a revisão no nome (doors-0.4-devNN-amd64.hybrid.iso, do changelog do doors-os); mksquashfs com prioridade -10. ISO 0.4~dev19 (SHA-256 ba84263751410677…) validada em VM: instalação KDE em btrfs, boot, fstab/subvolumes/swapfile, snapshot btrfs do Timeshift (0 s).
- Teste na rede real da empresa falhou nos dois assistentes. Pasta do scanner: o useradd "scanner" falhava porque o grupo scanner já existe no Debian (SANE) — conta da impressora agora é "impressora". Entrar no domínio refeito pela receita do usuário: caixas zenity (pkexec com DISPLAY), nome do PC ≤15 com /etc/hosts (127.0.1.1 NOME.dominio NOME), DNS (_ldap/_kerberos), fuso/NTP, placa de rede (interfaces + bind interfaces only), smb.conf ADS com enum users/groups e /home/%U, net ads join (motivo do erro na caixa), winbind+smbd, net ads dns register + dispatcher do NetworkManager, PAM winbind+mkhomedir, testes no fim; com a pasta do scanner já criada, mostra o novo caminho/usuário (PC\impressora) e pede para rodar a pasta de novo para os usuários do domínio. zenity, pkexec e bind9-host na lista corporativa. Validado em contêiner (Samba AD EMPRESA.LOCAL): nome longo recusado, usuário sem permissão com mensagem clara, ingresso, testjoin, wbinfo, getent/id, su - d123456 (/home/d123456), registro no DNS, ordem scanner→domínio, ACL do d123456. ISO 0.4~dev20 (SHA-256 b56a781b503ea98c…) instalada em VM (KDE, btrfs): os dois assistentes abrem pelo menu; pasta do scanner + gravação da impressora simulada OK. Pendente: teste no AD real; janela do pkexec mostra o comando (falta política polkit própria).
- Assistentes com Aceitar/Cancelar: nada muda no PC antes da confirmação (resumo com nome atual/novo, domínio, placa, usuário). Entrar no domínio detecta PC já no domínio (nsswitch winbind ou security=ads) e oferece Remover do domínio (net ads leave com usuário informado ou só local; avisa se não apagou no servidor; /etc/hosts e nome ficam; pasta do scanner continua). Pasta do scanner pergunta tudo, confirma e só então cria; se já existe oferece Cancelar/Alterar/Remover (ACL refeita do zero ao alterar). Testado em contêiner AD (cancelar não muda nada, aceitar, detectar, remover apaga o computador no AD, senha errada avisa) e em VM (botões zenity). Corrigidos antes da ISO: confirmação que exibiria a senha da impressora e texto duplicado na remoção. ISO 0.4~dev21 (SHA-256 e8f64270c7d5425f…), instalada em VM dev21 (KDE, btrfs): confirmação pelo menu, cancelar não cria nada.
- Assistentes testados instalados nos 4 ambientes (dev21): KDE, Cinnamon e XFCE OK (agente de senha próprio, MATE no XFCE). GNOME: caixa do zenity vazia rodando como root (MESA "Failed to attach to x11 shm": o Xorg do GDM roda como usuário) — corrigido com GSK_RENDERER=cairo nos dois assistentes. ISO 0.4~dev22 (SHA-256 96dd6bea81446af0…) instalada em VM dev22 (GNOME, btrfs): Entrar no domínio abre pelo menu com a caixa visível.
- **0.4~rc1** (candidata à 0.4, a pedido do usuário; ISO não testada em VM — só mudou o pedido de senha dos assistentes): DOORS_VERSION=0.4-rc1; política polkit org.doors-os.{entrar-dominio,pasta-scanner} (mensagem própria, allow_gui) e pkexec direto no script; links do instalador para github.com/1tbps/doors-os; exemplo de nome de PC genérico e ESTADO público sem dados da empresa/PC (o histórico do git ainda tem). ISO SHA-256 1313bdcd7641be32…, publicada na Release v0.4-rc1 em 2 partes. Pendente para a 0.4: teste dos assistentes na rede real; APT público (Pages) ainda em 0.3.

- Teste da 0.4~rc1 no AD real: ingresso OK depois de trocar no /etc/hosts 127.0.1.1 por 127.0.0.1 (o nome do PC precisa cair no lo do Samba). doors-entrar-dominio corrigido: apaga a linha 127.0.1.1 e põe "127.0.0.1 NOME.dominio NOME" logo depois do localhost (marcada, para reingresso). Investigar: logoff de usuário do AD congela no KDE (SDDM lista todos os usuários com winbind enum users = yes).
- ISO 0.4~rc2 (SHA-256 d762fb9be8a07c1d…): Entrar no domínio com 127.0.0.1 no /etc/hosts, enum users/groups = no e /etc/sddm.conf.d/doors-dominio.conf (MaximumUid=999, RememberLastUser) — tela do KDE abre no campo de usuário com o último preenchido. Pergunta de grupo de login (require_membership_of) testada e retirada a pedido (grupo não existe; outras unidades devem poder usar). Validado em contêiner AD e em VM KDE (logoff normal pelo menu volta à tela de login em segundos; com loginctl terminate-user a tela fica preta — sddm-helper morto, não é o logoff normal). VMs rc2-kde, rc2-cinnamon, rc2-xfce, rc2-gnome instaladas (teste/0000).
