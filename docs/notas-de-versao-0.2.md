# Doors OS 0.2: notas de versão

A segunda versão do Doors OS continua baseada no **Debian 13 "trixie"** (stable) e em português do Brasil.

## Edições

| Edição | Ambiente | ISO |
|---|---|---|
| XFCE | XFCE 4.20, no estilo Mint/Xubuntu | `doors-0.2-xfce-amd64.hybrid.iso` |
| KDE | KDE Plasma 6.3 | `doors-0.2-kde-amd64.hybrid.iso` |
| Cinnamon | Cinnamon 6.4 | `doors-0.2-cinnamon-amd64.hybrid.iso` |
| Base | sem ambiente gráfico | `doors-0.2-base-amd64.hybrid.iso` |

Os tamanhos e SHA-256 estão em `relatorio-validacao-0.2.md` e nos arquivos `.sha256` ao lado de cada ISO.

## Novidades

### Atualizações do Doors OS

- O sistema instalado já vem com o repositório de atualizações do Doors OS, em `https://1tbps.github.io/doors-os-apt/`. O repositório é assinado com a chave `7FB5 B13D 47EB 2F03 04DF BCF3 2520 C04A 24AC 0AE3`.
- A identidade, a arte e as configurações do sistema viraram pacotes: `doors-release`, `doors-artwork`, `doors-archive-keyring` e `doors-xfce-settings`. Correções e melhorias chegam pelo `apt upgrade` ou pelo atualizador gráfico.
- Os demais pacotes continuam vindo dos repositórios oficiais do Debian.

### Visual novo

- **Logo novo:** uma porta de madeira em arco. Ele aparece no menu, no instalador, no avatar padrão, nas telas de login, no menu de boot e em "Sobre o sistema".
- **Papéis de parede:** fotos de portas do Pexels. A padrão é "Lavanda"; as outras são Azulejos, Casarão, Colonial, Laranja, Grade vermelha, Número 45 e Arco. Telas de login e bloqueio, GRUB, menu de boot e instalador usam as mesmas fotos. Os créditos estão em `/usr/share/doc/doors-artwork/CREDITOS.md`.
- **XFCE:**
  - painel único na parte de baixo, como no Linux Mint: menu Whisker, atalhos, lista de janelas, bandeja e relógio;
  - aparência como a do Xubuntu: tema Greybird, ícones elementary-xfce e fontes Noto Sans;
  - tela de login no mesmo estilo.
- **Instalador:** imagens novas e slides durante a instalação.

### Correções

- O resumo do instalador mostrava "Doors OS 0.1" fixo; agora segue a versão real.
- Os nomes dos ícones da área de trabalho do XFCE apareciam dentro de caixas brancas (Greybird + XFCE 4.20). Agora ficam em branco, sem fundo.

## Atualizando a partir da 0.1

A 0.1 não tinha o repositório do Doors OS. Para passar a receber atualizações, instale uma vez o pacote `doors-archive-keyring` publicado no repositório. Para quem tem uma instalação 0.1, o mais simples é reinstalar a 0.2.

## Limitações conhecidas

- Testado só em máquina virtual (QEMU/KVM, UEFI com Secure Boot). Hardware físico e BIOS legado não foram testados.
- Não testados: dual boot, particionamento manual e criptografia.
- O visual do XFCE vale para usuários novos. Configurações já existentes de um usuário não são alteradas.
- O Cinnamon em VM sem aceleração 3D pode cair raramente em "modo de recuperação" (renderizador por software). Isso não se reproduziu em hardware.
- O resumo do instalador mostra "Configure timezone para 1". O texto vem do Calamares upstream.
- O build depende da internet para dois pacotes de firmware (`firmware-b43*`), que baixam arquivos durante a instalação.

## Como verificar a ISO

```
sha256sum -c doors-0.2-<edição>-amd64.hybrid.iso.sha256
```
