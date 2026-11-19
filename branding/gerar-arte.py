#!/usr/bin/env python3
"""Gera a arte do Doors OS a partir das fotos de portas (Pexels) + logo vetorial.
Roda DENTRO do container de build (PIL + fontes Noto): branding/renderizar.sh
Uso: python3 -I branding/gerar-arte.py /caminho/do/repo

Fotos: branding/fotos/originais/ — licença Pexels (uso e modificação livres; ver branding/fotos/CREDITOS.md)
Saídas:
  packages/doors-os/artwork/doors-theme/   tema do desktop-base (papel de parede, bloqueio, login, GRUB)
  packages/doors-os/artwork/backgrounds/   coleção de papéis de parede (todas as fotos)
  packages/doors-os/artwork/wallpapers/    pacotes de papel de parede do KDE (links para backgrounds/)
  packages/doors-os/debian/doors-artwork.links (gerado)
  config/common/bootloaders/splash.svg     menu de boot da ISO live
  config/desktop-common/.../calamares/branding/doors/{welcome,slide1..3}.png
"""
import sys, os, io, json, base64, shutil
from PIL import Image, ImageOps, ImageDraw, ImageFont, ImageEnhance

R = sys.argv[1]
ART = f"{R}/packages/doors-os/artwork"
TH = f"{ART}/doors-theme"
SRC = f"{R}/branding/fotos/originais"
CAL = f"{R}/config/desktop-common/includes.chroot/etc/calamares/branding/doors"
FONT = "Noto Sans, DejaVu Sans, sans-serif"
NOTO = "/usr/share/fonts/truetype/noto"
AMBER, GOLD = "#ffb84c", "#ffd27a"

# chave: (arquivo, nome exibido, fotógrafo, id Pexels, centro do recorte (x, y))
FOTOS = {
    "lavanda":   ("pexels-lich-13403615.jpg", "Lavanda", "lich", 13403615, (0.5, 0.88)),
    "azulejos":  ("pexels-lucaspcarlini-38904535.jpg", "Azulejos", "Lucas Carlini", 38904535, (0.5, 0.5)),
    "casarao":   ("pexels-bruno-vieira-694029667-18556523.jpg", "Casarão", "Bruno Vieira", 18556523, (0.5, 0.5)),
    "colonial":  ("pexels-giovanna-prado-2148616234-30190296.jpg", "Colonial", "Giovanna Prado", 30190296, (0.5, 0.6)),
    "laranja":   ("pexels-jan-van-der-wolf-11680885-40840715.jpg", "Laranja", "Jan van der Wolf", 40840715, (0.5, 0.5)),
    "grade":     ("pexels-furkanakt-31928394.jpg", "Grade vermelha", "Furkan Aktaş", 31928394, (0.5, 0.5)),
    "numero45":  ("pexels-malcolnphoto-28452846.jpg", "Número 45", "malcolnphoto", 28452846, (0.5, 0.55)),
    "arco":      ("pexels-shantumsingh-41109588.jpg", "Arco", "Shantum Singh", 41109588, (0.5, 0.62)),
}
PADRAO = "lavanda"


def put(path, data, mode="w"):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, mode) as f:
        f.write(data)


def foto(key, w, h):
    arq, _, _, _, c = FOTOS[key]
    im = ImageOps.exif_transpose(Image.open(f"{SRC}/{arq}")).convert("RGB")
    return ImageOps.fit(im, (w, h), Image.LANCZOS, centering=c)


def escurecer(im, f):
    return ImageEnhance.Brightness(im).enhance(1 - f)


def jpg(im, path, q=86):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    im.save(path, "JPEG", quality=q, optimize=True, progressive=True)


def data_uri(im, q=82):
    b = io.BytesIO(); im.save(b, "JPEG", quality=q, optimize=True, progressive=True)
    return "data:image/jpeg;base64," + base64.b64encode(b.getvalue()).decode()


# ---------------------------------------------------------------- logo (vetorial)
LOGO_DEFS = ('<linearGradient id="lg_bg" x1="0" y1="0" x2="1" y2="1">'
             '<stop offset="0" stop-color="#2a78ad"/><stop offset="1" stop-color="#16456e"/></linearGradient>'
             '<linearGradient id="lg_door" x1="0" y1="0" x2="1" y2="1">'
             f'<stop offset="0" stop-color="{GOLD}"/><stop offset="1" stop-color="#f59e2a"/></linearGradient>')


def logo_svg(x, y, s, opacity=1.0):
    k = s / 256
    return (f'<g transform="translate({x:.1f},{y:.1f}) scale({k:.4f})" opacity="{opacity}">'
            '<rect x="8" y="8" width="240" height="240" rx="52" fill="url(#lg_bg)"/>'
            '<rect x="78" y="40" width="100" height="184" rx="10" fill="#f0f4f8"/>'
            '<polygon points="78,40 150,58 150,238 78,224" fill="url(#lg_door)"/>'
            '<circle cx="138" cy="134" r="6" fill="#1e5e8c"/></g>')


