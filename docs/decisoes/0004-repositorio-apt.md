# 0004: Repositório APT próprio do Doors OS

- **Data:** 2026-11-19
- **Status:** aceito. A hospedagem pública ainda está pendente (ver "Em aberto").

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

## Em aberto

- **URL pública definitiva** (hospedagem). Enquanto isso, a URL é de teste: `http://192.168.122.1:8080/`, servida pelo host para as VMs. A URL é gravada no sistema instalado, então precisa estar definida antes do próximo release.
- **Os sistemas 0.1 já instalados não têm o repositório.** Para receberem atualizações, é preciso instalar `doors-archive-keyring` manualmente, uma única vez.
