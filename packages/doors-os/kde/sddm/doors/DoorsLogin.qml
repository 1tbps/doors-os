// Doors OS: cópia do Login.qml do tema breeze do Debian (Plasma 6.3.6) com a foto do usuário
// digitado. Com fotoPeloNome, a "lista" tem um item só: o nome digitado e a foto que o
// AccountsService guarda em /var/lib/AccountsService/icons/<usuário> (a que se escolhe em
// Configurações > Usuários). Usuário sem foto ou campo vazio: a figura padrão do Breeze.
import org.kde.breeze.components

import QtQuick 2.15
import QtQuick.Layouts 1.15
import QtQuick.Controls 2.15 as QQC2

import org.kde.plasma.components 3.0 as PlasmaComponents3
import org.kde.plasma.extras 2.0 as PlasmaExtras
import org.kde.kirigami 2.20 as Kirigami

SessionManagementScreen {
    id: root
    property Item mainPasswordBox: passwordBox

    property bool showUsernamePrompt: !showUserList

    property string lastUserName
    property bool loginScreenUiVisible: false

    // ---- Doors OS: foto de quem foi digitado ----
    property bool fotoPeloNome: false
    readonly property string fotoPadrao: Qt.resolvedUrl("../breeze/faces/.face.icon")
    // um modelo (e não uma lista de objetos) por causa do QTBUG-75900, como no Breeze
    property ListModel fotoModel: ListModel {
        ListElement {
            name: ""
            icon: ""
        }
    }

    // "DOMINIO\usuario" e "usuario@dominio" -> "usuario"; o AccountsService guarda em minúsculas
    function nomeDigitado() {
        let n = userNameInput.text.trim();
        n = n.substring(n.lastIndexOf("\\") + 1);
        if (n.indexOf("@") > 0) {
            n = n.substring(0, n.indexOf("@"));
        }
        return n.toLowerCase();
    }

    function atualizarFoto() {
        if (!fotoPeloNome) {
            return;
        }
        const n = nomeDigitado();
        fotoModel.setProperty(0, "name", n || i18nd("plasma-desktop-sddm-theme", "Type in Username and Password"));
        if (!n || n.indexOf("/") >= 0) {
            root.sonda.source = "";
            fotoModel.setProperty(0, "icon", fotoPadrao);
        } else {
            root.sonda.source = "file:///var/lib/AccountsService/icons/" + encodeURIComponent(n);
        }
    }

    // carrega a foto escondida; só troca a figura quando a foto existe (sem foto: figura padrão).
    // Como propriedades: os filhos diretos daqui vão para o layout dos campos (_children).
    property Image sonda: Image {
        visible: false
        asynchronous: true
        cache: false
        sourceSize.width: 1
        sourceSize.height: 1
        onStatusChanged: {
            if (status === Image.Ready) {
                root.fotoModel.setProperty(0, "icon", source.toString());
            } else if (status === Image.Error || status === Image.Null) {
                root.fotoModel.setProperty(0, "icon", root.fotoPadrao);
            }
        }
    }

    property Timer esperaDigitar: Timer {
        interval: 300
        onTriggered: root.atualizarFoto()
    }

    Component.onCompleted: atualizarFoto()
    onFotoPeloNomeChanged: atualizarFoto()
    // ---- fim Doors OS ----

    //the y position that should be ensured visible when the on screen keyboard is visible
    property int visibleBoundary: mapFromItem(loginButton, 0, 0).y
    onHeightChanged: visibleBoundary = mapFromItem(loginButton, 0, 0).y + loginButton.height + Kirigami.Units.smallSpacing

    property real fontSize: Kirigami.Theme.defaultFont.pointSize

    signal loginRequest(string username, string password)

    onShowUsernamePromptChanged: {
        if (!showUsernamePrompt) {
            lastUserName = ""
        }
    }

    onUserSelected: {
        // Don't startLogin() here, because the signal is connected to the
        // Escape key as well, for which it wouldn't make sense to trigger
        // login.
        passwordBox.clear()
        focusFirstVisibleFormControl();
    }

    QQC2.StackView.onActivating: {
        // Controls are not visible yet.
        Qt.callLater(focusFirstVisibleFormControl);
    }

    function focusFirstVisibleFormControl() {
        const nextControl = (userNameInput.visible
            ? userNameInput
            : (passwordBox.visible
                ? passwordBox
                : loginButton));
        // Using TabFocusReason, so that the loginButton gets the visual highlight.
        nextControl.forceActiveFocus(Qt.TabFocusReason);
    }

    /*
     * Login has been requested with the following username and password
     * If username field is visible, it will be taken from that, otherwise from the "name" property of the currentIndex
     */
    function startLogin() {
        const username = showUsernamePrompt ? userNameInput.text : userList.selectedUser
        const password = passwordBox.text

        footer.enabled = false
        mainStack.enabled = false
        userListComponent.userList.opacity = 0.5

        // This is partly because it looks nicer, but more importantly it
        // works round a Qt bug that can trigger if the app is closed with a
        // TextField focused.
        //
        // See https://bugreports.qt.io/browse/QTBUG-55460
        loginButton.forceActiveFocus();
        loginRequest(username, password);
    }

    PlasmaComponents3.TextField {
        id: userNameInput
        font.pointSize: fontSize + 1
        Layout.fillWidth: true

        text: lastUserName
        visible: showUsernamePrompt
        focus: showUsernamePrompt && !lastUserName //if there's a username prompt it gets focus first, otherwise password does
        placeholderText: i18nd("plasma-desktop-sddm-theme", "Username")

        // Doors OS: troca a foto quando o nome muda (esperando parar de digitar)
        onTextChanged: root.esperaDigitar.restart()

        onAccepted: {
            if (root.loginScreenUiVisible) {
                passwordBox.forceActiveFocus()
            }
        }
    }

    RowLayout {
        Layout.fillWidth: true

        PlasmaExtras.PasswordField {
            id: passwordBox
            font.pointSize: fontSize + 1
            Layout.fillWidth: true

            placeholderText: i18nd("plasma-desktop-sddm-theme", "Password")
            focus: !showUsernamePrompt || lastUserName

            // Disable reveal password action because SDDM does not have the breeze icon set loaded
            rightActions: []

            onAccepted: {
                if (root.loginScreenUiVisible) {
                    startLogin();
                }
            }

            visible: root.showUsernamePrompt || userList.currentItem.needsPassword

            Keys.onEscapePressed: {
                mainStack.currentItem.forceActiveFocus();
            }

            //if empty and left or right is pressed change selection in user switch
            //this cannot be in keys.onLeftPressed as then it doesn't reach the password box
            Keys.onPressed: event => {
                if (event.key === Qt.Key_Left && !text) {
                    userList.decrementCurrentIndex();
                    event.accepted = true
                }
                if (event.key === Qt.Key_Right && !text) {
                    userList.incrementCurrentIndex();
                    event.accepted = true
                }
            }

            Connections {
                target: sddm
                function onLoginFailed() {
                    passwordBox.selectAll()
                    passwordBox.forceActiveFocus()
                }
            }
        }

        PlasmaComponents3.Button {
            id: loginButton
            Accessible.name: i18nd("plasma-desktop-sddm-theme", "Log In")
            Layout.preferredHeight: passwordBox.implicitHeight
            Layout.preferredWidth: text.length === 0 ? loginButton.Layout.preferredHeight : -1

            icon.name: text.length === 0 ? (root.LayoutMirroring.enabled ? "go-previous" : "go-next") : ""

            text: root.showUsernamePrompt || userList.currentItem.needsPassword ? "" : i18n("Log In")
            onClicked: startLogin()
            Keys.onEnterPressed: clicked()
            Keys.onReturnPressed: clicked()
        }
    }
}
