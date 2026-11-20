# Repositório de atualizações: como operar

Todos os comandos rodam no host, a partir de `/mnt/disco/doors/repo`. O container de build precisa estar disponível; se o Docker estiver parado, rode `sudo systemctl start docker`.

## Publicar uma atualização dos pacotes Doors

1. Altere os arquivos em `packages/doors-os/` (tema, os-release etc.).
2. Acrescente uma entrada em `packages/doors-os/debian/changelog` com uma versão **maior** que a anterior, por exemplo `0.2~dev1` → `0.2~dev2` → `0.2` → `0.2.1`.
3. Gere os pacotes e publique:

```
scripts/builder.sh run bash /doors/repo/scripts/build-packages.sh
scripts/builder.sh run bash /doors/repo/scripts/apt-repo.sh publish
scripts/builder.sh run bash /doors/repo/scripts/apt-repo.sh list
```

4. Publique no GitHub Pages (`https://1tbps.github.io/doors-os-apt/`):

```
scripts/apt-pages.sh
```

   O Pages leva cerca de 1 minuto para atualizar. Os sistemas instalados recebem a atualização no próximo `apt update && apt upgrade`, ou pelo atualizador gráfico.

## Gerar ISOs

O `scripts/build.sh` copia para a ISO os `.deb` **publicados**, que ficam em `apt/public/pool`. Por isso, publique antes de gerar as ISOs.

## Onde fica cada coisa

| Caminho | O que é | Vai para o git? |
|---|---|---|
| `packages/doors-os/` | Fonte dos pacotes | sim |
| `config/doors.env` | Versão da distro e URL do repositório | sim |
| `/mnt/disco/doors/apt/public/` | Árvore pública do repositório (`dists/`, `pool/`, chave `.asc`) | não (é publicada) |
| `/mnt/disco/doors/apt/reprepro/` | Banco de dados do reprepro | não |
| `/mnt/disco/doors/apt/pages/` | Clone do repositório público `1tbps/doors-os-apt` (GitHub Pages) | é o próprio repositório público |
| `/mnt/disco/doors/keys/` | **Chave secreta**, certificado de revogação, impressão digital | **nunca** |

## Chave de assinatura

- **Impressão digital:** `7FB5 B13D 47EB 2F03 04DF  BCF3 2520 C04A 24AC 0AE3`, ed25519, expira em 2029-11-18.
- **Renovar antes de expirar:**

```
GNUPGHOME=/doors/keys/gnupg gpg --quick-set-expire <fpr> 3y
```

  Depois exporte a chave pública de novo para `packages/doors-os/keyring/`, gere uma nova versão do `doors-archive-keyring` e publique.
- **Em caso de vazamento:**
  - Publique a revogação.
  - Gere uma chave nova e distribua-a num `doors-archive-keyring` assinado ainda com a chave antiga, se ela continuar confiável. Se não for, a nova chave terá de ser instalada manualmente.

## Testar localmente com VMs (antes de publicar)

```
python3 -m http.server 8080 --bind 192.168.122.1 --directory /mnt/disco/doors/apt/public
```

Na VM, aponte um arquivo `.sources` temporário para `http://192.168.122.1:8080/`.
