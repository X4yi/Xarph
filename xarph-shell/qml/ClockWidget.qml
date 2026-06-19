import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Xarph

Item {
    id: clockWidget
    width: clockLabel.width + dateLabel.width + 32
    height: 40

    property string clockFormat: "%H:%M"
    property string dateFormat: "%a %d %b"

    ClockBridge {
        id: clockBridge
    }

    RowLayout {
        anchors.fill: parent
        anchors.leftMargin: 8
        anchors.rightMargin: 8
        spacing: 8

        Text {
            id: clockLabel
            text: clockBridge.timeText
            font.pixelSize: 13
            font.family: "monospace"
            color: "#e0e0f0"
            Layout.alignment: Qt.AlignVCenter
        }

        Text {
            id: dateLabel
            text: clockBridge.dateText
            font.pixelSize: 11
            color: "#8888a0"
            Layout.alignment: Qt.AlignVCenter
        }
    }

    Timer {
        interval: 1000
        running: true
        repeat: true
        onTriggered: {
            clockBridge.updateTime()
        }
    }

    Component.onCompleted: {
        clockBridge.updateTime()
    }

    MouseArea {
        anchors.fill: parent
        acceptedButtons: Qt.RightButton
        onClicked: function(mouse) {
            contextMenu.buildMenu("widget", "Clock")
            contextMenu.x = mouse.x
            contextMenu.y = mouse.y
            contextMenu.visible = true
        }
    }
}
