// Doors OS: nome do computador, IP e domínio no canto de cima à direita do papel de
// parede (como o BGInfo do Windows). O texto vem de /usr/bin/doors-info-maquina,
// chamado a cada 10 segundos. Fica junto do fundo da tela: atrás das janelas.
import Clutter from 'gi://Clutter';
import Gio from 'gi://Gio';
import GLib from 'gi://GLib';
import St from 'gi://St';
import * as Main from 'resource:///org/gnome/shell/ui/main.js';
import {Extension} from 'resource:///org/gnome/shell/extensions/extension.js';

const COMANDO = ['/usr/bin/doors-info-maquina'];
const INTERVALO = 10; // segundos
const MARGEM = 24;

export default class InfoMaquina extends Extension {
    enable() {
        this._caixa = new St.BoxLayout({
            vertical: true,
            style_class: 'doors-info-maquina',
            reactive: false,
        });
        Main.layoutManager._backgroundGroup.add_child(this._caixa);

        this._idTamanho = this._caixa.connect('notify::width', () => this._posicionar());
        this._idMonitores = Main.layoutManager.connect('monitors-changed', () => this._posicionar());

        this._atualizar();
        this._idTempo = GLib.timeout_add_seconds(GLib.PRIORITY_LOW, INTERVALO, () => {
            this._atualizar();
            return GLib.SOURCE_CONTINUE;
        });
    }

    disable() {
        if (this._idTempo) {
            GLib.source_remove(this._idTempo);
            this._idTempo = 0;
        }
        if (this._idMonitores) {
            Main.layoutManager.disconnect(this._idMonitores);
            this._idMonitores = 0;
        }
        this._cancelar?.cancel();
        this._cancelar = null;
        this._caixa?.destroy();
        this._caixa = null;
        this._textoAtual = null;
    }

    _atualizar() {
        this._cancelar?.cancel();
        this._cancelar = new Gio.Cancellable();
        let proc;
        try {
            proc = Gio.Subprocess.new(COMANDO,
                Gio.SubprocessFlags.STDOUT_PIPE | Gio.SubprocessFlags.STDERR_SILENCE);
        } catch (e) {
            logError(e, 'doors-info-maquina');
            return;
        }
        proc.communicate_utf8_async(null, this._cancelar, (p, res) => {
            let saida;
            try {
                [, saida] = p.communicate_utf8_finish(res);
            } catch (e) {
                return; // cancelado (extensão desligada) ou erro
            }
            this._mostrar((saida || '').trim());
        });
    }

    _mostrar(texto) {
        if (!this._caixa || !texto || texto === this._textoAtual)
            return;
        this._textoAtual = texto;
        this._caixa.destroy_all_children();
        texto.split('\n').forEach((linha, i) => {
            const rotulo = new St.Label({
                text: linha,
                style_class: i === 0 ? 'doors-info-maquina-nome' : '',
                x_align: Clutter.ActorAlign.END,
            });
            this._caixa.add_child(rotulo);
        });
        this._posicionar();
    }

    _posicionar() {
        const monitor = Main.layoutManager.primaryMonitor;
        if (!this._caixa || !monitor)
            return;
        const topo = Main.panel?.height ?? 0;
        this._caixa.set_position(
            monitor.x + monitor.width - this._caixa.width - MARGEM,
            monitor.y + topo + MARGEM);
    }
}
