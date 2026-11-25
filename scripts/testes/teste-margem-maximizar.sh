#!/bin/bash
# Doors OS (KDE Plasma 6): maximizar com margem, como o "Preencher" do macOS.
#   bash teste-margem-maximizar.sh ligar [margem] -> liga (margem em px, padrao 12)
#   bash teste-margem-maximizar.sh desligar       -> volta ao maximizar normal
# Instala so para o seu usuario (~/.local/share/kwin/scripts); nao precisa de sudo.
set -e
ID=doors-margem-maximizar
DIR=$(mktemp -d)
trap 'rm -rf "$DIR"' EXIT

recarregar() {
  gdbus call --session --dest org.kde.KWin --object-path /KWin \
    --method org.kde.KWin.reconfigure >/dev/null
}

# Tira o script da memoria do KWin na hora (o reconfigure sozinho nao basta:
# ele roda depois e, com o pacote ja apagado, nao descarrega o script antigo).
descarregar() {
  gdbus call --session --dest org.kde.KWin --object-path /Scripting \
    --method org.kde.kwin.Scripting.unloadScript "$ID" >/dev/null 2>&1 || true
}

criar_pacote() {
  mkdir -p "$DIR/$ID/contents/code"
  cat > "$DIR/$ID/metadata.json" <<'JSON'
{
    "KPlugin": {
        "Id": "doors-margem-maximizar",
        "Name": "Doors: maximizar com margem",
        "Description": "Ao maximizar, a janela ocupa a tela deixando uma pequena margem em volta (como o Preencher do macOS). Maximizar de novo devolve o tamanho anterior.",
        "Version": "1.0",
        "License": "GPL-3.0-or-later",
        "Authors": [ { "Name": "Doors OS" } ]
    },
    "X-Plasma-API": "javascript",
    "X-Plasma-MainScript": "code/main.js",
    "KPackageStructure": "KWin/Script"
}
JSON
  cat > "$DIR/$ID/contents/code/main.js" <<'JS'
// Doors OS: maximizar com margem (estilo "Preencher" do macOS).
// Quando a janela e maximizada, ela sai do estado maximizado e passa a ocupar
// a area da tela menos uma margem em volta. Maximizar de novo (botao, clique
// duplo no titulo, Meta+PgUp) devolve a janela ao tamanho que tinha antes.
const MARGEM = Math.max(0, Number(readConfig("Margem", 12)) || 0);
const TOLERANCIA = 32;           // px: janela "ainda preenchida" se perto disso
const estado = {};               // internalId -> { antes, preenchida }
let ocupado = false;             // evita reagir as nossas proprias mudancas

function chave(w) { return String(w.internalId); }

function elegivel(w) {
    return w && w.normalWindow && !w.specialWindow && !w.fullScreen
        && w.resizeable && w.moveable && !w.transient;
}

function copia(r) { return { x: r.x, y: r.y, width: r.width, height: r.height }; }

function perto(a, b) {
    return a && b
        && Math.abs(a.x - b.x) <= TOLERANCIA
        && Math.abs(a.y - b.y) <= TOLERANCIA
        && Math.abs(a.x + a.width - (b.x + b.width)) <= TOLERANCIA
        && Math.abs(a.y + a.height - (b.y + b.height)) <= TOLERANCIA;
}

function areaPreenchida(w) {
    const a = workspace.clientArea(KWin.MaximizeArea, w);
    return { x: a.x + MARGEM, y: a.y + MARGEM,
             width: a.width - 2 * MARGEM, height: a.height - 2 * MARGEM };
}

function areaPadrao(w) {
    // Sem tamanho anterior conhecido (janela que ja abriu maximizada): 75% centrado.
    const a = workspace.clientArea(KWin.MaximizeArea, w);
    const lw = Math.round(a.width * 0.75), lh = Math.round(a.height * 0.75);
    return { x: a.x + Math.round((a.width - lw) / 2), y: a.y + Math.round((a.height - lh) / 2),
             width: lw, height: lh };
}

function aplicar(w, r) {
    // KWin converte um objeto {x, y, width, height} em retangulo (nao existe Qt.rect aqui).
    w.frameGeometry = { x: r.x, y: r.y, width: r.width, height: r.height };
}

function aoMaximizar(w) {
    if (ocupado || !elegivel(w) || w.maximizeMode !== 3) return;  // 3 = maximizada nos dois eixos
    const k = chave(w);
    const e = estado[k] || {};
    ocupado = true;
    try {
        w.setMaximize(false, false);
        if (e.preenchida && perto(e.antesDeMaximizar, e.preenchida)) {
            // Estava "preenchida" e pediu maximizar de novo: volta ao tamanho antigo.
            aplicar(w, e.antes || areaPadrao(w));
            delete estado[k];
        } else {
            const p = areaPreenchida(w);
            estado[k] = { antes: e.antesDeMaximizar || null, preenchida: p };
            aplicar(w, p);
        }
    } finally {
        ocupado = false;
    }
}

// Roda fora do sinal do KWin (evita mexer na janela no meio da maximizacao).
function depois(fn) {
    const t = new QTimer();
    t.singleShot = true;
    t.interval = 0;
    t.timeout.connect(fn);
    t.start();
}

// Encaixe lado a lado (arrastar para a borda, Meta+setas): usa o "padding" dos
// blocos de encaixe rapido do KWin. Ele deixa a margem inteira contra a borda da
// tela e meia margem em cada janela entre elas (vao total = margem).
function aoEncaixar(w, bloco) {
    if (!bloco) return;
    let raiz = bloco;
    while (raiz.parent) raiz = raiz.parent;
    const t = workspace.tilingForScreen(w.output);
    if (t && t.rootTile === raiz) return;   // mosaico personalizado (Meta+T): nao mexe
    if (raiz.padding !== MARGEM) raiz.padding = MARGEM;
}

function acompanhar(w) {
    if (!w || !w.normalWindow) return;
    w.maximizedAboutToChange.connect(function (modo) {
        if (ocupado || modo !== 3 || w.maximizeMode !== 0) return;  // so se estava normal
        const k = chave(w);
        estado[k] = estado[k] || {};
        estado[k].antesDeMaximizar = copia(w.frameGeometry);
    });
    w.maximizedChanged.connect(function () { depois(function () { aoMaximizar(w); }); });
    w.tileChanged.connect(function (bloco) { aoEncaixar(w, bloco); });
    w.interactiveMoveResizeFinished.connect(function () {
        // O usuario arrastou ou redimensionou: deixa de ser "preenchida".
        const e = estado[chave(w)];
        if (e) e.preenchida = null;
    });
    if (w.tile) aoEncaixar(w, w.tile);
    if (w.maximizeMode === 3) depois(function () { aoMaximizar(w); });
}

workspace.windowList().forEach(acompanhar);
workspace.windowAdded.connect(acompanhar);
workspace.windowRemoved.connect(function (w) { delete estado[chave(w)]; });
JS
}

