#!/usr/bin/env bash
# Repositório APT do Doors OS (reprepro). Roda DENTRO do container.
#   apt-repo.sh publish   — inclui /doors/apt/incoming/*.deb e assina
#   apt-repo.sh list      — lista o conteúdo
# Estado do reprepro: /doors/apt/reprepro  ·  árvore pública: /doors/apt/public
# Chave secreta: /doors/keys/gnupg (fora do git)
set -euo pipefail
export GNUPGHOME=/doors/keys/gnupg
B=/doors/apt/reprepro; PUB=/doors/apt/public; FPR=$(cat /doors/keys/fingerprint)
mkdir -p "$B/conf" "$PUB"
cat > "$B/conf/distributions" <<EOT
Origin: Doors OS
Label: Doors OS
Suite: trixie
Codename: trixie
Architectures: amd64
Components: main
Description: Pacotes próprios do Doors OS
SignWith: $FPR
EOT
echo "outdir $PUB" > "$B/conf/options"
R() { reprepro --basedir "$B" --outdir "$PUB" "$@"; }
case "${1:-}" in
  publish)
    for d in /doors/apt/incoming/*.deb; do
      # versão já publicada não muda (o reprepro recusa bytes diferentes para a mesma versão)
      n=$(dpkg-deb -f "$d" Package); v=$(dpkg-deb -f "$d" Version)
      if R list trixie "$n" | grep -q " $n $v\$"; then echo "já publicado: $n $v"; continue; fi
      R includedeb trixie "$d"
    done
    R export trixie
    cp /doors/keys/doors-archive-keyring.asc "$PUB/doors-archive-keyring.asc" ;;
  list) R list trixie ;;
  *) echo "uso: $0 publish|list" >&2; exit 1 ;;
esac
chown -R "$(stat -c %u:%g /doors/repo)" /doors/apt
gpgconf --kill gpg-agent 2>/dev/null || true
