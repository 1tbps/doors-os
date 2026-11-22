//! Sessão de digitação: posição, erros, velocidade (PPM) e precisão. Sem interface (testável).

use std::time::{Duration, Instant};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Resultado {
    Certo,
    Errado,
    Fim,
}

pub struct Sessao {
    pub alvo: Vec<char>,
    pub pos: usize,
    pub erros: u32,
    pub teclas: u32,
    /// caracteres em que houve erro (para pintar de vermelho)
    pub errou: Vec<bool>,
    inicio: Option<Instant>,
    fim: Option<Instant>,
    /// limite de tempo (testes de velocidade)
    pub limite: Option<Duration>,
    /// true: um erro marca o caractere e passa para o próximo; false: para até acertar
    pub seguir_no_erro: bool,
    /// a última tecla foi um erro (para destacar o caractere atual)
    pub errou_agora: bool,
}

impl Sessao {
    pub fn nova(texto: &str, limite: Option<Duration>) -> Sessao {
        let alvo: Vec<char> = texto.chars().collect();
        let n = alvo.len();
        Sessao { alvo, pos: 0, erros: 0, teclas: 0, errou: vec![false; n], inicio: None, fim: None, limite, seguir_no_erro: false, errou_agora: false }
    }

    pub fn terminou(&self) -> bool {
        self.fim.is_some()
    }

    pub fn comecou(&self) -> bool {
        self.inicio.is_some()
    }

    /// Próximo caractere esperado.
    pub fn esperado(&self) -> Option<char> {
        self.alvo.get(self.pos).copied()
    }

    /// Parar no erro: o cursor só avança com o caractere certo. Seguir no erro: o erro fica
    /// marcado e o cursor avança. Nos dois casos cada erro conta na precisão.
    pub fn digitar(&mut self, c: char) -> Resultado {
        if self.terminou() {
            return Resultado::Fim;
        }
        let agora = Instant::now();
        self.inicio.get_or_insert(agora);
        if self.tempo_esgotado(agora) {
            self.fim = Some(agora);
            return Resultado::Fim;
        }
        self.teclas += 1;
        if Some(c) == self.esperado() {
            self.errou_agora = false;
            self.pos += 1;
            if self.pos >= self.alvo.len() {
                self.fim = Some(agora);
                return Resultado::Fim;
            }
            Resultado::Certo
        } else {
            self.erros += 1;
            if let Some(e) = self.errou.get_mut(self.pos) {
                *e = true;
            }
            self.errou_agora = true;
            if self.seguir_no_erro {
                self.pos += 1;
                if self.pos >= self.alvo.len() {
                    self.fim = Some(agora);
                    return Resultado::Fim;
                }
            }
            Resultado::Errado
        }
    }

    fn tempo_esgotado(&self, agora: Instant) -> bool {
        match (self.limite, self.inicio) {
            (Some(l), Some(i)) => agora.duration_since(i) >= l,
            _ => false,
        }
    }

    /// Verifica o limite de tempo (chamado pelo relógio da tela). Devolve true se acabou agora.
    pub fn verificar_tempo(&mut self) -> bool {
        let agora = Instant::now();
        if !self.terminou() && self.tempo_esgotado(agora) {
            self.fim = Some(agora);
            return true;
        }
        false
    }

    pub fn segundos(&self) -> f64 {
        match self.inicio {
            None => 0.0,
            Some(i) => {
                let f = self.fim.unwrap_or_else(Instant::now);
                let d = f.duration_since(i).as_secs_f64();
                match self.limite {
                    Some(l) => d.min(l.as_secs_f64()),
                    None => d,
                }
            }
        }
    }

    /// Palavras por minuto (5 caracteres certos = 1 palavra).
    pub fn ppm(&self) -> f64 {
        calcular_ppm(self.pos, self.segundos())
    }

    /// Precisão em % (acertos / teclas digitadas).
    pub fn precisao(&self) -> f64 {
        calcular_precisao(self.pos as u32, self.teclas)
    }

    pub fn restante(&self) -> Option<f64> {
        self.limite.map(|l| (l.as_secs_f64() - self.segundos()).max(0.0))
    }
}

