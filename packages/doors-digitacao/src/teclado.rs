//! Layouts de teclado: posição das teclas, dedo de cada tecla, como digitar cada
//! caractere (inclusive acentos por teclas mortas) e detecção do layout em uso.

/// Layouts suportados.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Layout {
    /// Português do Brasil (ABNT2): tem ç e teclas de acento próprias.
    Abnt2,
    /// Inglês (EUA) internacional com teclas mortas: ' " ` ~ ^ são acentos; ' + c = ç.
    UsIntl,
    /// Inglês (EUA) sem teclas mortas: sem acentos nem ç.
    Us,
}

impl Layout {
    pub const TODOS: [Layout; 3] = [Layout::Abnt2, Layout::UsIntl, Layout::Us];

    pub fn id(self) -> &'static str {
        match self {
            Layout::Abnt2 => "abnt2",
            Layout::UsIntl => "us-intl",
            Layout::Us => "us",
        }
    }
    pub fn de_id(s: &str) -> Option<Layout> {
        Layout::TODOS.iter().copied().find(|l| l.id() == s)
    }
    pub fn nome(self) -> &'static str {
        match self {
            Layout::Abnt2 => "Português (Brasil, ABNT2)",
            Layout::UsIntl => "Inglês (EUA) internacional, com teclas mortas",
            Layout::Us => "Inglês (EUA), sem acentos",
        }
    }
}

/// Acentos digitados com tecla morta.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Acento {
    Agudo,
    Grave,
    Til,
    Circunflexo,
    Trema,
}

/// Uma tecla física: o que ela produz sem e com Shift (None = tecla morta naquela posição).
#[derive(Clone, Debug)]
pub struct Tecla {
    pub normal: char,
    pub shift: char,
    /// rótulo desenhado (ex. "´ `")
    pub dedo: u8,
    pub morta_normal: Option<Acento>,
    pub morta_shift: Option<Acento>,
}

/// Dedos: 0..3 mão esquerda (mínimo, anelar, médio, indicador), 4..7 mão direita
/// (indicador, médio, anelar, mínimo), 8 polegar.
pub const NOMES_DEDOS: [&str; 9] = [
    "mínimo esquerdo",
    "anelar esquerdo",
    "médio esquerdo",
    "indicador esquerdo",
    "indicador direito",
    "médio direito",
    "anelar direito",
    "mínimo direito",
    "polegar",
];

fn t(normal: char, shift: char, dedo: u8) -> Tecla {
    Tecla { normal, shift, dedo, morta_normal: None, morta_shift: None }
}

fn morta(normal: char, shift: char, dedo: u8, mn: Option<Acento>, ms: Option<Acento>) -> Tecla {
    Tecla { normal, shift, dedo, morta_normal: mn, morta_shift: ms }
}

/// Fileiras de teclas (0 = números, 1 = de cima, 2 = do meio, 3 = de baixo).
pub fn fileiras(l: Layout) -> Vec<Vec<Tecla>> {
    use Acento::*;
    let numeros = |grave_til: bool, trema6: bool| -> Vec<Tecla> {
        let mut v = vec![if grave_til {
            morta('`', '~', 0, Some(Grave), Some(Til))
        } else if l == Layout::Abnt2 {
            t('\'', '"', 0)
        } else {
            t('`', '~', 0)
        }];
        let pares = [('1', '!'), ('2', '@'), ('3', '#'), ('4', '$'), ('5', '%'), ('6', '^'), ('7', '&'), ('8', '*'), ('9', '('), ('0', ')'), ('-', '_'), ('=', '+')];
        let dedos = [0, 1, 2, 3, 3, 4, 4, 5, 6, 7, 7, 7];
        for (i, (a, b)) in pares.iter().enumerate() {
            let mut k = t(*a, *b, dedos[i]);
            if *a == '6' {
                if trema6 {
                    k = morta('6', '¨', dedos[i], None, Some(Trema));
                } else if l == Layout::UsIntl {
                    k = morta('6', '^', dedos[i], None, Some(Circunflexo));
                }
            }
            v.push(k);
        }
        v
    };
    let letras = |s: &str, dedos: &[u8]| -> Vec<Tecla> {
        s.chars().zip(dedos.iter()).map(|(c, d)| t(c, c.to_ascii_uppercase(), *d)).collect()
    };
    match l {
        Layout::Abnt2 => {
            let mut cima = letras("qwertyuiop", &[0, 1, 2, 3, 3, 4, 4, 5, 6, 7]);
            cima.push(morta('´', '`', 7, Some(Agudo), Some(Grave)));
            cima.push(t('[', '{', 7));
            let mut meio = letras("asdfghjkl", &[0, 1, 2, 3, 3, 4, 4, 5, 6]);
            meio.push(t('ç', 'Ç', 7));
            meio.push(morta('~', '^', 7, Some(Til), Some(Circunflexo)));
            meio.push(t(']', '}', 7));
            let mut baixo = vec![t('\\', '|', 0)];
            baixo.extend(letras("zxcvbnm", &[0, 1, 2, 3, 3, 4, 4]));
            baixo.extend([t(',', '<', 5), t('.', '>', 6), t(';', ':', 7), t('/', '?', 7)]);
            vec![numeros(false, true), cima, meio, baixo]
        }
        Layout::UsIntl | Layout::Us => {
            let intl = l == Layout::UsIntl;
            let mut cima = letras("qwertyuiop", &[0, 1, 2, 3, 3, 4, 4, 5, 6, 7]);
            cima.extend([t('[', '{', 7), t(']', '}', 7), t('\\', '|', 7)]);
            let mut meio = letras("asdfghjkl", &[0, 1, 2, 3, 3, 4, 4, 5, 6]);
            meio.push(t(';', ':', 7));
            meio.push(if intl { morta('\'', '"', 7, Some(Agudo), Some(Trema)) } else { t('\'', '"', 7) });
            let mut baixo = letras("zxcvbnm", &[0, 1, 2, 3, 3, 4, 4]);
            baixo.extend([t(',', '<', 5), t('.', '>', 6), t('/', '?', 7)]);
            vec![numeros(intl, false), cima, meio, baixo]
        }
    }
}

