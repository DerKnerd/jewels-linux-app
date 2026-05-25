import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import cloud.ulbricht.jewels

Kirigami.ScrollablePage {
    Install {
        id: install
        Component.onCompleted: install.search("")
    }

    id: installPage
    objectName: "InstallPage"
    title: "Software installieren"
    background: Kirigami.Theme.backgroundColor

    actions: [
        Kirigami.Action {
            text: "Software installieren"

            onTriggered: installConfirm.open()
        }
    ]

    header: Controls.ToolBar
    {
        id: toolbar
        RowLayout {
            anchors.fill: parent
            Kirigami.SearchField {
                id: searchField
                delaySearch: true

                placeholderText: "Software durchsuchen"
                Layout.alignment: Qt.AlignHCenter
                Layout.fillWidth: true

                onAccepted: { install.search(searchField.text) }
            }
        }
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
            columns: 3
            visible: install.installInProgress
            rowSpacing: Kirigami.Units.smallSpacing
            columnSpacing: Kirigami.Units.smallSpacing

            Controls.Label {
                Layout.minimumWidth: 150
                Layout.maximumWidth: 150
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
                Layout.minimumWidth: 150
                Layout.maximumWidth: 150
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
                Layout.minimumWidth: 150
                Layout.maximumWidth: 150
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
                Layout.minimumWidth: 150
                Layout.maximumWidth: 150
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
                Layout.minimumWidth: 150
                Layout.maximumWidth: 150
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
            spacing: Kirigami.Units.largeSpacing

            model: install
            delegate: Kirigami.AbstractCard
            {
                header: GridLayout {
                    columns: 2

                    Kirigami.Heading {
                        text: name
                        level: 2
                    }
                    Controls.Label {
                        Layout.alignment: Qt.AlignRight
                        text: version
                        opacity: 0.5
                        wrapMode: Text.WordWrap
                    }
                }

                contentItem: Controls.Label
                {
                    text: description
                    wrapMode: Text.WordWrap
                }

                footer: Kirigami.ActionToolBar
                {
                    id: actionsToolBar
                    position: Controls.ToolBar.Footer
                    actions: [
                        Kirigami.Action {
                            text: "Hinzufügen"
                            icon.name: "add"
                            checkable: true
                            onToggled: install.togglePackage(name)
                        }
                    ]
                }
            }
        }
    }

    Kirigami.PromptDialog {
        id: installConfirm
        title: "Software installieren?"
        subtitle: "Wenn du auf OK klickst wird die von dir gewählte Software installiert."
        standardButtons: Kirigami.Dialog.Ok | Kirigami.Dialog.Cancel

        onAccepted: install.performInstall()
    }
}
