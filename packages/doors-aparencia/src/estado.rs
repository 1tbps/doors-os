//! Escolhas do usuário, guardadas em ~/.config/doors/aparencia (chave=valor).

use crate::tema::{cor_por_id, Cor, Modo, Painel, CORES};
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq)]
pub struct Estado {
    pub modo: Modo,
    pub cor: Cor,
    pub painel: Painel,
    pub papel: Option<String>,
    /// tamanho do texto em porcentagem (100 = normal)
    pub texto: u32,
    /// alterna entre A e B: o KDE e o GTK só recarregam o tema quando o nome muda
    pub geracao: u32,
}

impl Default for Estado {
    fn default() -> Self {
        Estado { modo: Modo::Claro, cor: CORES[0], painel: Painel::Embaixo, papel: None, texto: 100, geracao: 0 }
    }
}

impl Estado {
    pub fn de_texto(texto: &str) -> Estado {
        let mut e = Estado::default();
        for linha in texto.lines() {
            let Some((k, v)) = linha.split_once('=') else { continue };
            let v = v.trim();
            match k.trim() {
                "modo" => e.modo = Modo::de_id(v).unwrap_or(e.modo),
                "cor" => e.cor = cor_por_id(v).unwrap_or(e.cor),
                "painel" => e.painel = Painel::de_id(v).unwrap_or(e.painel),
                "papel" if !v.is_empty() => e.papel = Some(v.to_string()),
                "texto" => e.texto = v.parse().ok().filter(|t| (50..=200).contains(t)).unwrap_or(100),
                "geracao" => e.geracao = v.parse().unwrap_or(0),
                _ => {}
            }
        }
        e
    }

    pub fn para_texto(&self) -> String {
        format!(
            "# Doors Aparência\nmodo={}\ncor={}\npainel={}\npapel={}\ntexto={}\ngeracao={}\n",
            self.modo.id(),
            self.cor.id,
            self.painel.id(),
            self.papel.as_deref().unwrap_or(""),
            self.texto,
            self.geracao
        )
    }

    /// nome do tema pessoal desta geração ("DoorsPessoalA"/"DoorsPessoalB")
    pub fn nome_tema(&self) -> String {
        format!("DoorsPessoal{}", if self.geracao % 2 == 0 { "A" } else { "B" })
    }

    fn arquivo() -> Option<PathBuf> {
        let base = std::env::var_os("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".config")))?;
        Some(base.join("doors").join("aparencia"))
    }

    pub fn carregar() -> Estado {
        Self::arquivo()
            .and_then(|f| std::fs::read_to_string(f).ok())
            .map(|t| Estado::de_texto(&t))
            .unwrap_or_default()
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
        let mut e = Estado::default();
        e.modo = Modo::Escuro;
        e.cor = cor_por_id("verde").unwrap();
        e.painel = Painel::EmCima;
        e.papel = Some("/usr/share/backgrounds/doors/colina.jpg".into());
        e.texto = 115;
        e.geracao = 3;
        assert_eq!(Estado::de_texto(&e.para_texto()), e);
        assert_eq!(e.nome_tema(), "DoorsPessoalB");
    }

    #[test]
    fn texto_invalido_vira_padrao() {
        let e = Estado::de_texto("modo=xyz\ncor=rosa\nlixo\npapel=\n");
        assert_eq!(e, Estado::default());
    }
}
