import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import cloud.ulbricht.jewels
import org.kde.kirigami as Kirigami

Kirigami.ScrollablePage {
    id: installPage

    background: Kirigami.Theme.backgroundColor
    objectName: "InstallPage"
    title: "Software installieren"

    actions: [
        Kirigami.Action {
            text: "Software installieren"

            onTriggered: installConfirm.open()
        }
    ]
    header: Controls.ToolBar {
        id: toolbar

        RowLayout {
            anchors.fill: parent

            Kirigami.SearchField {
                id: searchField

                Layout.alignment: Qt.AlignHCenter
                Layout.fillWidth: true
                delaySearch: true
                placeholderText: "Software durchsuchen"

                onAccepted: {
                    install.search(searchField.text);
                }
            }
        }
    }

    Install {
        id: install

        Component.onCompleted: install.search("")
    }
    ColumnLayout {
        anchors.fill: parent
        spacing: Kirigami.Units.largeSpacing

        Controls.BusyIndicator {
            id: installBusyIndicator

            Layout.alignment: Qt.AlignTop | Qt.AlignHCenter
            visible: install.refreshing
        }
        GridLayout {
            Layout.alignment: Qt.AlignTop
            columnSpacing: Kirigami.Units.smallSpacing
            columns: 3
            rowSpacing: Kirigami.Units.smallSpacing
            visible: install.installInProgress

            Controls.Label {
                Layout.maximumWidth: 150
                Layout.minimumWidth: 150
                elide: Text.ElideRight
                text: install.downloadStatus1.name
                visible: install.installInProgress && !install.downloadFinished
            }
            Controls.ProgressBar {
                Layout.fillWidth: true
                from: 0
                to: install.downloadStatus1.total
                value: install.downloadStatus1.current
                visible: install.installInProgress && !install.downloadFinished
            }
            Controls.Label {
                Layout.alignment: Qt.AlignRight
                text: `${install.downloadStatus1.percent.toFixed(0)} %`
                visible: install.installInProgress && !install.downloadFinished
            }
            Controls.Label {
                Layout.maximumWidth: 150
                Layout.minimumWidth: 150
                elide: Text.ElideRight
                text: install.downloadStatus2.name
                visible: install.installInProgress && !install.downloadFinished
            }
            Controls.ProgressBar {
                Layout.fillWidth: true
                from: 0
                to: install.downloadStatus2.total
                value: install.downloadStatus2.current
                visible: install.installInProgress && !install.downloadFinished
            }
            Controls.Label {
                Layout.alignment: Qt.AlignRight
                text: `${install.downloadStatus2.percent.toFixed(0)} %`
                visible: install.installInProgress && !install.downloadFinished
            }
            Controls.Label {
                Layout.maximumWidth: 150
                Layout.minimumWidth: 150
                elide: Text.ElideRight
                text: install.downloadStatus3.name
                visible: install.installInProgress && !install.downloadFinished
            }
            Controls.ProgressBar {
                Layout.fillWidth: true
                from: 0
                to: install.downloadStatus3.total
                value: install.downloadStatus3.current
                visible: install.installInProgress && !install.downloadFinished
            }
            Controls.Label {
                Layout.alignment: Qt.AlignRight
                text: `${install.downloadStatus3.percent.toFixed(0)} %`
                visible: install.installInProgress && !install.downloadFinished
            }
            Controls.Label {
                Layout.maximumWidth: 150
                Layout.minimumWidth: 150
                elide: Text.ElideRight
                text: install.downloadStatus4.name
                visible: install.installInProgress && !install.downloadFinished
            }
            Controls.ProgressBar {
                Layout.fillWidth: true
                from: 0
                to: install.downloadStatus4.total
                value: install.downloadStatus4.current
                visible: install.installInProgress && !install.downloadFinished
            }
            Controls.Label {
                Layout.alignment: Qt.AlignRight
                text: `${install.downloadStatus4.percent.toFixed(0)} %`
                visible: install.installInProgress && !install.downloadFinished
            }
            Controls.Label {
                Layout.maximumWidth: 150
                Layout.minimumWidth: 150
                elide: Text.ElideRight
                text: install.installPackage
                visible: install.installInProgress && install.downloadFinished
            }
            Controls.ProgressBar {
                Layout.fillWidth: true
                from: 0
                to: install.installHowMany
                value: install.installCurrent
                visible: install.installInProgress && install.downloadFinished
            }
            Controls.Label {
                text: `${install.installCurrent} von ${install.installHowMany}`
                visible: install.installInProgress && install.downloadFinished
            }
        }
        ListView {
            id: view

            Layout.fillHeight: true
            Layout.fillWidth: true
            model: install
            spacing: Kirigami.Units.largeSpacing

            delegate: Kirigami.AbstractCard {
                contentItem: Controls.Label {
                    text: description
                    wrapMode: Text.WordWrap
                }
                footer: Kirigami.ActionToolBar {
                    id: actionsToolBar

                    position: Controls.ToolBar.Footer

                    actions: [
                        Kirigami.Action {
                            checkable: true
                            icon.name: "add"
                            text: "Hinzufügen"

                            onToggled: install.togglePackage(name)
                        }
                    ]
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
    Kirigami.PromptDialog {
        id: installConfirm

        standardButtons: Kirigami.Dialog.Ok | Kirigami.Dialog.Cancel
        subtitle: "Wenn du auf OK klickst wird die von dir gewählte Software installiert."
        title: "Software installieren?"

        onAccepted: install.performInstall()
    }
}
