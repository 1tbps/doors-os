#!/usr/bin/env python3
"""Papel de parede principal do Doors OS: ilustração original no estilo do ícone
(porta azul de tábuas em arco de madeira, latão), entreaberta, com luz dourada saindo,
sobre um céu de entardecer azul. Lado esquerdo limpo para os ícones da área de trabalho.

Uso: python3 -I branding/gerar-papel-de-parede.py <saida.svg>
Renderizar: rsvg-convert -w 3840 -h 2160 saida.svg -o saida.png   (feito por renderizar.sh)
Licença: a do projeto (desenho original).
"""
import sys, random

W, H = 1920, 1080
CX, BASE = 1330, 868          # centro e base da porta
FW, FH = 330, 560             # largura e altura do batente (arco incluído)

rnd = random.Random(2026)     # estrelas sempre iguais
estrelas = []
for _ in range(140):
    x, y = rnd.uniform(0, W), rnd.uniform(0, 560) ** 1.0
    if 1100 < x < 1560 and y > 260:   # nada atrás do brilho da porta
        continue
    r = rnd.choice((0.9, 1.1, 1.3, 1.6, 2.2))
    o = rnd.uniform(0.25, 0.85) * (1 - y / 700)
    estrelas.append(f'<circle cx="{x:.0f}" cy="{y:.0f}" r="{r}" fill="#eaf4ff" opacity="{o:.2f}"/>')


def arco(cx, base, w, h):
    """contorno em arco (como o do ícone): laterais retas e topo semicircular"""
    x0, x1 = cx - w / 2, cx + w / 2
    topo = base - h
    ombro = topo + w * 0.62
    return (f"M{x0:.1f} {base} V{ombro:.1f} C{x0:.1f} {topo + w * 0.12:.1f} {cx - w * 0.30:.1f} {topo:.1f} {cx:.1f} {topo:.1f} "
            f"C{cx + w * 0.30:.1f} {topo:.1f} {x1:.1f} {topo + w * 0.12:.1f} {x1:.1f} {ombro:.1f} V{base} Z")


fora = arco(CX, BASE, FW, FH)            # batente de madeira
vao = arco(CX, BASE, FW - 64, FH - 40)   # vão por onde sai a luz
VX0, VX1 = CX - (FW - 64) / 2, CX + (FW - 64) / 2
# folha da porta aberta para dentro, girando nas dobradiças da direita (vista em perspectiva)
fx1, fx0 = VX1 - 2, VX1 - 112
folha = (f"M{fx1:.1f} {BASE - 2} L{fx1:.1f} {BASE - FH + 120:.1f} "
         f"C{fx1 - 30:.1f} {BASE - FH + 70:.1f} {fx0 + 20:.1f} {BASE - FH + 92:.1f} {fx0:.1f} {BASE - FH + 150:.1f} "
         f"L{fx0:.1f} {BASE - 30} Z")
tabuas = "".join(f'<line x1="{x:.1f}" y1="{BASE - FH + 130 + (fx1 - x) * 0.3:.1f}" x2="{x:.1f}" y2="{BASE - 22 + (x - fx0) * 0.18:.1f}"/>'
                 for x in (fx0 + 28, fx0 + 56, fx0 + 84))