case "$1" in
  ligar)
    margem="${2:-12}"
    [[ "$margem" =~ ^[0-9]+$ ]] || { echo "margem precisa ser um numero (px)"; exit 1; }
    criar_pacote
    kpackagetool6 --type=KWin/Script --remove "$ID" >/dev/null 2>&1 || true
    kwriteconfig6 --file kwinrc --group Plugins --key "${ID}Enabled" false
    descarregar
    kpackagetool6 --type=KWin/Script --install "$DIR/$ID" >/dev/null
    kwriteconfig6 --file kwinrc --group "Script-$ID" --key Margem "$margem"
    kwriteconfig6 --file kwinrc --group Plugins --key "${ID}Enabled" true
    recarregar
    echo "Maximizar com margem ligado (margem de $margem px)."
    echo "Maximize uma janela para testar. Maximizar de novo volta ao tamanho anterior." ;;
  desligar)
    kwriteconfig6 --file kwinrc --group Plugins --key "${ID}Enabled" --delete
    kwriteconfig6 --file kwinrc --group "Script-$ID" --key Margem --delete
    descarregar
    recarregar
    kpackagetool6 --type=KWin/Script --remove "$ID" >/dev/null 2>&1 || true
    echo "Maximizar normal de volta."
    echo "(O vao entre janelas encaixadas lado a lado some depois de sair e entrar na sessao.)" ;;
  *) echo "uso: bash $0 ligar [margem] | desligar"; exit 1 ;;
esac
