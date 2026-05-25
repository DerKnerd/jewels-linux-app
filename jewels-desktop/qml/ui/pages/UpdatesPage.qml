import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import cloud.ulbricht.jewels
import org.kde.kirigami as Kirigami

Kirigami.ScrollablePage {
    id: updatesPage

    background: Kirigami.Theme.backgroundColor
    objectName: "UpdatesPage"
    title: "Updates"

    Updates {
        id: updates

        Component.onCompleted: updates.refreshCache()
        onDownloadFinishedChanged: {
            updates.downloadStatus1.reset();
            updates.downloadStatus2.reset();
            updates.downloadStatus3.reset();
            updates.downloadStatus4.reset();
        }
        onUpdateFinishedChanged: updates.refreshCache()
    }
    ColumnLayout {
        anchors.fill: parent
        spacing: Kirigami.Units.largeSpacing

        Kirigami.Heading {
            Layout.alignment: Qt.AlignTop
            Layout.fillWidth: true
            text: "Verfügbare Updates"
        }
        Controls.BusyIndicator {
            id: updatesBusyIndicator

            Layout.alignment: Qt.AlignTop | Qt.AlignHCenter
            visible: updates.refreshing
        }
        Kirigami.InlineMessage {
            id: updatesMessage

            Layout.alignment: Qt.AlignTop
            Layout.fillWidth: true
            text: `Du hast aktuell ${updates.updateCount} verfügbare Updates.`
            visible: updates.updateCount > 0 && !updates.refreshing

            actions: [
                Kirigami.Action {
                    text: "Jetzt aktualisieren"
                    visible: !updates.updateInProgress

                    onTriggered: {
                        updatesMessage.text = "Die Updates werden installiert. Bitte warte ein bisschen, " + "du siehst unten den Fortschritt.";
                        updates.updateSystem();
                    }
                }
            ]
        }
        Kirigami.InlineMessage {
            id: noUpdatesMessage

            Layout.alignment: Qt.AlignTop
            Layout.fillWidth: true
            text: "Super, dein Rechner ist aktuell. Es gibt nichts zu tun."
            type: Kirigami.MessageType.Positive
            visible: updates.updateCount === 0 && !updates.refreshing && !updates.refreshingFailed

            actions: [
                Kirigami.Action {
                    text: "Such nochmal"

                    onTriggered: updates.refreshCache()
                }
            ]
        }
        Kirigami.InlineMessage {
            id: refreshFailedMessage

            Layout.alignment: Qt.AlignTop
            Layout.fillWidth: true
            text: "Leider konnten die neuesten Updates nicht abgerufen werden. " + "Du kannst es jederzeit erneut versuchen."
            type: Kirigami.MessageType.Error
            visible: updates.refreshingFailed && !updates.refreshing

            actions: [
                Kirigami.Action {
                    text: "Erneut versuchen"

                    onTriggered: updates.refreshCache()
                }
            ]
        }
        GridLayout {
            Layout.alignment: Qt.AlignTop
            columnSpacing: Kirigami.Units.smallSpacing
            columns: 3
            rowSpacing: Kirigami.Units.smallSpacing
            visible: updates.updateInProgress || !updates.downloadFinished

            Controls.Label {
                Layout.maximumWidth: 150
                Layout.minimumWidth: 150
                elide: Text.ElideRight
                text: updates.downloadStatus1.name
                visible: updates.updateInProgress && !updates.downloadFinished
            }
            Controls.ProgressBar {
                Layout.fillWidth: true
                from: 0
                to: updates.downloadStatus1.total
                value: updates.downloadStatus1.current
                visible: updates.updateInProgress && !updates.downloadFinished
            }
            Controls.Label {
                Layout.alignment: Qt.AlignRight
                text: `${updates.downloadStatus1.percent.toFixed(0)} %`
                visible: updates.updateInProgress && !updates.downloadFinished
            }
            Controls.Label {
                Layout.maximumWidth: 150
                Layout.minimumWidth: 150
                elide: Text.ElideRight
                text: updates.downloadStatus2.name
                visible: updates.updateInProgress && !updates.downloadFinished
            }
            Controls.ProgressBar {
                Layout.fillWidth: true
                from: 0
                to: updates.downloadStatus2.total
                value: updates.downloadStatus2.current
                visible: updates.updateInProgress && !updates.downloadFinished
            }
            Controls.Label {
                Layout.alignment: Qt.AlignRight
                text: `${updates.downloadStatus2.percent.toFixed(0)} %`
                visible: updates.updateInProgress && !updates.downloadFinished
            }
            Controls.Label {
                Layout.maximumWidth: 150
                Layout.minimumWidth: 150
                elide: Text.ElideRight
                text: updates.downloadStatus3.name
                visible: updates.updateInProgress && !updates.downloadFinished
            }
            Controls.ProgressBar {
                Layout.fillWidth: true
                from: 0
                to: updates.downloadStatus3.total
                value: updates.downloadStatus3.current
                visible: updates.updateInProgress && !updates.downloadFinished
            }
            Controls.Label {
                Layout.alignment: Qt.AlignRight
                text: `${updates.downloadStatus3.percent.toFixed(0)} %`
                visible: updates.updateInProgress && !updates.downloadFinished
            }
            Controls.Label {
                Layout.maximumWidth: 150
                Layout.minimumWidth: 150
                elide: Text.ElideRight
                text: updates.downloadStatus4.name
                visible: updates.updateInProgress && !updates.downloadFinished
            }
            Controls.ProgressBar {
                Layout.fillWidth: true
                from: 0
                to: updates.downloadStatus4.total
                value: updates.downloadStatus4.current
                visible: updates.updateInProgress && !updates.downloadFinished
            }
            Controls.Label {
                Layout.alignment: Qt.AlignRight
                text: `${updates.downloadStatus4.percent.toFixed(0)} %`
                visible: updates.updateInProgress && !updates.downloadFinished
            }
            Controls.Label {
                Layout.maximumWidth: 150
                Layout.minimumWidth: 150
                elide: Text.ElideRight
                text: updates.installPackage
                visible: updates.updateInProgress && updates.downloadFinished
            }
            Controls.ProgressBar {
                Layout.fillWidth: true
                from: 0
                to: updates.installHowMany
                value: updates.installCurrent
                visible: updates.updateInProgress && updates.downloadFinished
            }
            Controls.Label {
                text: `${updates.installCurrent} von ${updates.installHowMany}`
                visible: updates.updateInProgress && updates.downloadFinished
            }
        }
        Item {
            Layout.fillHeight: true
            Layout.fillWidth: true
            Layout.verticalStretchFactor: 1
            visible: updates.updateCount === 0
        }
        ListView {
            id: view

            Layout.fillHeight: true
            Layout.fillWidth: true
            model: updates
            spacing: Kirigami.Units.largeSpacing

            delegate: Kirigami.AbstractCard {
                contentItem: Controls.Label {
                    text: description
                    wrapMode: Text.WordWrap
                }
                header: GridLayout {
                    columns: 2

                    Kirigami.Heading {
                        level: 2
                        text: name
                    }
                    Controls.Label {
                        Layout.alignment: Qt.AlignRight
                        opacity: 0.5
                        text: version
                        wrapMode: Text.WordWrap
                    }
                }
            }
        }
    }
}
