import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import cloud.ulbricht.jewels
import org.kde.kirigami as Kirigami

Kirigami.ScrollablePage {
    id: jewelsPage

    required property Login login

    title: "Meine Geräte"

    Component.onCompleted: jewels.loadDevices()

    Jewels {
        id: jewels
    }
    ColumnLayout {
        anchors.fill: parent

        Kirigami.Heading {
            Layout.alignment: Qt.AlignTop
            Layout.fillWidth: true
            text: "Meine Geräte"
        }
        Controls.BusyIndicator {
            id: devicesBusyIndicator

            Layout.alignment: Qt.AlignTop | Qt.AlignHCenter
            visible: jewels.isLoading
        }
        Kirigami.InlineMessage {
            id: loadingFailedMessage

            Layout.alignment: Qt.AlignTop
            Layout.fillWidth: true
            text: "Leider konnten deine Geräte nicht geladen werden."
            type: Kirigami.MessageType.Error
            visible: jewels.loadingFailed && !jewels.isLoading

            actions: [
                Kirigami.Action {
                    text: "Erneut versuchen"

                    onTriggered: jewels.loadDevices()
                }
            ]
        }
        Repeater {
            id: view

            clip: true
            model: jewels

            delegate: Kirigami.AbstractCard {
                contentItem: ColumnLayout {
                    Kirigami.Heading {
                        level: 3
                        text: "Betriebssystem"
                    }
                    Controls.Label {
                        text: os
                    }
                    Kirigami.Heading {
                        level: 3
                        text: "Prozessor"
                        visible: deviceType === "computer"
                    }
                    Controls.Label {
                        text: cpu
                        visible: deviceType === "computer"
                    }
                    Kirigami.Heading {
                        level: 3
                        text: "Arbeitsspeicher"
                        visible: deviceType === "computer"
                    }
                    Controls.Label {
                        text: ram.toFixed(2) + " GB"
                        visible: deviceType === "computer"
                    }
                    Kirigami.Heading {
                        level: 3
                        text: "Speicherplatz"
                    }
                    Controls.Label {
                        text: storage.toFixed(2) + " GB"
                    }
                }
                header: Kirigami.Heading {
                    level: 2
                    text: manufacturer + " " + model
                }
            }
        }
    }
}