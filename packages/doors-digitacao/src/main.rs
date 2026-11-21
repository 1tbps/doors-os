//! Doors Digitação — curso de digitação todo em português (Rust + GTK4).
//! Páginas: Lições, Teste de velocidade, Jogo, Progresso e Teclado.

mod licoes;
mod motor;
mod progresso;
mod teclado;
mod textos;

use gtk::prelude::*;
use gtk::{cairo, gdk, glib};
use licoes::{Sorteio, LICOES};
use motor::{estrelas, Resultado, Sessao};
use progresso::Progresso;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::Duration;
use teclado::{Layout, Passo};

const ID: &str = "org.doors.Digitacao";
const DURACOES_TESTE: [(u64, &str); 3] = [(30, "30 segundos"), (60, "1 minuto"), (120, "2 minutos")];

struct App {
    progresso: RefCell<Progresso>,
    detectado: (Layout, bool),
    janela: RefCell<Option<gtk::ApplicationWindow>>,
}

impl App {
    fn layout(&self) -> Layout {
        Layout::de_id(&self.progresso.borrow().layout).unwrap_or(self.detectado.0)
    }
    fn semente() -> u64 {
        glib::real_time() as u64
    }
}

fn main() -> glib::ExitCode {
    let app = gtk::Application::builder().application_id(ID).build();
    app.connect_activate(montar);
    app.run()
}

// ------------------------------------------------------------------ estilo e peças comuns

