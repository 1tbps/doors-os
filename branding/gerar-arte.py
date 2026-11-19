#!/usr/bin/env python3
"""Gera a arte do Doors OS (SVG vetorial, sem filtros — compatível com librsvg e QtSvg).
Uso: python3 -I branding/gerar-arte.py /caminho/do/repo
Saídas:
  packages/doors-os/artwork/doors-theme/   tema do desktop-base (papel de parede, bloqueio, login)
  packages/doors-os/artwork/backgrounds/   papéis de parede extras (Portal, Aurora, Geométrico, Horizonte)
  config/common/bootloaders/splash.svg     menu de boot da ISO live
  *.png (GRUB, Calamares) são gerados depois por branding/renderizar.sh (rsvg-convert)
"""
import sys, os, math, random, json

R = sys.argv[1]
ART = f"{R}/packages/doors-os/artwork"
TH = f"{ART}/doors-theme"
SIZES = ["1920x1080", "1920x1200", "1280x800", "1280x1024", "2520x1080", "1080x2160"]

NAVY, DEEP, BLUE, TEAL = "#0a1a2e", "#10294a", "#1d5487", "#1e6f9c"
AMBER, GOLD, WARM = "#ffb84c", "#ffd27a", "#fff1d2"
FONT = "Noto Sans, DejaVu Sans, sans-serif"


def put(path, data):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w") as f:
        f.write(data)


def f1(x):
    return f"{x:.1f}"


# ---------------------------------------------------------------- logo
def logo_svg(x, y, s, opacity=1.0):
    k = s / 256
    return (f'<g transform="translate({f1(x)},{f1(y)}) scale({k:.4f})" opacity="{opacity}">'
            '<rect x="8" y="8" width="240" height="240" rx="52" fill="url(#lg_bg)"/>'
            '<rect x="78" y="40" width="100" height="184" rx="10" fill="#f0f4f8"/>'
            '<polygon points="78,40 150,58 150,238 78,224" fill="url(#lg_door)"/>'
            '<circle cx="138" cy="134" r="6" fill="#1e5e8c"/></g>')


LOGO_DEFS = ('<linearGradient id="lg_bg" x1="0" y1="0" x2="1" y2="1">'
             '<stop offset="0" stop-color="#2a78ad"/><stop offset="1" stop-color="#16456e"/></linearGradient>'
             '<linearGradient id="lg_door" x1="0" y1="0" x2="1" y2="1">'
             f'<stop offset="0" stop-color="{GOLD}"/><stop offset="1" stop-color="#f59e2a"/></linearGradient>')


def wordmark(x, y, s, opacity=1.0):
    """logo + 'Doors OS' — s = altura do logo"""
    return (logo_svg(x, y, s, opacity) +
            f'<text x="{f1(x + s * 1.18)}" y="{f1(y + s * 0.68)}" font-family="{FONT}" font-weight="700" '
            f'font-size="{f1(s * 0.52)}" fill="#f0f4f8" opacity="{opacity}">Doors OS</text>')


def svg(w, h, defs, body):
    return (f'<svg xmlns="http://www.w3.org/2000/svg" width="{w}" height="{h}" viewBox="0 0 {w} {h}">'
            f'<defs>{LOGO_DEFS}{defs}</defs>{body}</svg>')


# ---------------------------------------------------------------- elementos comuns
def stars(rng, w, h, n, ymax, color="#ffffff"):
    out = []
    for _ in range(n):
        x, y = rng.uniform(0, w), rng.uniform(0, ymax)
        r = rng.choice([0.6, 0.8, 1.0, 1.2, 1.6]) * (min(w, h) / 1080)
        out.append(f'<circle cx="{f1(x)}" cy="{f1(y)}" r="{r:.2f}" fill="{color}" opacity="{rng.uniform(0.15, 0.75):.2f}"/>')
    return "".join(out)


