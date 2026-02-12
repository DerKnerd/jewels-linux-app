import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import cloud.ulbricht.jewels

Kirigami.AbstractCard {
    id: card

    required property var accountIssuer
    required property var accountName
    required property var canEdit
    required property var host
    required property var iconSource
    required property var otpId
    required property var secretKey
    required property var sharedWithEmails

    signal edit(var otpId, var accountName, var accountIssuer)
    signal remove(var otpId, var accountIssuer)
    signal share(var otpId, var accountIssuer, var sharedWithEmails)

    clip: true
    showClickFeedback: true
    width: Kirigami.Units.gridUnit * 18

    contentItem: Item {
        anchors.fill: parent
        implicitHeight: contentLayout.implicitHeight
        width: parent.width

        Item {
            id: topProgressStrip

            anchors.left: parent.left
            anchors.leftMargin: -card.leftPadding
            anchors.right: parent.right
            anchors.rightMargin: -card.rightPadding
            anchors.top: parent.top
            anchors.topMargin: -card.topPadding
            clip: true
            height: 5

            Rectangle {
                anchors.fill: parent
                color: Kirigami.Theme.backgroundColor
                opacity: 0.35
            }
            Rectangle {
                anchors.bottom: parent.bottom
                anchors.left: parent.left
                anchors.top: parent.top
                bottomLeftRadius: 0
                bottomRightRadius: 0
                color: Kirigami.Theme.highlightColor
                topLeftRadius: 8
                topRightRadius: 8
                width: parent.width * (timeout.value - timeout.from) / (timeout.to - timeout.from)
            }
        }
        Timer {
            property int lastWindow: -1

            interval: 100
            repeat: true
            running: true

            onTriggered: {
                const window = Math.floor(Date.now() / 30000);
                if (window !== lastWindow) {
                    lastWindow = window;
                    timeout.resync();
                    otpCode.text = Otp.generate(secretKey);
                }
            }
        }
        ColumnLayout {
            id: contentLayout

            anchors.fill: parent
            spacing: Kirigami.Units.smallSpacing

            Controls.ProgressBar {
                id: timeout

                function resync() {
                    const now = Date.now();
                    const elapsedMs = now % 30000;
                    const remainingMs = 30000 - elapsedMs;

                    const elapsed = elapsedMs / 1000.0;
                    timeout.value = elapsed;

                    fill.stop();
                    fill.from = elapsed;
                    fill.to = 30;
                    fill.duration = remainingMs;
                    fill.start();
                }

                Layout.fillWidth: true
                from: 0
                to: 30
                visible: false

                Component.onCompleted: resync()

                NumberAnimation {
                    id: fill

                    easing.type: Easing.Linear
                    property: "value"
                    target: timeout
                }
            }
            Item {
                Layout.fillWidth: true
                Layout.preferredHeight: Kirigami.Units.gridUnit * 3

                Image {
                    anchors.centerIn: parent
                    asynchronous: true
                    cache: true
                    fillMode: Image.PreserveAspectFit
                    height: Kirigami.Units.gridUnit * 3
                    smooth: true
                    source: `${host}${iconSource}`
                    sourceSize: Qt.size(width * Screen.devicePixelRatio * 2, height * Screen.devicePixelRatio * 2)
                    width: Kirigami.Units.gridUnit * 3
                }
            }
            Kirigami.Heading {
                Layout.fillWidth: true
                elide: Text.ElideRight
                horizontalAlignment: Text.AlignHCenter
                level: 2
                maximumLineCount: 1
                text: accountIssuer
                wrapMode: Text.NoWrap
            }
            Text {
                id: otpCode

                Layout.fillWidth: true
                font.family: "monospace"
                font.pointSize: Kirigami.Units.gridUnit
                horizontalAlignment: Text.AlignHCenter
                text: Otp.generate(secretKey)
            }
            Text {
                Layout.fillWidth: true
                elide: Text.ElideRight
                horizontalAlignment: Text.AlignHCenter
                maximumLineCount: 1
                text: accountName
                wrapMode: Text.NoWrap
            }
        }
    }
    footer: RowLayout {
        spacing: Kirigami.Units.smallSpacing
        visible: canEdit

        Controls.ToolButton {
            icon.name: "document-share"
            text: "Teilen"

            onClicked: {
                share(otpId, accountIssuer, sharedWithEmails);
            }
        }
        Controls.ToolButton {
            icon.name: "edit"
            text: "Bearbeiten"

            onClicked: {
                edit(otpId, accountName, accountIssuer);
            }
        }
        Controls.ToolButton {
            icon.name: "delete"
            text: "Löschen"

            onClicked: {
                remove(otpId, accountIssuer);
            }
        }
    }

    onClicked: {
        Clipboard.copy(Otp.generate(secretKey));
        showPassiveNotification(`Der Code für ${accountIssuer} wurde kopiert!`);
    }
}