import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import cloud.ulbricht.jewels

Kirigami.ScrollablePage {
    required property Login login

    objectName: "MainPage"

    id: pageRoot

    implicitWidth: Kirigami.Units.gridUnit * 20

    leftPadding: 0
    rightPadding: 0
    bottomPadding: 0
    topPadding: 0
    title: "Jewels"

    actions: [
        Kirigami.Action {
            text: "Abmelden"
            visible: login.loggedIn
            onTriggered: {
                login.logout()

                const stack = applicationWindow().pageStack
                const current = stack.currentItem

                const onUpdatesPage = current && current.objectName === "UpdatesPage"
                if (!onUpdatesPage && stack.depth > 1) {
                    stack.replace(Qt.resolvedUrl("pages/LoginPage.qml"), { login: login })
                }
            }
        }
    ]

    Kirigami.PagePool {
        id: mainPagePool
    }
    Kirigami.PagePoolAction {
        id: loginAction
        pagePool: mainPagePool
        basePage: pageRoot
        page: "/cloud/ulbricht/jewels/qml/ui/pages/LoginPage.qml"
        initialProperties: ({
            "login": login
        })
    }
    Kirigami.PagePoolAction {
        id: updateAction
        pagePool: mainPagePool
        basePage: pageRoot
        page: "/cloud/ulbricht/jewels/qml/ui/pages/UpdatesPage.qml"
    }
    Kirigami.PagePoolAction {
        id: jewelsAction
        pagePool: mainPagePool
        basePage: pageRoot
        page: "/cloud/ulbricht/jewels/qml/ui/pages/JewelsPage.qml"
        initialProperties: ({
            "login": login
        })
    }
    Kirigami.PagePoolAction {
        id: twoFactorAction
        pagePool: mainPagePool
        basePage: pageRoot
        page: "/cloud/ulbricht/jewels/qml/ui/pages/TwoFactorPage.qml"
        initialProperties: ({
            "login": login
        })
    }
    background: Rectangle {
        anchors.fill: parent
        Kirigami.Theme.colorSet: Kirigami.Theme.View
        color: Kirigami.Theme.backgroundColor
    }

    component JewelsCard: Kirigami.AbstractCard
    {
        property string title: ""
        property string info: ""

        Layout.fillHeight: true
        header: Kirigami.Heading
        {
            text: title
            level: 2
        }
        contentItem: Text
        {
            wrapMode: Text.WordWrap
            text: info
        }
        highlighted: action.checked
        implicitWidth: Kirigami.Units.gridUnit * 30
        Layout.maximumWidth: Kirigami.Units.gridUnit * 30
        activeFocusOnTab: true
        showClickFeedback: true
    }
    component JewelsDelegate: Controls.ItemDelegate {
        Layout.fillWidth: true
        id: delegate
        visible: root.pageStack.wideMode
    }

    ColumnLayout {
        spacing: 0
        ColumnLayout {
            JewelsDelegate {
                visible: !login.loggedIn && root.pageStack.wideMode
                text: "Anmelden"
                action: loginAction
            }
            JewelsDelegate {
                text: "Updates"
                action: updateAction
            }
            JewelsDelegate {
                visible: login.loggedIn && root.pageStack.wideMode
                text: "Jewels"
                action: jewelsAction
            }
            JewelsDelegate {
                visible: login.loggedIn && root.pageStack.wideMode
                text: "Zwei Faktor Codes"
                action: twoFactorAction
            }
        }

        Kirigami.CardsLayout {
            visible: !root.pageStack.wideMode
            Layout.topMargin: Kirigami.Units.largeSpacing
            Layout.leftMargin: Kirigami.Units.gridUnit
            Layout.rightMargin: Kirigami.Units.gridUnit
            JewelsCard {
                action: loginAction
                visible: !login.loggedIn
                title: "Anmelden"
                info: "Hier kannst du dich bei Jewels anmelden um alle Features zu nutzen die den Server brauchen. Dazu zählt das automatische Inventar und auch der Zwei Faktor Dienst"
            }
            JewelsCard {
                action: updateAction
                title: "Updates"
                info: "Hier kannst du deinen Laptop aktuell halten. Einfach aufmachen, schauen ob es Updates gibt, installieren und etwas warten. Danach ist dein Laptop auf dem neuesten Stand."
            }
            JewelsCard {
                action: jewelsAction
                visible: login.loggedIn
                title: "Jewels"
                info: "Hier kannst du dir alle deine Geräte anschauen, sei es dein Laptop oder ein Smartphone. Du bekommst Informationen zum Gerät und kannst auch schauen ob du bald ein neues brauchst."
            }
            JewelsCard {
                action: twoFactorAction
                visible: login.loggedIn
                title: "Zwei Faktor Codes"
                info: "Du brauchst einen Zwei Faktor Code? Dann bist du hier genau richtig. Such einfach die entsprechende Website raus, kopier dir den Code und los gehts."
            }
        }
    }
}