/// Um passo para digitar um caractere: tecla (fileira, coluna) com ou sem Shift.
/// fileira 4 = barra de espaço.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Passo {
    pub fileira: usize,
    pub coluna: usize,
    pub shift: bool,
}

const ESPACO: Passo = Passo { fileira: 4, coluna: 0, shift: false };

/// Letra base e acento de um caractere acentuado (á -> (a, Agudo)).
pub fn decompor(c: char) -> Option<(char, Acento)> {
    use Acento::*;
    let (base, ac) = match c {
        'á' => ('a', Agudo), 'é' => ('e', Agudo), 'í' => ('i', Agudo), 'ó' => ('o', Agudo), 'ú' => ('u', Agudo),
        'Á' => ('A', Agudo), 'É' => ('E', Agudo), 'Í' => ('I', Agudo), 'Ó' => ('O', Agudo), 'Ú' => ('U', Agudo),
        'à' => ('a', Grave), 'À' => ('A', Grave),
        'ã' => ('a', Til), 'õ' => ('o', Til), 'Ã' => ('A', Til), 'Õ' => ('O', Til),
        'â' => ('a', Circunflexo), 'ê' => ('e', Circunflexo), 'ô' => ('o', Circunflexo),
        'Â' => ('A', Circunflexo), 'Ê' => ('E', Circunflexo), 'Ô' => ('O', Circunflexo),
        'ü' => ('u', Trema), 'Ü' => ('U', Trema),
        _ => return None,
    };
    Some((base, ac))
}

/// Passos para digitar `c` no layout `l` (vazio = não dá para digitar).
pub fn passos(l: Layout, c: char) -> Vec<Passo> {
    if c == ' ' {
        return vec![ESPACO];
    }
    let f = fileiras(l);
    let achar = |alvo: char| -> Option<Passo> {
        for (fi, linha) in f.iter().enumerate() {
            for (co, k) in linha.iter().enumerate() {
                if k.normal == alvo && k.morta_normal.is_none() {
                    return Some(Passo { fileira: fi, coluna: co, shift: false });
                }
                if k.shift == alvo && k.morta_shift.is_none() {
                    return Some(Passo { fileira: fi, coluna: co, shift: true });
                }
            }
        }
        None
    };
    let morta_de = |ac: Acento| -> Option<Passo> {
        for (fi, linha) in f.iter().enumerate() {
            for (co, k) in linha.iter().enumerate() {
                if k.morta_normal == Some(ac) {
                    return Some(Passo { fileira: fi, coluna: co, shift: false });
                }
                if k.morta_shift == Some(ac) {
                    return Some(Passo { fileira: fi, coluna: co, shift: true });
                }
            }
        }
        None
    };
    if let Some(p) = achar(c) {
        return vec![p];
    }
    // ç no internacional: ' (agudo) + c
    if l == Layout::UsIntl && (c == 'ç' || c == 'Ç') {
        let base = if c == 'ç' { 'c' } else { 'C' };
        if let (Some(m), Some(b)) = (morta_de(Acento::Agudo), achar(base)) {
            return vec![m, b];
        }
    }
    if let Some((base, ac)) = decompor(c) {
        if let (Some(m), Some(b)) = (morta_de(ac), achar(base)) {
            return vec![m, b];
        }
    }
    // o próprio sinal da tecla morta: acento + espaço (ex. ' no internacional, ~ no ABNT2)
    for (fi, linha) in f.iter().enumerate() {
        for (co, k) in linha.iter().enumerate() {
            if k.normal == c && k.morta_normal.is_some() {
                return vec![Passo { fileira: fi, coluna: co, shift: false }, ESPACO];
            }
            if k.shift == c && k.morta_shift.is_some() {
                return vec![Passo { fileira: fi, coluna: co, shift: true }, ESPACO];
            }
        }
    }
    Vec::new()
}

