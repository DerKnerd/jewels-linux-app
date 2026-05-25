import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami

import cloud.ulbricht.jewels

Kirigami.ScrollablePage {
    id: loginPage

    required property Login login

    Layout.fillWidth: true
    title: "Anmelden"

    Connections {
        function onLoginSuccessful() {
            Owners.load();
            const stack = applicationWindow().pageStack;
            stack.replace(Qt.resolvedUrl("JewelsPage.qml"), {
                login: login
            });
        }

        target: login
    }
    ColumnLayout {
        spacing: Kirigami.Units.smallSpacing
        width: loginPage.width

        Kirigami.InlineMessage {
            id: loginMessage

            Layout.alignment: Qt.AlignStart | Qt.AlignTop
            Layout.fillWidth: true
            text: "Zum Anmelden musst du einfach nur den Button unten klicken. Der Rest passiert automatisch."
            visible: true

            actions: [
                Kirigami.Action {
                    text: "Login starten"
                    visible: !login.loginInProgress

                    onTriggered: {
                        loginMessage.text = "Der Login wurde gestartet, bitte schau in deinem Browser nach.";
                        login.login();
                    }
                }
            ]
        }
        Controls.BusyIndicator {
            id: loginBusyIndicator

            Layout.alignment: Qt.AlignTop | Qt.AlignHCenter
            visible: login.loginInProgress
        }
    }
}