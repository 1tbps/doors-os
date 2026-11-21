//! Doors Aparência — personalização do Doors OS em um clique (KDE Plasma, Cinnamon, XFCE e GNOME).
//! Organização em páginas com menu lateral (Tema, Fundo, Layout, Fontes), com prévias
//! desenhadas de cada opção.

mod ambiente;
mod estado;
mod tema;

use ambiente::Ambiente;
use estado::Estado;
use gtk::prelude::*;
use gtk::{cairo, gdk, gdk_pixbuf, glib};
use std::cell::RefCell;
use std::rc::Rc;
use tema::{Cor, Modo, Painel, CORES, TAMANHOS_TEXTO};

const ID: &str = "org.doors.Aparencia";
const PAPEIS: &str = "/usr/share/backgrounds/doors";
const COLECAO_XML: &str = "/usr/share/gnome-background-properties/doors.xml";
const PADRAO: &str = "/usr/share/backgrounds/doors/colina.jpg";

struct App {
    estado: RefCell<Estado>,
    ambiente: Ambiente,
    aviso: gtk::Label,
    /// prévias que dependem do estado (redesenhadas a cada mudança)
    previas: RefCell<Vec<gtk::DrawingArea>>,
    /// miniatura do papel de parede atual, usada como fundo das prévias
    fundo: RefCell<Option<gdk_pixbuf::Pixbuf>>,
}

impl App {
    /// Mostra o resultado de uma ação e guarda o estado.
    fn aplicar(&self, o_que: &str, r: Result<(), String>) {
        match r {
            Ok(()) => {
                self.aviso.set_markup(&format!("<span foreground=\"#2e9e5b\">✓</span> {o_que}: aplicado"));
                self.estado.borrow().salvar();
            }
            Err(e) => self.aviso.set_markup(&format!(
                "<span foreground=\"#c0393d\">✗</span> {o_que}: {}",
                glib::markup_escape_text(&e)
            )),
        }
        self.redesenhar();
    }

    fn aplicar_tema(&self) {
        let e = {
            let mut e = self.estado.borrow_mut();
            e.geracao = e.geracao.wrapping_add(1);
            e.clone()
        };
        self.aplicar("Tema", self.ambiente.aplicar_tema(&e));
        modo_do_app(e.modo);
    }

    fn redesenhar(&self) {
        for p in self.previas.borrow().iter() {
            p.queue_draw();
        }
    }

    fn trocar_fundo(&self, arquivo: &str) {
        *self.fundo.borrow_mut() = gdk_pixbuf::Pixbuf::from_file_at_scale(arquivo, 320, 180, false).ok();
    }
}

/// O próprio app acompanha o modo escolhido (claro/escuro).
fn modo_do_app(modo: Modo) {
    if let Some(s) = gtk::Settings::default() {
        s.set_gtk_application_prefer_dark_theme(modo == Modo::Escuro);
    }
}

fn main() -> glib::ExitCode {
    let app = gtk::Application::builder().application_id(ID).build();
    app.connect_activate(montar);
    app.run()
}

// ------------------------------------------------------------------ estilo e peças comuns

