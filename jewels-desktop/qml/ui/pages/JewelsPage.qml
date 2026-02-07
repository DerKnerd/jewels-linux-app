import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import cloud.ulbricht.jewels

Kirigami.ScrollablePage {
    required property Login login

    title: "Jewels"
    id: jewelsPage

    GridLayout {
        columns: 1
        flow: GridLayout.TopToBottom

        Kirigami.Heading {
            Layout.fillWidth: true
            text: "Verbunden"
        }
        Text {
            Layout.fillWidth: true
            text: "Du bist mit <b>" + login.host + "</b> verbunden"
            textFormat: Text.RichText
        }
    }
}
