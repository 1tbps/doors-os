//! Aplica as escolhas no ambiente gráfico em uso (KDE Plasma, Cinnamon ou XFCE),
//! usando as ferramentas de cada um (plasma-apply-*, gsettings, xfconf-query, gdbus).

use crate::estado::Estado;
use crate::tema::{self, Modo, Painel};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ambiente {
    Kde,
    Cinnamon,
    Xfce,
    Gnome,
    Outro,
}

impl Ambiente {
    pub fn detectar() -> Ambiente {
        let d = std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default().to_uppercase();
        if d.contains("KDE") {
            Ambiente::Kde
        } else if d.contains("CINNAMON") {
            Ambiente::Cinnamon
        } else if d.contains("XFCE") {
            Ambiente::Xfce
        } else if d.contains("GNOME") {
            Ambiente::Gnome
        } else {
            Ambiente::Outro
        }
    }

    pub fn nome(self) -> &'static str {
        match self {
            Ambiente::Kde => "KDE Plasma",
            Ambiente::Cinnamon => "Cinnamon",
            Ambiente::Xfce => "XFCE",
            Ambiente::Gnome => "GNOME",
            Ambiente::Outro => "ambiente não suportado",
        }
    }

    /// O ambiente tem a página Layout (posição do painel)?
    pub fn tem_layout(self) -> bool {
        !matches!(self, Ambiente::Gnome | Ambiente::Outro)
    }

    /// Tema claro/escuro + cor de destaque.
    pub fn aplicar_tema(self, e: &Estado) -> Result<(), String> {
        match self {
            Ambiente::Kde => kde_tema(e),
            Ambiente::Cinnamon => {
                let nome = gtk_tema_pessoal(e)?;
                gsettings("org.cinnamon.desktop.interface", "gtk-theme", &nome)?;
                preferencia_escura(e.modo);
                Ok(())
            }
            Ambiente::Xfce => {
                let nome = gtk_tema_pessoal(e)?;
                xfconf("xsettings", "/Net/ThemeName", &nome)?;
                xfconf("xfwm4", "/general/theme", &nome)?;
                preferencia_escura(e.modo);
                Ok(())
            }
            Ambiente::Gnome => {
                let escuro = e.modo == Modo::Escuro;
                gsettings("org.gnome.desktop.interface", "color-scheme", if escuro { "'prefer-dark'" } else { "'default'" })?;
                gsettings("org.gnome.desktop.interface", "gtk-theme", if escuro { "'Adwaita-dark'" } else { "'Adwaita'" })?;
                gsettings("org.gnome.desktop.interface", "accent-color", &format!("'{}'", tema::destaque_gnome(&e.cor)))?;
                // borda da janela ativa (extensão de cantos arredondados, esquema dentro da extensão)
                let dir = "/usr/share/gnome-shell/extensions/rounded-window-corners@fxgn/schemas";
                if Path::new(dir).is_dir() {
                    executar(
                        "gsettings",
                        &["--schemadir", dir, "set", "org.gnome.shell.extensions.rounded-window-corners-reborn",
                          "global-rounded-corner-settings", &tema::cantos_gnome(&e.cor)],
                    )?;
                }
                Ok(())
            }
            Ambiente::Outro => Err(nao_suportado()),
        }
    }

    pub fn aplicar_painel(self, painel: Painel) -> Result<(), String> {
        match self {
            Ambiente::Kde => {
                let local = if painel == Painel::EmCima { "top" } else { "bottom" };
                plasma_script(&format!("panels().forEach(function(p) {{ p.location = \"{local}\"; }});"))
            }
            Ambiente::Cinnamon => {
                let atual = saida("gsettings", &["get", "org.cinnamon", "panels-enabled"])?;
                gsettings("org.cinnamon", "panels-enabled", &tema::painel_cinnamon(atual.trim(), painel))
            }
            Ambiente::Xfce => {
                let ids = saida("xfconf-query", &["-c", "xfce4-panel", "-p", "/panels"])?;
                let mut algum = false;
                for id in ids.lines().filter_map(|l| l.trim().parse::<u32>().ok()) {
                    xfconf("xfce4-panel", &format!("/panels/panel-{id}/position"), tema::painel_xfce(painel))?;
                    algum = true;
                }
                if algum { Ok(()) } else { Err("nenhum painel do XFCE encontrado".into()) }
            }
            Ambiente::Gnome => Err("no GNOME a barra fica no topo, do jeito do GNOME".into()),
            Ambiente::Outro => Err(nao_suportado()),
        }
    }

    pub fn aplicar_papel(self, arquivo: &str) -> Result<(), String> {
        match self {
            Ambiente::Kde => executar("plasma-apply-wallpaperimage", &[arquivo]),
            Ambiente::Cinnamon => gsettings(
                "org.cinnamon.desktop.background",
                "picture-uri",
                &format!("'file://{arquivo}'"),
            ),
            Ambiente::Gnome => {
                let uri = format!("'file://{arquivo}'");
                gsettings("org.gnome.desktop.background", "picture-uri", &uri)?;
                gsettings("org.gnome.desktop.background", "picture-uri-dark", &uri)
            }
            Ambiente::Xfce => {
                // O XFCE 4.20 usa /backdrop/screen0/monitor<SAÍDA>/workspace<N>/last-image
                // (ex.: monitorVirtual-1, monitorHDMI-1). Propriedades com outros nomes
                // (monitor0, de configurações antigas) também são atualizadas.
                let mut props: Vec<String> = saida("xfconf-query", &["-c", "xfce4-desktop", "-l"])
                    .unwrap_or_default()
                    .lines()
                    .filter(|l| l.ends_with("/last-image"))
                    .map(|l| l.trim().to_string())
                    .collect();
                let monitores = saida("xrandr", &["--listmonitors"]).unwrap_or_default();
                let areas = saida("xfconf-query", &["-c", "xfwm4", "-p", "/general/workspace_count"])
                    .ok()
                    .and_then(|s| s.trim().parse::<u32>().ok())
                    .unwrap_or(4);
                for m in tema::monitores_xrandr(&monitores) {
                    for w in 0..areas {
                        props.push(format!("/backdrop/screen0/monitor{m}/workspace{w}/last-image"));
                    }
                }
                props.sort();
                props.dedup();
                if props.is_empty() {
                    return Err("não encontrei o monitor para trocar o papel de parede".into());
                }
                for p in &props {
                    executar("xfconf-query", &["-c", "xfce4-desktop", "-n", "-t", "string", "-p", p, "-s", arquivo])?;
                }
                Ok(())
            }
            Ambiente::Outro => Err(nao_suportado()),
        }
    }

    /// Tamanho do texto (porcentagem: 100 = normal).
    pub fn aplicar_texto(self, escala: u32) -> Result<(), String> {
        match self {
            Ambiente::Kde => {
                // o KDE aplica o DPI das fontes a partir do próximo login
                if escala == 100 {
                    executar("kwriteconfig6", &["--file", "kcmfonts", "--group", "General", "--key", "forceFontDPI", "--delete"])
                } else {
                    executar("kwriteconfig6", &["--file", "kcmfonts", "--group", "General", "--key", "forceFontDPI", &tema::dpi(escala).to_string()])
                }
            }
            Ambiente::Cinnamon => gsettings(
                "org.cinnamon.desktop.interface",
                "text-scaling-factor",
                &format!("{:.2}", escala as f64 / 100.0),
            ),
            Ambiente::Xfce => executar(
                "xfconf-query",
                &["-c", "xsettings", "-p", "/Xft/DPI", "-n", "-t", "int", "-s", &tema::dpi(escala).to_string()],
            ),
            Ambiente::Gnome => gsettings(
                "org.gnome.desktop.interface",
                "text-scaling-factor",
                &format!("{:.2}", escala as f64 / 100.0),
            ),
            Ambiente::Outro => Err(nao_suportado()),
        }
    }

    /// "Imagem do dia" (só no KDE: plugin org.kde.potd, fonte Bing).
    pub fn aplicar_imagem_do_dia(self) -> Result<(), String> {
        if self != Ambiente::Kde {
            return Err("a imagem do dia só existe no KDE Plasma".into());
        }
        plasma_script(
            "desktops().forEach(function(d) { d.wallpaperPlugin = \"org.kde.potd\"; \
             d.currentConfigGroup = [\"Wallpaper\", \"org.kde.potd\", \"General\"]; \
             d.writeConfig(\"Provider\", \"bing\"); });",
        )
    }
}