/// Dá para digitar este caractere neste layout?
pub fn digitavel(l: Layout, c: char) -> bool {
    !passos(l, c).is_empty()
}

/// Texto adaptado ao layout: no inglês sem acentos, tira acentos e troca ç por c.
pub fn adaptar(l: Layout, texto: &str) -> String {
    texto
        .chars()
        .map(|c| {
            if digitavel(l, c) {
                c
            } else if c == 'ç' {
                'c'
            } else if c == 'Ç' {
                'C'
            } else if let Some((b, _)) = decompor(c) {
                b
            } else {
                c
            }
        })
        .filter(|c| digitavel(l, *c))
        .collect()
}

/// Explicação curta de como digitar `c` (ex. "´ e depois e", "Shift + a").
pub fn dica(l: Layout, c: char) -> String {
    let ps = passos(l, c);
    let f = fileiras(l);
    let nome = |p: &Passo| -> String {
        if p.fileira == 4 {
            return "espaço".into();
        }
        let k = &f[p.fileira][p.coluna];
        let s = if p.shift { k.shift } else { k.normal };
        let mut n = s.to_string();
        if p.shift && s.is_alphabetic() {
            n = format!("Shift + {}", k.normal);
        } else if p.shift {
            n = format!("Shift + tecla {}", k.normal);
        }
        n
    };
    match ps.len() {
        0 => String::new(),
        1 => {
            let p = &ps[0];
            let dedo = if p.fileira == 4 { 8 } else { f[p.fileira][p.coluna].dedo };
            format!("{}  ·  dedo {}", nome(p), NOMES_DEDOS[dedo as usize])
        }
        _ => format!("{} e depois {}", nome(&ps[0]), nome(&ps[1])),
    }
}

/// Lê o layout a partir de "layout" e "variant" do XKB (ex. "us" + "intl", "br").
pub fn de_xkb(layout: &str, variante: &str) -> Option<Layout> {
    let layout = layout.split(',').next().unwrap_or("").trim();
    let variante = variante.split(',').next().unwrap_or("").trim();
    match layout {
        "br" => Some(Layout::Abnt2),
        "us" if variante.contains("intl") => Some(Layout::UsIntl),
        "us" => Some(Layout::Us),
        _ => None,
    }
}

/// Saída de `setxkbmap -query`.
pub fn de_setxkbmap(saida: &str) -> Option<Layout> {
    let campo = |nome: &str| {
        saida.lines().find_map(|l| l.strip_prefix(nome).map(|r| r.trim_start_matches(':').trim().to_string()))
    };
    de_xkb(&campo("layout")?, &campo("variant").unwrap_or_default())
}

/// Valor de `gsettings get org.gnome.desktop.input-sources sources`, ex. "[('xkb', 'us+intl')]".
pub fn de_gnome(valor: &str) -> Option<Layout> {
    let i = valor.find("'xkb', '")? + 8;
    let resto = &valor[i..];
    let fonte = &resto[..resto.find('\'')?];
    let (l, v) = fonte.split_once('+').unwrap_or((fonte, ""));
    de_xkb(l, v)
}

/// Conteúdo de ~/.config/kxkbrc (KDE) ou /etc/default/keyboard.
pub fn de_config(texto: &str, chave_layout: &str, chave_variante: &str) -> Option<Layout> {
    let valor = |k: &str| {
        texto.lines().find_map(|l| {
            let (a, b) = l.split_once('=')?;
            (a.trim() == k).then(|| b.trim().trim_matches('"').to_string())
        })
    };
    de_xkb(&valor(chave_layout)?, &valor(chave_variante).unwrap_or_default())
}

