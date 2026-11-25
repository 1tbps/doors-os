# ~/.bashrc: lido pelo bash nos terminais (shells interativos que não são de login).
# Doors OS: o básico do .bashrc do Debian + Oh My Bash (/usr/share/oh-my-bash, pacote
# doors-oh-my-bash). O .bashrc original do Debian está em
# /etc/doors-oh-my-bash/bashrc.debian.

# Fora de um terminal interativo, não faz nada
case $- in
  *i*) ;;
    *) return;;
esac

# --- do Debian ---------------------------------------------------------------------
# histórico: sem linhas repetidas nem começadas por espaço; acrescenta em vez de apagar
HISTCONTROL=ignoreboth
shopt -s histappend
HISTSIZE=1000
HISTFILESIZE=2000
# acompanha o tamanho da janela depois de cada comando
shopt -s checkwinsize
# less mais amigável com arquivos que não são texto
[ -x /usr/bin/lesspipe ] && eval "$(SHELL=/bin/sh lesspipe)"
# nome do chroot no prompt (o tema do Oh My Bash também mostra)
if [ -z "${debian_chroot:-}" ] && [ -r /etc/debian_chroot ]; then
  debian_chroot=$(cat /etc/debian_chroot)
fi

# --- Oh My Bash --------------------------------------------------------------------
# Instalado para todo o sistema e atualizado pelo apt (não pelo próprio Oh My Bash).
# Suas personalizações (aliases, plugins, temas): ~/.local/share/oh-my-bash/custom
# Temas: ls /usr/share/oh-my-bash/themes  ·  Plugins: ls /usr/share/oh-my-bash/plugins
export OSH=/usr/share/oh-my-bash

# Tema ("random" sorteia um a cada terminal)
OSH_THEME="font"

# Atualização: vem com o pacote doors-oh-my-bash
DISABLE_AUTO_UPDATE="true"

# Descomente para diferenciar maiúsculas de minúsculas no Tab
# OMB_CASE_SENSITIVE="true"

# Descomente para não mudar o título da janela do terminal
# DISABLE_AUTO_TITLE="true"

# Descomente para deixar o status do git mais rápido em repositórios grandes
# DISABLE_UNTRACKED_FILES_DIRTY="true"

# Data no comando history: 'dd.mm.yyyy', 'yyyy-mm-dd', etc.
# HIST_STAMPS='dd.mm.yyyy'

# O Oh My Bash pode usar o sudo (ex.: atalhos de apt); "false" para nunca usar
OMB_USE_SUDO=true

# Completar com Tab (/usr/share/oh-my-bash/completions)
completions=(
  git
  composer
  ssh
)

# Atalhos (/usr/share/oh-my-bash/aliases)
aliases=(
  general
)

# Plugins (/usr/share/oh-my-bash/plugins)
plugins=(
  git
  bashmarks
)

if [ -f "$OSH/oh-my-bash.sh" ]; then
  source "$OSH/oh-my-bash.sh"
else
  # pacote doors-oh-my-bash removido: prompt colorido comum do Debian
  PS1='${debian_chroot:+($debian_chroot)}\[\033[01;32m\]\u@\h\[\033[00m\]:\[\033[01;34m\]\w\[\033[00m\]\$ '
  alias ls='ls --color=auto' grep='grep --color=auto' ll='ls -alF' la='ls -A' l='ls -CF'
fi

# --- seus ajustes ------------------------------------------------------------------
# Atalhos pessoais podem ficar aqui ou em ~/.bash_aliases
if [ -f ~/.bash_aliases ]; then
  . ~/.bash_aliases
fi

# completar com Tab dos comandos do sistema (bash-completion)
if ! shopt -oq posix; then
  if [ -f /usr/share/bash-completion/bash_completion ]; then
    . /usr/share/bash-completion/bash_completion
  elif [ -f /etc/bash_completion ]; then
    . /etc/bash_completion
  fi
fi