def bokeh(rng, w, h, n, region, grads, op=(0.25, 0.8)):
    x0, y0, x1, y1 = region
    out = []
    m = min(w, h)
    for _ in range(n):
        r = m * rng.uniform(0.004, 0.035)
        out.append(f'<circle cx="{f1(rng.uniform(x0, x1) * w)}" cy="{f1(rng.uniform(y0, y1) * h)}" r="{f1(r)}" '
                   f'fill="url(#{rng.choice(grads)})" opacity="{rng.uniform(*op):.2f}"/>')
    return "".join(out)


BOKEH_DEFS = (f'<radialGradient id="bk_a"><stop offset="0" stop-color="{GOLD}" stop-opacity="0.9"/>'
              f'<stop offset="0.6" stop-color="{AMBER}" stop-opacity="0.35"/><stop offset="1" stop-color="{AMBER}" stop-opacity="0"/></radialGradient>'
              '<radialGradient id="bk_t"><stop offset="0" stop-color="#7fd3ff" stop-opacity="0.7"/>'
              '<stop offset="0.6" stop-color="#3a9bd1" stop-opacity="0.25"/><stop offset="1" stop-color="#3a9bd1" stop-opacity="0"/></radialGradient>'
              '<radialGradient id="bk_w"><stop offset="0" stop-color="#ffffff" stop-opacity="0.8"/>'
              '<stop offset="1" stop-color="#ffffff" stop-opacity="0"/></radialGradient>')


