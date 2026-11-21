//! Partes "puras" do Doors Aparência: cores, geração de esquema de cores do KDE,
//! tema GTK/xfwm4 pessoal e ajustes de texto. Nada aqui toca no sistema (testável).

/// Claro ou escuro.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Modo {
    Claro,
    Escuro,
}

impl Modo {
    pub fn id(self) -> &'static str {
        match self {
            Modo::Claro => "claro",
            Modo::Escuro => "escuro",
        }
    }
    pub fn de_id(s: &str) -> Option<Modo> {
        match s {
            "claro" => Some(Modo::Claro),
            "escuro" => Some(Modo::Escuro),
            _ => None,
        }
    }
}

/// Uma cor de destaque oferecida pelo app.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cor {
    pub id: &'static str,
    pub nome: &'static str,
    pub rgb: (u8, u8, u8),
}

impl Cor {
    pub fn hex(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.rgb.0, self.rgb.1, self.rgb.2)
    }
    /// formato dos arquivos .colors do KDE: "r,g,b"
    pub fn kde(&self) -> String {
        format!("{},{},{}", self.rgb.0, self.rgb.1, self.rgb.2)
    }
    /// versão clara da cor (para seleção alternativa)
    pub fn clara(&self) -> String {
        let m = |c: u8| (c as u16 + (255 - c as u16) * 55 / 100) as u8;
        format!("{},{},{}", m(self.rgb.0), m(self.rgb.1), m(self.rgb.2))
    }
}

pub const CORES: [Cor; 6] = [
    Cor { id: "azul", nome: "Azul Doors", rgb: (47, 120, 181) },
    Cor { id: "verde", nome: "Verde", rgb: (39, 138, 92) },
    Cor { id: "roxo", nome: "Roxo", rgb: (125, 84, 178) },
    Cor { id: "laranja", nome: "Laranja", rgb: (206, 108, 32) },
    Cor { id: "vermelho", nome: "Vermelho", rgb: (192, 57, 61) },
    Cor { id: "grafite", nome: "Grafite", rgb: (84, 93, 106) },
];

pub fn cor_por_id(id: &str) -> Option<Cor> {
    CORES.iter().copied().find(|c| c.id == id)
}

/// Painel do ambiente: embaixo (estilo Mint) ou em cima.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Painel {
    Embaixo,
    EmCima,
}

impl Painel {
    pub fn id(self) -> &'static str {
        match self {
            Painel::Embaixo => "embaixo",
            Painel::EmCima => "emcima",
        }
    }
    pub fn de_id(s: &str) -> Option<Painel> {
        match s {
            "embaixo" => Some(Painel::Embaixo),
            "emcima" => Some(Painel::EmCima),
            _ => None,
        }
    }
}

// ------------------------------------------------------------------ KDE

/// Azuis usados como destaque nos esquemas de base (Doors Claro e Breeze Dark).
const DESTAQUES_BASE: [&str; 3] = ["47,120,181", "61,174,233", "29,153,243"];
const DESTAQUES_CLAROS_BASE: [&str; 2] = ["170,200,230", "163,212,250"];

/// Gera um esquema de cores do KDE a partir de um esquema de base:
/// troca o azul de destaque pela cor escolhida, remove os grupos `Colors:Header`
/// (com eles o KWin ignora `[WM]` e a barra de título não fica colorida),
/// pinta a barra de título da janela ativa e renomeia o esquema.
pub fn esquema_kde(base: &str, cor: &Cor, nome_id: &str, nome: &str) -> String {
    let mut saida = String::new();
    let mut grupo = String::new();
    let mut pular = false;
    for linha in base.lines() {
        let t = linha.trim();
        if t.starts_with('[') {
            grupo = t.to_string();
            pular = grupo.starts_with("[Colors:Header]");
            if pular {
                continue;
            }
        }
        if pular {
            continue;
        }
        // traduções do nome/comentário do esquema de base não valem para o novo
        if (t.starts_with("Name[") || t.starts_with("Comment[")) && grupo == "[General]" {
            continue;
        }
        let mut l = linha.to_string();
        if let Some((chave, valor)) = linha.split_once('=') {
            let chave = chave.trim();
            let valor = valor.trim();
            l = match (grupo.as_str(), chave) {
                ("[General]", "ColorScheme") => format!("ColorScheme={nome_id}"),
                ("[General]", "Name") => format!("Name={nome}"),
                ("[WM]", "activeBackground") | ("[WM]", "activeBlend") => {
                    format!("{chave}={}", cor.kde())
                }
                ("[WM]", "activeForeground") => "activeForeground=255,255,255".to_string(),
                _ if DESTAQUES_BASE.contains(&valor) => format!("{chave}={}", cor.kde()),
                _ if DESTAQUES_CLAROS_BASE.contains(&valor) => {
                    format!("{chave}={}", cor.clara())
                }
                _ => l,
            };
        }
        saida.push_str(&l);
        saida.push('\n');
    }
    saida
}

