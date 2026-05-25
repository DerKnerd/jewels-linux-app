import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import cloud.ulbricht.jewels
import org.kde.kirigami as Kirigami

Kirigami.ScrollablePage {
    id: pageRoot

    required property Login login

    bottomPadding: 0
    implicitWidth: Kirigami.Units.gridUnit * 20
    leftPadding: 0
    objectName: "MainPage"
    rightPadding: 0
    title: "Jewels"
    topPadding: 0

    actions: [
        Kirigami.Action {
            text: "Abmelden"
            visible: login.loggedIn

            onTriggered: {
                login.logout();
                const stack = applicationWindow().pageStack;
                const current = stack.currentItem;
                const onUpdatesPage = current && current.objectName === "UpdatesPage";
                if (!onUpdatesPage && stack.depth > 1)
                    stack.replace(Qt.resolvedUrl("pages/LoginPage.qml"), {
                        "login": login
                    });
            }
        }
    ]
    background: Rectangle {
        Kirigami.Theme.colorSet: Kirigami.Theme.View
        anchors.fill: parent
        color: Kirigami.Theme.backgroundColor
    }

    Component.onCompleted: {
        if (login.loggedIn) {
            jewels.sendData();
            jewels.checkEolDevices();
        }
    }

    Jewels {
        id: jewels
    }
    Connections {
        function onLoginSuccessful() {
            jewels.sendData();
            jewels.checkEolDevices();
        }

        target: login
    }
    Kirigami.PagePool {
        id: mainPagePool
    }
    Kirigami.PagePoolAction {
        id: loginAction

        basePage: pageRoot
        initialProperties: ({
                "login": login
            })
        page: "pages/LoginPage.qml"
        pagePool: mainPagePool
    }
    Kirigami.PagePoolAction {
        id: updateAction

        basePage: pageRoot
        page: "pages/UpdatesPage.qml"
        pagePool: mainPagePool
    }
    Kirigami.PagePoolAction {
        id: installAction

        basePage: pageRoot
        page: "pages/InstallPage.qml"
        pagePool: mainPagePool
    }
    Kirigami.PagePoolAction {
        id: jewelsAction

        basePage: pageRoot
        initialProperties: ({
                "login": login
            })
        page: "pages/JewelsPage.qml"
        pagePool: mainPagePool
    }
    Kirigami.PagePoolAction {
        id: twoFactorAction

        basePage: pageRoot
        initialProperties: ({
                "login": login
            })
        page: "pages/TwoFactorPage.qml"
        pagePool: mainPagePool
    }
    ColumnLayout {
        spacing: 0

        ColumnLayout {
            JewelsDelegate {
                action: loginAction
                text: "Anmelden"
                visible: !login.loggedIn && root.pageStack.wideMode
            }
            JewelsDelegate {
                action: updateAction
                text: "Updates"
            }
            JewelsDelegate {
                action: installAction
                text: "Software"
            }
            JewelsDelegate {
                action: jewelsAction
                text: "Meine Geräte"
                visible: login.loggedIn && root.pageStack.wideMode
            }
            JewelsDelegate {
                action: twoFactorAction
                text: "Zwei Faktor Codes"
                visible: login.loggedIn && root.pageStack.wideMode
            }
        }
        Kirigami.CardsLayout {
            Layout.leftMargin: Kirigami.Units.gridUnit
            Layout.rightMargin: Kirigami.Units.gridUnit
            Layout.topMargin: Kirigami.Units.largeSpacing
            visible: !root.pageStack.wideMode

            JewelsCard {
                action: loginAction
                info: "Hier kannst du dich bei Jewels anmelden um alle Features zu nutzen die den Server brauchen. Dazu zählt das automatische Inventar und auch der Zwei Faktor Dienst"
                title: "Anmelden"
                visible: !login.loggedIn
            }
            JewelsCard {
                action: updateAction
                info: "Hier kannst du deinen Rechner aktuell halten. Einfach aufmachen, schauen ob es Updates gibt, installieren und etwas warten. Danach ist dein Laptop auf dem neuesten Stand."
                title: "Updates"
            }
            JewelsCard {
                action: installAction
                info: "Hier kannst du neue Software installieren. Durchsuche die offiziellen Quellen von Arch Linux und wähle die Programme die du installieren willst."
                title: "Software"
            }
            JewelsCard {
                action: jewelsAction
                info: "Hier kannst du dir alle deine Geräte anschauen, sei es dein Laptop oder ein Smartphone. Du bekommst Informationen zum Gerät und kannst auch schauen ob du bald ein neues brauchst."
                title: "Meine Geräte"
                visible: login.loggedIn
            }
            JewelsCard {
                action: twoFactorAction
                info: "Du brauchst einen Zwei Faktor Code? Dann bist du hier genau richtig. Such einfach die entsprechende Website raus, kopier dir den Code und los gehts."
                title: "Zwei Faktor Codes"
                visible: login.loggedIn
            }
        }
    }

    component JewelsCard: Kirigami.AbstractCard {
        property string info: ""
        property string title: ""

        Layout.fillHeight: true
        Layout.maximumWidth: Kirigami.Units.gridUnit * 30
        activeFocusOnTab: true
        highlighted: action.checked
        implicitWidth: Kirigami.Units.gridUnit * 30
        showClickFeedback: true

        contentItem: Controls.Label {
            text: info
            wrapMode: Text.WordWrap
        }
        header: Kirigami.Heading {
            level: 2
            text: title
        }
    }
    component JewelsDelegate: Controls.ItemDelegate {
        id: delegate

        Layout.fillWidth: true
        visible: root.pageStack.wideMode
    }
}
