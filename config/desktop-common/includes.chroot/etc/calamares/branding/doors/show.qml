import QtQuick 2.0;
import calamares.slideshow 1.0;
Presentation {
    id: presentation
    function onActivate() { }
    function onLeave() { }
    Timer {
        interval: 6000; running: true; repeat: true
        onTriggered: presentation.goToNextSlide()
    }
    Slide { Image { source: "slide1.png"; width: 640; height: 320; fillMode: Image.PreserveAspectFit; anchors.centerIn: parent } }
    Slide { Image { source: "slide2.png"; width: 640; height: 320; fillMode: Image.PreserveAspectFit; anchors.centerIn: parent } }
    Slide { Image { source: "slide3.png"; width: 640; height: 320; fillMode: Image.PreserveAspectFit; anchors.centerIn: parent } }
}
