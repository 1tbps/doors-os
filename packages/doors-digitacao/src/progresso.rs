//! Progresso do aluno, guardado em ~/.local/share/doors-digitacao/progresso (chave=valor).

use std::path::PathBuf;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Resultado {
    pub estrelas: u8,
    pub ppm: u32,
    pub precisao: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Teste {
    pub data: String,
    pub ppm: u32,
    pub precisao: u32,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Progresso {
    /// melhor resultado de cada lição (índice da lição)
    pub licoes: Vec<Resultado>,
    pub pontos: u64,
    pub testes: Vec<Teste>,
    pub recorde_jogo: u32,
    /// layout escolhido à mão ("" = automático)
    pub layout: String,
    /// como o texto se move: "fixa" (linha atual sempre no meio) ou "rolagem"
    pub texto_modo: String,
    /// "parar" (só avança com o caractere certo) ou "seguir" (marca o erro e avança)
    pub erro_modo: String,
}

impl Progresso {
    pub fn de_texto(t: &str) -> Progresso {
        let mut p = Progresso::default();
        for linha in t.lines() {
            let Some((k, v)) = linha.split_once('=') else { continue };
            let nums: Vec<u32> = v.split(',').filter_map(|n| n.trim().parse().ok()).collect();
            if let Some(i) = k.strip_prefix("licao.").and_then(|n| n.parse::<usize>().ok()) {
                if i < 200 && nums.len() == 3 {
                    if p.licoes.len() <= i {
                        p.licoes.resize(i + 1, Resultado::default());
                    }
                    p.licoes[i] = Resultado { estrelas: nums[0].min(3) as u8, ppm: nums[1], precisao: nums[2] };
                }
                continue;
            }
            match k {
                "pontos" => p.pontos = v.trim().parse().unwrap_or(0),
                "jogo" => p.recorde_jogo = v.trim().parse().unwrap_or(0),
                "layout" => p.layout = v.trim().to_string(),
                "texto" => p.texto_modo = v.trim().to_string(),
                "erro" => p.erro_modo = v.trim().to_string(),
                "teste" => {
                    let partes: Vec<&str> = v.split(',').collect();
                    if partes.len() == 3 {
                        if let (Ok(a), Ok(b)) = (partes[1].trim().parse(), partes[2].trim().parse()) {
                            p.testes.push(Teste { data: partes[0].trim().to_string(), ppm: a, precisao: b });
                        }
                    }
                }
                _ => {}
            }
        }
        p
    }

    pub fn para_texto(&self) -> String {
        let mut s = String::from("# Doors Digitação\n");
        s.push_str(&format!(
            "pontos={}\njogo={}\nlayout={}\ntexto={}\nerro={}\n",
            self.pontos, self.recorde_jogo, self.layout, self.texto_modo, self.erro_modo
        ));
        for (i, r) in self.licoes.iter().enumerate() {
            if r.estrelas > 0 || r.ppm > 0 {
                s.push_str(&format!("licao.{i}={},{},{}\n", r.estrelas, r.ppm, r.precisao));
            }
        }
        for t in &self.testes {
            s.push_str(&format!("teste={},{},{}\n", t.data.replace(',', " "), t.ppm, t.precisao));
        }
        s
    }

    pub fn resultado(&self, i: usize) -> Resultado {
        self.licoes.get(i).cloned().unwrap_or_default()
    }

    /// Guarda o resultado se for melhor (mais estrelas, ou mesmas estrelas e mais PPM).
    /// Devolve os pontos ganhos.
    pub fn registrar_licao(&mut self, i: usize, r: Resultado, caracteres: usize) -> u64 {
        if self.licoes.len() <= i {
            self.licoes.resize(i + 1, Resultado::default());
        }
        let atual = &self.licoes[i];
        if (r.estrelas, r.ppm) > (atual.estrelas, atual.ppm) {
            self.licoes[i] = r.clone();
        }
        let ganho = caracteres as u64 * (1 + r.estrelas as u64);
        self.pontos += ganho;
        ganho
    }

    pub fn registrar_teste(&mut self, t: Teste, caracteres: usize) -> u64 {
        self.testes.push(t);
        let n = self.testes.len();
        if n > 30 {
            self.testes.drain(0..n - 30);
        }
        let ganho = caracteres as u64 * 2;
        self.pontos += ganho;
        ganho
    }

    /// Lição liberada: a primeira sempre; as outras depois de ao menos 1 estrela na anterior.
    pub fn liberada(&self, i: usize) -> bool {
        i == 0 || self.resultado(i - 1).estrelas > 0
    }

    pub fn estrelas_total(&self) -> u32 {
        self.licoes.iter().map(|r| r.estrelas as u32).sum()
    }

    /// Nível: cresce cada vez mais devagar (100, 300, 600, 1000... pontos).
    pub fn nivel(&self) -> (u32, u64, u64) {
        let mut nivel = 1u32;
        let mut base = 0u64;
        loop {
            let prox = base + 100 * nivel as u64;
            if self.pontos < prox {
                return (nivel, self.pontos - base, prox - base);
            }
            base = prox;
            nivel += 1;
        }
    }

    pub fn melhor_ppm_teste(&self) -> u32 {
        self.testes.iter().map(|t| t.ppm).max().unwrap_or(0)
    }

    fn arquivo() -> Option<PathBuf> {
        let base = std::env::var_os("XDG_DATA_HOME")
            .map(PathBuf::from)
            .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".local/share")))?;
        Some(base.join("doors-digitacao").join("progresso"))
    }

    pub fn carregar() -> Progresso {
        Self::arquivo().and_then(|f| std::fs::read_to_string(f).ok()).map(|t| Progresso::de_texto(&t)).unwrap_or_default()
    }

    pub fn salvar(&self) {
        if let Some(f) = Self::arquivo() {
            if let Some(d) = f.parent() {
                let _ = std::fs::create_dir_all(d);
            }
            let _ = std::fs::write(f, self.para_texto());
        }
    }
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn ida_e_volta() {
        let mut p = Progresso::default();
        p.registrar_licao(2, Resultado { estrelas: 2, ppm: 18, precisao: 96 }, 100);
        p.registrar_teste(Teste { data: "21/11/2026".into(), ppm: 30, precisao: 97 }, 150);
        p.recorde_jogo = 420;
        p.layout = "us-intl".into();
        p.texto_modo = "rolagem".into();
        p.erro_modo = "seguir".into();
        let q = Progresso::de_texto(&p.para_texto());
        assert_eq!(q.resultado(2), Resultado { estrelas: 2, ppm: 18, precisao: 96 });
        assert_eq!(q.testes.len(), 1);
        assert_eq!(q.recorde_jogo, 420);
        assert_eq!(q.layout, "us-intl");
        assert_eq!((q.texto_modo.as_str(), q.erro_modo.as_str()), ("rolagem", "seguir"));
        assert_eq!(q.pontos, 300 + 300);
    }

    #[test]
    fn guarda_so_o_melhor_e_libera() {
        let mut p = Progresso::default();
        assert!(p.liberada(0));
        assert!(!p.liberada(1));
        p.registrar_licao(0, Resultado { estrelas: 3, ppm: 20, precisao: 99 }, 10);
        p.registrar_licao(0, Resultado { estrelas: 1, ppm: 40, precisao: 91 }, 10);
        assert_eq!(p.resultado(0).estrelas, 3);
        assert!(p.liberada(1));
    }

    #[test]
    fn niveis() {
        let mut p = Progresso::default();
        assert_eq!(p.nivel(), (1, 0, 100));
        p.pontos = 150;
        assert_eq!(p.nivel(), (2, 50, 200));
        p.pontos = 300;
        assert_eq!(p.nivel().0, 3);
    }
}