fn nao_suportado() -> String {
    "este ambiente gráfico não é suportado (use KDE Plasma, Cinnamon, XFCE ou GNOME)".into()
}

// ------------------------------------------------------------------ KDE

fn kde_tema(e: &Estado) -> Result<(), String> {
    let base = match e.modo {
        Modo::Claro => "/usr/share/color-schemes/DoorsClaro.colors",
        Modo::Escuro => "/usr/share/color-schemes/BreezeDark.colors",
    };
    let texto = fs::read_to_string(base).map_err(|err| format!("não consegui ler {base}: {err}"))?;
    let nome_id = e.nome_tema();
    let nome = format!("Doors Pessoal ({})", if e.modo == Modo::Claro { "claro" } else { "escuro" });
    let dir = dados_usuario().join("color-schemes");
    fs::create_dir_all(&dir).map_err(|err| err.to_string())?;
    fs::write(dir.join(format!("{nome_id}.colors")), tema::esquema_kde(&texto, &e.cor, &nome_id, &nome))
        .map_err(|err| err.to_string())?;
    // apaga o esquema da geração anterior (fica só o atual na lista do KDE)
    let antigo = if nome_id.ends_with('A') { "DoorsPessoalB" } else { "DoorsPessoalA" };
    let _ = fs::remove_file(dir.join(format!("{antigo}.colors")));
    // a cor de destaque já vai no esquema; grava também a do KDE (o padrão do Doors fica em /etc/xdg).
    // Não usar "plasma-apply-colorscheme --accent-color": o Plasma 6.3 recusa "#rrggbb" ("Cor de destaque inválida").
    let _ = executar("kwriteconfig6", &["--file", "kdeglobals", "--group", "General", "--key", "AccentColor", &e.cor.kde()]);
    executar("plasma-apply-colorscheme", &[&nome_id])?;
    // contorno e brilho da janela ativa (efeito "Rounded Corners", pacote doors-kwin-cantos)
    for (chave, valor) in tema::cantos_kde(&e.cor) {
        let _ = executar("kwriteconfig6", &["--file", "kwinrc", "--group", "Round-Corners", "--key", chave, &valor]);
    }
    let _ = executar(
        "gdbus",
        &["call", "--session", "--dest", "org.kde.KWin", "--object-path", "/Effects",
          "--method", "org.kde.kwin.Effects.reconfigureEffect", "kwin4_effect_shapecorners"],
    );
    Ok(())
}

