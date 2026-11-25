// Doors OS: painel único embaixo, no estilo do Linux Mint
// menu | atalhos e janelas abertas | bandeja | relógio | mostrar área de trabalho
var panel = new Panel
panel.location = "bottom"
panel.height = 2 * Math.floor(gridUnit * 2.5 / 2)

var menu = panel.addWidget("org.kde.plasma.kickoff")
menu.currentConfigGroup = ["General"]
menu.writeConfig("icon", "doors-logo")

var tasks = panel.addWidget("org.kde.plasma.icontasks")
tasks.currentConfigGroup = ["General"]
tasks.writeConfig("launchers", ["applications:chromium.desktop",
                                "applications:org.kde.dolphin.desktop",
                                "applications:org.kde.konsole.desktop"])

panel.addWidget("org.kde.plasma.marginsseparator")
panel.addWidget("org.kde.plasma.systemtray")
panel.addWidget("org.kde.plasma.digitalclock")
panel.addWidget("org.kde.plasma.showdesktop")

var desktopsArray = desktopsForActivity(currentActivity());
for (var j = 0; j < desktopsArray.length; j++) {
    desktopsArray[j].wallpaperPlugin = 'org.kde.image';
    // nome do computador, IP e domínio no canto de cima à direita (como o BGInfo)
    var tela = screenGeometry(desktopsArray[j].screen);
    desktopsArray[j].addWidget("org.doors.infomaquina",
                               Math.max(0, tela.width - gridUnit * 19), gridUnit,
                               gridUnit * 18, gridUnit * 6);
}