# ---------------------------------------------------------------- 1. Portal (padrão)
def portal(w, h, logo=True, dim=0.0, seed=7):
    rng = random.Random(seed)
    m = min(w, h)
    hz = h * 0.64                        # linha do horizonte (base da porta)
    cx = w * (0.64 if w >= h else 0.5)   # centro da porta
    dh = m * 0.40; dw = dh * 0.46        # altura/largura do vão
    dx0, dx1, dy0 = cx - dw / 2, cx + dw / 2, hz - dh
    hy = hz - dh * 0.45                  # centro do halo
    defs = BOKEH_DEFS + (
        f'<linearGradient id="sky" x1="0" y1="0" x2="0" y2="1"><stop offset="0" stop-color="{NAVY}"/>'
        f'<stop offset="0.55" stop-color="{DEEP}"/><stop offset="1" stop-color="{BLUE}"/></linearGradient>'
        f'<radialGradient id="halo" gradientUnits="userSpaceOnUse" cx="{f1(cx)}" cy="{f1(hy)}" r="{f1(m * 0.75)}">'
        f'<stop offset="0" stop-color="{AMBER}" stop-opacity="0.26"/><stop offset="0.3" stop-color="{AMBER}" stop-opacity="0.07"/>'
        f'<stop offset="1" stop-color="{AMBER}" stop-opacity="0"/></radialGradient>'
        f'<radialGradient id="dglow" gradientUnits="userSpaceOnUse" cx="{f1(cx)}" cy="{f1(hz - dh * 0.5)}" r="{f1(dh * 0.95)}">'
        f'<stop offset="0" stop-color="{GOLD}" stop-opacity="0.55"/><stop offset="0.4" stop-color="{AMBER}" stop-opacity="0.18"/>'
        f'<stop offset="1" stop-color="{AMBER}" stop-opacity="0"/></radialGradient>'
        f'<linearGradient id="floor" x1="0" y1="0" x2="0" y2="1"><stop offset="0" stop-color="#0c2238"/>'
        f'<stop offset="1" stop-color="{NAVY}"/></linearGradient>'
        f'<linearGradient id="opening" x1="0" y1="0" x2="0" y2="1"><stop offset="0" stop-color="{WARM}"/>'
        f'<stop offset="0.7" stop-color="{GOLD}"/><stop offset="1" stop-color="{AMBER}"/></linearGradient>'
        f'<linearGradient id="beam" x1="0" y1="0" x2="0" y2="1"><stop offset="0" stop-color="{GOLD}" stop-opacity="0.75"/>'
        f'<stop offset="0.5" stop-color="{AMBER}" stop-opacity="0.22"/><stop offset="1" stop-color="{AMBER}" stop-opacity="0"/></linearGradient>'
        f'<linearGradient id="leaf" x1="0" y1="0" x2="1" y2="0"><stop offset="0" stop-color="#1b4d78"/>'
        f'<stop offset="1" stop-color="#0f2e4c"/></linearGradient>'
        f'<linearGradient id="dimg" x1="0" y1="0" x2="0" y2="1"><stop offset="0" stop-color="#000" stop-opacity="{dim:.2f}"/>'
        f'<stop offset="1" stop-color="#000" stop-opacity="{dim * 0.6:.2f}"/></linearGradient>')
    b = [f'<rect width="{w}" height="{h}" fill="url(#sky)"/>',
         stars(rng, w, h, int(260 * w * h / (1920 * 1080)), hz * 0.9),
         f'<rect width="{w}" height="{h}" fill="url(#halo)"/>']
    # colinas distantes
    for i, (col, amp, base) in enumerate([("#0f2a44", 0.05, 0.03), ("#0c2238", 0.035, 0.0)]):
        pts = [f"0,{f1(h)}"]
        n = 24
        for k in range(n + 1):
            x = w * k / n
            y = hz - h * base - h * amp * (0.5 + 0.5 * math.sin(k * 0.9 + i * 2.1)) * rng.uniform(0.6, 1.0)
            pts.append(f"{f1(x)},{f1(y)}")
        pts.append(f"{w},{f1(h)}")
        b.append(f'<polygon points="{" ".join(pts)}" fill="{col}"/>')
    b.append(f'<rect x="0" y="{f1(hz)}" width="{w}" height="{f1(h - hz)}" fill="url(#floor)"/>')
    # grade em perspectiva no piso
    vx, vy = cx, hz - dh * 0.35
    for k in range(-14, 15):
        xb = cx + k * w * 0.09
        t = (hz - vy) / (h - vy)
        xh = vx + (xb - vx) * t
        b.append(f'<line x1="{f1(xh)}" y1="{f1(hz)}" x2="{f1(xb)}" y2="{h}" stroke="#3a7fb8" stroke-width="{f1(m * 0.0012)}" opacity="0.18"/>')
    for k in range(1, 9):
        y = hz + (h - hz) * (k / 9) ** 1.8
        b.append(f'<line x1="0" y1="{f1(y)}" x2="{w}" y2="{f1(y)}" stroke="#3a7fb8" stroke-width="{f1(m * 0.0012)}" opacity="{0.06 + 0.02 * k:.2f}"/>')
    # raios de luz saindo da porta
    for k in range(16):
        a = math.radians(-172 + k * 11.5 + rng.uniform(-3, 3))
        L = m * rng.uniform(0.5, 1.0); sp = math.radians(rng.uniform(1.2, 3.2))
        ox, oy = cx, hz - dh * 0.5
        p1 = (ox + L * math.cos(a - sp), oy + L * math.sin(a - sp)); p2 = (ox + L * math.cos(a + sp), oy + L * math.sin(a + sp))
        b.append(f'<polygon points="{f1(ox)},{f1(oy)} {f1(p1[0])},{f1(p1[1])} {f1(p2[0])},{f1(p2[1])}" fill="{GOLD}" opacity="{rng.uniform(0.015, 0.04):.3f}"/>')
    b.append(f'<rect width="{w}" height="{h}" fill="url(#dglow)"/>')
    # feixe de luz no chão
    b.append(f'<polygon points="{f1(dx0)},{f1(hz)} {f1(dx1)},{f1(hz)} {f1(cx + dw * 2.6)},{h} {f1(cx - dw * 1.4)},{h}" fill="url(#beam)"/>')
    # moldura, vão iluminado e folha da porta aberta
    fr = m * 0.012
    b.append(f'<rect x="{f1(dx0 - fr)}" y="{f1(dy0 - fr)}" width="{f1(dw + 2 * fr)}" height="{f1(dh + fr)}" rx="{f1(fr * 0.6)}" fill="#0a1a2b"/>')
    b.append(f'<rect x="{f1(dx0)}" y="{f1(dy0)}" width="{f1(dw)}" height="{f1(dh)}" fill="url(#opening)"/>')
    lw = dw * 0.55
    b.append(f'<polygon points="{f1(dx0)},{f1(dy0)} {f1(dx0 - lw)},{f1(dy0 - dh * 0.07)} {f1(dx0 - lw)},{f1(hz + dh * 0.07)} {f1(dx0)},{f1(hz)}" fill="url(#leaf)"/>')
    b.append(f'<circle cx="{f1(dx0 - lw * 0.82)}" cy="{f1(hz - dh * 0.46)}" r="{f1(m * 0.005)}" fill="{AMBER}"/>')
    # partículas de luz
    b.append(bokeh(rng, w, h, int(22 * max(1, w / h)), (0.05, 0.08, 0.98, 0.72), ["bk_a", "bk_a", "bk_t", "bk_w"], (0.12, 0.5)))
    if logo:
        s = m * 0.075
        b.append(wordmark(w * 0.05, h * 0.86 - s, s, 0.92))
    if dim:
        b.append(f'<rect width="{w}" height="{h}" fill="url(#dimg)"/>')
    return svg(w, h, defs, "".join(b))


