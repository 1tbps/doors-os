// Doors OS: nome do computador, IP e domínio no papel de parede (como o BGInfo).
// O texto vem de /usr/bin/doors-info-maquina, chamado a cada 10 segundos.
import QtQuick
import QtQuick.Layouts
import org.kde.plasma.plasmoid
import org.kde.plasma.core as PlasmaCore
import org.kde.plasma.plasma5support as P5Support
import org.kde.kirigami as Kirigami

PlasmoidItem {
    id: root

    property var linhas: []
    readonly property string comando: "/usr/bin/doors-info-maquina"

    Plasmoid.backgroundHints: PlasmaCore.Types.NoBackground | PlasmaCore.Types.ConfigurableBackground
    preferredRepresentation: fullRepresentation
    toolTipMainText: linhas.length > 0 ? linhas[0] : ""
    toolTipSubText: linhas.slice(1).join("\n")

    P5Support.DataSource {
        id: executar
        engine: "executable"
        connectedSources: []
        onNewData: (fonte, dados) => {
            const saida = (dados["stdout"] || "").trim();
            if (saida.length > 0) {
                root.linhas = saida.split("\n");
            }
            disconnectSource(fonte);
        }
    }

    Timer {
        interval: 10000
        running: true
        repeat: true
        triggeredOnStart: true
        onTriggered: executar.connectSource(root.comando)
    }

    fullRepresentation: ColumnLayout {
        spacing: 0
        Layout.minimumWidth: Kirigami.Units.gridUnit * 10
        Layout.preferredWidth: Kirigami.Units.gridUnit * 16
        Layout.minimumHeight: implicitHeight

        Repeater {
            model: root.linhas
            delegate: Text {
                required property string modelData
                required property int index
                Layout.fillWidth: true
                horizontalAlignment: Text.AlignRight
                text: modelData
                color: "white"
                style: Text.Raised
                styleColor: "#cc000000"
                font.family: Kirigami.Theme.defaultFont.family
                font.bold: true
                font.pointSize: index === 0 ? Kirigami.Theme.defaultFont.pointSize * 1.6
                                            : Kirigami.Theme.defaultFont.pointSize * 1.1
                elide: Text.ElideLeft
            }
        }
    }
}
