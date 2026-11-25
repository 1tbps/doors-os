// Doors OS: janelas do Cinnamon sem sombra.
// O muffin desenha a sombra das janelas com bordas do gerenciador (.ssd); com
// "janelas gelatinosas" ela deixava rastro ao arrastar. Aqui a opacidade das
// sombras das janelas vai a 0 (o tamanho fica igual, para nada mudar de lugar).
// Os menus continuam com sombra. Ao desligar a extensão, volta o que era.
const Meta = imports.gi.Meta;

// classes de sombra do muffin que são janelas (menus ficam de fora)
const CLASSES = ['normal', 'dialog', 'modal_dialog', 'utility', 'border', 'attached'];

let originais = null;

function init(meta) {}

function enable() {
    const fabrica = Meta.ShadowFactory.get_default();
    originais = [];
    for (const classe of CLASSES) {
        for (const focada of [true, false]) {
            let p;
            try {
                p = fabrica.get_params(classe, focada);
            } catch (e) {
                continue;
            }
            if (!p || typeof p.radius !== 'number') continue;
            originais.push([classe, focada, {
                radius: p.radius, top_fade: p.top_fade,
                x_offset: p.x_offset, y_offset: p.y_offset, opacity: p.opacity,
            }]);
            fabrica.set_params(classe, focada, new Meta.ShadowParams({
                radius: p.radius, top_fade: p.top_fade,
                x_offset: p.x_offset, y_offset: p.y_offset, opacity: 0,
            }));
        }
    }
}

function disable() {
    if (!originais) return;
    const fabrica = Meta.ShadowFactory.get_default();
    for (const [classe, focada, v] of originais) {
        try {
            fabrica.set_params(classe, focada, new Meta.ShadowParams(v));
        } catch (e) {
            global.logError(e);
        }
    }
    originais = null;
}