# ---------------------------------------------------------------- 2. Aurora
def aurora(w, h, seed=11):
    rng = random.Random(seed)
    defs = BOKEH_DEFS + (
        f'<linearGradient id="bg" x1="0" y1="0" x2="1" y2="1"><stop offset="0" stop-color="{NAVY}"/>'
        f'<stop offset="1" stop-color="#0d2c47"/></linearGradient>')
    cols = [("#1e88c8", "#5fd0ff"), ("#14507e", "#2a9bd8"), (AMBER, GOLD), ("#2a6fa8", "#7fd3ff"), ("#f07e3c", AMBER)]
    for i, (a, c) in enumerate(cols):
        defs += (f'<linearGradient id="rb{i}" x1="0" y1="0" x2="1" y2="0"><stop offset="0" stop-color="{a}" stop-opacity="0"/>'
                 f'<stop offset="0.35" stop-color="{a}" stop-opacity="0.75"/><stop offset="0.7" stop-color="{c}" stop-opacity="0.6"/>'
                 f'<stop offset="1" stop-color="{c}" stop-opacity="0"/></linearGradient>')
    b = [f'<rect width="{w}" height="{h}" fill="url(#bg)"/>', stars(rng, w, h, int(160 * w * h / (1920 * 1080)), h * 0.5)]
    n = 60
    for i, _ in enumerate(cols):
        ph, fr, yc, amp, th = rng.uniform(0, 6), rng.uniform(1.2, 2.2), h * (0.38 + 0.09 * i), h * rng.uniform(0.06, 0.12), h * rng.uniform(0.015, 0.06)
        for layer in range(3):
            off = layer * th * 0.7
            top, bot = [], []
            for k in range(n + 1):
                x = w * k / n; t = k / n
                y = yc + amp * math.sin(t * math.pi * fr + ph) + off
                thick = th * (0.4 + 0.6 * math.sin(t * math.pi)) * (1 - layer * 0.25)
                top.append(f"{f1(x)},{f1(y - thick)}"); bot.append(f"{f1(x)},{f1(y + thick)}")
            b.append(f'<polygon points="{" ".join(top + bot[::-1])}" fill="url(#rb{i})" opacity="{0.55 - layer * 0.15:.2f}"/>')
    b.append(bokeh(rng, w, h, 30, (0, 0.2, 1, 0.9), ["bk_t", "bk_w", "bk_a"]))
    s = min(w, h) * 0.06
    b.append(wordmark(w * 0.05, h * 0.9 - s, s, 0.85))
    return svg(w, h, defs, "".join(b))


