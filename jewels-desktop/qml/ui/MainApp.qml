pragma ComponentBehavior: Bound

import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import org.kde.config as KConfig
import cloud.ulbricht.jewels

Kirigami.ApplicationWindow {
    id: root

    Login {
        id: login

        Component.onCompleted: {
            Owners.load();
        }
    }

    minimumHeight: Kirigami.Units.gridUnit * 20
    minimumWidth: Kirigami.Units.gridUnit * 20
    height: Kirigami.Units.gridUnit * 40
    width: Kirigami.Units.gridUnit * 65

    title: "Jewels"

    pageStack {
        initialPage: mainPageComponent
        columnView.columnResizeMode: pageStack.wideMode ? Kirigami.ColumnView.DynamicColumns : Kirigami.ColumnView.SingleColumn
    }

    Component {
        id: mainPageComponent
        MainPage {
            login: login
        }
    }
}
