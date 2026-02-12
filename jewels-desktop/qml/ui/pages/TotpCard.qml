import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import cloud.ulbricht.jewels

Kirigami.AbstractCard {
    required property var otpId
    required property var accountName
    required property var accountIssuer
    required property var secretKey
    required property var canEdit
    required property var iconSource
    required property var host
    required property var sharedWithEmails
    signal share(var otpId, var accountIssuer, var sharedWithEmails)
    signal edit(var otpId, var accountName, var accountIssuer)
    signal remove(var otpId, var accountIssuer)

    id: card
    width: Kirigami.Units.gridUnit * 18
    clip: true
    onClicked: {
        Clipboard.copy(Otp.generate(secretKey));
        showPassiveNotification(`Der Code für ${accountIssuer} wurde kopiert!`);
    }

    contentItem: Item {
        width: parent.width
        implicitHeight: contentLayout.implicitHeight
        anchors.fill: parent

        Item {
            id: topProgressStrip
            anchors.left: parent.left
            anchors.right: parent.right
            anchors.top: parent.top
            anchors.leftMargin: -card.leftPadding
            anchors.rightMargin: -card.rightPadding
            anchors.topMargin: -card.topPadding
            height: 5
            clip: true

            Rectangle {
                anchors.fill: parent
                color: Kirigami.Theme.backgroundColor
                opacity: 0.35
            }

            Rectangle {
                anchors.left: parent.left
                anchors.top: parent.top
                anchors.bottom: parent.bottom
                width: parent.width * (timeout.value - timeout.from) / (timeout.to - timeout.from)
                color: Kirigami.Theme.highlightColor

                topLeftRadius: 8
                topRightRadius: 8
                bottomLeftRadius: 0
                bottomRightRadius: 0
            }
        }

        Timer {
            property int lastWindow: -1

            interval: 100
            running: true
            repeat: true
            onTriggered: {
                const window = Math.floor(Date.now() / 30000)
                if (window !== lastWindow) {
                    lastWindow = window
                    timeout.resync()
                    otpCode.text = Otp.generate(secretKey)
                }
            }
        }

        ColumnLayout {
            id: contentLayout
            anchors.fill: parent
            spacing: Kirigami.Units.smallSpacing

            Controls.ProgressBar {
                Layout.fillWidth: true

                visible: false
                id: timeout
                from: 0
                to: 30

                function resync() {
                    const now = Date.now()
                    const elapsedMs = now % 30000
                    const remainingMs = 30000 - elapsedMs

                    const elapsed = elapsedMs / 1000.0
                    timeout.value = elapsed

                    fill.stop()
                    fill.from = elapsed
                    fill.to = 30
                    fill.duration = remainingMs
                    fill.start()
                }

                NumberAnimation {
                    id: fill
                    target: timeout
                    property: "value"
                    easing.type: Easing.Linear
                }

                Component.onCompleted: resync()
            }
            Item {
                Layout.fillWidth: true
                Layout.preferredHeight: Kirigami.Units.gridUnit * 3

                Image {
                    anchors.centerIn: parent
                    width: Kirigami.Units.gridUnit * 3
                    height: Kirigami.Units.gridUnit * 3
                    source: `${host}${iconSource}`
                    asynchronous: true
                    cache: true
                    smooth: true
                    fillMode: Image.PreserveAspectFit
                    sourceSize: Qt.size(
                        width * Screen.devicePixelRatio * 2,
                        height * Screen.devicePixelRatio * 2
                    )
                }
            }
            Kirigami.Heading {
                Layout.fillWidth: true

                level: 2
                text: accountIssuer
                horizontalAlignment: Text.AlignHCenter
                wrapMode: Text.NoWrap
                elide: Text.ElideRight
                maximumLineCount: 1
            }
            Text {
                Layout.fillWidth: true

                text: Otp.generate(secretKey)
                id: otpCode
                font.family: "monospace"
                font.pointSize: Kirigami.Units.gridUnit
                horizontalAlignment: Text.AlignHCenter
            }
            Text {
                Layout.fillWidth: true

                text: accountName
                horizontalAlignment: Text.AlignHCenter
                wrapMode: Text.NoWrap
                elide: Text.ElideRight
                maximumLineCount: 1
            }
        }
    }

    footer: Controls.ToolBar
    {
        id: footerBar

        visible: canEdit

        Kirigami.Theme.inherit: true
        Kirigami.Theme.colorSet: Kirigami.Theme.View

        background: Rectangle {
            border.width: 0
        }
        contentItem: RowLayout {
            spacing: Kirigami.Units.smallSpacing

            Controls.ToolButton {
                text: "Teilen"
                icon.name: "document-share"
                onClicked: {
                    share(otpId, accountIssuer, sharedWithEmails)
                }
            }
            Controls.ToolButton {
                text: "Bearbeiten"
                icon.name: "edit"
                onClicked: {
                    edit(otpId, accountName, accountIssuer)
                }
            }
            Controls.ToolButton {
                text: "Löschen"
                icon.name: "delete"
                onClicked: {
                    remove(otpId, accountIssuer)
                }
            }
        }
    }
}