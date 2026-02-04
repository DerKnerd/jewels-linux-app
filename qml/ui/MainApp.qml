pragma ComponentBehavior: Bound

import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import org.kde.config as KConfig
import cloud.ulbricht.jewels

Kirigami.ApplicationWindow {
    id: root

    minimumHeight: Kirigami.Units.gridUnit * 15
    minimumWidth: Kirigami.Units.gridUnit * 15

    title: "Jewels"

    pageStack {
        initialPage: mainPageComponent
        columnView.columnResizeMode: pageStack.wideMode ? Kirigami.ColumnView.DynamicColumns : Kirigami.ColumnView.SingleColumn
    }

    Component {
        id: mainPageComponent
        MainPage {}
    }
}
