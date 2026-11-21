#!/usr/bin/env python3
"""Gera a arte do Doors OS a partir das ilustrações do projeto + logo vetorial.
Roda DENTRO do container de build (PIL + fontes Noto): branding/renderizar.sh
Uso: python3 -I branding/gerar-arte.py /caminho/do/repo

Ilustrações: branding/fontes/papeis/*.jpg (arte do autor do projeto, 2560x1440) 
Saídas:
  packages/doors-os/artwork/doors-theme/   tema do desktop-base (papel de parede, bloqueio, login, GRUB)
  packages/doors-os/artwork/backgrounds/   coleção de papéis de parede
  packages/doors-os/artwork/wallpapers/    pacotes de papel de parede do KDE (links para backgrounds/)
  packages/doors-os/artwork/plymouth/doors tela de inicialização (Plymouth)
  packages/doors-os/debian/doors-artwork.links (gerado)
  config/common/bootloaders/splash.svg     menu de boot da ISO live
  config/desktop-common/.../calamares/branding/doors/{welcome,slide1..3}.png
"""
import sys, os, io, json, base64, shutil
from PIL import Image, ImageOps, ImageDraw, ImageFont, ImageEnhance

R = sys.argv[1]
ART = f"{R}/packages/doors-os/artwork"
TH = f"{ART}/doors-theme"
PAP = f"{R}/branding/fontes/papeis"
CAL = f"{R}/config/desktop-common/includes.chroot/etc/calamares/branding/doors"
FONT = "Noto Sans, DejaVu Sans, sans-serif"
NOTO = "/usr/share/fonts/truetype/noto"
AMBER, GOLD = "#ffb84c", "#ffd27a"

# chave: (arquivo relativo a branding/fontes, nome exibido); a ordem é a da coleção
AUTOR = "Doors OS Project"
ILUSTRACOES = {
    "colina": ("papeis/colina.jpg", "Colina ensolarada"),
    "bibliotecadia": ("papeis/bibliotecadia.jpg", "Biblioteca mágica (dia)"),
    "bibliotecanoite": ("papeis/bibliotecanoite.jpg", "Biblioteca mágica (noite)"),
    "cidadefuturista": ("papeis/cidadefuturista.jpg", "Cidade futurista chuvosa"),
    "dunas": ("papeis/dunas.jpg", "Dunas do deserto ao pôr do sol"),
    "aurora": ("papeis/aurora.jpg", "Floresta nevada com aurora boreal"),
    "oceano": ("papeis/oceano.jpg", "Fundo do oceano"),
    "greenline": ("papeis/greenline.jpg", "Green Line"),
    "ilhaflutuante": ("papeis/ilhaflutuante.jpg", "Ilha flutuante no espaço sideral"),
    "portaencantada": ("papeis/portaencantada.jpg", "Portal na floresta"),
    "portalinferno": ("papeis/portalinferno.jpg", "Portal no Inferno"),
    "tunel": ("papeis/tunel.jpg", "Túnel subterrâneo antigo"),
}
PADRAO = "colina"


def put(path, data, mode="w"):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, mode) as f:
        f.write(data)


def foto(key, w, h):
    im = Image.open(f"{R}/branding/fontes/{ILUSTRACOES[key][0]}").convert("RGB")
    return ImageOps.fit(im, (w, h), Image.LANCZOS, centering=(0.5, 0.5))


def escurecer(im, f):
    return ImageEnhance.Brightness(im).enhance(1 - f)


def jpg(im, path, q=86):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    im.save(path, "JPEG", quality=q, optimize=True, progressive=True)


def data_uri(im, q=82):
    b = io.BytesIO(); im.save(b, "JPEG", quality=q, optimize=True, progressive=True)
    return "data:image/jpeg;base64," + base64.b64encode(b.getvalue()).decode()


