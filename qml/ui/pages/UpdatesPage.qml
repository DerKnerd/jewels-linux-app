import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import cloud.ulbricht.jewels

Kirigami.ScrollablePage {
    objectName: "UpdatesPage"

    id: updatesPage

    Layout.fillHeight: true
    Layout.fillWidth: true
    title: "Updates"

    actions: [
        Kirigami.Action {
            text: "Updates installieren"

            onTriggered: {
                Updates.updateSystem();
            }
        }
    ]

    ColumnLayout {
        width: updatesPage.width

        Kirigami.Heading {
            Layout.fillWidth: true
            text: "Verfügbare Updates"
        }
        Controls.BusyIndicator {
            id: updatesBusyIndicator
            visible: Updates.refreshing
            Layout.alignment: Qt.AlignHCenter
        }
        Controls.Label {
            visible: Updates.updateCount > 0 && !Updates.refreshing
            Layout.fillWidth: true
            text: `Du hast aktuell ${Updates.updateCount} verfügbare Updates.`
        }
    }
}
