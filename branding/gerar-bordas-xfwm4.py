#!/usr/bin/env python3
"""Gera as peças de borda do xfwm4 do tema Doors: contorno de 2 px em volta da janela
inteira, com os quatro cantos arredondados (raio 6).
Uso: python3 -I branding/gerar-bordas-xfwm4.py packages/doors-os/artwork/themes/Doors/xfwm4

Cores das peças (as mesmas do Greybird): " " transparente, "." contorno, "+" brilho
da barra de título, "@" fundo da barra (cor do tema GTK, "s active_color_2").
Ativa e inativa têm a mesma espessura (senão a janela "pula" ao ganhar o foco).
"""
import math
import os
import sys

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
print("bordas do xfwm4 geradas")