svg = f'''<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" viewBox="0 0 {W} {H}">
<!-- Doors OS — papel de parede "Porta Azul" (desenho original, gerado por branding/gerar-papel-de-parede.py) -->
<defs>
  <linearGradient id="ceu" x1="0" y1="0" x2="0" y2="1">
    <stop offset="0" stop-color="#0b1f36"/><stop offset="0.45" stop-color="#16416f"/>
    <stop offset="0.78" stop-color="#2f78b5"/><stop offset="1" stop-color="#5e9fd6"/>
  </linearGradient>
  <radialGradient id="brilho" cx="{CX}" cy="{BASE - FH * 0.45:.0f}" r="520" gradientUnits="userSpaceOnUse">
    <stop offset="0" stop-color="#ffd27a" stop-opacity="0.55"/><stop offset="0.35" stop-color="#ffb84c" stop-opacity="0.18"/>
    <stop offset="1" stop-color="#ffb84c" stop-opacity="0"/>
  </radialGradient>
  <linearGradient id="luz" x1="0" y1="0" x2="0" y2="1">
    <stop offset="0" stop-color="#fff3cf"/><stop offset="0.6" stop-color="#ffd27a"/><stop offset="1" stop-color="#ffb84c"/>
  </linearGradient>
  <linearGradient id="facho" x1="0" y1="0" x2="0" y2="1">
    <stop offset="0" stop-color="#ffd27a" stop-opacity="0.75"/><stop offset="1" stop-color="#ffb84c" stop-opacity="0"/>
  </linearGradient>
  <linearGradient id="madeira" x1="0" y1="0" x2="1" y2="0">
    <stop offset="0" stop-color="#8a5a32"/><stop offset="0.5" stop-color="#b07a48"/><stop offset="1" stop-color="#7d4f2b"/>
  </linearGradient>
  <linearGradient id="azul" x1="0" y1="0" x2="1" y2="0">
    <stop offset="0" stop-color="#245f96"/><stop offset="0.6" stop-color="#3d8fd1"/><stop offset="1" stop-color="#2f86cf"/>
  </linearGradient>
  <linearGradient id="morro1" x1="0" y1="0" x2="0" y2="1">
    <stop offset="0" stop-color="#173a5e"/><stop offset="1" stop-color="#0f2742"/>
  </linearGradient>
  <linearGradient id="morro2" x1="0" y1="0" x2="0" y2="1">
    <stop offset="0" stop-color="#10294a"/><stop offset="1" stop-color="#091a2e"/>
  </linearGradient>
</defs>
<rect width="{W}" height="{H}" fill="url(#ceu)"/>
<g>{"".join(estrelas)}</g>
<!-- morros ao fundo -->
<path d="M0 760 C220 700 420 720 640 748 C860 776 1040 700 1280 690 C1520 680 1720 730 1920 712 V{H} H0 Z" fill="url(#morro1)" opacity="0.9"/>
<rect width="{W}" height="{H}" fill="url(#brilho)"/>
<!-- chão -->
<path d="M0 846 C320 820 620 836 960 850 C1300 864 1600 842 1920 830 V{H} H0 Z" fill="url(#morro2)"/>
<!-- facho de luz no chão -->
<path d="M{VX0 + 8:.0f} {BASE} L{VX1 - 110:.0f} {BASE} L{CX + 120:.0f} {H} L{CX - 360:.0f} {H} Z" fill="url(#facho)" opacity="0.8"/>
<!-- degrau -->
<rect x="{CX - FW / 2 - 26:.0f}" y="{BASE - 4}" width="{FW + 52}" height="26" rx="10" fill="url(#madeira)" stroke="#2a1b10" stroke-width="7"/>
<!-- batente -->
<path d="{fora}" fill="url(#madeira)" stroke="#2a1b10" stroke-width="9" stroke-linejoin="round"/>
<g fill="none" stroke="#5e3a1d" stroke-width="4" stroke-linecap="round" opacity="0.6">
  <path d="M{CX - FW / 2 + 14:.0f} {BASE - 40} V{BASE - 190}"/><path d="M{CX + FW / 2 - 14:.0f} {BASE - 60} V{BASE - 220}"/>
  <path d="M{CX - 40} {BASE - FH + 16} C{CX - 14} {BASE - FH + 11} {CX + 18} {BASE - FH + 11} {CX + 44} {BASE - FH + 16}"/>
</g>
<!-- vão iluminado -->
<path d="{vao}" fill="url(#luz)" stroke="#2a1b10" stroke-width="6" stroke-linejoin="round"/>
<!-- folha azul entreaberta -->
<path d="{folha}" fill="url(#azul)" stroke="#16324d" stroke-width="7" stroke-linejoin="round"/>
<g stroke="#1b5287" stroke-width="5" stroke-linecap="round">{tabuas}</g>
<!-- dobradiças -->
<g stroke="#1a1d22" stroke-width="5" stroke-linejoin="round" fill="#5b6470">
  <path d="M{fx1 - 52:.0f} {BASE - FH + 210} H{fx1 + 8:.0f} v12 H{fx1 - 52:.0f} Z"/>
  <path d="M{fx1 - 46:.0f} {BASE - 120} H{fx1 + 8:.0f} v12 H{fx1 - 46:.0f} Z"/>
</g>
<!-- argola de latão -->
<circle cx="{fx0 + 20:.0f}" cy="{BASE - 250}" r="13" fill="#f6cf6a" stroke="#4a3210" stroke-width="5"/>
</svg>
'''
open(sys.argv[1], "w").write(svg)
