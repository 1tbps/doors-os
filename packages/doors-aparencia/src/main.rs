//! Doors Aparência — personalização do Doors OS em um clique (KDE Plasma, Cinnamon e XFCE).

mod ambiente;
mod estado;
mod tema;

use ambiente::Ambiente;
use estado::Estado;
use gtk::prelude::*;
use gtk::{gdk, gdk_pixbuf, glib};
use std::cell::RefCell;
use std::rc::Rc;
use tema::{Modo, Painel, CORES};

const ID: &str = "org.doors.Aparencia";
const PAPEIS: &str = "/usr/share/backgrounds/doors";

struct App {
    estado: RefCell<Estado>,
    ambiente: Ambiente,
    aviso: gtk::Label,
}

impl App {
    /// Roda uma ação, mostra o resultado e guarda o estado.
    fn aplicar(&self, o_que: &str, r: Result<(), String>) {
        match r {
            Ok(()) => {
                self.aviso.set_markup(&format!("<span foreground=\"#2e9e5b\">✓</span> {o_que} aplicado"));
                self.estado.borrow().salvar();
            }
            Err(e) => self.aviso.set_markup(&format!(
                "<span foreground=\"#c0393d\">✗</span> {o_que}: {}",
                glib::markup_escape_text(&e)
            )),
        }
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

fn titulo(texto: &str) -> gtk::Label {
    let l = gtk::Label::new(None);
    l.set_markup(&format!("<b>{}</b>", glib::markup_escape_text(texto)));
    l.set_halign(gtk::Align::Start);
    l.add_css_class("secao");
    l
}

fn par_de_botoes(a: &str, b: &str, primeiro_ativo: bool) -> (gtk::Box, gtk::ToggleButton, gtk::ToggleButton) {
    let caixa = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    caixa.add_css_class("linked");
    let ba = gtk::ToggleButton::with_label(a);
    let bb = gtk::ToggleButton::with_label(b);
    bb.set_group(Some(&ba));
    for btn in [&ba, &bb] {
        btn.set_size_request(190, 40);
        caixa.append(btn);
    }
    if primeiro_ativo { ba.set_active(true) } else { bb.set_active(true) }
    (caixa, ba, bb)
}

fn estilo() {
    let mut css = String::from(
        ".secao { font-size: 1.1em; margin-top: 6px; }\n\
         .titulo { font-size: 1.6em; font-weight: bold; }\n\
         .cor { min-width: 34px; min-height: 34px; border-radius: 999px; padding: 0; border: 2px solid rgba(0,0,0,0.25); }\n\
         .cor label { color: white; font-weight: bold; }\n\
         .papel { padding: 3px; }\n",
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

fn miniatura(arquivo: &str) -> Option<gtk::Picture> {
    let pix = gdk_pixbuf::Pixbuf::from_file_at_scale(arquivo, 208, 117, true).ok()?;
    #[allow(deprecated)]
    let tex = gdk::Texture::for_pixbuf(&pix);
    let pic = gtk::Picture::for_paintable(&tex);
    pic.set_size_request(208, 117);
    Some(pic)
}

/// nome legível a partir do arquivo: "portaencantada.jpg" -> "Portaencantada"
fn nome_papel(arquivo: &str) -> String {
    let base = std::path::Path::new(arquivo).file_stem().map(|s| s.to_string_lossy().to_string()).unwrap_or_default();
    match base.as_str() {
        "portaencantada" => "Porta Encantada".into(),
        "portaazul" => "Porta Azul".into(),
        "casarao" => "Casarão".into(),
        "numero45" => "Número 45".into(),
        "grade" => "Grade vermelha".into(),
        _ => {
            let mut c = base.chars();
            c.next().map(|p| p.to_uppercase().collect::<String>() + c.as_str()).unwrap_or_default()
        }
    }
}

fn montar(gapp: &gtk::Application) {
    estilo();
    let ambiente = Ambiente::detectar();
    let aviso = gtk::Label::new(None);
    aviso.set_halign(gtk::Align::Start);
    aviso.set_wrap(true);
    let app = Rc::new(App { estado: RefCell::new(Estado::carregar()), ambiente, aviso: aviso.clone() });
    let e0 = app.estado.borrow().clone();

    let raiz = gtk::Box::new(gtk::Orientation::Vertical, 10);
    for m in [&raiz] {
        m.set_margin_top(20);
        m.set_margin_bottom(20);
        m.set_margin_start(24);
        m.set_margin_end(24);
    }

    // cabeçalho
    let cab = gtk::Box::new(gtk::Orientation::Horizontal, 12);
    let logo = gtk::Image::from_icon_name("doors-logo");
    logo.set_pixel_size(48);
    let textos = gtk::Box::new(gtk::Orientation::Vertical, 2);
    let t = gtk::Label::new(Some("Aparência"));
    t.add_css_class("titulo");
    t.set_halign(gtk::Align::Start);
    let sub = gtk::Label::new(Some(&format!("Personalize o Doors OS com um clique — {}", ambiente.nome())));
    sub.set_halign(gtk::Align::Start);
    sub.add_css_class("dim-label");
    textos.append(&t);
    textos.append(&sub);
    cab.append(&logo);
    cab.append(&textos);
    raiz.append(&cab);
    // resultado do último clique (no topo: sempre visível)
    raiz.append(&aviso);
    modo_do_app(e0.modo);

    // tema
    raiz.append(&titulo("Tema"));
    let (caixa, claro, escuro) = par_de_botoes("☀  Claro", "☾  Escuro", e0.modo == Modo::Claro);
    caixa.set_halign(gtk::Align::Start);
    raiz.append(&caixa);
    for (btn, modo) in [(claro, Modo::Claro), (escuro, Modo::Escuro)] {
        let app = app.clone();
        btn.connect_toggled(move |b| {
            if b.is_active() && app.estado.borrow().modo != modo {
                app.estado.borrow_mut().modo = modo;
                app.aplicar_tema();
            }
        });
    }

    // cor de destaque
    raiz.append(&titulo("Cor de destaque"));
    let cores = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    let botoes_cor: Rc<RefCell<Vec<(gtk::Button, &'static str)>>> = Rc::new(RefCell::new(Vec::new()));
    for c in CORES.iter().copied() {
        let b = gtk::Button::new();
        b.add_css_class("cor");
        b.add_css_class(&format!("cor-{}", c.id));
        b.set_tooltip_text(Some(c.nome));
        b.set_label(if c.id == e0.cor.id { "✓" } else { "" });
        cores.append(&b);
        botoes_cor.borrow_mut().push((b.clone(), c.id));
        let app = app.clone();
        let todos = botoes_cor.clone();
        b.connect_clicked(move |_| {
            for (outro, id) in todos.borrow().iter() {
                outro.set_label(if *id == c.id { "✓" } else { "" });
            }
            app.estado.borrow_mut().cor = c;
            app.aplicar_tema();
        });
    }
    let nota_cor = gtk::Label::new(Some(match ambiente {
        Ambiente::Kde => "Muda a barra de título da janela ativa, as seleções e os destaques.",
        _ => "Muda o contorno da janela ativa.",
    }));
    nota_cor.set_halign(gtk::Align::Start);
    nota_cor.add_css_class("dim-label");
    raiz.append(&cores);
    raiz.append(&nota_cor);

    // painel
    raiz.append(&titulo("Painel"));
    let (caixa, baixo, cima) = par_de_botoes("Embaixo (estilo Mint)", "Em cima", e0.painel == Painel::Embaixo);
    caixa.set_halign(gtk::Align::Start);
    raiz.append(&caixa);
    for (btn, painel) in [(baixo, Painel::Embaixo), (cima, Painel::EmCima)] {
        let app = app.clone();
        btn.connect_toggled(move |b| {
            if b.is_active() && app.estado.borrow().painel != painel {
                app.estado.borrow_mut().painel = painel;
                app.aplicar("Painel", app.ambiente.aplicar_painel(painel));
            }
        });
    }

    // papel de parede
    raiz.append(&titulo("Papel de parede"));
    let grade = gtk::FlowBox::new();
    grade.set_selection_mode(gtk::SelectionMode::None);
    grade.set_max_children_per_line(4);
    grade.set_min_children_per_line(2);
    grade.set_row_spacing(8);
    grade.set_column_spacing(8);
    let mut arquivos: Vec<String> = std::fs::read_dir(PAPEIS)
        .map(|d| d.flatten().map(|f| f.path().to_string_lossy().to_string()).filter(|p| p.ends_with(".jpg") || p.ends_with(".png")).collect())
        .unwrap_or_default();
    // a ilustração principal primeiro
    arquivos.sort_by_key(|a| (!a.contains("portaencantada"), !a.contains("portaazul"), a.clone()));
    for arq in arquivos {
        let Some(pic) = miniatura(&arq) else { continue };
        let caixa = gtk::Box::new(gtk::Orientation::Vertical, 3);
        caixa.append(&pic);
        caixa.append(&gtk::Label::new(Some(&nome_papel(&arq))));
        let b = gtk::Button::new();
        b.add_css_class("papel");
        b.add_css_class("flat");
        b.set_child(Some(&caixa));
        let app = app.clone();
        b.connect_clicked(move |_| {
            app.estado.borrow_mut().papel = Some(arq.clone());
            app.aplicar("Papel de parede", app.ambiente.aplicar_papel(&arq));
        });
        grade.insert(&b, -1);
    }
    raiz.append(&grade);
    if ambiente == Ambiente::Kde {
        let dia = gtk::Button::with_label("Usar a imagem do dia (muda todo dia)");
        dia.set_halign(gtk::Align::Start);
        let app = app.clone();
        dia.connect_clicked(move |_| app.aplicar("Imagem do dia", app.ambiente.aplicar_imagem_do_dia()));
        raiz.append(&dia);
    }

    if ambiente == Ambiente::Outro {
        aviso.set_text("Este ambiente gráfico não é suportado. O Doors Aparência funciona no KDE Plasma, Cinnamon e XFCE.");
    }

    let rolagem = gtk::ScrolledWindow::new();
    rolagem.set_hscrollbar_policy(gtk::PolicyType::Never);
    rolagem.set_child(Some(&raiz));
    let janela = gtk::ApplicationWindow::builder()
        .application(gapp)
        .title("Doors Aparência")
        .icon_name("doors-logo")
        .default_width(940)
        .default_height(720)
        .child(&rolagem)
        .build();
    janela.present();
}
