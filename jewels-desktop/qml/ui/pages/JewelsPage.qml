import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import cloud.ulbricht.jewels

Kirigami.ScrollablePage {
    required property Login login

    title: "Meine Geräte"
    id: jewelsPage

    Jewels {
        id: jewels
    }

    Component.onCompleted: jewels.loadDevices()

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
            Layout.alignment: Qt.AlignTop
            id: loadingFailedMessage
            Layout.fillWidth: true
            visible: jewels.loadingFailed && !jewels.isLoading
            type: Kirigami.MessageType.Error
            text: "Leider konnten deine Geräte nicht geladen werden."

            actions: [
                Kirigami.Action {
                    text: "Erneut versuchen"
                    onTriggered: jewels.loadDevices()
                }
            ]
        }

        Repeater {
            id: view

            model: jewels
            clip: true
            delegate: Kirigami.AbstractCard
            {
                header: Kirigami.Heading
                {
                    text: manufacturer + " " + model
                    level: 2
                }

                contentItem: ColumnLayout {
                    Kirigami.Heading {
                        text: "Betriebssystem"
                        level: 3
                    }
                    Controls.Label {
                        text: os
                    }

                    Kirigami.Heading {
                        text: "Prozessor"
                        level: 3
                        visible: deviceType === "computer"
                    }
                    Controls.Label {
                        text: cpu
                        visible: deviceType === "computer"
                    }

                    Kirigami.Heading {
                        text: "Arbeitsspeicher"
                        level: 3
                        visible: deviceType === "computer"
                    }
                    Controls.Label {
                        text: ram.toFixed(2) + " GB"
                        visible: deviceType === "computer"
                    }

                    Kirigami.Heading {
                        text: "Speicherplatz"
                        level: 3
                    }
                    Controls.Label {
                        text: storage.toFixed(2) + " GB"
                    }
                }
            }
        }
    }
}