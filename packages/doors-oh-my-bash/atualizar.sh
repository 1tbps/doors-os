#!/usr/bin/env bash
# Troca a versão do Oh My Bash embutida neste pacote (pasta oh-my-bash/) por um commit
# fixo do github.com/ohmybash/oh-my-bash. Não roda no build: o build usa só o que está no git.
# Uso: packages/doors-oh-my-bash/atualizar.sh <commit>
# Depois: confira o diff, troque COMMIT em debian/rules, o commit no debian/copyright e
# a versão no debian/changelog (0+gitAAAAMMDD.<7 letras do commit>).
set -euo pipefail
C=${1:?uso: $0 <commit do oh-my-bash>}
D=$(cd "$(dirname "$0")" && pwd)
T=$(mktemp -d)
trap 'rm -r -- "$T"' EXIT
git clone -q https://github.com/ohmybash/oh-my-bash "$T/src"
C=$(git -C "$T/src" rev-parse --verify "$C^{commit}")
[ -d "$D/oh-my-bash" ] && mv "$D/oh-my-bash" "$T/antigo"
mkdir "$D/oh-my-bash"
# sem .github e img (só do site) e sem o .gitignore deles (aqui ele ignoraria custom/ e cache/)
git -C "$T/src" archive "$C" | tar -x -C "$D/oh-my-bash" --exclude=.github --exclude=img --exclude=.gitignore
echo "oh-my-bash em $C ($(git -C "$T/src" log -1 --format=%cs "$C"))"
