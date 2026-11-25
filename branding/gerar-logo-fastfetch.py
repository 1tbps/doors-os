#!/usr/bin/env python3
"""Gera o logo do fastfetch a partir do logo oficial (porta de madeira em arco).

Uso: python3 branding/gerar-logo-fastfetch.py [colunas]
Entrada: packages/doors-os/artwork/icons/hicolor/256x256/apps/doors-logo.png
         (renderizado de branding/fontes/icone-porta-arco.svg)
Saída:   packages/doors-os/artwork/fastfetch/doors.txt

Cada caractere é um meio-bloco (▀) com a cor do pixel de cima no texto e a
do pixel de baixo no fundo. As cores são as do desenho (paleta fixa abaixo),
em cor real (24 bits): Konsole, GNOME Terminal, xfce4-terminal e o terminal
do Cinnamon suportam. O arquivo tem códigos ANSI prontos, por isso o
config.jsonc usa "type": "file-raw" com largura e altura fixas: se mudar o
número de colunas, ajuste "width" e "height" lá.
"""
import sys
from pathlib import Path
from PIL import Image

RAIZ = Path(__file__).resolve().parent.parent
COLUNAS = int(sys.argv[1]) if len(sys.argv) > 1 else 32

# Cores do desenho (branding/fontes/icone-porta-arco.svg)
PALETA = [
    (42, 27, 16),     # contorno
    (126, 80, 43),    # batente (sombra)
    (176, 122, 72),   # batente
    (27, 82, 135),    # tábuas (divisões)
    (47, 134, 207),   # porta azul
    (71, 161, 230),   # porta azul (meio)
    (143, 208, 255),  # brilho
    (246, 207, 106),  # latão claro
    (201, 143, 34),   # latão
    (91, 100, 112),   # ferro
    (26, 29, 34),     # preto
]

def mais_proxima(rgb):
    return min(PALETA, key=lambda c: sum((a - b) ** 2 for a, b in zip(c, rgb)))

im = Image.open(RAIZ / "packages/doors-os/artwork/icons/hicolor/256x256/apps/doors-logo.png").convert("RGBA")
im = im.crop(im.getbbox())
altura_px = round(COLUNAS * im.height / im.width / 2) * 2  # par: 2 pixels por linha
im = im.resize((COLUNAS, altura_px), Image.LANCZOS)

def cor(x, y):
    r, g, b, a = im.getpixel((x, y))
    return None if a < 110 else mais_proxima((r, g, b))

linhas = []
for y in range(0, altura_px, 2):
    s = ""
    for x in range(COLUNAS):
        cima, baixo = cor(x, y), cor(x, y + 1)
        if cima is None and baixo is None:
            s += "\x1b[0m "
        elif baixo is None:
            s += f"\x1b[0;38;2;{cima[0]};{cima[1]};{cima[2]}m▀"
        elif cima is None:
            s += f"\x1b[0;38;2;{baixo[0]};{baixo[1]};{baixo[2]}m▄"
        else:
            s += f"\x1b[0;38;2;{cima[0]};{cima[1]};{cima[2]};48;2;{baixo[0]};{baixo[1]};{baixo[2]}m▀"
    linhas.append(s + "\x1b[0m")
(RAIZ / "packages/doors-os/artwork/fastfetch/doors.txt").write_text("\n".join(linhas) + "\n")
print(f"{COLUNAS} colunas x {len(linhas)} linhas")
