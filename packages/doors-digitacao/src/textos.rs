//! Conteúdo em português: palavras para os exercícios e frases para os testes.
//! Frases escritas para o Doors OS.

/// Palavras comuns do português (minúsculas), das mais simples às com acento e ç.
pub const PALAVRAS: &[&str] = &[
    // só fileira do meio (a s d f g h j k l ç)
    "da", "das", "dada", "fada", "fadas", "gala", "sala", "salas", "falas", "fala", "alga", "algas",
    "asa", "asas", "saga", "lá", "já", "há", "faça", "laça", "ala", "alas", "falha", "falhas",
    "galha", "salsa",
    // + e i r u t o
    "se", "de", "do", "dos", "eu", "ela", "ele", "elas", "eles", "isso", "isto", "este", "esta", "ser",
    "ter", "ir", "seu", "sua", "rio", "rua", "tudo", "todo", "toda", "tio", "tia", "dia", "dias",
    "fio", "frio", "fruta", "festa", "teto", "terra", "tarde", "sorte", "forte", "outro", "outra",
    "agora", "ainda", "aqui", "assim", "triste", "feliz", "falar", "olhar", "sair", "jogar", "ler",
    "leite", "sol", "lua", "sal", "uso", "luta", "tela", "tecla", "lista", "artista", "estrada",
    "girafa", "garrafa", "dedo", "dedos", "rede", "sede", "festa", "letra", "letras", "regra",
    "lago", "jardim", "ideia", "idade", "hora", "horas", "lado", "logo", "lugar", "jogo", "jogos",
    // + w q y p
    "quase", "quero", "quer", "que", "quatro", "parte", "porta", "portas", "papel", "pedra", "pai",
    "pato", "prato", "porque", "quilo", "pipoca", "praia", "sapato", "tempo", "aquele", "aquela",
    // + c m v n b x z
    "casa", "carro", "cama", "comer", "como", "com", "cada", "cidade", "certo", "claro", "mesa",
    "mar", "mais", "mas", "mundo", "muito", "muita", "mesmo", "menos", "manter", "nada", "nome",
    "noite", "nunca", "novo", "nova", "nosso", "nossa", "vida", "vez", "ver", "verde", "vento",
    "viagem", "bom", "boa", "bola", "beber", "bem", "banco", "branco", "barco", "zero", "zona",
    "fazer", "dizer", "luz", "paz", "vez", "xadrez", "caixa", "peixe", "baixo", "lixo", "texto",
    "computador", "teclado", "monitor", "janela", "programa", "sistema", "internet", "arquivo",
    "pasta", "imagem", "escola", "livro", "caderno", "amigo", "amiga", "trabalho",
    "conta", "contar", "cantar", "dormir", "acordar", "cozinha", "banheiro", "quarto", "sala",
    // com acentos e ç
    "você", "também", "está", "então", "até", "só", "já", "pé", "café", "avó", "avô", "três",
    "mês", "você", "lâmpada", "música", "página", "número", "fácil", "difícil", "rápido", "árvore",
    "pássaro", "chão", "mão", "pão", "irmão", "coração", "canção", "lição", "ação", "nação",
    "atenção", "informação", "educação", "computação", "começo", "começar", "praça", "cabeça",
    "força", "dança", "criança", "esperança", "açúcar", "maçã", "açaí", "caça", "laço", "braço",
    "português", "inglês", "à", "às", "países", "saúde", "família", "história", "memória",
];

/// Frases para testes de velocidade e para a última lição.
pub const FRASES: &[&str] = &[
    "A porta azul se abriu e uma luz dourada tomou conta do jardim.",
    "Quem treina um pouco todo dia digita rápido sem olhar para o teclado.",
    "O gato dormiu na janela enquanto a chuva fina caía sobre a cidade.",
    "Cada lição concluída é um passo a mais na sua jornada.",
    "Mantenha os dedos na fileira do meio e use o polegar para o espaço.",
    "A biblioteca estava em silêncio, e os livros pareciam esperar por alguém.",
    "No verão a praia fica cheia, mas no inverno o mar é só do vento.",
    "Precisão vem antes da velocidade: primeiro acerte, depois acelere.",
    "O trem partiu às sete horas e chegou à serra antes do almoço.",
    "Minha avó faz o melhor pão de queijo da rua, quentinho e macio.",
    "Os pássaros cantam cedo, quando o sol ainda está nascendo atrás do morro.",
    "Escrever bem exige atenção: acentos, vírgulas e pontos fazem diferença.",
    "A criança desenhou uma casa, uma árvore e um cachorro de orelhas grandes.",
    "Depois da tempestade, o céu ficou limpo e cheio de estrelas.",
    "Organize suas pastas e salve os arquivos com nomes fáceis de lembrar.",
    "O time treinou a semana toda e venceu o jogo no último minuto.",
    "Uma xícara de café, um bom livro e a tarde passa sem pressa.",
    "Na feira havia maçã, banana, laranja, abacaxi e muito açaí.",
    "Quem chega primeiro abre a porta e acende as luzes da sala.",
    "Digitar sem olhar para as teclas é como andar de bicicleta: nunca se esquece.",
    "O barco atravessou o rio devagar, levando três famílias para a outra margem.",
    "Ela guardou a carta numa caixa de madeira, junto com fotos antigas.",
    "Às vezes a melhor ideia aparece quando estamos caminhando sem destino.",
    "O computador ligou, a tela acendeu e a aula de digitação começou.",
    "Respire fundo, relaxe os ombros e deixe os dedos encontrarem o caminho.",
    "Na escola, aprendemos que ler todos os dias abre portas para o mundo.",
];

#[cfg(test)]
mod testes {
    use super::*;
    #[test]
    fn conteudo_basico() {
        assert!(PALAVRAS.len() > 200);
        assert!(FRASES.len() >= 20);
        assert!(PALAVRAS.iter().all(|p| p.chars().all(|c| c.is_lowercase() || !c.is_alphabetic())));
    }
}
