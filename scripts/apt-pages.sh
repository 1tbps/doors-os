#!/usr/bin/env bash
# Publica /mnt/doors/apt/public no GitHub Pages (repositório PÚBLICO 1tbps/doors-os-apt). Roda no HOST.
# Uso: scripts/apt-pages.sh            (requer gh autenticado)
# Só a árvore pública do APT (dists/, pool/, chave pública) vai para lá — nunca /mnt/doors/keys.
set -euo pipefail
PUB=/mnt/doors/apt/public; W=/mnt/doors/apt/pages; REPO=1tbps/doors-os-apt
FPR=$(cat /mnt/doors/keys/fingerprint)
[ -f "$PUB/dists/trixie/InRelease" ] || { echo "repositório não publicado (rode apt-repo.sh publish)" >&2; exit 1; }
if [ ! -d "$W/.git" ]; then git clone -q "https://github.com/$REPO.git" "$W"; fi
cd "$W"; git pull -q --rebase 2>/dev/null || true
rsync -a --delete --exclude .git --exclude .nojekyll --exclude index.html --exclude README.md "$PUB/" "$W/"
# guarda: nada de material secreto
if grep -rIl -E "PRIVATE KEY" --exclude-dir=.git . >/dev/null; then echo "ABORTADO: material secreto detectado" >&2; exit 1; fi
touch .nojekyll
cat > index.html <<HTML
<!doctype html><html lang="pt-BR"><head><meta charset="utf-8"><title>Doors OS — repositório APT</title></head>
<body style="font-family:sans-serif;max-width:720px;margin:40px auto;padding:0 16px">
<h1>Doors OS — repositório APT</h1>
<p>Pacotes próprios do Doors OS (identidade, tema e chave). Os demais pacotes vêm do Debian.</p>
<p>Chave de assinatura: <a href="doors-archive-keyring.asc">doors-archive-keyring.asc</a><br>
Impressão digital: <code>$FPR</code></p>
<p>Sistemas Doors OS já vêm configurados (pacote <code>doors-archive-keyring</code>).</p>
</body></html>
HTML
cat > README.md <<MD
# Doors OS — repositório APT

Árvore pública do repositório de atualizações do Doors OS, servida em https://1tbps.github.io/doors-os-apt/

Gerada automaticamente (reprepro) — não editar à mão. Impressão digital da chave: \`$FPR\`
MD
git add -A
if git diff --cached --quiet; then echo "sem mudanças"; exit 0; fi
git -c user.name="Doors OS Project" -c user.email="noreply@doors-os.invalid" commit -q -m "Publicação $(date -u +%Y-%m-%dT%H:%MZ)"
git push -q origin HEAD:main
echo "publicado: https://1tbps.github.io/doors-os-apt/"