// ------------------------------------------------------------------ GTK / xfwm4

/// CSS do tema GTK3 pessoal: importa o Greybird (claro ou escuro) e pinta o
/// contorno da janela ativa (o Cinnamon 6 desenha as bordas por este CSS).
pub fn css_gtk3(base_tema: &str, cor: &Cor) -> String {
    let h = cor.hex();
    format!(
        "/* Gerado pelo Doors Aparência. Base: {base_tema}. */\n\
@import url(\"/usr/share/themes/{base_tema}/gtk-3.0/gtk.css\");\n\n\
.ssd decoration {{ box-shadow: 0 0 0 2px {h}; }}\n\
.ssd decoration:backdrop {{ box-shadow: 0 0 0 1px rgba(0, 0, 0, 0.23); }}\n\
decoration {{ box-shadow: 0 3px 9px 1px rgba(0, 0, 0, 0.5), 0 0 0 2px {h}; }}\n\
decoration:backdrop {{ box-shadow: 0 3px 9px 1px transparent, 0 2px 6px 2px rgba(0, 0, 0, 0.2), 0 0 0 1px rgba(0, 0, 0, 0.18); }}\n\
.maximized decoration, .fullscreen decoration, .tiled decoration, .popup decoration,\n\
tooltip decoration, .csd.popup decoration {{ box-shadow: none; }}\n"
    )
}

/// Cor da borda num XPM de 1 pixel do xfwm4 (ex.: left-active.xpm do Greybird).
pub fn cor_da_borda_xpm(xpm: &str) -> Option<String> {
    // linha de cor: "<char>\tc #rrggbb"
    xpm.lines().find_map(|l| {
        let i = l.find("c #")?;
        let hex = l.get(i + 2..i + 9)?;
        if hex.len() == 7 && hex[1..].chars().all(|c| c.is_ascii_hexdigit()) {
            Some(hex.to_ascii_lowercase())
        } else {
            None
        }
    })
}

/// Troca uma cor (#rrggbb, sem diferenciar maiúsculas) por outra num XPM.
pub fn recolorir_xpm(xpm: &str, de: &str, para: &str) -> String {
    let de = de.to_ascii_lowercase();
    let mut saida = String::with_capacity(xpm.len());
    for linha in xpm.split_inclusive('\n') {
        let baixa = linha.to_ascii_lowercase();
        if let Some(i) = baixa.find(&format!("c {de}")) {
            saida.push_str(&linha[..i + 2]);
            saida.push_str(para);
            saida.push_str(&linha[i + 2 + de.len()..]);
        } else {
            saida.push_str(linha);
        }
    }
    saida
}

/// Cinnamon: troca a posição em `panels-enabled` (ex.: "['1:0:bottom']").
pub fn painel_cinnamon(valor: &str, painel: Painel) -> String {
    let (de, para) = match painel {
        Painel::Embaixo => (":top", ":bottom"),
        Painel::EmCima => (":bottom", ":top"),
    };
    valor.replace(de, para)
}

/// XFCE: valor de /panels/panel-N/position (p=8 embaixo, p=6 em cima).
pub fn painel_xfce(painel: Painel) -> &'static str {
    match painel {
        Painel::Embaixo => "p=8;x=0;y=0",
        Painel::EmCima => "p=6;x=0;y=0",
    }
}

/// Nomes das saídas em `xrandr --listmonitors` (última coluna de cada linha " N: ...").
pub fn monitores_xrandr(saida: &str) -> Vec<String> {
    saida
        .lines()
        .filter(|l| l.trim_start().chars().next().is_some_and(|c| c.is_ascii_digit()))
        .filter_map(|l| l.split_whitespace().last().map(str::to_string))
        .collect()
}