fn plasma_script(js: &str) -> Result<(), String> {
    executar(
        "gdbus",
        &["call", "--session", "--dest", "org.kde.plasmashell", "--object-path", "/PlasmaShell",
          "--method", "org.kde.PlasmaShell.evaluateScript", js],
    )
}

// ------------------------------------------------------------------ GTK (Cinnamon e XFCE)

/// Cria ~/.local/share/themes/DoorsPessoal{A,B}: GTK3 com contorno na cor escolhida,
/// GTK2/GTK4 do Greybird e bordas xfwm4 recoloridas. Devolve o nome do tema.
fn gtk_tema_pessoal(e: &Estado) -> Result<String, String> {
    let base = match e.modo {
        Modo::Claro => "Greybird",
        Modo::Escuro => "Greybird-dark",
    };
    let origem = Path::new("/usr/share/themes").join(base);
    if !origem.is_dir() {
        return Err(format!("tema {base} não instalado"));
    }
    let nome = e.nome_tema();
    let temas = dados_usuario().join("themes");
    let destino = temas.join(&nome);
    let _ = fs::remove_dir_all(&destino);
    fs::create_dir_all(destino.join("gtk-3.0")).map_err(|err| err.to_string())?;
    fs::write(
        destino.join("index.theme"),
        format!("[Desktop Entry]\nType=X-GNOME-Metatheme\nName={nome}\nComment=Gerado pelo Doors Aparência\n\n\
                 [X-GNOME-Metatheme]\nGtkTheme={nome}\nMetacityTheme={base}\n"),
    )
    .map_err(|err| err.to_string())?;
    fs::write(destino.join("gtk-3.0/gtk.css"), tema::css_gtk3(base, &e.cor)).map_err(|err| err.to_string())?;
    for sub in ["gtk-2.0", "gtk-4.0", "metacity-1"] {
        if origem.join(sub).is_dir() {
            let _ = std::os::unix::fs::symlink(origem.join(sub), destino.join(sub));
        }
    }
    // xfwm4: copia as bordas do tema Doors (contorno de 2 px, 4 cantos arredondados) e pinta o
    // contorno das peças ativas na cor de destaque. O fundo das peças vem do tema GTK
    // ("active_color_2"); no escuro, tira a linha de brilho branca e clareia o título.
    let doors_xf = Path::new("/usr/share/themes/Doors/xfwm4");
    let xf_origem = if doors_xf.is_dir() { doors_xf.to_path_buf() } else { origem.join("xfwm4") };
    if xf_origem.is_dir() {
        let xf = destino.join("xfwm4");
        fs::create_dir_all(&xf).map_err(|err| err.to_string())?;
        let borda = fs::read_to_string(xf_origem.join("left-active.xpm"))
            .ok()
            .and_then(|t| tema::cor_da_borda_xpm(&t));
        let escuro = e.modo == Modo::Escuro;
        for entrada in fs::read_dir(&xf_origem).map_err(|err| err.to_string())?.flatten() {
            let caminho = entrada.path();
            let nome_arq = entrada.file_name().to_string_lossy().to_string();
            let alvo = xf.join(&nome_arq);
            if nome_arq.ends_with(".xpm") || nome_arq == "themerc" {
                let mut t = fs::read_to_string(&caminho).map_err(|err| err.to_string())?;
                if let (Some(b), true) = (&borda, nome_arq.ends_with("-active.xpm")) {
                    t = tema::recolorir_xpm(&t, b, &e.cor.hex());
                }
                if escuro {
                    t = tema::xfwm4_escuro(&t);
                }
                fs::write(&alvo, t).map_err(|err| err.to_string())?;
            } else {
                fs::copy(&caminho, &alvo).map_err(|err| err.to_string())?;
            }
        }
    }
    // apps GTK 4 com barra própria: contorno na cor de destaque (só se o arquivo for do Doors)
    let g4 = config_usuario().join("gtk-4.0").join("gtk.css");
    let do_doors = fs::read_to_string(&g4).map(|t| t.contains("Doors OS")).unwrap_or(true);
    if do_doors {
        let _ = fs::create_dir_all(g4.parent().unwrap_or(Path::new(".")));
        let _ = fs::write(&g4, tema::css_gtk4(&e.cor));
    }
    // remove o tema da geração anterior
    let antigo = if nome.ends_with('A') { "DoorsPessoalB" } else { "DoorsPessoalA" };
    let _ = fs::remove_dir_all(temas.join(antigo));
    Ok(nome)
}

