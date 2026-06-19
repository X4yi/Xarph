import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Xarph

Button {
    id: startButton
    width: 40
    height: 40

    background: Rectangle {
        color: parent.hovered ? "#2a2a4e" : "transparent"
        radius: 4
    }

    contentItem: Text {
        text: "☰"
        font.pixelSize: 18
        color: "#e0e0f0"
        horizontalAlignment: Text.AlignHCenter
        verticalAlignment: Text.AlignVCenter
    }

    onClicked: {
        startMenu.visible = !startMenu.visible
        if (startMenu.visible) {
            startMenu.loadApps()
        }
    }
}
