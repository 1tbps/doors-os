#!/usr/bin/env python3
"""Gera o logo do fastfetch (texto com blocos coloridos) a partir de doors-logo.png.

Uso: python3 branding/gerar-logo-fastfetch.py
Saída: packages/doors-os/artwork/fastfetch/doors.txt
Cores: $1 azul (fundo), $2 amarelo (porta), $3 branco (batente);
definidas em packages/doors-os/artwork/fastfetch/config.jsonc.
"""
from pathlib import Path
from PIL import Image

RAIZ = Path(__file__).resolve().parent.parent
COLUNAS, LINHAS = 30, 15  # caractere do terminal é ~2x mais alto que largo

im = Image.open(RAIZ / "branding/doors-logo.png").convert("RGBA")
im = im.crop(im.getbbox())
saida = []
for r in range(LINHAS):
    linha, atual = "", None
    for c in range(COLUNAS):
        x = int((c + 0.5) * im.width / COLUNAS)
        y = int((r + 0.5) * im.height / LINHAS)
        R, G, B, A = im.getpixel((x, y))
        if A < 128:
            linha += " "
            continue
        cor = 3 if (R > 200 and B > 200) else 2 if (R > 200 and G > 150) else 1
        if cor != atual:
            linha += f"${cor}"
            atual = cor
        linha += "█"
    saida.append(linha.rstrip())
(RAIZ / "packages/doors-os/artwork/fastfetch/doors.txt").write_text("\n".join(saida) + "\n")
