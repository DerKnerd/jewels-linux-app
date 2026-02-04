import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami

import cloud.ulbricht.jewels

Kirigami.ScrollablePage {
    title: "Anmelden"
    id: loginPage
    Layout.fillWidth: true

    ColumnLayout {
        width: loginPage.width
        spacing: Kirigami.Units.smallSpacing

        Kirigami.InlineMessage {
            id: loginMessage
            Layout.fillWidth: true
            Layout.alignment: Qt.AlignStart | Qt.AlignTop
            text: "Zum Anmelden musst du einfach nur den Button unten klicken. Der Rest passiert automatisch."
            visible: true
            actions: [
                Kirigami.Action {
                    text: "Login starten"
                    visible: !Login.loginInProgress
                    onTriggered: {
                        loginMessage.text = "Der Login wurde gestartet, bitte schau in deinem Browser nach."
                        Login.triggerLogin()
                    }
                }
            ]
        }

        Controls.BusyIndicator {
            Layout.alignment: Qt.AlignTop | Qt.AlignHCenter
            id: loginBusyIndicator
            visible: Login.loginInProgress
        }
    }
}