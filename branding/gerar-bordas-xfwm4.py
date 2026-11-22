#!/usr/bin/env python3
"""Gera as peças de borda do xfwm4 do tema Doors: contorno de 2 px em volta da janela
inteira, com os quatro cantos arredondados (raio 6).
Uso: python3 -I branding/gerar-bordas-xfwm4.py packages/doors-os/artwork/themes/Doors/xfwm4

Cores das peças (as mesmas do Greybird): " " transparente, "." contorno, "+" brilho
da barra de título, "@" fundo da barra (cor do tema GTK, "s active_color_2").
Ativa e inativa têm a mesma espessura (senão a janela "pula" ao ganhar o foco).

Os botões (hide/close/menu...) trazem por cima um PNG com degradê branco; a barra de título
e os cantos de cima ganham o mesmo degradê (title-*.png, top-*.png), senão os botões viram
quadrados mais claros. O topo dos botões (contorno + brilho) é ajustado para a espessura T.
"""
import glob
import math
import os
import re
import sys

from PIL import Image

X = sys.argv[1]
R, T = 6, 2  # raio dos cantos e espessura do contorno
CORES = {"active": "#2f78b5", "inactive": "#a8a8a8"}
FUNDO = "c #DEDEDE s active_color_2"
ALTURA_TITULO = 24


def xpm(nome, linhas, borda):
    w, h = len(linhas[0]), len(linhas)
    cab = [f'"{w} {h} 4 1"', '" \tc None"', f'".\tc {borda}"', '"+\tc #FFFFFF"', f'"@\t{FUNDO}"']
    corpo = [f'"{l}"' for l in linhas]
    return f"/* XPM */\nstatic char * {nome.replace('-', '_')}_xpm[] = {{\n" + ",\n".join(cab + corpo) + "};\n"


def linha_titulo(y):
    """caractere de uma linha da barra de título (contorno em cima, brilho, fundo)"""
    return "." if y < T else ("+" if y == T else "@")


def canto(x, y, cx, cy):
    """dentro do arco do canto: None (fora), "." (contorno) ou "" (dentro)"""
    d = math.hypot(x + 0.5 - cx, y + 0.5 - cy)
    if d > R:
        return None
    return "." if d > R - T else ""


def topo_esquerdo():
    linhas = []
    for y in range(ALTURA_TITULO):
        l = ""
        for x in range(8):
            if x < R and y < R:
                c = canto(x, y, R, R)
                l += " " if c is None else (c or linha_titulo(y))
            elif x < T or y < T:
                l += "."
            else:
                l += linha_titulo(y)
        linhas.append(l)
    return linhas


def baixo_esquerdo():
    linhas = []
    for y in range(R):
        l = ""
        for x in range(R):
            c = canto(x, y, R, 0)
            l += " " if c is None else ("." if c == "." or y >= R - T else "@")
        linhas.append(l)
    return linhas


def titulo():
    return [linha_titulo(y) * 2 for y in range(ALTURA_TITULO)]


def degrade():
    """alfa do degradê branco por linha, lido do botão do próprio tema"""
    im = Image.open(os.path.join(X, "close-active.png")).convert("RGBA")
    return [im.getpixel((0, y))[3] for y in range(im.size[1])]


def png(nome, linhas, alfa):
    """sobreposição: degradê só onde a peça é barra (brilho/fundo), nunca no contorno"""
    w, h = len(linhas[0]), len(linhas)
    im = Image.new("RGBA", (w, h), (0, 0, 0, 0))
    for y, l in enumerate(linhas):
        for x, c in enumerate(l):
            if c in "+@" and y < len(alfa):
                im.putpixel((x, y), (255, 255, 255, alfa[y]))
    im.save(os.path.join(X, nome + ".png"))


def topo_dos_botoes(arq, borda):
    """linhas 0..T-1 = contorno, linha T = brilho, como na barra de título"""
    t = open(arq).read()
    # "`" e "|" não aparecem em nenhum botão do Greybird; sobram de uma geração anterior
    t, extras = re.subn(r'"[`|]\tc [^"]*",\n', "", t)
    cab = re.search(r'"(\d+) (\d+) (\d+) 1"', t)
    w, h, n = map(int, cab.groups())
    n -= extras
    t = t.replace(cab.group(0), f'"{w} {h} {n + 2} 1",\n"`\tc {borda}",\n"|\tc #FFFFFF"', 1)
    linhas = re.findall(r'^("[^"]*")', t, re.M)  # cabeçalho, paleta, pixels
    ini = len(linhas) - h
    novas = ["`" * w] * T + ["|" * w] + [l.strip('"') for l in linhas[ini + T + 1:]]
    t = t[:t.index(linhas[ini], t.index(linhas[ini - 1]) + len(linhas[ini - 1]))]
    t += ",\n".join(f'"{l}"' for l in novas) + "};\n"
    open(arq, "w").write(t)


ALFA = degrade()
for estado, borda in CORES.items():
    te = topo_esquerdo()
    be = baixo_esquerdo()
    pecas = {
        f"top-left-{estado}": te,
        f"top-right-{estado}": [l[::-1] for l in te],
        f"bottom-left-{estado}": be,
        f"bottom-right-{estado}": [l[::-1] for l in be],
        f"bottom-{estado}": ["@"] * (R - T) + ["."] * T,
        f"left-{estado}": ["." * T],
        f"right-{estado}": ["." * T],
    }
    for n in range(1, 6):
        pecas[f"title-{n}-{estado}"] = titulo()
    for nome, linhas in pecas.items():
        with open(os.path.join(X, nome + ".xpm"), "w") as f:
            f.write(xpm(nome, linhas, borda))
        if nome.startswith(("top-", "title-")):
            png(nome, linhas, ALFA)
for arq in sorted(glob.glob(os.path.join(X, "*.xpm"))):
    nome = os.path.basename(arq)
    if not nome.startswith(("top-", "bottom", "left-", "right-", "title-")):
        topo_dos_botoes(arq, CORES["inactive" if "inactive" in nome else "active"])
print("bordas do xfwm4 geradas")
