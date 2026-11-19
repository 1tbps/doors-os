#!/usr/bin/env python3
"""Gera a arte provisória do Doors OS (SVG/PNG) dentro das camadas de config.
Uso: python3 -I branding/gerar-arte.py /mnt/doors/repo"""
import sys, os
from PIL import Image, ImageDraw, ImageFont
R = sys.argv[1]
TH = f"{R}/config/desktop-common/includes.chroot/usr/share/desktop-base/doors-theme"
SIZES = ["1920x1080","1920x1200","1280x800","1280x1024","2520x1080","1080x2160"]

def logo_svg(x, y, s):
    k = s/256
    return (f'<g transform="translate({x},{y}) scale({k})">'
            '<rect x="8" y="8" width="240" height="240" rx="48" fill="#1e5e8c"/>'
            '<rect x="78" y="40" width="100" height="184" rx="10" fill="#f0f4f8"/>'
            '<polygon points="78,40 150,58 150,238 78,224" fill="#ffb84c"/>'
            '<circle cx="138" cy="134" r="6" fill="#1e5e8c"/></g>')

def wall_svg(w, h, logo=True, text=True):
    m = min(w, h)
    parts = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{w}" height="{h}" viewBox="0 0 {w} {h}">',
      '<defs><linearGradient id="g" x1="0" y1="0" x2="1" y2="1">'
      '<stop offset="0" stop-color="#0d1f2d"/><stop offset="0.6" stop-color="#16324a"/><stop offset="1" stop-color="#1e5e8c"/></linearGradient>'
      '<radialGradient id="l" cx="0.72" cy="0.38" r="0.6"><stop offset="0" stop-color="#ffb84c" stop-opacity="0.18"/><stop offset="1" stop-color="#ffb84c" stop-opacity="0"/></radialGradient></defs>',
      f'<rect width="{w}" height="{h}" fill="url(#g)"/><rect width="{w}" height="{h}" fill="url(#l)"/>',
      # porta grande estilizada ao fundo
      f'<g opacity="0.10" fill="none" stroke="#f0f4f8" stroke-width="{m*0.006:.1f}">'
      f'<rect x="{w*0.62:.0f}" y="{h*0.12:.0f}" width="{m*0.36:.0f}" height="{m*0.66:.0f}" rx="{m*0.02:.0f}"/></g>',
      f'<polygon opacity="0.12" fill="#ffb84c" points="{w*0.62:.0f},{h*0.12:.0f} {w*0.62+m*0.26:.0f},{h*0.12+m*0.07:.0f} {w*0.62+m*0.26:.0f},{h*0.12+m*0.73:.0f} {w*0.62:.0f},{h*0.12+m*0.66:.0f}"/>']
    if logo:
        s = m*0.12; parts.append(logo_svg(w*0.08, h*0.78 - s, s))
        if text:
            parts.append(f'<text x="{w*0.08 + s*1.15:.0f}" y="{h*0.78 - s*0.35:.0f}" font-family="Noto Sans, DejaVu Sans, sans-serif" font-weight="700" font-size="{s*0.5:.0f}" fill="#f0f4f8">Doors OS</text>')
    parts.append('</svg>')
    return "".join(parts)

def put(path, data, mode="w"):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, mode) as f: f.write(data)

for sz in SIZES:
    w, h = map(int, sz.split("x"))
    put(f"{TH}/wallpaper/contents/images/{sz}.svg", wall_svg(w, h))
    put(f"{TH}/lockscreen/contents/images/{sz}.svg", wall_svg(w, h, logo=False))
put(f"{TH}/login/background.svg", wall_svg(1920, 1080))
put(f"{TH}/login/background-nologo.svg", wall_svg(1920, 1080, logo=False))

# GRUB (PNG, sem texto) — sistema instalado
def grub_png(w, h, path):
    im = Image.new("RGB", (w, h)); d = ImageDraw.Draw(im)
    for y in range(h):
        t = y/h; d.line([(0,y),(w,y)], fill=(int(13+17*t), int(31+63*t), int(45+95*t)))
    os.makedirs(os.path.dirname(path), exist_ok=True); im.save(path)
grub_png(1920, 1080, f"{TH}/grub/grub-16x9.png")
grub_png(1280, 1024, f"{TH}/grub/grub-4x3.png")

# Splash do menu de boot da ISO live (live-build substitui @VERSION@)
splash = wall_svg(800, 600, logo=False).replace('</svg>',
  logo_svg(40, 40, 90) +
  '<text x="150" y="98" font-family="DejaVu Sans" font-weight="bold" font-size="40" fill="#f0f4f8">Doors OS</text>'
  '<text x="152" y="128" font-family="DejaVu Sans" font-size="16" fill="#ffb84c">versão @VERSION@ · base Debian trixie</text></svg>')
put(f"{R}/config/common/bootloaders/splash.svg", splash)
print("arte gerada")