# ---------------------------------------------------------------- 3. Geométrico (low-poly)
def lerp(a, b, t):
    return tuple(int(a[i] + (b[i] - a[i]) * t) for i in range(3))


def hexc(c):
    return "#%02x%02x%02x" % c


def geometrico(w, h, seed=5):
    rng = random.Random(seed)
    nx, ny = 18, 11
    pts = [[(w * (i / nx) + (rng.uniform(-0.4, 0.4) * w / nx if 0 < i < nx else 0),
             h * (j / ny) + (rng.uniform(-0.4, 0.4) * h / ny if 0 < j < ny else 0)) for i in range(nx + 1)] for j in range(ny + 1)]
    c0, c1, c2, ca = (8, 19, 31), (23, 64, 106), (35, 110, 160), (255, 184, 76)
    hx, hy = w * 0.78, h * 0.3
    b = []

    def color(x, y):
        t = (x / w * 0.55 + (1 - y / h) * 0.45)
        base = lerp(c0, c1, min(1, t * 1.3)) if t < 0.75 else lerp(c1, c2, (t - 0.75) * 4)
        d = math.hypot((x - hx) / w, (y - hy) / h)
        glow = max(0, 1 - d / 0.33) ** 2
        c = lerp(base, ca, glow * 0.85)
        return lerp(c, (c[0] + 10, c[1] + 10, c[2] + 12), rng.uniform(0, 1) * 0.6)

    for j in range(ny):
        for i in range(nx):
            a, bb, c, d = pts[j][i], pts[j][i + 1], pts[j + 1][i], pts[j + 1][i + 1]
            for tri in ((a, bb, d), (a, d, c)) if (i + j) % 2 else ((a, bb, c), (bb, d, c)):
                gx, gy = sum(p[0] for p in tri) / 3, sum(p[1] for p in tri) / 3
                col = hexc(tuple(min(255, v) for v in color(gx, gy)))
                b.append(f'<polygon points="{" ".join(f"{f1(p[0])},{f1(p[1])}" for p in tri)}" fill="{col}" stroke="{col}" stroke-width="0.8"/>')
    for j in range(ny + 1):
        for i in range(nx + 1):
            x, y = pts[j][i]
            if rng.random() < 0.18:
                b.append(f'<circle cx="{f1(x)}" cy="{f1(y)}" r="{f1(min(w, h) * 0.0025)}" fill="{GOLD}" opacity="0.5"/>')
    s = min(w, h) * 0.06
    b.append(wordmark(w * 0.05, h * 0.9 - s, s, 0.9))
    return svg(w, h, "", "".join(b))


