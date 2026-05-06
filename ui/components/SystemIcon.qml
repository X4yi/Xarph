import QtQuick 2.15

Item {
    id: root

    property string icon: "unknown"
    property bool active: false
    property string text: ""
    property string tooltip: ""

    width: 24
    height: 24

    Rectangle {
        anchors.fill: parent
        radius: 4
        color: root.active ? "#4a90d9" : "#3a3a4e"

        Text {
            anchors.centerIn: parent
            text: root.icon.charAt(0).toUpperCase()
            color: root.active ? "white" : "#666666"
            font.pixelSize: 10
            font.bold: true
        }
    }

    Text {
        anchors.left: parent.right
        anchors.leftMargin: 4
        anchors.verticalCenter: parent.verticalCenter
        text: root.text
        color: root.active ? "white" : "#666666"
        font.pixelSize: 10
        visible: root.text !== ""
    }

    MouseArea {
        anchors.fill: parent
        hoverEnabled: true
        onEntered: {
            if (root.tooltip) {
                console.log("Tooltip:", root.tooltip)
            }
        }
    }
}
