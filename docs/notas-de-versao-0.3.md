# Doors OS 0.3: notas de versão

A terceira versão do Doors OS continua baseada no **Debian 13 "trixie"** (stable) e em português do Brasil.

## Uma ISO só, três ambientes

Até a 0.2 havia uma ISO por ambiente. Agora há uma única ISO, `doors-0.3-amd64.hybrid.iso`, com **KDE Plasma, Cinnamon e XFCE**. Ela instala sem internet.

- **Menu de boot:** "Doors OS — KDE Plasma" (padrão, tecla `k`), "Cinnamon" (tecla `n`) e "XFCE" (tecla `x`), com 10 segundos para escolher. Dá para experimentar cada ambiente antes de instalar.
- **Instalador:** a tela nova "Ambiente gráfico", logo depois do teclado, mostra uma prévia de cada ambiente. O sistema instalado fica só com o ambiente escolhido e o login certo para ele (KDE usa SDDM; Cinnamon e XFCE usam LightDM).
- **Espaço mínimo:** 25 GB, porque os três ambientes são copiados antes da remoção dos que não foram escolhidos.

## Visual

- **Papel de parede principal:** "Porta Encantada". Aparece na área de trabalho, no bloqueio, no login, no GRUB, no menu de boot e no instalador. As fotos de portas e a ilustração "Porta Azul" continuam na coleção.
- **Mais papéis de parede:** as coleções do KDE e do GNOME vêm nos três ambientes. No KDE também há a "imagem do dia".
- **Ícones:** tema "Doors", com os ícones elementary-xfce do Xubuntu, nos três ambientes.
- **Janelas:**
  - KDE: barra de título e bordas da janela ativa em azul.
  - Cinnamon e XFCE: janelas Greybird, no estilo do Xubuntu e do MX, com contorno azul na janela ativa.
- **Painel:** embaixo, no estilo do Linux Mint, nos três ambientes.

## Doors Aparência (novo)

Aplicativo próprio do Doors OS, escrito em Rust com GTK4. Aparece no menu como "Aparência do Doors". Cada clique aplica na hora:

- tema claro ou escuro;
- 6 cores de destaque: azul Doors, verde, roxo, laranja, vermelho e grafite;
- painel embaixo (estilo Mint) ou em cima;
- papel de parede da coleção Doors (e a imagem do dia no KDE).

## Pacotes

Pacotes novos: `doors-kde-settings`, `doors-cinnamon-settings` e `doors-aparencia`. Os pacotes `doors-release`, `doors-artwork`, `doors-archive-keyring` e `doors-xfce-settings` foram atualizados para 0.3. Todos são publicados no repositório de atualizações do Doors OS.

## Atualizando a partir da 0.2

Quem tem a 0.2 instalada recebe pelo `apt upgrade` a identidade 0.3, a arte nova e o tema "Doors". As configurações visuais (painel, tema, ícones) valem para usuários novos. Usuários existentes podem aplicá-las com o Doors Aparência, depois de instalar o pacote:

```
sudo apt install doors-aparencia
```

O papel de parede de quem já usa o sistema não é trocado automaticamente.

## Limitações conhecidas

- Testado só em máquina virtual (QEMU/KVM, UEFI). Hardware físico, BIOS legado e Secure Boot da ISO 0.3 não foram testados.
- Não testados: particionamento manual, dual boot e criptografia.
- A tela "Ambiente gráfico" não vem com o KDE pré-selecionado (limitação do Calamares 3.3). A escolha é obrigatória.
- O resumo do instalador não mostra o ambiente escolhido.
- Doors Aparência no Cinnamon e no XFCE: a cor de destaque muda só o contorno da janela ativa. No modo escuro, apps GTK4 continuam claros.
