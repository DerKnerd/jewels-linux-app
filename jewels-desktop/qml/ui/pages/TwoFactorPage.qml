import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import cloud.ulbricht.jewels

Kirigami.ScrollablePage {
    required property Login login

    OneTimePasswords {
        id: oneTimePasswords
    }

    Component.onCompleted: {
        oneTimePasswords.loadOneTimePasswords();
    }

    title: "Zwei-Faktor Codes"
    id: twoFactorPage

    ColumnLayout {
        anchors.fill: parent

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

        Flow {
            Layout.fillWidth: true
            Layout.fillHeight: true
            spacing: Kirigami.Units.largeSpacing

            Repeater {
                model: oneTimePasswords.myOneTimePasswords

                delegate: Item {
                    property var rOtpId: otpId
                    property var rAccountName: accountName
                    property var rAccountIssuer: accountIssuer
                    property var rSecretKey: secretKey
                    property var rCanEdit: canEdit
                    property var rIconSource: iconSource
                    property var rSharedWithEmails: sharedWithEmails

                    width: card.implicitWidth
                    height: card.implicitHeight

                    TotpCard {
                        id: card

                        host: login.host
                        sharedWithEmails: parent.rSharedWithEmails
                        otpId: parent.rOtpId
                        accountName: parent.rAccountName
                        accountIssuer: parent.rAccountIssuer
                        secretKey: parent.rSecretKey
                        canEdit: parent.rCanEdit
                        iconSource: parent.rIconSource

                        onShare: (otpId, accountIssuer, sharedWithEmails) => {
                            shareDialog.sharedWithEmails = sharedWithEmails;
                            shareDialog.otpId = otpId;
                            shareDialog.accountIssuer = accountIssuer;
                            shareDialog.open();
                        }
                        onRemove: (otpId, accountIssuer) => {
                            deleteDialog.accountIssuer = accountIssuer;
                            deleteDialog.otpId = otpId;
                            deleteDialog.open();
                        }
                        onEdit: (otpId, accountName, accountIssuer) => {
                            editDialog.accountName = accountName;
                            editDialog.accountIssuer = accountIssuer;
                            editDialog.otpId = otpId;
                            editDialog.open();
                        }
                    }
                }
            }
        }

        Repeater {
            model: oneTimePasswords.sharedOneTimePasswords

            delegate: ColumnLayout {
                Layout.fillWidth: true
                Layout.fillHeight: true

                required property var modelData

                spacing: Kirigami.Units.largeSpacing

                Kirigami.Heading {
                    text: `Geteilt von ${modelData.name}`
                    level: 1
                }

                Flow {
                    Layout.fillWidth: true
                    Layout.fillHeight: true
                    spacing: Kirigami.Units.largeSpacing

                    Repeater {
                        model: modelData.otpCodes

                        delegate: Item {
                            property var rOtpId: modelData.otpId
                            property var rAccountName: modelData.accountName
                            property var rAccountIssuer: modelData.accountIssuer
                            property var rSecretKey: modelData.secretKey
                            property var rCanEdit: modelData.canEdit
                            property var rIconSource: modelData.iconSource

                            width: card.width
                            height: card.height

                            TotpCard {
                                id: card

                                host: login.host
                                sharedWithEmails: parent.rSharedWithEmails
                                otpId: parent.rOtpId
                                accountName: parent.rAccountName
                                accountIssuer: parent.rAccountIssuer
                                secretKey: parent.rSecretKey
                                canEdit: false
                                iconSource: parent.rIconSource
                            }
                        }
                    }
                }
            }
        }
    }

    Kirigami.Dialog {
        id: shareDialog

        property var sharedWithEmails: []
        property var accountIssuer: ""
        property var otpId: 0

        function addEmail(email) {
            if (shareDialog.sharedWithEmails.indexOf(email) === -1) {
                shareDialog.sharedWithEmails = shareDialog.sharedWithEmails.concat([email])
            }
        }

        function removeEmail(email) {
            shareDialog.sharedWithEmails = shareDialog.sharedWithEmails.filter((e) => e !== email)
        }

        title: "Teilen mit…"
        standardButtons: Kirigami.Dialog.NoButton
        customFooterActions: [
            Kirigami.Action {
                text: "Teilen"
                onTriggered: {
                    oneTimePasswords.shareOtp(shareDialog.otpId, shareDialog.sharedWithEmails);
                    shareDialog.close()
                    showPassiveNotification(`Der Account ${shareDialog.accountIssuer} wurde geteilt!`);
                }
            }
            ,
            Kirigami.Action {
                text: "Verwerfen"
                onTriggered: {
                    shareDialog.close()
                }
            }
        ]

        contentItem: Controls.ScrollView
        {
            ListView {
                id: usersList
                clip: true
                model: Owners.owners
                height: Math.min(contentHeight, 400)
                width: parent.width

                delegate: Controls.SwitchDelegate
                {
                    width: ListView.view.width - ListView.view.leftMargin - ListView.view.rightMargin
                    text: name
                    checked: shareDialog.sharedWithEmails.indexOf(email) !== -1
                    onToggled: {
                        if (checked) {
                            shareDialog.addEmail(email)
                        } else {
                            shareDialog.removeEmail(email)
                        }
                    }
                }
            }
            Component.onCompleted: background.visible = true;
        }
    }

    Kirigami.Dialog {
        id: editDialog

        property var accountName: ""
        property var accountIssuer: ""
        property var otpId: 0

        title: "Namen bearbeiten"
        standardButtons: Kirigami.Dialog.NoButton
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
                    editDialog.close()
                }
            }
        ]

        contentItem: Item {
            implicitWidth: layout.implicitWidth + Kirigami.Units.largeSpacing * 2
            implicitHeight: layout.implicitHeight + Kirigami.Units.largeSpacing * 2

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
    }

    Kirigami.PromptDialog {
        id: deleteDialog

        property var accountIssuer: ""
        property var otpId: 0

        title: `${accountIssuer} löschen?`
        subtitle: `Soll der Zwei-Faktor Account ${deleteDialog.accountIssuer} wirklich gelöscht werden?`

        standardButtons: Kirigami.Dialog.NoButton
        customFooterActions: [
            Kirigami.Action {
                text: "Löschen"
                onTriggered: {
                    oneTimePasswords.deleteOtp(deleteDialog.otpId);
                    deleteDialog.close()
                    showPassiveNotification(`Der Account ${deleteDialog.accountIssuer} wurde gelöscht!`
                    );
                }
            },
            Kirigami.Action {
                text: "Behalten"
                onTriggered: {
                    deleteDialog.close()
                }
            }
        ]
    }
}
