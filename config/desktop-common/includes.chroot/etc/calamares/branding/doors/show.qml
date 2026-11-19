import QtQuick 2.0;
import calamares.slideshow 1.0;
Presentation {
    id: presentation
    function onActivate() { }
    function onLeave() { }
    Slide {
        Image { source: "slide1.png"; width: 560; height: 280; fillMode: Image.PreserveAspectFit; anchors.centerIn: parent }
        Text {
            anchors.horizontalCenter: parent.horizontalCenter; anchors.top: parent.verticalCenter; anchors.topMargin: 160
            text: "Bem-vindo ao Doors OS.<br/>A instalação é automática e termina em alguns minutos."
            horizontalAlignment: Text.AlignHCenter; wrapMode: Text.WordWrap; width: parent.width
        }
    }
}
