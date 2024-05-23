import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import org.kde.kirigamiaddons.formcard as FormCard
import dev.imanuel.jewels

Kirigami.ApplicationWindow {
    id: root

    minimumHeight: Kirigami.Units.gridUnit * 20
    minimumWidth: Kirigami.Units.gridUnit * 20
    pageStack.initialPage: Qt.resolvedUrl("jewels.qml")
    title: "Jewels"

    contextDrawer: Kirigami.ContextDrawer {
        id: contextDrawer

    }

    Component.onCompleted: App.restoreWindowGeometry(root)
    onActiveChanged: {
        if (!App.loggedIn) {
            loginDialog.open();
        }
    }
    onClosing: App.saveWindowGeometry(root)
    onHeightChanged: saveWindowGeometryTimer.restart()
    onWidthChanged: saveWindowGeometryTimer.restart()
    onXChanged: saveWindowGeometryTimer.restart()
    onYChanged: saveWindowGeometryTimer.restart()

    Timer {
        id: saveWindowGeometryTimer

        interval: 1000

        onTriggered: App.saveWindowGeometry(root)
    }
    Kirigami.Dialog {
        id: loginDialog

        modal: true
        showCloseButton: false
        standardButtons: Kirigami.Dialog.NoButton
        title: "Anmelden"

        customFooterActions: [
            Kirigami.Action {
                Controls.DialogButtonBox.buttonRole: Controls.DialogButtonBox.AcceptRole
                text: "Anmelden"

                onTriggered: {
                    App.login(loginUrl.text, loginToken.text);
                    loginDialog.close();
                }
            }
        ]

        onClosed: {
            if (!App.loggedIn) {
                loginDialog.open();
            }
        }

        ColumnLayout {
            anchors.fill: parent
            spacing: Kirigami.Units.largeSpacing

            Kirigami.InlineMessage {
                Layout.fillWidth: true
                Layout.leftMargin: 16
                Layout.rightMargin: 16
                Layout.topMargin: 16
                text: "Die Url und das Token findest du im Browser unter dem QR Code"
                visible: true
            }
            Kirigami.FormLayout {
                Layout.bottomMargin: 16
                Layout.fillWidth: true
                Layout.leftMargin: 16
                Layout.rightMargin: 16

                Controls.TextField {
                    id: loginUrl

                    Kirigami.FormData.label: "Url"
                }
                Controls.TextField {
                    id: loginToken

                    Kirigami.FormData.label: "Token"
                }
            }
        }
    }
}