fn estilo() {
    let css = "
        .pagina-titulo { font-size: 1.5em; font-weight: bold; }
        .secao { font-weight: bold; margin-top: 8px; }
        .texto-treino { font-family: monospace; font-size: 22px; padding: 16px 20px; border-radius: 12px;
                        background: alpha(currentColor, 0.06); }
        .numero-grande { font-size: 2.2em; font-weight: bold; }
        .estrelas { font-size: 1.6em; color: #e0a020; }
        .cartao-licao { padding: 10px 12px; border-radius: 12px; min-width: 190px; }
        .cartao-licao label.titulo { font-weight: bold; font-size: 1.05em; }
        .lateral { padding: 8px; }
        .lateral row { padding: 10px 14px; border-radius: 8px; }
        .captura { opacity: 0; min-width: 1px; min-height: 1px; }
        .dica { font-size: 1.05em; }
        .resultado { padding: 18px; border-radius: 14px; background: alpha(@theme_selected_bg_color, 0.12); }
    ";
    let p = gtk::CssProvider::new();
    #[allow(deprecated)]
    p.load_from_data(css);
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
    if !descricao.is_empty() {
        b.append(&rotulo(descricao, "dim-label"));
    }
    b
}

fn limpar(b: &gtk::Box) {
    while let Some(c) = b.first_child() {
        b.remove(&c);
    }
}

fn texto_estrelas(n: u8) -> String {
    (0..3).map(|i| if i < n { '★' } else { '☆' }).collect()
}

fn hoje() -> String {
    glib::DateTime::now_local().ok().and_then(|d| d.format("%d/%m/%Y").ok()).map(|s| s.to_string()).unwrap_or_default()
}

// ------------------------------------------------------------------ teclado na tela

/// Cores dos dedos (mínimo, anelar, médio, indicador ×2 mãos, polegar).
const CORES_DEDOS: [(f64, f64, f64); 9] = [
    (0.91, 0.55, 0.62), (0.95, 0.73, 0.45), (0.96, 0.86, 0.45), (0.55, 0.80, 0.55),
    (0.50, 0.72, 0.92), (0.96, 0.86, 0.45), (0.95, 0.73, 0.45), (0.91, 0.55, 0.62), (0.75, 0.70, 0.90),
];

fn caixa(cr: &cairo::Context, x: f64, y: f64, w: f64, h: f64, r: f64) {
    use std::f64::consts::PI;
    cr.new_sub_path();
    cr.arc(x + w - r, y + r, r, -PI / 2.0, 0.0);
    cr.arc(x + w - r, y + h - r, r, 0.0, PI / 2.0);
    cr.arc(x + r, y + h - r, r, PI / 2.0, PI);
    cr.arc(x + r, y + r, r, PI, 3.0 * PI / 2.0);
    cr.close_path();
}

fn texto_centro(cr: &cairo::Context, s: &str, cx: f64, cy: f64, tam: f64) {
    cr.select_font_face("Noto Sans", cairo::FontSlant::Normal, cairo::FontWeight::Bold);
    cr.set_font_size(tam);
    if let Ok(ext) = cr.text_extents(s) {
        cr.move_to(cx - ext.width() / 2.0 - ext.x_bearing(), cy - ext.height() / 2.0 - ext.y_bearing());
        let _ = cr.show_text(s);
    }
}

/// Desenha o teclado do layout, com as teclas de `destaque` marcadas (1, 2 na ordem).
fn desenhar_teclado(cr: &cairo::Context, w: f64, h: f64, layout: Layout, destaque: &[Passo], escuro: bool) {
    let fileiras = teclado::fileiras(layout);
    let u = (w / 15.2).min(h / 5.4);
    let x0 = (w - u * 15.0) / 2.0;
    let gap = u * 0.08;
    let deslocamento = [0.0, 1.5, 1.8, if layout == Layout::Abnt2 { 1.3 } else { 2.3 }];
    let marcada = |f: usize, c: usize| destaque.iter().position(|p| p.fileira == f && p.coluna == c);
    let fundo = if escuro { 0.20 } else { 0.93 };
    let tinta = if escuro { 0.92 } else { 0.15 };
    let tecla = |x: f64, y: f64, kw: f64, cor: Option<(f64, f64, f64)>, marca: Option<usize>, rot: &str| {
        caixa(cr, x + gap, y + gap, kw - 2.0 * gap, u - 2.0 * gap, u * 0.14);
        match (marca, cor) {
            (Some(_), _) => cr.set_source_rgb(0.18, 0.47, 0.71),
            (None, Some((r, g, b))) => {
                let m = if escuro { 0.45 } else { 0.55 };
                cr.set_source_rgb(fundo * (1.0 - m) + r * m, fundo * (1.0 - m) + g * m, fundo * (1.0 - m) + b * m)
            }
            _ => cr.set_source_rgb(fundo, fundo, fundo),
        }
        let _ = cr.fill();
        if marca.is_some() { cr.set_source_rgb(1.0, 1.0, 1.0) } else { cr.set_source_rgb(tinta, tinta, tinta) }
        texto_centro(cr, rot, x + kw / 2.0, y + u / 2.0, u * 0.36);
        if let (Some(n), true) = (marca, destaque.len() > 1) {
            cr.set_source_rgb(1.0, 0.82, 0.48);
            texto_centro(cr, &(n + 1).to_string(), x + kw - u * 0.22, y + u * 0.24, u * 0.22);
        }
    };
    for (f, linha) in fileiras.iter().enumerate() {
        let y = f as f64 * u + (h - u * 5.0) / 2.0;
        let mut x = x0 + deslocamento[f] * u;
        if f == 3 {
            // Shift esquerdo
            let precisa = destaque.iter().any(|p| p.shift && p.fileira < 4 && fileiras[p.fileira][p.coluna].dedo >= 4);
            tecla(x0, y, deslocamento[3] * u, Some(CORES_DEDOS[0]), precisa.then_some(0), "Shift");
        }
        for (c, k) in linha.iter().enumerate() {
            let rot = if k.normal.is_alphabetic() { k.normal.to_uppercase().to_string() } else { k.normal.to_string() };
            tecla(x, y, u, Some(CORES_DEDOS[k.dedo as usize]), marcada(f, c), &rot);
            x += u;
        }
        if f == 3 {
            let precisa = destaque.iter().any(|p| p.shift && p.fileira < 4 && fileiras[p.fileira][p.coluna].dedo <= 3);
            let fim = x0 + 15.0 * u;
            tecla(x, y, fim - x, Some(CORES_DEDOS[7]), precisa.then_some(0), "Shift");
        }
    }
    // barra de espaço
    let y = 4.0 * u + (h - u * 5.0) / 2.0;
    tecla(x0 + 4.0 * u, y, 6.5 * u, Some(CORES_DEDOS[8]), marcada(4, 0), "espaço");
}

fn e_escuro() -> bool {
    gtk::Settings::default().map(|s| s.is_gtk_application_prefer_dark_theme() || s.gtk_theme_name().map(|n| n.to_lowercase().contains("dark")).unwrap_or(false)).unwrap_or(false)
}

// ------------------------------------------------------------------ captura do que é digitado

/// Campo invisível que recebe o texto já composto (acentos por tecla morta, ç etc.).
fn captura(ao_digitar: impl Fn(char) + 'static) -> gtk::Text {
    let t = gtk::Text::new();
    t.add_css_class("captura");
    t.set_can_focus(true);
    let limpando = Rc::new(Cell::new(false));
    t.connect_changed(move |e| {
        if limpando.get() {
            return;
        }
        let s = e.text().to_string();
        if s.is_empty() {
            return;
        }
        limpando.set(true);
        e.set_text("");
        limpando.set(false);
        for c in s.chars() {
            ao_digitar(c);
        }
    });
    t
}

// ------------------------------------------------------------------ treino (lição ou teste)

#[derive(Clone, Copy)]
enum Modo {
    Licao(usize),
    Teste(u64),
}

/// Monta a tela de treino dentro de `area`. `voltar` é chamado pelo botão Voltar.
fn treino(app: &Rc<App>, area: &gtk::Box, modo: Modo, voltar: Rc<dyn Fn()>) {
    limpar(area);
    let layout = app.layout();
    let (titulo, texto, limite) = match modo {
        Modo::Licao(i) => (
            format!("Lição {} — {}", i + 1, LICOES[i].titulo),
            licoes::texto(i, layout, App::semente(), 110),
            None,
        ),
        Modo::Teste(seg) => (
            format!("Teste de velocidade — {}", DURACOES_TESTE.iter().find(|d| d.0 == seg).map(|d| d.1).unwrap_or("")),
            licoes::frases(layout, App::semente(), 1500),
            Some(Duration::from_secs(seg)),
        ),
    };
    let sessao = Rc::new(RefCell::new(Sessao::nova(&texto, limite)));

    let topo = gtk::Box::new(gtk::Orientation::Horizontal, 12);
    let t = rotulo(&titulo, "pagina-titulo");
    t.set_hexpand(true);
    topo.append(&t);
    let recomecar = gtk::Button::with_label("Recomeçar");
    let botao_voltar = gtk::Button::with_label("Voltar");
    topo.append(&recomecar);
    topo.append(&botao_voltar);
    area.append(&topo);
    if let Modo::Licao(i) = modo {
        let novas = licoes::novas(i, layout);
        let extra = if novas.is_empty() { String::new() } else { format!("  Teclas novas: {}", novas.chars().map(|c| c.to_string()).collect::<Vec<_>>().join(" ")) };
        area.append(&rotulo(&format!("{}{}", LICOES[i].descricao, extra), "dim-label"));
    } else {
        area.append(&rotulo("O tempo começa na primeira tecla. Erros não precisam ser apagados: digite o caractere certo para seguir.", "dim-label"));
    }

    let quadro = rotulo("", "texto-treino");
    quadro.set_selectable(false);
    quadro.set_halign(gtk::Align::Fill);
    area.append(&quadro);

    let numeros = gtk::Box::new(gtk::Orientation::Horizontal, 36);
    let mk = |nome: &str| {
        let b = gtk::Box::new(gtk::Orientation::Vertical, 0);
        let v = rotulo("—", "numero-grande");
        b.append(&v);
        b.append(&rotulo(nome, "dim-label"));
        numeros.append(&b);
        v
    };
    let v_ppm = mk("palavras por minuto");
    let v_prec = mk("precisão");
    let v_tempo = mk(if limite.is_some() { "restante" } else { "tempo" });
    area.append(&numeros);

    let dica = rotulo("", "dica");
    area.append(&dica);

    let desenho = gtk::DrawingArea::new();
    desenho.set_content_height(250);
    desenho.set_hexpand(true);
    area.append(&desenho);

    let resultado = gtk::Box::new(gtk::Orientation::Vertical, 8);
    resultado.add_css_class("resultado");
    resultado.set_visible(false);
    area.append(&resultado);

    // desenho do teclado com a próxima tecla
    {
        let s = sessao.clone();
        desenho.set_draw_func(move |_, cr, w, h| {
            let p = s.borrow().esperado().map(|c| teclado::passos(layout, c)).unwrap_or_default();
            desenhar_teclado(cr, w as f64, h as f64, layout, &p, e_escuro());
        });
    }

    let atualizar: Rc<dyn Fn()> = {
        let (s, quadro, v_ppm, v_prec, v_tempo, dica, desenho) =
            (sessao.clone(), quadro.clone(), v_ppm.clone(), v_prec.clone(), v_tempo.clone(), dica.clone(), desenho.clone());
        Rc::new(move || {
            let s = s.borrow();
            // janela do texto em volta do cursor
            let ini = s.pos.saturating_sub(60);
            let fim = (s.pos + 140).min(s.alvo.len());
            let mut m = String::new();
            for k in ini..fim {
                let c = s.alvo[k];
                let txt = if c == ' ' && k == s.pos { "␣".to_string() } else { glib::markup_escape_text(&c.to_string()).to_string() };
                if k < s.pos {
                    let cor = if s.errou[k] { "#c0393d" } else { "#2e9e5b" };
                    m.push_str(&format!("<span foreground=\"{cor}\">{txt}</span>"));
                } else if k == s.pos && !s.terminou() {
                    m.push_str(&format!("<span background=\"#2f78b5\" foreground=\"#ffffff\"><u>{txt}</u></span>"));
                } else {
                    m.push_str(&txt);
                }
            }
            quadro.set_markup(&m);
            if s.comecou() {
                v_ppm.set_text(&format!("{:.0}", s.ppm()));
                v_prec.set_text(&format!("{:.0}%", s.precisao()));
            } else {
                v_ppm.set_text("—");
                v_prec.set_text("—");
            }
            v_tempo.set_text(&match s.restante() {
                Some(r) => format!("{:.0}s", r.ceil()),
                None => format!("{:.0}s", s.segundos()),
            });
            dica.set_text(&match s.esperado() {
                Some(' ') if !s.terminou() => "Próximo: espaço  ·  dedo polegar".to_string(),
                Some(c) if !s.terminou() => format!("Próximo: “{c}”   →   {}", teclado::dica(layout, c)),
                _ => String::new(),
            });
            desenho.queue_draw();
        })
    };

    // fim: resultado, estrelas e pontos
    let concluir: Rc<dyn Fn()> = {
        let (app, s, resultado, area) = (app.clone(), sessao.clone(), resultado.clone(), area.clone());
        let voltar = voltar.clone();
        Rc::new(move || {
            let (ppm, prec, n) = {
                let s = s.borrow();
                (s.ppm(), s.precisao(), s.pos)
            };
            limpar(&resultado);
            let mut p = app.progresso.borrow_mut();
            let (cabecalho, ganho) = match modo {
                Modo::Licao(i) => {
                    let st = estrelas(ppm, prec, LICOES[i].meta_ppm);
                    let ganho = p.registrar_licao(i, progresso::Resultado { estrelas: st, ppm: ppm.round() as u32, precisao: prec.round() as u32 }, n);
                    let msg = match st {
                        0 => "Quase lá! Para passar, a precisão precisa chegar a 90%. Vá com calma.".to_string(),
                        3 => "Excelente! Três estrelas.".to_string(),
                        _ => format!("Muito bem! Para 3 estrelas: 97% de precisão e {} palavras por minuto.", LICOES[i].meta_ppm),
                    };
                    (format!("{}  {msg}", texto_estrelas(st)), ganho)
                }
                Modo::Teste(_) => {
                    let ganho = p.registrar_teste(progresso::Teste { data: hoje(), ppm: ppm.round() as u32, precisao: prec.round() as u32 }, n);
                    let recorde = p.melhor_ppm_teste() == ppm.round() as u32;
                    (if recorde { "Novo recorde pessoal!".to_string() } else { "Teste concluído.".to_string() }, ganho)
                }
            };
            p.salvar();
            let (nivel, _, _) = p.nivel();
            drop(p);
            let c = rotulo(&cabecalho, "estrelas");
            resultado.append(&c);
            resultado.append(&rotulo(
                &format!("{ppm:.0} palavras por minuto  ·  {prec:.0}% de precisão  ·  +{ganho} pontos  ·  nível {nivel}"),
                "secao",
            ));
            let botoes = gtk::Box::new(gtk::Orientation::Horizontal, 10);
            let repetir = gtk::Button::with_label("Tentar de novo");
            {
                let (app, area, voltar) = (app.clone(), area.clone(), voltar.clone());
                repetir.connect_clicked(move |_| treino(&app, &area, modo, voltar.clone()));
            }
            botoes.append(&repetir);
            if let Modo::Licao(i) = modo {
                if i + 1 < LICOES.len() && app.progresso.borrow().liberada(i + 1) {
                    let prox = gtk::Button::with_label("Próxima lição →");
                    prox.add_css_class("suggested-action");
                    let (app, area, voltar) = (app.clone(), area.clone(), voltar.clone());
                    prox.connect_clicked(move |_| treino(&app, &area, Modo::Licao(i + 1), voltar.clone()));
                    botoes.append(&prox);
                }
            }
            let lista = gtk::Button::with_label("Voltar");
            let v = voltar.clone();
            lista.connect_clicked(move |_| v());
            botoes.append(&lista);
            resultado.append(&botoes);
            resultado.set_visible(true);
        })
    };

    let entrada = {
        let (s, atualizar, concluir) = (sessao.clone(), atualizar.clone(), concluir.clone());
        captura(move |c| {
            if c == '\n' || c == '\r' {
                return;
            }
            let r = s.borrow_mut().digitar(c);
            atualizar();
            if r == Resultado::Fim {
                concluir();
            }
        })
    };
    area.append(&entrada);
    // relógio (tempo, PPM e fim dos testes)
    {
        let (s, atualizar, concluir, area) = (sessao.clone(), atualizar.clone(), concluir.clone(), area.clone());
        let entrada = entrada.clone();
        glib::timeout_add_local(Duration::from_millis(250), move || {
            if entrada.root().is_none() || area.first_child().is_none() {
                return glib::ControlFlow::Break;
            }
            if s.borrow_mut().verificar_tempo() {
                atualizar();
                concluir();
                return glib::ControlFlow::Break;
            }
            if s.borrow().terminou() {
                return glib::ControlFlow::Break;
            }
            if s.borrow().comecou() {
                atualizar();
            }
            glib::ControlFlow::Continue
        });
    }
    // clique em qualquer lugar devolve o foco ao campo invisível
    let clique = gtk::GestureClick::new();
    {
        let e = entrada.clone();
        clique.connect_pressed(move |_, _, _, _| {
            e.grab_focus();
        });
    }
    area.add_controller(clique);
    {
        let (app, area2, voltar) = (app.clone(), area.clone(), voltar.clone());
        recomecar.connect_clicked(move |_| treino(&app, &area2, modo, voltar.clone()));
    }
    {
        let v = voltar.clone();
        botao_voltar.connect_clicked(move |_| v());
    }
    atualizar();
    entrada.grab_focus();
}

// ------------------------------------------------------------------ páginas

fn pagina_licoes(app: &Rc<App>) -> gtk::Box {
    let raiz = gtk::Box::new(gtk::Orientation::Vertical, 0);
    let mostrar_lista: Rc<RefCell<Option<Rc<dyn Fn()>>>> = Rc::new(RefCell::new(None));
    let lista: Rc<dyn Fn()> = {
        let (app, raiz, ml) = (app.clone(), raiz.clone(), mostrar_lista.clone());
        Rc::new(move || {
            limpar(&raiz);
            let p = pagina("Lições", "");
            let l = app.layout();
            p.append(&rotulo(
                &format!("Siga na ordem: cada lição libera a próxima. Teclado: {}.", l.nome()),
                "dim-label",
            ));
            let grade = gtk::FlowBox::new();
            grade.set_selection_mode(gtk::SelectionMode::None);
            grade.set_max_children_per_line(4);
            grade.set_min_children_per_line(2);
            grade.set_row_spacing(10);
            grade.set_column_spacing(10);
            grade.set_homogeneous(true);
            let prog = app.progresso.borrow().clone();
            for (i, li) in LICOES.iter().enumerate() {
                let r = prog.resultado(i);
                let liberada = prog.liberada(i);
                let caixa = gtk::Box::new(gtk::Orientation::Vertical, 4);
                let titulo = gtk::Label::new(Some(&format!("{}. {}", i + 1, li.titulo)));
                titulo.add_css_class("titulo");
                titulo.set_halign(gtk::Align::Start);
                caixa.append(&titulo);
                let est = gtk::Label::new(Some(&if liberada { texto_estrelas(r.estrelas) } else { "🔒".to_string() }));
                est.add_css_class("estrelas");
                est.set_halign(gtk::Align::Start);
                caixa.append(&est);
                let sub = if r.ppm > 0 { format!("melhor: {} ppm · {}%", r.ppm, r.precisao) } else if liberada { "ainda não feita".into() } else { "conclua a anterior".into() };
                let s = gtk::Label::new(Some(&sub));
                s.add_css_class("dim-label");
                s.set_halign(gtk::Align::Start);
                caixa.append(&s);
                let b = gtk::Button::new();
                b.add_css_class("cartao-licao");
                b.set_child(Some(&caixa));
                b.set_sensitive(liberada);
                let (app, raiz, ml) = (app.clone(), raiz.clone(), ml.clone());
                b.connect_clicked(move |_| {
                    let voltar = ml.borrow().clone().unwrap();
                    let area = gtk::Box::new(gtk::Orientation::Vertical, 12);
                    area.set_margin_top(20);
                    area.set_margin_bottom(20);
                    area.set_margin_start(24);
                    area.set_margin_end(24);
                    limpar(&raiz);
                    raiz.append(&area);
                    treino(&app, &area, Modo::Licao(i), voltar);
                });
                grade.insert(&b, -1);
            }
            p.append(&grade);
            raiz.append(&p);
        })
    };
    *mostrar_lista.borrow_mut() = Some(lista.clone());
    lista();
    raiz
}

fn pagina_teste(app: &Rc<App>) -> gtk::Box {
    let raiz = gtk::Box::new(gtk::Orientation::Vertical, 0);
    let inicio: Rc<RefCell<Option<Rc<dyn Fn()>>>> = Rc::new(RefCell::new(None));
    let mostrar: Rc<dyn Fn()> = {
        let (app, raiz, ini) = (app.clone(), raiz.clone(), inicio.clone());
        Rc::new(move || {
            limpar(&raiz);
            let p = pagina("Teste de velocidade", "Textos em português, com acentos e pontuação. Veja quantas palavras por minuto você digita.");
            let melhor = app.progresso.borrow().melhor_ppm_teste();
            if melhor > 0 {
                p.append(&rotulo(&format!("Seu recorde: {melhor} palavras por minuto."), "secao"));
            }
            let linha = gtk::Box::new(gtk::Orientation::Horizontal, 12);
            for (seg, nome) in DURACOES_TESTE {
                let b = gtk::Button::with_label(nome);
                b.set_size_request(150, 48);
                if seg == 60 {
                    b.add_css_class("suggested-action");
                }
                let (app, raiz, ini) = (app.clone(), raiz.clone(), ini.clone());
                b.connect_clicked(move |_| {
                    let voltar = ini.borrow().clone().unwrap();
                    let area = gtk::Box::new(gtk::Orientation::Vertical, 12);
                    area.set_margin_top(20);
                    area.set_margin_bottom(20);
                    area.set_margin_start(24);
                    area.set_margin_end(24);
                    limpar(&raiz);
                    raiz.append(&area);
                    treino(&app, &area, Modo::Teste(seg), voltar);
                });
                linha.append(&b);
            }
            p.append(&linha);
            raiz.append(&p);
        })
    };
    *inicio.borrow_mut() = Some(mostrar.clone());
    mostrar();
    raiz
}

// ------------------------------------------------------------------ jogo: palavras caindo

struct Caindo {
    texto: String,
    x: f64,
    y: f64,
}

struct Jogo {
    palavras: Vec<Caindo>,
    digitado: String,
    pontos: u32,
    vidas: u32,
    ativo: bool,
    ticks: u64,
    sorteio: Sorteio,
    erro: u32,
}

fn pagina_jogo(app: &Rc<App>) -> gtk::Box {
    let p = pagina("Jogo: palavras caindo", "Digite cada palavra antes que ela chegue ao chão. Quanto mais você acerta, mais rápido fica.");
    let layout = app.layout();
    let banco: Rc<Vec<String>> = Rc::new(
        textos::PALAVRAS.iter().filter(|w| w.chars().count() >= 3).map(|w| teclado::adaptar(layout, w)).collect(),
    );
    let jogo = Rc::new(RefCell::new(Jogo { palavras: vec![], digitado: String::new(), pontos: 0, vidas: 3, ativo: false, ticks: 0, sorteio: Sorteio::novo(App::semente()), erro: 0 }));
    let placar = rotulo("", "secao");
    p.append(&placar);
    let desenho = gtk::DrawingArea::new();
    desenho.set_content_height(440);
    desenho.set_hexpand(true);
    p.append(&desenho);
    let iniciar = gtk::Button::with_label("Começar");
    iniciar.add_css_class("suggested-action");
    iniciar.set_halign(gtk::Align::Start);
    p.append(&iniciar);
    {
        let j = jogo.clone();
        desenho.set_draw_func(move |_, cr, w, h| {
            let (w, h) = (w as f64, h as f64);
            let j = j.borrow();
            caixa(cr, 0.0, 0.0, w, h, 14.0);
            cr.set_source_rgb(0.06, 0.10, 0.17);
            let _ = cr.fill();
            cr.set_source_rgb(0.18, 0.47, 0.71);
            cr.rectangle(0.0, h - 6.0, w, 6.0);
            let _ = cr.fill();
            for pal in &j.palavras {
                let x = pal.x * (w - 160.0) + 20.0;
                let y = pal.y * (h - 40.0) + 24.0;
                let acerto = pal.texto.starts_with(&j.digitado) && !j.digitado.is_empty();
                cr.select_font_face("Noto Sans", cairo::FontSlant::Normal, cairo::FontWeight::Bold);
                cr.set_font_size(22.0);
                if acerto {
                    let n = j.digitado.chars().count();
                    let feito: String = pal.texto.chars().take(n).collect();
                    let resto: String = pal.texto.chars().skip(n).collect();
                    cr.set_source_rgb(1.0, 0.82, 0.48);
                    cr.move_to(x, y);
                    let _ = cr.show_text(&feito);
                    cr.set_source_rgb(0.95, 0.96, 0.97);
                    let _ = cr.show_text(&resto);
                } else {
                    cr.set_source_rgb(0.95, 0.96, 0.97);
                    cr.move_to(x, y);
                    let _ = cr.show_text(&pal.texto);
                }
            }
            if !j.ativo {
                cr.set_source_rgb(0.95, 0.96, 0.97);
                texto_centro(cr, if j.vidas == 0 { "Fim de jogo" } else { "Clique em Começar" }, w / 2.0, h / 2.0, 30.0);
            }
            if j.erro > 0 {
                cr.set_source_rgba(0.75, 0.22, 0.24, 0.35);
                cr.rectangle(0.0, 0.0, w, h);
                let _ = cr.fill();
            }
        });
    }
    let atualizar_placar = {
        let (j, placar, app) = (jogo.clone(), placar.clone(), app.clone());
        move || {
            let j = j.borrow();
            placar.set_text(&format!(
                "Pontos: {}   ·   Vidas: {}   ·   Digitando: {}   ·   Recorde: {}",
                j.pontos,
                "♥".repeat(j.vidas as usize),
                if j.digitado.is_empty() { "—".to_string() } else { j.digitado.clone() },
                app.progresso.borrow().recorde_jogo
            ));
        }
    };
    atualizar_placar();
    let entrada = {
        let (j, desenho, ap) = (jogo.clone(), desenho.clone(), atualizar_placar.clone());
        captura(move |c| {
            let mut j = j.borrow_mut();
            if !j.ativo {
                return;
            }
            if c == ' ' || c == '\n' {
                j.digitado.clear();
            } else {
                j.digitado.push(c);
                let d = j.digitado.clone();
                if let Some(k) = j.palavras.iter().position(|p| p.texto == d) {
                    let n = j.palavras[k].texto.chars().count() as u32;
                    j.palavras.remove(k);
                    j.pontos += n * 10;
                    j.digitado.clear();
                } else if !j.palavras.iter().any(|p| p.texto.starts_with(&d)) {
                    j.digitado.clear();
                    j.erro = 4;
                }
            }
            drop(j);
            ap();
            desenho.queue_draw();
        })
    };
    p.append(&entrada);
    {
        let (j, desenho, ap, banco, app, entrada) = (jogo.clone(), desenho.clone(), atualizar_placar.clone(), banco.clone(), app.clone(), entrada.clone());
        let botao = iniciar.clone();
        iniciar.connect_clicked(move |_| {
            {
                let mut g = j.borrow_mut();
                if g.ativo {
                    return;
                }
                g.palavras.clear();
                g.digitado.clear();
                g.pontos = 0;
                g.vidas = 3;
                g.ticks = 0;
                g.ativo = true;
            }
            botao.set_sensitive(false);
            entrada.grab_focus();
            let (j, desenho, ap, banco, app, botao) = (j.clone(), desenho.clone(), ap.clone(), banco.clone(), app.clone(), botao.clone());
            glib::timeout_add_local(Duration::from_millis(33), move || {
                let mut g = j.borrow_mut();
                g.ticks += 1;
                if g.erro > 0 {
                    g.erro -= 1;
                }
                let nivel = 1.0 + g.pontos as f64 / 400.0;
                let intervalo = (75.0 / nivel).max(22.0) as u64;
                if g.ticks % intervalo == 1 || g.palavras.is_empty() {
                    let k = g.sorteio.ate(banco.len());
                    let x = g.sorteio.ate(1000) as f64 / 1000.0;
                    g.palavras.push(Caindo { texto: banco[k].clone(), x, y: 0.0 });
                }
                let vel = 0.0018 * nivel;
                for pal in g.palavras.iter_mut() {
                    pal.y += vel;
                }
                let antes = g.palavras.len();
                g.palavras.retain(|p| p.y < 1.0);
                let perdidas = (antes - g.palavras.len()) as u32;
                g.vidas = g.vidas.saturating_sub(perdidas);
                let fim = g.vidas == 0;
                if fim {
                    g.ativo = false;
                    let pts = g.pontos;
                    drop(g);
                    let mut pr = app.progresso.borrow_mut();
                    if pts > pr.recorde_jogo {
                        pr.recorde_jogo = pts;
                    }
                    pr.pontos += pts as u64 / 10;
                    pr.salvar();
                    drop(pr);
                    botao.set_sensitive(true);
                    ap();
                    desenho.queue_draw();
                    return glib::ControlFlow::Break;
                }
                drop(g);
                ap();
                desenho.queue_draw();
                glib::ControlFlow::Continue
            });
        });
    }
    let clique = gtk::GestureClick::new();
    {
        let e = entrada.clone();
        clique.connect_pressed(move |_, _, _, _| {
            e.grab_focus();
        });
    }
    desenho.add_controller(clique);
    p
}

fn pagina_progresso(app: &Rc<App>) -> gtk::Box {
    let p = pagina("Progresso", "");
    let pr = app.progresso.borrow();
    let (nivel, atual, prox) = pr.nivel();
    p.append(&rotulo(&format!("Nível {nivel}"), "numero-grande"));
    let barra = gtk::LevelBar::new();
    barra.set_min_value(0.0);
    barra.set_max_value(prox as f64);
    barra.set_value(atual as f64);
    barra.set_size_request(420, 14);
    barra.set_halign(gtk::Align::Start);
    p.append(&barra);
    p.append(&rotulo(&format!("{atual} de {prox} pontos para o nível {}  ·  {} pontos no total", nivel + 1, pr.pontos), "dim-label"));
    let feitas = pr.licoes.iter().filter(|r| r.estrelas > 0).count();
    p.append(&rotulo(
        &format!("Lições: {feitas} de {}  ·  Estrelas: {} de {}  ·  Recorde no jogo: {}", LICOES.len(), pr.estrelas_total(), LICOES.len() * 3, pr.recorde_jogo),
        "secao",
    ));
    p.append(&rotulo("Testes de velocidade", "secao"));
    if pr.testes.is_empty() {
        p.append(&rotulo("Nenhum teste ainda. Faça um na página “Teste de velocidade”.", "dim-label"));
    } else {
        // gráfico simples da evolução (PPM dos últimos testes)
        let dados: Vec<u32> = pr.testes.iter().map(|t| t.ppm).collect();
        let g = gtk::DrawingArea::new();
        g.set_content_height(180);
        g.set_content_width(560);
        g.set_halign(gtk::Align::Start);
        g.set_draw_func(move |_, cr, w, h| {
            let (w, h) = (w as f64, h as f64);
            let max = (*dados.iter().max().unwrap_or(&1)).max(10) as f64 * 1.15;
            cr.set_source_rgba(0.5, 0.5, 0.5, 0.25);
            for k in 0..4 {
                let y = h - 20.0 - (h - 30.0) * k as f64 / 3.0;
                cr.rectangle(30.0, y, w - 40.0, 1.0);
            }
            let _ = cr.fill();
            let n = dados.len().max(2) as f64 - 1.0;
            cr.set_source_rgb(0.18, 0.47, 0.71);
            cr.set_line_width(3.0);
            for (i, v) in dados.iter().enumerate() {
                let x = 30.0 + (w - 40.0) * i as f64 / n;
                let y = h - 20.0 - (h - 30.0) * *v as f64 / max;
                if i == 0 { cr.move_to(x, y) } else { cr.line_to(x, y) }
            }
            let _ = cr.stroke();
            for (i, v) in dados.iter().enumerate() {
                let x = 30.0 + (w - 40.0) * i as f64 / n;
                let y = h - 20.0 - (h - 30.0) * *v as f64 / max;
                cr.arc(x, y, 4.0, 0.0, 6.3);
                let _ = cr.fill();
            }
            cr.set_source_rgba(0.5, 0.5, 0.5, 0.9);
            cr.select_font_face("Noto Sans", cairo::FontSlant::Normal, cairo::FontWeight::Normal);
            cr.set_font_size(11.0);
            cr.move_to(0.0, 14.0);
            let _ = cr.show_text(&format!("{:.0} ppm", max / 1.15));
        });
        p.append(&g);
        for t in pr.testes.iter().rev().take(8) {
            p.append(&rotulo(&format!("{}  —  {} ppm, {}% de precisão", t.data, t.ppm, t.precisao), ""));
        }
    }
    p
}

fn pagina_teclado(app: &Rc<App>, ao_mudar: Rc<dyn Fn()>) -> gtk::Box {
    let p = pagina("Teclado", "As lições e o teclado na tela seguem o layout do seu teclado. O Doors detecta sozinho, mas você pode escolher outro, por exemplo para treinar para o teclado do trabalho.");
    let (det, achou) = app.detectado;
    let atual = app.progresso.borrow().layout.clone();
    let mut grupo: Option<gtk::CheckButton> = None;
    let desenho = gtk::DrawingArea::new();
    desenho.set_content_height(250);
    desenho.set_hexpand(true);
    let opcoes: Vec<(String, String)> = std::iter::once((
        String::new(),
        format!("Automático — {}{}", det.nome(), if achou { "" } else { " (não consegui detectar; padrão)" }),
    ))
    .chain(Layout::TODOS.iter().map(|l| (l.id().to_string(), l.nome().to_string())))
    .collect();
    for (id, nome) in opcoes {
        let b = gtk::CheckButton::with_label(&nome);
        if let Some(g) = grupo.as_ref() {
            b.set_group(Some(g));
        }
        b.set_active(atual == id);
        let (app, desenho, ao_mudar) = (app.clone(), desenho.clone(), ao_mudar.clone());
        b.connect_toggled(move |b| {
            if b.is_active() {
                app.progresso.borrow_mut().layout = id.clone();
                app.progresso.borrow().salvar();
                desenho.queue_draw();
                ao_mudar();
            }
        });
        p.append(&b);
        grupo.get_or_insert(b);
    }
    let dica = rotulo("", "dim-label");
    let atualizar_dica = {
        let (app, dica) = (app.clone(), dica.clone());
        move || {
            let l = app.layout();
            dica.set_text(&match l {
                Layout::Abnt2 => "No ABNT2 o ç tem tecla própria. Acentos: ´ ~ ^ ` antes da letra.".to_string(),
                Layout::UsIntl => "No internacional: ' + c = ç, ' + e = é, ~ + a = ã (Shift + `), ^ + o = ô (Shift + 6). Para um apóstrofo, ' e depois espaço.".to_string(),
                Layout::Us => "Sem teclas mortas: as lições usam palavras sem acento e sem ç.".to_string(),
            });
        }
    };
    atualizar_dica();
    {
        let app = app.clone();
        desenho.set_draw_func(move |_, cr, w, h| desenhar_teclado(cr, w as f64, h as f64, app.layout(), &[], e_escuro()));
    }
    let d2 = desenho.clone();
    let tick = gtk::GestureClick::new();
    tick.connect_released(move |_, _, _, _| {
        atualizar_dica();
        d2.queue_draw();
    });
    p.add_controller(tick);
    p.append(&dica);
    p.append(&desenho);
    p
}

// ------------------------------------------------------------------ janela

fn montar(gapp: &gtk::Application) {
    estilo();
    let app = Rc::new(App { progresso: RefCell::new(Progresso::carregar()), detectado: teclado::detectar(), janela: RefCell::new(None) });
    let janela = gtk::ApplicationWindow::builder()
        .application(gapp)
        .title("Doors Digitação")
        .icon_name("doors-logo")
        .default_width(1100)
        .default_height(760)
        .build();
    *app.janela.borrow_mut() = Some(janela.clone());

    let pilha = gtk::Stack::new();
    pilha.set_transition_type(gtk::StackTransitionType::Crossfade);
    pilha.set_hexpand(true);
    pilha.set_vexpand(true);

    // páginas que dependem do progresso/layout são refeitas ao serem abertas
    let fazer: Rc<dyn Fn(&str) -> gtk::Box> = {
        let app = app.clone();
        let pilha2 = pilha.clone();
        Rc::new(move |id: &str| -> gtk::Box {
            let refazer_tudo: Rc<dyn Fn()> = {
                let pilha3 = pilha2.clone();
                Rc::new(move || {
                    // muda o layout: refaz lições, teste e jogo na próxima visita
                    for id in ["licoes", "teste", "jogo"] {
                        if let Some(c) = pilha3.child_by_name(id) {
                            if let Some(r) = c.downcast_ref::<gtk::ScrolledWindow>() {
                                r.set_child(None::<&gtk::Widget>);
                            }
                        }
                    }
                })
            };
            match id {
                "licoes" => pagina_licoes(&app),
                "teste" => pagina_teste(&app),
                "jogo" => pagina_jogo(&app),
                "progresso" => pagina_progresso(&app),
                _ => pagina_teclado(&app, refazer_tudo),
            }
        })
    };
    for (id, nome) in [("licoes", "Lições"), ("teste", "Teste de velocidade"), ("jogo", "Jogo"), ("progresso", "Progresso"), ("teclado", "Teclado")] {
        let rolagem = gtk::ScrolledWindow::new();
        rolagem.set_hscrollbar_policy(gtk::PolicyType::Never);
        rolagem.set_child(Some(&fazer(id)));
        pilha.add_titled(&rolagem, Some(id), nome);
    }
    {
        let fazer = fazer.clone();
        pilha.connect_visible_child_name_notify(move |s| {
            let Some(nome) = s.visible_child_name() else { return };
            let Some(filho) = s.visible_child() else { return };
            let Some(r) = filho.downcast_ref::<gtk::ScrolledWindow>() else { return };
            // Progresso sempre atualizado; as outras só se foram limpas (troca de layout)
            if nome == "progresso" || r.child().is_none() {
                r.set_child(Some(&fazer(nome.as_str())));
            }
        });
    }

    let lateral = gtk::StackSidebar::new();
    lateral.set_stack(&pilha);
    lateral.set_size_request(200, -1);
    lateral.add_css_class("lateral");
    lateral.set_vexpand(true);
    let topo = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    topo.set_margin_top(16);
    topo.set_margin_bottom(8);
    topo.set_margin_start(16);
    topo.set_margin_end(16);
    let logo = gtk::Image::from_icon_name("doors-logo");
    logo.set_pixel_size(40);
    let tx = gtk::Box::new(gtk::Orientation::Vertical, 0);
    tx.append(&rotulo("Digitação", "secao"));
    tx.append(&rotulo("Doors OS", "dim-label"));
    topo.append(&logo);
    topo.append(&tx);
    let coluna = gtk::Box::new(gtk::Orientation::Vertical, 0);
    coluna.append(&topo);
    coluna.append(&lateral);

    let raiz = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    raiz.append(&coluna);
    raiz.append(&gtk::Separator::new(gtk::Orientation::Vertical));
    raiz.append(&pilha);
    janela.set_child(Some(&raiz));
    janela.present();
}
