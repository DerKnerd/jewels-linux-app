pragma ComponentBehavior: Bound
import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import cloud.ulbricht.jewels
import org.kde.kirigami as Kirigami

Kirigami.ApplicationWindow {
    id: root

    height: Kirigami.Units.gridUnit * 40
    minimumHeight: Kirigami.Units.gridUnit * 20
    minimumWidth: Kirigami.Units.gridUnit * 20
    title: "Jewels"
    width: Kirigami.Units.gridUnit * 65

    Login {
        id: login

        Component.onCompleted: {
            Owners.load();
        }
    }
    pageStack {
        columnView.columnResizeMode: pageStack.wideMode ? Kirigami.ColumnView.DynamicColumns : Kirigami.ColumnView.SingleColumn
        initialPage: mainPageComponent
    }
    Component {
        id: mainPageComponent

        MainPage {
            login: login
        }
    }
}