# ---------------------------------------------------------------- logo (vetorial)
# Logo oficial: porta de madeira em arco (desenho original, branding/fontes/icone-porta-arco.svg, 512x512)
_LOGO = open(f"{R}/branding/fontes/icone-porta-arco.svg").read()
LOGO_DEFS = _LOGO.split("<defs>", 1)[1].split("</defs>", 1)[0]
_LOGO_BODY = _LOGO.split("</defs>", 1)[1].rsplit("</svg>", 1)[0]


def logo_svg(x, y, s, opacity=1.0):
    """logo dentro de um quadrado s×s com canto em (x, y)"""
    return (f'<g transform="translate({x:.1f},{y:.1f}) scale({s / 512:.4f})" opacity="{opacity}">'
            f'{_LOGO_BODY}</g>')


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
        "Name": "Doors OS", "License": "Doors OS", "Authors": [{"Name": AUTOR}]}},
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
for key, (arq, nome) in ILUSTRACOES.items():
    jpg(foto(key, 2560, 1440), f"{ART}/backgrounds/doors/{key}.jpg", 86)
    kid = "Doors" + key.capitalize()
    put(f"{ART}/wallpapers/{kid}/metadata.json", json.dumps({"KPlugin": {"Id": kid, "Name": f"Doors — {nome}",
        "License": "Doors OS", "Authors": [{"Name": AUTOR}]}},
        ensure_ascii=False, indent=2) + "\n")
    links.append(f"usr/share/backgrounds/doors/{key}.jpg usr/share/wallpapers/{kid}/contents/images/2560x1440.jpg")
    xml += ['  <wallpaper deleted="false">', f"    <name>Doors — {nome}</name>",
            f"    <filename>/usr/share/backgrounds/doors/{key}.jpg</filename>", "    <options>zoom</options>", "  </wallpaper>"]
xml.append("</wallpapers>")
for d in ("gnome-background-properties", "cinnamon-background-properties", "mate-background-properties"):
    put(f"{ART}/{d}/doors.xml", "\n".join(xml) + "\n")
put(f"{R}/packages/doors-os/debian/doors-artwork.links", "\n".join(links) + "\n")

cred = ["# Créditos dos papéis de parede", "",
        "Ilustrações usadas nos papéis de parede, login, GRUB, tela de inicialização, menu de boot e",
        "instalador do Doors OS. Todas são arte original do projeto.", "",
        "| Papel de parede | Autor |", "|---|---|"]
for key, (arq, nome) in ILUSTRACOES.items():
    cred.append(f"| {nome}{' (padrão)' if key == PADRAO else ''} | {AUTOR} |")
put(f"{ART}/CREDITOS.md", "\n".join(cred) + "\n")

# ---------------------------------------------------------------- avatar padrão
put(f"{R}/branding/fontes/face.svg", '<svg xmlns="http://www.w3.org/2000/svg" width="256" height="256" viewBox="0 0 256 256"><defs>'
    + LOGO_DEFS + '<radialGradient id="fbg" cx="0.5" cy="0.35" r="0.75"><stop offset="0" stop-color="#cfe9fb"/><stop offset="1" stop-color="#7fb6de"/></radialGradient></defs>'
    '<rect width="256" height="256" fill="url(#fbg)"/>' + logo_svg(38, 34, 180) + '</svg>')

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


banner(PADRAO, "Simples. Aberto. Seu.", "Bem-vindo ao Doors OS", f"{CAL}/welcome.png")
banner("bibliotecadia", "Instalação em poucos minutos", "Siga as etapas — o resto é automático", f"{CAL}/slide1.png")
banner("aurora", "Seguro desde o início", "Secure Boot, atualizações assinadas e base Debian", f"{CAL}/slide2.png")
banner("cidadefuturista", "Tudo pronto para usar", "Navegador, escritório, multimídia e loja de apps", f"{CAL}/slide3.png")

# ---------------------------------------------------------------- tela de inicialização (Plymouth, tema "doors")
PLY = f"{ART}/plymouth/doors"
os.makedirs(PLY, exist_ok=True)   # doors.plymouth e doors.script são fixos (não gerados)
escurecer(foto(PADRAO, 1920, 1080), 0.35).save(f"{PLY}/fundo.png")
marca = Image.new("RGBA", (520, 140), (0, 0, 0, 0))
if os.path.exists(LOGO_PNG):
    lg = Image.open(LOGO_PNG).convert("RGBA").resize((120, 120), Image.LANCZOS)
    marca.paste(lg, (0, 10), lg)
