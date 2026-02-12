import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import cloud.ulbricht.jewels

Kirigami.ScrollablePage {
    id: twoFactorPage

    required property Login login

    title: "Zwei-Faktor Codes"

    Component.onCompleted: {
        oneTimePasswords.loadOneTimePasswords();
    }

    OneTimePasswords {
        id: oneTimePasswords
    }
    ColumnLayout {
        anchors.fill: parent
        spacing: Kirigami.Units.largeSpacing

        Kirigami.Heading {
            Layout.alignment: Qt.AlignTop
            Layout.fillWidth: true
            text: 'Meine Zwei-Faktor Codes'
        }
        Controls.BusyIndicator {
            id: updatesBusyIndicator

            Layout.alignment: Qt.AlignTop | Qt.AlignHCenter
            visible: oneTimePasswords.loading
        }
        ColumnLayout {
            Layout.fillHeight: true
            Layout.fillWidth: true

            Flow {
                Layout.fillHeight: true
                Layout.fillWidth: true
                spacing: Kirigami.Units.largeSpacing

                Repeater {
                    model: oneTimePasswords.myOneTimePasswords

                    delegate: Item {
                        property var rAccountIssuer: accountIssuer
                        property var rAccountName: accountName
                        property var rCanEdit: canEdit
                        property var rIconSource: iconSource
                        property var rOtpId: otpId
                        property var rSecretKey: secretKey
                        property var rSharedWithEmails: sharedWithEmails

                        height: card.height
                        width: card.width

                        TotpCard {
                            id: card

                            accountIssuer: parent.rAccountIssuer
                            accountName: parent.rAccountName
                            canEdit: parent.rCanEdit
                            host: login.host
                            iconSource: parent.rIconSource
                            otpId: parent.rOtpId
                            secretKey: parent.rSecretKey
                            sharedWithEmails: parent.rSharedWithEmails

                            onEdit: (otpId, accountName, accountIssuer) => {
                                editDialog.accountName = accountName;
                                editDialog.accountIssuer = accountIssuer;
                                editDialog.otpId = otpId;
                                editDialog.open();
                            }
                            onRemove: (otpId, accountIssuer) => {
                                deleteDialog.accountIssuer = accountIssuer;
                                deleteDialog.otpId = otpId;
                                deleteDialog.open();
                            }
                            onShare: (otpId, accountIssuer, sharedWithEmails) => {
                                shareDialog.sharedWithEmails = sharedWithEmails;
                                shareDialog.otpId = otpId;
                                shareDialog.accountIssuer = accountIssuer;
                                shareDialog.open();
                            }
                        }
                    }
                }
            }
        }
        Repeater {
            model: oneTimePasswords.sharedOneTimePasswords

            delegate: ColumnLayout {
                required property var modelData

                Layout.fillHeight: true
                Layout.fillWidth: true
                spacing: Kirigami.Units.largeSpacing

                Kirigami.Heading {
                    level: 1
                    text: `Geteilt von ${modelData.name}`
                }
                Flow {
                    Layout.fillHeight: true
                    Layout.fillWidth: true
                    spacing: Kirigami.Units.largeSpacing

                    Repeater {
                        model: modelData.otpCodes

                        delegate: Item {
                            property var rAccountIssuer: modelData.accountIssuer
                            property var rAccountName: modelData.accountName
                            property var rCanEdit: modelData.canEdit
                            property var rIconSource: modelData.iconSource
                            property var rOtpId: modelData.otpId
                            property var rSecretKey: modelData.secretKey

                            height: card.height
                            width: card.width

                            TotpCard {
                                id: card

                                accountIssuer: parent.rAccountIssuer
                                accountName: parent.rAccountName
                                canEdit: false
                                host: login.host
                                iconSource: parent.rIconSource
                                otpId: parent.rOtpId
                                secretKey: parent.rSecretKey
                                sharedWithEmails: parent.rSharedWithEmails
                            }
                        }
                    }
                }
            }
        }
    }
    Kirigami.Dialog {
        id: shareDialog

        property var accountIssuer: ""
        property var otpId: 0
        property var sharedWithEmails: []

        function addEmail(email) {
            if (shareDialog.sharedWithEmails.indexOf(email) === -1) {
                shareDialog.sharedWithEmails = shareDialog.sharedWithEmails.concat([email]);
            }
        }
        function removeEmail(email) {
            shareDialog.sharedWithEmails = shareDialog.sharedWithEmails.filter(e => e !== email);
        }

        standardButtons: Kirigami.Dialog.NoButton
        title: "Teilen mit…"

        contentItem: Controls.ScrollView {
            Component.onCompleted: background.visible = true

            ListView {
                id: usersList

                clip: true
                height: Math.min(contentHeight, 400)
                model: Owners.owners
                width: parent.width

                delegate: Controls.SwitchDelegate {
                    checked: shareDialog.sharedWithEmails.indexOf(email) !== -1
                    text: name
                    width: ListView.view.width - ListView.view.leftMargin - ListView.view.rightMargin

                    onToggled: {
                        if (checked) {
                            shareDialog.addEmail(email);
                        } else {
                            shareDialog.removeEmail(email);
                        }
                    }
                }
            }
        }
        customFooterActions: [
            Kirigami.Action {
                text: "Teilen"

                onTriggered: {
                    oneTimePasswords.shareOtp(shareDialog.otpId, shareDialog.sharedWithEmails);
                    shareDialog.close();
                    showPassiveNotification(`Der Account ${shareDialog.accountIssuer} wurde geteilt!`);
                }
            },
            Kirigami.Action {
                text: "Verwerfen"

                onTriggered: {
                    shareDialog.close();
                }
            }
        ]
    }
    Kirigami.Dialog {
        id: editDialog

        property var accountIssuer: ""
        property var accountName: ""
        property var otpId: 0

        standardButtons: Kirigami.Dialog.NoButton
        title: "Namen bearbeiten"

        contentItem: Item {
            implicitHeight: layout.implicitHeight + Kirigami.Units.largeSpacing * 2
            implicitWidth: layout.implicitWidth + Kirigami.Units.largeSpacing * 2

            Kirigami.FormLayout {
                id: layout

                anchors.fill: parent
                anchors.margins: Kirigami.Units.largeSpacing

                Controls.TextField {
                    Kirigami.FormData.label: "Name:"
                    text: editDialog.accountName

                    onTextEdited: editDialog.accountName = text
                }
            }
        }
        customFooterActions: [
            Kirigami.Action {
                text: "Speichern"

                onTriggered: {
                    oneTimePasswords.editOtp(editDialog.otpId, editDialog.accountName);
                    editDialog.close();
                    showPassiveNotification(`Der Account ${editDialog.accountIssuer} wurde umbenannt!`);
                }
            },
            Kirigami.Action {
                text: "Verwerfen"

                onTriggered: {
                    editDialog.close();
                }
            }
        ]
    }
    Kirigami.PromptDialog {
        id: deleteDialog

        property var accountIssuer: ""
        property var otpId: 0

        standardButtons: Kirigami.Dialog.NoButton
        subtitle: `Soll der Zwei-Faktor Account ${deleteDialog.accountIssuer} wirklich gelöscht werden?`
        title: `${accountIssuer} löschen?`

        customFooterActions: [
            Kirigami.Action {
                text: "Löschen"

                onTriggered: {
                    oneTimePasswords.deleteOtp(deleteDialog.otpId);
                    deleteDialog.close();
                    showPassiveNotification(`Der Account ${deleteDialog.accountIssuer} wurde gelöscht!`);
                }
            },
            Kirigami.Action {
                text: "Behalten"

                onTriggered: {
                    deleteDialog.close();
                }
            }
        ]
    }
}