def wordmark(x, y, s, opacity=1.0):
    return (logo_svg(x, y, s, opacity) +
            f'<text x="{x + s * 1.18:.1f}" y="{y + s * 0.68:.1f}" font-family="{FONT}" font-weight="700" '
            f'font-size="{s * 0.52:.1f}" fill="#f0f4f8" opacity="{opacity}">Doors OS</text>')


def svg_com_foto(im, w, h, extra=""):
    return (f'<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="{w}" height="{h}" '
            f'viewBox="0 0 {w} {h}"><defs>{LOGO_DEFS}</defs>'
            f'<image x="0" y="0" width="{w}" height="{h}" preserveAspectRatio="xMidYMid slice" xlink:href="{data_uri(im)}"/>'
            f'{extra}</svg>')


def bg_xml(paths):
    x = ['<background>', '  <static>', '    <duration>8640000.0</duration>', '    <file>']
    for (w, h), p in paths:
        x.append(f'      <size width="{w}" height="{h}">{p}</size>')
    x += ['    </file>', '  </static>', '</background>', '']
    return "\n".join(x)


# ---------------------------------------------------------------- limpeza das saídas antigas
for d in (f"{TH}/wallpaper/contents/images", f"{TH}/lockscreen/contents/images", f"{ART}/backgrounds/doors", f"{ART}/wallpapers"):
    shutil.rmtree(d, ignore_errors=True)

T = "/usr/share/desktop-base/doors-theme"
# ---------------------------------------------------------------- tema (papel de parede padrão)
WP = [(1920, 1080), (2560, 1440), (3840, 2160)]
for w, h in WP:
    jpg(foto(PADRAO, w, h), f"{TH}/wallpaper/contents/images/{w}x{h}.jpg", 88)
# alternativa desktop-background do Debian aponta para 1920x1080.svg: SVG com a foto embutida
put(f"{TH}/wallpaper/contents/images/1920x1080.svg", svg_com_foto(foto(PADRAO, 1920, 1080), 1920, 1080))
put(f"{TH}/wallpaper/gnome-background.xml", bg_xml([((w, h), f"{T}/wallpaper/contents/images/{w}x{h}.jpg") for w, h in WP]))
for w, h in WP[:2]:
    jpg(escurecer(foto(PADRAO, w, h), 0.25), f"{TH}/lockscreen/contents/images/{w}x{h}.jpg", 86)
put(f"{TH}/lockscreen/gnome-background.xml", bg_xml([((w, h), f"{T}/lockscreen/contents/images/{w}x{h}.jpg") for w, h in WP[:2]]))
for k in ("wallpaper", "lockscreen"):
    put(f"{TH}/{k}/metadata.json", json.dumps({"KPlugin": {"Id": "DoorsOS" + ("" if k == "wallpaper" else "Lockscreen"),
        "Name": "Doors OS", "License": "Pexels License", "Authors": [{"Name": FOTOS[PADRAO][2] + " (Pexels)"}]}},
        ensure_ascii=False, indent=4) + "\n")

# login (LightDM/slick-greeter: background.svg; SDDM: background-nologo.svg)
login = escurecer(foto(PADRAO, 1920, 1080), 0.30)
put(f"{TH}/login/background.svg", svg_com_foto(login, 1920, 1080, wordmark(96, 930, 72, 0.95)))
put(f"{TH}/login/background-nologo.svg", svg_com_foto(login, 1920, 1080))
put(f"{TH}/login/logo-text.svg", f'<svg xmlns="http://www.w3.org/2000/svg" width="300" height="80" viewBox="0 0 300 80"><defs>{LOGO_DEFS}</defs>'
    + logo_svg(4, 4, 72) + f'<text x="88" y="54" font-family="{FONT}" font-weight="700" font-size="38" fill="#f0f4f8">Doors OS</text></svg>')

# GRUB do sistema instalado
escurecer(foto(PADRAO, 1920, 1080), 0.45).save(f"{TH}/grub/grub-16x9.png")
escurecer(foto(PADRAO, 1280, 1024), 0.45).save(f"{TH}/grub/grub-4x3.png")

# ---------------------------------------------------------------- coleção (todas as fotos)
links = ["usr/share/desktop-base/doors-theme/wallpaper/contents/images/2560x1440.jpg usr/share/backgrounds/xfce/xfce-x.svg"]
xml = ['<?xml version="1.0"?>', '<!DOCTYPE wallpapers SYSTEM "gnome-wp-list.dtd">', "<wallpapers>"]
for key, (arq, nome, autor, pid, _) in FOTOS.items():
    jpg(foto(key, 2560, 1440), f"{ART}/backgrounds/doors/{key}.jpg", 86)
    kid = "Doors" + key.capitalize()
    put(f"{ART}/wallpapers/{kid}/metadata.json", json.dumps({"KPlugin": {"Id": kid, "Name": f"Doors — {nome}",
        "License": "Pexels License", "Authors": [{"Name": f"{autor} (Pexels)"}]}}, ensure_ascii=False, indent=2) + "\n")
    links.append(f"usr/share/backgrounds/doors/{key}.jpg usr/share/wallpapers/{kid}/contents/images/2560x1440.jpg")
    xml += ['  <wallpaper deleted="false">', f"    <name>Doors — {nome}</name>",
            f"    <filename>/usr/share/backgrounds/doors/{key}.jpg</filename>", "    <options>zoom</options>", "  </wallpaper>"]