ImageDraw.Draw(marca).text((140, 30), "Doors OS", font=fonte("Bold", 64), fill=(240, 244, 248, 255))
marca = marca.crop(marca.getbbox()); marca.save(f"{PLY}/marca.png")
for nome_, tam in (("ponto", 14),):
    pt = Image.new("RGBA", (tam * 4, tam * 4), (0, 0, 0, 0))
    ImageDraw.Draw(pt).ellipse((0, 0, tam * 4 - 1, tam * 4 - 1), fill=(255, 210, 122, 255))
    pt.resize((tam, tam), Image.LANCZOS).save(f"{PLY}/{nome_}.png")

# ---------------------------------------------------------------- instalador: prévia da opção "software de terceiros"
AMB = f"{R}/config/editions/desktop/includes.chroot/usr/share/doors/ambientes"
if os.path.isdir(AMB):
    w, h = 640, 400
    im = escurecer(foto(PADRAO, w, h), 0.55)
    d = ImageDraw.Draw(im)
    d.rounded_rectangle((40, 40, w - 40, h - 40), 18, fill=(14, 24, 38), outline=(47, 120, 181), width=3)
    d.text((72, 74), "Software de terceiros", font=fonte("Bold", 30), fill=(240, 244, 248))
    itens = [("1", "Codecs de áudio e vídeo", "MP3, H.264, HEVC e outros formatos"),
             ("2", "Driver da NVIDIA", "instalado só se houver placa NVIDIA"),
             ("3", "Precisa de internet", "vem dos repositórios contrib e non-free do Debian")]
    for i, (ic, t1, t2) in enumerate(itens):
        y = 140 + i * 70
        d.ellipse((72, y, 116, y + 44), fill=(47, 120, 181))
        d.text((94, y + 22), ic, font=fonte("Bold", 24), fill=(255, 255, 255), anchor="mm")
        d.text((134, y + 1), t1, font=fonte("Bold", 21), fill=(240, 244, 248))
        d.text((134, y + 27), t2, font=fonte("Regular", 16), fill=(255, 210, 122))
    im.resize((560, 350), Image.LANCZOS).save(f"{AMB}/terceiros.png")

# ---------------------------------------------------------------- logos do sistema (alternativa "vendor-logos" do desktop-base;
# usados pela tela de login do GNOME/GDM e outros). Mesmos nomes de arquivo do debian-logos.
VL = f"{ART}/doors-logos"
shutil.rmtree(VL, ignore_errors=True); os.makedirs(VL)
if os.path.exists(LOGO_PNG):
    base = Image.open(LOGO_PNG).convert("RGBA")
    for t in (64, 128, 256):
        base.resize((t, t), Image.LANCZOS).save(f"{VL}/logo-{t}.png")
        lg = base.resize((t, t), Image.LANCZOS)
        f = fonte("Bold", int(t * 0.55))
        larg = int(ImageDraw.Draw(Image.new("RGBA", (1, 1))).textlength("Doors OS", font=f))
        im = Image.new("RGBA", (t + int(t * 0.18) + larg + 4, t), (0, 0, 0, 0))
        im.paste(lg, (0, 0), lg)
        ImageDraw.Draw(im).text((t + int(t * 0.18), t // 2), "Doors OS", font=f, fill=(240, 244, 248, 255), anchor="lm")
        for nome in ("logo-text", "logo-text-version"):
            im.save(f"{VL}/{nome}-{t}.png")
shutil.copy(f"{R}/branding/fontes/icone-porta-arco.svg", f"{VL}/logo.svg")
for nome in ("logo-text.svg", "logo-text-version.svg"):
    shutil.copy(f"{TH}/login/logo-text.svg", f"{VL}/{nome}")
print("arte gerada")
