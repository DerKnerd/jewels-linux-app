import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import cloud.ulbricht.jewels

Kirigami.Page {
    id: jewelsPage

    Layout.fillHeight: true
    Layout.fillWidth: true
    title: "Jewels"

    actions: [
        Kirigami.Action {
            text: "Updates installieren"
            visible: loggedIn

            onTriggered: {
                Jewels.updateSystem();
            }
        },
        Kirigami.Action {
            text: "Daten senden"
            visible: loggedIn

            onTriggered: {
                Jewels.sendData(Config.host, Config.token);
            }
        },
        Kirigami.Action {
            text: "Abmelden"
            visible: loggedIn

            onTriggered: {
                Config.host = "";
                Config.token = "";
                loginSheet.open();
            }
        }
    ]

    ColumnLayout {
        visible: loggedIn
        width: jewelsPage.width

        Kirigami.Heading {
            Layout.fillWidth: true
            text: "Verbunden"
        }
        Controls.Label {
            Layout.fillWidth: true
            text: `Du bist mit <b>${Config.host}</b> verbunden`
        }
    }
}
