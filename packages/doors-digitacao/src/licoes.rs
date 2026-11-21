//! Lições em sequência e geração dos textos de cada uma (adaptados ao layout).

use crate::teclado::{adaptar, digitavel, Layout};
use crate::textos::{FRASES, PALAVRAS};

/// Uma lição: teclas novas e o conjunto de caracteres liberados até ela.
pub struct Licao {
    pub titulo: &'static str,
    pub descricao: &'static str,
    /// teclas apresentadas nesta lição (no ABNT2; adaptadas para outros layouts)
    pub novas: &'static str,
    /// meta de velocidade (palavras por minuto) para 3 estrelas
    pub meta_ppm: u32,
    pub tipo: Tipo,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tipo {
    /// treino de teclas + palavras só com as letras liberadas
    Teclas,
    /// palavras com maiúscula inicial
    Maiusculas,
    /// palavras com acentos (e ç)
    Acentos,
    /// números
    Numeros,
    /// frases completas
    Frases,
}

pub const LICOES: &[Licao] = &[
    Licao { titulo: "F e J", descricao: "Os indicadores nas teclas com relevo. É daqui que tudo começa.", novas: "fj", meta_ppm: 10, tipo: Tipo::Teclas },
    Licao { titulo: "D e K", descricao: "Dedos médios na fileira do meio.", novas: "dk", meta_ppm: 10, tipo: Tipo::Teclas },
    Licao { titulo: "S e L", descricao: "Agora os anelares.", novas: "sl", meta_ppm: 11, tipo: Tipo::Teclas },
    Licao { titulo: "A e Ç", descricao: "Os mínimos completam a fileira do meio.", novas: "aç", meta_ppm: 11, tipo: Tipo::Teclas },
    Licao { titulo: "G e H", descricao: "Os indicadores se esticam para o centro.", novas: "gh", meta_ppm: 12, tipo: Tipo::Teclas },
    Licao { titulo: "E e I", descricao: "Primeira subida para a fileira de cima.", novas: "ei", meta_ppm: 12, tipo: Tipo::Teclas },
    Licao { titulo: "R e U", descricao: "Indicadores na fileira de cima.", novas: "ru", meta_ppm: 13, tipo: Tipo::Teclas },
    Licao { titulo: "T e O", descricao: "Mais duas teclas muito usadas no português.", novas: "to", meta_ppm: 13, tipo: Tipo::Teclas },
    Licao { titulo: "Q, W, Y e P", descricao: "Completa a fileira de cima.", novas: "qwyp", meta_ppm: 14, tipo: Tipo::Teclas },
    Licao { titulo: "C e M", descricao: "Descendo para a fileira de baixo.", novas: "cm", meta_ppm: 14, tipo: Tipo::Teclas },
    Licao { titulo: "V e N", descricao: "Indicadores na fileira de baixo.", novas: "vn", meta_ppm: 15, tipo: Tipo::Teclas },
    Licao { titulo: "B, X e Z", descricao: "As últimas letras.", novas: "bxz", meta_ppm: 15, tipo: Tipo::Teclas },
    Licao { titulo: "Vírgula e ponto", descricao: "Pontuação básica com médio e anelar direitos.", novas: ",.", meta_ppm: 16, tipo: Tipo::Teclas },
    Licao { titulo: "Maiúsculas", descricao: "Shift com o mínimo da mão oposta à da letra.", novas: "", meta_ppm: 16, tipo: Tipo::Maiusculas },
    Licao { titulo: "Acentos", descricao: "Teclas mortas: primeiro o acento, depois a letra.", novas: "áéíóúãõâêô", meta_ppm: 16, tipo: Tipo::Acentos },
    Licao { titulo: "Ç e -ção", descricao: "Palavras com cedilha, tão comuns no português.", novas: "ç", meta_ppm: 17, tipo: Tipo::Acentos },
    Licao { titulo: "Números", descricao: "A fileira dos números, sem tirar os dedos do lugar.", novas: "0123456789", meta_ppm: 14, tipo: Tipo::Numeros },
    Licao { titulo: "Frases", descricao: "Textos completos, com tudo o que você aprendeu.", novas: "", meta_ppm: 20, tipo: Tipo::Frases },
];

/// Gerador de números pseudoaleatórios simples (determinístico por semente).
pub struct Sorteio(u64);

impl Sorteio {
    pub fn novo(semente: u64) -> Sorteio {
        Sorteio(semente.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407) | 1)
    }
    pub fn proximo(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    pub fn ate(&mut self, n: usize) -> usize {
        if n == 0 { 0 } else { (self.proximo() % n as u64) as usize }
    }
}

/// Letras (minúsculas) liberadas até a lição `i`, no layout dado.
/// No inglês, o "ç" da 4ª lição vira ";" (a tecla que fica no lugar dele).
pub fn liberadas(i: usize, l: Layout) -> Vec<char> {
    let mut v: Vec<char> = Vec::new();
    for li in LICOES.iter().take(i + 1) {
        if li.tipo == Tipo::Teclas {
            for c in li.novas.chars() {
                let c = if c == 'ç' && l != Layout::Abnt2 { ';' } else { c };
                if !v.contains(&c) {
                    v.push(c);
                }
            }
        }
    }
    v
}

/// Teclas novas da lição, já adaptadas ao layout.
pub fn novas(i: usize, l: Layout) -> String {
    let li = &LICOES[i];
    let s: String = li.novas.chars().map(|c| if c == 'ç' && li.tipo == Tipo::Teclas && l != Layout::Abnt2 { ';' } else { c }).collect();
    adaptar(l, &s)
}

fn palavras_com(permitidas: &[char], l: Layout) -> Vec<String> {
    let mut v: Vec<String> = PALAVRAS
        .iter()
        .filter(|p| p.chars().all(|c| permitidas.contains(&c)))
        .map(|p| adaptar(l, p))
        .filter(|p| !p.is_empty())
        .collect();
    v.sort();
    v.dedup();
    v
}

fn capitalizar(p: &str) -> String {
    let mut c = p.chars();
    match c.next() {
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        None => String::new(),
    }
}

/// Texto do exercício da lição `i` (cerca de `tamanho` caracteres).
pub fn texto(i: usize, l: Layout, semente: u64, tamanho: usize) -> String {
    let li = &LICOES[i];
    let mut r = Sorteio::novo(semente ^ (i as u64 * 7919));
    let mut partes: Vec<String> = Vec::new();
    let total = |p: &Vec<String>| p.iter().map(|s| s.chars().count() + 1).sum::<usize>();
    match li.tipo {
        Tipo::Teclas => {
            let lib = liberadas(i, l);
            let nov: Vec<char> = novas(i, l).chars().collect();
            // 1) treino das teclas novas (ex. "fff jjj fj jf")
            for c in &nov {
                partes.push(c.to_string().repeat(3));
            }
            for _ in 0..4 {
                let s: String = (0..2 + r.ate(3)).map(|_| nov[r.ate(nov.len())]).collect();
                partes.push(s);
            }
            // 2) palavras com as letras liberadas (ou combinações, se ainda houver poucas)
            let ws = palavras_com(&lib, l);
            while total(&partes) < tamanho {
                if ws.len() >= 4 && r.ate(3) > 0 {
                    partes.push(ws[r.ate(ws.len())].clone());
                } else {
                    let n = 2 + r.ate(4);
                    let s: String = (0..n)
                        .map(|k| if k == 0 && r.ate(2) == 0 { nov[r.ate(nov.len())] } else { lib[r.ate(lib.len())] })
                        .collect();
                    partes.push(s);
                }
            }
        }
        Tipo::Maiusculas => {
            let ws = palavras_com(&liberadas(i, l).iter().copied().chain(['ç']).collect::<Vec<_>>(), l);
            while total(&partes) < tamanho {
                let w = &ws[r.ate(ws.len())];
                partes.push(if r.ate(2) == 0 { capitalizar(w) } else { w.clone() });
            }
        }
        Tipo::Acentos => {
            let marcas: Vec<char> = li.novas.chars().collect();
            let ws: Vec<String> = PALAVRAS
                .iter()
                .filter(|p| p.chars().any(|c| marcas.contains(&c)))
                .map(|p| adaptar(l, p))
                .collect();
            let ws = if ws.is_empty() { palavras_com(&liberadas(i, l), l) } else { ws };
            while total(&partes) < tamanho {
                partes.push(ws[r.ate(ws.len())].clone());
            }
        }
        Tipo::Numeros => {
            let ws = palavras_com(&liberadas(i, l), l);
            while total(&partes) < tamanho {
                if r.ate(2) == 0 {
                    let n = 1 + r.ate(4);
                    partes.push((0..n).map(|_| char::from(b'0' + r.ate(10) as u8)).collect());
                } else {
                    partes.push(ws[r.ate(ws.len())].clone());
                }
            }
        }
        Tipo::Frases => return frases(l, semente, tamanho),
    }
    let mut s = partes.join(" ");
    // vírgula e ponto: espalha nas palavras a partir da lição deles
    let lib = liberadas(i, l);
    if li.tipo == Tipo::Teclas && lib.contains(&',') {
        let mut palavras: Vec<String> = s.split(' ').map(str::to_string).collect();
        let n = palavras.len();
        for k in (3..n).step_by(4) {
            palavras[k].push(if r.ate(2) == 0 { ',' } else { '.' });
        }
        s = palavras.join(" ");
    }
    s.chars().filter(|c| digitavel(l, *c)).collect::<String>().trim().to_string()
}

/// Frases sorteadas (testes de velocidade e última lição).
pub fn frases(l: Layout, semente: u64, tamanho: usize) -> String {
    let mut r = Sorteio::novo(semente);
    let mut v: Vec<String> = Vec::new();
    let mut usadas = Vec::new();
    while v.iter().map(|s| s.chars().count() + 1).sum::<usize>() < tamanho && usadas.len() < FRASES.len() {
        let k = r.ate(FRASES.len());
        if usadas.contains(&k) {
            continue;
        }
        usadas.push(k);
        v.push(adaptar(l, FRASES[k]));
    }
    v.join(" ")
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn primeiras_licoes_so_com_letras_liberadas() {
        for l in Layout::TODOS {
            let t = texto(0, l, 1, 80);
            assert!(t.chars().all(|c| "fj ".contains(c)), "{t}");
            let t = texto(3, l, 2, 80);
            let ok = if l == Layout::Abnt2 { "fjdkslaç " } else { "fjdksla; " };
            assert!(t.chars().all(|c| ok.contains(c)), "{l:?}: {t}");
            assert!(t.chars().count() >= 60);
        }
    }

    #[test]
    fn todos_os_textos_sao_digitaveis() {
        for l in Layout::TODOS {
            for i in 0..LICOES.len() {
                let t = texto(i, l, 42, 120);
                assert!(!t.is_empty(), "lição {i} vazia em {l:?}");
                assert!(t.chars().all(|c| digitavel(l, c)), "lição {i} em {l:?}: {t}");
            }
        }
    }

    #[test]
    fn acentos_no_abnt2_e_sem_acentos_no_us() {
        let i = LICOES.iter().position(|x| x.titulo == "Acentos").unwrap();
        assert!(texto(i, Layout::Abnt2, 3, 120).chars().any(|c| "áéíóúãõâêô".contains(c)));
        assert!(!texto(i, Layout::Us, 3, 120).chars().any(|c| "áéíóúãõâêôç".contains(c)));
    }

    #[test]
    fn frases_variam_com_a_semente() {
        assert_ne!(frases(Layout::Abnt2, 1, 200), frases(Layout::Abnt2, 2, 200));
    }
}