#[cfg(test)]
mod testes {
    #[test]
    fn monitores() {
        let s = "Monitors: 2\n 0: +*Virtual-1 1280/325x800/203+0+0  Virtual-1\n 1: +HDMI-A-1 1920/510x1080/290+1280+0  HDMI-A-1\n";
        assert_eq!(monitores_xrandr(s), vec!["Virtual-1", "HDMI-A-1"]);
        assert!(monitores_xrandr("Monitors: 0\n").is_empty());
    }

    use super::*;

    const BASE: &str = "[ColorEffects:Disabled]\nColor=56,56,56\n\n\
[Colors:Header]\nBackgroundNormal=222,224,226\nDecorationFocus=47,120,181\n\n\
[Colors:Header][Inactive]\nBackgroundNormal=239,240,241\n\n\
[Colors:Selection]\nBackgroundAlternate=170,200,230\nBackgroundNormal=47,120,181\n\n\
[General]\nColorScheme=DoorsClaro\nName=Doors Claro\nName[pt_BR]=Doors claro\n\n\
[WM]\nactiveBackground=47,120,181\nactiveBlend=47,120,181\nactiveForeground=255,255,255\n\
inactiveBackground=239,240,241\n";

    #[test]
    fn esquema_kde_troca_destaque_e_tira_header() {
        let roxo = cor_por_id("roxo").unwrap();
        let s = esquema_kde(BASE, &roxo, "DoorsPessoalA", "Doors Pessoal");
        assert!(!s.contains("Colors:Header"), "{s}");
        assert!(!s.contains("222,224,226"));
        assert!(s.contains("[Colors:Selection]\nBackgroundAlternate="));
        assert!(s.contains("BackgroundNormal=125,84,178"));
        assert!(s.contains("activeBackground=125,84,178"));
        assert!(s.contains("activeBlend=125,84,178"));
        assert!(s.contains("ColorScheme=DoorsPessoalA"));
        assert!(s.contains("Name=Doors Pessoal"));
        assert!(!s.contains("Name[pt_BR]"));
        assert!(s.contains("inactiveBackground=239,240,241"));
        assert!(!s.contains("47,120,181"));
    }

    #[test]
    fn esquema_kde_de_breeze_dark() {
        let base = "[Colors:Window]\nDecorationFocus=61,174,233\nBackgroundNormal=32,35,38\n\n\
[WM]\nactiveBackground=49,54,59\nactiveForeground=252,252,252\n";
        let s = esquema_kde(base, &cor_por_id("verde").unwrap(), "X", "Y");
        assert!(s.contains("DecorationFocus=39,138,92"));
        assert!(s.contains("BackgroundNormal=32,35,38"));
        assert!(s.contains("activeBackground=39,138,92"));
        assert!(s.contains("activeForeground=255,255,255"));
    }

    #[test]
    fn xpm_recolorido() {
        let xpm = "/* XPM */\nstatic char * left_active_xpm[] = {\n\"1 1 1 1\",\n\" \tc #939393\",\n\" \"};\n";
        assert_eq!(cor_da_borda_xpm(xpm).as_deref(), Some("#939393"));
        let novo = recolorir_xpm(xpm, "#939393", "#c0393d");
        assert!(novo.contains("c #c0393d"));
        assert!(!novo.contains("939393"));
        // maiúsculas no arquivo de origem
        let novo = recolorir_xpm("\" \tc #9A9A9A\",\n", "#9a9a9a", "#000000");
        assert_eq!(novo, "\" \tc #000000\",\n");
    }

    #[test]
    fn css_tem_import_e_cor() {
        let css = css_gtk3("Greybird-dark", &cor_por_id("laranja").unwrap());
        assert!(css.contains("/usr/share/themes/Greybird-dark/gtk-3.0/gtk.css"));
        assert!(css.contains("0 0 0 2px #ce6c20"));
    }

    #[test]
    fn paineis() {
        assert_eq!(painel_cinnamon("['1:0:bottom']", Painel::EmCima), "['1:0:top']");
        assert_eq!(painel_cinnamon("['1:0:top', '2:1:top']", Painel::Embaixo), "['1:0:bottom', '2:1:bottom']");
        assert_eq!(painel_xfce(Painel::EmCima), "p=6;x=0;y=0");
    }

    #[test]
    fn cores_validas() {
        assert_eq!(cor_por_id("azul").unwrap().hex(), "#2f78b5");
        assert!(cor_por_id("rosa").is_none());
        assert_eq!(Modo::de_id("escuro"), Some(Modo::Escuro));
        assert_eq!(Painel::de_id("emcima"), Some(Painel::EmCima));
    }
}
