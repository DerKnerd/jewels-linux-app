import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import cloud.ulbricht.jewels

Kirigami.Page {
    id: updatesPage
    objectName: "UpdatesPage"
    title: "Updates"

    Updates {
        id: updates
        // Component.onCompleted: updates.refreshCache()
        onUpdateFinishedChanged: updates.refreshCache()
    }

    ColumnLayout {
        anchors.fill: parent

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
            Layout.alignment: Qt.AlignTop
            id: updatesMessage
            Layout.fillWidth: true
            text: `Du hast aktuell ${updates.updateCount} verfügbare Updates.`
            visible: updates.updateCount > 0 && !updates.refreshing

            actions: [
                Kirigami.Action {
                    text: "Jetzt aktualisieren"
                    visible: !updates.updateInProgress
                    onTriggered: {
                        updatesMessage.text =
                            "Die Updates werden installiert. Bitte warte ein bisschen, " +
                            "du siehst unten den Fortschritt.";
                        updates.updateSystem();
                    }
                }
            ]
        }

        Kirigami.InlineMessage {
            Layout.alignment: Qt.AlignTop
            id: noUpdatesMessage
            Layout.fillWidth: true
            text: "Super, dein Rechner ist aktuell. Es gibt nichts zu tun."
            visible: updates.updateCount === 0 && !updates.refreshing
            type: Kirigami.MessageType.Positive

            actions: [
                Kirigami.Action {
                    text: "Such nochmal"
                    onTriggered: updates.refreshCache()
                }
            ]
        }

        Kirigami.InlineMessage {
            Layout.alignment: Qt.AlignTop
            id: refreshFailedMessage
            Layout.fillWidth: true
            visible: updates.refreshingFailed
            type: Kirigami.MessageType.Error
            text: "Leider konnten die neuesten Updates nicht abgerufen werden. " +
                "Du kannst es jederzeit erneut versuchen."

            actions: [
                Kirigami.Action {
                    text: "Erneut versuchen"
                    onTriggered: updates.refreshCache()
                }
            ]
        }

        GridLayout {
            Layout.alignment: Qt.AlignTop
            columns: 3
            visible: updates.updateInProgress || !updates.downloadFinished
            rowSpacing: Kirigami.Units.smallSpacing
            columnSpacing: Kirigami.Units.smallSpacing

            Text {
                Layout.minimumWidth: 150
                Layout.maximumWidth: 150
                elide: Text.ElideRight
                text: updates.downloadStatus1.name
                visible: updates.updateInProgress && !updates.downloadFinished
            }
            Controls.ProgressBar {
                Layout.fillWidth: true
                from: 0
                to: 100
                value: updates.downloadStatus1.percent
                visible: updates.updateInProgress && !updates.downloadFinished
            }
            Text {
                Layout.alignment: Qt.AlignRight
                text: `${updates.downloadStatus1.percent.toFixed(2)} %`
                visible: updates.updateInProgress && !updates.downloadFinished
            }

            Text {
                Layout.bottomMargin: Kirigami.Units.mediumSpacing
                Layout.minimumWidth: 150
                Layout.maximumWidth: 150
                elide: Text.ElideRight
                text: updates.downloadStatus2.name
                visible: updates.updateInProgress && !updates.downloadFinished
            }
            Controls.ProgressBar {
                Layout.fillWidth: true
                from: 0
                to: 100
                value: updates.downloadStatus2.percent
                visible: updates.updateInProgress && !updates.downloadFinished
            }
            Text {
                Layout.alignment: Qt.AlignRight
                text: `${updates.downloadStatus2.percent.toFixed(2)} %`
                visible: updates.updateInProgress && !updates.downloadFinished
            }

            Text {
                Layout.minimumWidth: 150
                Layout.maximumWidth: 150
                elide: Text.ElideRight
                text: updates.downloadStatus3.name
                visible: updates.updateInProgress && !updates.downloadFinished
            }
            Controls.ProgressBar {
                Layout.fillWidth: true
                from: 0
                to: 100
                value: updates.downloadStatus3.percent
                visible: updates.updateInProgress && !updates.downloadFinished
            }
            Text {
                Layout.alignment: Qt.AlignRight
                text: `${updates.downloadStatus3.percent.toFixed(2)} %`
                visible: updates.updateInProgress && !updates.downloadFinished
            }

            Text {
                Layout.minimumWidth: 150
                Layout.maximumWidth: 150
                elide: Text.ElideRight
                text: updates.downloadStatus4.name
                visible: updates.updateInProgress && !updates.downloadFinished
            }
            Controls.ProgressBar {
                Layout.fillWidth: true
                from: 0
                to: 100
                value: updates.downloadStatus4.percent
                visible: updates.updateInProgress && !updates.downloadFinished
            }
            Text {
                Layout.alignment: Qt.AlignRight
                text: `${updates.downloadStatus4.percent.toFixed(2)} %`
                visible: updates.updateInProgress && !updates.downloadFinished
            }

            Text {
                Layout.minimumWidth: 150
                Layout.maximumWidth: 150
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
            Text {
                text: `${updates.installCurrent} von ${updates.installHowMany}`
                visible: updates.updateInProgress && updates.downloadFinished
            }
        }

        Controls.ScrollView {
            Layout.fillWidth: true
            Layout.fillHeight: true

            Kirigami.CardsListView {
                id: view

                model: updates.updatablePackages
                clip: true
                delegate: Kirigami.AbstractCard
                {
                    header: GridLayout {
                        columns: 2

                        Kirigami.Heading {
                            text: name
                            level: 2
                        }
                        Text {
                            Layout.alignment: Qt.AlignRight
                            text: version
                            opacity: 0.5
                            wrapMode: Text.WordWrap
                        }
                    }

                    contentItem: Text
                    {
                        text: description
                        wrapMode: Text.WordWrap
                    }
                }
            }
        }
    }
}