/// Apps GTK4/libadwaita seguem org.gnome.desktop.interface color-scheme.
fn preferencia_escura(modo: Modo) {
    let v = if modo == Modo::Escuro { "'prefer-dark'" } else { "'default'" };
    let _ = gsettings("org.gnome.desktop.interface", "color-scheme", v);
}

// ------------------------------------------------------------------ utilitários

fn config_usuario() -> PathBuf {
    std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(std::env::var_os("HOME").unwrap_or_default()).join(".config"))
}

fn dados_usuario() -> PathBuf {
    std::env::var_os("XDG_DATA_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(std::env::var_os("HOME").unwrap_or_default()).join(".local/share"))
}

fn gsettings(esquema: &str, chave: &str, valor: &str) -> Result<(), String> {
    executar("gsettings", &["set", esquema, chave, valor])
}

fn xfconf(canal: &str, prop: &str, valor: &str) -> Result<(), String> {
    executar("xfconf-query", &["-c", canal, "-p", prop, "-s", valor])
}

fn saida(prog: &str, args: &[&str]) -> Result<String, String> {
    let s = Command::new(prog).args(args).output().map_err(|err| format!("{prog}: {err}"))?;
    if s.status.success() {
        Ok(String::from_utf8_lossy(&s.stdout).to_string())
    } else {
        Err(format!("{prog}: {}", String::from_utf8_lossy(&s.stderr).trim()))
    }
}

fn executar(prog: &str, args: &[&str]) -> Result<(), String> {
    saida(prog, args).map(|_| ())
}
