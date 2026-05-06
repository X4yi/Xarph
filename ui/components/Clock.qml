import QtQuick 2.15
import qs.services 1.0 as Services

Item {
    id: root

    width: clockText.implicitWidth + 16
    height: clockText.implicitHeight + 8

    property var timeService: Services.TimeService

    Rectangle {
        anchors.fill: parent
        color: "#2a2a3e"
        radius: 6
    }

    Text {
        id: clockText
        anchors.centerIn: parent
        color: "white"
        font.pixelSize: 13
        font.family: "Monospace"

        text: root.timeService ? root.timeService.formattedTime : "00:00"
    }
}
