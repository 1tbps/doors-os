# 0004: Repositório APT próprio do Doors OS

- **Data:** 2026-11-19
- **Status:** aceito e implantado.

## Contexto

Na 0.1, a identidade (os-release), o tema e o logo de login eram aplicados por hooks e arquivos soltos na ISO. Assim, um sistema já instalado não tinha como receber correções ou atualizações do Doors OS.

## Decisão

1. **Pacotes próprios** (pacote-fonte `packages/doors-os`, formato nativo):
   - `doors-archive-keyring`: chave pública e `/etc/apt/sources.list.d/doors.sources`, em formato deb822 com `Signed-By`.
   - `doors-release`: `/usr/lib/os-release` do Doors OS. O arquivo do `base-files` fica desviado (dpkg-divert) para `os-release.debian`.
   - `doors-artwork`: tema registrado no desktop-base (alternativa `desktop-theme`), papel de parede padrão do XFCE (via diversão) e logo de login do SDDM.
2. **Repositório:**
   - Gerado com `reprepro` e assinado com uma chave ed25519 própria, válida até 2029-11-18.
   - Uma única suíte, `trixie`, com o componente `main` e arquitetura `amd64` (os pacotes `all` entram nela).
3. **O repositório traz só pacotes Doors.** Todo o resto continua vindo dos espelhos oficiais do Debian (main, security e updates).
4. **ISOs:** o build copia os `.deb` publicados para `config/packages.chroot`. Assim a ISO leva as mesmas versões que estão no repositório, e o sistema instalado já sai apontando para ele.
5. **Configuração central** em `config/doors.env`: `DOORS_VERSION` e `DOORS_APT_URL`.

## Segurança

- **Chave secreta:** fica em `/mnt/doors/keys/gnupg`, com modo 700, **fora do git** e sem senha, porque a assinatura é automatizada. A proteção é o acesso ao disco.
- **Revogação:** o certificado fica em `/mnt/doors/keys/gnupg/openpgp-revocs.d/`.
- **Backup:** fazer uma cópia offline de `/mnt/doors/keys`. Sem a chave não é possível publicar atualizações que os sistemas instalados aceitem.
- **Testado:** o APT recusa um repositório adulterado.

## Hospedagem

- O repositório é servido pelo **GitHub Pages**, a partir do repositório **público** separado `1tbps/doors-os-apt`, no endereço `https://1tbps.github.io/doors-os-apt/`.
- O código-fonte continua privado em `1tbps/doors-os`. O repositório público contém só a árvore do APT: `dists/`, `pool/` e a chave **pública**.
- O script `scripts/apt-pages.sh` sincroniza `/mnt/doors/apt/public` com esse repositório.
  - Ele recusa a publicação se encontrar material de chave privada.
  - Os commits usam uma identidade neutra: "Doors OS Project".
- **Validado:**
  - um sistema na `0.2~dev1` (URL de teste) recebeu a `0.2~dev3` e passou a apontar para o Pages sozinho;
  - `apt update` e download de pacotes funcionam pelo HTTPS do Pages, com a assinatura verificada.

## Limitações

- Os sistemas 0.1 já instalados não têm o repositório. Para receberem atualizações, é preciso instalar `doors-archive-keyring` manualmente, uma única vez.
- A ISO de teste `doors-0.2-dev-xfce` aponta para a URL de teste antiga. As próximas ISOs já saem com a URL do Pages.
- O GitHub Pages tem limites de uso: cerca de 1 GB por site e 100 GB de banda por mês, como limite flexível. Isso sobra para pacotes de identidade e tema, mas não serve para hospedar ISOs.