# ---------------------------------------------------------------- 4. Horizonte
def horizonte(w, h, seed=3):
    rng = random.Random(seed)
    sx, sy = w * 0.68, h * 0.56
    defs = (f'<linearGradient id="sk" x1="0" y1="0" x2="0" y2="1"><stop offset="0" stop-color="{NAVY}"/>'
            '<stop offset="0.45" stop-color="#1b3a63"/><stop offset="0.75" stop-color="#6a4a6e"/>'
            f'<stop offset="1" stop-color="#e48a4a"/></linearGradient>'
            f'<radialGradient id="sun" cx="0.5" cy="0.5" r="0.5"><stop offset="0" stop-color="{WARM}"/>'
            f'<stop offset="0.35" stop-color="{GOLD}" stop-opacity="0.9"/><stop offset="1" stop-color="{AMBER}" stop-opacity="0"/></radialGradient>'
            '<linearGradient id="mist" x1="0" y1="0" x2="0" y2="1"><stop offset="0" stop-color="#ffffff" stop-opacity="0"/>'
            '<stop offset="0.5" stop-color="#ffd9b0" stop-opacity="0.16"/><stop offset="1" stop-color="#ffffff" stop-opacity="0"/></linearGradient>')
    ridges = [("#4a3d63", "#2c3557"), ("#2d3a5e", "#1c2a48"), ("#1a2b4a", "#122038"), ("#10203a", "#0b1828"), ("#0a1626", "#070f1a")]
    for i, (a, c) in enumerate(ridges):
        defs += f'<linearGradient id="rg{i}" x1="0" y1="0" x2="0" y2="1"><stop offset="0" stop-color="{a}"/><stop offset="1" stop-color="{c}"/></linearGradient>'
    m = min(w, h)
    b = [f'<rect width="{w}" height="{h}" fill="url(#sk)"/>', stars(rng, w, h, int(140 * w * h / (1920 * 1080)), h * 0.35),
         f'<circle cx="{f1(sx)}" cy="{f1(sy)}" r="{f1(m * 0.42)}" fill="url(#sun)" opacity="0.55"/>',
         f'<circle cx="{f1(sx)}" cy="{f1(sy)}" r="{f1(m * 0.07)}" fill="{WARM}" opacity="0.95"/>']
    for i in range(len(ridges)):
        base = h * (0.60 + i * 0.085); amp = h * (0.13 - i * 0.015)
        ph = [rng.uniform(0, 6.3) for _ in range(4)]
        fq = [rng.uniform(1.5, 2.5), rng.uniform(3.5, 5.5), rng.uniform(8, 11), rng.uniform(17, 23)]
        wt = [0.55, 0.28, 0.12, 0.05]
        pts = [f"0,{h}"]
        n = 120
        for k in range(n + 1):
            t = k / n
            v = sum(wt[j] * math.sin(t * fq[j] * math.pi + ph[j]) for j in range(4))
            pts.append(f"{f1(w * t)},{f1(base - amp * (0.5 + 0.6 * v))}")
        pts.append(f"{w},{h}")
        b.append(f'<polygon points="{" ".join(pts)}" fill="url(#rg{i})"/>')
        if i < 3:
            b.append(f'<rect x="0" y="{f1(base - amp * 0.05)}" width="{w}" height="{f1(amp * 0.7)}" fill="url(#mist)" opacity="0.6"/>')
    s = m * 0.06
    b.append(wordmark(w * 0.05, h * 0.92 - s, s, 0.9))
    return svg(w, h, defs, "".join(b))


# ---------------------------------------------------------------- saídas
for sz in SIZES:
    w, h = map(int, sz.split("x"))
    put(f"{TH}/wallpaper/contents/images/{sz}.svg", portal(w, h))
    put(f"{TH}/lockscreen/contents/images/{sz}.svg", portal(w, h, logo=False, dim=0.25))
put(f"{TH}/login/background.svg", portal(1920, 1080, logo=True, dim=0.15))
put(f"{TH}/login/background-nologo.svg", portal(1920, 1080, logo=False, dim=0.15))
put(f"{TH}/login/logo-text.svg", f'<svg xmlns="http://www.w3.org/2000/svg" width="300" height="80" viewBox="0 0 300 80"><defs>{LOGO_DEFS}</defs>'
    + logo_svg(4, 4, 72) + f'<text x="88" y="54" font-family="{FONT}" font-weight="700" font-size="38" fill="#f0f4f8">Doors OS</text></svg>')

EXTRAS = {"portal": ("Doors Portal", portal), "aurora": ("Doors Aurora", aurora),
          "geometrico": ("Doors Geométrico", geometrico), "horizonte": ("Doors Horizonte", horizonte)}