/// Quebra o texto em linhas de até `largura` caracteres sem cortar palavras: devolve
/// (início, fim) de cada linha; o espaço que separa fica no fim da linha.
pub fn quebrar(alvo: &[char], largura: usize) -> Vec<(usize, usize)> {
    let mut linhas = Vec::new();
    let mut ini = 0;
    while ini < alvo.len() {
        let mut fim = (ini + largura).min(alvo.len());
        if fim < alvo.len() {
            // volta até o último espaço da linha (se houver)
            if let Some(k) = (ini..fim).rev().find(|&k| alvo[k] == ' ') {
                fim = k + 1;
            }
        }
        linhas.push((ini, fim));
        ini = fim;
    }
    linhas
}

/// Linha onde está a posição `pos` (a última, se estiver no fim).
pub fn linha_de(linhas: &[(usize, usize)], pos: usize) -> usize {
    linhas.iter().position(|&(a, b)| pos >= a && pos < b).unwrap_or(linhas.len().saturating_sub(1))
}

pub fn calcular_ppm(certos: usize, segundos: f64) -> f64 {
    if segundos < 1.0 { 0.0 } else { certos as f64 / 5.0 / (segundos / 60.0) }
}

pub fn calcular_precisao(certos: u32, teclas: u32) -> f64 {
    if teclas == 0 { 100.0 } else { (certos as f64 * 100.0 / teclas as f64).min(100.0) }
}

/// Estrelas de uma lição: precisão primeiro, depois a velocidade.
pub fn estrelas(ppm: f64, precisao: f64, meta_ppm: u32) -> u8 {
    let meta = meta_ppm as f64;
    if precisao < 90.0 {
        0
    } else if precisao >= 97.0 && ppm >= meta {
        3
    } else if precisao >= 94.0 && ppm >= meta * 0.7 {
        2
    } else {
        1
    }
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn avanca_so_com_acerto() {
        let mut s = Sessao::nova("ab", None);
        assert_eq!(s.digitar('x'), Resultado::Errado);
        assert_eq!(s.pos, 0);
        assert!(s.errou[0]);
        assert_eq!(s.digitar('a'), Resultado::Certo);
        assert_eq!(s.digitar('b'), Resultado::Fim);
        assert!(s.terminou());
        assert_eq!(s.erros, 1);
        assert_eq!(s.teclas, 3);
        assert!((s.precisao() - 66.66).abs() < 0.1);
        assert_eq!(s.digitar('c'), Resultado::Fim);
    }

    #[test]
    fn seguir_no_erro() {
        let mut s = Sessao::nova("abc", None);
        s.seguir_no_erro = true;
        assert_eq!(s.digitar('x'), Resultado::Errado);
        assert_eq!(s.pos, 1);
        assert!(s.errou[0] && s.errou_agora);
        assert_eq!(s.digitar('b'), Resultado::Certo);
        assert!(!s.errou_agora);
        assert_eq!(s.digitar('z'), Resultado::Fim);
        assert_eq!(s.erros, 2);
    }

    #[test]
    fn linhas_sem_cortar_palavras() {
        let t: Vec<char> = "abc def ghi jkl".chars().collect();
        let l = quebrar(&t, 8);
        assert_eq!(l, vec![(0, 8), (8, 15)]);
        assert_eq!(linha_de(&l, 9), 1);
        assert_eq!(linha_de(&l, 15), 1);
        let longo: Vec<char> = "abcdefghij".chars().collect();
        assert_eq!(quebrar(&longo, 4), vec![(0, 4), (4, 8), (8, 10)]);
    }

    #[test]
    fn contas() {
        assert_eq!(calcular_ppm(100, 60.0), 20.0);
        assert_eq!(calcular_ppm(10, 0.5), 0.0);
        assert_eq!(calcular_precisao(0, 0), 100.0);
        assert_eq!(estrelas(30.0, 99.0, 20), 3);
        assert_eq!(estrelas(15.0, 95.0, 20), 2);
        assert_eq!(estrelas(5.0, 92.0, 20), 1);
        assert_eq!(estrelas(50.0, 80.0, 20), 0);
    }

    #[test]
    fn caracteres_compostos() {
        let mut s = Sessao::nova("ação", None);
        for c in "ação".chars() {
            s.digitar(c);
        }
        assert!(s.terminou());
        assert_eq!(s.erros, 0);
    }
}