xml.append("</wallpapers>")
for d in ("gnome-background-properties", "cinnamon-background-properties", "mate-background-properties"):
    put(f"{ART}/{d}/doors.xml", "\n".join(xml) + "\n")
put(f"{R}/packages/doors-os/debian/doors-artwork.links", "\n".join(links) + "\n")

cred = ["# Créditos das fotos", "",
        "Fotos de portas usadas nos papéis de parede, login, GRUB, menu de boot e instalador do Doors OS.",
        "Licença Pexels (https://www.pexels.com/license/): uso e modificação livres, crédito opcional;",
        "não redistribuir as fotos em outros bancos de imagens ou plataformas de papéis de parede.", "",
        "| Papel de parede | Fotógrafo | Origem |", "|---|---|---|"]
for key, (arq, nome, autor, pid, _) in FOTOS.items():
    cred.append(f"| {nome}{' (padrão)' if key == PADRAO else ''} | {autor} | https://www.pexels.com/photo/{pid}/ |")
put(f"{R}/branding/fotos/CREDITOS.md", "\n".join(cred) + "\n")
put(f"{ART}/CREDITOS.md", "\n".join(cred) + "\n")

# ---------------------------------------------------------------- avatar padrão
put(f"{R}/branding/fontes/face.svg", '<svg xmlns="http://www.w3.org/2000/svg" width="256" height="256" viewBox="0 0 256 256"><defs>'
    + LOGO_DEFS + '<radialGradient id="fbg" cx="0.5" cy="0.35" r="0.75"><stop offset="0" stop-color="#2a78ad"/><stop offset="1" stop-color="#0a1a2e"/></radialGradient></defs>'
    '<rect width="256" height="256" fill="url(#fbg)"/>'
    '<g transform="translate(40,32) scale(0.75)"><rect x="78" y="40" width="100" height="184" rx="10" fill="#f0f4f8"/>'
    '<polygon points="78,40 150,58 150,238 78,224" fill="url(#lg_door)"/><circle cx="138" cy="134" r="6" fill="#1e5e8c"/></g></svg>')

# ---------------------------------------------------------------- menu de boot da ISO live (live-build troca @VERSION@)
sp = escurecer(foto(PADRAO, 800, 600), 0.40)
put(f"{R}/config/common/bootloaders/splash.svg", svg_com_foto(sp, 800, 600, wordmark(40, 40, 80) +
    f'<text x="136" y="126" font-family="{FONT}" font-size="16" fill="{GOLD}">versão @VERSION@ · base Debian trixie</text>'))

# ---------------------------------------------------------------- instalador (PNG 800x400, texto sobre a foto)
def fonte(peso, tam):
    for f in (f"{NOTO}/NotoSans-{peso}.ttf", f"/usr/share/fonts/truetype/dejavu/DejaVuSans{'-Bold' if peso == 'Bold' else ''}.ttf"):
        if os.path.exists(f):
            return ImageFont.truetype(f, tam)
    return ImageFont.load_default()


LOGO_PNG = f"{CAL}/doors-logo.png"


def banner(key, titulo, sub, out):
    w, h = 800, 400
    im = foto(key, 1000, h).crop((0, 0, w, h))   # desloca o assunto para a direita (texto à esquerda)
    # degradê escuro da esquerda para a direita (legibilidade do texto)
    g = Image.new("L", (w, 1))
    for x in range(w):
        g.putpixel((x, 0), int(238 * max(0.0, 1 - x / (w * 0.95)) ** 0.85))
    sombra = Image.new("RGB", (w, h), (8, 18, 30))
    im = Image.composite(sombra, im, g.resize((w, h)))
    d = ImageDraw.Draw(im)
    if os.path.exists(LOGO_PNG):
        lg = Image.open(LOGO_PNG).convert("RGBA").resize((76, 76), Image.LANCZOS)
        im.paste(lg, (44, 52), lg)
    d.text((134, 66), "Doors OS", font=fonte("Bold", 38), fill=(240, 244, 248))
    d.text((46, 236), titulo, font=fonte("Bold", 30), fill=(240, 244, 248))
    d.text((48, 282), sub, font=fonte("Regular", 19), fill=(255, 210, 122))
    im.save(out)


banner("lavanda", "Simples. Aberto. Seu.", "Bem-vindo ao Doors OS", f"{CAL}/welcome.png")
banner("azulejos", "Instalação em poucos minutos", "Siga as etapas — o resto é automático", f"{CAL}/slide1.png")
banner("colonial", "Seguro desde o início", "Secure Boot, atualizações assinadas e base Debian", f"{CAL}/slide2.png")
banner("casarao", "Tudo pronto para usar", "Navegador, escritório, multimídia e loja de apps", f"{CAL}/slide3.png")
print("arte gerada")