/// Detecta o layout em uso (X11, KDE, GNOME ou configuração do sistema). Padrão: ABNT2.
pub fn detectar() -> (Layout, bool) {
    let rodar = |prog: &str, args: &[&str]| {
        std::process::Command::new(prog)
            .args(args)
            .output()
            .ok()
            .filter(|s| s.status.success())
            .map(|s| String::from_utf8_lossy(&s.stdout).to_string())
    };
    let home = std::env::var("HOME").unwrap_or_default();
    let tentativas: [Box<dyn Fn() -> Option<Layout>>; 4] = [
        Box::new(|| rodar("setxkbmap", &["-query"]).and_then(|s| de_setxkbmap(&s))),
        Box::new(|| std::fs::read_to_string(format!("{home}/.config/kxkbrc")).ok().and_then(|s| de_config(&s, "LayoutList", "VariantList"))),
        Box::new(|| rodar("gsettings", &["get", "org.gnome.desktop.input-sources", "sources"]).and_then(|s| de_gnome(&s))),
        Box::new(|| std::fs::read_to_string("/etc/default/keyboard").ok().and_then(|s| de_config(&s, "XKBLAYOUT", "XKBVARIANT"))),
    ];
    for f in tentativas.iter() {
        if let Some(l) = f() {
            return (l, true);
        }
    }
    (Layout::Abnt2, false)
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn cedilha() {
        assert_eq!(passos(Layout::Abnt2, 'ç').len(), 1);
        let p = passos(Layout::UsIntl, 'ç');
        assert_eq!(p.len(), 2, "' + c no internacional");
        assert_eq!(dica(Layout::UsIntl, 'ç'), "' e depois c");
        assert!(passos(Layout::Us, 'ç').is_empty());
    }

    #[test]
    fn acentos() {
        assert_eq!(dica(Layout::Abnt2, 'á'), "´ e depois a");
        assert_eq!(dica(Layout::Abnt2, 'ã'), "~ e depois a");
        assert_eq!(dica(Layout::Abnt2, 'ê'), "Shift + tecla ~ e depois e");
        assert_eq!(dica(Layout::UsIntl, 'é'), "' e depois e");
        assert_eq!(dica(Layout::UsIntl, 'õ'), "Shift + tecla ` e depois o");
        assert_eq!(passos(Layout::UsIntl, '\''), vec![passos(Layout::UsIntl, 'á')[0], ESPACO]);
        assert!(dica(Layout::Abnt2, 'F').starts_with("Shift + f"));
        assert!(dica(Layout::Abnt2, 'j').contains("indicador direito"));
    }

    #[test]
    fn adaptar_texto() {
        assert_eq!(adaptar(Layout::Us, "Ação já"), "Acao ja");
        assert_eq!(adaptar(Layout::UsIntl, "Ação já"), "Ação já");
    }

    #[test]
    fn deteccao() {
        assert_eq!(de_setxkbmap("rules:      evdev\nmodel:      pc105\nlayout:     us\nvariant:    intl\n"), Some(Layout::UsIntl));
        assert_eq!(de_setxkbmap("layout:     br\n"), Some(Layout::Abnt2));
        assert_eq!(de_gnome("[('xkb', 'us+intl'), ('xkb', 'br')]"), Some(Layout::UsIntl));
        assert_eq!(de_gnome("[('xkb', 'br')]"), Some(Layout::Abnt2));
        assert_eq!(de_config("[Layout]\nLayoutList=us\nVariantList=intl\n", "LayoutList", "VariantList"), Some(Layout::UsIntl));
        assert_eq!(de_config("XKBLAYOUT=\"br\"\nXKBVARIANT=\"\"\n", "XKBLAYOUT", "XKBVARIANT"), Some(Layout::Abnt2));
        assert_eq!(de_xkb("de", ""), None);
    }

    #[test]
    fn todas_as_letras_digitaveis() {
        for l in Layout::TODOS {
            for c in "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789 ,.;:?!-".chars() {
                assert!(digitavel(l, c), "{c} em {l:?}");
            }
        }
        for c in "áéíóúàãõâêôçÁÉÇ".chars() {
            assert!(digitavel(Layout::Abnt2, c), "{c}");
            assert!(digitavel(Layout::UsIntl, c), "{c}");
        }
    }
}
