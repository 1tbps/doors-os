// Doors OS: liga o widget "Nome e IP do computador" (org.doors.infomaquina) no canto
// de cima à direita da área de trabalho. O Plasma roda este script uma vez por usuário;
// quem já tem o widget (ou o tirou depois) não é alterado.
var largura = gridUnit * 18
var altura = gridUnit * 6
var areas = desktops()
for (var i = 0; i < areas.length; i++) {
    var area = areas[i]
    if (area.widgets("org.doors.infomaquina").length > 0) {
        continue
    }
    var tela = screenGeometry(area.screen)
    var w = area.addWidget("org.doors.infomaquina",
                           Math.max(0, tela.width - largura - gridUnit), gridUnit,
                           largura, altura)
}