xml = ['<?xml version="1.0"?>', '<!DOCTYPE wallpapers SYSTEM "gnome-wp-list.dtd">', "<wallpapers>"]
for key, (name, fn) in EXTRAS.items():
    data = fn(2560, 1440)
    put(f"{ART}/backgrounds/doors/{key}.svg", data)                                       # XFCE / GNOME / Cinnamon
    put(f"{ART}/wallpapers/Doors{key.capitalize()}/contents/images/2560x1440.svg", data)  # KDE
    put(f"{ART}/wallpapers/Doors{key.capitalize()}/metadata.json", json.dumps(
        {"KPlugin": {"Id": f"Doors{key.capitalize()}", "Name": name, "License": "GPLv2+",
                     "Authors": [{"Name": "Doors OS Project"}]}}, ensure_ascii=False, indent=2) + "\n")
    xml += ["  <wallpaper deleted=\"false\">", f"    <name>{name}</name>",
            f"    <filename>/usr/share/backgrounds/doors/{key}.svg</filename>",
            "    <options>zoom</options>", "  </wallpaper>"]
xml.append("</wallpapers>")
for d in ("gnome-background-properties", "cinnamon-background-properties", "mate-background-properties"):
    put(f"{ART}/{d}/doors.xml", "\n".join(xml) + "\n")

# fontes p/ PNG (renderizados por branding/renderizar.sh)
put(f"{R}/branding/fontes/grub-16x9.svg", portal(1920, 1080, logo=False, dim=0.35))
put(f"{R}/branding/fontes/grub-4x3.svg", portal(1280, 1024, logo=False, dim=0.35))


def banner(title, sub, seed):
    w, h = 800, 400
    base = portal(w, h, logo=False, seed=seed)
    s = 86
    extra = (wordmark(48, 60, s) +
             f'<text x="50" y="250" font-family="{FONT}" font-weight="700" font-size="30" fill="#f0f4f8">{title}</text>'
             f'<text x="50" y="290" font-family="{FONT}" font-size="19" fill="{GOLD}">{sub}</text>')
    return base.replace("</svg>", extra + "</svg>")


put(f"{R}/branding/fontes/welcome.svg", banner("Simples. Aberto. Seu.", "Bem-vindo ao Doors OS", 7))
put(f"{R}/branding/fontes/slide1.svg", banner("Instalação em poucos minutos", "Basta seguir as etapas — o resto é automático", 9))
put(f"{R}/branding/fontes/slide2.svg", banner("Seguro desde o início", "Secure Boot, atualizações assinadas e base Debian", 13))
put(f"{R}/branding/fontes/slide3.svg", banner("Tudo pronto para usar", "Navegador, escritório, multimídia e loja de apps", 21))

# avatar padrão dos usuários (/etc/skel/.face) — recortado em círculo pelos greeters/menus
put(f"{R}/branding/fontes/face.svg", '<svg xmlns="http://www.w3.org/2000/svg" width="256" height="256" viewBox="0 0 256 256"><defs>'
    + LOGO_DEFS + f'<radialGradient id="fbg" cx="0.5" cy="0.35" r="0.75"><stop offset="0" stop-color="#2a78ad"/><stop offset="1" stop-color="{NAVY}"/></radialGradient></defs>'
    '<rect width="256" height="256" fill="url(#fbg)"/>'
    '<g transform="translate(40,32) scale(0.75)"><rect x="78" y="40" width="100" height="184" rx="10" fill="#f0f4f8"/>'
    '<polygon points="78,40 150,58 150,238 78,224" fill="url(#lg_door)"/><circle cx="138" cy="134" r="6" fill="#1e5e8c"/></g></svg>')

# menu de boot da ISO live (live-build substitui @VERSION@)
sp = portal(800, 600, logo=False, dim=0.2, seed=7).replace("</svg>",
    wordmark(40, 40, 80) +
    f'<text x="136" y="126" font-family="{FONT}" font-size="16" fill="{GOLD}">versão @VERSION@ · base Debian trixie</text></svg>')
put(f"{R}/config/common/bootloaders/splash.svg", sp)
print("arte gerada")