fn estilo() {
    let mut css = String::from(
        ".pagina-titulo { font-size: 1.5em; font-weight: bold; }\n\
         .secao { font-weight: bold; margin-top: 8px; }\n\
         .cartao { padding: 8px; border-radius: 12px; }\n\
         .cartao:checked { box-shadow: inset 0 0 0 3px @theme_selected_bg_color; }\n\
         .cartao-nome { margin-top: 4px; }\n\
         .cor { min-width: 36px; min-height: 36px; border-radius: 999px; padding: 0; border: 2px solid rgba(0,0,0,0.2); }\n\
         .cor label { color: white; font-weight: bold; }\n\
         .papel { padding: 4px; border-radius: 10px; }\n\
         .papel picture { border-radius: 8px; }\n\
         .lateral { padding: 8px; }\n\
         .lateral row { padding: 10px 14px; border-radius: 8px; }\n\
         .rodape { padding: 6px 16px; }\n",
    );
    for c in CORES.iter() {
        css.push_str(&format!(".cor-{} {{ background: {}; }}\n", c.id, c.hex()));
    }
    let p = gtk::CssProvider::new();
    #[allow(deprecated)]
    p.load_from_data(&css);
    if let Some(d) = gdk::Display::default() {
        #[allow(deprecated)]
        gtk::style_context_add_provider_for_display(&d, &p, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
    }
}

fn rotulo(texto: &str, classe: &str) -> gtk::Label {
    let l = gtk::Label::new(Some(texto));
    l.set_halign(gtk::Align::Start);
    l.set_wrap(true);
    l.set_xalign(0.0);
    if !classe.is_empty() {
        l.add_css_class(classe);
    }
    l
}

fn pagina(titulo: &str, descricao: &str) -> gtk::Box {
    let b = gtk::Box::new(gtk::Orientation::Vertical, 12);
    b.set_margin_top(24);
    b.set_margin_bottom(24);
    b.set_margin_start(28);
    b.set_margin_end(28);
    b.append(&rotulo(titulo, "pagina-titulo"));
    b.append(&rotulo(descricao, "dim-label"));
    b
}

/// Cartão de escolha (prévia desenhada + nome), no estilo do Zorin Appearance.
fn cartao(nome: &str, desenho: gtk::DrawingArea, grupo: Option<&gtk::ToggleButton>) -> gtk::ToggleButton {
    let caixa = gtk::Box::new(gtk::Orientation::Vertical, 0);
    caixa.append(&desenho);
    let l = gtk::Label::new(Some(nome));
    l.add_css_class("cartao-nome");
    caixa.append(&l);
    let b = gtk::ToggleButton::new();
    b.add_css_class("cartao");
    b.add_css_class("flat");
    b.set_child(Some(&caixa));
    if let Some(g) = grupo {
        b.set_group(Some(g));
    }
    b
}

fn area(w: i32, h: i32) -> gtk::DrawingArea {
    let a = gtk::DrawingArea::new();
    a.set_content_width(w);
    a.set_content_height(h);
    a
}

// ------------------------------------------------------------------ desenho das prévias

fn rgb(cr: &cairo::Context, r: f64, g: f64, b: f64) {
    cr.set_source_rgb(r / 255.0, g / 255.0, b / 255.0);
}

fn retangulo_arredondado(cr: &cairo::Context, x: f64, y: f64, w: f64, h: f64, r: f64) {
    use std::f64::consts::PI;
    cr.new_sub_path();
    cr.arc(x + w - r, y + r, r, -PI / 2.0, 0.0);
    cr.arc(x + w - r, y + h - r, r, 0.0, PI / 2.0);
    cr.arc(x + r, y + h - r, r, PI / 2.0, PI);
    cr.arc(x + r, y + r, r, PI, 3.0 * PI / 2.0);
    cr.close_path();
}

/// Mini área de trabalho: papel de parede, painel (embaixo/em cima) e uma janela
/// no modo e na cor de destaque dados.
fn desenhar_desktop(cr: &cairo::Context, w: f64, h: f64, fundo: Option<&gdk_pixbuf::Pixbuf>, modo: Modo, cor: &Cor, painel: Painel) {
    retangulo_arredondado(cr, 0.0, 0.0, w, h, 10.0);
    cr.clip();
    match fundo {
        Some(p) => {
            cr.save().ok();
            cr.scale(w / p.width() as f64, h / p.height() as f64);
            cr.set_source_pixbuf(p, 0.0, 0.0);
            cr.paint().ok();
            cr.restore().ok();
        }
        None => {
            rgb(cr, 40.0, 70.0, 110.0);
            cr.paint().ok();
        }
    }
    let escuro = modo == Modo::Escuro;
    let (c0, c1, c2) = (cor.rgb.0 as f64, cor.rgb.1 as f64, cor.rgb.2 as f64);
    // painel
    let ph = h * 0.1;
    let py = if painel == Painel::EmCima { 0.0 } else { h - ph };
    cr.set_source_rgba(0.10, 0.11, 0.13, 0.92);
    cr.rectangle(0.0, py, w, ph);
    cr.fill().ok();
    // botão do menu e ícones do painel
    rgb(cr, c0, c1, c2);
    cr.arc(ph * 0.7, py + ph / 2.0, ph * 0.28, 0.0, 6.3);
    cr.fill().ok();
    cr.set_source_rgba(1.0, 1.0, 1.0, 0.55);
    for i in 0..3 {
        cr.rectangle(ph * 1.5 + i as f64 * ph * 0.9, py + ph * 0.28, ph * 0.55, ph * 0.44);
    }
    cr.fill().ok();
    // janela
    let (jx, jy, jw, jh) = (w * 0.2, if painel == Painel::EmCima { h * 0.22 } else { h * 0.12 }, w * 0.6, h * 0.62);
    // brilho na cor de destaque
    for (i, a) in [(6.0, 0.10), (4.0, 0.16), (2.0, 0.25)] {
        cr.set_source_rgba(c0 / 255.0, c1 / 255.0, c2 / 255.0, a);
        retangulo_arredondado(cr, jx - i, jy - i, jw + 2.0 * i, jh + 2.0 * i, 8.0 + i);
        cr.fill().ok();
    }
    rgb(cr, c0, c1, c2);
    retangulo_arredondado(cr, jx - 1.5, jy - 1.5, jw + 3.0, jh + 3.0, 8.0);
    cr.fill().ok();
    if escuro { rgb(cr, 42.0, 45.0, 50.0) } else { rgb(cr, 246.0, 246.0, 246.0) }
    retangulo_arredondado(cr, jx, jy, jw, jh, 7.0);
    cr.fill().ok();
    // barra de título
    if escuro { rgb(cr, 32.0, 34.0, 38.0) } else { rgb(cr, 222.0, 222.0, 222.0) }
    cr.save().ok();
    retangulo_arredondado(cr, jx, jy, jw, jh, 7.0);
    cr.clip();
    cr.rectangle(jx, jy, jw, jh * 0.16);
    cr.fill().ok();
    cr.restore().ok();
    // botões da barra
    for (i, (r, g, b)) in [(192.0, 57.0, 61.0), (150.0, 150.0, 150.0), (150.0, 150.0, 150.0)].iter().enumerate() {
        rgb(cr, *r, *g, *b);
        cr.arc(jx + jw - 10.0 - i as f64 * 11.0, jy + jh * 0.08, 3.2, 0.0, 6.3);
        cr.fill().ok();
    }
    // conteúdo: linhas de texto e um botão na cor de destaque
    let txt = if escuro { 0.75 } else { 0.35 };
    cr.set_source_rgba(txt, txt, txt, 0.6);
    for i in 0..3 {
        cr.rectangle(jx + 12.0, jy + jh * 0.28 + i as f64 * 10.0, jw * (0.65 - i as f64 * 0.15), 4.0);
    }
    cr.fill().ok();
    rgb(cr, c0, c1, c2);
    retangulo_arredondado(cr, jx + jw - 52.0, jy + jh - 22.0, 40.0, 13.0, 4.0);
    cr.fill().ok();
}

// ------------------------------------------------------------------ páginas

fn pagina_tema(app: &Rc<App>) -> gtk::Box {
    let p = pagina("Tema", "Escolha entre claro e escuro e a cor de destaque das janelas, seleções e botões.");
    let e0 = app.estado.borrow().clone();

    let linha = gtk::Box::new(gtk::Orientation::Horizontal, 16);
    let mut primeiro: Option<gtk::ToggleButton> = None;
    for (modo, nome) in [(Modo::Claro, "Claro"), (Modo::Escuro, "Escuro")] {
        let d = area(260, 150);
        let a = app.clone();
        d.set_draw_func(move |_, cr, w, h| {
            let e = a.estado.borrow();
            desenhar_desktop(cr, w as f64, h as f64, a.fundo.borrow().as_ref(), modo, &e.cor, e.painel);
        });
        app.previas.borrow_mut().push(d.clone());
        let b = cartao(nome, d, primeiro.as_ref());
        b.set_active(e0.modo == modo);
        let a = app.clone();
        b.connect_toggled(move |b| {
            if b.is_active() && a.estado.borrow().modo != modo {
                a.estado.borrow_mut().modo = modo;
                a.aplicar_tema();
            }
        });
        linha.append(&b);
        primeiro.get_or_insert(b);
    }
    p.append(&linha);

    p.append(&rotulo("Cor de destaque", "secao"));
    let cores = gtk::Box::new(gtk::Orientation::Horizontal, 12);
    let botoes: Rc<RefCell<Vec<(gtk::Button, &'static str)>>> = Rc::new(RefCell::new(Vec::new()));
    for c in CORES.iter().copied() {
        let b = gtk::Button::new();
        b.add_css_class("cor");
        b.add_css_class(&format!("cor-{}", c.id));
        b.set_tooltip_text(Some(c.nome));
        b.set_label(if c.id == e0.cor.id { "✓" } else { "" });
        cores.append(&b);
        botoes.borrow_mut().push((b.clone(), c.id));
        let a = app.clone();
        let todos = botoes.clone();
        b.connect_clicked(move |_| {
            for (outro, id) in todos.borrow().iter() {
                outro.set_label(if *id == c.id { "✓" } else { "" });
            }
            a.estado.borrow_mut().cor = c;
            a.aplicar_tema();
        });
    }
    p.append(&cores);
    p.append(&rotulo(
        match app.ambiente {
            Ambiente::Kde => "A cor aparece no contorno arredondado da janela ativa, na barra de título, nas seleções e nos destaques.",
            Ambiente::Gnome => "A cor aparece nos botões, seleções e destaques do GNOME (cor mais próxima entre as do GNOME).",
            _ => "A cor aparece no contorno arredondado da janela ativa.",
        },
        "dim-label",
    ));
    p
}

fn pagina_fundo(app: &Rc<App>, janela: &gtk::ApplicationWindow) -> gtk::Box {
    let p = pagina("Fundo", "Escolha o papel de parede da área de trabalho. Todas as ilustrações são do Doors OS.");
    let grade = gtk::FlowBox::new();
    grade.set_selection_mode(gtk::SelectionMode::None);
    grade.set_max_children_per_line(4);
    grade.set_min_children_per_line(2);
    grade.set_row_spacing(10);
    grade.set_column_spacing(10);
    grade.set_homogeneous(true);

    // nomes da coleção (XML); arquivos fora do XML entram com o nome do arquivo
    let mut lista: Vec<(String, String)> = std::fs::read_to_string(COLECAO_XML).map(|x| tema::nomes_papeis(&x)).unwrap_or_default();
    lista.retain(|(f, _)| std::path::Path::new(f).exists());
    if let Ok(d) = std::fs::read_dir(PAPEIS) {
        for f in d.flatten() {
            let s = f.path().to_string_lossy().to_string();
            if (s.ends_with(".jpg") || s.ends_with(".png")) && !lista.iter().any(|(a, _)| *a == s) {
                let n = f.path().file_stem().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
                lista.push((s, n));
            }
        }
    }
    for (arq, nome) in lista {
        let Ok(pix) = gdk_pixbuf::Pixbuf::from_file_at_scale(&arq, 200, 113, false) else { continue };
        #[allow(deprecated)]
        let pic = gtk::Picture::for_paintable(&gdk::Texture::for_pixbuf(&pix));
        pic.set_size_request(200, 113);
        let caixa = gtk::Box::new(gtk::Orientation::Vertical, 4);
        caixa.append(&pic);
        caixa.append(&gtk::Label::new(Some(&nome)));
        let b = gtk::Button::new();
        b.add_css_class("papel");
        b.add_css_class("flat");
        b.set_child(Some(&caixa));
        let a = app.clone();
        b.connect_clicked(move |_| usar_papel(&a, &arq));
        grade.insert(&b, -1);
    }
    p.append(&grade);

    let botoes = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    let proprio = gtk::Button::with_label("Usar uma imagem minha…");
    {
        let a = app.clone();
        let j = janela.clone();
        proprio.connect_clicked(move |_| {
            let filtro = gtk::FileFilter::new();
            filtro.set_name(Some("Imagens"));
            filtro.add_mime_type("image/*");
            let filtros = gtk::gio::ListStore::new::<gtk::FileFilter>();
            filtros.append(&filtro);
            let dlg = gtk::FileDialog::builder().title("Escolher papel de parede").filters(&filtros).modal(true).build();
            let a = a.clone();
            dlg.open(Some(&j), gtk::gio::Cancellable::NONE, move |r| {
                if let Some(caminho) = r.ok().and_then(|f| f.path()) {
                    usar_papel(&a, &caminho.to_string_lossy());
                }
            });
        });
    }
    botoes.append(&proprio);
    if app.ambiente == Ambiente::Kde {
        let dia = gtk::Button::with_label("Usar a imagem do dia (muda todo dia)");
        let a = app.clone();
        dia.connect_clicked(move |_| a.aplicar("Imagem do dia", a.ambiente.aplicar_imagem_do_dia()));
        botoes.append(&dia);
    }
    p.append(&botoes);
    p
}

fn usar_papel(a: &Rc<App>, arq: &str) {
    a.estado.borrow_mut().papel = Some(arq.to_string());
    a.trocar_fundo(arq);
    a.aplicar("Papel de parede", a.ambiente.aplicar_papel(arq));
}

fn pagina_layout(app: &Rc<App>) -> gtk::Box {
    let p = pagina("Layout", "Escolha onde fica o painel com o menu, os aplicativos abertos e o relógio.");
    let e0 = app.estado.borrow().clone();
    let linha = gtk::Box::new(gtk::Orientation::Horizontal, 16);
    let mut primeiro: Option<gtk::ToggleButton> = None;
    for (painel, nome) in [(Painel::Embaixo, "Clássico: painel embaixo (estilo Mint)"), (Painel::EmCima, "Painel em cima")] {
        let d = area(260, 150);
        let a = app.clone();
        d.set_draw_func(move |_, cr, w, h| {
            let e = a.estado.borrow();
            desenhar_desktop(cr, w as f64, h as f64, a.fundo.borrow().as_ref(), e.modo, &e.cor, painel);
        });
        app.previas.borrow_mut().push(d.clone());
        let b = cartao(nome, d, primeiro.as_ref());
        b.set_active(e0.painel == painel);
        let a = app.clone();
        b.connect_toggled(move |b| {
            if b.is_active() && a.estado.borrow().painel != painel {
                a.estado.borrow_mut().painel = painel;
                a.aplicar("Layout", a.ambiente.aplicar_painel(painel));
            }
        });
        linha.append(&b);
        primeiro.get_or_insert(b);
    }
    p.append(&linha);
    p
}

fn pagina_fontes(app: &Rc<App>) -> gtk::Box {
    let p = pagina("Fontes", "Deixe o texto do sistema maior ou menor. A fonte do Doors OS é a Noto Sans.");
    let e0 = app.estado.borrow().clone();
    p.append(&rotulo("Tamanho do texto", "secao"));
    let linha = gtk::Box::new(gtk::Orientation::Horizontal, 12);
    let mut primeiro: Option<gtk::ToggleButton> = None;
    for (escala, nome) in TAMANHOS_TEXTO {
        let caixa = gtk::Box::new(gtk::Orientation::Vertical, 4);
        let amostra = gtk::Label::new(None);
        amostra.set_markup(&format!("<span size=\"{}%\">Aa</span>", escala * 160 / 100));
        amostra.set_size_request(110, 70);
        caixa.append(&amostra);
        let l = gtk::Label::new(Some(&format!("{nome} ({escala}%)")));
        l.add_css_class("cartao-nome");
        caixa.append(&l);
        let b = gtk::ToggleButton::new();
        b.add_css_class("cartao");
        b.add_css_class("flat");
        b.set_child(Some(&caixa));
        if let Some(g) = primeiro.as_ref() {
            b.set_group(Some(g));
        }
        b.set_active(e0.texto == escala);
        let a = app.clone();
        b.connect_toggled(move |b| {
            if b.is_active() && a.estado.borrow().texto != escala {
                a.estado.borrow_mut().texto = escala;
                let r = a.ambiente.aplicar_texto(escala);
                let o_que = if a.ambiente == Ambiente::Kde { "Tamanho do texto (vale a partir do próximo login)" } else { "Tamanho do texto" };
                a.aplicar(o_que, r);
            }
        });
        linha.append(&b);
        primeiro.get_or_insert(b);
    }
    p.append(&linha);
    p
}

// ------------------------------------------------------------------ janela

fn montar(gapp: &gtk::Application) {
    estilo();
    let ambiente = Ambiente::detectar();
    let aviso = gtk::Label::new(None);
    aviso.set_halign(gtk::Align::Start);
    aviso.set_wrap(true);
    let app = Rc::new(App {
        estado: RefCell::new(Estado::carregar()),
        ambiente,
        aviso: aviso.clone(),
        previas: RefCell::new(Vec::new()),
        fundo: RefCell::new(None),
    });
    let e0 = app.estado.borrow().clone();
    app.trocar_fundo(e0.papel.as_deref().unwrap_or(PADRAO));
    modo_do_app(e0.modo);

    let janela = gtk::ApplicationWindow::builder()
        .application(gapp)
        .title("Aparência")
        .icon_name("doors-logo")
        .default_width(1000)
        .default_height(700)
        .build();

    let pilha = gtk::Stack::new();
    pilha.set_transition_type(gtk::StackTransitionType::Crossfade);
    let paginas: [(&str, &str, gtk::Box); 4] = [
        ("tema", "Tema", pagina_tema(&app)),
        ("fundo", "Fundo", pagina_fundo(&app, &janela)),
        ("layout", "Layout", pagina_layout(&app)),
        ("fontes", "Fontes", pagina_fontes(&app)),
    ];
    for (id, nome, conteudo) in paginas {
        let rolagem = gtk::ScrolledWindow::new();
        rolagem.set_hscrollbar_policy(gtk::PolicyType::Never);
        rolagem.set_child(Some(&conteudo));
        pilha.add_titled(&rolagem, Some(id), nome);
    }

    let lateral = gtk::StackSidebar::new();
    lateral.set_stack(&pilha);
    lateral.set_size_request(190, -1);
    lateral.add_css_class("lateral");

    // cabeçalho da lateral: logo + nome do ambiente
    let topo = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    topo.set_margin_top(16);
    topo.set_margin_bottom(8);
    topo.set_margin_start(16);
    topo.set_margin_end(16);
    let logo = gtk::Image::from_icon_name("doors-logo");
    logo.set_pixel_size(40);
    let tx = gtk::Box::new(gtk::Orientation::Vertical, 0);
    tx.append(&rotulo("Aparência", "secao"));
    tx.append(&rotulo(ambiente.nome(), "dim-label"));
    topo.append(&logo);
    topo.append(&tx);
    let coluna = gtk::Box::new(gtk::Orientation::Vertical, 0);
    coluna.append(&topo);
    lateral.set_vexpand(true);
    coluna.append(&lateral);

    let direita = gtk::Box::new(gtk::Orientation::Vertical, 0);
    pilha.set_hexpand(true);
    pilha.set_vexpand(true);
    direita.append(&pilha);
    direita.append(&gtk::Separator::new(gtk::Orientation::Horizontal));
    aviso.add_css_class("rodape");
    if ambiente == Ambiente::Outro {
        aviso.set_text("Este ambiente gráfico não é suportado. O Doors Aparência funciona no KDE Plasma, Cinnamon, XFCE e GNOME.");
    } else {
        aviso.set_text("Cada escolha é aplicada na hora.");
    }
    direita.append(&aviso);

    let raiz = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    raiz.append(&coluna);
    raiz.append(&gtk::Separator::new(gtk::Orientation::Vertical));
    raiz.append(&direita);
    janela.set_child(Some(&raiz));
    janela.present();
}
