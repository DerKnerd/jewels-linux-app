import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import cloud.ulbricht.jewels

Kirigami.ScrollablePage {
    title: "Jewels"
    id: jewelsPage

    ColumnLayout {
        anchors.fill: parent
        spacing: Kirigami.Units.smallSpacing

        Kirigami.Heading {
            Layout.fillWidth: true
            text: "Verbunden"
        }
        Controls.Label {
            Layout.fillWidth: true
            text: "Du bist mit <b>" + Login.host + "</b> verbunden"
            textFormat: Text.RichText
        }
    }
}
