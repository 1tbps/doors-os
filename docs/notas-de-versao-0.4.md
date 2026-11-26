# Doors OS 0.4: notas de versão

A quarta versão do Doors OS continua baseada no **Debian 13 "trixie"** (stable) e em português do Brasil. É a mesma versão que circulou como 0.4~rc3, testada nos quatro ambientes.

## Quatro ambientes numa ISO só

Uma única ISO, `doors-0.4-amd64.hybrid.iso`, com **KDE Plasma, GNOME, Cinnamon e XFCE**. O ambiente é escolhido no menu de boot (teclas `k`, `g`, `n` e `x`), e o instalador instala o ambiente que estiver em uso. A instalação funciona sem internet.

- Instalação em **btrfs**, com o **Timeshift** já configurado para snapshots.
- Opção no instalador para **codecs e driver NVIDIA** (precisa de internet).
- KDE inicia em **X11** por padrão, por causa do acesso remoto (AnyDesk, RustDesk).

## Visual

- Papéis de parede ilustrados de portas, feitos para o Doors.
- Tela de inicialização própria (Plymouth) e logo do Doors no fastfetch.
- **Tema claro de fábrica**, agora com painel e menu claros no KDE e no XFCE. Quem prefere o visual antigo liga "Painel e menu escuros" no Doors Aparência.
- Janelas com cantos arredondados e contorno colorido, **sem sombra** nos 4 ambientes (a sombra deixava rastro com as janelas gelatinosas).
- **Nome do computador, IP e domínio** no canto do papel de parede, como o BGInfo do Windows. Dá para desligar.

## Uso corporativo

- **Entrar no domínio:** Active Directory pelo winbind, login com o usuário da rede e pasta pessoal criada no primeiro login. Testado num AD real.
  - O usuário do domínio pode trocar a própria foto, e ela aparece na tela de login do KDE.
  - No KDE, a tela de login abre com o último usuário preenchido.
- **Pasta do scanner:** a impressora grava as digitalizações direto no PC, por SMB3.
- **SSH ligado por padrão** (pacote `doors-ssh`), para manutenção pela rede. Cada computador gera as próprias chaves no primeiro boot; root não entra por SSH. Na sessão live o SSH não sobe.

## Aplicativos e terminal

- **Doors Aparência** (tema, cores, painel, papel de parede) e **Doors Digitação** (curso de digitação em português, teclado ABNT2 e internacional).
- Chromium no lugar do Firefox, Flameshot na tecla Print Screen, sem LibreOffice na instalação padrão.
- **Oh My Bash** no terminal de todo usuário novo (pacote `doors-oh-my-bash`). Quem já existe pode usar com `cp /etc/skel/.bashrc ~/`.
- **Teclado numérico ligado** na tela de login e na sessão do KDE.
- ç com `'` + `c` no teclado inglês internacional.

## Atualizando a partir da 0.3

Os pacotes Doors chegam pelo `apt upgrade` a partir do repositório de atualizações. Para receber também o SSH e o Oh My Bash:

```
sudo apt install doors-ssh doors-oh-my-bash
```

As configurações de visual valem para usuários novos; os existentes podem aplicá-las pelo Doors Aparência.

## Problemas conhecidos

- O resumo do instalador mostra "Configure timezone para 1" e "modelo teclado foi configurado para 1". É um defeito de tradução do instalador; a instalação fica correta.
- No Cinnamon, as caixas dos assistentes abrem no canto superior esquerdo da tela, e os cantos de baixo das janelas ficam retos.
- O virt-manager não reconhece a ISO sozinho: escolha "Debian 13".
- A ISO final foi testada em máquina virtual (QEMU/KVM, UEFI), nos 4 ambientes.
